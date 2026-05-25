//! Board-level helpers for pins, display selection, timing, and identity.
//!
//! These methods live on [`crate::M5Unified`] and model top-level M5Unified
//! functions such as board detection, pin lookup, display selection, and touch
//! button sizing.

use core::ffi::c_int;

use crate::{DisplayKind, M5Unified};

impl M5Unified {
    pub fn board(&self) -> Board {
        Board::from_raw(unsafe { m5unified_sys::m5u_get_board() as i32 })
    }

    pub fn get_pin(&self, name: PinName) -> Option<u8> {
        let pin = unsafe { m5unified_sys::m5u_get_pin(name.raw()) };
        (pin >= 0).then_some(pin as u8)
    }

    pub fn set_primary_display(&mut self, index: usize) -> bool {
        unsafe { m5unified_sys::m5u_set_primary_display_index(index) }
    }

    pub fn set_primary_display_type(&mut self, kind: DisplayKind) -> bool {
        unsafe { m5unified_sys::m5u_set_primary_display_type(kind.raw() as c_int) }
    }

    pub fn set_primary_display_types(&mut self, kinds: &[DisplayKind]) -> bool {
        let kinds = raw_display_kinds(kinds);
        unsafe { m5unified_sys::m5u_set_primary_display_types(kinds.as_ptr(), kinds.len()) }
    }

    pub fn set_log_display_index(&mut self, index: usize) {
        unsafe { m5unified_sys::m5u_set_log_display_index(index) }
    }

    pub fn set_log_display_type(&mut self, kind: DisplayKind) {
        unsafe { m5unified_sys::m5u_set_log_display_type(kind.raw() as c_int) }
    }

    pub fn set_log_display_types(&mut self, kinds: &[DisplayKind]) {
        let kinds = raw_display_kinds(kinds);
        unsafe { m5unified_sys::m5u_set_log_display_types(kinds.as_ptr(), kinds.len()) }
    }

    pub fn set_touch_button_height(&mut self, pixel: u16) {
        unsafe { m5unified_sys::m5u_set_touch_button_height(pixel) }
    }

    pub fn set_touch_button_height_by_ratio(&mut self, ratio: u8) {
        unsafe { m5unified_sys::m5u_set_touch_button_height_by_ratio(ratio) }
    }

    pub fn touch_button_height(&self) -> u16 {
        unsafe { m5unified_sys::m5u_get_touch_button_height() }
    }

    pub fn millis(&self) -> u32 {
        unsafe { m5unified_sys::m5u_millis() }
    }

    pub fn micros(&self) -> u32 {
        unsafe { m5unified_sys::m5u_micros() }
    }

    pub fn update_msec(&self) -> u32 {
        unsafe { m5unified_sys::m5u_get_update_msec() }
    }
}

pub(crate) fn raw_display_kinds(kinds: &[DisplayKind]) -> Vec<c_int> {
    kinds.iter().map(|kind| kind.raw() as c_int).collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HeapCaps(u32);

impl HeapCaps {
    pub const EIGHT_BIT: Self = Self(1 << 2);
    pub const DMA: Self = Self(1 << 3);
    pub const SPIRAM: Self = Self(1 << 10);
    pub const INTERNAL: Self = Self(1 << 11);
    pub const DEFAULT_CAPS: Self = Self(1 << 12);

    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn internal_8bit() -> Self {
        Self::INTERNAL.union(Self::EIGHT_BIT)
    }

    pub const fn psram_8bit() -> Self {
        Self::SPIRAM.union(Self::EIGHT_BIT)
    }

    pub const fn dma_8bit() -> Self {
        Self::DMA.union(Self::EIGHT_BIT)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HeapStats {
    pub free_bytes: usize,
    pub largest_free_block: usize,
}

pub fn heap_stats(caps: HeapCaps) -> HeapStats {
    let bits = caps.bits();
    HeapStats {
        free_bytes: unsafe { m5unified_sys::m5u_heap_get_free_size(bits) },
        largest_free_block: unsafe { m5unified_sys::m5u_heap_get_largest_free_block(bits) },
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Board {
    Unknown,
    M5Stack,
    M5StackCore2,
    M5StickC,
    M5StickCPlus,
    M5StickCPlus2,
    M5StackCoreInk,
    M5Paper,
    M5Tough,
    M5Station,
    M5StackCoreS3,
    M5AtomS3,
    M5Dial,
    M5DinMeter,
    M5Cardputer,
    M5AirQ,
    M5VAMeter,
    M5StackCoreS3SE,
    M5AtomS3R,
    M5PaperS3,
    M5CoreMP135,
    M5StampPLC,
    M5Tab5,
    ArduinoNessoN1,
    M5CardputerAdv,
    M5UnitC6L,
    M5StickS3,
    M5AtomLite,
    M5AtomPsram,
    M5AtomU,
    M5Camera,
    M5TimerCam,
    M5StampPico,
    M5StampC3,
    M5StampC3U,
    M5StampS3,
    M5AtomS3Lite,
    M5AtomS3U,
    M5Capsule,
    M5NanoC6,
    M5AtomMatrix,
    M5AtomEcho,
    M5AtomS3RExt,
    M5AtomS3RCam,
    M5AtomEchoS3R,
    M5PowerHub,
    M5DualKey,
    M5UnitPoEP4,
    M5AtomDisplay,
    M5UnitLcd,
    M5UnitOled,
    M5UnitMiniOled,
    M5UnitGlass,
    M5UnitGlass2,
    M5UnitRca,
    M5ModuleDisplay,
    M5ModuleRca,
    FrameBuffer,
    Raw(i32),
}

impl Board {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Unknown,
            1 => Self::M5Stack,
            2 => Self::M5StackCore2,
            3 => Self::M5StickC,
            4 => Self::M5StickCPlus,
            5 => Self::M5StickCPlus2,
            6 => Self::M5StackCoreInk,
            7 => Self::M5Paper,
            8 => Self::M5Tough,
            9 => Self::M5Station,
            10 => Self::M5StackCoreS3,
            11 => Self::M5AtomS3,
            12 => Self::M5Dial,
            13 => Self::M5DinMeter,
            14 => Self::M5Cardputer,
            15 => Self::M5AirQ,
            16 => Self::M5VAMeter,
            17 => Self::M5StackCoreS3SE,
            18 => Self::M5AtomS3R,
            19 => Self::M5PaperS3,
            20 => Self::M5CoreMP135,
            21 => Self::M5StampPLC,
            22 => Self::M5Tab5,
            23 => Self::ArduinoNessoN1,
            24 => Self::M5CardputerAdv,
            25 => Self::M5UnitC6L,
            26 => Self::M5StickS3,
            128 => Self::M5AtomLite,
            129 => Self::M5AtomPsram,
            130 => Self::M5AtomU,
            131 => Self::M5Camera,
            132 => Self::M5TimerCam,
            133 => Self::M5StampPico,
            134 => Self::M5StampC3,
            135 => Self::M5StampC3U,
            136 => Self::M5StampS3,
            137 => Self::M5AtomS3Lite,
            138 => Self::M5AtomS3U,
            139 => Self::M5Capsule,
            140 => Self::M5NanoC6,
            141 => Self::M5AtomMatrix,
            142 => Self::M5AtomEcho,
            143 => Self::M5AtomS3RExt,
            144 => Self::M5AtomS3RCam,
            145 => Self::M5AtomEchoS3R,
            146 => Self::M5PowerHub,
            147 => Self::M5DualKey,
            148 => Self::M5UnitPoEP4,
            192 => Self::M5AtomDisplay,
            193 => Self::M5UnitLcd,
            194 => Self::M5UnitOled,
            195 => Self::M5UnitMiniOled,
            196 => Self::M5UnitGlass,
            197 => Self::M5UnitGlass2,
            198 => Self::M5UnitRca,
            199 => Self::M5ModuleDisplay,
            200 => Self::M5ModuleRca,
            512 => Self::FrameBuffer,
            raw => Self::Raw(raw),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::M5Stack => 1,
            Self::M5StackCore2 => 2,
            Self::M5StickC => 3,
            Self::M5StickCPlus => 4,
            Self::M5StickCPlus2 => 5,
            Self::M5StackCoreInk => 6,
            Self::M5Paper => 7,
            Self::M5Tough => 8,
            Self::M5Station => 9,
            Self::M5StackCoreS3 => 10,
            Self::M5AtomS3 => 11,
            Self::M5Dial => 12,
            Self::M5DinMeter => 13,
            Self::M5Cardputer => 14,
            Self::M5AirQ => 15,
            Self::M5VAMeter => 16,
            Self::M5StackCoreS3SE => 17,
            Self::M5AtomS3R => 18,
            Self::M5PaperS3 => 19,
            Self::M5CoreMP135 => 20,
            Self::M5StampPLC => 21,
            Self::M5Tab5 => 22,
            Self::ArduinoNessoN1 => 23,
            Self::M5CardputerAdv => 24,
            Self::M5UnitC6L => 25,
            Self::M5StickS3 => 26,
            Self::M5AtomLite => 128,
            Self::M5AtomPsram => 129,
            Self::M5AtomU => 130,
            Self::M5Camera => 131,
            Self::M5TimerCam => 132,
            Self::M5StampPico => 133,
            Self::M5StampC3 => 134,
            Self::M5StampC3U => 135,
            Self::M5StampS3 => 136,
            Self::M5AtomS3Lite => 137,
            Self::M5AtomS3U => 138,
            Self::M5Capsule => 139,
            Self::M5NanoC6 => 140,
            Self::M5AtomMatrix => 141,
            Self::M5AtomEcho => 142,
            Self::M5AtomS3RExt => 143,
            Self::M5AtomS3RCam => 144,
            Self::M5AtomEchoS3R => 145,
            Self::M5PowerHub => 146,
            Self::M5DualKey => 147,
            Self::M5UnitPoEP4 => 148,
            Self::M5AtomDisplay => 192,
            Self::M5UnitLcd => 193,
            Self::M5UnitOled => 194,
            Self::M5UnitMiniOled => 195,
            Self::M5UnitGlass => 196,
            Self::M5UnitGlass2 => 197,
            Self::M5UnitRca => 198,
            Self::M5ModuleDisplay => 199,
            Self::M5ModuleRca => 200,
            Self::FrameBuffer => 512,
            Self::Raw(raw) => raw,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PinName {
    InI2cScl,
    InI2cSda,
    PortAPin1,
    PortAPin2,
    PortBPin1,
    PortBPin2,
    PortCPin1,
    PortCPin2,
    PortDPin1,
    PortDPin2,
    PortEPin1,
    PortEPin2,
    SdSpiSclk,
    SdSpiCopi,
    SdSpiCipo,
    SdSpiCs,
    RgbLed,
    PowerHold,
    MBusPin(u8),
    Raw(i32),
}

impl PinName {
    pub const PORT_A_SCL: Self = Self::PortAPin1;
    pub const EX_I2C_SCL: Self = Self::PortAPin1;
    pub const PORT_A_SDA: Self = Self::PortAPin2;
    pub const EX_I2C_SDA: Self = Self::PortAPin2;
    pub const PORT_B_IN: Self = Self::PortBPin1;
    pub const PORT_B_OUT: Self = Self::PortBPin2;
    pub const PORT_C_RXD: Self = Self::PortCPin1;
    pub const PORT_C_TXD: Self = Self::PortCPin2;
    pub const PORT_D_RXD: Self = Self::PortDPin1;
    pub const PORT_B2_PIN1: Self = Self::PortDPin1;
    pub const PORT_D_TXD: Self = Self::PortDPin2;
    pub const PORT_B2_PIN2: Self = Self::PortDPin2;
    pub const PORT_E_RXD: Self = Self::PortEPin1;
    pub const PORT_C2_PIN1: Self = Self::PortEPin1;
    pub const PORT_E_TXD: Self = Self::PortEPin2;
    pub const PORT_C2_PIN2: Self = Self::PortEPin2;
    pub const SD_SPI_MOSI: Self = Self::SdSpiCopi;
    pub const SD_SPI_MISO: Self = Self::SdSpiCipo;
    pub const SD_SPI_SS: Self = Self::SdSpiCs;

    pub(crate) fn raw(self) -> c_int {
        match self {
            Self::InI2cScl => 0,
            Self::InI2cSda => 1,
            Self::PortAPin1 => 2,
            Self::PortAPin2 => 3,
            Self::PortBPin1 => 4,
            Self::PortBPin2 => 5,
            Self::PortCPin1 => 6,
            Self::PortCPin2 => 7,
            Self::PortDPin1 => 8,
            Self::PortDPin2 => 9,
            Self::PortEPin1 => 10,
            Self::PortEPin2 => 11,
            Self::SdSpiSclk => 12,
            Self::SdSpiCopi => 13,
            Self::SdSpiCipo => 14,
            Self::SdSpiCs => 15,
            Self::RgbLed => 16,
            Self::PowerHold => 17,
            Self::MBusPin(pin) if (1..=30).contains(&pin) => 17 + c_int::from(pin),
            Self::MBusPin(_) => -1,
            Self::Raw(raw) => raw,
        }
    }
}
