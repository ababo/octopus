#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(untagged_unions)]

#[cfg(test)]
extern crate core;

#[macro_use]
mod log;

#[cfg(not(test))]
mod arch;

use arch::start::halt;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    halt();
}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    halt();
}

fn main() -> ! {
    info!("Hello {}", "World!");
    halt();
}
