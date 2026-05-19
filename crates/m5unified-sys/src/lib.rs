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

use core::ffi::{c_char, c_float, c_int, c_void};

pub type m5u_log_callback_t = Option<unsafe extern "C" fn(c_int, bool, *const c_char, *mut c_void)>;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_sd_spi_config_t {
    pub pin_sclk: c_int,
    pub pin_mosi: c_int,
    pub pin_miso: c_int,
    pub pin_cs: c_int,
    pub host_id: c_int,
    pub frequency_khz: u32,
    pub max_files: c_int,
    pub format_if_mount_failed: u8,
}

impl Default for m5u_sd_spi_config_t {
    fn default() -> Self {
        Self {
            pin_sclk: -1,
            pin_mosi: -1,
            pin_miso: -1,
            pin_cs: -1,
            host_id: -1,
            frequency_khz: 20_000,
            max_files: 5,
            format_if_mount_failed: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_config_t {
    pub serial_baudrate: u32,
    pub external_speaker_value: u8,
    pub external_display_value: u16,
    pub clear_display: u8,
    pub output_power: u8,
    pub pmic_button: u8,
    pub internal_imu: u8,
    pub internal_rtc: u8,
    pub internal_mic: u8,
    pub internal_spk: u8,
    pub external_imu: u8,
    pub external_rtc: u8,
    pub disable_rtc_irq: u8,
    pub led_brightness: u8,
    pub fallback_board: c_int,
}

impl Default for m5u_config_t {
    fn default() -> Self {
        Self {
            serial_baudrate: 0,
            external_speaker_value: 0x00,
            external_display_value: 0xFFFF,
            clear_display: 1,
            output_power: 1,
            pmic_button: 1,
            internal_imu: 1,
            internal_rtc: 1,
            internal_mic: 1,
            internal_spk: 1,
            external_imu: 0,
            external_rtc: 0,
            disable_rtc_irq: 1,
            led_brightness: 0,
            fallback_board: -1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_mic_config_t {
    pub pin_data_in: c_int,
    pub pin_bck: c_int,
    pub pin_mck: c_int,
    pub pin_ws: c_int,
    pub sample_rate: u32,
    pub left_channel: u8,
    pub stereo: u8,
    pub over_sampling: u8,
    pub magnification: u8,
    pub noise_filter_level: u8,
    pub use_adc: u8,
    pub dma_buf_len: usize,
    pub dma_buf_count: usize,
    pub task_priority: u8,
    pub task_pinned_core: u8,
    pub i2s_port: c_int,
}

impl Default for m5u_mic_config_t {
    fn default() -> Self {
        Self {
            pin_data_in: -1,
            pin_bck: -1,
            pin_mck: -1,
            pin_ws: -1,
            sample_rate: 16_000,
            left_channel: 0,
            stereo: 0,
            over_sampling: 2,
            magnification: 16,
            noise_filter_level: 0,
            use_adc: 0,
            dma_buf_len: 128,
            dma_buf_count: 8,
            task_priority: 2,
            task_pinned_core: u8::MAX,
            i2s_port: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_speaker_config_t {
    pub pin_data_out: c_int,
    pub pin_bck: c_int,
    pub pin_mck: c_int,
    pub pin_ws: c_int,
    pub sample_rate: u32,
    pub stereo: u8,
    pub buzzer: u8,
    pub use_dac: u8,
    pub dac_zero_level: u8,
    pub magnification: u8,
    pub dma_buf_len: usize,
    pub dma_buf_count: usize,
    pub task_priority: u8,
    pub task_pinned_core: u8,
    pub i2s_port: c_int,
}

impl Default for m5u_speaker_config_t {
    fn default() -> Self {
        Self {
            pin_data_out: -1,
            pin_bck: -1,
            pin_mck: -1,
            pin_ws: -1,
            sample_rate: 48_000,
            stereo: 0,
            buzzer: 0,
            use_dac: 0,
            dac_zero_level: 0,
            magnification: 16,
            dma_buf_len: 256,
            dma_buf_count: 8,
            task_priority: 2,
            task_pinned_core: u8::MAX,
            i2s_port: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_power_ext_port_bus_t {
    pub voltage_mv: u16,
    pub current_limit_ma: u8,
    pub enable: bool,
    pub direction_output: bool,
}

impl Default for m5u_power_ext_port_bus_t {
    fn default() -> Self {
        Self {
            voltage_mv: 5_000,
            current_limit_ma: 0,
            enable: false,
            direction_output: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct m5u_power_ina226_config_t {
    pub shunt_res: c_float,
    pub max_expected_current: c_float,
    pub sampling_rate: u8,
    pub shunt_conversion_time: u8,
    pub bus_conversion_time: u8,
    pub mode: u8,
}

impl Default for m5u_power_ina226_config_t {
    fn default() -> Self {
        Self {
            shunt_res: 0.1,
            max_expected_current: 2.0,
            sampling_rate: 0b010,
            shunt_conversion_time: 0b100,
            bus_conversion_time: 0b100,
            mode: 0b111,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct m5u_led_color_t {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct m5u_touch_detail_t {
    pub x: c_int,
    pub y: c_int,
    pub prev_x: c_int,
    pub prev_y: c_int,
    pub base_x: c_int,
    pub base_y: c_int,
    pub base_msec: u32,
    pub state: u8,
    pub is_pressed: bool,
    pub was_pressed: bool,
    pub was_released: bool,
    pub was_clicked: bool,
    pub was_hold: bool,
    pub is_holding: bool,
    pub click_count: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_rtc_datetime_t {
    pub year: c_int,
    pub month: c_int,
    pub day: c_int,
    pub weekday: c_int,
    pub hour: c_int,
    pub minute: c_int,
    pub second: c_int,
}

impl Default for m5u_rtc_datetime_t {
    fn default() -> Self {
        Self {
            year: 2026,
            month: 1,
            day: 1,
            weekday: 4,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct m5u_imu_data_t {
    pub usec: u32,
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub mag_x: f32,
    pub mag_y: f32,
    pub mag_z: f32,
}

#[cfg(target_os = "espidf")]
extern "C" {
    pub fn m5u_begin() -> bool;
    pub fn m5u_begin_with_config(config: *const m5u_config_t) -> bool;
    pub fn m5u_update();
    pub fn m5u_delay_ms(ms: u32);
    pub fn m5u_millis() -> u32;
    pub fn m5u_micros() -> u32;
    pub fn m5u_get_update_msec() -> u32;
    pub fn m5u_get_board() -> c_int;
    pub fn m5u_get_pin(name: c_int) -> c_int;
    pub fn m5u_set_primary_display_index(index: usize) -> bool;
    pub fn m5u_set_primary_display_type(kind: c_int) -> bool;
    pub fn m5u_set_primary_display_types(kinds: *const c_int, len: usize) -> bool;
    pub fn m5u_set_log_display_index(index: usize);
    pub fn m5u_set_log_display_type(kind: c_int);
    pub fn m5u_set_log_display_types(kinds: *const c_int, len: usize);
    pub fn m5u_set_touch_button_height(pixel: u16);
    pub fn m5u_set_touch_button_height_by_ratio(ratio: u8);
    pub fn m5u_get_touch_button_height() -> u16;

    pub fn m5u_io_expander_available(index: usize) -> bool;
    pub fn m5u_io_expander_set_direction(index: usize, pin: u8, output: bool) -> bool;
    pub fn m5u_io_expander_enable_pull(index: usize, pin: u8, enable: bool) -> bool;
    pub fn m5u_io_expander_set_pull_mode(index: usize, pin: u8, pull_up: bool) -> bool;
    pub fn m5u_io_expander_set_high_impedance(index: usize, pin: u8, enable: bool) -> bool;
    pub fn m5u_io_expander_get_write_value(index: usize, pin: u8) -> bool;
    pub fn m5u_io_expander_digital_write(index: usize, pin: u8, level: bool) -> bool;
    pub fn m5u_io_expander_digital_read(index: usize, pin: u8) -> bool;
    pub fn m5u_io_expander_reset_irq(index: usize) -> bool;
    pub fn m5u_io_expander_disable_irq(index: usize) -> bool;
    pub fn m5u_io_expander_enable_irq(index: usize) -> bool;

    pub fn m5u_i2c_set_port(bus: c_int, port_num: c_int, pin_sda: c_int, pin_scl: c_int);
    pub fn m5u_i2c_begin(bus: c_int) -> bool;
    pub fn m5u_i2c_begin_with_port(
        bus: c_int,
        port_num: c_int,
        pin_sda: c_int,
        pin_scl: c_int,
    ) -> bool;
    pub fn m5u_i2c_release(bus: c_int) -> bool;
    pub fn m5u_i2c_is_enabled(bus: c_int) -> bool;
    pub fn m5u_i2c_get_port(bus: c_int) -> c_int;
    pub fn m5u_i2c_get_sda(bus: c_int) -> c_int;
    pub fn m5u_i2c_get_scl(bus: c_int) -> c_int;
    pub fn m5u_i2c_start(bus: c_int, address: u8, read: bool, freq: u32) -> bool;
    pub fn m5u_i2c_restart(bus: c_int, address: u8, read: bool, freq: u32) -> bool;
    pub fn m5u_i2c_stop(bus: c_int) -> bool;
    pub fn m5u_i2c_write_byte(bus: c_int, data: u8) -> bool;
    pub fn m5u_i2c_write(bus: c_int, data: *const u8, length: usize) -> bool;
    pub fn m5u_i2c_read(bus: c_int, result: *mut u8, length: usize, last_nack: bool) -> bool;
    pub fn m5u_i2c_write_register(
        bus: c_int,
        address: u8,
        reg: u8,
        data: *const u8,
        length: usize,
        freq: u32,
    ) -> bool;
    pub fn m5u_i2c_read_register(
        bus: c_int,
        address: u8,
        reg: u8,
        result: *mut u8,
        length: usize,
        freq: u32,
    ) -> bool;
    pub fn m5u_i2c_write_register8(bus: c_int, address: u8, reg: u8, data: u8, freq: u32) -> bool;
    pub fn m5u_i2c_read_register8(bus: c_int, address: u8, reg: u8, freq: u32) -> u8;
    pub fn m5u_i2c_bit_on(bus: c_int, address: u8, reg: u8, data: u8, freq: u32) -> bool;
    pub fn m5u_i2c_bit_off(bus: c_int, address: u8, reg: u8, data: u8, freq: u32) -> bool;
    pub fn m5u_i2c_scan(bus: c_int, result: *mut bool, freq: u32);
    pub fn m5u_i2c_scan_address(bus: c_int, address: u8, freq: u32) -> bool;

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
    pub fn m5u_mic_record_u8(buffer: *mut u8, samples: usize) -> bool;
    pub fn m5u_speaker_begin() -> bool;
    pub fn m5u_speaker_set_volume(volume: u8);
    pub fn m5u_speaker_tone(frequency_hz: u32, duration_ms: u32) -> bool;
    pub fn m5u_speaker_play_i16(samples: *const i16, len: usize, sample_rate_hz: u32) -> bool;

    pub fn m5u_imu_begin() -> bool;
    pub fn m5u_imu_begin_for_board(board: c_int) -> bool;
    pub fn m5u_imu_get_accel(x: *mut f32, y: *mut f32, z: *mut f32) -> bool;
    pub fn m5u_imu_get_gyro(x: *mut f32, y: *mut f32, z: *mut f32) -> bool;
    pub fn m5u_imu_get_mag(x: *mut f32, y: *mut f32, z: *mut f32) -> bool;
    pub fn m5u_imu_get_data(out: *mut m5u_imu_data_t) -> bool;
    pub fn m5u_imu_get_temp_c(temp: *mut f32) -> bool;

    pub fn m5u_touch_count() -> c_int;
    pub fn m5u_touch_get(index: c_int, x: *mut c_int, y: *mut c_int) -> bool;
    pub fn m5u_touch_get_raw(index: c_int, x: *mut c_int, y: *mut c_int) -> bool;

    pub fn m5u_rtc_begin() -> bool;
    pub fn m5u_rtc_begin_for_board(board: c_int) -> bool;
    pub fn m5u_rtc_is_enabled() -> bool;
    pub fn m5u_rtc_get_volt_low() -> bool;
    pub fn m5u_rtc_get_datetime(
        year: *mut c_int,
        month: *mut c_int,
        day: *mut c_int,
        hour: *mut c_int,
        minute: *mut c_int,
        second: *mut c_int,
    ) -> bool;
    pub fn m5u_rtc_get_datetime_detail(out: *mut m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_get_date_detail(out: *mut m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_get_time_detail(out: *mut m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_set_datetime(
        year: c_int,
        month: c_int,
        day: c_int,
        hour: c_int,
        minute: c_int,
        second: c_int,
    ) -> bool;
    pub fn m5u_rtc_set_datetime_detail(datetime: *const m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_set_date_detail(date: *const m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_set_time_detail(time: *const m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_device_begin(kind: c_int) -> bool;
    pub fn m5u_rtc_device_get_datetime_detail(kind: c_int, out: *mut m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_device_get_date_detail(kind: c_int, out: *mut m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_device_get_time_detail(kind: c_int, out: *mut m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_device_set_datetime_detail(
        kind: c_int,
        datetime: *const m5u_rtc_datetime_t,
    ) -> bool;
    pub fn m5u_rtc_device_set_date_detail(kind: c_int, date: *const m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_device_set_time_detail(kind: c_int, time: *const m5u_rtc_datetime_t) -> bool;
    pub fn m5u_rtc_device_get_volt_low(kind: c_int) -> bool;
    pub fn m5u_rtc_device_set_timer_irq(kind: c_int, timer_msec: u32) -> u32;
    pub fn m5u_rtc_device_set_alarm_irq_datetime(
        kind: c_int,
        datetime: *const m5u_rtc_datetime_t,
    ) -> c_int;
    pub fn m5u_rtc_device_set_alarm_irq_time(kind: c_int, time: *const m5u_rtc_datetime_t)
        -> c_int;
    pub fn m5u_rtc_device_get_irq_status(kind: c_int) -> bool;
    pub fn m5u_rtc_device_clear_irq(kind: c_int);
    pub fn m5u_rtc_device_disable_irq(kind: c_int);

    pub fn m5u_battery_level() -> c_int;
    pub fn m5u_battery_voltage_mv() -> c_int;
    pub fn m5u_power_begin() -> bool;
    pub fn m5u_power_get_type() -> c_int;
    pub fn m5u_power_get_charge_state() -> c_int;
    pub fn m5u_power_is_charging() -> bool;
    pub fn m5u_power_set_led(brightness: u8);
    pub fn m5u_power_set_ext_output(enable: bool, port_mask: u16);
    pub fn m5u_power_get_ext_output() -> bool;
    pub fn m5u_power_set_usb_output(enable: bool);
    pub fn m5u_power_get_usb_output() -> bool;
    pub fn m5u_power_set_battery_charge(enable: bool);
    pub fn m5u_power_set_charge_current(max_ma: u16);
    pub fn m5u_power_set_charge_voltage(max_mv: u16);
    pub fn m5u_power_get_vbus_voltage_mv() -> c_int;
    pub fn m5u_power_get_battery_current_ma() -> c_int;
    pub fn m5u_power_get_ext_voltage_mv(port_mask: u16) -> c_float;
    pub fn m5u_power_get_ext_current_ma(port_mask: u16) -> c_float;
    pub fn m5u_power_get_key_state() -> u8;
    pub fn m5u_power_set_ext_port_bus_config(config: *const m5u_power_ext_port_bus_t);
    pub fn m5u_power_set_vibration(level: u8);
    pub fn m5u_power_power_off();
    pub fn m5u_power_timer_sleep_seconds(seconds: c_int);
    pub fn m5u_power_timer_sleep_time(time: *const m5u_rtc_datetime_t);
    pub fn m5u_power_timer_sleep_date_time(
        date: *const m5u_rtc_datetime_t,
        time: *const m5u_rtc_datetime_t,
    );
    pub fn m5u_power_deep_sleep_us(micro_seconds: u64, touch_wakeup: bool);
    pub fn m5u_power_light_sleep_us(micro_seconds: u64, touch_wakeup: bool);

    pub fn m5u_display_get_rotation() -> c_int;
    pub fn m5u_display_set_brightness(brightness: u8);
    pub fn m5u_display_set_epd_fastest();
    pub fn m5u_display_set_epd_mode(mode: c_int);
    pub fn m5u_display_set_text_scroll(scroll: bool);
    pub fn m5u_display_set_font(font: c_int) -> bool;
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
    pub fn m5u_display_index_for_kinds(kinds: *const c_int, len: usize) -> c_int;
    pub fn m5u_display_width_at(index: c_int) -> c_int;
    pub fn m5u_display_height_at(index: c_int) -> c_int;
    pub fn m5u_display_fill_screen_at(index: c_int, color: u16);
    pub fn m5u_display_set_cursor_at(index: c_int, x: c_int, y: c_int);
    pub fn m5u_display_set_text_size_at(index: c_int, size: c_int);
    pub fn m5u_display_set_text_color_at(index: c_int, fg: u16, bg: u16);
    pub fn m5u_display_get_rotation_at(index: c_int) -> c_int;
    pub fn m5u_display_set_rotation_at(index: c_int, rotation: c_int);
    pub fn m5u_display_set_color_at(index: c_int, color: u16);
    pub fn m5u_display_start_write_at(index: c_int);
    pub fn m5u_display_end_write_at(index: c_int);
    pub fn m5u_display_print_at(index: c_int, text: *const c_char);
    pub fn m5u_display_println_at(index: c_int, text: *const c_char);
    pub fn m5u_display_draw_string_at(
        index: c_int,
        text: *const c_char,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn m5u_display_draw_line_at(
        index: c_int,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        color: u16,
    );
    pub fn m5u_display_draw_rect_at(
        index: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        color: u16,
    );
    pub fn m5u_display_fill_rect_at(
        index: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        color: u16,
    );
    pub fn m5u_display_draw_circle_at(index: c_int, x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_display_fill_circle_at(index: c_int, x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_display_write_pixel_at(index: c_int, x: c_int, y: c_int, color: u16);
    pub fn m5u_display_draw_pixel_at(index: c_int, x: c_int, y: c_int, color: u16);

    pub fn m5u_button_is_pressed(button: c_int) -> bool;
    pub fn m5u_button_was_pressed(button: c_int) -> bool;
    pub fn m5u_button_was_released(button: c_int) -> bool;
    pub fn m5u_button_was_clicked(button: c_int) -> bool;
    pub fn m5u_button_was_hold(button: c_int) -> bool;
    pub fn m5u_button_is_holding(button: c_int) -> bool;
    pub fn m5u_button_was_decide_click_count(button: c_int) -> bool;
    pub fn m5u_button_get_click_count(button: c_int) -> c_int;
    pub fn m5u_button_was_single_clicked(button: c_int) -> bool;
    pub fn m5u_button_was_double_clicked(button: c_int) -> bool;
    pub fn m5u_button_was_change_pressed(button: c_int) -> bool;
    pub fn m5u_button_is_released(button: c_int) -> bool;
    pub fn m5u_button_was_released_after_hold(button: c_int) -> bool;
    pub fn m5u_button_was_release_for(button: c_int, ms: u32) -> bool;
    pub fn m5u_button_pressed_for(button: c_int, ms: u32) -> bool;
    pub fn m5u_button_released_for(button: c_int, ms: u32) -> bool;
    pub fn m5u_button_set_debounce_thresh(button: c_int, ms: u32);
    pub fn m5u_button_set_hold_thresh(button: c_int, ms: u32);
    pub fn m5u_button_set_raw_state(button: c_int, msec: u32, press: bool);
    pub fn m5u_button_set_state(button: c_int, msec: u32, state: u8);
    pub fn m5u_button_get_state(button: c_int) -> u8;
    pub fn m5u_button_last_change(button: c_int) -> u32;
    pub fn m5u_button_get_debounce_thresh(button: c_int) -> u32;
    pub fn m5u_button_get_hold_thresh(button: c_int) -> u32;
    pub fn m5u_button_get_update_msec(button: c_int) -> u32;

    pub fn m5u_mic_is_enabled() -> bool;
    pub fn m5u_mic_is_running() -> bool;
    pub fn m5u_mic_is_recording() -> bool;
    pub fn m5u_mic_recording_state() -> usize;
    pub fn m5u_mic_end();
    pub fn m5u_mic_record_i16_at(buffer: *mut i16, samples: usize, sample_rate_hz: u32) -> bool;
    pub fn m5u_mic_record_i16_ex(
        buffer: *mut i16,
        samples: usize,
        sample_rate_hz: u32,
        stereo: bool,
    ) -> bool;
    pub fn m5u_mic_record_u8_ex(
        buffer: *mut u8,
        samples: usize,
        sample_rate_hz: u32,
        stereo: bool,
    ) -> bool;
    pub fn m5u_mic_set_sample_rate(sample_rate_hz: u32);
    pub fn m5u_mic_get_config(out: *mut m5u_mic_config_t) -> bool;
    pub fn m5u_mic_set_config(config: *const m5u_mic_config_t) -> bool;
    pub fn m5u_mic_get_noise_filter_level() -> c_int;
    pub fn m5u_mic_set_noise_filter_level(level: c_int) -> bool;

    pub fn m5u_speaker_is_enabled() -> bool;
    pub fn m5u_speaker_is_running() -> bool;
    pub fn m5u_speaker_end();
    pub fn m5u_speaker_get_volume() -> u8;
    pub fn m5u_speaker_get_config(out: *mut m5u_speaker_config_t) -> bool;
    pub fn m5u_speaker_set_config(config: *const m5u_speaker_config_t) -> bool;
    pub fn m5u_speaker_tone_ex(frequency_hz: c_float, duration_ms: u32, channel: c_int) -> bool;
    pub fn m5u_speaker_tone_options(
        frequency_hz: c_float,
        duration_ms: u32,
        channel: c_int,
        stop_current_sound: bool,
    ) -> bool;
    pub fn m5u_speaker_tone_full(
        frequency_hz: c_float,
        duration_ms: u32,
        channel: c_int,
        stop_current_sound: bool,
        raw_data: *const u8,
        len: usize,
        stereo: bool,
    ) -> bool;
    pub fn m5u_speaker_play_u8(samples: *const u8, len: usize, sample_rate_hz: u32) -> bool;
    pub fn m5u_speaker_play_u8_ex(
        samples: *const u8,
        len: usize,
        sample_rate_hz: u32,
        stereo: bool,
        repeat: u32,
        channel: c_int,
        stop_current_sound: bool,
    ) -> bool;
    pub fn m5u_speaker_play_i8_ex(
        samples: *const i8,
        len: usize,
        sample_rate_hz: u32,
        stereo: bool,
        repeat: u32,
        channel: c_int,
        stop_current_sound: bool,
    ) -> bool;
    pub fn m5u_speaker_play_i16_ex(
        samples: *const i16,
        len: usize,
        sample_rate_hz: u32,
        stereo: bool,
        repeat: u32,
        channel: c_int,
        stop_current_sound: bool,
    ) -> bool;
    pub fn m5u_speaker_play_wav(data: *const u8, len: usize) -> bool;
    pub fn m5u_speaker_play_wav_ex(
        data: *const u8,
        len: usize,
        repeat: u32,
        channel: c_int,
        stop_current_sound: bool,
    ) -> bool;
    pub fn m5u_speaker_is_playing(channel: c_int) -> bool;
    pub fn m5u_speaker_playing_channels() -> usize;
    pub fn m5u_speaker_channel_playing_state(channel: c_int) -> usize;
    pub fn m5u_speaker_stop(channel: c_int);
    pub fn m5u_speaker_get_channel_volume(channel: c_int) -> u8;
    pub fn m5u_speaker_set_channel_volume(channel: c_int, volume: u8);
    pub fn m5u_speaker_set_all_channel_volume(volume: u8);

    pub fn m5u_imu_is_enabled() -> bool;
    pub fn m5u_imu_get_type() -> c_int;
    pub fn m5u_imu_update() -> bool;
    pub fn m5u_imu_update_mask() -> c_int;
    pub fn m5u_imu_sleep() -> bool;
    pub fn m5u_imu_set_clock(freq: u32);
    pub fn m5u_imu_set_axis_order(axis0: c_int, axis1: c_int, axis2: c_int) -> bool;
    pub fn m5u_imu_set_axis_order_right_handed(axis0: c_int, axis1: c_int) -> bool;
    pub fn m5u_imu_set_axis_order_left_handed(axis0: c_int, axis1: c_int) -> bool;
    pub fn m5u_imu_set_int_pin_active_logic(level: bool) -> bool;
    pub fn m5u_imu_load_offset_from_nvs() -> bool;
    pub fn m5u_imu_save_offset_to_nvs() -> bool;
    pub fn m5u_imu_get_offset_data(index: c_int) -> c_float;
    pub fn m5u_imu_set_calibration(x: c_float, y: c_float, z: c_float);
    pub fn m5u_imu_set_calibration_strength(accel: u8, gyro: u8, mag: u8);
    pub fn m5u_imu_clear_offset_data();
    pub fn m5u_imu_set_offset_data(index: usize, value: i32);
    pub fn m5u_imu_get_offset_data_i32(index: usize) -> i32;
    pub fn m5u_imu_get_raw_data(index: usize) -> i16;

    pub fn m5u_touch_get_detail(index: c_int, out: *mut m5u_touch_detail_t) -> bool;
    pub fn m5u_touch_is_enabled() -> bool;
    pub fn m5u_touch_set_hold_thresh(ms: u16);
    pub fn m5u_touch_set_flick_thresh(distance: u16);
    pub fn m5u_rtc_set_system_time_from_rtc();
    pub fn m5u_rtc_set_timer_irq(timer_msec: u32) -> u32;
    pub fn m5u_rtc_set_alarm_irq_after_seconds(after_seconds: c_int) -> c_int;
    pub fn m5u_rtc_set_alarm_irq_datetime(datetime: *const m5u_rtc_datetime_t) -> c_int;
    pub fn m5u_rtc_set_alarm_irq_time(time: *const m5u_rtc_datetime_t) -> c_int;
    pub fn m5u_rtc_get_irq_status() -> bool;
    pub fn m5u_rtc_clear_irq();
    pub fn m5u_rtc_disable_irq();

    pub fn m5u_power_axp192_begin() -> bool;
    pub fn m5u_power_axp192_get_battery_level() -> c_int;
    pub fn m5u_power_axp192_set_battery_charge(enable: bool) -> bool;
    pub fn m5u_power_axp192_set_charge_current(max_ma: u16) -> bool;
    pub fn m5u_power_axp192_set_charge_voltage(max_mv: u16) -> bool;
    pub fn m5u_power_axp192_is_charging() -> bool;
    pub fn m5u_power_axp192_set_dcdc(channel: u8, voltage_mv: c_int) -> bool;
    pub fn m5u_power_axp192_set_ldo(channel: u8, voltage_mv: c_int) -> bool;
    pub fn m5u_power_axp192_set_gpio(gpio_num: u8, state: bool) -> bool;
    pub fn m5u_power_axp192_power_off() -> bool;
    pub fn m5u_power_axp192_set_adc_state(enable: bool) -> bool;
    pub fn m5u_power_axp192_set_adc_rate(rate: u8) -> bool;
    pub fn m5u_power_axp192_set_exten(enable: bool) -> bool;
    pub fn m5u_power_axp192_set_backup(enable: bool) -> bool;
    pub fn m5u_power_axp192_is_acin() -> bool;
    pub fn m5u_power_axp192_is_vbus() -> bool;
    pub fn m5u_power_axp192_get_bat_state() -> bool;
    pub fn m5u_power_axp192_get_exten() -> bool;
    pub fn m5u_power_axp192_get_battery_voltage_v() -> c_float;
    pub fn m5u_power_axp192_get_battery_discharge_current_ma() -> c_float;
    pub fn m5u_power_axp192_get_battery_charge_current_ma() -> c_float;
    pub fn m5u_power_axp192_get_battery_power_mw() -> c_float;
    pub fn m5u_power_axp192_get_acin_voltage_v() -> c_float;
    pub fn m5u_power_axp192_get_acin_current_ma() -> c_float;
    pub fn m5u_power_axp192_get_vbus_voltage_v() -> c_float;
    pub fn m5u_power_axp192_get_vbus_current_ma() -> c_float;
    pub fn m5u_power_axp192_get_aps_voltage_v() -> c_float;
    pub fn m5u_power_axp192_get_internal_temperature_c() -> c_float;
    pub fn m5u_power_axp192_get_pek_press() -> u8;
    pub fn m5u_power_aw32001_begin() -> bool;
    pub fn m5u_power_aw32001_set_battery_charge(enable: bool) -> bool;
    pub fn m5u_power_aw32001_set_charge_current(max_ma: u16) -> bool;
    pub fn m5u_power_aw32001_set_charge_voltage(max_mv: u16) -> bool;
    pub fn m5u_power_aw32001_is_charging() -> bool;
    pub fn m5u_power_aw32001_get_charge_current() -> u16;
    pub fn m5u_power_aw32001_get_charge_voltage() -> u16;
    pub fn m5u_power_aw32001_get_charge_status() -> c_int;
    pub fn m5u_power_bq27220_begin() -> bool;
    pub fn m5u_power_bq27220_get_current_ma() -> i16;
    pub fn m5u_power_bq27220_get_voltage_mv() -> i16;
    pub fn m5u_power_bq27220_get_current_a() -> c_float;
    pub fn m5u_power_bq27220_get_voltage_v() -> c_float;
    pub fn m5u_power_ina226_begin() -> bool;
    pub fn m5u_power_ina226_config(config: *const m5u_power_ina226_config_t) -> bool;
    pub fn m5u_power_ina226_get_bus_voltage_v() -> c_float;
    pub fn m5u_power_ina226_get_shunt_voltage_v() -> c_float;
    pub fn m5u_power_ina226_get_shunt_current_a() -> c_float;
    pub fn m5u_power_ina226_get_power_w() -> c_float;
    pub fn m5u_power_ina3221_begin(index: usize) -> bool;
    pub fn m5u_power_ina3221_get_bus_voltage_v(index: usize, channel: u8) -> c_float;
    pub fn m5u_power_ina3221_get_shunt_voltage_v(index: usize, channel: u8) -> c_float;
    pub fn m5u_power_ina3221_get_current_a(index: usize, channel: u8) -> c_float;
    pub fn m5u_power_ina3221_get_bus_voltage_mv(index: usize, channel: u8) -> i32;
    pub fn m5u_power_ina3221_get_shunt_voltage_mv(index: usize, channel: u8) -> i32;
    pub fn m5u_power_ina3221_set_shunt_res(index: usize, channel: u8, res: u32) -> bool;
    pub fn m5u_power_ip5306_begin() -> bool;
    pub fn m5u_power_ip5306_get_battery_level() -> c_int;
    pub fn m5u_power_ip5306_set_battery_charge(enable: bool) -> bool;
    pub fn m5u_power_ip5306_set_charge_current(max_ma: u16) -> bool;
    pub fn m5u_power_ip5306_set_charge_voltage(max_mv: u16) -> bool;
    pub fn m5u_power_ip5306_is_charging() -> bool;
    pub fn m5u_power_ip5306_set_power_boost_keep_on(enable: bool) -> bool;
    pub fn m5u_power_py32pmic_begin() -> bool;
    pub fn m5u_power_py32pmic_set_ext_output(enable: bool) -> bool;
    pub fn m5u_power_py32pmic_set_battery_charge(enable: bool) -> bool;
    pub fn m5u_power_py32pmic_set_charge_current(max_ma: u16) -> bool;
    pub fn m5u_power_py32pmic_set_charge_voltage(max_mv: u16) -> bool;
    pub fn m5u_power_py32pmic_is_charging() -> bool;
    pub fn m5u_power_py32pmic_get_charge_current() -> u16;
    pub fn m5u_power_py32pmic_get_charge_voltage() -> u16;
    pub fn m5u_power_py32pmic_get_pek_press() -> u8;
    pub fn m5u_power_py32pmic_power_off() -> bool;
    pub fn m5u_power_axp2101_begin() -> bool;
    pub fn m5u_power_axp2101_get_battery_level() -> c_int;
    pub fn m5u_power_axp2101_set_battery_charge(enable: bool) -> bool;
    pub fn m5u_power_axp2101_set_pre_charge_current(max_ma: u16) -> bool;
    pub fn m5u_power_axp2101_set_charge_current(max_ma: u16) -> bool;
    pub fn m5u_power_axp2101_set_charge_voltage(max_mv: u16) -> bool;
    pub fn m5u_power_axp2101_get_charge_status() -> c_int;
    pub fn m5u_power_axp2101_is_charging() -> bool;
    pub fn m5u_power_axp2101_set_ldo(kind: c_int, channel: c_int, voltage_mv: c_int) -> bool;
    pub fn m5u_power_axp2101_get_ldo_enabled(kind: c_int, channel: c_int) -> bool;
    pub fn m5u_power_axp2101_power_off() -> bool;
    pub fn m5u_power_axp2101_set_adc_state(enable: bool) -> bool;
    pub fn m5u_power_axp2101_set_adc_rate(rate: u8) -> bool;
    pub fn m5u_power_axp2101_set_backup(enable: bool) -> bool;
    pub fn m5u_power_axp2101_is_acin() -> bool;
    pub fn m5u_power_axp2101_is_vbus() -> bool;
    pub fn m5u_power_axp2101_get_bat_state() -> bool;
    pub fn m5u_power_axp2101_get_battery_voltage_v() -> c_float;
    pub fn m5u_power_axp2101_get_battery_discharge_current_ma() -> c_float;
    pub fn m5u_power_axp2101_get_battery_charge_current_ma() -> c_float;
    pub fn m5u_power_axp2101_get_battery_power_mw() -> c_float;
    pub fn m5u_power_axp2101_get_acin_voltage_v() -> c_float;
    pub fn m5u_power_axp2101_get_acin_current_ma() -> c_float;
    pub fn m5u_power_axp2101_get_vbus_voltage_v() -> c_float;
    pub fn m5u_power_axp2101_get_vbus_current_ma() -> c_float;
    pub fn m5u_power_axp2101_get_ts_voltage_v() -> c_float;
    pub fn m5u_power_axp2101_get_aps_voltage_v() -> c_float;
    pub fn m5u_power_axp2101_get_internal_temperature_c() -> c_float;
    pub fn m5u_power_axp2101_get_pek_press() -> u8;
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
    pub fn m5u_led_set_colors_rgb(colors: *const m5u_led_color_t, index: usize, length: usize);
    pub fn m5u_led_get_type(index: usize) -> c_int;
    pub fn m5u_led_is_enabled() -> bool;

    pub fn m5u_log_print(text: *const c_char);
    pub fn m5u_log_println(text: *const c_char);
    pub fn m5u_log_println_empty();
    pub fn m5u_log_level(level: c_int, text: *const c_char);
    pub fn m5u_log_dump(addr: *const c_void, len: u32, level: c_int);
    pub fn m5u_log_path_to_file_name(path: *const c_char) -> *const c_char;
    pub fn m5u_log_set_callback(callback: m5u_log_callback_t, user_data: *mut c_void) -> bool;
    pub fn m5u_log_set_enable_color(target: c_int, enable: bool) -> bool;
    pub fn m5u_log_get_enable_color(target: c_int) -> bool;
    pub fn m5u_log_set_level(target: c_int, level: c_int) -> bool;
    pub fn m5u_log_get_level(target: c_int) -> c_int;
    pub fn m5u_log_set_suffix(target: c_int, suffix: *const c_char) -> bool;
    pub fn m5u_sd_begin() -> bool;
    pub fn m5u_sd_begin_spi(config: *const m5u_sd_spi_config_t) -> bool;
    pub fn m5u_sd_is_mounted() -> bool;
    pub fn m5u_sd_end();
}

#[cfg(not(target_os = "espidf"))]
mod host_stubs {
    use super::*;
    use core::ptr;

    pub unsafe fn m5u_begin() -> bool {
        true
    }
    pub unsafe fn m5u_begin_with_config(_config: *const m5u_config_t) -> bool {
        true
    }
    pub unsafe fn m5u_update() {}
    pub unsafe fn m5u_delay_ms(_ms: u32) {}
    pub unsafe fn m5u_millis() -> u32 {
        0
    }
    pub unsafe fn m5u_micros() -> u32 {
        0
    }
    pub unsafe fn m5u_get_update_msec() -> u32 {
        0
    }
    pub unsafe fn m5u_get_board() -> c_int {
        0
    }
    pub unsafe fn m5u_get_pin(_name: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_set_primary_display_index(index: usize) -> bool {
        index == 0
    }
    pub unsafe fn m5u_set_primary_display_type(_kind: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_set_primary_display_types(_kinds: *const c_int, _len: usize) -> bool {
        false
    }
    pub unsafe fn m5u_set_log_display_index(_index: usize) {}
    pub unsafe fn m5u_set_log_display_type(_kind: c_int) {}
    pub unsafe fn m5u_set_log_display_types(_kinds: *const c_int, _len: usize) {}
    pub unsafe fn m5u_set_touch_button_height(_pixel: u16) {}
    pub unsafe fn m5u_set_touch_button_height_by_ratio(_ratio: u8) {}
    pub unsafe fn m5u_get_touch_button_height() -> u16 {
        0
    }

    pub unsafe fn m5u_io_expander_available(_index: usize) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_set_direction(_index: usize, _pin: u8, _output: bool) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_enable_pull(_index: usize, _pin: u8, _enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_set_pull_mode(_index: usize, _pin: u8, _pull_up: bool) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_set_high_impedance(
        _index: usize,
        _pin: u8,
        _enable: bool,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_get_write_value(_index: usize, _pin: u8) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_digital_write(_index: usize, _pin: u8, _level: bool) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_digital_read(_index: usize, _pin: u8) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_reset_irq(_index: usize) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_disable_irq(_index: usize) -> bool {
        false
    }
    pub unsafe fn m5u_io_expander_enable_irq(_index: usize) -> bool {
        false
    }

    pub unsafe fn m5u_i2c_set_port(
        _bus: c_int,
        _port_num: c_int,
        _pin_sda: c_int,
        _pin_scl: c_int,
    ) {
    }
    pub unsafe fn m5u_i2c_begin(_bus: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_begin_with_port(
        _bus: c_int,
        _port_num: c_int,
        _pin_sda: c_int,
        _pin_scl: c_int,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_release(_bus: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_is_enabled(_bus: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_get_port(_bus: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_i2c_get_sda(_bus: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_i2c_get_scl(_bus: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_i2c_start(_bus: c_int, _address: u8, _read: bool, _freq: u32) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_restart(_bus: c_int, _address: u8, _read: bool, _freq: u32) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_stop(_bus: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_write_byte(_bus: c_int, _data: u8) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_write(_bus: c_int, _data: *const u8, _length: usize) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_read(
        _bus: c_int,
        result: *mut u8,
        length: usize,
        _last_nack: bool,
    ) -> bool {
        if !result.is_null() {
            for i in 0..length {
                ptr::write(result.add(i), 0);
            }
        }
        false
    }
    pub unsafe fn m5u_i2c_write_register(
        _bus: c_int,
        _address: u8,
        _reg: u8,
        _data: *const u8,
        _length: usize,
        _freq: u32,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_read_register(
        _bus: c_int,
        _address: u8,
        _reg: u8,
        result: *mut u8,
        length: usize,
        _freq: u32,
    ) -> bool {
        if !result.is_null() {
            for i in 0..length {
                ptr::write(result.add(i), 0);
            }
        }
        false
    }
    pub unsafe fn m5u_i2c_write_register8(
        _bus: c_int,
        _address: u8,
        _reg: u8,
        _data: u8,
        _freq: u32,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_read_register8(_bus: c_int, _address: u8, _reg: u8, _freq: u32) -> u8 {
        0
    }
    pub unsafe fn m5u_i2c_bit_on(
        _bus: c_int,
        _address: u8,
        _reg: u8,
        _data: u8,
        _freq: u32,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_bit_off(
        _bus: c_int,
        _address: u8,
        _reg: u8,
        _data: u8,
        _freq: u32,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_scan(_bus: c_int, result: *mut bool, _freq: u32) {
        if !result.is_null() {
            for i in 0..120 {
                ptr::write(result.add(i), false);
            }
        }
    }
    pub unsafe fn m5u_i2c_scan_address(_bus: c_int, _address: u8, _freq: u32) -> bool {
        false
    }

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
    pub unsafe fn m5u_mic_record_u8(buffer: *mut u8, samples: usize) -> bool {
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
    pub unsafe fn m5u_imu_begin_for_board(_board: c_int) -> bool {
        false
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
    pub unsafe fn m5u_imu_get_mag(x: *mut f32, y: *mut f32, z: *mut f32) -> bool {
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
    pub unsafe fn m5u_imu_get_data(out: *mut m5u_imu_data_t) -> bool {
        if !out.is_null() {
            *out = m5u_imu_data_t {
                accel_z: 1.0,
                ..m5u_imu_data_t::default()
            };
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
    pub unsafe fn m5u_touch_get_raw(_index: c_int, _x: *mut c_int, _y: *mut c_int) -> bool {
        false
    }

    pub unsafe fn m5u_rtc_begin() -> bool {
        true
    }
    pub unsafe fn m5u_rtc_begin_for_board(_board: c_int) -> bool {
        true
    }
    pub unsafe fn m5u_rtc_is_enabled() -> bool {
        true
    }
    pub unsafe fn m5u_rtc_get_volt_low() -> bool {
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
    pub unsafe fn m5u_rtc_get_datetime_detail(out: *mut m5u_rtc_datetime_t) -> bool {
        if !out.is_null() {
            *out = m5u_rtc_datetime_t::default();
        }
        true
    }
    pub unsafe fn m5u_rtc_get_date_detail(out: *mut m5u_rtc_datetime_t) -> bool {
        m5u_rtc_get_datetime_detail(out)
    }
    pub unsafe fn m5u_rtc_get_time_detail(out: *mut m5u_rtc_datetime_t) -> bool {
        m5u_rtc_get_datetime_detail(out)
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
    pub unsafe fn m5u_rtc_set_datetime_detail(datetime: *const m5u_rtc_datetime_t) -> bool {
        !datetime.is_null()
    }
    pub unsafe fn m5u_rtc_set_date_detail(date: *const m5u_rtc_datetime_t) -> bool {
        !date.is_null()
    }
    pub unsafe fn m5u_rtc_set_time_detail(time: *const m5u_rtc_datetime_t) -> bool {
        !time.is_null()
    }
    pub unsafe fn m5u_rtc_device_begin(_kind: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_get_datetime_detail(
        _kind: c_int,
        _out: *mut m5u_rtc_datetime_t,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_get_date_detail(
        _kind: c_int,
        _out: *mut m5u_rtc_datetime_t,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_get_time_detail(
        _kind: c_int,
        _out: *mut m5u_rtc_datetime_t,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_set_datetime_detail(
        _kind: c_int,
        _datetime: *const m5u_rtc_datetime_t,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_set_date_detail(
        _kind: c_int,
        _date: *const m5u_rtc_datetime_t,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_set_time_detail(
        _kind: c_int,
        _time: *const m5u_rtc_datetime_t,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_get_volt_low(_kind: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_set_timer_irq(_kind: c_int, _timer_msec: u32) -> u32 {
        0
    }
    pub unsafe fn m5u_rtc_device_set_alarm_irq_datetime(
        _kind: c_int,
        _datetime: *const m5u_rtc_datetime_t,
    ) -> c_int {
        -1
    }
    pub unsafe fn m5u_rtc_device_set_alarm_irq_time(
        _kind: c_int,
        _time: *const m5u_rtc_datetime_t,
    ) -> c_int {
        -1
    }
    pub unsafe fn m5u_rtc_device_get_irq_status(_kind: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_device_clear_irq(_kind: c_int) {}
    pub unsafe fn m5u_rtc_device_disable_irq(_kind: c_int) {}

    pub unsafe fn m5u_battery_level() -> c_int {
        100
    }
    pub unsafe fn m5u_battery_voltage_mv() -> c_int {
        4200
    }
    pub unsafe fn m5u_power_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_get_type() -> c_int {
        0
    }
    pub unsafe fn m5u_power_get_charge_state() -> c_int {
        2
    }
    pub unsafe fn m5u_power_is_charging() -> bool {
        false
    }
    pub unsafe fn m5u_power_set_led(_brightness: u8) {}
    pub unsafe fn m5u_power_set_ext_output(_enable: bool, _port_mask: u16) {}
    pub unsafe fn m5u_power_get_ext_output() -> bool {
        false
    }
    pub unsafe fn m5u_power_set_usb_output(_enable: bool) {}
    pub unsafe fn m5u_power_get_usb_output() -> bool {
        false
    }
    pub unsafe fn m5u_power_set_battery_charge(_enable: bool) {}
    pub unsafe fn m5u_power_set_charge_current(_max_ma: u16) {}
    pub unsafe fn m5u_power_set_charge_voltage(_max_mv: u16) {}
    pub unsafe fn m5u_power_get_vbus_voltage_mv() -> c_int {
        -1
    }
    pub unsafe fn m5u_power_get_battery_current_ma() -> c_int {
        0
    }
    pub unsafe fn m5u_power_get_ext_voltage_mv(_port_mask: u16) -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_get_ext_current_ma(_port_mask: u16) -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_get_key_state() -> u8 {
        0
    }
    pub unsafe fn m5u_power_set_ext_port_bus_config(_config: *const m5u_power_ext_port_bus_t) {}
    pub unsafe fn m5u_power_set_vibration(_level: u8) {}
    pub unsafe fn m5u_power_power_off() {}
    pub unsafe fn m5u_power_timer_sleep_seconds(_seconds: c_int) {}
    pub unsafe fn m5u_power_timer_sleep_time(_time: *const m5u_rtc_datetime_t) {}
    pub unsafe fn m5u_power_timer_sleep_date_time(
        _date: *const m5u_rtc_datetime_t,
        _time: *const m5u_rtc_datetime_t,
    ) {
    }
    pub unsafe fn m5u_power_deep_sleep_us(_micro_seconds: u64, _touch_wakeup: bool) {}
    pub unsafe fn m5u_power_light_sleep_us(_micro_seconds: u64, _touch_wakeup: bool) {}

    pub unsafe fn m5u_display_get_rotation() -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_brightness(_brightness: u8) {}
    pub unsafe fn m5u_display_set_epd_fastest() {}
    pub unsafe fn m5u_display_set_epd_mode(_mode: c_int) {}
    pub unsafe fn m5u_display_set_text_scroll(_scroll: bool) {}
    pub unsafe fn m5u_display_set_font(font: c_int) -> bool {
        (0..=3).contains(&font)
    }
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
    pub unsafe fn m5u_display_index_for_kinds(_kinds: *const c_int, _len: usize) -> c_int {
        -1
    }
    pub unsafe fn m5u_display_width_at(_index: c_int) -> c_int {
        320
    }
    pub unsafe fn m5u_display_height_at(_index: c_int) -> c_int {
        240
    }
    pub unsafe fn m5u_display_fill_screen_at(_index: c_int, _color: u16) {}
    pub unsafe fn m5u_display_set_cursor_at(_index: c_int, _x: c_int, _y: c_int) {}
    pub unsafe fn m5u_display_set_text_size_at(_index: c_int, _size: c_int) {}
    pub unsafe fn m5u_display_set_text_color_at(_index: c_int, _fg: u16, _bg: u16) {}
    pub unsafe fn m5u_display_get_rotation_at(_index: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_rotation_at(_index: c_int, _rotation: c_int) {}
    pub unsafe fn m5u_display_set_color_at(_index: c_int, _color: u16) {}
    pub unsafe fn m5u_display_start_write_at(_index: c_int) {}
    pub unsafe fn m5u_display_end_write_at(_index: c_int) {}
    pub unsafe fn m5u_display_print_at(_index: c_int, _text: *const c_char) {}
    pub unsafe fn m5u_display_println_at(_index: c_int, _text: *const c_char) {}
    pub unsafe fn m5u_display_draw_string_at(
        _index: c_int,
        _text: *const c_char,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }
    pub unsafe fn m5u_display_draw_line_at(
        _index: c_int,
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_rect_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_fill_rect_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_circle_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_fill_circle_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_write_pixel_at(_index: c_int, _x: c_int, _y: c_int, _color: u16) {}
    pub unsafe fn m5u_display_draw_pixel_at(_index: c_int, _x: c_int, _y: c_int, _color: u16) {}

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
    pub unsafe fn m5u_button_was_single_clicked(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_double_clicked(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_change_pressed(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_is_released(_button: c_int) -> bool {
        true
    }
    pub unsafe fn m5u_button_was_released_after_hold(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_release_for(_button: c_int, _ms: u32) -> bool {
        false
    }
    pub unsafe fn m5u_button_pressed_for(_button: c_int, _ms: u32) -> bool {
        false
    }
    pub unsafe fn m5u_button_released_for(_button: c_int, _ms: u32) -> bool {
        true
    }
    pub unsafe fn m5u_button_set_debounce_thresh(_button: c_int, _ms: u32) {}
    pub unsafe fn m5u_button_set_hold_thresh(_button: c_int, _ms: u32) {}
    pub unsafe fn m5u_button_set_raw_state(_button: c_int, _msec: u32, _press: bool) {}
    pub unsafe fn m5u_button_set_state(_button: c_int, _msec: u32, _state: u8) {}
    pub unsafe fn m5u_button_get_state(_button: c_int) -> u8 {
        0
    }
    pub unsafe fn m5u_button_last_change(_button: c_int) -> u32 {
        0
    }
    pub unsafe fn m5u_button_get_debounce_thresh(_button: c_int) -> u32 {
        10
    }
    pub unsafe fn m5u_button_get_hold_thresh(_button: c_int) -> u32 {
        500
    }
    pub unsafe fn m5u_button_get_update_msec(_button: c_int) -> u32 {
        0
    }

    pub unsafe fn m5u_mic_is_enabled() -> bool {
        true
    }
    pub unsafe fn m5u_mic_is_running() -> bool {
        false
    }
    pub unsafe fn m5u_mic_is_recording() -> bool {
        false
    }
    pub unsafe fn m5u_mic_recording_state() -> usize {
        0
    }
    pub unsafe fn m5u_mic_end() {}
    pub unsafe fn m5u_mic_record_i16_at(
        buffer: *mut i16,
        samples: usize,
        _sample_rate_hz: u32,
    ) -> bool {
        m5u_mic_record_i16(buffer, samples)
    }
    pub unsafe fn m5u_mic_record_i16_ex(
        buffer: *mut i16,
        samples: usize,
        _sample_rate_hz: u32,
        _stereo: bool,
    ) -> bool {
        m5u_mic_record_i16(buffer, samples)
    }
    pub unsafe fn m5u_mic_record_u8_ex(
        buffer: *mut u8,
        samples: usize,
        _sample_rate_hz: u32,
        _stereo: bool,
    ) -> bool {
        m5u_mic_record_u8(buffer, samples)
    }
    pub unsafe fn m5u_mic_set_sample_rate(_sample_rate_hz: u32) {}
    pub unsafe fn m5u_mic_get_noise_filter_level() -> c_int {
        0
    }
    pub unsafe fn m5u_mic_set_noise_filter_level(_level: c_int) -> bool {
        true
    }
    pub unsafe fn m5u_mic_get_config(out: *mut m5u_mic_config_t) -> bool {
        if !out.is_null() {
            *out = m5u_mic_config_t::default();
        }
        true
    }
    pub unsafe fn m5u_mic_set_config(config: *const m5u_mic_config_t) -> bool {
        !config.is_null()
    }

    pub unsafe fn m5u_speaker_is_enabled() -> bool {
        true
    }
    pub unsafe fn m5u_speaker_is_running() -> bool {
        false
    }
    pub unsafe fn m5u_speaker_end() {}
    pub unsafe fn m5u_speaker_get_volume() -> u8 {
        64
    }
    pub unsafe fn m5u_speaker_get_config(out: *mut m5u_speaker_config_t) -> bool {
        if !out.is_null() {
            *out = m5u_speaker_config_t::default();
        }
        true
    }
    pub unsafe fn m5u_speaker_set_config(config: *const m5u_speaker_config_t) -> bool {
        !config.is_null()
    }
    pub unsafe fn m5u_speaker_tone_ex(
        _frequency_hz: c_float,
        _duration_ms: u32,
        _channel: c_int,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_tone_options(
        _frequency_hz: c_float,
        _duration_ms: u32,
        _channel: c_int,
        _stop_current_sound: bool,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_tone_full(
        _frequency_hz: c_float,
        _duration_ms: u32,
        _channel: c_int,
        _stop_current_sound: bool,
        _raw_data: *const u8,
        _len: usize,
        _stereo: bool,
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
    pub unsafe fn m5u_speaker_play_u8_ex(
        _samples: *const u8,
        _len: usize,
        _sample_rate_hz: u32,
        _stereo: bool,
        _repeat: u32,
        _channel: c_int,
        _stop_current_sound: bool,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_play_i8_ex(
        _samples: *const i8,
        _len: usize,
        _sample_rate_hz: u32,
        _stereo: bool,
        _repeat: u32,
        _channel: c_int,
        _stop_current_sound: bool,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_play_i16_ex(
        _samples: *const i16,
        _len: usize,
        _sample_rate_hz: u32,
        _stereo: bool,
        _repeat: u32,
        _channel: c_int,
        _stop_current_sound: bool,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_play_wav(_data: *const u8, _len: usize) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_play_wav_ex(
        _data: *const u8,
        _len: usize,
        _repeat: u32,
        _channel: c_int,
        _stop_current_sound: bool,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_speaker_is_playing(_channel: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_speaker_playing_channels() -> usize {
        0
    }
    pub unsafe fn m5u_speaker_channel_playing_state(_channel: c_int) -> usize {
        0
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
    pub unsafe fn m5u_imu_update_mask() -> c_int {
        0
    }
    pub unsafe fn m5u_imu_sleep() -> bool {
        true
    }
    pub unsafe fn m5u_imu_set_clock(_freq: u32) {}
    pub unsafe fn m5u_imu_set_axis_order(_axis0: c_int, _axis1: c_int, _axis2: c_int) -> bool {
        true
    }
    pub unsafe fn m5u_imu_set_axis_order_right_handed(_axis0: c_int, _axis1: c_int) -> bool {
        true
    }
    pub unsafe fn m5u_imu_set_axis_order_left_handed(_axis0: c_int, _axis1: c_int) -> bool {
        true
    }
    pub unsafe fn m5u_imu_set_int_pin_active_logic(_level: bool) -> bool {
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
    pub unsafe fn m5u_imu_set_calibration_strength(_accel: u8, _gyro: u8, _mag: u8) {}
    pub unsafe fn m5u_imu_clear_offset_data() {}
    pub unsafe fn m5u_imu_set_offset_data(_index: usize, _value: i32) {}
    pub unsafe fn m5u_imu_get_offset_data_i32(_index: usize) -> i32 {
        0
    }
    pub unsafe fn m5u_imu_get_raw_data(_index: usize) -> i16 {
        0
    }

    pub unsafe fn m5u_touch_get_detail(_index: c_int, out: *mut m5u_touch_detail_t) -> bool {
        if !out.is_null() {
            *out = m5u_touch_detail_t::default();
        }
        false
    }
    pub unsafe fn m5u_touch_is_enabled() -> bool {
        false
    }
    pub unsafe fn m5u_touch_set_hold_thresh(_ms: u16) {}
    pub unsafe fn m5u_touch_set_flick_thresh(_distance: u16) {}
    pub unsafe fn m5u_rtc_set_system_time_from_rtc() {}
    pub unsafe fn m5u_rtc_set_timer_irq(timer_msec: u32) -> u32 {
        timer_msec
    }
    pub unsafe fn m5u_rtc_set_alarm_irq_after_seconds(after_seconds: c_int) -> c_int {
        after_seconds
    }
    pub unsafe fn m5u_rtc_set_alarm_irq_datetime(datetime: *const m5u_rtc_datetime_t) -> c_int {
        if datetime.is_null() {
            -1
        } else {
            0
        }
    }
    pub unsafe fn m5u_rtc_set_alarm_irq_time(time: *const m5u_rtc_datetime_t) -> c_int {
        if time.is_null() {
            -1
        } else {
            0
        }
    }
    pub unsafe fn m5u_rtc_get_irq_status() -> bool {
        false
    }
    pub unsafe fn m5u_rtc_clear_irq() {}
    pub unsafe fn m5u_rtc_disable_irq() {}

    pub unsafe fn m5u_power_axp192_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_get_battery_level() -> c_int {
        -1
    }
    pub unsafe fn m5u_power_axp192_set_battery_charge(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_charge_current(_max_ma: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_charge_voltage(_max_mv: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_is_charging() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_dcdc(_channel: u8, _voltage_mv: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_ldo(_channel: u8, _voltage_mv: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_gpio(_gpio_num: u8, _state: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_power_off() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_adc_state(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_adc_rate(_rate: u8) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_exten(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_set_backup(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_is_acin() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_is_vbus() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_get_bat_state() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_get_exten() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp192_get_battery_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_battery_discharge_current_ma() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_battery_charge_current_ma() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_battery_power_mw() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_acin_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_acin_current_ma() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_vbus_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_vbus_current_ma() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_aps_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_internal_temperature_c() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp192_get_pek_press() -> u8 {
        0
    }
    pub unsafe fn m5u_power_aw32001_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_aw32001_set_battery_charge(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_aw32001_set_charge_current(_max_ma: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_aw32001_set_charge_voltage(_max_mv: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_aw32001_is_charging() -> bool {
        false
    }
    pub unsafe fn m5u_power_aw32001_get_charge_current() -> u16 {
        0
    }
    pub unsafe fn m5u_power_aw32001_get_charge_voltage() -> u16 {
        0
    }
    pub unsafe fn m5u_power_aw32001_get_charge_status() -> c_int {
        -1
    }
    pub unsafe fn m5u_power_bq27220_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_bq27220_get_current_ma() -> i16 {
        0
    }
    pub unsafe fn m5u_power_bq27220_get_voltage_mv() -> i16 {
        0
    }
    pub unsafe fn m5u_power_bq27220_get_current_a() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_bq27220_get_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_ina226_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_ina226_config(_config: *const m5u_power_ina226_config_t) -> bool {
        false
    }
    pub unsafe fn m5u_power_ina226_get_bus_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_ina226_get_shunt_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_ina226_get_shunt_current_a() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_ina226_get_power_w() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_ina3221_begin(_index: usize) -> bool {
        false
    }
    pub unsafe fn m5u_power_ina3221_get_bus_voltage_v(_index: usize, _channel: u8) -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_ina3221_get_shunt_voltage_v(_index: usize, _channel: u8) -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_ina3221_get_current_a(_index: usize, _channel: u8) -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_ina3221_get_bus_voltage_mv(_index: usize, _channel: u8) -> i32 {
        0
    }
    pub unsafe fn m5u_power_ina3221_get_shunt_voltage_mv(_index: usize, _channel: u8) -> i32 {
        0
    }
    pub unsafe fn m5u_power_ina3221_set_shunt_res(_index: usize, _channel: u8, _res: u32) -> bool {
        false
    }
    pub unsafe fn m5u_power_ip5306_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_ip5306_get_battery_level() -> c_int {
        -1
    }
    pub unsafe fn m5u_power_ip5306_set_battery_charge(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_ip5306_set_charge_current(_max_ma: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_ip5306_set_charge_voltage(_max_mv: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_ip5306_is_charging() -> bool {
        false
    }
    pub unsafe fn m5u_power_ip5306_set_power_boost_keep_on(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_py32pmic_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_py32pmic_set_ext_output(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_py32pmic_set_battery_charge(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_py32pmic_set_charge_current(_max_ma: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_py32pmic_set_charge_voltage(_max_mv: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_py32pmic_is_charging() -> bool {
        false
    }
    pub unsafe fn m5u_power_py32pmic_get_charge_current() -> u16 {
        0
    }
    pub unsafe fn m5u_power_py32pmic_get_charge_voltage() -> u16 {
        0
    }
    pub unsafe fn m5u_power_py32pmic_get_pek_press() -> u8 {
        0
    }
    pub unsafe fn m5u_power_py32pmic_power_off() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_get_battery_level() -> c_int {
        -1
    }
    pub unsafe fn m5u_power_axp2101_set_battery_charge(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_set_pre_charge_current(_max_ma: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_set_charge_current(_max_ma: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_set_charge_voltage(_max_mv: u16) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_get_charge_status() -> c_int {
        -2
    }
    pub unsafe fn m5u_power_axp2101_is_charging() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_set_ldo(
        _kind: c_int,
        _channel: c_int,
        _voltage_mv: c_int,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_get_ldo_enabled(_kind: c_int, _channel: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_power_off() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_set_adc_state(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_set_adc_rate(_rate: u8) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_set_backup(_enable: bool) -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_is_acin() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_is_vbus() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_get_bat_state() -> bool {
        false
    }
    pub unsafe fn m5u_power_axp2101_get_battery_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_battery_discharge_current_ma() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_battery_charge_current_ma() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_battery_power_mw() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_acin_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_acin_current_ma() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_vbus_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_vbus_current_ma() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_ts_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_aps_voltage_v() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_internal_temperature_c() -> c_float {
        0.0
    }
    pub unsafe fn m5u_power_axp2101_get_pek_press() -> u8 {
        0
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
    pub unsafe fn m5u_led_set_colors_rgb(
        _colors: *const m5u_led_color_t,
        _index: usize,
        _length: usize,
    ) {
    }
    pub unsafe fn m5u_led_get_type(_index: usize) -> c_int {
        0
    }
    pub unsafe fn m5u_led_is_enabled() -> bool {
        false
    }

    pub unsafe fn m5u_log_print(_text: *const c_char) {}
    pub unsafe fn m5u_log_println(_text: *const c_char) {}
    pub unsafe fn m5u_log_println_empty() {}
    pub unsafe fn m5u_log_level(_level: c_int, _text: *const c_char) {}
    pub unsafe fn m5u_log_dump(_addr: *const c_void, _len: u32, _level: c_int) {}
    pub unsafe fn m5u_log_path_to_file_name(path: *const c_char) -> *const c_char {
        if path.is_null() {
            return path;
        }
        let mut current = path;
        let mut file = path;
        while *current != 0 {
            if *current == b'/' as c_char || *current == b'\\' as c_char {
                file = current.add(1);
            }
            current = current.add(1);
        }
        file
    }
    pub unsafe fn m5u_log_set_callback(
        _callback: m5u_log_callback_t,
        _user_data: *mut c_void,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_log_set_enable_color(target: c_int, _enable: bool) -> bool {
        (0..=2).contains(&target)
    }
    pub unsafe fn m5u_log_get_enable_color(target: c_int) -> bool {
        (0..=2).contains(&target)
    }
    pub unsafe fn m5u_log_set_level(target: c_int, level: c_int) -> bool {
        (0..=2).contains(&target) && (0..=5).contains(&level)
    }
    pub unsafe fn m5u_log_get_level(target: c_int) -> c_int {
        if (0..=2).contains(&target) {
            3
        } else {
            -1
        }
    }
    pub unsafe fn m5u_log_set_suffix(target: c_int, suffix: *const c_char) -> bool {
        (0..=2).contains(&target) && !suffix.is_null()
    }
    pub unsafe fn m5u_sd_begin() -> bool {
        false
    }
    pub unsafe fn m5u_sd_begin_spi(_config: *const m5u_sd_spi_config_t) -> bool {
        false
    }
    pub unsafe fn m5u_sd_is_mounted() -> bool {
        false
    }
    pub unsafe fn m5u_sd_end() {}
}

#[cfg(not(target_os = "espidf"))]
pub use host_stubs::*;
