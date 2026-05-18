//! M5Unified logging helpers.
//!
//! The logger supports string output, log levels, targets, suffixes, color
//! toggles, byte dumps, and raw callbacks.

use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};

use crate::Error;

#[derive(Debug)]
pub struct Log;

impl Log {
    pub fn println(&self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_println(text.as_ptr()) }
        Ok(())
    }

    pub fn println_empty(&self) {
        unsafe { m5unified_sys::m5u_log_println_empty() }
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

    pub fn dump(&self, data: &[u8], level: LogLevel) {
        unsafe {
            m5unified_sys::m5u_log_dump(
                data.as_ptr().cast::<c_void>(),
                data.len().min(u32::MAX as usize) as u32,
                level as c_int,
            )
        }
    }

    pub fn path_to_file_name(path: &str) -> Result<String, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        let file_name = unsafe { m5unified_sys::m5u_log_path_to_file_name(path.as_ptr()) };
        if file_name.is_null() {
            return Ok(String::new());
        }
        Ok(unsafe { CStr::from_ptr(file_name) }
            .to_string_lossy()
            .into_owned())
    }

    /// # Safety
    ///
    /// The callback is invoked by M5Unified's logging machinery with a borrowed
    /// C string that is only valid for the duration of the call. `user_data`
    /// must remain valid for as long as the callback is registered, and the
    /// callback must be safe to call from the task/thread that emits the log.
    pub unsafe fn set_raw_callback(
        &self,
        callback: Option<RawLogCallback>,
        user_data: *mut c_void,
    ) -> bool {
        unsafe { m5unified_sys::m5u_log_set_callback(callback, user_data) }
    }

    pub fn clear_callback(&self) -> bool {
        unsafe { m5unified_sys::m5u_log_set_callback(None, core::ptr::null_mut()) }
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

pub type RawLogCallback = unsafe extern "C" fn(
    level: c_int,
    use_color: bool,
    text: *const c_char,
    user_data: *mut c_void,
);
