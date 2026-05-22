//! Off-screen sprite canvas backed by LGFX_Sprite.
//!
//! Draw all primitives here, then call [`Canvas::push`] once per frame to
//! transfer the complete image to the display in a single DMA transaction,
//! eliminating per-primitive flicker.

use crate::{Point, Rect};

/// Off-screen canvas.  Drop frees the sprite buffer.
pub struct Canvas {
    _private: (),
}

impl Canvas {
    /// Allocate a sprite the given size.  Returns `None` on allocation failure
    /// (unlikely on devices with PSRAM).
    pub fn create(width: i32, height: i32) -> Option<Self> {
        let ok = unsafe { m5unified_sys::m5u_canvas_create(width, height) };
        ok.then_some(Self { _private: () })
    }

    /// Push the finished frame to the display starting at `(x, y)`.
    /// Call this once after drawing everything for the frame.
    pub fn push(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_canvas_push(x, y) }
    }

    pub fn fill_screen(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_fill_screen(color) }
    }

    pub fn fill_smooth_circle(&mut self, center: Point, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_fill_smooth_circle(center.x, center.y, r, color) }
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_draw_circle(x, y, r, color) }
    }

    pub fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_fill_circle(x, y, r, color) }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_fill_rect(x, y, w, h, color) }
    }

    pub fn fill_smooth_round_rect(&mut self, rect: Rect, r: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_fill_smooth_round_rect(
                rect.x, rect.y, rect.w, rect.h, r, color,
            )
        }
    }

    pub fn fill_arc(
        &mut self,
        center: Point,
        r_inner: i32,
        r_outer: i32,
        angle0: f32,
        angle1: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_canvas_fill_arc(
                center.x, center.y, r_inner, r_outer, angle0, angle1, color,
            )
        }
    }

    pub fn fill_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_fill_ellipse(x, y, rx, ry, color) }
    }

    pub fn draw_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_draw_ellipse(x, y, rx, ry, color) }
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe { m5unified_sys::m5u_canvas_delete() }
    }
}
