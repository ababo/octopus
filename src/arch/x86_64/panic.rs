use super::serial;
use core::panic::PanicInfo;
use ufmt::uWrite;

/// Halts the CPU.
#[no_mangle]
pub extern "C" fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

/// This unused function is supposed to be called on libcore panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt();
}

/// This type is used to handle a panic triggered by our own panic!() macro.
#[doc(hidden)]
pub struct __PanicHandler;

impl __PanicHandler {
    pub fn pre_panic(&mut self) {
        serial::write_str("kernel panic: ");
    }

    pub fn post_panic(&mut self) -> ! {
        serial::write_str("\n");
        halt();
    }
}

impl uWrite for __PanicHandler {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        serial::write_str(s);
        Ok(())
    }
}

#[doc(hidden)]
pub static mut __PANIC_HANDLER: __PanicHandler = __PanicHandler {};
