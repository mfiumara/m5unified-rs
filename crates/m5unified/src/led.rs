//! RGB LED helpers.
//!
//! The LED wrapper exposes the count, type, brightness, auto-display behavior,
//! and per-pixel or bulk color setters for boards with controllable LEDs.

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

    pub fn set_colors(&mut self, index: usize, colors: &[LedColor]) {
        let raw: Vec<m5unified_sys::m5u_led_color_t> = colors
            .iter()
            .map(|color| m5unified_sys::m5u_led_color_t {
                r: color.r,
                g: color.g,
                b: color.b,
            })
            .collect();
        unsafe { m5unified_sys::m5u_led_set_colors_rgb(raw.as_ptr(), index, raw.len()) }
    }

    pub fn led_type(&self, index: usize) -> LedType {
        LedType::from_raw(unsafe { m5unified_sys::m5u_led_get_type(index) })
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_led_is_enabled() }
    }

    pub fn power_hub(&self) -> LedPowerHub {
        LedPowerHub
    }

    pub fn strip(&self) -> LedStrip {
        LedStrip
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LedType {
    Unknown,
    FullColor,
    Single,
    Raw(i32),
}

impl LedType {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Unknown,
            1 => Self::FullColor,
            2 => Self::Single,
            other => Self::Raw(other),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::FullColor => 1,
            Self::Single => 2,
            Self::Raw(raw) => raw,
        }
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

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct LedPowerHub;

impl LedPowerHub {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_led_power_hub_begin() }
    }

    pub fn count(&self) -> usize {
        unsafe { m5unified_sys::m5u_led_power_hub_count() }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_led_power_hub_set_brightness(brightness) }
    }

    pub fn set_color(&mut self, index: usize, color: LedColor) {
        unsafe { m5unified_sys::m5u_led_power_hub_set_color_rgb(index, color.r, color.g, color.b) }
    }

    pub fn set_colors(&mut self, index: usize, colors: &[LedColor]) {
        let raw: Vec<m5unified_sys::m5u_led_color_t> = colors
            .iter()
            .map(|color| m5unified_sys::m5u_led_color_t {
                r: color.r,
                g: color.g,
                b: color.b,
            })
            .collect();
        unsafe { m5unified_sys::m5u_led_power_hub_set_colors_rgb(raw.as_ptr(), index, raw.len()) }
    }

    pub fn display(&mut self) {
        unsafe { m5unified_sys::m5u_led_power_hub_display() }
    }

    pub fn led_type(&self, index: usize) -> LedType {
        LedType::from_raw(unsafe { m5unified_sys::m5u_led_power_hub_get_type(index) })
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct LedStrip;

impl LedStrip {
    pub fn set_config(&mut self, config: LedStripConfig) -> bool {
        let raw = config.to_raw();
        unsafe { m5unified_sys::m5u_led_strip_set_config(&raw) }
    }

    pub fn set_rmt_bus_config(&mut self, config: LedStripRmtConfig) -> bool {
        let raw = config.to_raw();
        unsafe { m5unified_sys::m5u_led_strip_set_rmt_bus_config(&raw) }
    }

    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_led_strip_begin() }
    }

    pub fn count(&self) -> usize {
        unsafe { m5unified_sys::m5u_led_strip_count() }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_led_strip_set_brightness(brightness) }
    }

    pub fn set_color(&mut self, index: usize, color: LedColor) {
        unsafe {
            m5unified_sys::m5u_led_strip_set_color_rgb(index, color.r, color.g, color.b);
        }
    }

    pub fn set_colors(&mut self, index: usize, colors: &[LedColor]) {
        let raw: Vec<m5unified_sys::m5u_led_color_t> = colors
            .iter()
            .map(|color| m5unified_sys::m5u_led_color_t {
                r: color.r,
                g: color.g,
                b: color.b,
            })
            .collect();
        unsafe { m5unified_sys::m5u_led_strip_set_colors_rgb(raw.as_ptr(), index, raw.len()) }
    }

    pub fn display(&mut self) {
        unsafe { m5unified_sys::m5u_led_strip_display() }
    }

    pub fn led_type(&self, index: usize) -> LedType {
        LedType::from_raw(unsafe { m5unified_sys::m5u_led_strip_get_type(index) })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LedStripConfig {
    pub led_count: usize,
    pub color_order: LedStripColorOrder,
    pub byte_per_led: u8,
}

impl LedStripConfig {
    pub const fn new(led_count: usize, color_order: LedStripColorOrder, byte_per_led: u8) -> Self {
        Self {
            led_count,
            color_order,
            byte_per_led,
        }
    }

    const fn to_raw(self) -> m5unified_sys::m5u_led_strip_config_t {
        m5unified_sys::m5u_led_strip_config_t {
            led_count: self.led_count,
            color_order: self.color_order.raw(),
            byte_per_led: self.byte_per_led,
        }
    }
}

impl Default for LedStripConfig {
    fn default() -> Self {
        Self {
            led_count: 1,
            color_order: LedStripColorOrder::Grb,
            byte_per_led: 3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LedStripColorOrder {
    Rgb,
    Rbg,
    Grb,
    Gbr,
    Brg,
    Bgr,
    Raw(i32),
}

impl LedStripColorOrder {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Rgb,
            1 => Self::Rbg,
            2 => Self::Grb,
            3 => Self::Gbr,
            4 => Self::Brg,
            5 => Self::Bgr,
            other => Self::Raw(other),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::Rgb => 0,
            Self::Rbg => 1,
            Self::Grb => 2,
            Self::Gbr => 3,
            Self::Brg => 4,
            Self::Bgr => 5,
            Self::Raw(raw) => raw,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LedStripRmtConfig {
    pub frequency: u32,
    pub t0h_ns: u16,
    pub t0l_ns: u16,
    pub t1h_ns: u16,
    pub t1l_ns: u16,
    pub reset_us: u16,
    pub pin_data: i8,
}

impl LedStripRmtConfig {
    pub const fn new(pin_data: i8) -> Self {
        Self {
            pin_data,
            ..Self::DEFAULT
        }
    }

    pub const DEFAULT: Self = Self {
        frequency: 10_000_000,
        t0h_ns: 300,
        t0l_ns: 900,
        t1h_ns: 900,
        t1l_ns: 300,
        reset_us: 280,
        pin_data: -1,
    };

    const fn to_raw(self) -> m5unified_sys::m5u_led_strip_rmt_config_t {
        m5unified_sys::m5u_led_strip_rmt_config_t {
            frequency: self.frequency,
            t0h_ns: self.t0h_ns,
            t0l_ns: self.t0l_ns,
            t1h_ns: self.t1h_ns,
            t1l_ns: self.t1l_ns,
            reset_us: self.reset_us,
            pin_data: self.pin_data,
        }
    }
}

impl Default for LedStripRmtConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}
