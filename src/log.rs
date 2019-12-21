#![allow(dead_code)]
#![allow(unused_macros)]

use ufmt::uWrite;

/// A log level.
#[derive(PartialEq, PartialOrd)]
pub enum Level {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
}

#[doc(hidden)]
pub struct __Logger {
    write_str: fn(&str),
    max_level: Level,
}

impl __Logger {
    pub fn pre_log(&mut self, level: Level) -> bool {
        if level > self.max_level {
            return false;
        }

        // TODO: Lock and write timestamp.

        let prefix = match level {
            Level::Fatal => "F ",
            Level::Error => "E ",
            Level::Warning => "W ",
            Level::Info => "i ",
            Level::Debug => "d ",
        };

        (self.write_str)(prefix);
        true
    }

    pub fn post_log(&mut self, level: Level) {
        // TODO: Unlock.

        if level == Level::Fatal {
            // TODO: Replace panic machinery to avoid binary bloat.
            // panic!("a fatal event is encountered by the logger");
        }
    }
}

impl uWrite for __Logger {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        (self.write_str)(s);
        Ok(())
    }
}

fn write_nothing(_s: &str) {}

#[doc(hidden)]
pub static mut __LOGGER: __Logger = __Logger {
    write_str: write_nothing,
    max_level: Level::Fatal,
};

/// Initializes kernel logging.
pub fn init(write_str: fn(&str), max_level: Level) {
    unsafe {
        __LOGGER = __Logger {
            write_str,
            max_level,
        };
    }
}

macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        let logger = unsafe { &mut $crate::log::__LOGGER } ;
        if logger.pre_log($level) {
            let _ = uwriteln!(logger, $($arg)*);
            logger.post_log($level);
        }
    };
}

/// Logs a fatal message and panics.
macro_rules! fatal {
    ($($arg:tt)*) => ({
        log!($crate::log::Level::Fatal, $($arg)*);
    })
}

/// Logs an error message.
macro_rules! error {
    ($($arg:tt)*) => ({
        log!($crate::log::Level::Error, $($arg)*);
    })
}

/// Logs a warning message.
macro_rules! warning {
    ($($arg:tt)*) => ({
        log!($crate::log::Level::Warning, $($arg)*);
    })
}

/// Logs an info message.
macro_rules! info {
    ($($arg:tt)*) => ({
        log!($crate::log::Level::Info, $($arg)*);
    })
}

/// Logs a debug message.
#[cfg(build = "debug")]
macro_rules! debug {
    ($($arg:tt)*) => ({
        log!($crate::log::Level::Debug, $($arg)*);
    })
}

/// Logs a debug message.
#[cfg(build = "release")]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
