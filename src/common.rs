/// This macro triggers a panic. We redefine the libcore panicing machinery to
// avoid binary bloat.
macro_rules! panic {
    ($($arg:tt)*) => ({
        let mut handler = unsafe { &mut $crate::arch::panic::__PANIC_HANDLER };
        handler.pre_panic();
        let _ = uwrite!(&mut handler, $($arg)*);
        handler.post_panic();
    })
}
