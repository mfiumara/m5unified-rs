//! Touch point and gesture helpers.
//!
//! The touch wrapper exposes point lists, raw points, detailed touch state,
//! threshold configuration, and convenience predicates for click, hold, flick,
//! and drag gestures.

use core::ffi::c_int;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct TouchPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum TouchState {
    #[default]
    None,
    Touch,
    TouchEnd,
    TouchBegin,
    Hold,
    HoldEnd,
    HoldBegin,
    Flick,
    FlickEnd,
    FlickBegin,
    Drag,
    DragEnd,
    DragBegin,
    Raw(u8),
}

impl TouchState {
    const MASK_TOUCH: u8 = 0b0001;
    const MASK_CHANGE: u8 = 0b0010;
    const MASK_HOLDING: u8 = 0b0100;
    const DRAG_MASK: u8 = 0b1101;
    const FLICK_RAW: u8 = 0b1001;
    const DRAG_RAW: u8 = 0b1101;

    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0b0000 => Self::None,
            0b0001 => Self::Touch,
            0b0010 => Self::TouchEnd,
            0b0011 => Self::TouchBegin,
            0b0101 => Self::Hold,
            0b0110 => Self::HoldEnd,
            0b0111 => Self::HoldBegin,
            0b1001 => Self::Flick,
            0b1010 => Self::FlickEnd,
            0b1011 => Self::FlickBegin,
            0b1101 => Self::Drag,
            0b1110 => Self::DragEnd,
            0b1111 => Self::DragBegin,
            raw => Self::Raw(raw),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::None => 0b0000,
            Self::Touch => 0b0001,
            Self::TouchEnd => 0b0010,
            Self::TouchBegin => 0b0011,
            Self::Hold => 0b0101,
            Self::HoldEnd => 0b0110,
            Self::HoldBegin => 0b0111,
            Self::Flick => 0b1001,
            Self::FlickEnd => 0b1010,
            Self::FlickBegin => 0b1011,
            Self::Drag => 0b1101,
            Self::DragEnd => 0b1110,
            Self::DragBegin => 0b1111,
            Self::Raw(raw) => raw,
        }
    }

    pub fn is_pressed(self) -> bool {
        self.raw() & Self::MASK_TOUCH != 0
    }

    pub fn is_released(self) -> bool {
        !self.is_pressed()
    }

    pub fn was_pressed(self) -> bool {
        self == Self::TouchBegin
    }

    pub fn was_released(self) -> bool {
        self.raw() & (Self::MASK_TOUCH | Self::MASK_CHANGE) == Self::MASK_CHANGE
    }

    pub fn was_clicked(self) -> bool {
        self == Self::TouchEnd
    }

    pub fn is_holding(self) -> bool {
        self.raw() & (Self::MASK_TOUCH | Self::MASK_HOLDING)
            == (Self::MASK_TOUCH | Self::MASK_HOLDING)
    }

    pub fn was_hold(self) -> bool {
        self == Self::HoldBegin
    }

    pub fn was_flick_start(self) -> bool {
        self == Self::FlickBegin
    }

    pub fn is_flicking(self) -> bool {
        self.raw() & Self::DRAG_MASK == Self::FLICK_RAW
    }

    pub fn was_flicked(self) -> bool {
        self == Self::FlickEnd
    }

    pub fn was_drag_start(self) -> bool {
        self == Self::DragBegin
    }

    pub fn is_dragging(self) -> bool {
        self.raw() & Self::DRAG_MASK == Self::DRAG_RAW
    }

    pub fn was_dragged(self) -> bool {
        self == Self::DragEnd
    }
}

#[derive(Debug)]
pub struct Touch;

impl Touch {
    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_touch_is_enabled() }
    }

    pub fn count(&self) -> usize {
        unsafe { m5unified_sys::m5u_touch_count() }.max(0) as usize
    }

    pub fn points(&self) -> Vec<TouchPoint> {
        (0..self.count())
            .filter_map(|index| {
                let (mut x, mut y) = (0, 0);
                let ok = unsafe { m5unified_sys::m5u_touch_get(index as c_int, &mut x, &mut y) };
                ok.then_some(TouchPoint { x, y })
            })
            .collect()
    }

    pub fn raw_point(&self, index: usize) -> Option<TouchPoint> {
        let (mut x, mut y) = (0, 0);
        let ok = unsafe { m5unified_sys::m5u_touch_get_raw(index as c_int, &mut x, &mut y) };
        ok.then_some(TouchPoint { x, y })
    }

    pub fn raw_points(&self) -> Vec<TouchPoint> {
        (0..self.count())
            .filter_map(|index| self.raw_point(index))
            .collect()
    }

    pub fn is_pressed(&self) -> bool {
        self.count() > 0
    }

    pub fn detail(&self, index: usize) -> Option<TouchDetail> {
        let mut raw = m5unified_sys::m5u_touch_detail_t::default();
        let ok = unsafe { m5unified_sys::m5u_touch_get_detail(index as c_int, &mut raw) };
        ok.then_some(TouchDetail {
            x: raw.x,
            y: raw.y,
            prev_x: raw.prev_x,
            prev_y: raw.prev_y,
            base_x: raw.base_x,
            base_y: raw.base_y,
            base_msec: raw.base_msec,
            state: TouchState::from_raw(raw.state),
            is_pressed: raw.is_pressed,
            was_pressed: raw.was_pressed,
            was_released: raw.was_released,
            was_clicked: raw.was_clicked,
            was_hold: raw.was_hold,
            is_holding: raw.is_holding,
            click_count: raw.click_count,
        })
    }

    pub fn details(&self) -> Vec<TouchDetail> {
        (0..self.count())
            .filter_map(|index| self.detail(index))
            .collect()
    }

    pub fn set_hold_thresh_ms(&mut self, ms: u16) {
        unsafe { m5unified_sys::m5u_touch_set_hold_thresh(ms) }
    }

    pub fn set_flick_thresh_px(&mut self, distance: u16) {
        unsafe { m5unified_sys::m5u_touch_set_flick_thresh(distance) }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct TouchDetail {
    pub x: i32,
    pub y: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    pub base_x: i32,
    pub base_y: i32,
    pub base_msec: u32,
    pub state: TouchState,
    pub is_pressed: bool,
    pub was_pressed: bool,
    pub was_released: bool,
    pub was_clicked: bool,
    pub was_hold: bool,
    pub is_holding: bool,
    pub click_count: i32,
}

impl TouchDetail {
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    pub fn was_pressed(&self) -> bool {
        self.was_pressed
    }

    pub fn was_clicked(&self) -> bool {
        self.was_clicked
    }

    pub fn was_released(&self) -> bool {
        self.was_released
    }

    pub fn is_holding(&self) -> bool {
        self.is_holding
    }

    pub fn was_hold(&self) -> bool {
        self.was_hold
    }

    pub fn click_count(&self) -> i32 {
        self.click_count
    }

    pub fn delta_x(&self) -> i32 {
        self.x - self.prev_x
    }

    pub fn delta_y(&self) -> i32 {
        self.y - self.prev_y
    }

    pub fn delta(&self) -> (i32, i32) {
        (self.delta_x(), self.delta_y())
    }

    pub fn distance_x(&self) -> i32 {
        self.x - self.base_x
    }

    pub fn distance_y(&self) -> i32 {
        self.y - self.base_y
    }

    pub fn distance(&self) -> (i32, i32) {
        (self.distance_x(), self.distance_y())
    }

    pub fn is_released(&self) -> bool {
        self.state.is_released()
    }

    pub fn was_flick_start(&self) -> bool {
        self.state.was_flick_start()
    }

    pub fn is_flicking(&self) -> bool {
        self.state.is_flicking()
    }

    pub fn was_flicked(&self) -> bool {
        self.state.was_flicked()
    }

    pub fn was_drag_start(&self) -> bool {
        self.state.was_drag_start()
    }

    pub fn is_dragging(&self) -> bool {
        self.state.is_dragging()
    }

    pub fn was_dragged(&self) -> bool {
        self.state.was_dragged()
    }
}
