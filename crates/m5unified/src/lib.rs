//! Safe Rust wrapper for a small M5Unified C ABI surface.
//!
//! The API is intentionally shaped around M5Unified's common examples while
//! keeping Rust call sites safe and host-checkable. Hardware calls are provided
//! by `m5unified-sys`; on non-ESP-IDF targets that crate supplies no-op stubs so
//! examples compile in CI.

use core::ffi::c_int;
use std::ffi::CString;
use std::fmt;

/// Common RGB565 color constants used by the translated examples.
pub mod colors {
    pub const BLACK: u16 = 0x0000;
    pub const NAVY: u16 = 0x000F;
    pub const DARK_GREEN: u16 = 0x03E0;
    pub const DARK_CYAN: u16 = 0x03EF;
    pub const MAROON: u16 = 0x7800;
    pub const PURPLE: u16 = 0x780F;
    pub const OLIVE: u16 = 0x7BE0;
    pub const LIGHT_GREY: u16 = 0xC618;
    pub const DARK_GREY: u16 = 0x7BEF;
    pub const BLUE: u16 = 0x001F;
    pub const GREEN: u16 = 0x07E0;
    pub const CYAN: u16 = 0x07FF;
    pub const RED: u16 = 0xF800;
    pub const MAGENTA: u16 = 0xF81F;
    pub const YELLOW: u16 = 0xFFE0;
    pub const WHITE: u16 = 0xFFFF;
    pub const ORANGE: u16 = 0xFD20;
    pub const GREEN_YELLOW: u16 = 0xAFE5;
    pub const PINK: u16 = 0xF81F;
}

/// Errors returned by the high-level wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// M5Unified initialization failed.
    BeginFailed,
    /// The provided string contained an interior NUL byte.
    InvalidString,
    /// Requested operation is not available on this board/build.
    Unavailable(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BeginFailed => f.write_str("M5Unified initialization failed"),
            Self::InvalidString => f.write_str("string contains an interior NUL byte"),
            Self::Unavailable(feature) => write!(f, "M5Unified feature unavailable: {feature}"),
        }
    }
}

impl std::error::Error for Error {}

/// Top-level handle for M5Unified-backed board features.
#[derive(Debug)]
pub struct M5Unified {
    pub display: Display,
    pub buttons: Buttons,
    pub mic: Mic,
    pub speaker: Speaker,
    pub imu: Imu,
    pub touch: Touch,
    pub rtc: Rtc,
    pub power: Power,
    pub log: Log,
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
            speaker: Speaker,
            imu: Imu,
            touch: Touch,
            rtc: Rtc,
            power: Power,
            log: Log,
        })
    }

    /// Poll/update M5Unified internals, including button edge state.
    pub fn update(&mut self) {
        unsafe { m5unified_sys::m5u_update() }
    }

    /// Delay execution. On host builds this is currently a no-op.
    pub fn delay_ms(&self, ms: u32) {
        unsafe { m5unified_sys::m5u_delay_ms(ms) }
    }
}

#[derive(Debug)]
pub struct Display;

impl Display {
    pub fn width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_width() as i32 }
    }

    pub fn height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_height() as i32 }
    }

    pub fn clear(&mut self) {
        self.fill_screen(colors::BLACK);
    }

    pub fn fill_screen(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_screen(color) }
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_set_cursor(x as c_int, y as c_int) }
    }

    pub fn set_text_size(&mut self, size: i32) {
        unsafe { m5unified_sys::m5u_display_set_text_size(size as c_int) }
    }

    pub fn set_text_color(&mut self, fg: u16, bg: u16) {
        unsafe { m5unified_sys::m5u_display_set_text_color(fg, bg) }
    }

    pub fn print(&mut self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_display_print(text.as_ptr()) }
        Ok(())
    }

    pub fn println(&mut self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_display_println(text.as_ptr()) }
        Ok(())
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_line(x0, y0, x1, y1, color) }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_rect(x, y, w, h, color) }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_rect(x, y, w, h, color) }
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_circle(x, y, r, color) }
    }

    pub fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_circle(x, y, r, color) }
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        unsafe { m5unified_sys::m5u_display_set_rotation(rotation) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonId {
    A,
    B,
    C,
}

#[derive(Debug)]
pub struct Buttons;

impl Buttons {
    pub fn button(&self, id: ButtonId) -> Button {
        Button { id }
    }

    pub fn a(&self) -> Button {
        self.button(ButtonId::A)
    }

    pub fn b(&self) -> Button {
        self.button(ButtonId::B)
    }

    pub fn c(&self) -> Button {
        self.button(ButtonId::C)
    }

    pub fn a_is_pressed(&self) -> bool {
        self.a().is_pressed()
    }

    pub fn b_was_pressed(&self) -> bool {
        self.b().was_pressed()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Button {
    id: ButtonId,
}

impl Button {
    pub fn is_pressed(&self) -> bool {
        unsafe {
            match self.id {
                ButtonId::A => m5unified_sys::m5u_btn_a_is_pressed(),
                ButtonId::B => m5unified_sys::m5u_btn_b_is_pressed(),
                ButtonId::C => m5unified_sys::m5u_btn_c_is_pressed(),
            }
        }
    }

    pub fn was_pressed(&self) -> bool {
        unsafe {
            match self.id {
                ButtonId::A => m5unified_sys::m5u_btn_a_was_pressed(),
                ButtonId::B => m5unified_sys::m5u_btn_b_was_pressed(),
                ButtonId::C => m5unified_sys::m5u_btn_c_was_pressed(),
            }
        }
    }

    pub fn was_released(&self) -> bool {
        unsafe {
            match self.id {
                ButtonId::A => m5unified_sys::m5u_btn_a_was_released(),
                ButtonId::B => m5unified_sys::m5u_btn_b_was_released(),
                ButtonId::C => m5unified_sys::m5u_btn_c_was_released(),
            }
        }
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

    pub fn rms(&mut self, buffer: &mut [i16]) -> Option<f32> {
        if !self.record_i16(buffer) || buffer.is_empty() {
            return None;
        }
        let sum_sq: f32 = buffer.iter().map(|&s| (s as f32) * (s as f32)).sum();
        Some((sum_sq / buffer.len() as f32).sqrt())
    }
}

#[derive(Debug)]
pub struct Speaker;

impl Speaker {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_speaker_begin() }
    }

    pub fn set_volume(&mut self, volume: u8) {
        unsafe { m5unified_sys::m5u_speaker_set_volume(volume) }
    }

    pub fn tone(&mut self, frequency_hz: u32, duration_ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_speaker_tone(frequency_hz, duration_ms) }
    }

    pub fn play_i16(&mut self, samples: &[i16], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_i16(samples.as_ptr(), samples.len(), sample_rate_hz)
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Imu;

impl Imu {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_begin() }
    }

    pub fn accel(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_accel(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn gyro(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_gyro(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn temperature_c(&self) -> Option<f32> {
        let mut temp = 0.0;
        let ok = unsafe { m5unified_sys::m5u_imu_get_temp_c(&mut temp) };
        ok.then_some(temp)
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct TouchPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Touch;

impl Touch {
    pub fn points(&self) -> Vec<TouchPoint> {
        let count = unsafe { m5unified_sys::m5u_touch_count() }.max(0) as usize;
        (0..count)
            .filter_map(|index| {
                let (mut x, mut y) = (0, 0);
                let ok = unsafe { m5unified_sys::m5u_touch_get(index as c_int, &mut x, &mut y) };
                ok.then_some(TouchPoint { x, y })
            })
            .collect()
    }

    pub fn is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_touch_count() > 0 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

#[derive(Debug)]
pub struct Rtc;

impl Rtc {
    pub fn get_datetime(&self) -> Option<DateTime> {
        let (mut year, mut month, mut day, mut hour, mut minute, mut second) = (0, 0, 0, 0, 0, 0);
        let ok = unsafe {
            m5unified_sys::m5u_rtc_get_datetime(
                &mut year,
                &mut month,
                &mut day,
                &mut hour,
                &mut minute,
                &mut second,
            )
        };
        ok.then_some(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    pub fn set_datetime(&mut self, datetime: DateTime) -> bool {
        unsafe {
            m5unified_sys::m5u_rtc_set_datetime(
                datetime.year,
                datetime.month,
                datetime.day,
                datetime.hour,
                datetime.minute,
                datetime.second,
            )
        }
    }
}

#[derive(Debug)]
pub struct Power;

impl Power {
    pub fn battery_level(&self) -> Option<u8> {
        let level = unsafe { m5unified_sys::m5u_battery_level() };
        if (0..=100).contains(&level) {
            Some(level as u8)
        } else {
            None
        }
    }

    pub fn battery_voltage_mv(&self) -> Option<u16> {
        let mv = unsafe { m5unified_sys::m5u_battery_voltage_mv() };
        (mv >= 0).then_some(mv as u16)
    }

    pub fn is_charging(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_is_charging() }
    }
}

#[derive(Debug)]
pub struct Log;

impl Log {
    pub fn println(&self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_println(text.as_ptr()) }
        Ok(())
    }
}

pub fn sd_begin() -> bool {
    unsafe { m5unified_sys::m5u_sd_begin() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_dimensions_are_available_on_host_stubs() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(m5.display.width() > 0);
        assert!(m5.display.height() > 0);
    }

    #[test]
    fn invalid_strings_are_rejected_before_ffi() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert_eq!(m5.display.print("bad\0string"), Err(Error::InvalidString));
    }

    #[test]
    fn mic_rms_uses_recorded_buffer() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        let mut buffer = [0_i16; 8];
        assert_eq!(m5.mic.rms(&mut buffer), Some(0.0));
    }
}
