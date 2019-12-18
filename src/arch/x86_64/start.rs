use super::multiboot as mb;
use core::fmt::{self, Write};
use log;
use main;

const MB_HEADER_FLAGS: u32 = mb::HEADER_MEMORY_INFO;

#[link_section = ".mb_header"]
#[linkage = "external"]
static MB_HEADER: mb::Header = mb::Header {
    magic: mb::HEADER_MAGIC,
    flags: MB_HEADER_FLAGS,
    checksum: (-((mb::HEADER_MAGIC + MB_HEADER_FLAGS) as i32) as u32),
    header_addr: 0,
    load_addr: 0,
    load_end_addr: 0,
    bss_end_addr: 0,
    entry_addr: 0,
    mode_type: 0,
    width: 0,
    height: 0,
    depth: 0,
};

#[cfg(not(test))]
global_asm!(include_str!("boot.s"));

struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let port = 0x400 as *const u16;
        for chr in s.chars() {
            unsafe {
                asm!("outb $0, $1" : : "{al}"(chr as u8), "{dx}"(*port));
            }
        }
        Ok(())
    }
}

static mut SERIAL_WRITER: SerialWriter = SerialWriter;

/// A Rust entry point in multiboot mode.
#[no_mangle]
pub extern "C" fn start_mb(magic: u32, _info: &mb::Info) -> ! {
    unsafe {
        log::init(&mut SERIAL_WRITER, log::Level::Debug);
    }

    if magic != mb::BOOTLOADER_MAGIC {
        fatal!("bad bootloader magic");
    }

    // TODO: Collect multiboot info.

    main();
}

/// Halts the CPU.
#[no_mangle]
pub extern "C" fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
