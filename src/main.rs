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
extern crate ufmt;

#[macro_use]
mod log;

#[cfg(not(test))]
mod arch;

use arch::serial;
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

#[no_mangle]
extern "C" fn main() -> ! {
    serial::init();
    log::init(serial::write_str, log::Level::Debug);

    info!("Hello {}", "World!");

    halt();
}
