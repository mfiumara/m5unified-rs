use core::ffi::c_int;
use std::ffi::CString;

use crate::Error;

#[derive(Debug)]
pub struct Log;

impl Log {
    pub fn println(&self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_println(text.as_ptr()) }
        Ok(())
    }

    pub fn print(&self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_print(text.as_ptr()) }
        Ok(())
    }

    pub fn log(&self, level: LogLevel, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_level(level as c_int, text.as_ptr()) }
        Ok(())
    }

    pub fn set_enable_color(&self, target: LogTarget, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_log_set_enable_color(target as c_int, enable) }
    }

    pub fn enable_color(&self, target: LogTarget) -> bool {
        unsafe { m5unified_sys::m5u_log_get_enable_color(target as c_int) }
    }

    pub fn set_log_level(&self, target: LogTarget, level: LogLevel) -> bool {
        unsafe { m5unified_sys::m5u_log_set_level(target as c_int, level as c_int) }
    }

    pub fn log_level(&self, target: LogTarget) -> Option<LogLevel> {
        LogLevel::from_raw(unsafe { m5unified_sys::m5u_log_get_level(target as c_int) })
    }

    pub fn set_suffix(&self, target: LogTarget, suffix: &str) -> Result<bool, Error> {
        let suffix = CString::new(suffix).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_log_set_suffix(target as c_int, suffix.as_ptr()) })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogLevel {
    None = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Verbose = 5,
}

impl LogLevel {
    fn from_raw(value: c_int) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Error),
            2 => Some(Self::Warn),
            3 => Some(Self::Info),
            4 => Some(Self::Debug),
            5 => Some(Self::Verbose),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogTarget {
    Serial = 0,
    Display = 1,
    Callback = 2,
}
