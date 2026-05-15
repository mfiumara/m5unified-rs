#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

//! Raw bindings for a small C ABI shim over M5Unified.
//!
//! This crate intentionally does not bind M5Unified's C++ classes directly.
//! Instead, `native/m5u_shim.cpp` exposes stable `extern "C"` functions that
//! Rust can call from the higher-level `m5unified` crate. On non-ESP-IDF host
//! targets these functions are stubbed so the safe wrapper and translated
//! samples can be checked in CI without hardware.

use core::ffi::{c_char, c_int};

#[cfg(target_os = "espidf")]
extern "C" {
    pub fn m5u_begin() -> bool;
    pub fn m5u_update();
    pub fn m5u_delay_ms(ms: u32);

    pub fn m5u_display_width() -> c_int;
    pub fn m5u_display_height() -> c_int;
    pub fn m5u_display_fill_screen(color: u16);
    pub fn m5u_display_set_cursor(x: c_int, y: c_int);
    pub fn m5u_display_set_text_size(size: c_int);
    pub fn m5u_display_set_text_color(fg: u16, bg: u16);
    pub fn m5u_display_print(text: *const c_char);
    pub fn m5u_display_println(text: *const c_char);
    pub fn m5u_display_draw_line(x0: c_int, y0: c_int, x1: c_int, y1: c_int, color: u16);
    pub fn m5u_display_draw_rect(x: c_int, y: c_int, w: c_int, h: c_int, color: u16);
    pub fn m5u_display_fill_rect(x: c_int, y: c_int, w: c_int, h: c_int, color: u16);
    pub fn m5u_display_draw_circle(x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_display_fill_circle(x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_display_set_rotation(rotation: c_int);

    pub fn m5u_btn_a_is_pressed() -> bool;
    pub fn m5u_btn_a_was_pressed() -> bool;
    pub fn m5u_btn_a_was_released() -> bool;
    pub fn m5u_btn_b_is_pressed() -> bool;
    pub fn m5u_btn_b_was_pressed() -> bool;
    pub fn m5u_btn_b_was_released() -> bool;
    pub fn m5u_btn_c_is_pressed() -> bool;
    pub fn m5u_btn_c_was_pressed() -> bool;
    pub fn m5u_btn_c_was_released() -> bool;

    pub fn m5u_mic_begin() -> bool;
    pub fn m5u_mic_record_i16(buffer: *mut i16, samples: usize) -> bool;
    pub fn m5u_speaker_begin() -> bool;
    pub fn m5u_speaker_set_volume(volume: u8);
    pub fn m5u_speaker_tone(frequency_hz: u32, duration_ms: u32) -> bool;
    pub fn m5u_speaker_play_i16(samples: *const i16, len: usize, sample_rate_hz: u32) -> bool;

    pub fn m5u_imu_begin() -> bool;
    pub fn m5u_imu_get_accel(x: *mut f32, y: *mut f32, z: *mut f32) -> bool;
    pub fn m5u_imu_get_gyro(x: *mut f32, y: *mut f32, z: *mut f32) -> bool;
    pub fn m5u_imu_get_temp_c(temp: *mut f32) -> bool;

    pub fn m5u_touch_count() -> c_int;
    pub fn m5u_touch_get(index: c_int, x: *mut c_int, y: *mut c_int) -> bool;

    pub fn m5u_rtc_get_datetime(
        year: *mut c_int,
        month: *mut c_int,
        day: *mut c_int,
        hour: *mut c_int,
        minute: *mut c_int,
        second: *mut c_int,
    ) -> bool;
    pub fn m5u_rtc_set_datetime(
        year: c_int,
        month: c_int,
        day: c_int,
        hour: c_int,
        minute: c_int,
        second: c_int,
    ) -> bool;

    pub fn m5u_battery_level() -> c_int;
    pub fn m5u_battery_voltage_mv() -> c_int;
    pub fn m5u_power_is_charging() -> bool;

    pub fn m5u_log_println(text: *const c_char);
    pub fn m5u_sd_begin() -> bool;
}

#[cfg(not(target_os = "espidf"))]
mod host_stubs {
    use super::*;
    use core::ptr;

    pub unsafe fn m5u_begin() -> bool {
        true
    }
    pub unsafe fn m5u_update() {}
    pub unsafe fn m5u_delay_ms(_ms: u32) {}

    pub unsafe fn m5u_display_width() -> c_int {
        320
    }
    pub unsafe fn m5u_display_height() -> c_int {
        240
    }
    pub unsafe fn m5u_display_fill_screen(_color: u16) {}
    pub unsafe fn m5u_display_set_cursor(_x: c_int, _y: c_int) {}
    pub unsafe fn m5u_display_set_text_size(_size: c_int) {}
    pub unsafe fn m5u_display_set_text_color(_fg: u16, _bg: u16) {}
    pub unsafe fn m5u_display_print(_text: *const c_char) {}
    pub unsafe fn m5u_display_println(_text: *const c_char) {}
    pub unsafe fn m5u_display_draw_line(
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_rect(_x: c_int, _y: c_int, _w: c_int, _h: c_int, _color: u16) {}
    pub unsafe fn m5u_display_fill_rect(_x: c_int, _y: c_int, _w: c_int, _h: c_int, _color: u16) {}
    pub unsafe fn m5u_display_draw_circle(_x: c_int, _y: c_int, _r: c_int, _color: u16) {}
    pub unsafe fn m5u_display_fill_circle(_x: c_int, _y: c_int, _r: c_int, _color: u16) {}
    pub unsafe fn m5u_display_set_rotation(_rotation: c_int) {}

    pub unsafe fn m5u_btn_a_is_pressed() -> bool {
        false
    }
    pub unsafe fn m5u_btn_a_was_pressed() -> bool {
        false
    }
    pub unsafe fn m5u_btn_a_was_released() -> bool {
        false
    }
    pub unsafe fn m5u_btn_b_is_pressed() -> bool {
        false
    }
    pub unsafe fn m5u_btn_b_was_pressed() -> bool {
        false
    }
    pub unsafe fn m5u_btn_b_was_released() -> bool {
        false
    }
    pub unsafe fn m5u_btn_c_is_pressed() -> bool {
        false
    }
    pub unsafe fn m5u_btn_c_was_pressed() -> bool {
        false
    }
    pub unsafe fn m5u_btn_c_was_released() -> bool {
        false
    }

    pub unsafe fn m5u_mic_begin() -> bool {
        true
    }
    pub unsafe fn m5u_mic_record_i16(buffer: *mut i16, samples: usize) -> bool {
        if !buffer.is_null() {
            for i in 0..samples {
                ptr::write(buffer.add(i), 0);
            }
        }
        true
    }
    pub unsafe fn m5u_speaker_begin() -> bool {
        true
    }
    pub unsafe fn m5u_speaker_set_volume(_volume: u8) {}
    pub unsafe fn m5u_speaker_tone(_frequency_hz: u32, _duration_ms: u32) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_play_i16(
        _samples: *const i16,
        _len: usize,
        _sample_rate_hz: u32,
    ) -> bool {
        true
    }

    pub unsafe fn m5u_imu_begin() -> bool {
        true
    }
    pub unsafe fn m5u_imu_get_accel(x: *mut f32, y: *mut f32, z: *mut f32) -> bool {
        if !x.is_null() {
            *x = 0.0;
        }
        if !y.is_null() {
            *y = 0.0;
        }
        if !z.is_null() {
            *z = 1.0;
        }
        true
    }
    pub unsafe fn m5u_imu_get_gyro(x: *mut f32, y: *mut f32, z: *mut f32) -> bool {
        if !x.is_null() {
            *x = 0.0;
        }
        if !y.is_null() {
            *y = 0.0;
        }
        if !z.is_null() {
            *z = 0.0;
        }
        true
    }
    pub unsafe fn m5u_imu_get_temp_c(temp: *mut f32) -> bool {
        if !temp.is_null() {
            *temp = 25.0;
        }
        true
    }

    pub unsafe fn m5u_touch_count() -> c_int {
        0
    }
    pub unsafe fn m5u_touch_get(_index: c_int, _x: *mut c_int, _y: *mut c_int) -> bool {
        false
    }

    pub unsafe fn m5u_rtc_get_datetime(
        year: *mut c_int,
        month: *mut c_int,
        day: *mut c_int,
        hour: *mut c_int,
        minute: *mut c_int,
        second: *mut c_int,
    ) -> bool {
        if !year.is_null() {
            *year = 2026;
        }
        if !month.is_null() {
            *month = 1;
        }
        if !day.is_null() {
            *day = 1;
        }
        if !hour.is_null() {
            *hour = 0;
        }
        if !minute.is_null() {
            *minute = 0;
        }
        if !second.is_null() {
            *second = 0;
        }
        true
    }
    pub unsafe fn m5u_rtc_set_datetime(
        _year: c_int,
        _month: c_int,
        _day: c_int,
        _hour: c_int,
        _minute: c_int,
        _second: c_int,
    ) -> bool {
        true
    }

    pub unsafe fn m5u_battery_level() -> c_int {
        100
    }
    pub unsafe fn m5u_battery_voltage_mv() -> c_int {
        4200
    }
    pub unsafe fn m5u_power_is_charging() -> bool {
        false
    }

    pub unsafe fn m5u_log_println(_text: *const c_char) {}
    pub unsafe fn m5u_sd_begin() -> bool {
        false
    }
}

#[cfg(not(target_os = "espidf"))]
pub use host_stubs::*;
