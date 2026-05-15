//! Safe wrapper for M5Unified.
//!
//! The public API is intentionally tiny while the FFI layer is proven on real
//! M5StickS3 hardware.

use core::ffi::c_int;
use std::ffi::CString;

/// Errors returned by the high-level wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// M5Unified initialization failed.
    BeginFailed,
    /// The provided string contained an interior NUL byte.
    InvalidString,
}

/// Top-level handle for M5Unified-backed board features.
#[derive(Debug)]
pub struct M5Unified {
    pub display: Display,
    pub buttons: Buttons,
    pub mic: Mic,
    pub power: Power,
}

impl M5Unified {
    /// Initialize M5Unified and return a board handle.
    pub fn begin() -> Result<Self, Error> {
        let ok = unsafe { m5unified_sys::m5u_begin() };
        if !ok {
            return Err(Error::BeginFailed);
        }

        Ok(Self {
            display: Display,
            buttons: Buttons,
            mic: Mic,
            power: Power,
        })
    }

    /// Poll/update M5Unified internals, including button edge state.
    pub fn update(&mut self) {
        unsafe { m5unified_sys::m5u_update() }
    }
}

#[derive(Debug)]
pub struct Display;

impl Display {
    pub fn fill_screen(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_screen(color) }
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_set_cursor(x as c_int, y as c_int) }
    }

    pub fn print(&mut self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_display_print(text.as_ptr()) }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Buttons;

impl Buttons {
    pub fn a_is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_a_is_pressed() }
    }

    pub fn b_was_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_b_was_pressed() }
    }
}

#[derive(Debug)]
pub struct Mic;

impl Mic {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_mic_begin() }
    }

    pub fn record_i16(&mut self, buffer: &mut [i16]) -> bool {
        unsafe { m5unified_sys::m5u_mic_record_i16(buffer.as_mut_ptr(), buffer.len()) }
    }
}

#[derive(Debug)]
pub struct Power;

impl Power {
    pub fn battery_level(&self) -> Option<u8> {
        battery_level_from_raw(unsafe { m5unified_sys::m5u_battery_level() })
    }
}

fn battery_level_from_raw(level: c_int) -> Option<u8> {
    if (0..=100).contains(&level) {
        Some(level as u8)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battery_level_accepts_percent_range() {
        assert_eq!(battery_level_from_raw(0), Some(0));
        assert_eq!(battery_level_from_raw(42), Some(42));
        assert_eq!(battery_level_from_raw(100), Some(100));
    }

    #[test]
    fn battery_level_rejects_out_of_range_values() {
        assert_eq!(battery_level_from_raw(-1), None);
        assert_eq!(battery_level_from_raw(101), None);
    }

    #[test]
    fn display_print_rejects_interior_nul() {
        let mut display = Display;
        assert_eq!(display.print("hello\0world"), Err(Error::InvalidString));
    }
}
