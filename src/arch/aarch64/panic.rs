use super::uart;
use core::panic::PanicInfo;
use ufmt::uWrite;

/// Halts the CPU.
#[no_mangle]
pub extern "C" fn halt() -> ! {
    loop {
        unsafe {
            llvm_asm!("wfi");
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
        uart::write_str("kernel panic: ");
    }

    pub fn post_panic(&mut self) -> ! {
        uart::write_str("\n");
        halt();
    }
}

impl uWrite for __PanicHandler {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        uart::write_str(s);
        Ok(())
    }
}

#[doc(hidden)]
pub static mut __PANIC_HANDLER: __PanicHandler = __PanicHandler {};
