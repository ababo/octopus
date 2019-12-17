use config::BSP_STACK_SIZE;
use core::fmt::{self, Write};
use log;

#[export_name = "_bsp_stack_size"]
#[linkage = "external"]
#[repr(align(16))]
struct BspStack {
    #[allow(dead_code)]
    body: [u8; BSP_STACK_SIZE],
}

#[export_name = "_bsp_stack"]
#[linkage = "external"]
static mut BSP_STACK: BspStack = BspStack {
    body: [0; BSP_STACK_SIZE],
};

#[export_name = "_bsp_stack_size"]
#[linkage = "external"]
static BSP_STACK_SIZE2: usize = BSP_STACK_SIZE;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    loop {}
}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    loop {}
}

struct SerialWriter;

impl Write for SerialWriter {
    #[cfg(target_arch = "x86_64")]
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

#[no_mangle]
pub extern "C" fn _boot() -> ! {
    unsafe {
        log::init(&mut SERIAL_WRITER, log::Level::Debug);
    }
    info!("Hello {}", "World!");
    loop {}
}
