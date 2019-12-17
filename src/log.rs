#![allow(dead_code)]
#![allow(unused_macros)]
use core::fmt::{self, Write};

/// A log level.
#[derive(PartialEq, PartialOrd)]
pub enum Level {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
}

/// An event logger.
pub struct Logger<'a> {
    writer: &'a mut dyn Write,
    max_level: Level,
}

impl<'a> Logger<'a> {
    /// Creates a new logger.
    pub fn new(writer: &'a mut dyn Write, max_level: Level) -> Logger<'a> {
        Logger { writer, max_level }
    }

    /// Logs a message built from given format arguments.
    pub fn log_fmt(&mut self, level: Level, args: fmt::Arguments) {
        if level > self.max_level {
            return;
        }

        // TODO: Support timestamps and locking.

        let prefix = match level {
            Level::Fatal => "F ",
            Level::Error => "E ",
            Level::Warning => "W ",
            Level::Info => "i ",
            Level::Debug => "d ",
        };

        self.writer.write_str(prefix).unwrap();
        self.writer.write_fmt(args).unwrap();
        self.writer.write_str("\n").unwrap();

        if level == Level::Fatal {
            panic!("fatal condition triggered by logger");
        }
    }
}

static mut LOGGER: Option<Logger<'static>> = None;

/// Creates a default logger to be used by the logging macros.
pub fn init(writer: &'static mut dyn Write, max_level: Level) {
    unsafe {
        LOGGER = Some(Logger::new(writer, max_level));
    }
}

/// Logs a message using the default logger.
pub fn log_fmt(level: Level, args: fmt::Arguments) {
    unsafe {
        LOGGER.as_mut().unwrap().log_fmt(level, args);
    }
}

/// Logs a fatal message and panics.
macro_rules! fatal {
    ($($arg:tt)*) => ({
        use log;
        log::log_fmt(log::Level::Fatal, format_args!($($arg)*));
    })
}

/// Logs an error message.
macro_rules! error {
    ($($arg:tt)*) => ({
        use log;
        log::log_fmt(log::Level::Error, format_args!($($arg)*));
    })
}

/// Logs a warning message.
macro_rules! warning {
    ($($arg:tt)*) => ({
        use log;
        log::log_fmt(log::Level::Warning, format_args!($($arg)*));
    })
}

/// Logs an info message.
macro_rules! info {
    ($($arg:tt)*) => ({
        use log;
        log::log_fmt(log::Level::Info, format_args!($($arg)*));
    })
}

/// Logs a debug message.
macro_rules! debug {
    ($($arg:tt)*) => ({
        use log;
        log::log_fmt(log::Level::Debug, format_args!($($arg)*));
    })
}
