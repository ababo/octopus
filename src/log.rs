#![allow(dead_code)]
#![allow(unused_macros)]
use core::fmt::{self, Write};

#[derive(PartialEq, PartialOrd)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

fn write_nothing(_: &str) {}

static mut WRITE: fn(&str) = write_nothing;
static mut LEVEL: Level = Level::Info;

pub fn init(write: fn(&str), level: Level) {
    unsafe {
        WRITE = write;
        LEVEL = level;
    }
}

struct Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            WRITE(s);
        }
        Ok(())
    }
}

pub fn log(level: Level, args: fmt::Arguments) {
    unsafe {
        if level < LEVEL {
            return;
        }

        let mut writer = Writer {};
        let prefix = match level {
            Level::Debug => "d ",
            Level::Info => "i ",
            Level::Warning => "W ",
            Level::Error => "E ",
            Level::Fatal => "F ",
        };

        // TODO: Support locking.
        writer.write_str(prefix).unwrap();
        writer.write_fmt(args).unwrap();
        writer.write_str("\n").unwrap();
    }

    if level == Level::Fatal {
        panic!("fatal event logged")
    }
}

macro_rules! log_debug {
    ($($arg:tt)*) => ({
        use log;
        log::log(log::Level::Debug, format_args!($($arg)*));
    })
}

macro_rules! log_info {
    ($($arg:tt)*) => ({
        use log;
        log::log(log::Level::Info, format_args!($($arg)*));
    })
}

macro_rules! log_warning {
    ($($arg:tt)*) => ({
        use log;
        log::log(log::Level::Warning, format_args!($($arg)*));
    })
}

macro_rules! log_error {
    ($($arg:tt)*) => ({
        use log;
        log::log(log::Level::Error, format_args!($($arg)*));
    })
}

macro_rules! log_fatal {
    ($($arg:tt)*) => ({
        use log;
        log::log(log::Level::Fatal, format_args!($($arg)*));
    })
}
