use super::multiboot as mb;

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
global_asm!(include_str!("start.s"));

/// Halts the CPU.
#[no_mangle]
pub extern "C" fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
