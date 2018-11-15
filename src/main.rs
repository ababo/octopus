#![no_main]
#![no_std]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(linkage)]

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("start.x86_64.s"));

mod multiboot;

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
