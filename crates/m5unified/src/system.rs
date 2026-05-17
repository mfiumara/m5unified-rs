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

    pub fn set_log_display_index(&mut self, index: usize) {
        unsafe { m5unified_sys::m5u_set_log_display_index(index) }
    }

    pub fn set_log_display_type(&mut self, kind: DisplayKind) {
        unsafe { m5unified_sys::m5u_set_log_display_type(kind.raw() as c_int) }
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Board {
    Unknown,
    Raw(i32),
}

impl Board {
    pub const fn from_raw(raw: i32) -> Self {
        if raw == 0 {
            Self::Unknown
        } else {
            Self::Raw(raw)
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::Unknown => 0,
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
