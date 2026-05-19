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

    pub fn brightness(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_brightness() }
    }

    pub fn sleep(&mut self) {
        unsafe { m5unified_sys::m5u_display_sleep() }
    }

    pub fn wakeup(&mut self) {
        unsafe { m5unified_sys::m5u_display_wakeup() }
    }

    pub fn power_save(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_display_power_save(enable) }
    }

    pub fn invert_display(&mut self, invert: bool) {
        unsafe { m5unified_sys::m5u_display_invert_display(invert) }
    }

    pub fn inverted(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_get_invert() }
    }

    pub fn set_swap_bytes(&mut self, swap: bool) {
        unsafe { m5unified_sys::m5u_display_set_swap_bytes(swap) }
    }

    pub fn swap_bytes(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_get_swap_bytes() }
    }

    pub fn set_color_depth(&mut self, depth: ColorDepth) {
        unsafe { m5unified_sys::m5u_display_set_color_depth(depth.raw() as c_int) }
    }

    pub fn color_depth(&self) -> ColorDepth {
        ColorDepth::new(unsafe { m5unified_sys::m5u_display_get_color_depth() as u16 })
    }

    pub fn set_addr_window(&mut self, rect: Rect) {
        unsafe { m5unified_sys::m5u_display_set_addr_window(rect.x, rect.y, rect.w, rect.h) }
    }

    pub fn set_window(&mut self, start: Point, end: Point) {
        unsafe { m5unified_sys::m5u_display_set_window(start.x, start.y, end.x, end.y) }
    }

    pub fn begin_transaction(&mut self) {
        unsafe { m5unified_sys::m5u_display_begin_transaction() }
    }

    pub fn end_transaction(&mut self) {
        unsafe { m5unified_sys::m5u_display_end_transaction() }
    }

    pub fn start_count(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_start_count() }
    }

    pub fn scan_line(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_scan_line() as i32 }
    }

    pub fn set_raw_color(&mut self, color: u32) {
        unsafe { m5unified_sys::m5u_display_set_raw_color(color) }
    }

    pub fn raw_color(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_raw_color() }
    }

    pub fn write_color(&mut self, color: u16, length: u32) {
        unsafe { m5unified_sys::m5u_display_write_color(color, length) }
    }

    pub fn draw_pixel_current(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_draw_pixel_current(x, y) }
    }

    pub fn write_pixel_current(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_write_pixel_current(x, y) }
    }

    pub fn write_fill_rect(&mut self, rect: Rect, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_fill_rect(rect.x, rect.y, rect.w, rect.h, color) }
    }

    pub fn write_fill_rect_preclipped(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_write_fill_rect_preclipped(
                rect.x, rect.y, rect.w, rect.h, color,
            )
        }
    }

    pub fn push_block(&mut self, color: u16, length: u32) {
        unsafe { m5unified_sys::m5u_display_push_block(color, length) }
    }

    pub fn progress_bar(&mut self, rect: Rect, value: u8) {
        unsafe {
            m5unified_sys::m5u_display_progress_bar(rect.x, rect.y, rect.w, rect.h, value);
        }
    }

    pub fn push_state(&mut self) {
        unsafe { m5unified_sys::m5u_display_push_state() }
    }

    pub fn pop_state(&mut self) {
        unsafe { m5unified_sys::m5u_display_pop_state() }
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

    pub fn display_region(&mut self, rect: Rect) {
        unsafe { m5unified_sys::m5u_display_display_region(rect.x, rect.y, rect.w, rect.h) }
    }

    pub fn display_busy(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_display_busy() }
    }

    pub fn wait_display(&self) {
        unsafe { m5unified_sys::m5u_display_wait_display() }
    }

    pub fn has_palette(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_has_palette() }
    }

    pub fn palette_count(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_palette_count() }
    }

    pub fn is_readable(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_is_readable() }
    }

    pub fn is_epd(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_is_epd() }
    }

    pub fn is_bus_shared(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_is_bus_shared() }
    }

    pub fn set_auto_display(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_display_set_auto_display(enable) }
    }

    pub fn init_dma(&mut self) {
        unsafe { m5unified_sys::m5u_display_init_dma() }
    }

    pub fn wait_dma(&self) {
        unsafe { m5unified_sys::m5u_display_wait_dma() }
    }

    pub fn dma_busy(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_dma_busy() }
    }

    pub fn cursor_x(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_x() as i32 }
    }

    pub fn cursor_y(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_y() as i32 }
    }

    pub fn font_height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_height() as i32 }
    }

    pub fn font_width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_width() as i32 }
    }

    pub fn base_color(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_base_color() }
    }

    pub fn set_base_color(&mut self, color: u32) {
        unsafe { m5unified_sys::m5u_display_set_base_color(color) }
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

    pub fn text_datum(&self) -> Option<TextDatum> {
        TextDatum::from_raw(unsafe { m5unified_sys::m5u_display_get_text_datum() as i32 })
    }

    pub fn draw_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_draw_string(text.as_ptr(), x, y) as i32 })
    }

    pub fn draw_center_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_draw_center_string(text.as_ptr(), x, y) as i32 })
    }

    pub fn draw_right_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_draw_right_string(text.as_ptr(), x, y) as i32 })
    }

    pub fn draw_number(&mut self, value: i32, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_number(value, x, y) as i32 }
    }

    pub fn draw_float(&mut self, value: f32, decimals: u8, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_float(value, decimals, x, y) as i32 }
    }

    pub fn draw_char(&mut self, codepoint: u16, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_char(codepoint, x, y) as i32 }
    }

    pub fn text_width(&self, text: &str) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_text_width(text.as_ptr()) as i32 })
    }

    pub fn text_length(&self, text: &str, width: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_text_length(text.as_ptr(), width) as i32 })
    }

    pub fn qr_code(&mut self, text: &str, options: QrCodeOptions) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe {
            m5unified_sys::m5u_display_qrcode(
                text.as_ptr(),
                options.position.x,
                options.position.y,
                options.width,
                options.version,
                options.margin,
            );
        }
        Ok(())
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_pixel(x, y, color) }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_pixel(x, y, color) }
    }

    pub fn draw_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_fast_hline(x, y, w, color) }
    }

    pub fn write_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_fast_hline(x, y, w, color) }
    }

    pub fn draw_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_fast_vline(x, y, h, color) }
    }

    pub fn write_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_fast_vline(x, y, h, color) }
    }

    pub fn draw_round_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_round_rect(x, y, w, h, r, color) }
    }

    pub fn fill_round_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_round_rect(x, y, w, h, r, color) }
    }

    pub fn draw_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_triangle(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color);
        }
    }

    pub fn fill_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_triangle(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color);
        }
    }

    pub fn draw_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_ellipse(x, y, rx, ry, color) }
    }

    pub fn fill_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_ellipse(x, y, rx, ry, color) }
    }

    pub fn draw_arc(
        &mut self,
        center: Point,
        r0: i32,
        r1: i32,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_draw_arc(center.x, center.y, r0, r1, angle0, angle1, color)
        }
    }

    pub fn fill_arc(
        &mut self,
        center: Point,
        r0: i32,
        r1: i32,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_fill_arc(center.x, center.y, r0, r1, angle0, angle1, color)
        }
    }

    pub fn draw_ellipse_arc(
        &mut self,
        center: Point,
        inner: Size,
        outer: Size,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_draw_ellipse_arc(
                center.x, center.y, inner.w, outer.w, inner.h, outer.h, angle0, angle1, color,
            )
        }
    }

    pub fn fill_ellipse_arc(
        &mut self,
        center: Point,
        inner: Size,
        outer: Size,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_fill_ellipse_arc(
                center.x, center.y, inner.w, outer.w, inner.h, outer.h, angle0, angle1, color,
            )
        }
    }

    pub fn draw_quadratic_bezier(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_bezier3(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color);
        }
    }

    pub fn draw_cubic_bezier(&mut self, p0: Point, p1: Point, p2: Point, p3: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_bezier4(
                p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, color,
            );
        }
    }

    pub fn draw_smooth_line(&mut self, start: Point, end: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_smooth_line(start.x, start.y, end.x, end.y, color);
        }
    }

    pub fn draw_wide_line(&mut self, start: Point, end: Point, radius: f32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_wide_line(
                start.x, start.y, end.x, end.y, radius, color,
            );
        }
    }

    pub fn draw_wedge_line(&mut self, start: Point, end: Point, r0: f32, r1: f32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_wedge_line(
                start.x, start.y, end.x, end.y, r0, r1, color,
            );
        }
    }

    pub fn draw_gradient_line(
        &mut self,
        start: Point,
        end: Point,
        start_color: u16,
        end_color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_draw_gradient_line(
                start.x,
                start.y,
                end.x,
                end.y,
                start_color,
                end_color,
            );
        }
    }

    pub fn draw_spot(&mut self, center: Point, radius: f32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_spot(center.x, center.y, radius, color);
        }
    }

    pub fn fill_smooth_circle(&mut self, center: Point, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_smooth_circle(center.x, center.y, radius, color);
        }
    }

    pub fn fill_smooth_round_rect(&mut self, rect: Rect, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_smooth_round_rect(
                rect.x, rect.y, rect.w, rect.h, radius, color,
            );
        }
    }

    pub fn fill_gradient_rect(
        &mut self,
        rect: Rect,
        start_color: u16,
        end_color: u16,
        style: GradientFillStyle,
    ) {
        unsafe {
            m5unified_sys::m5u_display_fill_gradient_rect(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                start_color,
                end_color,
                style.raw() as c_int,
            );
        }
    }

    pub fn flood_fill(&mut self, point: Point, color: u16) {
        unsafe { m5unified_sys::m5u_display_flood_fill(point.x, point.y, color) }
    }

    pub fn paint(&mut self, point: Point, color: u16) {
        self.flood_fill(point, color);
    }

    pub fn set_scroll_rect(&mut self, rect: Rect) {
        unsafe { m5unified_sys::m5u_display_set_scroll_rect(rect.x, rect.y, rect.w, rect.h) }
    }

    pub fn set_scroll_rect_color(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_set_scroll_rect_color(rect.x, rect.y, rect.w, rect.h, color);
        }
    }

    pub fn scroll_rect(&self) -> Rect {
        let mut rect = Rect::default();
        unsafe {
            m5unified_sys::m5u_display_get_scroll_rect(
                &mut rect.x,
                &mut rect.y,
                &mut rect.w,
                &mut rect.h,
            );
        }
        rect
    }

    pub fn clear_scroll_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_scroll_rect() }
    }

    pub fn scroll(&mut self, dx: i32, dy: i32) {
        unsafe { m5unified_sys::m5u_display_scroll(dx, dy) }
    }

    pub fn set_text_padding(&mut self, padding: u32) {
        unsafe { m5unified_sys::m5u_display_set_text_padding(padding) }
    }

    pub fn text_padding(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_text_padding() }
    }

    pub fn text_size_x(&self) -> f32 {
        unsafe { m5unified_sys::m5u_display_get_text_size_x() }
    }

    pub fn text_size_y(&self) -> f32 {
        unsafe { m5unified_sys::m5u_display_get_text_size_y() }
    }

    pub fn set_clip_rect(&mut self, rect: Rect) {
        unsafe { m5unified_sys::m5u_display_set_clip_rect(rect.x, rect.y, rect.w, rect.h) }
    }

    pub fn clip_rect(&self) -> Rect {
        let mut rect = Rect::default();
        unsafe {
            m5unified_sys::m5u_display_get_clip_rect(
                &mut rect.x,
                &mut rect.y,
                &mut rect.w,
                &mut rect.h,
            );
        }
        rect
    }

    pub fn clear_clip_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_clip_rect() }
    }

    pub fn color888(&self, r: u8, g: u8, b: u8) -> u32 {
        unsafe { m5unified_sys::m5u_display_color888(r, g, b) }
    }

    pub fn set_pivot(&mut self, x: f32, y: f32) {
        unsafe { m5unified_sys::m5u_display_set_pivot(x, y) }
    }

    pub fn pivot_x(&self) -> f32 {
        unsafe { m5unified_sys::m5u_display_get_pivot_x() }
    }

    pub fn pivot_y(&self) -> f32 {
        unsafe { m5unified_sys::m5u_display_get_pivot_y() }
    }

    pub fn pivot(&self) -> (f32, f32) {
        (self.pivot_x(), self.pivot_y())
    }

    pub fn push_image_rgb565(&mut self, rect: Rect, pixels: &[u16]) -> Result<(), Error> {
        let required = validate_pixel_buffer(rect, pixels.len())?;
        if required == 0 {
            return Ok(());
        }

        let ok = unsafe {
            m5unified_sys::m5u_display_push_image_rgb565(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                pixels.as_ptr(),
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display push_image_rgb565"))
    }

    pub fn push_image_rgb565_transparent(
        &mut self,
        rect: Rect,
        pixels: &[u16],
        transparent: u16,
    ) -> Result<(), Error> {
        let required = validate_pixel_buffer(rect, pixels.len())?;
        if required == 0 {
            return Ok(());
        }

        let ok = unsafe {
            m5unified_sys::m5u_display_push_image_rgb565_transparent(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                pixels.as_ptr(),
                transparent,
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display push_image_rgb565_transparent"))
    }

    pub fn read_pixel(&mut self, x: i32, y: i32) -> u16 {
        unsafe { m5unified_sys::m5u_display_read_pixel(x, y) }
    }

    pub fn read_rect_rgb565(&mut self, rect: Rect, pixels: &mut [u16]) -> Result<(), Error> {
        let required = validate_pixel_buffer(rect, pixels.len())?;
        if required == 0 {
            return Ok(());
        }

        let ok = unsafe {
            m5unified_sys::m5u_display_read_rect_rgb565(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                pixels.as_mut_ptr(),
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display read_rect_rgb565"))
    }

    pub fn copy_rect(&mut self, dst: Point, size: Size, src: Point) {
        unsafe {
            m5unified_sys::m5u_display_copy_rect(dst.x, dst.y, size.w, size.h, src.x, src.y);
        }
    }

    pub fn draw_image(
        &mut self,
        format: ImageFormat,
        data: &[u8],
        options: ImageDrawOptions,
    ) -> Result<(), Error> {
        validate_encoded_image_buffer(data)?;
        let options = options.to_raw();
        let ok = unsafe {
            m5unified_sys::m5u_display_draw_image(
                format.raw() as c_int,
                data.as_ptr(),
                data.len(),
                &options,
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display draw_image"))
    }

    pub fn draw_image_file(
        &mut self,
        format: ImageFormat,
        path: &str,
        options: ImageDrawOptions,
    ) -> Result<(), Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        let options = options.to_raw();
        let ok = unsafe {
            m5unified_sys::m5u_display_draw_image_file(
                format.raw() as c_int,
                path.as_ptr(),
                &options,
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display draw_image_file"))
    }

    pub fn draw_bmp(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image(ImageFormat::Bmp, data, options)
    }

    pub fn draw_jpg(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image(ImageFormat::Jpg, data, options)
    }

    pub fn draw_png(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image(ImageFormat::Png, data, options)
    }

    pub fn draw_qoi(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image(ImageFormat::Qoi, data, options)
    }

    pub fn draw_bmp_file(&mut self, path: &str, options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image_file(ImageFormat::Bmp, path, options)
    }

    pub fn draw_jpg_file(&mut self, path: &str, options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image_file(ImageFormat::Jpg, path, options)
    }

    pub fn draw_png_file(&mut self, path: &str, options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image_file(ImageFormat::Png, path, options)
    }

    pub fn draw_qoi_file(&mut self, path: &str, options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image_file(ImageFormat::Qoi, path, options)
    }
}

impl core::fmt::Write for Display {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s).map_err(|_| core::fmt::Error)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color565(pub u16);

impl Color565 {
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub fn rgb888(r: u8, g: u8, b: u8) -> Self {
        Self(((u16::from(r) >> 3) << 11) | ((u16::from(g) >> 2) << 5) | (u16::from(b) >> 3))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ColorDepth(pub u16);

impl ColorDepth {
    pub const BIT_MASK: u16 = 0x00FF;
    pub const HAS_PALETTE: u16 = 0x0800;
    pub const NONSWAPPED: u16 = 0x0100;
    pub const ALTERNATE: u16 = 0x1000;

    pub const GRAYSCALE_1BIT: Self = Self(1);
    pub const GRAYSCALE_2BIT: Self = Self(2);
    pub const GRAYSCALE_4BIT: Self = Self(4);
    pub const GRAYSCALE_8BIT: Self = Self(8 | Self::ALTERNATE);
    pub const PALETTE_1BIT: Self = Self(1 | Self::HAS_PALETTE);
    pub const PALETTE_2BIT: Self = Self(2 | Self::HAS_PALETTE);
    pub const PALETTE_4BIT: Self = Self(4 | Self::HAS_PALETTE);
    pub const PALETTE_8BIT: Self = Self(8 | Self::HAS_PALETTE);
    pub const RGB332_1BYTE: Self = Self(8);
    pub const RGB565_2BYTE: Self = Self(16);
    pub const RGB666_3BYTE: Self = Self(24 | Self::ALTERNATE);
    pub const RGB888_3BYTE: Self = Self(24);
    pub const ARGB8888_4BYTE: Self = Self(32);
    pub const RGB565_NONSWAPPED: Self = Self(16 | Self::NONSWAPPED);
    pub const RGB666_NONSWAPPED: Self = Self(24 | Self::NONSWAPPED | Self::ALTERNATE);
    pub const RGB888_NONSWAPPED: Self = Self(24 | Self::NONSWAPPED);
    pub const ARGB8888_NONSWAPPED: Self = Self(32 | Self::NONSWAPPED);

    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u16 {
        self.0
    }

    pub const fn bits(self) -> u8 {
        (self.0 & Self::BIT_MASK) as u8
    }

    pub const fn has_palette(self) -> bool {
        self.0 & Self::HAS_PALETTE != 0
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

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    fn pixel_count(self) -> Option<usize> {
        if self.w <= 0 || self.h <= 0 {
            return Some(0);
        }

        (self.w as usize).checked_mul(self.h as usize)
    }
}

fn validate_pixel_buffer(rect: Rect, len: usize) -> Result<usize, Error> {
    let required = rect.pixel_count().ok_or(Error::InvalidBufferLength)?;
    if len < required {
        return Err(Error::InvalidBufferLength);
    }
    Ok(required)
}

fn validate_encoded_image_buffer(data: &[u8]) -> Result<(), Error> {
    if data.is_empty() || data.len() > u32::MAX as usize {
        return Err(Error::InvalidBufferLength);
    }
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
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

impl TextDatum {
    const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::TopLeft),
            1 => Some(Self::TopCenter),
            2 => Some(Self::TopRight),
            4 => Some(Self::MiddleLeft),
            5 => Some(Self::MiddleCenter),
            6 => Some(Self::MiddleRight),
            8 => Some(Self::BottomLeft),
            9 => Some(Self::BottomCenter),
            10 => Some(Self::BottomRight),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImageFormat {
    Bmp,
    Jpg,
    Png,
    Qoi,
}

impl ImageFormat {
    const fn raw(self) -> i32 {
        match self {
            Self::Bmp => 0,
            Self::Jpg => 1,
            Self::Png => 2,
            Self::Qoi => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ImageDrawOptions {
    pub position: Point,
    pub max_size: Size,
    pub offset: Point,
    pub scale_x: f32,
    pub scale_y: f32,
    pub datum: TextDatum,
}

impl Default for ImageDrawOptions {
    fn default() -> Self {
        Self {
            position: Point { x: 0, y: 0 },
            max_size: Size { w: 0, h: 0 },
            offset: Point { x: 0, y: 0 },
            scale_x: 1.0,
            scale_y: 0.0,
            datum: TextDatum::TopLeft,
        }
    }
}

impl ImageDrawOptions {
    fn to_raw(self) -> m5unified_sys::m5u_image_options_t {
        m5unified_sys::m5u_image_options_t {
            x: self.position.x,
            y: self.position.y,
            max_width: self.max_size.w,
            max_height: self.max_size.h,
            off_x: self.offset.x,
            off_y: self.offset.y,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            datum: self.datum as c_int,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct QrCodeOptions {
    pub position: Point,
    pub width: i32,
    pub version: u8,
    pub margin: bool,
}

impl Default for QrCodeOptions {
    fn default() -> Self {
        Self {
            position: Point { x: -1, y: -1 },
            width: -1,
            version: 1,
            margin: false,
        }
    }
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
pub enum GradientFillStyle {
    HorizontalLinear,
    VerticalLinear,
    RadialCenter,
}

impl GradientFillStyle {
    const fn raw(self) -> i32 {
        match self {
            Self::HorizontalLinear => 0,
            Self::VerticalLinear => 1,
            Self::RadialCenter => 2,
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

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_display_set_brightness_at(self.index, brightness) }
    }

    pub fn brightness(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_brightness_at(self.index) }
    }

    pub fn sleep(&mut self) {
        unsafe { m5unified_sys::m5u_display_sleep_at(self.index) }
    }

    pub fn wakeup(&mut self) {
        unsafe { m5unified_sys::m5u_display_wakeup_at(self.index) }
    }

    pub fn power_save(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_display_power_save_at(self.index, enable) }
    }

    pub fn invert_display(&mut self, invert: bool) {
        unsafe { m5unified_sys::m5u_display_invert_display_at(self.index, invert) }
    }

    pub fn inverted(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_get_invert_at(self.index) }
    }

    pub fn set_swap_bytes(&mut self, swap: bool) {
        unsafe { m5unified_sys::m5u_display_set_swap_bytes_at(self.index, swap) }
    }

    pub fn swap_bytes(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_get_swap_bytes_at(self.index) }
    }

    pub fn set_color_depth(&mut self, depth: ColorDepth) {
        unsafe { m5unified_sys::m5u_display_set_color_depth_at(self.index, depth.raw() as c_int) }
    }

    pub fn color_depth(&self) -> ColorDepth {
        ColorDepth::new(unsafe { m5unified_sys::m5u_display_get_color_depth_at(self.index) as u16 })
    }

    pub fn set_addr_window(&mut self, rect: Rect) {
        unsafe {
            m5unified_sys::m5u_display_set_addr_window_at(
                self.index, rect.x, rect.y, rect.w, rect.h,
            );
        }
    }

    pub fn set_window(&mut self, start: Point, end: Point) {
        unsafe {
            m5unified_sys::m5u_display_set_window_at(self.index, start.x, start.y, end.x, end.y);
        }
    }

    pub fn begin_transaction(&mut self) {
        unsafe { m5unified_sys::m5u_display_begin_transaction_at(self.index) }
    }

    pub fn end_transaction(&mut self) {
        unsafe { m5unified_sys::m5u_display_end_transaction_at(self.index) }
    }

    pub fn start_count(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_start_count_at(self.index) }
    }

    pub fn scan_line(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_scan_line_at(self.index) as i32 }
    }

    pub fn set_raw_color(&mut self, color: u32) {
        unsafe { m5unified_sys::m5u_display_set_raw_color_at(self.index, color) }
    }

    pub fn raw_color(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_raw_color_at(self.index) }
    }

    pub fn write_color(&mut self, color: u16, length: u32) {
        unsafe { m5unified_sys::m5u_display_write_color_at(self.index, color, length) }
    }

    pub fn draw_pixel_current(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_draw_pixel_current_at(self.index, x, y) }
    }

    pub fn write_pixel_current(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_write_pixel_current_at(self.index, x, y) }
    }

    pub fn write_fill_rect(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_write_fill_rect_at(
                self.index, rect.x, rect.y, rect.w, rect.h, color,
            );
        }
    }

    pub fn write_fill_rect_preclipped(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_write_fill_rect_preclipped_at(
                self.index, rect.x, rect.y, rect.w, rect.h, color,
            );
        }
    }

    pub fn push_block(&mut self, color: u16, length: u32) {
        unsafe { m5unified_sys::m5u_display_push_block_at(self.index, color, length) }
    }

    pub fn progress_bar(&mut self, rect: Rect, value: u8) {
        unsafe {
            m5unified_sys::m5u_display_progress_bar_at(
                self.index, rect.x, rect.y, rect.w, rect.h, value,
            );
        }
    }

    pub fn push_state(&mut self) {
        unsafe { m5unified_sys::m5u_display_push_state_at(self.index) }
    }

    pub fn pop_state(&mut self) {
        unsafe { m5unified_sys::m5u_display_pop_state_at(self.index) }
    }

    pub fn set_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_set_color_at(self.index, color) }
    }

    pub fn base_color(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_base_color_at(self.index) }
    }

    pub fn set_base_color(&mut self, color: u32) {
        unsafe { m5unified_sys::m5u_display_set_base_color_at(self.index, color) }
    }

    pub fn cursor_x(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_x_at(self.index) as i32 }
    }

    pub fn cursor_y(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_y_at(self.index) as i32 }
    }

    pub fn font_width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_width_at(self.index) as i32 }
    }

    pub fn start_write(&mut self) {
        unsafe { m5unified_sys::m5u_display_start_write_at(self.index) }
    }

    pub fn end_write(&mut self) {
        unsafe { m5unified_sys::m5u_display_end_write_at(self.index) }
    }

    pub fn display(&mut self) {
        unsafe { m5unified_sys::m5u_display_display_at(self.index) }
    }

    pub fn display_region(&mut self, rect: Rect) {
        unsafe {
            m5unified_sys::m5u_display_display_region_at(self.index, rect.x, rect.y, rect.w, rect.h)
        }
    }

    pub fn display_busy(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_display_busy_at(self.index) }
    }

    pub fn wait_display(&self) {
        unsafe { m5unified_sys::m5u_display_wait_display_at(self.index) }
    }

    pub fn has_palette(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_has_palette_at(self.index) }
    }

    pub fn palette_count(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_palette_count_at(self.index) }
    }

    pub fn is_readable(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_is_readable_at(self.index) }
    }

    pub fn is_epd(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_is_epd_at(self.index) }
    }

    pub fn is_bus_shared(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_is_bus_shared_at(self.index) }
    }

    pub fn set_auto_display(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_display_set_auto_display_at(self.index, enable) }
    }

    pub fn init_dma(&mut self) {
        unsafe { m5unified_sys::m5u_display_init_dma_at(self.index) }
    }

    pub fn wait_dma(&self) {
        unsafe { m5unified_sys::m5u_display_wait_dma_at(self.index) }
    }

    pub fn dma_busy(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_dma_busy_at(self.index) }
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

    pub fn draw_center_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_display_draw_center_string_at(self.index, text.as_ptr(), x, y) as i32
        })
    }

    pub fn draw_right_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_display_draw_right_string_at(self.index, text.as_ptr(), x, y) as i32
        })
    }

    pub fn draw_number(&mut self, value: i32, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_number_at(self.index, value, x, y) as i32 }
    }

    pub fn draw_float(&mut self, value: f32, decimals: u8, x: i32, y: i32) -> i32 {
        unsafe {
            m5unified_sys::m5u_display_draw_float_at(self.index, value, decimals, x, y) as i32
        }
    }

    pub fn draw_char(&mut self, codepoint: u16, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_char_at(self.index, codepoint, x, y) as i32 }
    }

    pub fn text_width(&self, text: &str) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_text_width_at(self.index, text.as_ptr()) as i32 })
    }

    pub fn text_length(&self, text: &str, width: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_display_text_length_at(self.index, text.as_ptr(), width) as i32
        })
    }

    pub fn qr_code(&mut self, text: &str, options: QrCodeOptions) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe {
            m5unified_sys::m5u_display_qrcode_at(
                self.index,
                text.as_ptr(),
                options.position.x,
                options.position.y,
                options.width,
                options.version,
                options.margin,
            );
        }
        Ok(())
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

    pub fn draw_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_fast_hline_at(self.index, x, y, w, color) }
    }

    pub fn write_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_fast_hline_at(self.index, x, y, w, color) }
    }

    pub fn draw_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_fast_vline_at(self.index, x, y, h, color) }
    }

    pub fn write_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_fast_vline_at(self.index, x, y, h, color) }
    }

    pub fn draw_round_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_round_rect_at(self.index, x, y, w, h, r, color) }
    }

    pub fn fill_round_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_round_rect_at(self.index, x, y, w, h, r, color) }
    }

    pub fn draw_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_triangle_at(
                self.index, p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color,
            );
        }
    }

    pub fn fill_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_triangle_at(
                self.index, p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color,
            );
        }
    }

    pub fn draw_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_ellipse_at(self.index, x, y, rx, ry, color) }
    }

    pub fn fill_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_ellipse_at(self.index, x, y, rx, ry, color) }
    }

    pub fn draw_arc(
        &mut self,
        center: Point,
        r0: i32,
        r1: i32,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_draw_arc_at(
                self.index, center.x, center.y, r0, r1, angle0, angle1, color,
            );
        }
    }

    pub fn fill_arc(
        &mut self,
        center: Point,
        r0: i32,
        r1: i32,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_fill_arc_at(
                self.index, center.x, center.y, r0, r1, angle0, angle1, color,
            );
        }
    }

    pub fn draw_ellipse_arc(
        &mut self,
        center: Point,
        inner: Size,
        outer: Size,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_draw_ellipse_arc_at(
                self.index, center.x, center.y, inner.w, outer.w, inner.h, outer.h, angle0, angle1,
                color,
            );
        }
    }

    pub fn fill_ellipse_arc(
        &mut self,
        center: Point,
        inner: Size,
        outer: Size,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_fill_ellipse_arc_at(
                self.index, center.x, center.y, inner.w, outer.w, inner.h, outer.h, angle0, angle1,
                color,
            );
        }
    }

    pub fn draw_quadratic_bezier(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_bezier3_at(
                self.index, p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color,
            );
        }
    }

    pub fn draw_cubic_bezier(&mut self, p0: Point, p1: Point, p2: Point, p3: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_bezier4_at(
                self.index, p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, color,
            );
        }
    }

    pub fn draw_smooth_line(&mut self, start: Point, end: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_smooth_line_at(
                self.index, start.x, start.y, end.x, end.y, color,
            );
        }
    }

    pub fn draw_wide_line(&mut self, start: Point, end: Point, radius: f32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_wide_line_at(
                self.index, start.x, start.y, end.x, end.y, radius, color,
            );
        }
    }

    pub fn draw_wedge_line(&mut self, start: Point, end: Point, r0: f32, r1: f32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_wedge_line_at(
                self.index, start.x, start.y, end.x, end.y, r0, r1, color,
            );
        }
    }

    pub fn draw_gradient_line(
        &mut self,
        start: Point,
        end: Point,
        start_color: u16,
        end_color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_draw_gradient_line_at(
                self.index,
                start.x,
                start.y,
                end.x,
                end.y,
                start_color,
                end_color,
            );
        }
    }

    pub fn draw_spot(&mut self, center: Point, radius: f32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_spot_at(self.index, center.x, center.y, radius, color);
        }
    }

    pub fn fill_smooth_circle(&mut self, center: Point, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_smooth_circle_at(
                self.index, center.x, center.y, radius, color,
            );
        }
    }

    pub fn fill_smooth_round_rect(&mut self, rect: Rect, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_smooth_round_rect_at(
                self.index, rect.x, rect.y, rect.w, rect.h, radius, color,
            );
        }
    }

    pub fn fill_gradient_rect(
        &mut self,
        rect: Rect,
        start_color: u16,
        end_color: u16,
        style: GradientFillStyle,
    ) {
        unsafe {
            m5unified_sys::m5u_display_fill_gradient_rect_at(
                self.index,
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                start_color,
                end_color,
                style.raw() as c_int,
            );
        }
    }

    pub fn flood_fill(&mut self, point: Point, color: u16) {
        unsafe { m5unified_sys::m5u_display_flood_fill_at(self.index, point.x, point.y, color) }
    }

    pub fn paint(&mut self, point: Point, color: u16) {
        self.flood_fill(point, color);
    }

    pub fn set_scroll_rect(&mut self, rect: Rect) {
        unsafe {
            m5unified_sys::m5u_display_set_scroll_rect_at(
                self.index, rect.x, rect.y, rect.w, rect.h,
            );
        }
    }

    pub fn set_scroll_rect_color(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_set_scroll_rect_color_at(
                self.index, rect.x, rect.y, rect.w, rect.h, color,
            );
        }
    }

    pub fn scroll_rect(&self) -> Rect {
        let mut rect = Rect::default();
        unsafe {
            m5unified_sys::m5u_display_get_scroll_rect_at(
                self.index,
                &mut rect.x,
                &mut rect.y,
                &mut rect.w,
                &mut rect.h,
            );
        }
        rect
    }

    pub fn clear_scroll_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_scroll_rect_at(self.index) }
    }

    pub fn scroll(&mut self, dx: i32, dy: i32) {
        unsafe { m5unified_sys::m5u_display_scroll_at(self.index, dx, dy) }
    }

    pub fn text_datum(&self) -> Option<TextDatum> {
        TextDatum::from_raw(unsafe { m5unified_sys::m5u_display_get_text_datum_at(self.index) })
    }

    pub fn set_text_padding(&mut self, padding: u32) {
        unsafe { m5unified_sys::m5u_display_set_text_padding_at(self.index, padding) }
    }

    pub fn text_padding(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_text_padding_at(self.index) }
    }

    pub fn text_size_x(&self) -> f32 {
        unsafe { m5unified_sys::m5u_display_get_text_size_x_at(self.index) }
    }

    pub fn text_size_y(&self) -> f32 {
        unsafe { m5unified_sys::m5u_display_get_text_size_y_at(self.index) }
    }

    pub fn set_clip_rect(&mut self, rect: Rect) {
        unsafe {
            m5unified_sys::m5u_display_set_clip_rect_at(self.index, rect.x, rect.y, rect.w, rect.h);
        }
    }

    pub fn clip_rect(&self) -> Rect {
        let mut rect = Rect::default();
        unsafe {
            m5unified_sys::m5u_display_get_clip_rect_at(
                self.index,
                &mut rect.x,
                &mut rect.y,
                &mut rect.w,
                &mut rect.h,
            );
        }
        rect
    }

    pub fn clear_clip_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_clip_rect_at(self.index) }
    }

    pub fn set_pivot(&mut self, x: f32, y: f32) {
        unsafe { m5unified_sys::m5u_display_set_pivot_at(self.index, x, y) }
    }

    pub fn pivot_x(&self) -> f32 {
        unsafe { m5unified_sys::m5u_display_get_pivot_x_at(self.index) }
    }

    pub fn pivot_y(&self) -> f32 {
        unsafe { m5unified_sys::m5u_display_get_pivot_y_at(self.index) }
    }

    pub fn pivot(&self) -> (f32, f32) {
        (self.pivot_x(), self.pivot_y())
    }

    pub fn push_image_rgb565(&mut self, rect: Rect, pixels: &[u16]) -> Result<(), Error> {
        let required = validate_pixel_buffer(rect, pixels.len())?;
        if required == 0 {
            return Ok(());
        }

        let ok = unsafe {
            m5unified_sys::m5u_display_push_image_rgb565_at(
                self.index,
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                pixels.as_ptr(),
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display push_image_rgb565"))
    }

    pub fn push_image_rgb565_transparent(
        &mut self,
        rect: Rect,
        pixels: &[u16],
        transparent: u16,
    ) -> Result<(), Error> {
        let required = validate_pixel_buffer(rect, pixels.len())?;
        if required == 0 {
            return Ok(());
        }

        let ok = unsafe {
            m5unified_sys::m5u_display_push_image_rgb565_transparent_at(
                self.index,
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                pixels.as_ptr(),
                transparent,
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display push_image_rgb565_transparent"))
    }

    pub fn read_pixel(&mut self, x: i32, y: i32) -> u16 {
        unsafe { m5unified_sys::m5u_display_read_pixel_at(self.index, x, y) }
    }

    pub fn read_rect_rgb565(&mut self, rect: Rect, pixels: &mut [u16]) -> Result<(), Error> {
        let required = validate_pixel_buffer(rect, pixels.len())?;
        if required == 0 {
            return Ok(());
        }

        let ok = unsafe {
            m5unified_sys::m5u_display_read_rect_rgb565_at(
                self.index,
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                pixels.as_mut_ptr(),
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display read_rect_rgb565"))
    }

    pub fn copy_rect(&mut self, dst: Point, size: Size, src: Point) {
        unsafe {
            m5unified_sys::m5u_display_copy_rect_at(
                self.index, dst.x, dst.y, size.w, size.h, src.x, src.y,
            );
        }
    }

    pub fn draw_image(
        &mut self,
        format: ImageFormat,
        data: &[u8],
        options: ImageDrawOptions,
    ) -> Result<(), Error> {
        validate_encoded_image_buffer(data)?;
        let options = options.to_raw();
        let ok = unsafe {
            m5unified_sys::m5u_display_draw_image_at(
                self.index,
                format.raw() as c_int,
                data.as_ptr(),
                data.len(),
                &options,
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display draw_image"))
    }

    pub fn draw_image_file(
        &mut self,
        format: ImageFormat,
        path: &str,
        options: ImageDrawOptions,
    ) -> Result<(), Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        let options = options.to_raw();
        let ok = unsafe {
            m5unified_sys::m5u_display_draw_image_file_at(
                self.index,
                format.raw() as c_int,
                path.as_ptr(),
                &options,
            )
        };
        ok.then_some(())
            .ok_or(Error::Unavailable("display draw_image_file"))
    }

    pub fn draw_bmp(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image(ImageFormat::Bmp, data, options)
    }

    pub fn draw_jpg(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image(ImageFormat::Jpg, data, options)
    }

    pub fn draw_png(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image(ImageFormat::Png, data, options)
    }

    pub fn draw_qoi(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image(ImageFormat::Qoi, data, options)
    }

    pub fn draw_bmp_file(&mut self, path: &str, options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image_file(ImageFormat::Bmp, path, options)
    }

    pub fn draw_jpg_file(&mut self, path: &str, options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image_file(ImageFormat::Jpg, path, options)
    }

    pub fn draw_png_file(&mut self, path: &str, options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image_file(ImageFormat::Png, path, options)
    }

    pub fn draw_qoi_file(&mut self, path: &str, options: ImageDrawOptions) -> Result<(), Error> {
        self.draw_image_file(ImageFormat::Qoi, path, options)
    }
}

impl core::fmt::Write for DisplayRef {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s).map_err(|_| core::fmt::Error)
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
