use core::ffi::c_int;

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

    pub fn detail(&self, index: usize) -> Option<TouchDetail> {
        let mut raw = m5unified_sys::m5u_touch_detail_t::default();
        let ok = unsafe { m5unified_sys::m5u_touch_get_detail(index as c_int, &mut raw) };
        ok.then_some(TouchDetail {
            x: raw.x,
            y: raw.y,
            prev_x: raw.prev_x,
            prev_y: raw.prev_y,
            is_pressed: raw.is_pressed,
            was_pressed: raw.was_pressed,
            was_released: raw.was_released,
            was_clicked: raw.was_clicked,
            was_hold: raw.was_hold,
            is_holding: raw.is_holding,
            click_count: raw.click_count,
        })
    }
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
    pub fn delta(&self) -> (i32, i32) {
        (self.x - self.prev_x, self.y - self.prev_y)
    }
}
