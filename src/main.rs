#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(linkage)]

#[cfg(test)]
extern crate core;

#[macro_use]
extern crate ufmt;

#[macro_use]
mod common;

#[macro_use]
mod log;

#[cfg(not(test))]
mod arch;

use arch::panic::halt;
use arch::serial;

#[no_mangle]
extern "C" fn main() -> ! {
    serial::init();
    log::init(serial::write_str, log::Level::Debug);

    fatal!("Hello {}", "World!");

    halt();
}
