use config::BSP_STACK_SIZE;

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

#[cfg(target_arch = "x86_64")]
fn write_to_serial(s: &str) {
    let port = 0x400 as *const u16;
    for b in s.chars() {
        unsafe {
            asm!("outb $0, $1" : : "{al}"(b as u8), "{dx}"(*port));
        }
    }
}

use log;

#[no_mangle]
pub extern "C" fn _boot() -> ! {
    log::init(write_to_serial, log::Level::Debug);
    log_info!("Hello {}", "World!");
    loop {}
}
