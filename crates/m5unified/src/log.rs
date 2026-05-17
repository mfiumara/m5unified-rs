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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Verbose = 5,
}
