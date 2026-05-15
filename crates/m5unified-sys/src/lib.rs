#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//! Raw bindings for a small C ABI shim over M5Unified.
//!
//! This crate intentionally does not bind M5Unified's C++ classes directly.
//! Instead, `native/m5u_shim.cpp` will expose stable `extern "C"` functions
//! that Rust can call safely from the higher-level `m5unified` crate.

use core::ffi::{c_char, c_int};

extern "C" {
    pub fn m5u_begin() -> bool;
    pub fn m5u_update();

    pub fn m5u_display_fill_screen(color: u16);
    pub fn m5u_display_set_cursor(x: c_int, y: c_int);
    pub fn m5u_display_print(text: *const c_char);

    pub fn m5u_btn_a_is_pressed() -> bool;
    pub fn m5u_btn_b_was_pressed() -> bool;

    pub fn m5u_mic_begin() -> bool;
    pub fn m5u_mic_record_i16(buffer: *mut i16, samples: usize) -> bool;

    pub fn m5u_battery_level() -> c_int;
}
