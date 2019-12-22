use super::cpuio::{inb, outb};

const BASE_PORT: u16 = 0x3F8; // COM1.
const STATUS_OFFSET: u16 = 5;
const STATUS_THRE: u8 = 1 << 5;

/// Initializes UART communication.
pub fn init() {
    unsafe {
        outb(BASE_PORT + 1, 0x00); // Disable all interrupts.
        outb(BASE_PORT + 3, 0x80); // Enable DLAB (set baud rate divisor).
        outb(BASE_PORT + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud.
        outb(BASE_PORT + 1, 0x00); //                  (hi byte).
        outb(BASE_PORT + 3, 0x03); // 8 bits, no parity, one stop bit.
        outb(BASE_PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte
                                   // threshold.
        outb(BASE_PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set.
    }
}

#[inline]
fn is_ready_to_transmit() -> bool {
    unsafe { inb(BASE_PORT + STATUS_OFFSET) & STATUS_THRE != 0 }
}

/// Writes a string to UART port.
pub fn write_str(s: &str) {
    for b in s.bytes() {
        while !is_ready_to_transmit() {}
        unsafe {
            outb(BASE_PORT, b);
        }
    }
}
