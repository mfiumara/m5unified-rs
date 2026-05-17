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
//!
//! Firmware projects targeting ESP-IDF must provide the native shim component
//! from this crate's `native/` directory in their ESP-IDF component graph. The
//! host stubs are compile-time conveniences only and do not simulate M5Stack
//! hardware behavior.

use core::ffi::{c_char, c_float, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct m5u_touch_detail_t {
    pub x: c_int,
    pub y: c_int,
    pub prev_x: c_int,
    pub prev_y: c_int,
    pub is_pressed: bool,
    pub was_pressed: bool,
    pub was_released: bool,
    pub was_clicked: bool,
    pub was_hold: bool,
    pub is_holding: bool,
    pub click_count: c_int,
}

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

    pub fn m5u_display_get_rotation() -> c_int;
    pub fn m5u_display_set_brightness(brightness: u8);
    pub fn m5u_display_set_epd_fastest();
    pub fn m5u_display_start_write();
    pub fn m5u_display_end_write();
    pub fn m5u_display_display();
    pub fn m5u_display_display_busy() -> bool;
    pub fn m5u_display_wait_display();
    pub fn m5u_display_get_cursor_y() -> c_int;
    pub fn m5u_display_font_height() -> c_int;
    pub fn m5u_display_get_base_color() -> u16;
    pub fn m5u_display_set_color(color: u16);
    pub fn m5u_display_set_text_wrap(wrap_x: bool, wrap_y: bool);
    pub fn m5u_display_set_text_datum(datum: c_int);
    pub fn m5u_display_draw_string(text: *const c_char, x: c_int, y: c_int) -> c_int;
    pub fn m5u_display_write_pixel(x: c_int, y: c_int, color: u16);
    pub fn m5u_display_write_fast_vline(x: c_int, y: c_int, h: c_int, color: u16);
    pub fn m5u_display_set_clip_rect(x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn m5u_display_clear_clip_rect();
    pub fn m5u_display_color888(r: u8, g: u8, b: u8) -> u16;
    pub fn m5u_display_count() -> c_int;
    pub fn m5u_display_index_for_kind(kind: c_int) -> c_int;
    pub fn m5u_display_width_at(index: c_int) -> c_int;
    pub fn m5u_display_height_at(index: c_int) -> c_int;
    pub fn m5u_display_print_at(index: c_int, text: *const c_char);
    pub fn m5u_display_fill_circle_at(index: c_int, x: c_int, y: c_int, r: c_int, color: u16);

    pub fn m5u_button_is_pressed(button: c_int) -> bool;
    pub fn m5u_button_was_pressed(button: c_int) -> bool;
    pub fn m5u_button_was_released(button: c_int) -> bool;
    pub fn m5u_button_was_clicked(button: c_int) -> bool;
    pub fn m5u_button_was_hold(button: c_int) -> bool;
    pub fn m5u_button_is_holding(button: c_int) -> bool;
    pub fn m5u_button_was_decide_click_count(button: c_int) -> bool;
    pub fn m5u_button_get_click_count(button: c_int) -> c_int;

    pub fn m5u_mic_is_enabled() -> bool;
    pub fn m5u_mic_is_recording() -> bool;
    pub fn m5u_mic_end();
    pub fn m5u_mic_record_i16_at(buffer: *mut i16, samples: usize, sample_rate_hz: u32) -> bool;
    pub fn m5u_mic_get_noise_filter_level() -> c_int;
    pub fn m5u_mic_set_noise_filter_level(level: c_int) -> bool;

    pub fn m5u_speaker_is_enabled() -> bool;
    pub fn m5u_speaker_end();
    pub fn m5u_speaker_get_volume() -> u8;
    pub fn m5u_speaker_tone_ex(frequency_hz: c_float, duration_ms: u32, channel: c_int) -> bool;
    pub fn m5u_speaker_play_u8(samples: *const u8, len: usize, sample_rate_hz: u32) -> bool;
    pub fn m5u_speaker_play_wav(data: *const u8, len: usize) -> bool;
    pub fn m5u_speaker_is_playing(channel: c_int) -> bool;
    pub fn m5u_speaker_stop(channel: c_int);
    pub fn m5u_speaker_get_channel_volume(channel: c_int) -> u8;
    pub fn m5u_speaker_set_channel_volume(channel: c_int, volume: u8);
    pub fn m5u_speaker_set_all_channel_volume(volume: u8);

    pub fn m5u_imu_is_enabled() -> bool;
    pub fn m5u_imu_get_type() -> c_int;
    pub fn m5u_imu_update() -> bool;
    pub fn m5u_imu_load_offset_from_nvs() -> bool;
    pub fn m5u_imu_save_offset_to_nvs() -> bool;
    pub fn m5u_imu_get_offset_data(index: c_int) -> c_float;
    pub fn m5u_imu_set_calibration(x: c_float, y: c_float, z: c_float);

    pub fn m5u_touch_get_detail(index: c_int, out: *mut m5u_touch_detail_t) -> bool;
    pub fn m5u_rtc_is_enabled() -> bool;

    pub fn m5u_power_axp2101_disable_irq(mask: u64) -> bool;
    pub fn m5u_power_axp2101_enable_irq(mask: u64) -> bool;
    pub fn m5u_power_axp2101_clear_irq_statuses() -> bool;
    pub fn m5u_power_axp2101_get_irq_statuses() -> u64;
    pub fn m5u_power_axp2101_is_bat_charger_under_temperature_irq() -> bool;
    pub fn m5u_power_axp2101_is_bat_charger_over_temperature_irq() -> bool;
    pub fn m5u_power_axp2101_is_vbus_insert_irq() -> bool;
    pub fn m5u_power_axp2101_is_vbus_remove_irq() -> bool;

    pub fn m5u_led_begin() -> bool;
    pub fn m5u_led_display();
    pub fn m5u_led_set_auto_display(enable: bool);
    pub fn m5u_led_count() -> usize;
    pub fn m5u_led_set_brightness(brightness: u8);
    pub fn m5u_led_set_color_rgb(index: usize, r: u8, g: u8, b: u8);
    pub fn m5u_led_set_all_color_rgb(r: u8, g: u8, b: u8);
    pub fn m5u_led_is_enabled() -> bool;

    pub fn m5u_log_print(text: *const c_char);
    pub fn m5u_log_println(text: *const c_char);
    pub fn m5u_log_level(level: c_int, text: *const c_char);
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

    pub unsafe fn m5u_display_get_rotation() -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_brightness(_brightness: u8) {}
    pub unsafe fn m5u_display_set_epd_fastest() {}
    pub unsafe fn m5u_display_start_write() {}
    pub unsafe fn m5u_display_end_write() {}
    pub unsafe fn m5u_display_display() {}
    pub unsafe fn m5u_display_display_busy() -> bool {
        false
    }
    pub unsafe fn m5u_display_wait_display() {}
    pub unsafe fn m5u_display_get_cursor_y() -> c_int {
        0
    }
    pub unsafe fn m5u_display_font_height() -> c_int {
        16
    }
    pub unsafe fn m5u_display_get_base_color() -> u16 {
        0
    }
    pub unsafe fn m5u_display_set_color(_color: u16) {}
    pub unsafe fn m5u_display_set_text_wrap(_wrap_x: bool, _wrap_y: bool) {}
    pub unsafe fn m5u_display_set_text_datum(_datum: c_int) {}
    pub unsafe fn m5u_display_draw_string(_text: *const c_char, _x: c_int, _y: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_display_write_pixel(_x: c_int, _y: c_int, _color: u16) {}
    pub unsafe fn m5u_display_write_fast_vline(_x: c_int, _y: c_int, _h: c_int, _color: u16) {}
    pub unsafe fn m5u_display_set_clip_rect(_x: c_int, _y: c_int, _w: c_int, _h: c_int) {}
    pub unsafe fn m5u_display_clear_clip_rect() {}
    pub unsafe fn m5u_display_color888(r: u8, g: u8, b: u8) -> u16 {
        ((u16::from(r & 0xF8)) << 8) | ((u16::from(g & 0xFC)) << 3) | u16::from(b >> 3)
    }
    pub unsafe fn m5u_display_count() -> c_int {
        1
    }
    pub unsafe fn m5u_display_index_for_kind(_kind: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_display_width_at(_index: c_int) -> c_int {
        320
    }
    pub unsafe fn m5u_display_height_at(_index: c_int) -> c_int {
        240
    }
    pub unsafe fn m5u_display_print_at(_index: c_int, _text: *const c_char) {}
    pub unsafe fn m5u_display_fill_circle_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }

    pub unsafe fn m5u_button_is_pressed(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_pressed(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_released(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_clicked(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_hold(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_is_holding(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_decide_click_count(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_get_click_count(_button: c_int) -> c_int {
        0
    }

    pub unsafe fn m5u_mic_is_enabled() -> bool {
        true
    }
    pub unsafe fn m5u_mic_is_recording() -> bool {
        false
    }
    pub unsafe fn m5u_mic_end() {}
    pub unsafe fn m5u_mic_record_i16_at(
        buffer: *mut i16,
        samples: usize,
        _sample_rate_hz: u32,
    ) -> bool {
        m5u_mic_record_i16(buffer, samples)
    }
    pub unsafe fn m5u_mic_get_noise_filter_level() -> c_int {
        0
    }
    pub unsafe fn m5u_mic_set_noise_filter_level(_level: c_int) -> bool {
        true
    }

    pub unsafe fn m5u_speaker_is_enabled() -> bool {
        true
    }
    pub unsafe fn m5u_speaker_end() {}
    pub unsafe fn m5u_speaker_get_volume() -> u8 {
        64
    }
    pub unsafe fn m5u_speaker_tone_ex(
        _frequency_hz: c_float,
        _duration_ms: u32,
        _channel: c_int,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_play_u8(
        _samples: *const u8,
        _len: usize,
        _sample_rate_hz: u32,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_play_wav(_data: *const u8, _len: usize) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_is_playing(_channel: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_speaker_stop(_channel: c_int) {}
    pub unsafe fn m5u_speaker_get_channel_volume(_channel: c_int) -> u8 {
        255
    }
    pub unsafe fn m5u_speaker_set_channel_volume(_channel: c_int, _volume: u8) {}
    pub unsafe fn m5u_speaker_set_all_channel_volume(_volume: u8) {}

    pub unsafe fn m5u_imu_is_enabled() -> bool {
        true
    }
    pub unsafe fn m5u_imu_get_type() -> c_int {
        0
    }
    pub unsafe fn m5u_imu_update() -> bool {
        true
    }
    pub unsafe fn m5u_imu_load_offset_from_nvs() -> bool {
        false
    }
    pub unsafe fn m5u_imu_save_offset_to_nvs() -> bool {
        false
    }
    pub unsafe fn m5u_imu_get_offset_data(_index: c_int) -> c_float {
        0.0
    }
    pub unsafe fn m5u_imu_set_calibration(_x: c_float, _y: c_float, _z: c_float) {}

    pub unsafe fn m5u_touch_get_detail(_index: c_int, out: *mut m5u_touch_detail_t) -> bool {
        if !out.is_null() {
            *out = m5u_touch_detail_t::default();
        }
        false
    }
    pub unsafe fn m5u_rtc_is_enabled() -> bool {
        true
    }

    pub unsafe fn m5u_power_axp2101_disable_irq(_mask: u64) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_enable_irq(_mask: u64) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_clear_irq_statuses() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_get_irq_statuses() -> u64 {
        0
    }
    pub unsafe fn m5u_power_axp2101_is_bat_charger_under_temperature_irq() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_is_bat_charger_over_temperature_irq() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_is_vbus_insert_irq() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_is_vbus_remove_irq() -> bool {
        false
    }

    pub unsafe fn m5u_led_begin() -> bool {
        false
    }
    pub unsafe fn m5u_led_display() {}
    pub unsafe fn m5u_led_set_auto_display(_enable: bool) {}
    pub unsafe fn m5u_led_count() -> usize {
        0
    }
    pub unsafe fn m5u_led_set_brightness(_brightness: u8) {}
    pub unsafe fn m5u_led_set_color_rgb(_index: usize, _r: u8, _g: u8, _b: u8) {}
    pub unsafe fn m5u_led_set_all_color_rgb(_r: u8, _g: u8, _b: u8) {}
    pub unsafe fn m5u_led_is_enabled() -> bool {
        false
    }

    pub unsafe fn m5u_log_print(_text: *const c_char) {}
    pub unsafe fn m5u_log_println(_text: *const c_char) {}
    pub unsafe fn m5u_log_level(_level: c_int, _text: *const c_char) {}
    pub unsafe fn m5u_sd_begin() -> bool {
        false
    }
}

#[cfg(not(target_os = "espidf"))]
pub use host_stubs::*;
