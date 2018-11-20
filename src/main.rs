#![cfg_attr(not(test), no_main)]
#![no_std]
#![feature(asm)]
#![feature(const_slice_len)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(linkage)]

#[cfg(not(test))]
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("start_x86_64.s"));

#[macro_use]
mod log;
#[cfg(not(test))]
mod boot;
mod config;
mod detect;
mod mem;
