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
    Pwr,
    Ext,
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

    pub fn pwr(&self) -> Button {
        self.button(ButtonId::Pwr)
    }

    pub fn ext(&self) -> Button {
        self.button(ButtonId::Ext)
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
    fn raw_id(&self) -> c_int {
        match self.id {
            ButtonId::A => 0,
            ButtonId::B => 1,
            ButtonId::C => 2,
            ButtonId::Pwr => 3,
            ButtonId::Ext => 4,
        }
    }

    pub fn is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_pressed(self.raw_id()) }
    }

    pub fn was_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_pressed(self.raw_id()) }
    }

    pub fn was_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_released(self.raw_id()) }
    }

    pub fn was_clicked(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_clicked(self.raw_id()) }
    }

    pub fn was_hold(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_hold(self.raw_id()) }
    }

    pub fn is_holding(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_holding(self.raw_id()) }
    }

    pub fn was_decide_click_count(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_decide_click_count(self.raw_id()) }
    }

    pub fn click_count(&self) -> i32 {
        unsafe { m5unified_sys::m5u_button_get_click_count(self.raw_id()) as i32 }
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


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color565(pub u16);

impl Color565 {
    pub const fn new(raw: u16) -> Self { Self(raw) }
    pub fn rgb888(r: u8, g: u8, b: u8) -> Self {
        Self(unsafe { m5unified_sys::m5u_display_color888(r, g, b) })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point { pub x: i32, pub y: i32 }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size { pub w: i32, pub h: i32 }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect { pub x: i32, pub y: i32, pub w: i32, pub h: i32 }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextDatum {
    TopLeft = 0,
    TopCenter = 1,
    TopRight = 2,
    MiddleLeft = 4,
    MiddleCenter = 5,
    MiddleRight = 6,
    BottomLeft = 8,
    BottomCenter = 9,
    BottomRight = 10,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DisplayKind {
    ModuleDisplay,
    AtomDisplay,
    ModuleRca,
    UnitGlass,
    UnitGlass2,
    UnitOled,
    UnitMiniOled,
    UnitLcd,
    UnitRca,
    Raw(i32),
}

impl DisplayKind {
    fn raw(self) -> i32 {
        match self {
            // These values intentionally remain best-effort. M5Unified's enum is
            // version-dependent; callers can use Raw for exact board IDs.
            Self::ModuleDisplay => 0,
            Self::AtomDisplay => 1,
            Self::ModuleRca => 2,
            Self::UnitGlass => 3,
            Self::UnitGlass2 => 4,
            Self::UnitOled => 5,
            Self::UnitMiniOled => 6,
            Self::UnitLcd => 7,
            Self::UnitRca => 8,
            Self::Raw(value) => value,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DisplayRef { index: i32 }

impl DisplayRef {
    pub fn width(&self) -> i32 { unsafe { m5unified_sys::m5u_display_width_at(self.index) as i32 } }
    pub fn height(&self) -> i32 { unsafe { m5unified_sys::m5u_display_height_at(self.index) as i32 } }
    pub fn print(&mut self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_display_print_at(self.index, text.as_ptr()) }
        Ok(())
    }
    pub fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_circle_at(self.index, x, y, r, color) }
    }
}

impl M5Unified {
    pub fn display_count(&self) -> usize {
        unsafe { m5unified_sys::m5u_display_count().max(0) as usize }
    }

    pub fn display(&self, index: usize) -> Option<DisplayRef> {
        (index < self.display_count()).then_some(DisplayRef { index: index as i32 })
    }

    pub fn display_index(&self, kind: DisplayKind) -> Option<usize> {
        let index = unsafe { m5unified_sys::m5u_display_index_for_kind(kind.raw() as c_int) };
        (index >= 0).then_some(index as usize)
    }
}

impl Display {
    pub fn rotation(&self) -> i32 { unsafe { m5unified_sys::m5u_display_get_rotation() as i32 } }
    pub fn set_brightness(&mut self, brightness: u8) { unsafe { m5unified_sys::m5u_display_set_brightness(brightness) } }
    pub fn set_epd_fastest(&mut self) { unsafe { m5unified_sys::m5u_display_set_epd_fastest() } }
    pub fn start_write(&mut self) { unsafe { m5unified_sys::m5u_display_start_write() } }
    pub fn end_write(&mut self) { unsafe { m5unified_sys::m5u_display_end_write() } }
    pub fn transaction<R>(&mut self, f: impl FnOnce(&mut Display) -> R) -> R {
        self.start_write();
        let result = f(self);
        self.end_write();
        result
    }
    pub fn display(&mut self) { unsafe { m5unified_sys::m5u_display_display() } }
    pub fn display_busy(&self) -> bool { unsafe { m5unified_sys::m5u_display_display_busy() } }
    pub fn wait_display(&self) { unsafe { m5unified_sys::m5u_display_wait_display() } }
    pub fn cursor_y(&self) -> i32 { unsafe { m5unified_sys::m5u_display_get_cursor_y() as i32 } }
    pub fn font_height(&self) -> i32 { unsafe { m5unified_sys::m5u_display_font_height() as i32 } }
    pub fn base_color(&self) -> u16 { unsafe { m5unified_sys::m5u_display_get_base_color() } }
    pub fn set_color(&mut self, color: u16) { unsafe { m5unified_sys::m5u_display_set_color(color) } }
    pub fn set_text_wrap(&mut self, wrap_x: bool, wrap_y: bool) { unsafe { m5unified_sys::m5u_display_set_text_wrap(wrap_x, wrap_y) } }
    pub fn set_text_datum(&mut self, datum: TextDatum) { unsafe { m5unified_sys::m5u_display_set_text_datum(datum as c_int) } }
    pub fn draw_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_draw_string(text.as_ptr(), x, y) as i32 })
    }
    pub fn write_pixel(&mut self, x: i32, y: i32, color: u16) { unsafe { m5unified_sys::m5u_display_write_pixel(x, y, color) } }
    pub fn write_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) { unsafe { m5unified_sys::m5u_display_write_fast_vline(x, y, h, color) } }
    pub fn set_clip_rect(&mut self, rect: Rect) { unsafe { m5unified_sys::m5u_display_set_clip_rect(rect.x, rect.y, rect.w, rect.h) } }
    pub fn clear_clip_rect(&mut self) { unsafe { m5unified_sys::m5u_display_clear_clip_rect() } }
    pub fn color888(&self, r: u8, g: u8, b: u8) -> u16 { unsafe { m5unified_sys::m5u_display_color888(r, g, b) } }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct MicConfig { pub noise_filter_level: i32 }

impl Mic {
    pub fn is_enabled(&self) -> bool { unsafe { m5unified_sys::m5u_mic_is_enabled() } }
    pub fn is_recording(&self) -> bool { unsafe { m5unified_sys::m5u_mic_is_recording() } }
    pub fn end(&mut self) { unsafe { m5unified_sys::m5u_mic_end() } }
    pub fn record_i16_at(&mut self, buffer: &mut [i16], sample_rate_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_mic_record_i16_at(buffer.as_mut_ptr(), buffer.len(), sample_rate_hz) }
    }
    pub fn config(&self) -> MicConfig {
        MicConfig { noise_filter_level: unsafe { m5unified_sys::m5u_mic_get_noise_filter_level() as i32 } }
    }
    pub fn set_config(&mut self, config: MicConfig) -> Result<(), Error> {
        unsafe { m5unified_sys::m5u_mic_set_noise_filter_level(config.noise_filter_level as c_int) }
            .then_some(())
            .ok_or(Error::Unavailable("microphone config"))
    }
}

impl Speaker {
    pub fn is_enabled(&self) -> bool { unsafe { m5unified_sys::m5u_speaker_is_enabled() } }
    pub fn end(&mut self) { unsafe { m5unified_sys::m5u_speaker_end() } }
    pub fn volume(&self) -> u8 { unsafe { m5unified_sys::m5u_speaker_get_volume() } }
    pub fn tone_ex(&mut self, frequency_hz: f32, duration_ms: u32, channel: Option<u8>) -> bool {
        unsafe { m5unified_sys::m5u_speaker_tone_ex(frequency_hz, duration_ms, channel.map(i32::from).unwrap_or(-1)) }
    }
    pub fn play_u8(&mut self, samples: &[u8], sample_rate_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_speaker_play_u8(samples.as_ptr(), samples.len(), sample_rate_hz) }
    }
    pub fn play_wav(&mut self, data: &[u8]) -> bool {
        unsafe { m5unified_sys::m5u_speaker_play_wav(data.as_ptr(), data.len()) }
    }
    pub fn is_playing(&self, channel: Option<u8>) -> bool {
        unsafe { m5unified_sys::m5u_speaker_is_playing(channel.map(i32::from).unwrap_or(-1)) }
    }
    pub fn stop(&mut self, channel: Option<u8>) { unsafe { m5unified_sys::m5u_speaker_stop(channel.map(i32::from).unwrap_or(-1)) } }
    pub fn channel_volume(&self, channel: u8) -> u8 { unsafe { m5unified_sys::m5u_speaker_get_channel_volume(i32::from(channel)) } }
    pub fn set_channel_volume(&mut self, channel: u8, volume: u8) { unsafe { m5unified_sys::m5u_speaker_set_channel_volume(i32::from(channel), volume) } }
    pub fn set_all_channel_volume(&mut self, volume: u8) { unsafe { m5unified_sys::m5u_speaker_set_all_channel_volume(volume) } }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImuKind { Unknown(i32) }

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ImuData { pub accel: Vec3, pub gyro: Vec3, pub temperature_c: Option<f32> }

impl Imu {
    pub fn is_enabled(&self) -> bool { unsafe { m5unified_sys::m5u_imu_is_enabled() } }
    pub fn kind(&self) -> ImuKind { ImuKind::Unknown(unsafe { m5unified_sys::m5u_imu_get_type() as i32 }) }
    pub fn update(&mut self) -> bool { unsafe { m5unified_sys::m5u_imu_update() } }
    pub fn data(&self) -> Option<ImuData> {
        Some(ImuData { accel: self.accel()?, gyro: self.gyro()?, temperature_c: self.temperature_c() })
    }
    pub fn load_offset_from_nvs(&mut self) -> bool { unsafe { m5unified_sys::m5u_imu_load_offset_from_nvs() } }
    pub fn save_offset_to_nvs(&mut self) -> bool { unsafe { m5unified_sys::m5u_imu_save_offset_to_nvs() } }
    pub fn offset_data(&self, index: i32) -> f32 { unsafe { m5unified_sys::m5u_imu_get_offset_data(index) } }
    pub fn set_calibration(&mut self, x: f32, y: f32, z: f32) { unsafe { m5unified_sys::m5u_imu_set_calibration(x, y, z) } }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct TouchDetail {
    pub x: i32,
    pub y: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    pub is_pressed: bool,
    pub was_pressed: bool,
    pub was_released: bool,
    pub was_clicked: bool,
    pub was_hold: bool,
    pub is_holding: bool,
    pub click_count: i32,
}

impl TouchDetail {
    pub fn delta(&self) -> (i32, i32) { (self.x - self.prev_x, self.y - self.prev_y) }
}

impl Touch {
    pub fn detail(&self, index: usize) -> Option<TouchDetail> {
        let mut raw = m5unified_sys::m5u_touch_detail_t::default();
        let ok = unsafe { m5unified_sys::m5u_touch_get_detail(index as c_int, &mut raw) };
        ok.then_some(TouchDetail {
            x: raw.x, y: raw.y, prev_x: raw.prev_x, prev_y: raw.prev_y,
            is_pressed: raw.is_pressed, was_pressed: raw.was_pressed,
            was_released: raw.was_released, was_clicked: raw.was_clicked,
            was_hold: raw.was_hold, is_holding: raw.is_holding,
            click_count: raw.click_count,
        })
    }
}

impl Rtc {
    pub fn is_enabled(&self) -> bool { unsafe { m5unified_sys::m5u_rtc_is_enabled() } }
}

#[derive(Debug)]
pub struct Axp2101;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Axp2101IrqStatus { pub raw: u64 }

impl Axp2101IrqStatus {
    pub fn battery_charger_under_temperature(&self) -> bool { unsafe { m5unified_sys::m5u_power_axp2101_is_bat_charger_under_temperature_irq() } }
    pub fn battery_charger_over_temperature(&self) -> bool { unsafe { m5unified_sys::m5u_power_axp2101_is_bat_charger_over_temperature_irq() } }
    pub fn vbus_insert(&self) -> bool { unsafe { m5unified_sys::m5u_power_axp2101_is_vbus_insert_irq() } }
    pub fn vbus_remove(&self) -> bool { unsafe { m5unified_sys::m5u_power_axp2101_is_vbus_remove_irq() } }
}

impl Power {
    pub fn axp2101(&self) -> Axp2101 { Axp2101 }
}

impl Axp2101 {
    pub const IRQ_ALL: u64 = u64::MAX;
    pub fn disable_irq(&self, mask: u64) -> bool { unsafe { m5unified_sys::m5u_power_axp2101_disable_irq(mask) } }
    pub fn enable_irq(&self, mask: u64) -> bool { unsafe { m5unified_sys::m5u_power_axp2101_enable_irq(mask) } }
    pub fn clear_irq_statuses(&self) -> bool { unsafe { m5unified_sys::m5u_power_axp2101_clear_irq_statuses() } }
    pub fn irq_statuses(&self) -> Axp2101IrqStatus { Axp2101IrqStatus { raw: unsafe { m5unified_sys::m5u_power_axp2101_get_irq_statuses() } } }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogLevel { Error = 1, Warn = 2, Info = 3, Debug = 4, Verbose = 5 }

impl Log {
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
