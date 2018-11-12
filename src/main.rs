#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(linkage)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _boot() -> ! {
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

#[used]
#[linkage = "external"]
#[link_section = ".header"]
static HEADER: &[u8] = b"Some data I want to expose in the binary image";
