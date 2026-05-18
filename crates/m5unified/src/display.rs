//! Display drawing and multi-display helpers.
//!
//! The primary [`Display`] wrapper covers text, primitives, colors, EPD modes,
//! and transaction helpers. [`DisplayRef`] exposes indexed display access
//! without leaking M5Unified's C++ display objects.

use core::ffi::c_int;
use std::ffi::CString;

use crate::{raw_display_kinds, Error, M5Unified};

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

    pub fn rotation(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_rotation() as i32 }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_display_set_brightness(brightness) }
    }

    pub fn set_epd_fastest(&mut self) {
        self.set_epd_mode(EpdMode::Fastest);
    }

    pub fn set_epd_mode(&mut self, mode: EpdMode) {
        unsafe { m5unified_sys::m5u_display_set_epd_mode(mode.raw() as c_int) }
    }

    pub fn set_text_scroll(&mut self, scroll: bool) {
        unsafe { m5unified_sys::m5u_display_set_text_scroll(scroll) }
    }

    pub fn set_font(&mut self, font: DisplayFont) -> bool {
        unsafe { m5unified_sys::m5u_display_set_font(font.raw() as c_int) }
    }

    pub fn start_write(&mut self) {
        unsafe { m5unified_sys::m5u_display_start_write() }
    }

    pub fn end_write(&mut self) {
        unsafe { m5unified_sys::m5u_display_end_write() }
    }

    pub fn transaction<R>(&mut self, f: impl FnOnce(&mut Display) -> R) -> R {
        self.start_write();
        let result = f(self);
        self.end_write();
        result
    }

    pub fn display(&mut self) {
        unsafe { m5unified_sys::m5u_display_display() }
    }

    pub fn display_busy(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_display_busy() }
    }

    pub fn wait_display(&self) {
        unsafe { m5unified_sys::m5u_display_wait_display() }
    }

    pub fn cursor_y(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_y() as i32 }
    }

    pub fn font_height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_height() as i32 }
    }

    pub fn base_color(&self) -> u16 {
        unsafe { m5unified_sys::m5u_display_get_base_color() }
    }

    pub fn set_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_set_color(color) }
    }

    pub fn set_text_wrap(&mut self, wrap_x: bool, wrap_y: bool) {
        unsafe { m5unified_sys::m5u_display_set_text_wrap(wrap_x, wrap_y) }
    }

    pub fn set_text_datum(&mut self, datum: TextDatum) {
        unsafe { m5unified_sys::m5u_display_set_text_datum(datum as c_int) }
    }

    pub fn draw_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_draw_string(text.as_ptr(), x, y) as i32 })
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_pixel(x, y, color) }
    }

    pub fn write_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_fast_vline(x, y, h, color) }
    }

    pub fn set_clip_rect(&mut self, rect: Rect) {
        unsafe { m5unified_sys::m5u_display_set_clip_rect(rect.x, rect.y, rect.w, rect.h) }
    }

    pub fn clear_clip_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_clip_rect() }
    }

    pub fn color888(&self, r: u8, g: u8, b: u8) -> u16 {
        unsafe { m5unified_sys::m5u_display_color888(r, g, b) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color565(pub u16);

impl Color565 {
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub fn rgb888(r: u8, g: u8, b: u8) -> Self {
        Self(unsafe { m5unified_sys::m5u_display_color888(r, g, b) })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

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
pub enum EpdMode {
    Quality,
    Text,
    Fast,
    Fastest,
}

impl EpdMode {
    const fn raw(self) -> i32 {
        match self {
            Self::Quality => 1,
            Self::Text => 2,
            Self::Fast => 3,
            Self::Fastest => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DisplayFont {
    Default,
    Ascii8x16,
    LgfxJapanGothic12,
    DejaVu18,
}

impl DisplayFont {
    const fn raw(self) -> i32 {
        match self {
            Self::Default => 0,
            Self::Ascii8x16 => 1,
            Self::LgfxJapanGothic12 => 2,
            Self::DejaVu18 => 3,
        }
    }
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
    pub(crate) fn raw(self) -> i32 {
        match self {
            Self::AtomDisplay => 192,
            Self::UnitLcd => 193,
            Self::UnitOled => 194,
            Self::UnitMiniOled => 195,
            Self::UnitGlass => 196,
            Self::UnitGlass2 => 197,
            Self::UnitRca => 198,
            Self::ModuleDisplay => 199,
            Self::ModuleRca => 200,
            Self::Raw(value) => value,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DisplayRef {
    index: i32,
}

impl DisplayRef {
    pub fn width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_width_at(self.index) as i32 }
    }

    pub fn height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_height_at(self.index) as i32 }
    }

    pub fn clear(&mut self) {
        self.fill_screen(colors::BLACK);
    }

    pub fn fill_screen(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_screen_at(self.index, color) }
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_set_cursor_at(self.index, x as c_int, y as c_int) }
    }

    pub fn set_text_size(&mut self, size: i32) {
        unsafe { m5unified_sys::m5u_display_set_text_size_at(self.index, size as c_int) }
    }

    pub fn set_text_color(&mut self, fg: u16, bg: u16) {
        unsafe { m5unified_sys::m5u_display_set_text_color_at(self.index, fg, bg) }
    }

    pub fn rotation(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_rotation_at(self.index) as i32 }
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        unsafe {
            m5unified_sys::m5u_display_set_rotation_at(self.index, rotation as c_int);
        }
    }

    pub fn set_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_set_color_at(self.index, color) }
    }

    pub fn start_write(&mut self) {
        unsafe { m5unified_sys::m5u_display_start_write_at(self.index) }
    }

    pub fn end_write(&mut self) {
        unsafe { m5unified_sys::m5u_display_end_write_at(self.index) }
    }

    pub fn transaction<R>(&mut self, f: impl FnOnce(&mut DisplayRef) -> R) -> R {
        self.start_write();
        let result = f(self);
        self.end_write();
        result
    }

    pub fn print(&mut self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_display_print_at(self.index, text.as_ptr()) }
        Ok(())
    }

    pub fn println(&mut self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_display_println_at(self.index, text.as_ptr()) }
        Ok(())
    }

    pub fn draw_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_display_draw_string_at(self.index, text.as_ptr(), x, y) as i32
        })
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_line_at(self.index, x0, y0, x1, y1, color) }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_rect_at(self.index, x, y, w, h, color) }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_rect_at(self.index, x, y, w, h, color) }
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_circle_at(self.index, x, y, r, color) }
    }

    pub fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_circle_at(self.index, x, y, r, color) }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_pixel_at(self.index, x, y, color) }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_pixel_at(self.index, x, y, color) }
    }
}

impl M5Unified {
    pub fn display_count(&self) -> usize {
        unsafe { m5unified_sys::m5u_display_count().max(0) as usize }
    }

    pub fn display(&self, index: usize) -> Option<DisplayRef> {
        (index < self.display_count()).then_some(DisplayRef {
            index: index as i32,
        })
    }

    pub fn display_index(&self, kind: DisplayKind) -> Option<usize> {
        let index = unsafe { m5unified_sys::m5u_display_index_for_kind(kind.raw() as c_int) };
        (index >= 0).then_some(index as usize)
    }

    pub fn display_index_any(&self, kinds: &[DisplayKind]) -> Option<usize> {
        let kinds = raw_display_kinds(kinds);
        let index =
            unsafe { m5unified_sys::m5u_display_index_for_kinds(kinds.as_ptr(), kinds.len()) };
        (index >= 0).then_some(index as usize)
    }
}
