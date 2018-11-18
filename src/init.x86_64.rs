fn serial_out(s: &str) {
    let port = 0x400 as *const u16;
    for b in s.chars() {
        unsafe {
            asm!("outb $0, $1" : : "{al}"(b as u8), "{dx}"(*port));
        }
    }
}

#[no_mangle]
pub extern "C" fn _init(_magic: u32, _info_ptr: usize) -> ! {
    serial_out("Hello World!");
    loop {}
}
