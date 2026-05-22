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

use core::ffi::{c_char, c_float, c_int, c_void};

pub const M5U_CARDPUTER_KEYBOARD_WORD_CAPACITY: usize = 32;
pub const M5U_CARDPUTER_KEYBOARD_HID_CAPACITY: usize = 32;
pub const M5U_CARDPUTER_KEYBOARD_MODIFIER_CAPACITY: usize = 8;
pub const M5U_CARDPUTER_SD_DIR_ENTRY_NAME_CAPACITY: usize = 64;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_config_t {
    pub serial_baudrate: u32,
    pub clear_display: bool,
    pub output_power: bool,
    pub pmic_button: bool,
    pub internal_imu: bool,
    pub internal_rtc: bool,
    pub internal_mic: bool,
    pub internal_spk: bool,
    pub external_imu: bool,
    pub external_rtc: bool,
    pub disable_rtc_irq: bool,
    pub led_brightness: u8,
    pub external_speaker_value: u16,
    pub external_display_value: u16,
}

impl Default for m5u_config_t {
    fn default() -> Self {
        Self {
            serial_baudrate: 115_200,
            clear_display: true,
            output_power: true,
            pmic_button: true,
            internal_imu: true,
            internal_rtc: true,
            internal_mic: true,
            internal_spk: true,
            external_imu: false,
            external_rtc: false,
            disable_rtc_irq: true,
            led_brightness: 0,
            external_speaker_value: 0x0000,
            external_display_value: 0xffff,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct m5u_touch_detail_t {
    pub x: c_int,
    pub y: c_int,
    pub size: u16,
    pub id: u8,
    pub prev_x: c_int,
    pub prev_y: c_int,
    pub base_x: c_int,
    pub base_y: c_int,
    pub base_msec: u32,
    pub state: c_int,
    pub is_pressed: bool,
    pub was_pressed: bool,
    pub is_released: bool,
    pub was_released: bool,
    pub was_clicked: bool,
    pub was_hold: bool,
    pub is_holding: bool,
    pub was_flick_start: bool,
    pub is_flicking: bool,
    pub was_flicked: bool,
    pub was_drag_start: bool,
    pub is_dragging: bool,
    pub was_dragged: bool,
    pub click_count: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct m5u_touch_point_t {
    pub x: c_int,
    pub y: c_int,
    pub size: u16,
    pub id: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct m5u_stackchan_motion_status_t {
    pub ready: bool,
    pub moving: bool,
    pub yaw_tenths: i16,
    pub pitch_tenths: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_cardputer_keyboard_state_t {
    pub tab: bool,
    pub fn_key: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub opt: bool,
    pub alt: bool,
    pub del: bool,
    pub enter: bool,
    pub space: bool,
    pub modifiers: u8,
    pub word_len: usize,
    pub word: [u8; M5U_CARDPUTER_KEYBOARD_WORD_CAPACITY],
    pub hid_len: usize,
    pub hid_keys: [u8; M5U_CARDPUTER_KEYBOARD_HID_CAPACITY],
    pub modifier_len: usize,
    pub modifier_keys: [u8; M5U_CARDPUTER_KEYBOARD_MODIFIER_CAPACITY],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct m5u_cardputer_key_value_t {
    pub first: u8,
    pub second: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct m5u_cardputer_sd_dir_entry_t {
    pub name: [c_char; M5U_CARDPUTER_SD_DIR_ENTRY_NAME_CAPACITY],
    pub is_directory: bool,
    pub size: u64,
}

impl Default for m5u_cardputer_sd_dir_entry_t {
    fn default() -> Self {
        Self {
            name: [0; M5U_CARDPUTER_SD_DIR_ENTRY_NAME_CAPACITY],
            is_directory: false,
            size: 0,
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
    pub left_channel: bool,
    pub stereo: bool,
    pub over_sampling: u8,
    pub magnification: u8,
    pub noise_filter_level: u8,
    pub use_adc: bool,
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
            left_channel: false,
            stereo: false,
            over_sampling: 2,
            magnification: 16,
            noise_filter_level: 0,
            use_adc: false,
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
    pub pin_ws: c_int,
    pub sample_rate: u32,
    pub stereo: bool,
    pub buzzer: bool,
    pub use_dac: bool,
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
            pin_ws: -1,
            sample_rate: 48_000,
            stereo: false,
            buzzer: false,
            use_dac: false,
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

impl Default for m5u_cardputer_keyboard_state_t {
    fn default() -> Self {
        Self {
            tab: false,
            fn_key: false,
            shift: false,
            ctrl: false,
            opt: false,
            alt: false,
            del: false,
            enter: false,
            space: false,
            modifiers: 0,
            word_len: 0,
            word: [0; M5U_CARDPUTER_KEYBOARD_WORD_CAPACITY],
            hid_len: 0,
            hid_keys: [0; M5U_CARDPUTER_KEYBOARD_HID_CAPACITY],
            modifier_len: 0,
            modifier_keys: [0; M5U_CARDPUTER_KEYBOARD_MODIFIER_CAPACITY],
        }
    }
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

    pub fn m5u_display_width() -> c_int;
    pub fn m5u_display_height() -> c_int;
    pub fn m5u_display_fill_screen(color: u16);
    pub fn m5u_display_set_cursor(x: c_int, y: c_int);
    pub fn m5u_display_set_text_size(size: c_int);
    pub fn m5u_display_set_text_color(fg: u16, bg: u16);
    pub fn m5u_display_print(text: *const c_char);
    pub fn m5u_display_println(text: *const c_char);
    pub fn m5u_display_draw_line(x0: c_int, y0: c_int, x1: c_int, y1: c_int, color: u16);
    pub fn m5u_display_draw_pixel(x: c_int, y: c_int, color: u16);
    pub fn m5u_display_read_pixel(x: c_int, y: c_int) -> u16;
    pub fn m5u_display_draw_fast_hline(x: c_int, y: c_int, w: c_int, color: u16);
    pub fn m5u_display_draw_fast_vline(x: c_int, y: c_int, h: c_int, color: u16);
    pub fn m5u_display_draw_rect(x: c_int, y: c_int, w: c_int, h: c_int, color: u16);
    pub fn m5u_display_fill_rect(x: c_int, y: c_int, w: c_int, h: c_int, color: u16);
    pub fn m5u_display_fill_rect_alpha(
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        alpha: u8,
        color: u16,
    );
    pub fn m5u_display_draw_round_rect(
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        r: c_int,
        color: u16,
    );
    pub fn m5u_display_fill_round_rect(
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        r: c_int,
        color: u16,
    );
    pub fn m5u_display_draw_circle(x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_display_fill_circle(x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_display_draw_ellipse(x: c_int, y: c_int, rx: c_int, ry: c_int, color: u16);
    pub fn m5u_display_fill_ellipse(x: c_int, y: c_int, rx: c_int, ry: c_int, color: u16);
    pub fn m5u_display_draw_arc(
        x: c_int,
        y: c_int,
        r0: c_int,
        r1: c_int,
        angle0: c_float,
        angle1: c_float,
        color: u16,
    );
    pub fn m5u_display_fill_arc(
        x: c_int,
        y: c_int,
        r0: c_int,
        r1: c_int,
        angle0: c_float,
        angle1: c_float,
        color: u16,
    );
    pub fn m5u_display_draw_triangle(
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        color: u16,
    );
    pub fn m5u_display_fill_triangle(
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        color: u16,
    );
    pub fn m5u_display_progress_bar(x: c_int, y: c_int, w: c_int, h: c_int, value: u8);
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
    pub fn m5u_imu_get_mag(x: *mut f32, y: *mut f32, z: *mut f32) -> bool;
    pub fn m5u_imu_get_temp_c(temp: *mut f32) -> bool;

    pub fn m5u_touch_begin();
    pub fn m5u_touch_update(msec: u32);
    pub fn m5u_touch_is_enabled() -> bool;
    pub fn m5u_touch_end();
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
    pub fn m5u_battery_current_ma() -> c_int;
    pub fn m5u_power_is_charging() -> bool;
    pub fn m5u_power_charging_state() -> c_int;
    pub fn m5u_power_begin() -> bool;
    pub fn m5u_power_set_ext_output(enable: bool);
    pub fn m5u_power_get_ext_output() -> bool;
    pub fn m5u_power_set_usb_output(enable: bool);
    pub fn m5u_power_get_usb_output() -> bool;
    pub fn m5u_power_set_led(brightness: u8);
    pub fn m5u_power_power_off();
    pub fn m5u_power_timer_sleep(seconds: c_int);
    pub fn m5u_power_deep_sleep(micro_seconds: u64, touch_wakeup: bool);
    pub fn m5u_power_light_sleep(micro_seconds: u64, touch_wakeup: bool);
    pub fn m5u_power_set_battery_charge(enable: bool);
    pub fn m5u_power_set_charge_current(max_ma: u16);
    pub fn m5u_power_set_charge_voltage(max_mv: u16);
    pub fn m5u_power_get_key_state() -> u8;
    pub fn m5u_power_set_vibration(level: u8);
    pub fn m5u_power_get_type() -> c_int;
    pub fn m5u_led_begin() -> bool;
    pub fn m5u_led_is_enabled() -> bool;
    pub fn m5u_led_count() -> usize;
    pub fn m5u_led_display();
    pub fn m5u_led_set_auto_display(enable: bool);
    pub fn m5u_led_set_brightness(brightness: u8);
    pub fn m5u_led_set_color(index: usize, rgb: u32);
    pub fn m5u_led_set_all_color(rgb: u32);

    pub fn m5u_display_get_rotation() -> c_int;
    pub fn m5u_display_set_brightness(brightness: u8);
    pub fn m5u_display_get_brightness() -> u8;
    pub fn m5u_display_set_color_depth(depth: u8);
    pub fn m5u_display_get_color_depth() -> u8;
    pub fn m5u_display_is_epd() -> bool;
    pub fn m5u_display_set_epd_mode(mode: c_int);
    pub fn m5u_display_get_epd_mode() -> c_int;
    pub fn m5u_display_set_epd_fastest();
    pub fn m5u_display_set_resolution(
        logical_width: u16,
        logical_height: u16,
        refresh_rate: c_float,
        output_width: u16,
        output_height: u16,
        scale_w: u8,
        scale_h: u8,
        pixel_clock: u32,
    ) -> bool;
    pub fn m5u_display_start_write();
    pub fn m5u_display_end_write();
    pub fn m5u_display_display();
    pub fn m5u_display_display_busy() -> bool;
    pub fn m5u_display_wait_display();
    pub fn m5u_display_sleep();
    pub fn m5u_display_wakeup();
    pub fn m5u_display_power_save_on();
    pub fn m5u_display_power_save_off();
    pub fn m5u_display_power_save(enable: bool);
    pub fn m5u_display_invert_display(invert: bool);
    pub fn m5u_display_get_cursor_x() -> c_int;
    pub fn m5u_display_get_cursor_y() -> c_int;
    pub fn m5u_display_set_pivot(x: c_float, y: c_float);
    pub fn m5u_display_get_pivot_x() -> c_float;
    pub fn m5u_display_get_pivot_y() -> c_float;
    pub fn m5u_display_font_height() -> c_int;
    pub fn m5u_display_font_width() -> c_int;
    pub fn m5u_display_set_font(font: c_int) -> bool;
    pub fn m5u_display_show_font(duration_ms: u32) -> bool;
    pub fn m5u_display_unload_font();
    pub fn m5u_display_font_height_for(font: c_int) -> c_int;
    pub fn m5u_display_font_width_for(font: c_int) -> c_int;
    pub fn m5u_display_get_base_color() -> u16;
    pub fn m5u_display_set_base_color(color: u16);
    pub fn m5u_display_set_color(color: u16);
    pub fn m5u_display_set_rgb_color(r: u8, g: u8, b: u8);
    pub fn m5u_display_set_raw_color(color: u32);
    pub fn m5u_display_get_raw_color() -> u32;
    pub fn m5u_display_get_palette_count() -> u32;
    pub fn m5u_display_set_swap_bytes(swap: bool);
    pub fn m5u_display_get_swap_bytes() -> bool;
    pub fn m5u_display_swap565(r: u8, g: u8, b: u8) -> u16;
    pub fn m5u_display_swap888(r: u8, g: u8, b: u8) -> u32;
    pub fn m5u_display_set_text_wrap(wrap_x: bool, wrap_y: bool);
    pub fn m5u_display_set_text_datum(datum: c_int);
    pub fn m5u_display_get_text_datum() -> c_int;
    pub fn m5u_display_set_text_padding(padding_x: u32);
    pub fn m5u_display_get_text_padding() -> u32;
    pub fn m5u_display_get_text_size_x() -> u8;
    pub fn m5u_display_get_text_size_y() -> u8;
    pub fn m5u_display_text_length(text: *const c_char) -> c_int;
    pub fn m5u_display_text_width(text: *const c_char) -> c_int;
    pub fn m5u_display_draw_center_string(text: *const c_char, x: c_int, y: c_int) -> c_int;
    pub fn m5u_display_draw_string(text: *const c_char, x: c_int, y: c_int) -> c_int;
    pub fn m5u_display_draw_char(codepoint: u32, x: c_int, y: c_int) -> c_int;
    pub fn m5u_display_draw_number(value: i32, x: c_int, y: c_int) -> c_int;
    pub fn m5u_display_draw_float(value: c_float, decimals: u8, x: c_int, y: c_int) -> c_int;
    pub fn m5u_display_draw_bmp(
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_display_draw_jpg(
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_display_draw_png(
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_display_push_image_rgb565(
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        data: *const u16,
        len: usize,
    ) -> bool;
    pub fn m5u_display_write_pixel(x: c_int, y: c_int, color: u16);
    pub fn m5u_display_write_fast_vline(x: c_int, y: c_int, h: c_int, color: u16);
    pub fn m5u_display_set_addr_window(x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn m5u_display_set_window(xs: c_int, ys: c_int, xe: c_int, ye: c_int);
    pub fn m5u_display_set_clip_rect(x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn m5u_display_get_clip_rect(x: *mut c_int, y: *mut c_int, w: *mut c_int, h: *mut c_int);
    pub fn m5u_display_clear_clip_rect();
    pub fn m5u_display_scroll(dx: c_int, dy: c_int);
    pub fn m5u_display_set_text_scroll(enable: bool);
    pub fn m5u_display_set_scroll_rect(x: c_int, y: c_int, w: c_int, h: c_int, color: u16);
    pub fn m5u_display_get_scroll_rect(x: *mut c_int, y: *mut c_int, w: *mut c_int, h: *mut c_int);
    pub fn m5u_display_clear_scroll_rect();
    pub fn m5u_display_color888(r: u8, g: u8, b: u8) -> u16;
    pub fn m5u_canvas_create_for_display() -> *mut c_void;
    pub fn m5u_canvas_create_for_cardputer_display() -> *mut c_void;
    pub fn m5u_canvas_delete(canvas: *mut c_void);
    pub fn m5u_canvas_create_sprite(canvas: *mut c_void, w: c_int, h: c_int) -> bool;
    pub fn m5u_canvas_delete_sprite(canvas: *mut c_void);
    pub fn m5u_canvas_push_sprite(canvas: *mut c_void, x: c_int, y: c_int);
    pub fn m5u_canvas_width(canvas: *mut c_void) -> c_int;
    pub fn m5u_canvas_height(canvas: *mut c_void) -> c_int;
    pub fn m5u_canvas_fill_screen(canvas: *mut c_void, color: u16);
    pub fn m5u_canvas_set_cursor(canvas: *mut c_void, x: c_int, y: c_int);
    pub fn m5u_canvas_set_text_size(canvas: *mut c_void, size: c_float);
    pub fn m5u_canvas_set_text_color(canvas: *mut c_void, fg: u16, bg: u16);
    pub fn m5u_canvas_set_text_scroll(canvas: *mut c_void, enable: bool);
    pub fn m5u_canvas_set_text_datum(canvas: *mut c_void, datum: c_int);
    pub fn m5u_canvas_get_text_datum(canvas: *mut c_void) -> c_int;
    pub fn m5u_canvas_set_text_padding(canvas: *mut c_void, padding_x: u32);
    pub fn m5u_canvas_get_text_padding(canvas: *mut c_void) -> u32;
    pub fn m5u_canvas_get_text_size_x(canvas: *mut c_void) -> u8;
    pub fn m5u_canvas_get_text_size_y(canvas: *mut c_void) -> u8;
    pub fn m5u_canvas_get_base_color(canvas: *mut c_void) -> u16;
    pub fn m5u_canvas_set_base_color(canvas: *mut c_void, color: u16);
    pub fn m5u_canvas_set_color(canvas: *mut c_void, color: u16);
    pub fn m5u_canvas_set_rgb_color(canvas: *mut c_void, r: u8, g: u8, b: u8);
    pub fn m5u_canvas_set_raw_color(canvas: *mut c_void, color: u32);
    pub fn m5u_canvas_get_raw_color(canvas: *mut c_void) -> u32;
    pub fn m5u_canvas_set_swap_bytes(canvas: *mut c_void, swap: bool);
    pub fn m5u_canvas_get_swap_bytes(canvas: *mut c_void) -> bool;
    pub fn m5u_canvas_set_font(canvas: *mut c_void, font: c_int) -> bool;
    pub fn m5u_canvas_font_height(canvas: *mut c_void) -> c_int;
    pub fn m5u_canvas_font_width(canvas: *mut c_void) -> c_int;
    pub fn m5u_canvas_show_font(canvas: *mut c_void, duration_ms: u32) -> bool;
    pub fn m5u_canvas_unload_font(canvas: *mut c_void);
    pub fn m5u_canvas_text_width(canvas: *mut c_void, text: *const c_char) -> c_int;
    pub fn m5u_canvas_print(canvas: *mut c_void, text: *const c_char);
    pub fn m5u_canvas_println(canvas: *mut c_void, text: *const c_char);
    pub fn m5u_canvas_draw_center_string(
        canvas: *mut c_void,
        text: *const c_char,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn m5u_canvas_draw_string(
        canvas: *mut c_void,
        text: *const c_char,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn m5u_canvas_draw_line(
        canvas: *mut c_void,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        color: u16,
    );
    pub fn m5u_canvas_draw_rect(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        color: u16,
    );
    pub fn m5u_canvas_fill_rect(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        color: u16,
    );
    pub fn m5u_canvas_draw_circle(canvas: *mut c_void, x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_canvas_fill_circle(canvas: *mut c_void, x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_canvas_draw_pixel(canvas: *mut c_void, x: c_int, y: c_int, color: u16);
    pub fn m5u_canvas_read_pixel(canvas: *mut c_void, x: c_int, y: c_int) -> u16;
    pub fn m5u_canvas_draw_fast_hline(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        color: u16,
    );
    pub fn m5u_canvas_draw_fast_vline(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        h: c_int,
        color: u16,
    );
    pub fn m5u_canvas_draw_round_rect(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        r: c_int,
        color: u16,
    );
    pub fn m5u_canvas_fill_round_rect(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        r: c_int,
        color: u16,
    );
    pub fn m5u_canvas_draw_ellipse(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        rx: c_int,
        ry: c_int,
        color: u16,
    );
    pub fn m5u_canvas_fill_ellipse(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        rx: c_int,
        ry: c_int,
        color: u16,
    );
    pub fn m5u_canvas_draw_arc(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        r0: c_int,
        r1: c_int,
        angle0: c_float,
        angle1: c_float,
        color: u16,
    );
    pub fn m5u_canvas_fill_arc(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        r0: c_int,
        r1: c_int,
        angle0: c_float,
        angle1: c_float,
        color: u16,
    );
    pub fn m5u_canvas_draw_triangle(
        canvas: *mut c_void,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        color: u16,
    );
    pub fn m5u_canvas_fill_triangle(
        canvas: *mut c_void,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        color: u16,
    );
    pub fn m5u_canvas_progress_bar(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        value: u8,
    );
    pub fn m5u_canvas_text_length(canvas: *mut c_void, text: *const c_char) -> c_int;
    pub fn m5u_canvas_draw_char(canvas: *mut c_void, codepoint: u32, x: c_int, y: c_int) -> c_int;
    pub fn m5u_canvas_draw_number(canvas: *mut c_void, value: i32, x: c_int, y: c_int) -> c_int;
    pub fn m5u_canvas_draw_float(
        canvas: *mut c_void,
        value: c_float,
        decimals: u8,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn m5u_canvas_draw_bmp(
        canvas: *mut c_void,
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_canvas_draw_jpg(
        canvas: *mut c_void,
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_canvas_draw_png(
        canvas: *mut c_void,
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_canvas_push_image_rgb565(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        data: *const u16,
        len: usize,
    ) -> bool;
    pub fn m5u_canvas_write_pixel(canvas: *mut c_void, x: c_int, y: c_int, color: u16);
    pub fn m5u_canvas_write_fast_vline(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        h: c_int,
        color: u16,
    );
    pub fn m5u_canvas_set_addr_window(canvas: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn m5u_canvas_set_window(canvas: *mut c_void, xs: c_int, ys: c_int, xe: c_int, ye: c_int);
    pub fn m5u_canvas_set_clip_rect(canvas: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn m5u_canvas_get_clip_rect(
        canvas: *mut c_void,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    );
    pub fn m5u_canvas_clear_clip_rect(canvas: *mut c_void);
    pub fn m5u_canvas_scroll(canvas: *mut c_void, dx: c_int, dy: c_int);
    pub fn m5u_canvas_set_scroll_rect(
        canvas: *mut c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        color: u16,
    );
    pub fn m5u_canvas_get_scroll_rect(
        canvas: *mut c_void,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    );
    pub fn m5u_canvas_clear_scroll_rect(canvas: *mut c_void);
    pub fn m5u_display_count() -> c_int;
    pub fn m5u_display_index_for_kind(kind: c_int) -> c_int;
    pub fn m5u_display_set_primary(index: c_int) -> bool;
    pub fn m5u_display_set_primary_kind(kind: c_int) -> bool;
    pub fn m5u_display_set_rotation_at(index: c_int, rotation: c_int);
    pub fn m5u_display_get_rotation_at(index: c_int) -> c_int;
    pub fn m5u_display_set_brightness_at(index: c_int, brightness: u8);
    pub fn m5u_display_get_brightness_at(index: c_int) -> u8;
    pub fn m5u_display_set_color_depth_at(index: c_int, depth: u8);
    pub fn m5u_display_get_color_depth_at(index: c_int) -> u8;
    pub fn m5u_display_is_epd_at(index: c_int) -> bool;
    pub fn m5u_display_set_epd_mode_at(index: c_int, mode: c_int);
    pub fn m5u_display_get_epd_mode_at(index: c_int) -> c_int;
    pub fn m5u_display_set_resolution_at(
        index: c_int,
        logical_width: u16,
        logical_height: u16,
        refresh_rate: c_float,
        output_width: u16,
        output_height: u16,
        scale_w: u8,
        scale_h: u8,
        pixel_clock: u32,
    ) -> bool;
    pub fn m5u_display_width_at(index: c_int) -> c_int;
    pub fn m5u_display_height_at(index: c_int) -> c_int;
    pub fn m5u_display_start_write_at(index: c_int);
    pub fn m5u_display_end_write_at(index: c_int);
    pub fn m5u_display_display_at(index: c_int);
    pub fn m5u_display_display_busy_at(index: c_int) -> bool;
    pub fn m5u_display_wait_display_at(index: c_int);
    pub fn m5u_display_sleep_at(index: c_int);
    pub fn m5u_display_wakeup_at(index: c_int);
    pub fn m5u_display_power_save_on_at(index: c_int);
    pub fn m5u_display_power_save_off_at(index: c_int);
    pub fn m5u_display_power_save_at(index: c_int, enable: bool);
    pub fn m5u_display_invert_display_at(index: c_int, invert: bool);
    pub fn m5u_display_get_cursor_x_at(index: c_int) -> c_int;
    pub fn m5u_display_get_cursor_y_at(index: c_int) -> c_int;
    pub fn m5u_display_set_pivot_at(index: c_int, x: c_float, y: c_float);
    pub fn m5u_display_get_pivot_x_at(index: c_int) -> c_float;
    pub fn m5u_display_get_pivot_y_at(index: c_int) -> c_float;
    pub fn m5u_display_clear_at(index: c_int, color: u16);
    pub fn m5u_display_set_cursor_at(index: c_int, x: c_int, y: c_int);
    pub fn m5u_display_set_text_size_at(index: c_int, size: c_int);
    pub fn m5u_display_set_text_color_at(index: c_int, fg: u16, bg: u16);
    pub fn m5u_display_set_text_datum_at(index: c_int, datum: c_int);
    pub fn m5u_display_get_text_datum_at(index: c_int) -> c_int;
    pub fn m5u_display_set_text_padding_at(index: c_int, padding_x: u32);
    pub fn m5u_display_get_text_padding_at(index: c_int) -> u32;
    pub fn m5u_display_get_text_size_x_at(index: c_int) -> u8;
    pub fn m5u_display_get_text_size_y_at(index: c_int) -> u8;
    pub fn m5u_display_font_height_at(index: c_int) -> c_int;
    pub fn m5u_display_font_width_at(index: c_int) -> c_int;
    pub fn m5u_display_set_font_at(index: c_int, font: c_int) -> bool;
    pub fn m5u_display_show_font_at(index: c_int, duration_ms: u32) -> bool;
    pub fn m5u_display_unload_font_at(index: c_int);
    pub fn m5u_display_get_base_color_at(index: c_int) -> u16;
    pub fn m5u_display_set_base_color_at(index: c_int, color: u16);
    pub fn m5u_display_set_color_at(index: c_int, color: u16);
    pub fn m5u_display_set_rgb_color_at(index: c_int, r: u8, g: u8, b: u8);
    pub fn m5u_display_set_raw_color_at(index: c_int, color: u32);
    pub fn m5u_display_get_raw_color_at(index: c_int) -> u32;
    pub fn m5u_display_get_palette_count_at(index: c_int) -> u32;
    pub fn m5u_display_set_swap_bytes_at(index: c_int, swap: bool);
    pub fn m5u_display_get_swap_bytes_at(index: c_int) -> bool;
    pub fn m5u_display_swap565_at(index: c_int, r: u8, g: u8, b: u8) -> u16;
    pub fn m5u_display_swap888_at(index: c_int, r: u8, g: u8, b: u8) -> u32;
    pub fn m5u_display_set_text_wrap_at(index: c_int, wrap_x: bool, wrap_y: bool);
    pub fn m5u_display_color888_at(index: c_int, r: u8, g: u8, b: u8) -> u16;
    pub fn m5u_display_text_length_at(index: c_int, text: *const c_char) -> c_int;
    pub fn m5u_display_text_width_at(index: c_int, text: *const c_char) -> c_int;
    pub fn m5u_display_print_at(index: c_int, text: *const c_char);
    pub fn m5u_display_println_at(index: c_int, text: *const c_char);
    pub fn m5u_display_draw_center_string_at(
        index: c_int,
        text: *const c_char,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn m5u_display_draw_string_at(
        index: c_int,
        text: *const c_char,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn m5u_display_draw_char_at(index: c_int, codepoint: u32, x: c_int, y: c_int) -> c_int;
    pub fn m5u_display_draw_number_at(index: c_int, value: i32, x: c_int, y: c_int) -> c_int;
    pub fn m5u_display_draw_float_at(
        index: c_int,
        value: c_float,
        decimals: u8,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn m5u_display_draw_bmp_at(
        index: c_int,
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_display_draw_jpg_at(
        index: c_int,
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_display_draw_png_at(
        index: c_int,
        data: *const u8,
        len: usize,
        x: c_int,
        y: c_int,
        max_width: c_int,
        max_height: c_int,
        off_x: c_int,
        off_y: c_int,
        scale_x: c_float,
        scale_y: c_float,
        datum: c_int,
    ) -> bool;
    pub fn m5u_display_draw_line_at(
        index: c_int,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        color: u16,
    );
    pub fn m5u_display_draw_pixel_at(index: c_int, x: c_int, y: c_int, color: u16);
    pub fn m5u_display_read_pixel_at(index: c_int, x: c_int, y: c_int) -> u16;
    pub fn m5u_display_draw_fast_hline_at(index: c_int, x: c_int, y: c_int, w: c_int, color: u16);
    pub fn m5u_display_draw_fast_vline_at(index: c_int, x: c_int, y: c_int, h: c_int, color: u16);
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
    pub fn m5u_display_fill_rect_alpha_at(
        index: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        alpha: u8,
        color: u16,
    );
    pub fn m5u_display_draw_round_rect_at(
        index: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        r: c_int,
        color: u16,
    );
    pub fn m5u_display_fill_round_rect_at(
        index: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        r: c_int,
        color: u16,
    );
    pub fn m5u_display_draw_circle_at(index: c_int, x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_display_fill_circle_at(index: c_int, x: c_int, y: c_int, r: c_int, color: u16);
    pub fn m5u_display_draw_ellipse_at(
        index: c_int,
        x: c_int,
        y: c_int,
        rx: c_int,
        ry: c_int,
        color: u16,
    );
    pub fn m5u_display_fill_ellipse_at(
        index: c_int,
        x: c_int,
        y: c_int,
        rx: c_int,
        ry: c_int,
        color: u16,
    );
    pub fn m5u_display_draw_arc_at(
        index: c_int,
        x: c_int,
        y: c_int,
        r0: c_int,
        r1: c_int,
        angle0: c_float,
        angle1: c_float,
        color: u16,
    );
    pub fn m5u_display_fill_arc_at(
        index: c_int,
        x: c_int,
        y: c_int,
        r0: c_int,
        r1: c_int,
        angle0: c_float,
        angle1: c_float,
        color: u16,
    );
    pub fn m5u_display_draw_triangle_at(
        index: c_int,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        color: u16,
    );
    pub fn m5u_display_fill_triangle_at(
        index: c_int,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        color: u16,
    );
    pub fn m5u_display_progress_bar_at(
        index: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        value: u8,
    );
    pub fn m5u_display_push_image_rgb565_at(
        index: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        data: *const u16,
        len: usize,
    ) -> bool;
    pub fn m5u_display_write_pixel_at(index: c_int, x: c_int, y: c_int, color: u16);
    pub fn m5u_display_write_fast_vline_at(index: c_int, x: c_int, y: c_int, h: c_int, color: u16);
    pub fn m5u_display_set_addr_window_at(index: c_int, x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn m5u_display_set_window_at(index: c_int, xs: c_int, ys: c_int, xe: c_int, ye: c_int);
    pub fn m5u_display_set_clip_rect_at(index: c_int, x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn m5u_display_get_clip_rect_at(
        index: c_int,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    );
    pub fn m5u_display_clear_clip_rect_at(index: c_int);
    pub fn m5u_display_scroll_at(index: c_int, dx: c_int, dy: c_int);
    pub fn m5u_display_set_text_scroll_at(index: c_int, enable: bool);
    pub fn m5u_display_set_scroll_rect_at(
        index: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        color: u16,
    );
    pub fn m5u_display_get_scroll_rect_at(
        index: c_int,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    );
    pub fn m5u_display_clear_scroll_rect_at(index: c_int);

    pub fn m5u_button_is_pressed(button: c_int) -> bool;
    pub fn m5u_button_was_pressed(button: c_int) -> bool;
    pub fn m5u_button_was_released(button: c_int) -> bool;
    pub fn m5u_button_was_clicked(button: c_int) -> bool;
    pub fn m5u_button_was_hold(button: c_int) -> bool;
    pub fn m5u_button_was_single_clicked(button: c_int) -> bool;
    pub fn m5u_button_was_double_clicked(button: c_int) -> bool;
    pub fn m5u_button_was_change_pressed(button: c_int) -> bool;
    pub fn m5u_button_is_holding(button: c_int) -> bool;
    pub fn m5u_button_is_released(button: c_int) -> bool;
    pub fn m5u_button_was_released_after_hold(button: c_int) -> bool;
    pub fn m5u_button_was_release_for(button: c_int, ms: u32) -> bool;
    pub fn m5u_button_pressed_for(button: c_int, ms: u32) -> bool;
    pub fn m5u_button_released_for(button: c_int, ms: u32) -> bool;
    pub fn m5u_button_was_decide_click_count(button: c_int) -> bool;
    pub fn m5u_button_get_click_count(button: c_int) -> c_int;
    pub fn m5u_button_set_debounce_thresh(button: c_int, msec: u32);
    pub fn m5u_button_set_hold_thresh(button: c_int, msec: u32);
    pub fn m5u_button_set_raw_state(button: c_int, msec: u32, press: bool);
    pub fn m5u_button_set_state(button: c_int, msec: u32, state: c_int);
    pub fn m5u_button_get_state(button: c_int) -> c_int;
    pub fn m5u_button_last_change(button: c_int) -> u32;
    pub fn m5u_button_get_debounce_thresh(button: c_int) -> u32;
    pub fn m5u_button_get_hold_thresh(button: c_int) -> u32;
    pub fn m5u_button_get_update_msec(button: c_int) -> u32;

    pub fn m5u_mic_is_enabled() -> bool;
    pub fn m5u_mic_is_recording() -> bool;
    pub fn m5u_mic_end();
    pub fn m5u_mic_record_i16_at(buffer: *mut i16, samples: usize, sample_rate_hz: u32) -> bool;
    pub fn m5u_mic_get_noise_filter_level() -> c_int;
    pub fn m5u_mic_set_noise_filter_level(level: c_int) -> bool;
    pub fn m5u_mic_get_config(out: *mut m5u_mic_config_t) -> bool;
    pub fn m5u_mic_set_config(config: *const m5u_mic_config_t);

    pub fn m5u_speaker_is_enabled() -> bool;
    pub fn m5u_speaker_is_running() -> bool;
    pub fn m5u_speaker_end();
    pub fn m5u_speaker_get_config(out: *mut m5u_speaker_config_t) -> bool;
    pub fn m5u_speaker_set_config(config: *const m5u_speaker_config_t);
    pub fn m5u_speaker_get_volume() -> u8;
    pub fn m5u_speaker_tone_ex(frequency_hz: c_float, duration_ms: u32, channel: c_int) -> bool;
    pub fn m5u_speaker_play_u8(samples: *const u8, len: usize, sample_rate_hz: u32) -> bool;
    pub fn m5u_speaker_play_wav(data: *const u8, len: usize) -> bool;
    pub fn m5u_speaker_play_i16_ex(
        samples: *const i16,
        len: usize,
        sample_rate_hz: u32,
        stereo: bool,
        repeat: u32,
        channel: c_int,
        stop_current_sound: bool,
    ) -> bool;
    pub fn m5u_speaker_play_u8_ex(
        samples: *const u8,
        len: usize,
        sample_rate_hz: u32,
        stereo: bool,
        repeat: u32,
        channel: c_int,
        stop_current_sound: bool,
    ) -> bool;
    pub fn m5u_speaker_play_wav_ex(
        data: *const u8,
        len: usize,
        repeat: u32,
        channel: c_int,
        stop_current_sound: bool,
    ) -> bool;
    pub fn m5u_speaker_is_playing(channel: c_int) -> bool;
    pub fn m5u_speaker_get_playing_channels() -> usize;
    pub fn m5u_speaker_stop(channel: c_int);
    pub fn m5u_speaker_get_channel_volume(channel: c_int) -> u8;
    pub fn m5u_speaker_set_channel_volume(channel: c_int, volume: u8);
    pub fn m5u_speaker_set_all_channel_volume(volume: u8);

    pub fn m5u_imu_is_enabled() -> bool;
    pub fn m5u_imu_get_type() -> c_int;
    pub fn m5u_imu_update() -> bool;
    pub fn m5u_imu_sleep() -> bool;
    pub fn m5u_imu_load_offset_from_nvs() -> bool;
    pub fn m5u_imu_save_offset_to_nvs() -> bool;
    pub fn m5u_imu_clear_offset_data();
    pub fn m5u_imu_get_offset_data(index: usize) -> i32;
    pub fn m5u_imu_set_offset_data(index: usize, value: i32);
    pub fn m5u_imu_get_raw_data(index: usize) -> i16;
    pub fn m5u_imu_set_int_pin_active_logic(level: bool) -> bool;
    pub fn m5u_imu_set_calibration(accel_strength: u8, gyro_strength: u8, mag_strength: u8);

    pub fn m5u_touch_get_detail(index: c_int, out: *mut m5u_touch_detail_t) -> bool;
    pub fn m5u_touch_get_raw(index: c_int, out: *mut m5u_touch_point_t) -> bool;
    pub fn m5u_touch_set_hold_thresh(msec: u16);
    pub fn m5u_touch_set_flick_thresh(distance: u16);
    pub fn m5u_set_touch_button_height(pixel: u16);
    pub fn m5u_set_touch_button_height_by_ratio(ratio: u8);
    pub fn m5u_get_touch_button_height() -> u16;
    pub fn m5u_rtc_is_enabled() -> bool;
    pub fn m5u_rtc_begin() -> bool;
    pub fn m5u_rtc_get_volt_low() -> bool;
    pub fn m5u_rtc_get_date(
        year: *mut c_int,
        month: *mut c_int,
        day: *mut c_int,
        weekday: *mut c_int,
    ) -> bool;
    pub fn m5u_rtc_get_time(hour: *mut c_int, minute: *mut c_int, second: *mut c_int) -> bool;
    pub fn m5u_rtc_set_date(year: c_int, month: c_int, day: c_int, weekday: c_int) -> bool;
    pub fn m5u_rtc_set_time(hour: c_int, minute: c_int, second: c_int) -> bool;
    pub fn m5u_rtc_set_system_time_from_rtc();
    pub fn m5u_rtc_set_alarm_irq_after(seconds: c_int) -> bool;
    pub fn m5u_rtc_get_irq_status() -> bool;
    pub fn m5u_rtc_clear_irq();
    pub fn m5u_rtc_disable_irq();

    pub fn m5u_power_axp2101_disable_irq(mask: u64) -> bool;
    pub fn m5u_power_axp2101_enable_irq(mask: u64) -> bool;
    pub fn m5u_power_axp2101_clear_irq_statuses() -> bool;
    pub fn m5u_power_axp2101_get_irq_statuses() -> u64;
    pub fn m5u_power_axp2101_is_bat_charger_under_temperature_irq() -> bool;
    pub fn m5u_power_axp2101_is_bat_charger_over_temperature_irq() -> bool;
    pub fn m5u_power_axp2101_is_vbus_insert_irq() -> bool;
    pub fn m5u_power_axp2101_is_vbus_remove_irq() -> bool;

    pub fn m5u_log_print(text: *const c_char);
    pub fn m5u_log_println(text: *const c_char);
    pub fn m5u_log_level(level: c_int, text: *const c_char);
    pub fn m5u_log_set_level(target: c_int, level: c_int);
    pub fn m5u_log_get_level(target: c_int) -> c_int;
    pub fn m5u_log_set_enable_color(target: c_int, enable: bool);
    pub fn m5u_log_get_enable_color(target: c_int) -> bool;
    pub fn m5u_log_set_suffix(target: c_int, suffix: *const c_char);
    pub fn m5u_set_log_display_index(index: c_int);
    pub fn m5u_sd_begin() -> bool;
    pub fn m5u_sd_end();
    pub fn m5u_sd_card_type() -> c_int;
    pub fn m5u_sd_card_size_bytes() -> u64;
    pub fn m5u_sd_total_bytes() -> u64;
    pub fn m5u_sd_used_bytes() -> u64;
    pub fn m5u_sd_exists(path: *const c_char) -> bool;
    pub fn m5u_sd_file_size(path: *const c_char) -> u64;
    pub fn m5u_sd_is_directory(path: *const c_char) -> bool;
    pub fn m5u_sd_read_file(path: *const c_char, data: *mut u8, len: usize) -> usize;
    pub fn m5u_sd_write_file(
        path: *const c_char,
        data: *const u8,
        len: usize,
        append: bool,
    ) -> usize;
    pub fn m5u_sd_remove(path: *const c_char) -> bool;
    pub fn m5u_sd_mkdir(path: *const c_char) -> bool;
    pub fn m5u_sd_rmdir(path: *const c_char) -> bool;
    pub fn m5u_sd_rename(from_path: *const c_char, to_path: *const c_char) -> bool;
    pub fn m5u_sd_list_dir(
        path: *const c_char,
        entries: *mut m5u_cardputer_sd_dir_entry_t,
        capacity: usize,
    ) -> usize;
    pub fn m5u_i2c_begin(sda: c_int, scl: c_int, frequency_hz: u32) -> bool;
    pub fn m5u_i2c_end();
    pub fn m5u_i2c_probe(address: u8) -> bool;
    pub fn m5u_i2c_write(address: u8, data: *const u8, len: usize) -> bool;
    pub fn m5u_i2c_read(address: u8, data: *mut u8, len: usize) -> usize;
    pub fn m5u_i2c_write_reg(address: u8, reg: u8, data: *const u8, len: usize) -> bool;
    pub fn m5u_i2c_read_reg(address: u8, reg: u8, data: *mut u8, len: usize) -> usize;
    pub fn m5u_uart_begin(rx: c_int, tx: c_int, baud: u32) -> bool;
    pub fn m5u_uart_end();
    pub fn m5u_uart_available() -> usize;
    pub fn m5u_uart_read(data: *mut u8, len: usize) -> usize;
    pub fn m5u_uart_write(data: *const u8, len: usize) -> usize;
    pub fn m5u_uart_flush();
    pub fn m5u_gpio_pin_mode(pin: c_int, mode: c_int) -> bool;
    pub fn m5u_gpio_write(pin: c_int, high: bool) -> bool;
    pub fn m5u_gpio_read(pin: c_int) -> c_int;
    pub fn m5u_gpio_analog_read(pin: c_int) -> c_int;
    pub fn m5u_gpio_analog_read_millivolts(pin: c_int) -> c_int;
    pub fn m5u_gpio_analog_write(pin: c_int, duty: u8) -> bool;
    pub fn m5u_gpio_analog_write_frequency(pin: c_int, frequency_hz: u32) -> bool;
    pub fn m5u_gpio_analog_write_resolution(pin: c_int, resolution_bits: u8) -> bool;
    pub fn m5u_servo_attach(
        pin: c_int,
        channel: c_int,
        timer: c_int,
        frequency_hz: u32,
        min_us: u16,
        max_us: u16,
    ) -> bool;
    pub fn m5u_servo_detach(channel: c_int) -> bool;
    pub fn m5u_servo_write_pulse_us(channel: c_int, pulse_us: u16) -> bool;
    pub fn m5u_stackchan_motion_begin() -> bool;
    pub fn m5u_stackchan_motion_update();
    pub fn m5u_stackchan_motion_move(yaw_tenths: i16, pitch_tenths: i16, speed_bsp: u16) -> bool;
    pub fn m5u_stackchan_motion_home(speed_bsp: u16) -> bool;
    pub fn m5u_stackchan_motion_nod() -> bool;
    pub fn m5u_stackchan_motion_shake() -> bool;
    pub fn m5u_stackchan_motion_status(out: *mut m5u_stackchan_motion_status_t) -> bool;
    pub fn m5u_spi_begin(sck: c_int, miso: c_int, mosi: c_int, cs: c_int) -> bool;
    pub fn m5u_spi_end();
    pub fn m5u_spi_transfer_byte(value: u8, frequency_hz: u32, mode: u8, lsb_first: bool) -> u8;
    pub fn m5u_spi_transfer(
        tx: *const u8,
        rx: *mut u8,
        len: usize,
        frequency_hz: u32,
        mode: u8,
        lsb_first: bool,
    ) -> bool;
    pub fn m5u_spi_write(
        data: *const u8,
        len: usize,
        frequency_hz: u32,
        mode: u8,
        lsb_first: bool,
    ) -> bool;

    pub fn m5u_cardputer_begin(enable_keyboard: bool) -> bool;
    pub fn m5u_cardputer_begin_with_config(
        config: *const m5u_config_t,
        enable_keyboard: bool,
    ) -> bool;
    pub fn m5u_cardputer_update();
    pub fn m5u_cardputer_keyboard_begin();
    pub fn m5u_cardputer_keyboard_is_pressed() -> bool;
    pub fn m5u_cardputer_keyboard_pressed_count() -> u8;
    pub fn m5u_cardputer_keyboard_is_change() -> bool;
    pub fn m5u_cardputer_keyboard_is_key_pressed(key: u8) -> bool;
    pub fn m5u_cardputer_keyboard_get_key(x: u8, y: u8) -> u8;
    pub fn m5u_cardputer_keyboard_get_key_value(
        x: u8,
        y: u8,
        out: *mut m5u_cardputer_key_value_t,
    ) -> bool;
    pub fn m5u_cardputer_keyboard_get_state(out: *mut m5u_cardputer_keyboard_state_t) -> bool;
    pub fn m5u_cardputer_keyboard_capslocked() -> bool;
    pub fn m5u_cardputer_keyboard_set_capslocked(locked: bool);
    pub fn m5u_cardputer_sd_begin(
        sck: c_int,
        miso: c_int,
        mosi: c_int,
        cs: c_int,
        frequency_hz: u32,
    ) -> bool;
    pub fn m5u_cardputer_sd_end();
    pub fn m5u_cardputer_sd_card_type() -> c_int;
    pub fn m5u_cardputer_sd_card_size_bytes() -> u64;
    pub fn m5u_cardputer_sd_total_bytes() -> u64;
    pub fn m5u_cardputer_sd_used_bytes() -> u64;
    pub fn m5u_cardputer_sd_exists(path: *const c_char) -> bool;
    pub fn m5u_cardputer_sd_file_size(path: *const c_char) -> u64;
    pub fn m5u_cardputer_sd_is_directory(path: *const c_char) -> bool;
    pub fn m5u_cardputer_sd_list_dir(
        path: *const c_char,
        entries: *mut m5u_cardputer_sd_dir_entry_t,
        capacity: usize,
    ) -> usize;
    pub fn m5u_cardputer_sd_read_file(path: *const c_char, data: *mut u8, len: usize) -> usize;
    pub fn m5u_cardputer_sd_write_file(
        path: *const c_char,
        data: *const u8,
        len: usize,
        append: bool,
    ) -> usize;
    pub fn m5u_cardputer_sd_remove(path: *const c_char) -> bool;
    pub fn m5u_cardputer_sd_mkdir(path: *const c_char) -> bool;
    pub fn m5u_cardputer_sd_rmdir(path: *const c_char) -> bool;
    pub fn m5u_cardputer_sd_rename(from_path: *const c_char, to_path: *const c_char) -> bool;
    pub fn m5u_cardputer_ir_begin(pin: c_int) -> bool;
    pub fn m5u_cardputer_ir_send_nec(address: u16, command: u8, repeats: u8) -> bool;
    pub fn m5u_cardputer_grove_i2c_begin(sda: c_int, scl: c_int, frequency_hz: u32) -> bool;
    pub fn m5u_cardputer_grove_i2c_end();
    pub fn m5u_cardputer_grove_i2c_probe(address: u8) -> bool;
    pub fn m5u_cardputer_grove_i2c_write(address: u8, data: *const u8, len: usize) -> bool;
    pub fn m5u_cardputer_grove_i2c_read(address: u8, data: *mut u8, len: usize) -> usize;
    pub fn m5u_cardputer_grove_i2c_write_reg(
        address: u8,
        reg: u8,
        data: *const u8,
        len: usize,
    ) -> bool;
    pub fn m5u_cardputer_grove_i2c_read_reg(
        address: u8,
        reg: u8,
        data: *mut u8,
        len: usize,
    ) -> usize;
    pub fn m5u_cardputer_grove_gpio_pin_mode(pin: c_int, mode: c_int) -> bool;
    pub fn m5u_cardputer_grove_gpio_write(pin: c_int, high: bool) -> bool;
    pub fn m5u_cardputer_grove_gpio_read(pin: c_int) -> c_int;
    pub fn m5u_cardputer_grove_uart_begin(rx: c_int, tx: c_int, baud: u32) -> bool;
    pub fn m5u_cardputer_grove_uart_end();
    pub fn m5u_cardputer_grove_uart_available() -> usize;
    pub fn m5u_cardputer_grove_uart_read(data: *mut u8, len: usize) -> usize;
    pub fn m5u_cardputer_grove_uart_write(data: *const u8, len: usize) -> usize;
    pub fn m5u_cardputer_grove_uart_flush();
}

#[cfg(not(target_os = "espidf"))]
mod host_stubs {
    use super::*;
    use core::ptr;

    unsafe fn cstr_len(text: *const c_char) -> c_int {
        if text.is_null() {
            return 0;
        }
        let mut len = 0;
        while *text.add(len) != 0 {
            len += 1;
        }
        len as c_int
    }

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
    pub unsafe fn m5u_display_draw_pixel(_x: c_int, _y: c_int, _color: u16) {}
    pub unsafe fn m5u_display_read_pixel(_x: c_int, _y: c_int) -> u16 {
        0
    }
    pub unsafe fn m5u_display_draw_fast_hline(_x: c_int, _y: c_int, _w: c_int, _color: u16) {}
    pub unsafe fn m5u_display_draw_fast_vline(_x: c_int, _y: c_int, _h: c_int, _color: u16) {}
    pub unsafe fn m5u_display_draw_rect(_x: c_int, _y: c_int, _w: c_int, _h: c_int, _color: u16) {}
    pub unsafe fn m5u_display_fill_rect(_x: c_int, _y: c_int, _w: c_int, _h: c_int, _color: u16) {}
    pub unsafe fn m5u_display_fill_rect_alpha(
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _alpha: u8,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_round_rect(
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_fill_round_rect(
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_circle(_x: c_int, _y: c_int, _r: c_int, _color: u16) {}
    pub unsafe fn m5u_display_fill_circle(_x: c_int, _y: c_int, _r: c_int, _color: u16) {}
    pub unsafe fn m5u_display_draw_ellipse(
        _x: c_int,
        _y: c_int,
        _rx: c_int,
        _ry: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_fill_ellipse(
        _x: c_int,
        _y: c_int,
        _rx: c_int,
        _ry: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_arc(
        _x: c_int,
        _y: c_int,
        _r0: c_int,
        _r1: c_int,
        _angle0: c_float,
        _angle1: c_float,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_fill_arc(
        _x: c_int,
        _y: c_int,
        _r0: c_int,
        _r1: c_int,
        _angle0: c_float,
        _angle1: c_float,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_triangle(
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _x2: c_int,
        _y2: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_fill_triangle(
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _x2: c_int,
        _y2: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_progress_bar(_x: c_int, _y: c_int, _w: c_int, _h: c_int, _value: u8) {
    }
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
        false
    }
    pub unsafe fn m5u_imu_get_temp_c(temp: *mut f32) -> bool {
        if !temp.is_null() {
            *temp = 25.0;
        }
        true
    }

    pub unsafe fn m5u_touch_begin() {}
    pub unsafe fn m5u_touch_update(_msec: u32) {}
    pub unsafe fn m5u_touch_is_enabled() -> bool {
        false
    }
    pub unsafe fn m5u_touch_end() {}
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
    pub unsafe fn m5u_rtc_begin() -> bool {
        true
    }
    pub unsafe fn m5u_rtc_get_volt_low() -> bool {
        false
    }
    pub unsafe fn m5u_rtc_get_date(
        year: *mut c_int,
        month: *mut c_int,
        day: *mut c_int,
        weekday: *mut c_int,
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
        if !weekday.is_null() {
            *weekday = 4;
        }
        true
    }
    pub unsafe fn m5u_rtc_get_time(
        hour: *mut c_int,
        minute: *mut c_int,
        second: *mut c_int,
    ) -> bool {
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
    pub unsafe fn m5u_rtc_set_date(
        _year: c_int,
        _month: c_int,
        _day: c_int,
        _weekday: c_int,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_rtc_set_time(_hour: c_int, _minute: c_int, _second: c_int) -> bool {
        true
    }
    pub unsafe fn m5u_rtc_set_system_time_from_rtc() {}
    pub unsafe fn m5u_rtc_set_alarm_irq_after(_seconds: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_rtc_get_irq_status() -> bool {
        false
    }
    pub unsafe fn m5u_rtc_clear_irq() {}
    pub unsafe fn m5u_rtc_disable_irq() {}

    pub unsafe fn m5u_battery_level() -> c_int {
        100
    }
    pub unsafe fn m5u_battery_voltage_mv() -> c_int {
        4200
    }
    pub unsafe fn m5u_battery_current_ma() -> c_int {
        0
    }
    pub unsafe fn m5u_power_is_charging() -> bool {
        false
    }
    pub unsafe fn m5u_power_charging_state() -> c_int {
        0
    }
    pub unsafe fn m5u_power_begin() -> bool {
        false
    }
    pub unsafe fn m5u_power_set_ext_output(_enable: bool) {}
    pub unsafe fn m5u_power_get_ext_output() -> bool {
        false
    }
    pub unsafe fn m5u_power_set_usb_output(_enable: bool) {}
    pub unsafe fn m5u_power_get_usb_output() -> bool {
        false
    }
    pub unsafe fn m5u_power_set_led(_brightness: u8) {}
    pub unsafe fn m5u_power_power_off() {}
    pub unsafe fn m5u_power_timer_sleep(_seconds: c_int) {}
    pub unsafe fn m5u_power_deep_sleep(_micro_seconds: u64, _touch_wakeup: bool) {}
    pub unsafe fn m5u_power_light_sleep(_micro_seconds: u64, _touch_wakeup: bool) {}
    pub unsafe fn m5u_power_set_battery_charge(_enable: bool) {}
    pub unsafe fn m5u_power_set_charge_current(_max_ma: u16) {}
    pub unsafe fn m5u_power_set_charge_voltage(_max_mv: u16) {}
    pub unsafe fn m5u_power_get_key_state() -> u8 {
        0
    }
    pub unsafe fn m5u_power_set_vibration(_level: u8) {}
    pub unsafe fn m5u_power_get_type() -> c_int {
        0
    }
    pub unsafe fn m5u_led_begin() -> bool {
        false
    }
    pub unsafe fn m5u_led_is_enabled() -> bool {
        false
    }
    pub unsafe fn m5u_led_count() -> usize {
        0
    }
    pub unsafe fn m5u_led_display() {}
    pub unsafe fn m5u_led_set_auto_display(_enable: bool) {}
    pub unsafe fn m5u_led_set_brightness(_brightness: u8) {}
    pub unsafe fn m5u_led_set_color(_index: usize, _rgb: u32) {}
    pub unsafe fn m5u_led_set_all_color(_rgb: u32) {}

    pub unsafe fn m5u_display_get_rotation() -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_brightness(_brightness: u8) {}
    pub unsafe fn m5u_display_get_brightness() -> u8 {
        0
    }
    pub unsafe fn m5u_display_set_color_depth(_depth: u8) {}
    pub unsafe fn m5u_display_get_color_depth() -> u8 {
        16
    }
    pub unsafe fn m5u_display_is_epd() -> bool {
        false
    }
    pub unsafe fn m5u_display_set_epd_mode(_mode: c_int) {}
    pub unsafe fn m5u_display_get_epd_mode() -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_epd_fastest() {}
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_set_resolution(
        _logical_width: u16,
        _logical_height: u16,
        _refresh_rate: c_float,
        _output_width: u16,
        _output_height: u16,
        _scale_w: u8,
        _scale_h: u8,
        _pixel_clock: u32,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_display_start_write() {}
    pub unsafe fn m5u_display_end_write() {}
    pub unsafe fn m5u_display_display() {}
    pub unsafe fn m5u_display_display_busy() -> bool {
        false
    }
    pub unsafe fn m5u_display_wait_display() {}
    pub unsafe fn m5u_display_sleep() {}
    pub unsafe fn m5u_display_wakeup() {}
    pub unsafe fn m5u_display_power_save_on() {}
    pub unsafe fn m5u_display_power_save_off() {}
    pub unsafe fn m5u_display_power_save(_enable: bool) {}
    pub unsafe fn m5u_display_invert_display(_invert: bool) {}
    pub unsafe fn m5u_display_get_cursor_x() -> c_int {
        0
    }
    pub unsafe fn m5u_display_get_cursor_y() -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_pivot(_x: c_float, _y: c_float) {}
    pub unsafe fn m5u_display_get_pivot_x() -> c_float {
        0.0
    }
    pub unsafe fn m5u_display_get_pivot_y() -> c_float {
        0.0
    }
    pub unsafe fn m5u_display_font_height() -> c_int {
        16
    }
    pub unsafe fn m5u_display_font_width() -> c_int {
        8
    }
    pub unsafe fn m5u_display_set_font(font: c_int) -> bool {
        (0..=69).contains(&font)
    }
    pub unsafe fn m5u_display_show_font(_duration_ms: u32) -> bool {
        true
    }
    pub unsafe fn m5u_display_unload_font() {}
    pub unsafe fn m5u_display_font_height_for(_font: c_int) -> c_int {
        16
    }
    pub unsafe fn m5u_display_font_width_for(_font: c_int) -> c_int {
        8
    }
    pub unsafe fn m5u_display_get_base_color() -> u16 {
        0
    }
    pub unsafe fn m5u_display_set_base_color(_color: u16) {}
    pub unsafe fn m5u_display_set_color(_color: u16) {}
    pub unsafe fn m5u_display_set_rgb_color(_r: u8, _g: u8, _b: u8) {}
    pub unsafe fn m5u_display_set_raw_color(_color: u32) {}
    pub unsafe fn m5u_display_get_raw_color() -> u32 {
        0
    }
    pub unsafe fn m5u_display_get_palette_count() -> u32 {
        0
    }
    pub unsafe fn m5u_display_set_swap_bytes(_swap: bool) {}
    pub unsafe fn m5u_display_get_swap_bytes() -> bool {
        false
    }
    pub unsafe fn m5u_display_swap565(r: u8, g: u8, b: u8) -> u16 {
        let rgb565 =
            ((u16::from(r & 0xF8)) << 8) | ((u16::from(g & 0xFC)) << 3) | (u16::from(b) >> 3);
        rgb565.rotate_left(8)
    }
    pub unsafe fn m5u_display_swap888(r: u8, g: u8, b: u8) -> u32 {
        (u32::from(b) << 16) | (u32::from(g) << 8) | u32::from(r)
    }
    pub unsafe fn m5u_display_set_text_wrap(_wrap_x: bool, _wrap_y: bool) {}
    pub unsafe fn m5u_display_set_text_datum(_datum: c_int) {}
    pub unsafe fn m5u_display_get_text_datum() -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_text_padding(_padding_x: u32) {}
    pub unsafe fn m5u_display_get_text_padding() -> u32 {
        0
    }
    pub unsafe fn m5u_display_get_text_size_x() -> u8 {
        1
    }
    pub unsafe fn m5u_display_get_text_size_y() -> u8 {
        1
    }
    pub unsafe fn m5u_display_text_length(text: *const c_char) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_display_text_width(text: *const c_char) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_display_draw_center_string(
        text: *const c_char,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_display_draw_string(_text: *const c_char, _x: c_int, _y: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_display_draw_char(_codepoint: u32, _x: c_int, _y: c_int) -> c_int {
        8
    }
    pub unsafe fn m5u_display_draw_number(_value: i32, _x: c_int, _y: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_display_draw_float(
        _value: c_float,
        _decimals: u8,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_draw_bmp(
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_draw_jpg(
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_draw_png(
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_display_push_image_rgb565(
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _data: *const u16,
        _len: usize,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_display_write_pixel(_x: c_int, _y: c_int, _color: u16) {}
    pub unsafe fn m5u_display_write_fast_vline(_x: c_int, _y: c_int, _h: c_int, _color: u16) {}
    pub unsafe fn m5u_display_set_addr_window(_x: c_int, _y: c_int, _w: c_int, _h: c_int) {}
    pub unsafe fn m5u_display_set_window(_xs: c_int, _ys: c_int, _xe: c_int, _ye: c_int) {}
    pub unsafe fn m5u_display_set_clip_rect(_x: c_int, _y: c_int, _w: c_int, _h: c_int) {}
    pub unsafe fn m5u_display_get_clip_rect(
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    ) {
        if !x.is_null() {
            *x = 0;
        }
        if !y.is_null() {
            *y = 0;
        }
        if !w.is_null() {
            *w = 320;
        }
        if !h.is_null() {
            *h = 240;
        }
    }
    pub unsafe fn m5u_display_clear_clip_rect() {}
    pub unsafe fn m5u_display_scroll(_dx: c_int, _dy: c_int) {}
    pub unsafe fn m5u_display_set_text_scroll(_enable: bool) {}
    pub unsafe fn m5u_display_set_scroll_rect(
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_get_scroll_rect(
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    ) {
        if !x.is_null() {
            *x = 0;
        }
        if !y.is_null() {
            *y = 0;
        }
        if !w.is_null() {
            *w = 0;
        }
        if !h.is_null() {
            *h = 0;
        }
    }
    pub unsafe fn m5u_display_clear_scroll_rect() {}
    pub unsafe fn m5u_display_color888(r: u8, g: u8, b: u8) -> u16 {
        ((u16::from(r & 0xF8)) << 8) | ((u16::from(g & 0xFC)) << 3) | u16::from(b >> 3)
    }
    pub unsafe fn m5u_canvas_create_for_display() -> *mut c_void {
        ptr::without_provenance_mut(1)
    }
    pub unsafe fn m5u_canvas_create_for_cardputer_display() -> *mut c_void {
        ptr::without_provenance_mut(1)
    }
    pub unsafe fn m5u_canvas_delete(_canvas: *mut c_void) {}
    pub unsafe fn m5u_canvas_create_sprite(_canvas: *mut c_void, _w: c_int, _h: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_canvas_delete_sprite(_canvas: *mut c_void) {}
    pub unsafe fn m5u_canvas_push_sprite(_canvas: *mut c_void, _x: c_int, _y: c_int) {}
    pub unsafe fn m5u_canvas_width(_canvas: *mut c_void) -> c_int {
        0
    }
    pub unsafe fn m5u_canvas_height(_canvas: *mut c_void) -> c_int {
        0
    }
    pub unsafe fn m5u_canvas_fill_screen(_canvas: *mut c_void, _color: u16) {}
    pub unsafe fn m5u_canvas_set_cursor(_canvas: *mut c_void, _x: c_int, _y: c_int) {}
    pub unsafe fn m5u_canvas_set_text_size(_canvas: *mut c_void, _size: c_float) {}
    pub unsafe fn m5u_canvas_set_text_color(_canvas: *mut c_void, _fg: u16, _bg: u16) {}
    pub unsafe fn m5u_canvas_set_text_scroll(_canvas: *mut c_void, _enable: bool) {}
    pub unsafe fn m5u_canvas_set_text_datum(_canvas: *mut c_void, _datum: c_int) {}
    pub unsafe fn m5u_canvas_get_text_datum(_canvas: *mut c_void) -> c_int {
        0
    }
    pub unsafe fn m5u_canvas_set_text_padding(_canvas: *mut c_void, _padding_x: u32) {}
    pub unsafe fn m5u_canvas_get_text_padding(_canvas: *mut c_void) -> u32 {
        0
    }
    pub unsafe fn m5u_canvas_get_text_size_x(_canvas: *mut c_void) -> u8 {
        1
    }
    pub unsafe fn m5u_canvas_get_text_size_y(_canvas: *mut c_void) -> u8 {
        1
    }
    pub unsafe fn m5u_canvas_get_base_color(_canvas: *mut c_void) -> u16 {
        0
    }
    pub unsafe fn m5u_canvas_set_base_color(_canvas: *mut c_void, _color: u16) {}
    pub unsafe fn m5u_canvas_set_color(_canvas: *mut c_void, _color: u16) {}
    pub unsafe fn m5u_canvas_set_rgb_color(_canvas: *mut c_void, _r: u8, _g: u8, _b: u8) {}
    pub unsafe fn m5u_canvas_set_raw_color(_canvas: *mut c_void, _color: u32) {}
    pub unsafe fn m5u_canvas_get_raw_color(_canvas: *mut c_void) -> u32 {
        0
    }
    pub unsafe fn m5u_canvas_set_swap_bytes(_canvas: *mut c_void, _swap: bool) {}
    pub unsafe fn m5u_canvas_get_swap_bytes(_canvas: *mut c_void) -> bool {
        false
    }
    pub unsafe fn m5u_canvas_set_font(_canvas: *mut c_void, font: c_int) -> bool {
        (0..=69).contains(&font)
    }
    pub unsafe fn m5u_canvas_font_height(_canvas: *mut c_void) -> c_int {
        16
    }
    pub unsafe fn m5u_canvas_font_width(_canvas: *mut c_void) -> c_int {
        8
    }
    pub unsafe fn m5u_canvas_show_font(_canvas: *mut c_void, _duration_ms: u32) -> bool {
        true
    }
    pub unsafe fn m5u_canvas_unload_font(_canvas: *mut c_void) {}
    pub unsafe fn m5u_canvas_text_width(_canvas: *mut c_void, text: *const c_char) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_canvas_print(_canvas: *mut c_void, _text: *const c_char) {}
    pub unsafe fn m5u_canvas_println(_canvas: *mut c_void, _text: *const c_char) {}
    pub unsafe fn m5u_canvas_draw_center_string(
        _canvas: *mut c_void,
        text: *const c_char,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_canvas_draw_string(
        _canvas: *mut c_void,
        text: *const c_char,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        cstr_len(text)
    }
    pub unsafe fn m5u_canvas_draw_line(
        _canvas: *mut c_void,
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_draw_rect(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_fill_rect(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_draw_circle(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_fill_circle(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_draw_pixel(_canvas: *mut c_void, _x: c_int, _y: c_int, _color: u16) {}
    pub unsafe fn m5u_canvas_read_pixel(_canvas: *mut c_void, _x: c_int, _y: c_int) -> u16 {
        0
    }
    pub unsafe fn m5u_canvas_draw_fast_hline(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_draw_fast_vline(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_draw_round_rect(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_fill_round_rect(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_draw_ellipse(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _rx: c_int,
        _ry: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_fill_ellipse(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _rx: c_int,
        _ry: c_int,
        _color: u16,
    ) {
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_canvas_draw_arc(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _r0: c_int,
        _r1: c_int,
        _angle0: c_float,
        _angle1: c_float,
        _color: u16,
    ) {
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_canvas_fill_arc(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _r0: c_int,
        _r1: c_int,
        _angle0: c_float,
        _angle1: c_float,
        _color: u16,
    ) {
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_canvas_draw_triangle(
        _canvas: *mut c_void,
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _x2: c_int,
        _y2: c_int,
        _color: u16,
    ) {
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_canvas_fill_triangle(
        _canvas: *mut c_void,
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _x2: c_int,
        _y2: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_progress_bar(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _value: u8,
    ) {
    }
    pub unsafe fn m5u_canvas_text_length(_canvas: *mut c_void, text: *const c_char) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_canvas_draw_char(
        _canvas: *mut c_void,
        _codepoint: u32,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        8
    }
    pub unsafe fn m5u_canvas_draw_number(
        _canvas: *mut c_void,
        _value: i32,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }
    pub unsafe fn m5u_canvas_draw_float(
        _canvas: *mut c_void,
        _value: c_float,
        _decimals: u8,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_canvas_draw_bmp(
        _canvas: *mut c_void,
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_canvas_draw_jpg(
        _canvas: *mut c_void,
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_canvas_draw_png(
        _canvas: *mut c_void,
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_canvas_push_image_rgb565(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _data: *const u16,
        _len: usize,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_canvas_write_pixel(_canvas: *mut c_void, _x: c_int, _y: c_int, _color: u16) {}
    pub unsafe fn m5u_canvas_write_fast_vline(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_set_addr_window(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
    ) {
    }
    pub unsafe fn m5u_canvas_set_window(
        _canvas: *mut c_void,
        _xs: c_int,
        _ys: c_int,
        _xe: c_int,
        _ye: c_int,
    ) {
    }
    pub unsafe fn m5u_canvas_set_clip_rect(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
    ) {
    }
    pub unsafe fn m5u_canvas_get_clip_rect(
        _canvas: *mut c_void,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    ) {
        if !x.is_null() {
            *x = 0;
        }
        if !y.is_null() {
            *y = 0;
        }
        if !w.is_null() {
            *w = 0;
        }
        if !h.is_null() {
            *h = 0;
        }
    }
    pub unsafe fn m5u_canvas_clear_clip_rect(_canvas: *mut c_void) {}
    pub unsafe fn m5u_canvas_scroll(_canvas: *mut c_void, _dx: c_int, _dy: c_int) {}
    pub unsafe fn m5u_canvas_set_scroll_rect(
        _canvas: *mut c_void,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_canvas_get_scroll_rect(
        _canvas: *mut c_void,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    ) {
        if !x.is_null() {
            *x = 0;
        }
        if !y.is_null() {
            *y = 0;
        }
        if !w.is_null() {
            *w = 0;
        }
        if !h.is_null() {
            *h = 0;
        }
    }
    pub unsafe fn m5u_canvas_clear_scroll_rect(_canvas: *mut c_void) {}
    pub unsafe fn m5u_display_count() -> c_int {
        1
    }
    pub unsafe fn m5u_display_index_for_kind(_kind: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_display_set_primary(index: c_int) -> bool {
        index == 0
    }
    pub unsafe fn m5u_display_set_primary_kind(_kind: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_display_set_rotation_at(_index: c_int, _rotation: c_int) {}
    pub unsafe fn m5u_display_get_rotation_at(_index: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_brightness_at(_index: c_int, _brightness: u8) {}
    pub unsafe fn m5u_display_get_brightness_at(_index: c_int) -> u8 {
        0
    }
    pub unsafe fn m5u_display_set_color_depth_at(_index: c_int, _depth: u8) {}
    pub unsafe fn m5u_display_get_color_depth_at(_index: c_int) -> u8 {
        16
    }
    pub unsafe fn m5u_display_is_epd_at(_index: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_display_set_epd_mode_at(_index: c_int, _mode: c_int) {}
    pub unsafe fn m5u_display_get_epd_mode_at(_index: c_int) -> c_int {
        0
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_set_resolution_at(
        _index: c_int,
        _logical_width: u16,
        _logical_height: u16,
        _refresh_rate: c_float,
        _output_width: u16,
        _output_height: u16,
        _scale_w: u8,
        _scale_h: u8,
        _pixel_clock: u32,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_display_width_at(_index: c_int) -> c_int {
        320
    }
    pub unsafe fn m5u_display_height_at(_index: c_int) -> c_int {
        240
    }
    pub unsafe fn m5u_display_start_write_at(_index: c_int) {}
    pub unsafe fn m5u_display_end_write_at(_index: c_int) {}
    pub unsafe fn m5u_display_display_at(_index: c_int) {}
    pub unsafe fn m5u_display_display_busy_at(_index: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_display_wait_display_at(_index: c_int) {}
    pub unsafe fn m5u_display_sleep_at(_index: c_int) {}
    pub unsafe fn m5u_display_wakeup_at(_index: c_int) {}
    pub unsafe fn m5u_display_power_save_on_at(_index: c_int) {}
    pub unsafe fn m5u_display_power_save_off_at(_index: c_int) {}
    pub unsafe fn m5u_display_power_save_at(_index: c_int, _enable: bool) {}
    pub unsafe fn m5u_display_invert_display_at(_index: c_int, _invert: bool) {}
    pub unsafe fn m5u_display_get_cursor_x_at(_index: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_display_get_cursor_y_at(_index: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_pivot_at(_index: c_int, _x: c_float, _y: c_float) {}
    pub unsafe fn m5u_display_get_pivot_x_at(_index: c_int) -> c_float {
        0.0
    }
    pub unsafe fn m5u_display_get_pivot_y_at(_index: c_int) -> c_float {
        0.0
    }
    pub unsafe fn m5u_display_clear_at(_index: c_int, _color: u16) {}
    pub unsafe fn m5u_display_set_cursor_at(_index: c_int, _x: c_int, _y: c_int) {}
    pub unsafe fn m5u_display_set_text_size_at(_index: c_int, _size: c_int) {}
    pub unsafe fn m5u_display_set_text_color_at(_index: c_int, _fg: u16, _bg: u16) {}
    pub unsafe fn m5u_display_set_text_datum_at(_index: c_int, _datum: c_int) {}
    pub unsafe fn m5u_display_get_text_datum_at(_index: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_display_set_text_padding_at(_index: c_int, _padding_x: u32) {}
    pub unsafe fn m5u_display_get_text_padding_at(_index: c_int) -> u32 {
        0
    }
    pub unsafe fn m5u_display_get_text_size_x_at(_index: c_int) -> u8 {
        1
    }
    pub unsafe fn m5u_display_get_text_size_y_at(_index: c_int) -> u8 {
        1
    }
    pub unsafe fn m5u_display_font_height_at(_index: c_int) -> c_int {
        16
    }
    pub unsafe fn m5u_display_font_width_at(_index: c_int) -> c_int {
        8
    }
    pub unsafe fn m5u_display_set_font_at(_index: c_int, font: c_int) -> bool {
        (0..=69).contains(&font)
    }
    pub unsafe fn m5u_display_show_font_at(_index: c_int, _duration_ms: u32) -> bool {
        true
    }
    pub unsafe fn m5u_display_unload_font_at(_index: c_int) {}
    pub unsafe fn m5u_display_get_base_color_at(_index: c_int) -> u16 {
        0
    }
    pub unsafe fn m5u_display_set_base_color_at(_index: c_int, _color: u16) {}
    pub unsafe fn m5u_display_set_color_at(_index: c_int, _color: u16) {}
    pub unsafe fn m5u_display_set_rgb_color_at(_index: c_int, _r: u8, _g: u8, _b: u8) {}
    pub unsafe fn m5u_display_set_raw_color_at(_index: c_int, _color: u32) {}
    pub unsafe fn m5u_display_get_raw_color_at(_index: c_int) -> u32 {
        0
    }
    pub unsafe fn m5u_display_get_palette_count_at(_index: c_int) -> u32 {
        0
    }
    pub unsafe fn m5u_display_set_swap_bytes_at(_index: c_int, _swap: bool) {}
    pub unsafe fn m5u_display_get_swap_bytes_at(_index: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_display_swap565_at(_index: c_int, r: u8, g: u8, b: u8) -> u16 {
        let rgb565 =
            ((u16::from(r & 0xF8)) << 8) | ((u16::from(g & 0xFC)) << 3) | u16::from(b >> 3);
        rgb565.rotate_left(8)
    }
    pub unsafe fn m5u_display_swap888_at(_index: c_int, r: u8, g: u8, b: u8) -> u32 {
        (u32::from(b) << 16) | (u32::from(g) << 8) | u32::from(r)
    }
    pub unsafe fn m5u_display_set_text_wrap_at(_index: c_int, _wrap_x: bool, _wrap_y: bool) {}
    pub unsafe fn m5u_display_color888_at(_index: c_int, r: u8, g: u8, b: u8) -> u16 {
        ((u16::from(r & 0xF8)) << 8) | ((u16::from(g & 0xFC)) << 3) | u16::from(b >> 3)
    }
    pub unsafe fn m5u_display_text_length_at(_index: c_int, text: *const c_char) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_display_text_width_at(_index: c_int, text: *const c_char) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_display_print_at(_index: c_int, _text: *const c_char) {}
    pub unsafe fn m5u_display_println_at(_index: c_int, _text: *const c_char) {}
    pub unsafe fn m5u_display_draw_center_string_at(
        _index: c_int,
        text: *const c_char,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        cstr_len(text) * 8
    }
    pub unsafe fn m5u_display_draw_string_at(
        _index: c_int,
        _text: *const c_char,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }
    pub unsafe fn m5u_display_draw_char_at(
        _index: c_int,
        _codepoint: u32,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        8
    }
    pub unsafe fn m5u_display_draw_number_at(
        _index: c_int,
        _value: i32,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }
    pub unsafe fn m5u_display_draw_float_at(
        _index: c_int,
        _value: c_float,
        _decimals: u8,
        _x: c_int,
        _y: c_int,
    ) -> c_int {
        0
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_draw_bmp_at(
        _index: c_int,
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_draw_jpg_at(
        _index: c_int,
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_draw_png_at(
        _index: c_int,
        _data: *const u8,
        _len: usize,
        _x: c_int,
        _y: c_int,
        _max_width: c_int,
        _max_height: c_int,
        _off_x: c_int,
        _off_y: c_int,
        _scale_x: c_float,
        _scale_y: c_float,
        _datum: c_int,
    ) -> bool {
        false
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
    pub unsafe fn m5u_display_draw_pixel_at(_index: c_int, _x: c_int, _y: c_int, _color: u16) {}
    pub unsafe fn m5u_display_read_pixel_at(_index: c_int, _x: c_int, _y: c_int) -> u16 {
        0
    }
    pub unsafe fn m5u_display_draw_fast_hline_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_fast_vline_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _h: c_int,
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
    pub unsafe fn m5u_display_fill_rect_alpha_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _alpha: u8,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_draw_round_rect_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _r: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_fill_round_rect_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _r: c_int,
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
    pub unsafe fn m5u_display_draw_ellipse_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _rx: c_int,
        _ry: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_fill_ellipse_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _rx: c_int,
        _ry: c_int,
        _color: u16,
    ) {
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_draw_arc_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _r0: c_int,
        _r1: c_int,
        _angle0: c_float,
        _angle1: c_float,
        _color: u16,
    ) {
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_fill_arc_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _r0: c_int,
        _r1: c_int,
        _angle0: c_float,
        _angle1: c_float,
        _color: u16,
    ) {
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_draw_triangle_at(
        _index: c_int,
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _x2: c_int,
        _y2: c_int,
        _color: u16,
    ) {
    }
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn m5u_display_fill_triangle_at(
        _index: c_int,
        _x0: c_int,
        _y0: c_int,
        _x1: c_int,
        _y1: c_int,
        _x2: c_int,
        _y2: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_progress_bar_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _value: u8,
    ) {
    }
    pub unsafe fn m5u_display_push_image_rgb565_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _data: *const u16,
        _len: usize,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_display_write_pixel_at(_index: c_int, _x: c_int, _y: c_int, _color: u16) {}
    pub unsafe fn m5u_display_write_fast_vline_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_set_addr_window_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
    ) {
    }
    pub unsafe fn m5u_display_set_window_at(
        _index: c_int,
        _xs: c_int,
        _ys: c_int,
        _xe: c_int,
        _ye: c_int,
    ) {
    }
    pub unsafe fn m5u_display_set_clip_rect_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
    ) {
    }
    pub unsafe fn m5u_display_get_clip_rect_at(
        _index: c_int,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    ) {
        if !x.is_null() {
            *x = 0;
        }
        if !y.is_null() {
            *y = 0;
        }
        if !w.is_null() {
            *w = 320;
        }
        if !h.is_null() {
            *h = 240;
        }
    }
    pub unsafe fn m5u_display_clear_clip_rect_at(_index: c_int) {}
    pub unsafe fn m5u_display_scroll_at(_index: c_int, _dx: c_int, _dy: c_int) {}
    pub unsafe fn m5u_display_set_text_scroll_at(_index: c_int, _enable: bool) {}
    pub unsafe fn m5u_display_set_scroll_rect_at(
        _index: c_int,
        _x: c_int,
        _y: c_int,
        _w: c_int,
        _h: c_int,
        _color: u16,
    ) {
    }
    pub unsafe fn m5u_display_get_scroll_rect_at(
        _index: c_int,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
    ) {
        if !x.is_null() {
            *x = 0;
        }
        if !y.is_null() {
            *y = 0;
        }
        if !w.is_null() {
            *w = 0;
        }
        if !h.is_null() {
            *h = 0;
        }
    }
    pub unsafe fn m5u_display_clear_scroll_rect_at(_index: c_int) {}

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
    pub unsafe fn m5u_button_was_single_clicked(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_double_clicked(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_was_change_pressed(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_is_holding(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_is_released(_button: c_int) -> bool {
        false
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
        false
    }
    pub unsafe fn m5u_button_was_decide_click_count(_button: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_button_get_click_count(_button: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_button_set_debounce_thresh(_button: c_int, _msec: u32) {}
    pub unsafe fn m5u_button_set_hold_thresh(_button: c_int, _msec: u32) {}
    pub unsafe fn m5u_button_set_raw_state(_button: c_int, _msec: u32, _press: bool) {}
    pub unsafe fn m5u_button_set_state(_button: c_int, _msec: u32, _state: c_int) {}
    pub unsafe fn m5u_button_get_state(_button: c_int) -> c_int {
        0
    }
    pub unsafe fn m5u_button_last_change(_button: c_int) -> u32 {
        0
    }
    pub unsafe fn m5u_button_get_debounce_thresh(_button: c_int) -> u32 {
        0
    }
    pub unsafe fn m5u_button_get_hold_thresh(_button: c_int) -> u32 {
        0
    }
    pub unsafe fn m5u_button_get_update_msec(_button: c_int) -> u32 {
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
    pub unsafe fn m5u_mic_get_config(out: *mut m5u_mic_config_t) -> bool {
        if !out.is_null() {
            *out = m5u_mic_config_t::default();
        }
        true
    }
    pub unsafe fn m5u_mic_set_config(_config: *const m5u_mic_config_t) {}

    pub unsafe fn m5u_speaker_is_enabled() -> bool {
        true
    }
    pub unsafe fn m5u_speaker_is_running() -> bool {
        false
    }
    pub unsafe fn m5u_speaker_end() {}
    pub unsafe fn m5u_speaker_get_config(out: *mut m5u_speaker_config_t) -> bool {
        if !out.is_null() {
            *out = m5u_speaker_config_t::default();
        }
        true
    }
    pub unsafe fn m5u_speaker_set_config(_config: *const m5u_speaker_config_t) {}
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
    pub unsafe fn m5u_speaker_get_playing_channels() -> usize {
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
    pub unsafe fn m5u_imu_sleep() -> bool {
        false
    }
    pub unsafe fn m5u_imu_load_offset_from_nvs() -> bool {
        false
    }
    pub unsafe fn m5u_imu_save_offset_to_nvs() -> bool {
        false
    }
    pub unsafe fn m5u_imu_clear_offset_data() {}
    pub unsafe fn m5u_imu_get_offset_data(_index: usize) -> i32 {
        0
    }
    pub unsafe fn m5u_imu_set_offset_data(_index: usize, _value: i32) {}
    pub unsafe fn m5u_imu_get_raw_data(_index: usize) -> i16 {
        0
    }
    pub unsafe fn m5u_imu_set_int_pin_active_logic(_level: bool) -> bool {
        false
    }
    pub unsafe fn m5u_imu_set_calibration(
        _accel_strength: u8,
        _gyro_strength: u8,
        _mag_strength: u8,
    ) {
    }

    pub unsafe fn m5u_touch_get_detail(_index: c_int, out: *mut m5u_touch_detail_t) -> bool {
        if !out.is_null() {
            *out = m5u_touch_detail_t::default();
        }
        false
    }
    pub unsafe fn m5u_touch_get_raw(_index: c_int, out: *mut m5u_touch_point_t) -> bool {
        if !out.is_null() {
            *out = m5u_touch_point_t::default();
        }
        false
    }
    pub unsafe fn m5u_touch_set_hold_thresh(_msec: u16) {}
    pub unsafe fn m5u_touch_set_flick_thresh(_distance: u16) {}
    pub unsafe fn m5u_set_touch_button_height(_pixel: u16) {}
    pub unsafe fn m5u_set_touch_button_height_by_ratio(_ratio: u8) {}
    pub unsafe fn m5u_get_touch_button_height() -> u16 {
        0
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

    pub unsafe fn m5u_log_print(_text: *const c_char) {}
    pub unsafe fn m5u_log_println(_text: *const c_char) {}
    pub unsafe fn m5u_log_level(_level: c_int, _text: *const c_char) {}
    pub unsafe fn m5u_log_set_level(_target: c_int, _level: c_int) {}
    pub unsafe fn m5u_log_get_level(_target: c_int) -> c_int {
        3
    }
    pub unsafe fn m5u_log_set_enable_color(_target: c_int, _enable: bool) {}
    pub unsafe fn m5u_log_get_enable_color(_target: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_log_set_suffix(_target: c_int, _suffix: *const c_char) {}
    pub unsafe fn m5u_set_log_display_index(_index: c_int) {}
    pub unsafe fn m5u_sd_begin() -> bool {
        false
    }
    pub unsafe fn m5u_sd_end() {}
    pub unsafe fn m5u_sd_card_type() -> c_int {
        0
    }
    pub unsafe fn m5u_sd_card_size_bytes() -> u64 {
        0
    }
    pub unsafe fn m5u_sd_total_bytes() -> u64 {
        0
    }
    pub unsafe fn m5u_sd_used_bytes() -> u64 {
        0
    }
    pub unsafe fn m5u_sd_exists(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_sd_file_size(_path: *const c_char) -> u64 {
        0
    }
    pub unsafe fn m5u_sd_is_directory(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_sd_read_file(_path: *const c_char, _data: *mut u8, _len: usize) -> usize {
        0
    }
    pub unsafe fn m5u_sd_write_file(
        _path: *const c_char,
        _data: *const u8,
        _len: usize,
        _append: bool,
    ) -> usize {
        0
    }
    pub unsafe fn m5u_sd_remove(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_sd_mkdir(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_sd_rmdir(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_sd_rename(_from_path: *const c_char, _to_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_sd_list_dir(
        _path: *const c_char,
        _entries: *mut m5u_cardputer_sd_dir_entry_t,
        _capacity: usize,
    ) -> usize {
        0
    }
    pub unsafe fn m5u_i2c_begin(_sda: c_int, _scl: c_int, _frequency_hz: u32) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_end() {}
    pub unsafe fn m5u_i2c_probe(_address: u8) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_write(_address: u8, _data: *const u8, _len: usize) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_read(_address: u8, _data: *mut u8, _len: usize) -> usize {
        0
    }
    pub unsafe fn m5u_i2c_write_reg(_address: u8, _reg: u8, _data: *const u8, _len: usize) -> bool {
        false
    }
    pub unsafe fn m5u_i2c_read_reg(_address: u8, _reg: u8, _data: *mut u8, _len: usize) -> usize {
        0
    }
    pub unsafe fn m5u_uart_begin(_rx: c_int, _tx: c_int, _baud: u32) -> bool {
        false
    }
    pub unsafe fn m5u_uart_end() {}
    pub unsafe fn m5u_uart_available() -> usize {
        0
    }
    pub unsafe fn m5u_uart_read(_data: *mut u8, _len: usize) -> usize {
        0
    }
    pub unsafe fn m5u_uart_write(_data: *const u8, _len: usize) -> usize {
        0
    }
    pub unsafe fn m5u_uart_flush() {}
    pub unsafe fn m5u_gpio_pin_mode(_pin: c_int, _mode: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_gpio_write(_pin: c_int, _high: bool) -> bool {
        false
    }
    pub unsafe fn m5u_gpio_read(_pin: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_gpio_analog_read(_pin: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_gpio_analog_read_millivolts(_pin: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_gpio_analog_write(_pin: c_int, _duty: u8) -> bool {
        false
    }
    pub unsafe fn m5u_gpio_analog_write_frequency(_pin: c_int, _frequency_hz: u32) -> bool {
        false
    }
    pub unsafe fn m5u_gpio_analog_write_resolution(_pin: c_int, _resolution_bits: u8) -> bool {
        false
    }
    pub unsafe fn m5u_servo_attach(
        _pin: c_int,
        _channel: c_int,
        _timer: c_int,
        _frequency_hz: u32,
        _min_us: u16,
        _max_us: u16,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_servo_detach(_channel: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_servo_write_pulse_us(_channel: c_int, _pulse_us: u16) -> bool {
        false
    }
    pub unsafe fn m5u_stackchan_motion_begin() -> bool {
        false
    }
    pub unsafe fn m5u_stackchan_motion_update() {}
    pub unsafe fn m5u_stackchan_motion_move(
        _yaw_tenths: i16,
        _pitch_tenths: i16,
        _speed_bsp: u16,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_stackchan_motion_home(_speed_bsp: u16) -> bool {
        false
    }
    pub unsafe fn m5u_stackchan_motion_nod() -> bool {
        false
    }
    pub unsafe fn m5u_stackchan_motion_shake() -> bool {
        false
    }
    pub unsafe fn m5u_stackchan_motion_status(out: *mut m5u_stackchan_motion_status_t) -> bool {
        if let Some(out) = out.as_mut() {
            *out = m5u_stackchan_motion_status_t::default();
        }
        false
    }
    pub unsafe fn m5u_spi_begin(_sck: c_int, _miso: c_int, _mosi: c_int, _cs: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_spi_end() {}
    pub unsafe fn m5u_spi_transfer_byte(
        _value: u8,
        _frequency_hz: u32,
        _mode: u8,
        _lsb_first: bool,
    ) -> u8 {
        0
    }
    pub unsafe fn m5u_spi_transfer(
        _tx: *const u8,
        _rx: *mut u8,
        _len: usize,
        _frequency_hz: u32,
        _mode: u8,
        _lsb_first: bool,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_spi_write(
        _data: *const u8,
        _len: usize,
        _frequency_hz: u32,
        _mode: u8,
        _lsb_first: bool,
    ) -> bool {
        false
    }

    pub unsafe fn m5u_cardputer_begin(_enable_keyboard: bool) -> bool {
        true
    }
    pub unsafe fn m5u_cardputer_begin_with_config(
        _config: *const m5u_config_t,
        _enable_keyboard: bool,
    ) -> bool {
        true
    }
    pub unsafe fn m5u_cardputer_update() {}
    pub unsafe fn m5u_cardputer_keyboard_begin() {}
    pub unsafe fn m5u_cardputer_keyboard_is_pressed() -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_keyboard_pressed_count() -> u8 {
        0
    }
    pub unsafe fn m5u_cardputer_keyboard_is_change() -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_keyboard_is_key_pressed(_key: u8) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_keyboard_get_key(_x: u8, _y: u8) -> u8 {
        0
    }
    pub unsafe fn m5u_cardputer_keyboard_get_key_value(
        _x: u8,
        _y: u8,
        out: *mut m5u_cardputer_key_value_t,
    ) -> bool {
        if !out.is_null() {
            *out = m5u_cardputer_key_value_t::default();
        }
        false
    }
    pub unsafe fn m5u_cardputer_keyboard_get_state(
        out: *mut m5u_cardputer_keyboard_state_t,
    ) -> bool {
        if !out.is_null() {
            *out = m5u_cardputer_keyboard_state_t::default();
        }
        true
    }
    pub unsafe fn m5u_cardputer_keyboard_capslocked() -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_keyboard_set_capslocked(_locked: bool) {}
    pub unsafe fn m5u_cardputer_sd_begin(
        _sck: c_int,
        _miso: c_int,
        _mosi: c_int,
        _cs: c_int,
        _frequency_hz: u32,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_sd_end() {}
    pub unsafe fn m5u_cardputer_sd_card_type() -> c_int {
        0
    }
    pub unsafe fn m5u_cardputer_sd_card_size_bytes() -> u64 {
        0
    }
    pub unsafe fn m5u_cardputer_sd_total_bytes() -> u64 {
        0
    }
    pub unsafe fn m5u_cardputer_sd_used_bytes() -> u64 {
        0
    }
    pub unsafe fn m5u_cardputer_sd_exists(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_sd_file_size(_path: *const c_char) -> u64 {
        0
    }
    pub unsafe fn m5u_cardputer_sd_is_directory(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_sd_list_dir(
        _path: *const c_char,
        _entries: *mut m5u_cardputer_sd_dir_entry_t,
        _capacity: usize,
    ) -> usize {
        0
    }
    pub unsafe fn m5u_cardputer_sd_read_file(
        _path: *const c_char,
        _data: *mut u8,
        _len: usize,
    ) -> usize {
        0
    }
    pub unsafe fn m5u_cardputer_sd_write_file(
        _path: *const c_char,
        _data: *const u8,
        _len: usize,
        _append: bool,
    ) -> usize {
        0
    }
    pub unsafe fn m5u_cardputer_sd_remove(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_sd_mkdir(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_sd_rmdir(_path: *const c_char) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_sd_rename(
        _from_path: *const c_char,
        _to_path: *const c_char,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_ir_begin(_pin: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_ir_send_nec(_address: u16, _command: u8, _repeats: u8) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_grove_i2c_begin(
        _sda: c_int,
        _scl: c_int,
        _frequency_hz: u32,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_grove_i2c_end() {}
    pub unsafe fn m5u_cardputer_grove_i2c_probe(_address: u8) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_grove_i2c_write(
        _address: u8,
        _data: *const u8,
        _len: usize,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_grove_i2c_read(_address: u8, _data: *mut u8, _len: usize) -> usize {
        0
    }
    pub unsafe fn m5u_cardputer_grove_i2c_write_reg(
        _address: u8,
        _reg: u8,
        _data: *const u8,
        _len: usize,
    ) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_grove_i2c_read_reg(
        _address: u8,
        _reg: u8,
        _data: *mut u8,
        _len: usize,
    ) -> usize {
        0
    }
    pub unsafe fn m5u_cardputer_grove_gpio_pin_mode(_pin: c_int, _mode: c_int) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_grove_gpio_write(_pin: c_int, _high: bool) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_grove_gpio_read(_pin: c_int) -> c_int {
        -1
    }
    pub unsafe fn m5u_cardputer_grove_uart_begin(_rx: c_int, _tx: c_int, _baud: u32) -> bool {
        false
    }
    pub unsafe fn m5u_cardputer_grove_uart_end() {}
    pub unsafe fn m5u_cardputer_grove_uart_available() -> usize {
        0
    }
    pub unsafe fn m5u_cardputer_grove_uart_read(_data: *mut u8, _len: usize) -> usize {
        0
    }
    pub unsafe fn m5u_cardputer_grove_uart_write(_data: *const u8, _len: usize) -> usize {
        0
    }
    pub unsafe fn m5u_cardputer_grove_uart_flush() {}
}

#[cfg(not(target_os = "espidf"))]
pub use host_stubs::*;
