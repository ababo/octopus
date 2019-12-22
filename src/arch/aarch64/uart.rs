use core::ptr;

const UARTDR_ADDR: usize = 0x900_0000; // QEMU VIRT_UART.
const UARTFR_OFFSET: usize = 0x18;
const UARTFR_TXFF: u8 = 1 << 5;

/// Initializes UART communication.
pub fn init() {}

#[inline]
fn is_ready_to_transmit() -> bool {
    let ptr = (UARTDR_ADDR + UARTFR_OFFSET) as *mut u8;
    unsafe { ptr::read_volatile(ptr) & UARTFR_TXFF == 0 }
}

/// Writes a string to UART port.
pub fn write_str(s: &str) {
    let ptr = UARTDR_ADDR as *mut u8;
    for c in s.bytes() {
        while !is_ready_to_transmit() {}
        unsafe {
            ptr::write_volatile(ptr, c);
        }
    }
}
