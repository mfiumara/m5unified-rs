#[derive(Debug)]
pub struct Led;

impl Led {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_led_begin() }
    }

    pub fn display(&mut self) {
        unsafe { m5unified_sys::m5u_led_display() }
    }

    pub fn set_auto_display(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_led_set_auto_display(enable) }
    }

    pub fn count(&self) -> usize {
        unsafe { m5unified_sys::m5u_led_count() }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_led_set_brightness(brightness) }
    }

    pub fn set_color(&mut self, index: usize, color: LedColor) {
        unsafe { m5unified_sys::m5u_led_set_color_rgb(index, color.r, color.g, color.b) }
    }

    pub fn set_all_color(&mut self, color: LedColor) {
        unsafe { m5unified_sys::m5u_led_set_all_color_rgb(color.r, color.g, color.b) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_led_is_enabled() }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct LedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl LedColor {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
