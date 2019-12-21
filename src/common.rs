use ufmt::uWrite;

#[doc(hidden)]
pub struct __PanicWriter;

impl uWrite for __PanicWriter {
    type Error = ();
    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        #[no_mangle]
            fn panic(msg: &str) -> !;
        unsafe {
            panic(s);
        }
    }
}

/// This macro triggers a panic. We redefine the libcore panicing machinery to
// avoid binary bloat.
macro_rules! panic {
    ($($arg:tt)*) => ({
        use common;
        let _ = uwrite!(&mut common::__PanicWriter{}, $($arg)*);
    })
}
