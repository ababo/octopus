#![no_main]
#![no_std]
#![feature(asm)]
#![feature(const_slice_len)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(linkage)]

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("start.x86_64.s"));

#[cfg_attr(target_arch = "x86_64", path = "init.x86_64.rs")]
#[cfg_attr(target_arch = "aarch64", path = "init.aarch64.rs")]
mod init;

#[cfg_attr(target_arch = "x86_64", path = "memory.x86_64.rs")]
#[cfg_attr(target_arch = "aarch64", path = "memory.aarch64.rs")]
mod memory;

#[cfg(target_arch = "x86_64")]
mod multiboot;

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
