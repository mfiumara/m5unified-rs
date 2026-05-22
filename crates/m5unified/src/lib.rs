//! Safe Rust wrapper for a small M5Unified C ABI surface.
//!
//! The API is intentionally shaped around M5Unified's common examples while
//! keeping Rust call sites safe and host-checkable. Hardware calls are provided
//! by `m5unified-sys`; on non-ESP-IDF targets that crate supplies no-op stubs so
//! examples compile in CI.

use core::ffi::{c_int, c_void};
use std::ffi::{CStr, CString};
use std::fmt;
use std::ptr::NonNull;

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

/// Common RGB888 color constants for board LEDs.
pub mod rgb {
    pub const BLACK: u32 = 0x000000;
    pub const RED: u32 = 0xFF0000;
    pub const GREEN: u32 = 0x00FF00;
    pub const BLUE: u32 = 0x0000FF;
    pub const WHITE: u32 = 0xFFFFFF;
    pub const YELLOW: u32 = 0xFFFF00;
    pub const CYAN: u32 = 0x00FFFF;
    pub const MAGENTA: u32 = 0xFF00FF;
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct RgbColor(pub u32);

impl RgbColor {
    pub const BLACK: Self = Self(rgb::BLACK);
    pub const RED: Self = Self(rgb::RED);
    pub const GREEN: Self = Self(rgb::GREEN);
    pub const BLUE: Self = Self(rgb::BLUE);
    pub const WHITE: Self = Self(rgb::WHITE);
    pub const YELLOW: Self = Self(rgb::YELLOW);
    pub const CYAN: Self = Self(rgb::CYAN);
    pub const MAGENTA: Self = Self(rgb::MAGENTA);

    pub const fn new(raw: u32) -> Self {
        Self(raw & 0x00ff_ffff)
    }

    pub const fn from_rgb888(red: u8, green: u8, blue: u8) -> Self {
        Self(((red as u32) << 16) | ((green as u32) << 8) | blue as u32)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn red(self) -> u8 {
        ((self.0 >> 16) & 0xff) as u8
    }

    pub const fn green(self) -> u8 {
        ((self.0 >> 8) & 0xff) as u8
    }

    pub const fn blue(self) -> u8 {
        (self.0 & 0xff) as u8
    }

    pub const fn rgb888_components(self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }
}

impl From<u32> for RgbColor {
    fn from(raw: u32) -> Self {
        Self::new(raw)
    }
}

impl From<RgbColor> for u32 {
    fn from(color: RgbColor) -> Self {
        color.raw()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PinName {
    InternalI2cScl,
    InternalI2cSda,
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
    SdSpiMosi,
    SdSpiMiso,
    SdSpiCs,
    RgbLed,
    PowerHold,
    MBusPin(u8),
}

impl PinName {
    pub const PORT_A_SCL: Self = Self::PortAPin1;
    pub const EXTERNAL_I2C_SCL: Self = Self::PortAPin1;
    pub const PORT_A_SDA: Self = Self::PortAPin2;
    pub const EXTERNAL_I2C_SDA: Self = Self::PortAPin2;
    pub const PORT_B_IN: Self = Self::PortBPin1;
    pub const PORT_B_OUT: Self = Self::PortBPin2;
    pub const PORT_C_RXD: Self = Self::PortCPin1;
    pub const PORT_C_TXD: Self = Self::PortCPin2;
    pub const PORT_D_RXD: Self = Self::PortDPin1;
    pub const PORT_D_TXD: Self = Self::PortDPin2;
    pub const PORT_E_RXD: Self = Self::PortEPin1;
    pub const PORT_E_TXD: Self = Self::PortEPin2;
    pub const PORT_B2_PIN1: Self = Self::PortDPin1;
    pub const PORT_B2_PIN2: Self = Self::PortDPin2;
    pub const PORT_C2_PIN1: Self = Self::PortEPin1;
    pub const PORT_C2_PIN2: Self = Self::PortEPin2;
    pub const SD_SPI_SCLK: Self = Self::SdSpiSclk;
    pub const SD_SPI_COPI: Self = Self::SdSpiMosi;
    pub const SD_SPI_CIPO: Self = Self::SdSpiMiso;
    pub const SD_SPI_SS: Self = Self::SdSpiCs;

    fn raw(self) -> Option<i32> {
        Some(match self {
            Self::InternalI2cScl => 0,
            Self::InternalI2cSda => 1,
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
            Self::SdSpiMosi => 13,
            Self::SdSpiMiso => 14,
            Self::SdSpiCs => 15,
            Self::RgbLed => 16,
            Self::PowerHold => 17,
            Self::MBusPin(pin @ 1..=30) => 17 + pin as i32,
            Self::MBusPin(_) => return None,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoardKind {
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

impl BoardKind {
    pub fn from_raw(raw: i32) -> Self {
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
            value => Self::Raw(value),
        }
    }

    pub fn raw(self) -> i32 {
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
            Self::Raw(value) => value,
        }
    }

    pub fn is_cardputer(self) -> bool {
        self == Self::M5Cardputer
    }
}

/// Errors returned by the high-level wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// M5Unified initialization failed.
    BeginFailed,
    /// The provided string contained an interior NUL byte.
    InvalidString,
    /// The provided value is outside the range accepted by M5Unified.
    InvalidValue(&'static str),
    /// Requested operation is not available on this board/build.
    Unavailable(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BeginFailed => f.write_str("M5Unified initialization failed"),
            Self::InvalidString => f.write_str("string contains an interior NUL byte"),
            Self::InvalidValue(value) => write!(f, "invalid M5Unified value: {value}"),
            Self::Unavailable(feature) => write!(f, "M5Unified feature unavailable: {feature}"),
        }
    }
}

impl std::error::Error for Error {}

/// Startup options passed to `M5.begin(cfg)`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct M5Config {
    pub serial_baudrate: u32,
    pub clear_display: bool,
    pub output_power: bool,
    pub pmic_button: bool,
    pub internal_imu: bool,
    pub internal_rtc: bool,
    pub internal_mic: bool,
    pub internal_speaker: bool,
    pub external_imu: bool,
    pub external_rtc: bool,
    pub disable_rtc_irq: bool,
    pub led_brightness: u8,
    pub external_speaker: ExternalSpeakerConfig,
    pub external_display: ExternalDisplayConfig,
}

impl M5Config {
    pub const fn with_serial_baudrate(mut self, serial_baudrate: u32) -> Self {
        self.serial_baudrate = serial_baudrate;
        self
    }

    pub const fn with_clear_display(mut self, clear_display: bool) -> Self {
        self.clear_display = clear_display;
        self
    }

    pub const fn with_output_power(mut self, output_power: bool) -> Self {
        self.output_power = output_power;
        self
    }

    pub const fn with_pmic_button(mut self, pmic_button: bool) -> Self {
        self.pmic_button = pmic_button;
        self
    }

    pub const fn with_internal_imu(mut self, internal_imu: bool) -> Self {
        self.internal_imu = internal_imu;
        self
    }

    pub const fn with_internal_rtc(mut self, internal_rtc: bool) -> Self {
        self.internal_rtc = internal_rtc;
        self
    }

    pub const fn with_internal_mic(mut self, internal_mic: bool) -> Self {
        self.internal_mic = internal_mic;
        self
    }

    pub const fn with_internal_speaker(mut self, internal_speaker: bool) -> Self {
        self.internal_speaker = internal_speaker;
        self
    }

    pub const fn with_external_imu(mut self, external_imu: bool) -> Self {
        self.external_imu = external_imu;
        self
    }

    pub const fn with_external_rtc(mut self, external_rtc: bool) -> Self {
        self.external_rtc = external_rtc;
        self
    }

    pub const fn with_disable_rtc_irq(mut self, disable_rtc_irq: bool) -> Self {
        self.disable_rtc_irq = disable_rtc_irq;
        self
    }

    pub const fn with_led_brightness(mut self, led_brightness: u8) -> Self {
        self.led_brightness = led_brightness;
        self
    }

    pub const fn with_external_speaker(mut self, external_speaker: ExternalSpeakerConfig) -> Self {
        self.external_speaker = external_speaker;
        self
    }

    pub const fn with_external_display(mut self, external_display: ExternalDisplayConfig) -> Self {
        self.external_display = external_display;
        self
    }
}

impl Default for M5Config {
    fn default() -> Self {
        Self {
            serial_baudrate: 115_200,
            clear_display: true,
            output_power: true,
            pmic_button: true,
            internal_imu: true,
            internal_rtc: true,
            internal_mic: true,
            internal_speaker: true,
            external_imu: false,
            external_rtc: false,
            disable_rtc_irq: true,
            led_brightness: 0,
            external_speaker: ExternalSpeakerConfig::default(),
            external_display: ExternalDisplayConfig::default(),
        }
    }
}

impl From<M5Config> for m5unified_sys::m5u_config_t {
    fn from(config: M5Config) -> Self {
        Self {
            serial_baudrate: config.serial_baudrate,
            clear_display: config.clear_display,
            output_power: config.output_power,
            pmic_button: config.pmic_button,
            internal_imu: config.internal_imu,
            internal_rtc: config.internal_rtc,
            internal_mic: config.internal_mic,
            internal_spk: config.internal_speaker,
            external_imu: config.external_imu,
            external_rtc: config.external_rtc,
            disable_rtc_irq: config.disable_rtc_irq,
            led_brightness: config.led_brightness,
            external_speaker_value: config.external_speaker.bits(),
            external_display_value: config.external_display.bits(),
        }
    }
}

/// Bit-wise external speaker selection for `M5Config`.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ExternalSpeakerConfig {
    bits: u16,
}

impl ExternalSpeakerConfig {
    pub const NONE: Self = Self { bits: 0 };
    pub const ALL: Self = Self { bits: 0x003f };
    pub const MODULE_DISPLAY: Self = Self { bits: 1 << 0 };
    pub const MODULE_RCA: Self = Self { bits: 1 << 1 };
    pub const HAT_SPK: Self = Self { bits: 1 << 2 };
    pub const ATOMIC_SPK: Self = Self { bits: 1 << 3 };
    pub const HAT_SPK2: Self = Self { bits: 1 << 4 };
    pub const ATOMIC_ECHO: Self = Self { bits: 1 << 5 };

    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }

    pub const fn bits(self) -> u16 {
        self.bits
    }

    pub const fn with(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    pub const fn contains(self, other: Self) -> bool {
        self.bits & other.bits == other.bits
    }

    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    pub const fn any(self) -> bool {
        self.bits != 0
    }

    pub const fn without(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl core::ops::BitOr for ExternalSpeakerConfig {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.with(rhs)
    }
}

impl core::ops::BitOrAssign for ExternalSpeakerConfig {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.with(rhs);
    }
}

impl core::ops::BitAnd for ExternalSpeakerConfig {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl core::ops::BitAndAssign for ExternalSpeakerConfig {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl core::ops::Sub for ExternalSpeakerConfig {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.without(rhs)
    }
}

impl core::ops::SubAssign for ExternalSpeakerConfig {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.without(rhs);
    }
}

/// Bit-wise external display selection for `M5Config`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExternalDisplayConfig {
    bits: u16,
}

impl Default for ExternalDisplayConfig {
    fn default() -> Self {
        Self::ALL
    }
}

impl ExternalDisplayConfig {
    pub const MODULE_DISPLAY: Self = Self { bits: 1 << 0 };
    pub const ATOM_DISPLAY: Self = Self { bits: 1 << 1 };
    pub const UNIT_OLED: Self = Self { bits: 1 << 2 };
    pub const UNIT_MINI_OLED: Self = Self { bits: 1 << 3 };
    pub const UNIT_LCD: Self = Self { bits: 1 << 4 };
    pub const UNIT_GLASS: Self = Self { bits: 1 << 5 };
    pub const UNIT_GLASS2: Self = Self { bits: 1 << 6 };
    pub const UNIT_RCA: Self = Self { bits: 1 << 7 };
    pub const MODULE_RCA: Self = Self { bits: 1 << 8 };
    pub const NONE: Self = Self { bits: 0 };
    pub const ALL: Self = Self { bits: 0xffff };

    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }

    pub const fn bits(self) -> u16 {
        self.bits
    }

    pub const fn with(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    pub const fn contains(self, other: Self) -> bool {
        self.bits & other.bits == other.bits
    }

    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    pub const fn any(self) -> bool {
        self.bits != 0
    }

    pub const fn without(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl core::ops::BitOr for ExternalDisplayConfig {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.with(rhs)
    }
}

impl core::ops::BitOrAssign for ExternalDisplayConfig {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.with(rhs);
    }
}

impl core::ops::BitAnd for ExternalDisplayConfig {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl core::ops::BitAndAssign for ExternalDisplayConfig {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl core::ops::Sub for ExternalDisplayConfig {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.without(rhs)
    }
}

impl core::ops::SubAssign for ExternalDisplayConfig {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.without(rhs);
    }
}

/// Top-level handle for M5Unified-backed board features.
#[derive(Debug)]
pub struct M5Unified {
    pub display: Display,
    pub buttons: Buttons,
    pub mic: Mic,
    pub speaker: Speaker,
    pub imu: Imu,
    pub touch: Touch,
    pub rtc: Rtc,
    pub power: Power,
    pub led: Led,
    pub log: Log,
}

/// Top-level handle for M5Cardputer-specific hardware.
///
/// Cardputer uses the same display/audio/power devices exposed by M5Unified,
/// plus a front keyboard provided by the M5Cardputer support library.
#[derive(Debug)]
pub struct Cardputer {
    pub display: Display,
    pub button_a: Button,
    pub keyboard: CardputerKeyboard,
    pub sd: CardputerSd,
    pub ir: CardputerIr,
    pub grove: CardputerGrove,
    pub spi: CardputerSpi,
    pub mic: Mic,
    pub speaker: Speaker,
    pub imu: Imu,
    pub power: Power,
    pub led: Led,
    pub log: Log,
}

impl M5Unified {
    /// Initialize M5Unified and return a board handle.
    pub fn begin() -> Result<Self, Error> {
        Self::finish_begin(unsafe { m5unified_sys::m5u_begin() })
    }

    /// Initialize M5Unified with explicit startup options.
    pub fn begin_with_config(config: M5Config) -> Result<Self, Error> {
        let raw = m5unified_sys::m5u_config_t::from(config);
        Self::finish_begin(unsafe { m5unified_sys::m5u_begin_with_config(&raw) })
    }

    fn finish_begin(ok: bool) -> Result<Self, Error> {
        if !ok {
            return Err(Error::BeginFailed);
        }

        Ok(Self {
            display: Display,
            buttons: Buttons,
            mic: Mic,
            speaker: Speaker,
            imu: Imu,
            touch: Touch,
            rtc: Rtc,
            power: Power,
            led: Led,
            log: Log,
        })
    }

    /// Poll/update M5Unified internals, including button edge state.
    pub fn update(&mut self) {
        unsafe { m5unified_sys::m5u_update() }
    }

    /// Delay execution. On host builds this is currently a no-op.
    pub fn delay_ms(&self, ms: u32) {
        unsafe { m5unified_sys::m5u_delay_ms(ms) }
    }

    pub fn millis(&self) -> u32 {
        millis()
    }

    pub fn micros(&self) -> u32 {
        micros()
    }

    pub fn update_msec(&self) -> u32 {
        update_msec()
    }

    pub fn set_touch_button_height(&mut self, pixel: u16) {
        set_touch_button_height(pixel)
    }

    pub fn set_touch_button_height_by_ratio(&mut self, ratio: u8) {
        set_touch_button_height_by_ratio(ratio)
    }

    pub fn touch_button_height(&self) -> u16 {
        touch_button_height()
    }

    pub fn pin(&self, name: PinName) -> Option<i32> {
        get_pin(name)
    }

    pub fn board(&self) -> BoardKind {
        board()
    }

    pub fn set_primary_display(&mut self, index: usize) -> bool {
        set_primary_display(index)
    }

    pub fn try_set_primary_display(&mut self, index: usize) -> Result<(), Error> {
        try_set_primary_display(index)
    }

    pub fn set_primary_display_kind(&mut self, kind: DisplayKind) -> bool {
        set_primary_display_kind(kind)
    }

    pub fn try_set_primary_display_kind(&mut self, kind: DisplayKind) -> Result<(), Error> {
        try_set_primary_display_kind(kind)
    }

    pub fn set_log_display(&mut self, index: usize) -> bool {
        set_log_display(index)
    }

    pub fn try_set_log_display(&mut self, index: usize) -> Result<(), Error> {
        try_set_log_display(index)
    }

    pub fn canvas(&mut self) -> Option<Canvas> {
        Canvas::for_display()
    }

    pub fn stackchan_servos(
        &mut self,
        config: StackChanServoConfig,
    ) -> Result<StackChanServos, Error> {
        StackChanServos::attach(config)
    }

    pub fn stackchan_pwm_servos(
        &mut self,
        config: StackChanPwmServoConfig,
    ) -> Result<StackChanPwmServos, Error> {
        StackChanPwmServos::attach(config)
    }
}

impl Cardputer {
    /// Initialize M5Cardputer with keyboard scanning enabled.
    pub fn begin() -> Result<Self, Error> {
        Self::begin_with_keyboard(true)
    }

    /// Initialize M5Cardputer with explicit startup options and keyboard scanning enabled.
    pub fn begin_with_config(config: M5Config) -> Result<Self, Error> {
        Self::begin_with_config_and_keyboard(config, true)
    }

    /// Initialize M5Cardputer and choose whether to enable keyboard scanning.
    pub fn begin_with_keyboard(enable_keyboard: bool) -> Result<Self, Error> {
        Self::finish_begin(unsafe { m5unified_sys::m5u_cardputer_begin(enable_keyboard) })
    }

    /// Initialize M5Cardputer with explicit startup options and keyboard scanning control.
    pub fn begin_with_config_and_keyboard(
        config: M5Config,
        enable_keyboard: bool,
    ) -> Result<Self, Error> {
        let raw = m5unified_sys::m5u_config_t::from(config);
        Self::finish_begin(unsafe {
            m5unified_sys::m5u_cardputer_begin_with_config(&raw, enable_keyboard)
        })
    }

    fn finish_begin(ok: bool) -> Result<Self, Error> {
        if !ok {
            return Err(Error::BeginFailed);
        }

        Ok(Self {
            display: Display,
            button_a: Button { id: ButtonId::A },
            keyboard: CardputerKeyboard,
            sd: CardputerSd,
            ir: CardputerIr,
            grove: CardputerGrove,
            spi: CardputerSpi,
            mic: Mic,
            speaker: Speaker,
            imu: Imu,
            power: Power,
            led: Led,
            log: Log,
        })
    }

    /// Poll/update Cardputer internals, including keyboard and button state.
    pub fn update(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_update() }
    }

    /// Delay execution. On host builds this is currently a no-op.
    pub fn delay_ms(&self, ms: u32) {
        unsafe { m5unified_sys::m5u_delay_ms(ms) }
    }

    pub fn millis(&self) -> u32 {
        millis()
    }

    pub fn micros(&self) -> u32 {
        micros()
    }

    pub fn update_msec(&self) -> u32 {
        update_msec()
    }

    pub fn set_touch_button_height(&mut self, pixel: u16) {
        set_touch_button_height(pixel)
    }

    pub fn set_touch_button_height_by_ratio(&mut self, ratio: u8) {
        set_touch_button_height_by_ratio(ratio)
    }

    pub fn touch_button_height(&self) -> u16 {
        touch_button_height()
    }

    pub fn pin(&self, name: PinName) -> Option<i32> {
        get_pin(name)
    }

    pub fn board(&self) -> BoardKind {
        board()
    }

    pub fn set_primary_display(&mut self, index: usize) -> bool {
        set_primary_display(index)
    }

    pub fn try_set_primary_display(&mut self, index: usize) -> Result<(), Error> {
        try_set_primary_display(index)
    }

    pub fn set_primary_display_kind(&mut self, kind: DisplayKind) -> bool {
        set_primary_display_kind(kind)
    }

    pub fn try_set_primary_display_kind(&mut self, kind: DisplayKind) -> Result<(), Error> {
        try_set_primary_display_kind(kind)
    }

    pub fn set_log_display(&mut self, index: usize) -> bool {
        set_log_display(index)
    }

    pub fn try_set_log_display(&mut self, index: usize) -> Result<(), Error> {
        try_set_log_display(index)
    }

    pub fn canvas(&mut self) -> Option<Canvas> {
        Canvas::for_cardputer_display()
    }

    pub fn display_count(&self) -> usize {
        display_count()
    }

    pub fn display(&self, index: usize) -> Option<DisplayRef> {
        display_ref(index)
    }

    pub fn try_display(&self, index: usize) -> Result<DisplayRef, Error> {
        try_display_ref(index)
    }

    pub fn display_index(&self, kind: DisplayKind) -> Option<usize> {
        display_index(kind)
    }

    pub fn display_by_kind(&self, kind: DisplayKind) -> Option<DisplayRef> {
        display_ref_by_kind(kind)
    }

    pub fn try_display_by_kind(&self, kind: DisplayKind) -> Result<DisplayRef, Error> {
        try_display_ref_by_kind(kind)
    }
}

pub fn get_pin(name: PinName) -> Option<i32> {
    let raw_name = name.raw()?;
    let pin = unsafe { m5unified_sys::m5u_get_pin(raw_name as c_int) };
    (pin >= 0).then_some(pin as i32)
}

pub fn board() -> BoardKind {
    BoardKind::from_raw(unsafe { m5unified_sys::m5u_get_board() as i32 })
}

pub fn millis() -> u32 {
    unsafe { m5unified_sys::m5u_millis() }
}

pub fn micros() -> u32 {
    unsafe { m5unified_sys::m5u_micros() }
}

pub fn update_msec() -> u32 {
    unsafe { m5unified_sys::m5u_get_update_msec() }
}

pub fn set_touch_button_height(pixel: u16) {
    unsafe { m5unified_sys::m5u_set_touch_button_height(pixel) }
}

pub fn set_touch_button_height_by_ratio(ratio: u8) {
    unsafe { m5unified_sys::m5u_set_touch_button_height_by_ratio(ratio) }
}

pub fn touch_button_height() -> u16 {
    unsafe { m5unified_sys::m5u_get_touch_button_height() }
}

fn set_primary_display(index: usize) -> bool {
    let Ok(index) = c_int::try_from(index) else {
        return false;
    };
    unsafe { m5unified_sys::m5u_display_set_primary(index) }
}

fn try_set_primary_display(index: usize) -> Result<(), Error> {
    c_int::try_from(index).map_err(|_| Error::InvalidValue("display index"))?;
    set_primary_display(index)
        .then_some(())
        .ok_or(Error::Unavailable("display"))
}

fn set_primary_display_kind(kind: DisplayKind) -> bool {
    unsafe { m5unified_sys::m5u_display_set_primary_kind(kind.raw() as c_int) }
}

fn try_set_primary_display_kind(kind: DisplayKind) -> Result<(), Error> {
    set_primary_display_kind(kind)
        .then_some(())
        .ok_or(Error::Unavailable("display kind"))
}

fn set_log_display(index: usize) -> bool {
    let Ok(index) = c_int::try_from(index) else {
        return false;
    };
    unsafe { m5unified_sys::m5u_set_log_display_index(index) }
    true
}

fn try_set_log_display(index: usize) -> Result<(), Error> {
    let index = c_int::try_from(index).map_err(|_| Error::InvalidValue("display index"))?;
    if index < 0 || index >= unsafe { m5unified_sys::m5u_display_count() } {
        return Err(Error::InvalidValue("display index"));
    }
    unsafe { m5unified_sys::m5u_set_log_display_index(index) }
    Ok(())
}

#[derive(Debug, Copy, Clone)]
pub struct CardputerKeyboard;

#[derive(Debug, Copy, Clone)]
pub struct CardputerSd;

#[derive(Debug, Copy, Clone)]
pub struct CardputerIr;

#[derive(Debug, Copy, Clone)]
pub struct CardputerGrove;

#[derive(Debug, Copy, Clone)]
pub struct CardputerSpi;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PwmServoPins {
    pub pan: i32,
    pub tilt: i32,
}

impl PwmServoPins {
    /// CoreS3 Port A pins for an external PWM pan/tilt fallback build.
    ///
    /// Official Stack-chan CoreS3 hardware is not generic PWM on Port A. It
    /// uses StackChan-BSP Motion plus board power setup for its servos.
    pub const CORES3_PORT_A: Self = Self { pan: 2, tilt: 1 };

    pub const fn new(pan: i32, tilt: i32) -> Self {
        Self { pan, tilt }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PwmServoConfig {
    pub pin: i32,
    pub channel: u8,
    pub timer: u8,
    pub frequency_hz: u32,
    pub min_pulse_us: u16,
    pub max_pulse_us: u16,
    pub min_angle_tenths: i16,
    pub max_angle_tenths: i16,
    pub neutral_angle_tenths: i16,
}

impl PwmServoConfig {
    pub const DEFAULT_FREQUENCY_HZ: u32 = 50;
    pub const DEFAULT_MIN_PULSE_US: u16 = 500;
    pub const DEFAULT_MAX_PULSE_US: u16 = 2500;

    pub const fn new(
        pin: i32,
        channel: u8,
        timer: u8,
        min_angle_tenths: i16,
        max_angle_tenths: i16,
        neutral_angle_tenths: i16,
    ) -> Self {
        Self {
            pin,
            channel,
            timer,
            frequency_hz: Self::DEFAULT_FREQUENCY_HZ,
            min_pulse_us: Self::DEFAULT_MIN_PULSE_US,
            max_pulse_us: Self::DEFAULT_MAX_PULSE_US,
            min_angle_tenths,
            max_angle_tenths,
            neutral_angle_tenths,
        }
    }

    pub const fn with_pulse_range(mut self, min_pulse_us: u16, max_pulse_us: u16) -> Self {
        self.min_pulse_us = min_pulse_us;
        self.max_pulse_us = max_pulse_us;
        self
    }

    pub const fn with_frequency(mut self, frequency_hz: u32) -> Self {
        self.frequency_hz = frequency_hz;
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StackChanPwmServoConfig {
    pub pan: PwmServoConfig,
    pub tilt: PwmServoConfig,
}

impl StackChanPwmServoConfig {
    pub const PAN_MIN_TENTHS: i16 = StackChanMotionContract::X_MIN_TENTHS;
    pub const PAN_MAX_TENTHS: i16 = StackChanMotionContract::X_MAX_TENTHS;
    pub const PAN_NEUTRAL_TENTHS: i16 = StackChanMotionContract::X_NEUTRAL_TENTHS;
    pub const TILT_MIN_TENTHS: i16 = StackChanMotionContract::Y_MIN_TENTHS;
    pub const TILT_MAX_TENTHS: i16 = StackChanMotionContract::Y_MAX_TENTHS;
    pub const TILT_NEUTRAL_TENTHS: i16 = StackChanMotionContract::Y_NEUTRAL_TENTHS;

    pub const fn pwm_pins(pins: PwmServoPins) -> Self {
        Self {
            pan: PwmServoConfig::new(
                pins.pan,
                0,
                0,
                Self::PAN_MIN_TENTHS,
                Self::PAN_MAX_TENTHS,
                Self::PAN_NEUTRAL_TENTHS,
            ),
            tilt: PwmServoConfig::new(
                pins.tilt,
                1,
                0,
                Self::TILT_MIN_TENTHS,
                Self::TILT_MAX_TENTHS,
                Self::TILT_NEUTRAL_TENTHS,
            ),
        }
    }
}

impl Default for StackChanPwmServoConfig {
    fn default() -> Self {
        Self::pwm_pins(PwmServoPins::CORES3_PORT_A)
    }
}

/// Compatibility alias for earlier versions of the PWM fallback API.
///
/// Prefer `StackChanPwmServoConfig` for generic PWM builds. Official
/// Stack-chan CoreS3 hardware should use `StackChanBspMotion` or firmware that
/// exposes the same StackChan-BSP Motion contract.
pub type StackChanServoConfig = StackChanPwmServoConfig;

/// Stack-chan MCP/firmware motion limits.
///
/// The HTTP/MCP contract uses `x` yaw in -128..128 degrees, `y` pitch in 0..90
/// degrees, and `speed` in 0..100 percent. Official StackChan-BSP Motion sends
/// angles in 0.1 degree units and clamps physical pitch to 5..85 degrees.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StackChanMotionContract;

impl StackChanMotionContract {
    pub const X_MIN_DEGREES: i16 = -128;
    pub const X_MAX_DEGREES: i16 = 128;
    pub const X_NEUTRAL_DEGREES: i16 = 0;
    pub const Y_MIN_DEGREES: i16 = 0;
    pub const Y_MAX_DEGREES: i16 = 90;
    pub const Y_HARDWARE_MIN_DEGREES: i16 = 5;
    pub const Y_HARDWARE_MAX_DEGREES: i16 = 85;
    pub const Y_NEUTRAL_DEGREES: i16 = 45;
    pub const SPEED_MIN_PERCENT: u8 = 0;
    pub const SPEED_MAX_PERCENT: u8 = 100;

    pub const X_MIN_TENTHS: i16 = Self::X_MIN_DEGREES * 10;
    pub const X_MAX_TENTHS: i16 = Self::X_MAX_DEGREES * 10;
    pub const X_NEUTRAL_TENTHS: i16 = Self::X_NEUTRAL_DEGREES * 10;
    pub const Y_MIN_TENTHS: i16 = Self::Y_MIN_DEGREES * 10;
    pub const Y_MAX_TENTHS: i16 = Self::Y_MAX_DEGREES * 10;
    pub const Y_HARDWARE_MIN_TENTHS: i16 = Self::Y_HARDWARE_MIN_DEGREES * 10;
    pub const Y_HARDWARE_MAX_TENTHS: i16 = Self::Y_HARDWARE_MAX_DEGREES * 10;
    pub const Y_NEUTRAL_TENTHS: i16 = Self::Y_NEUTRAL_DEGREES * 10;

    pub const fn clamp_x_tenths(x_tenths: i16) -> i16 {
        if x_tenths < Self::X_MIN_TENTHS {
            Self::X_MIN_TENTHS
        } else if x_tenths > Self::X_MAX_TENTHS {
            Self::X_MAX_TENTHS
        } else {
            x_tenths
        }
    }

    pub const fn clamp_y_tenths(y_tenths: i16) -> i16 {
        if y_tenths < Self::Y_MIN_TENTHS {
            Self::Y_MIN_TENTHS
        } else if y_tenths > Self::Y_MAX_TENTHS {
            Self::Y_MAX_TENTHS
        } else {
            y_tenths
        }
    }

    pub const fn clamp_hardware_y_tenths(y_tenths: i16) -> i16 {
        if y_tenths < Self::Y_HARDWARE_MIN_TENTHS {
            Self::Y_HARDWARE_MIN_TENTHS
        } else if y_tenths > Self::Y_HARDWARE_MAX_TENTHS {
            Self::Y_HARDWARE_MAX_TENTHS
        } else {
            y_tenths
        }
    }

    pub const fn clamp_speed_percent(speed: u8) -> u8 {
        if speed > Self::SPEED_MAX_PERCENT {
            Self::SPEED_MAX_PERCENT
        } else {
            speed
        }
    }

    pub const fn speed_percent_to_bsp(speed: u8) -> u16 {
        Self::clamp_speed_percent(speed) as u16 * 10
    }

    pub fn clamp_x_degrees(x: f32) -> f32 {
        clamp_f32(x, Self::X_MIN_DEGREES as f32, Self::X_MAX_DEGREES as f32)
    }

    pub fn clamp_y_degrees(y: f32) -> f32 {
        clamp_f32(y, Self::Y_MIN_DEGREES as f32, Self::Y_MAX_DEGREES as f32)
    }

    pub const fn clamp_speed_i32(speed: i32) -> u8 {
        if speed < Self::SPEED_MIN_PERCENT as i32 {
            Self::SPEED_MIN_PERCENT
        } else if speed > Self::SPEED_MAX_PERCENT as i32 {
            Self::SPEED_MAX_PERCENT
        } else {
            speed as u8
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StackChanPose {
    pub pan_tenths: i16,
    pub tilt_tenths: i16,
}

impl StackChanPose {
    pub const NEUTRAL: Self = Self {
        pan_tenths: StackChanMotionContract::X_NEUTRAL_TENTHS,
        tilt_tenths: StackChanMotionContract::Y_NEUTRAL_TENTHS,
    };
    pub const LEFT: Self = Self {
        pan_tenths: 900,
        tilt_tenths: StackChanMotionContract::Y_NEUTRAL_TENTHS,
    };
    pub const RIGHT: Self = Self {
        pan_tenths: -900,
        tilt_tenths: StackChanMotionContract::Y_NEUTRAL_TENTHS,
    };
    pub const UP: Self = Self {
        pan_tenths: StackChanMotionContract::X_NEUTRAL_TENTHS,
        tilt_tenths: 900,
    };
    pub const DOWN: Self = Self {
        pan_tenths: StackChanMotionContract::X_NEUTRAL_TENTHS,
        tilt_tenths: 0,
    };

    pub const fn new(pan_tenths: i16, tilt_tenths: i16) -> Self {
        Self {
            pan_tenths,
            tilt_tenths,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StackChanMove {
    pub x_tenths: i16,
    pub y_tenths: i16,
    pub speed_percent: u8,
}

impl StackChanMove {
    pub const DEFAULT_SPEED_PERCENT: u8 = 50;

    pub fn from_mcp(x: f32, y: f32, speed: i32) -> Self {
        Self::from_tenths(
            degrees_to_tenths(StackChanMotionContract::clamp_x_degrees(x)),
            degrees_to_tenths(StackChanMotionContract::clamp_y_degrees(y)),
            StackChanMotionContract::clamp_speed_i32(speed),
        )
    }

    pub const fn new(x_degrees: i16, y_degrees: i16, speed_percent: u8) -> Self {
        Self::from_tenths(
            x_degrees.saturating_mul(10),
            y_degrees.saturating_mul(10),
            speed_percent,
        )
    }

    pub const fn from_tenths(x_tenths: i16, y_tenths: i16, speed_percent: u8) -> Self {
        Self {
            x_tenths: StackChanMotionContract::clamp_x_tenths(x_tenths),
            y_tenths: StackChanMotionContract::clamp_y_tenths(y_tenths),
            speed_percent: StackChanMotionContract::clamp_speed_percent(speed_percent),
        }
    }

    pub const fn neutral(speed_percent: u8) -> Self {
        Self::from_tenths(
            StackChanMotionContract::X_NEUTRAL_TENTHS,
            StackChanMotionContract::Y_NEUTRAL_TENTHS,
            speed_percent,
        )
    }

    pub const fn pose(self) -> StackChanPose {
        StackChanPose::new(self.x_tenths, self.y_tenths)
    }

    pub const fn bsp_y_tenths(self) -> i16 {
        StackChanMotionContract::clamp_hardware_y_tenths(self.y_tenths)
    }

    pub const fn bsp_speed(self) -> u16 {
        StackChanMotionContract::speed_percent_to_bsp(self.speed_percent)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StackChanMotionStatus {
    pub ready: bool,
    pub moving: bool,
    pub pose: StackChanPose,
}

#[derive(Debug)]
pub struct StackChanBspMotion {
    pose: StackChanPose,
}

#[derive(Debug)]
pub struct PwmServo {
    config: PwmServoConfig,
    angle_tenths: i16,
}

#[derive(Debug)]
pub struct StackChanPwmServos {
    pan: PwmServo,
    tilt: PwmServo,
    pose: StackChanPose,
}

/// Compatibility alias for earlier versions of the PWM fallback API.
///
/// Prefer `StackChanPwmServos`; official Stack-chan CoreS3 hardware requires
/// StackChan-BSP Motion rather than this generic LEDC PWM implementation.
pub type StackChanServos = StackChanPwmServos;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CardputerSdPins {
    pub sck: i32,
    pub miso: i32,
    pub mosi: i32,
    pub cs: i32,
}

impl CardputerSdPins {
    /// Built-in Cardputer/Cardputer-Adv microSD slot pins from M5Stack's Arduino examples.
    pub const BUILTIN: Self = Self {
        sck: 40,
        miso: 39,
        mosi: 14,
        cs: 12,
    };

    pub const fn new(sck: i32, miso: i32, mosi: i32, cs: i32) -> Self {
        Self {
            sck,
            miso,
            mosi,
            cs,
        }
    }

    pub const fn pins(self) -> (i32, i32, i32, i32) {
        (self.sck, self.miso, self.mosi, self.cs)
    }

    pub const fn sck(self) -> i32 {
        self.sck
    }

    pub const fn miso(self) -> i32 {
        self.miso
    }

    pub const fn mosi(self) -> i32 {
        self.mosi
    }

    pub const fn cs(self) -> i32 {
        self.cs
    }

    pub const fn spi_pins(self) -> SpiPins {
        SpiPins::new(self.sck, self.miso, self.mosi, self.cs)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SdCardType {
    None,
    Mmc,
    Sd,
    Sdhc,
    Unknown(i32),
}

impl SdCardType {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Mmc,
            2 => Self::Sd,
            3 => Self::Sdhc,
            value => Self::Unknown(value),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Mmc => 1,
            Self::Sd => 2,
            Self::Sdhc => 3,
            Self::Unknown(value) => value,
        }
    }

    pub const fn is_present(self) -> bool {
        !matches!(self, Self::None)
    }

    pub const fn is_absent(self) -> bool {
        matches!(self, Self::None)
    }

    pub const fn is_mmc(self) -> bool {
        matches!(self, Self::Mmc)
    }

    pub const fn is_sd(self) -> bool {
        matches!(self, Self::Sd | Self::Sdhc)
    }

    pub const fn is_standard_capacity_sd(self) -> bool {
        matches!(self, Self::Sd)
    }

    pub const fn is_high_capacity_sd(self) -> bool {
        matches!(self, Self::Sdhc)
    }

    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct SdCardInfo {
    pub size_bytes: u64,
    pub total_bytes: u64,
    pub used_bytes: u64,
}

impl SdCardInfo {
    pub const BYTES_PER_MEBIBYTE: u64 = 1024 * 1024;

    pub const fn free_bytes(self) -> u64 {
        self.total_bytes.saturating_sub(self.used_bytes)
    }

    pub const fn size_mebibytes(self) -> u64 {
        self.size_bytes / Self::BYTES_PER_MEBIBYTE
    }

    pub const fn total_mebibytes(self) -> u64 {
        self.total_bytes / Self::BYTES_PER_MEBIBYTE
    }

    pub const fn used_mebibytes(self) -> u64 {
        self.used_bytes / Self::BYTES_PER_MEBIBYTE
    }

    pub const fn free_mebibytes(self) -> u64 {
        self.free_bytes() / Self::BYTES_PER_MEBIBYTE
    }

    pub const fn has_capacity(self) -> bool {
        self.total_bytes > 0
    }

    pub fn used_fraction(self) -> Option<f32> {
        if self.total_bytes == 0 {
            None
        } else {
            Some(self.used_bytes as f32 / self.total_bytes as f32)
        }
    }

    pub fn used_percent(self) -> Option<f32> {
        self.used_fraction().map(|fraction| fraction * 100.0)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CardputerSdDirEntry {
    pub name: String,
    pub is_directory: bool,
    pub size: u64,
}

impl CardputerSdDirEntry {
    pub fn file(name: impl Into<String>, size: u64) -> Self {
        Self {
            name: name.into(),
            is_directory: false,
            size,
        }
    }

    pub fn directory(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_directory: true,
            size: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn is_directory(&self) -> bool {
        self.is_directory
    }

    pub const fn is_file(&self) -> bool {
        !self.is_directory
    }

    pub const fn size_bytes(&self) -> u64 {
        self.size
    }

    pub fn extension(&self) -> Option<&str> {
        self.name.rsplit_once('.').map(|(_, extension)| extension)
    }

    pub fn file_stem(&self) -> &str {
        self.name
            .rsplit_once('.')
            .map_or(self.name.as_str(), |(stem, _)| stem)
    }

    pub fn has_extension(&self, extension: &str) -> bool {
        self.extension()
            .is_some_and(|value| value.eq_ignore_ascii_case(extension))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NecFrame {
    pub address: u16,
    pub command: u8,
    pub repeats: u8,
}

impl NecFrame {
    pub const fn new(address: u16, command: u8) -> Self {
        Self {
            address,
            command,
            repeats: 0,
        }
    }

    pub const fn with_repeats(mut self, repeats: u8) -> Self {
        self.repeats = repeats;
        self
    }

    pub const fn with_address(mut self, address: u16) -> Self {
        self.address = address;
        self
    }

    pub const fn with_command(mut self, command: u8) -> Self {
        self.command = command;
        self
    }

    pub const fn repeat_count(self) -> u8 {
        self.repeats
    }

    pub const fn address(self) -> u16 {
        self.address
    }

    pub const fn command(self) -> u8 {
        self.command
    }

    pub const fn components(self) -> (u16, u8, u8) {
        (self.address, self.command, self.repeats)
    }

    pub const fn has_repeats(self) -> bool {
        self.repeats > 0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct I2cPins {
    pub sda: i32,
    pub scl: i32,
}

impl I2cPins {
    /// Built-in Cardputer/Cardputer-Adv HY2.0-4P Grove pins from M5Unified's external I2C table.
    pub const CARDPUTER_GROVE: Self = Self { sda: 2, scl: 1 };

    pub const fn new(sda: i32, scl: i32) -> Self {
        Self { sda, scl }
    }

    pub const fn pins(self) -> (i32, i32) {
        (self.sda, self.scl)
    }

    pub const fn sda(self) -> i32 {
        self.sda
    }

    pub const fn scl(self) -> i32 {
        self.scl
    }

    pub fn from_pin_names(sda: PinName, scl: PinName) -> Option<Self> {
        Some(Self {
            sda: get_pin(sda)?,
            scl: get_pin(scl)?,
        })
    }

    pub fn port_a() -> Option<Self> {
        Self::from_pin_names(PinName::PORT_A_SDA, PinName::PORT_A_SCL)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct I2cConfig {
    pub frequency_hz: u32,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            frequency_hz: Self::DEFAULT_FREQUENCY_HZ,
        }
    }
}

impl I2cConfig {
    pub const DEFAULT_FREQUENCY_HZ: u32 = 100_000;
    pub const FAST_FREQUENCY_HZ: u32 = 400_000;

    pub const fn new(frequency_hz: u32) -> Self {
        Self { frequency_hz }
    }

    pub const fn with_frequency_hz(mut self, frequency_hz: u32) -> Self {
        self.frequency_hz = frequency_hz;
        self
    }

    pub const fn frequency_hz(self) -> u32 {
        self.frequency_hz
    }

    pub const fn is_standard_mode(self) -> bool {
        self.frequency_hz == Self::DEFAULT_FREQUENCY_HZ
    }

    pub const fn is_fast_mode(self) -> bool {
        self.frequency_hz == Self::FAST_FREQUENCY_HZ
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpiPins {
    pub sck: i32,
    pub miso: i32,
    pub mosi: i32,
    pub cs: i32,
}

impl SpiPins {
    /// Built-in Cardputer/Cardputer-Adv microSD SPI pins.
    ///
    /// Use this only for raw SPI experiments when the SD card wrapper is not mounted.
    pub const CARDPUTER_SD: Self = Self {
        sck: 40,
        miso: 39,
        mosi: 14,
        cs: 12,
    };

    pub const fn new(sck: i32, miso: i32, mosi: i32, cs: i32) -> Self {
        Self {
            sck,
            miso,
            mosi,
            cs,
        }
    }

    pub const fn pins(self) -> (i32, i32, i32, i32) {
        (self.sck, self.miso, self.mosi, self.cs)
    }

    pub const fn sck(self) -> i32 {
        self.sck
    }

    pub const fn miso(self) -> i32 {
        self.miso
    }

    pub const fn mosi(self) -> i32 {
        self.mosi
    }

    pub const fn cs(self) -> i32 {
        self.cs
    }

    pub const fn from_cardputer_sd(pins: CardputerSdPins) -> Self {
        pins.spi_pins()
    }

    pub fn from_pin_names(sck: PinName, miso: PinName, mosi: PinName, cs: PinName) -> Option<Self> {
        Some(Self {
            sck: get_pin(sck)?,
            miso: get_pin(miso)?,
            mosi: get_pin(mosi)?,
            cs: get_pin(cs)?,
        })
    }

    pub fn sd() -> Option<Self> {
        Self::from_pin_names(
            PinName::SD_SPI_SCLK,
            PinName::SD_SPI_CIPO,
            PinName::SD_SPI_COPI,
            PinName::SD_SPI_SS,
        )
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum SpiBitOrder {
    #[default]
    MsbFirst,
    LsbFirst,
}

impl SpiBitOrder {
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::MsbFirst),
            1 => Some(Self::LsbFirst),
            _ => None,
        }
    }

    pub const fn from_lsb_first(lsb_first: bool) -> Self {
        if lsb_first {
            Self::LsbFirst
        } else {
            Self::MsbFirst
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::MsbFirst => 0,
            Self::LsbFirst => 1,
        }
    }

    pub const fn msb_first(self) -> bool {
        matches!(self, Self::MsbFirst)
    }

    pub const fn lsb_first(self) -> bool {
        matches!(self, Self::LsbFirst)
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum SpiMode {
    #[default]
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

impl SpiMode {
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::Mode0),
            1 => Some(Self::Mode1),
            2 => Some(Self::Mode2),
            3 => Some(Self::Mode3),
            _ => None,
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::Mode0 => 0,
            Self::Mode1 => 1,
            Self::Mode2 => 2,
            Self::Mode3 => 3,
        }
    }

    pub const fn clock_polarity(self) -> bool {
        matches!(self, Self::Mode2 | Self::Mode3)
    }

    pub const fn clock_phase(self) -> bool {
        matches!(self, Self::Mode1 | Self::Mode3)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpiConfig {
    pub frequency_hz: u32,
    pub mode: SpiMode,
    pub bit_order: SpiBitOrder,
}

impl Default for SpiConfig {
    fn default() -> Self {
        Self {
            frequency_hz: DEFAULT_SPI_FREQUENCY_HZ,
            mode: SpiMode::Mode0,
            bit_order: SpiBitOrder::MsbFirst,
        }
    }
}

impl SpiConfig {
    pub const fn new(frequency_hz: u32, mode: SpiMode, bit_order: SpiBitOrder) -> Self {
        Self {
            frequency_hz,
            mode,
            bit_order,
        }
    }

    pub const fn with_frequency_hz(mut self, frequency_hz: u32) -> Self {
        self.frequency_hz = frequency_hz;
        self
    }

    pub const fn with_mode(mut self, mode: SpiMode) -> Self {
        self.mode = mode;
        self
    }

    pub const fn with_bit_order(mut self, bit_order: SpiBitOrder) -> Self {
        self.bit_order = bit_order;
        self
    }

    pub const fn frequency_hz(self) -> u32 {
        self.frequency_hz
    }

    pub const fn mode(self) -> SpiMode {
        self.mode
    }

    pub const fn bit_order(self) -> SpiBitOrder {
        self.bit_order
    }

    pub const fn is_msb_first(self) -> bool {
        self.bit_order.msb_first()
    }

    pub const fn is_lsb_first(self) -> bool {
        self.bit_order.lsb_first()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UartPins {
    pub rx: i32,
    pub tx: i32,
}

impl UartPins {
    /// Built-in Cardputer/Cardputer-Adv Grove UART mapping used by serial-terminal examples.
    pub const CARDPUTER_GROVE: Self = Self { rx: 1, tx: 2 };

    pub const fn new(rx: i32, tx: i32) -> Self {
        Self { rx, tx }
    }

    pub const fn pins(self) -> (i32, i32) {
        (self.rx, self.tx)
    }

    pub const fn rx(self) -> i32 {
        self.rx
    }

    pub const fn tx(self) -> i32 {
        self.tx
    }

    pub fn from_pin_names(rx: PinName, tx: PinName) -> Option<Self> {
        Some(Self {
            rx: get_pin(rx)?,
            tx: get_pin(tx)?,
        })
    }

    pub fn port_c() -> Option<Self> {
        Self::from_pin_names(PinName::PORT_C_RXD, PinName::PORT_C_TXD)
    }

    pub fn port_d() -> Option<Self> {
        Self::from_pin_names(PinName::PORT_D_RXD, PinName::PORT_D_TXD)
    }

    pub fn port_e() -> Option<Self> {
        Self::from_pin_names(PinName::PORT_E_RXD, PinName::PORT_E_TXD)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UartConfig {
    pub baud: u32,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baud: Self::DEFAULT_BAUD,
        }
    }
}

impl UartConfig {
    pub const DEFAULT_BAUD: u32 = 115_200;

    pub const fn new(baud: u32) -> Self {
        Self { baud }
    }

    pub const fn with_baud(mut self, baud: u32) -> Self {
        self.baud = baud;
        self
    }

    pub const fn baud(self) -> u32 {
        self.baud
    }

    pub const fn is_default_baud(self) -> bool {
        self.baud == Self::DEFAULT_BAUD
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GpioPin {
    pub pin: i32,
}

impl GpioPin {
    pub const fn new(pin: i32) -> Self {
        Self { pin }
    }

    pub fn from_pin_name(name: PinName) -> Option<Self> {
        Some(Self {
            pin: get_pin(name)?,
        })
    }

    pub const fn raw(self) -> i32 {
        self.pin
    }

    pub const fn pin(self) -> i32 {
        self.pin
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct I2cAddress(u8);

impl I2cAddress {
    pub const MIN_7BIT: u8 = 0x00;
    pub const MAX_7BIT: u8 = 0x7f;
    pub const FIRST_NON_RESERVED: u8 = 0x08;
    pub const LAST_NON_RESERVED: u8 = 0x77;

    pub const fn new(raw: u8) -> Option<Self> {
        if raw <= Self::MAX_7BIT {
            Some(Self(raw))
        } else {
            None
        }
    }

    pub const fn from_7bit(raw: u8) -> Option<Self> {
        Self::new(raw)
    }

    pub const fn from_8bit(raw: u8) -> Self {
        Self(raw >> 1)
    }

    pub const fn new_unchecked(raw: u8) -> Self {
        Self(raw)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn raw(self) -> u8 {
        self.0
    }

    pub const fn as_7bit(self) -> u8 {
        self.0
    }

    pub const fn write_address_8bit(self) -> u8 {
        self.0 << 1
    }

    pub const fn read_address_8bit(self) -> u8 {
        (self.0 << 1) | 1
    }

    pub const fn is_reserved(self) -> bool {
        self.0 < Self::FIRST_NON_RESERVED || self.0 > Self::LAST_NON_RESERVED
    }

    pub const fn is_non_reserved(self) -> bool {
        !self.is_reserved()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GrovePin {
    G1,
    G2,
    Raw(i32),
}

impl GrovePin {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::G1,
            2 => Self::G2,
            pin => Self::Raw(pin),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::G1 => 1,
            Self::G2 => 2,
            Self::Raw(pin) => pin,
        }
    }

    pub const fn is_builtin(self) -> bool {
        matches!(self, Self::G1 | Self::G2)
    }

    pub const fn is_g1(self) -> bool {
        matches!(self, Self::G1)
    }

    pub const fn is_g2(self) -> bool {
        matches!(self, Self::G2)
    }

    pub const fn is_raw(self) -> bool {
        matches!(self, Self::Raw(_))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GpioMode {
    Input,
    Output,
    InputPullup,
    InputPulldown,
}

impl GpioMode {
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Input),
            1 => Some(Self::Output),
            2 => Some(Self::InputPullup),
            3 => Some(Self::InputPulldown),
            _ => None,
        }
    }

    pub const fn raw(self) -> c_int {
        match self {
            Self::Input => 0,
            Self::Output => 1,
            Self::InputPullup => 2,
            Self::InputPulldown => 3,
        }
    }

    pub const fn is_input(self) -> bool {
        matches!(self, Self::Input | Self::InputPullup | Self::InputPulldown)
    }

    pub const fn is_output(self) -> bool {
        matches!(self, Self::Output)
    }

    pub const fn has_pullup(self) -> bool {
        matches!(self, Self::InputPullup)
    }

    pub const fn has_pulldown(self) -> bool {
        matches!(self, Self::InputPulldown)
    }

    pub const fn has_pull(self) -> bool {
        self.has_pullup() || self.has_pulldown()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AnalogOutputConfig {
    pub duty: u8,
    pub frequency_hz: u32,
    pub resolution_bits: u8,
}

impl Default for AnalogOutputConfig {
    fn default() -> Self {
        Self {
            duty: 0,
            frequency_hz: Self::DEFAULT_FREQUENCY_HZ,
            resolution_bits: Self::DEFAULT_RESOLUTION_BITS,
        }
    }
}

impl AnalogOutputConfig {
    pub const DEFAULT_FREQUENCY_HZ: u32 = 1_000;
    pub const DEFAULT_RESOLUTION_BITS: u8 = 8;

    pub const fn new(duty: u8) -> Self {
        Self {
            duty,
            frequency_hz: Self::DEFAULT_FREQUENCY_HZ,
            resolution_bits: Self::DEFAULT_RESOLUTION_BITS,
        }
    }

    pub const fn with_duty(mut self, duty: u8) -> Self {
        self.duty = duty;
        self
    }

    pub const fn with_frequency_hz(mut self, frequency_hz: u32) -> Self {
        self.frequency_hz = frequency_hz;
        self
    }

    pub const fn with_resolution_bits(mut self, resolution_bits: u8) -> Self {
        self.resolution_bits = resolution_bits;
        self
    }

    pub const fn duty(self) -> u8 {
        self.duty
    }

    pub const fn frequency_hz(self) -> u32 {
        self.frequency_hz
    }

    pub const fn resolution_bits(self) -> u8 {
        self.resolution_bits
    }

    pub const fn is_default_frequency(self) -> bool {
        self.frequency_hz == Self::DEFAULT_FREQUENCY_HZ
    }

    pub const fn is_default_resolution(self) -> bool {
        self.resolution_bits == Self::DEFAULT_RESOLUTION_BITS
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CardputerKeyboardState {
    pub tab: bool,
    pub fn_key: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub opt: bool,
    pub alt: bool,
    pub del: bool,
    pub enter: bool,
    pub space: bool,
    pub modifiers: u8,
    pub word: Vec<u8>,
    pub hid_keys: Vec<u8>,
    pub modifier_keys: Vec<u8>,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct CardputerKeyValue {
    pub first: u8,
    pub second: u8,
}

impl CardputerKeyboardState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_word(mut self, word: impl Into<Vec<u8>>) -> Self {
        self.word = word.into();
        self
    }

    pub fn with_hid_keys(mut self, hid_keys: impl Into<Vec<u8>>) -> Self {
        self.hid_keys = hid_keys.into();
        self
    }

    pub fn with_modifier_keys(mut self, modifier_keys: impl Into<Vec<u8>>) -> Self {
        self.modifier_keys = modifier_keys.into();
        self
    }

    pub const fn with_modifiers(mut self, modifiers: u8) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn is_empty(&self) -> bool {
        !self.tab
            && !self.fn_key
            && !self.shift
            && !self.ctrl
            && !self.opt
            && !self.alt
            && !self.del
            && !self.enter
            && !self.space
            && self.modifiers == 0
            && self.word.is_empty()
            && self.hid_keys.is_empty()
            && self.modifier_keys.is_empty()
    }

    pub fn has_word(&self) -> bool {
        !self.word.is_empty()
    }

    pub fn has_hid_keys(&self) -> bool {
        !self.hid_keys.is_empty()
    }

    pub fn has_modifier_keys(&self) -> bool {
        !self.modifier_keys.is_empty()
    }

    pub fn has_modifiers(&self) -> bool {
        self.modifiers != 0 || self.ctrl || self.opt || self.alt || self.shift || self.fn_key
    }

    pub fn word_utf8(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.word)
    }

    pub fn word_lossy(&self) -> String {
        String::from_utf8_lossy(&self.word).into_owned()
    }

    pub fn first_word_byte(&self) -> Option<u8> {
        self.word.first().copied()
    }

    pub fn first_word_char(&self) -> Option<char> {
        self.word_utf8().ok()?.chars().next()
    }

    pub fn first_hid_key(&self) -> Option<u8> {
        self.hid_keys.first().copied()
    }

    pub fn first_modifier_key(&self) -> Option<u8> {
        self.modifier_keys.first().copied()
    }

    pub fn contains_word_byte(&self, byte: u8) -> bool {
        self.word.contains(&byte)
    }

    pub fn contains_hid_key(&self, key: u8) -> bool {
        self.hid_keys.contains(&key)
    }

    pub fn contains_modifier_key(&self, key: u8) -> bool {
        self.modifier_keys.contains(&key)
    }
}

impl CardputerKeyValue {
    pub const fn new(first: u8, second: u8) -> Self {
        Self { first, second }
    }

    pub const fn bytes(self) -> (u8, u8) {
        (self.first, self.second)
    }

    pub const fn is_empty(self) -> bool {
        self.first == 0 && self.second == 0
    }

    pub fn first_char(self) -> Option<char> {
        ascii_byte_to_char(self.first)
    }

    pub fn second_char(self) -> Option<char> {
        ascii_byte_to_char(self.second)
    }

    pub fn chars(self) -> (Option<char>, Option<char>) {
        (self.first_char(), self.second_char())
    }
}

impl CardputerKeyboard {
    pub const COLUMNS: u8 = 14;
    pub const ROWS: u8 = 4;

    pub fn begin(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_begin() }
    }

    pub fn is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_is_pressed() }
    }

    pub fn pressed_count(&self) -> u8 {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_pressed_count() }
    }

    pub fn is_change(&self) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_is_change() }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_is_key_pressed(key) }
    }

    pub fn is_char_pressed(&self, ch: char) -> bool {
        cardputer_keyboard_char_to_key(ch)
            .map(|key| self.is_key_pressed(key))
            .unwrap_or(false)
    }

    pub fn try_is_char_pressed(&self, ch: char) -> Result<bool, Error> {
        let key = cardputer_keyboard_char_to_key(ch)?;
        Ok(self.is_key_pressed(key))
    }

    pub fn key_at(&self, x: u8, y: u8) -> Option<u8> {
        if x >= Self::COLUMNS || y >= Self::ROWS {
            return None;
        }
        let key = unsafe { m5unified_sys::m5u_cardputer_keyboard_get_key(x, y) };
        (key != 0).then_some(key)
    }

    pub fn try_key_at(&self, x: u8, y: u8) -> Result<u8, Error> {
        validate_cardputer_keyboard_position(x, y)?;
        self.key_at(x, y)
            .ok_or(Error::Unavailable("cardputer keyboard key"))
    }

    pub fn key_value_at(&self, x: u8, y: u8) -> Option<CardputerKeyValue> {
        if x >= Self::COLUMNS || y >= Self::ROWS {
            return None;
        }
        let mut raw = m5unified_sys::m5u_cardputer_key_value_t::default();
        let ok = unsafe { m5unified_sys::m5u_cardputer_keyboard_get_key_value(x, y, &mut raw) };
        ok.then_some(CardputerKeyValue {
            first: raw.first,
            second: raw.second,
        })
    }

    pub fn try_key_value_at(&self, x: u8, y: u8) -> Result<CardputerKeyValue, Error> {
        validate_cardputer_keyboard_position(x, y)?;
        self.key_value_at(x, y)
            .ok_or(Error::Unavailable("cardputer keyboard key"))
    }

    pub fn state(&self) -> Option<CardputerKeyboardState> {
        let mut raw = m5unified_sys::m5u_cardputer_keyboard_state_t::default();
        let ok = unsafe { m5unified_sys::m5u_cardputer_keyboard_get_state(&mut raw) };
        if !ok {
            return None;
        }

        let word_len = raw
            .word_len
            .min(m5unified_sys::M5U_CARDPUTER_KEYBOARD_WORD_CAPACITY);
        let hid_len = raw
            .hid_len
            .min(m5unified_sys::M5U_CARDPUTER_KEYBOARD_HID_CAPACITY);
        let modifier_len = raw
            .modifier_len
            .min(m5unified_sys::M5U_CARDPUTER_KEYBOARD_MODIFIER_CAPACITY);

        Some(CardputerKeyboardState {
            tab: raw.tab,
            fn_key: raw.fn_key,
            shift: raw.shift,
            ctrl: raw.ctrl,
            opt: raw.opt,
            alt: raw.alt,
            del: raw.del,
            enter: raw.enter,
            space: raw.space,
            modifiers: raw.modifiers,
            word: raw.word[..word_len].to_vec(),
            hid_keys: raw.hid_keys[..hid_len].to_vec(),
            modifier_keys: raw.modifier_keys[..modifier_len].to_vec(),
        })
    }

    pub fn try_state(&self) -> Result<CardputerKeyboardState, Error> {
        self.state().ok_or(Error::Unavailable("cardputer keyboard"))
    }

    pub fn word_lossy(&self) -> Option<String> {
        self.state().map(|state| state.word_lossy())
    }

    pub fn try_word_lossy(&self) -> Result<String, Error> {
        Ok(self.try_state()?.word_lossy())
    }

    pub fn word_bytes(&self) -> Option<Vec<u8>> {
        self.state().map(|state| state.word)
    }

    pub fn try_word_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(self.try_state()?.word)
    }

    pub fn has_word(&self) -> Option<bool> {
        self.state().map(|state| state.has_word())
    }

    pub fn try_has_word(&self) -> Result<bool, Error> {
        Ok(self.try_state()?.has_word())
    }

    pub fn contains_word_byte(&self, byte: u8) -> Option<bool> {
        self.state().map(|state| state.contains_word_byte(byte))
    }

    pub fn try_contains_word_byte(&self, byte: u8) -> Result<bool, Error> {
        Ok(self.try_state()?.contains_word_byte(byte))
    }

    pub fn hid_keys(&self) -> Option<Vec<u8>> {
        self.state().map(|state| state.hid_keys)
    }

    pub fn try_hid_keys(&self) -> Result<Vec<u8>, Error> {
        Ok(self.try_state()?.hid_keys)
    }

    pub fn first_hid_key(&self) -> Option<u8> {
        self.state()?.first_hid_key()
    }

    pub fn try_first_hid_key(&self) -> Result<Option<u8>, Error> {
        Ok(self.try_state()?.first_hid_key())
    }

    pub fn contains_hid_key(&self, key: u8) -> Option<bool> {
        self.state().map(|state| state.contains_hid_key(key))
    }

    pub fn try_contains_hid_key(&self, key: u8) -> Result<bool, Error> {
        Ok(self.try_state()?.contains_hid_key(key))
    }

    pub fn modifier_keys(&self) -> Option<Vec<u8>> {
        self.state().map(|state| state.modifier_keys)
    }

    pub fn try_modifier_keys(&self) -> Result<Vec<u8>, Error> {
        Ok(self.try_state()?.modifier_keys)
    }

    pub fn first_modifier_key(&self) -> Option<u8> {
        self.state()?.first_modifier_key()
    }

    pub fn try_first_modifier_key(&self) -> Result<Option<u8>, Error> {
        Ok(self.try_state()?.first_modifier_key())
    }

    pub fn modifiers(&self) -> Option<u8> {
        self.state().map(|state| state.modifiers)
    }

    pub fn try_modifiers(&self) -> Result<u8, Error> {
        Ok(self.try_state()?.modifiers)
    }

    pub fn contains_modifier_key(&self, key: u8) -> Option<bool> {
        self.state().map(|state| state.contains_modifier_key(key))
    }

    pub fn try_contains_modifier_key(&self, key: u8) -> Result<bool, Error> {
        Ok(self.try_state()?.contains_modifier_key(key))
    }

    pub fn first_word_char(&self) -> Option<char> {
        self.state()?.first_word_char()
    }

    pub fn try_first_word_char(&self) -> Result<Option<char>, Error> {
        Ok(self.try_state()?.first_word_char())
    }

    pub fn capslocked(&self) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_capslocked() }
    }

    pub fn set_capslocked(&mut self, locked: bool) {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_set_capslocked(locked) }
    }
}

fn validate_cardputer_keyboard_position(x: u8, y: u8) -> Result<(), Error> {
    if x >= CardputerKeyboard::COLUMNS || y >= CardputerKeyboard::ROWS {
        return Err(Error::InvalidValue("cardputer keyboard position"));
    }
    Ok(())
}

fn cardputer_keyboard_char_to_key(ch: char) -> Result<u8, Error> {
    if ch == '\0' || !ch.is_ascii() {
        return Err(Error::InvalidValue("cardputer keyboard key"));
    }
    Ok(ch as u8)
}

fn ascii_byte_to_char(byte: u8) -> Option<char> {
    byte.is_ascii()
        .then_some(byte as char)
        .filter(|ch| *ch != '\0')
}

impl CardputerSd {
    pub const DEFAULT_FREQUENCY_HZ: u32 = 25_000_000;

    pub fn begin(&mut self) -> bool {
        self.begin_with(CardputerSdPins::BUILTIN, Self::DEFAULT_FREQUENCY_HZ)
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.begin()
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd"))
    }

    pub fn begin_with(&mut self, pins: CardputerSdPins, frequency_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_cardputer_sd_begin(
                pins.sck as c_int,
                pins.miso as c_int,
                pins.mosi as c_int,
                pins.cs as c_int,
                frequency_hz,
            )
        }
    }

    pub fn try_begin_with(
        &mut self,
        pins: CardputerSdPins,
        frequency_hz: u32,
    ) -> Result<(), Error> {
        validate_frequency_hz(frequency_hz)?;
        self.begin_with(pins, frequency_hz)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd"))
    }

    pub fn end(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_sd_end() }
    }

    pub fn card_type(&self) -> SdCardType {
        SdCardType::from_raw(unsafe { m5unified_sys::m5u_cardputer_sd_card_type() as i32 })
    }

    pub fn info(&self) -> SdCardInfo {
        SdCardInfo {
            size_bytes: unsafe { m5unified_sys::m5u_cardputer_sd_card_size_bytes() },
            total_bytes: unsafe { m5unified_sys::m5u_cardputer_sd_total_bytes() },
            used_bytes: unsafe { m5unified_sys::m5u_cardputer_sd_used_bytes() },
        }
    }

    pub fn exists(&self, path: &str) -> Result<bool, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_cardputer_sd_exists(path.as_ptr()) })
    }

    pub fn file_size(&self, path: &str) -> Result<u64, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_cardputer_sd_file_size(path.as_ptr()) })
    }

    pub fn is_directory(&self, path: &str) -> Result<bool, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_cardputer_sd_is_directory(path.as_ptr()) })
    }

    pub fn list_dir(
        &self,
        path: &str,
        max_entries: usize,
    ) -> Result<Vec<CardputerSdDirEntry>, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        if max_entries == 0 {
            return Ok(Vec::new());
        }

        let mut raw_entries =
            vec![m5unified_sys::m5u_cardputer_sd_dir_entry_t::default(); max_entries];
        let count = unsafe {
            m5unified_sys::m5u_cardputer_sd_list_dir(
                path.as_ptr(),
                raw_entries.as_mut_ptr(),
                raw_entries.len(),
            )
        }
        .min(raw_entries.len());

        let entries = raw_entries
            .iter()
            .take(count)
            .map(|entry| {
                let name = unsafe { CStr::from_ptr(entry.name.as_ptr()) }
                    .to_string_lossy()
                    .into_owned();
                CardputerSdDirEntry {
                    name,
                    is_directory: entry.is_directory,
                    size: entry.size,
                }
            })
            .collect();

        Ok(entries)
    }

    pub fn read_file(&self, path: &str, buffer: &mut [u8]) -> Result<usize, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_cardputer_sd_read_file(
                path.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        })
    }

    pub fn try_read_file_exact(&self, path: &str, buffer: &mut [u8]) -> Result<(), Error> {
        let read = self.read_file(path, buffer)?;
        (read == buffer.len())
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd read"))
    }

    pub fn write_file(&mut self, path: &str, data: &[u8]) -> Result<usize, Error> {
        self.write_file_with(path, data, false)
    }

    pub fn try_write_file_all(&mut self, path: &str, data: &[u8]) -> Result<(), Error> {
        let written = self.write_file(path, data)?;
        (written == data.len())
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd write"))
    }

    pub fn append_file(&mut self, path: &str, data: &[u8]) -> Result<usize, Error> {
        self.write_file_with(path, data, true)
    }

    pub fn try_append_file_all(&mut self, path: &str, data: &[u8]) -> Result<(), Error> {
        let written = self.append_file(path, data)?;
        (written == data.len())
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd write"))
    }

    pub fn remove_file(&mut self, path: &str) -> Result<bool, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_cardputer_sd_remove(path.as_ptr()) })
    }

    pub fn try_remove_file(&mut self, path: &str) -> Result<(), Error> {
        self.remove_file(path)?
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd remove"))
    }

    pub fn mkdir(&mut self, path: &str) -> Result<bool, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_cardputer_sd_mkdir(path.as_ptr()) })
    }

    pub fn try_mkdir(&mut self, path: &str) -> Result<(), Error> {
        self.mkdir(path)?
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd mkdir"))
    }

    pub fn rmdir(&mut self, path: &str) -> Result<bool, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_cardputer_sd_rmdir(path.as_ptr()) })
    }

    pub fn try_rmdir(&mut self, path: &str) -> Result<(), Error> {
        self.rmdir(path)?
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd rmdir"))
    }

    pub fn rename(&mut self, from_path: &str, to_path: &str) -> Result<bool, Error> {
        let from_path = CString::new(from_path).map_err(|_| Error::InvalidString)?;
        let to_path = CString::new(to_path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_cardputer_sd_rename(from_path.as_ptr(), to_path.as_ptr()) })
    }

    pub fn try_rename(&mut self, from_path: &str, to_path: &str) -> Result<(), Error> {
        self.rename(from_path, to_path)?
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd rename"))
    }

    fn write_file_with(&mut self, path: &str, data: &[u8], append: bool) -> Result<usize, Error> {
        let path = CString::new(path).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_cardputer_sd_write_file(
                path.as_ptr(),
                data.as_ptr(),
                data.len(),
                append,
            )
        })
    }
}

impl CardputerIr {
    /// Built-in Cardputer/Cardputer-Adv IR transmitter pin from M5Stack's Arduino examples.
    pub const BUILTIN_TX_PIN: i32 = 44;

    pub fn begin(&mut self) -> bool {
        self.begin_on_pin(Self::BUILTIN_TX_PIN)
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.begin()
            .then_some(())
            .ok_or(Error::Unavailable("cardputer ir"))
    }

    pub fn begin_on_pin(&mut self, pin: i32) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_ir_begin(pin as c_int) }
    }

    pub fn try_begin_on_pin(&mut self, pin: i32) -> Result<(), Error> {
        self.begin_on_pin(pin)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer ir"))
    }

    pub fn send_nec(&mut self, frame: NecFrame) -> bool {
        unsafe {
            m5unified_sys::m5u_cardputer_ir_send_nec(frame.address, frame.command, frame.repeats)
        }
    }

    pub fn try_send_nec(&mut self, frame: NecFrame) -> Result<(), Error> {
        self.send_nec(frame)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer ir"))
    }
}

impl PwmServo {
    pub fn attach(config: PwmServoConfig) -> Result<Self, Error> {
        validate_pwm_servo_config(config)?;
        let ok = unsafe {
            m5unified_sys::m5u_servo_attach(
                config.pin as c_int,
                config.channel as c_int,
                config.timer as c_int,
                config.frequency_hz,
                config.min_pulse_us,
                config.max_pulse_us,
            )
        };
        if !ok {
            return Err(Error::Unavailable("pwm servo"));
        }

        let mut servo = Self {
            config,
            angle_tenths: config.neutral_angle_tenths,
        };
        servo.write_angle_tenths(config.neutral_angle_tenths)?;
        Ok(servo)
    }

    pub fn detach(self) -> bool {
        unsafe { m5unified_sys::m5u_servo_detach(self.config.channel as c_int) }
    }

    pub const fn config(&self) -> PwmServoConfig {
        self.config
    }

    pub const fn angle_tenths(&self) -> i16 {
        self.angle_tenths
    }

    pub fn write_angle_degrees(&mut self, degrees: i16) -> Result<(), Error> {
        self.write_angle_tenths(
            degrees
                .checked_mul(10)
                .ok_or(Error::InvalidValue("servo angle"))?,
        )
    }

    pub fn write_angle_tenths(&mut self, angle_tenths: i16) -> Result<(), Error> {
        if angle_tenths < self.config.min_angle_tenths
            || angle_tenths > self.config.max_angle_tenths
        {
            return Err(Error::InvalidValue("servo angle"));
        }

        let pulse = self.pulse_us_for_angle(angle_tenths);
        self.write_pulse_us(pulse)?;
        self.angle_tenths = angle_tenths;
        Ok(())
    }

    pub fn write_pulse_us(&mut self, pulse_us: u16) -> Result<(), Error> {
        if pulse_us < self.config.min_pulse_us || pulse_us > self.config.max_pulse_us {
            return Err(Error::InvalidValue("servo pulse"));
        }

        unsafe { m5unified_sys::m5u_servo_write_pulse_us(self.config.channel as c_int, pulse_us) }
            .then_some(())
            .ok_or(Error::Unavailable("pwm servo"))
    }

    pub fn neutral(&mut self) -> Result<(), Error> {
        self.write_angle_tenths(self.config.neutral_angle_tenths)
    }

    fn pulse_us_for_angle(&self, angle_tenths: i16) -> u16 {
        let angle_span =
            i32::from(self.config.max_angle_tenths) - i32::from(self.config.min_angle_tenths);
        let pulse_span = u32::from(self.config.max_pulse_us - self.config.min_pulse_us);
        let angle_offset = i32::from(angle_tenths) - i32::from(self.config.min_angle_tenths);
        let pulse_offset = (u32::try_from(angle_offset).unwrap_or(0) * pulse_span)
            / u32::try_from(angle_span).unwrap_or(1);
        self.config.min_pulse_us + u16::try_from(pulse_offset).unwrap_or(u16::MAX)
    }
}

impl StackChanBspMotion {
    pub fn begin() -> Result<Self, Error> {
        unsafe { m5unified_sys::m5u_stackchan_motion_begin() }
            .then_some(Self {
                pose: StackChanPose::NEUTRAL,
            })
            .ok_or(Error::Unavailable("stackchan bsp motion"))
    }

    pub fn update(&mut self) {
        unsafe { m5unified_sys::m5u_stackchan_motion_update() }
    }

    pub const fn pose(&self) -> StackChanPose {
        self.pose
    }

    pub fn move_to(&mut self, command: StackChanMove) -> Result<(), Error> {
        unsafe {
            m5unified_sys::m5u_stackchan_motion_move(
                command.x_tenths,
                command.bsp_y_tenths(),
                command.bsp_speed(),
            )
        }
        .then_some(())
        .ok_or(Error::Unavailable("stackchan bsp motion"))?;
        self.pose = command.pose();
        Ok(())
    }

    pub fn home(&mut self, speed_percent: u8) -> Result<(), Error> {
        let speed_bsp = StackChanMotionContract::speed_percent_to_bsp(speed_percent);
        unsafe { m5unified_sys::m5u_stackchan_motion_home(speed_bsp) }
            .then_some(())
            .ok_or(Error::Unavailable("stackchan bsp motion"))?;
        self.pose = StackChanPose::NEUTRAL;
        Ok(())
    }

    pub fn neutral(&mut self, speed_percent: u8) -> Result<(), Error> {
        self.home(speed_percent)
    }

    pub fn nod(&mut self) -> Result<(), Error> {
        unsafe { m5unified_sys::m5u_stackchan_motion_nod() }
            .then_some(())
            .ok_or(Error::Unavailable("stackchan bsp motion"))?;
        self.pose = StackChanPose::NEUTRAL;
        Ok(())
    }

    pub fn shake(&mut self) -> Result<(), Error> {
        unsafe { m5unified_sys::m5u_stackchan_motion_shake() }
            .then_some(())
            .ok_or(Error::Unavailable("stackchan bsp motion"))?;
        self.pose = StackChanPose::NEUTRAL;
        Ok(())
    }

    pub fn status(&self) -> Result<StackChanMotionStatus, Error> {
        let mut raw = m5unified_sys::m5u_stackchan_motion_status_t::default();
        unsafe { m5unified_sys::m5u_stackchan_motion_status(&mut raw) }
            .then_some(StackChanMotionStatus {
                ready: raw.ready,
                moving: raw.moving,
                pose: StackChanPose::new(raw.yaw_tenths, raw.pitch_tenths),
            })
            .ok_or(Error::Unavailable("stackchan bsp motion"))
    }
}

impl StackChanPwmServos {
    pub fn attach(config: StackChanPwmServoConfig) -> Result<Self, Error> {
        let pan = PwmServo::attach(config.pan)?;
        let tilt = match PwmServo::attach(config.tilt) {
            Ok(tilt) => tilt,
            Err(error) => {
                let _ = pan.detach();
                return Err(error);
            }
        };
        let pose = StackChanPose::new(pan.angle_tenths(), tilt.angle_tenths());
        Ok(Self { pan, tilt, pose })
    }

    pub fn attach_pwm_pins(pins: PwmServoPins) -> Result<Self, Error> {
        Self::attach(StackChanPwmServoConfig::pwm_pins(pins))
    }

    pub fn detach(self) -> bool {
        self.pan.detach() && self.tilt.detach()
    }

    pub const fn pose(&self) -> StackChanPose {
        self.pose
    }

    pub fn neutral(&mut self) -> Result<(), Error> {
        self.write_pose(StackChanPose::NEUTRAL)
    }

    pub fn move_to(&mut self, command: StackChanMove) -> Result<(), Error> {
        self.write_pose(command.pose())
    }

    pub fn home(&mut self, _speed_percent: u8) -> Result<(), Error> {
        self.neutral()
    }

    pub fn nod<F>(&mut self, mut delay: F) -> Result<(), Error>
    where
        F: FnMut(u32),
    {
        for pose in [
            StackChanPose::new(0, 300),
            StackChanPose::new(0, 50),
            StackChanPose::new(0, 300),
            StackChanPose::NEUTRAL,
        ] {
            self.write_pose(pose)?;
            delay(200);
        }
        Ok(())
    }

    pub fn shake<F>(&mut self, mut delay: F) -> Result<(), Error>
    where
        F: FnMut(u32),
    {
        for pose in [
            StackChanPose::new(-400, StackChanPose::NEUTRAL.tilt_tenths),
            StackChanPose::new(400, StackChanPose::NEUTRAL.tilt_tenths),
            StackChanPose::new(-400, StackChanPose::NEUTRAL.tilt_tenths),
            StackChanPose::NEUTRAL,
        ] {
            self.write_pose(pose)?;
            delay(200);
        }
        Ok(())
    }

    pub fn status(&self) -> StackChanMotionStatus {
        StackChanMotionStatus {
            ready: true,
            moving: false,
            pose: self.pose,
        }
    }

    pub fn write_pose(&mut self, pose: StackChanPose) -> Result<(), Error> {
        validate_stackchan_pose(pose)?;
        self.pan.write_angle_tenths(pose.pan_tenths)?;
        self.tilt.write_angle_tenths(pose.tilt_tenths)?;
        self.pose = pose;
        Ok(())
    }

    pub fn write_pan_tenths(&mut self, pan_tenths: i16) -> Result<(), Error> {
        self.write_pose(StackChanPose::new(pan_tenths, self.pose.tilt_tenths))
    }

    pub fn write_tilt_tenths(&mut self, tilt_tenths: i16) -> Result<(), Error> {
        self.write_pose(StackChanPose::new(self.pose.pan_tenths, tilt_tenths))
    }

    pub fn smooth_move_to<F>(
        &mut self,
        target: StackChanPose,
        step_tenths: u16,
        delay_ms: u32,
        mut delay: F,
    ) -> Result<(), Error>
    where
        F: FnMut(u32),
    {
        validate_stackchan_pose(target)?;
        if step_tenths == 0 {
            return Err(Error::InvalidValue("servo step"));
        }

        while self.pose != target {
            let next = StackChanPose::new(
                step_axis(self.pose.pan_tenths, target.pan_tenths, step_tenths),
                step_axis(self.pose.tilt_tenths, target.tilt_tenths, step_tenths),
            );
            self.write_pose(next)?;
            delay(delay_ms);
        }

        Ok(())
    }
}

impl CardputerGrove {
    pub const DEFAULT_I2C_FREQUENCY_HZ: u32 = I2cConfig::DEFAULT_FREQUENCY_HZ;

    pub fn i2c_begin(&mut self) -> bool {
        self.i2c_begin_config(I2cConfig::default())
    }

    pub fn i2c_begin_config(&mut self, config: I2cConfig) -> bool {
        self.i2c_begin_with_config(I2cPins::CARDPUTER_GROVE, config)
    }

    pub fn i2c_try_begin(&mut self) -> Result<(), Error> {
        self.i2c_try_begin_config(I2cConfig::default())
    }

    pub fn i2c_try_begin_config(&mut self, config: I2cConfig) -> Result<(), Error> {
        validate_i2c_config(config)?;
        self.i2c_begin_config(config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove i2c"))
    }

    pub fn i2c_begin_with(&mut self, pins: I2cPins, frequency_hz: u32) -> bool {
        self.i2c_begin_with_config(pins, I2cConfig::new(frequency_hz))
    }

    pub fn i2c_begin_with_config(&mut self, pins: I2cPins, config: I2cConfig) -> bool {
        unsafe {
            m5unified_sys::m5u_cardputer_grove_i2c_begin(
                pins.sda as c_int,
                pins.scl as c_int,
                config.frequency_hz,
            )
        }
    }

    pub fn i2c_try_begin_with(&mut self, pins: I2cPins, frequency_hz: u32) -> Result<(), Error> {
        self.i2c_try_begin_with_config(pins, I2cConfig::new(frequency_hz))
    }

    pub fn i2c_try_begin_with_config(
        &mut self,
        pins: I2cPins,
        config: I2cConfig,
    ) -> Result<(), Error> {
        validate_i2c_config(config)?;
        self.i2c_begin_with_config(pins, config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove i2c"))
    }

    pub fn i2c_end(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_grove_i2c_end() }
    }

    pub fn i2c_probe(&self, address: I2cAddress) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_grove_i2c_probe(address.get()) }
    }

    pub fn i2c_write(&mut self, address: I2cAddress, data: &[u8]) -> bool {
        unsafe {
            m5unified_sys::m5u_cardputer_grove_i2c_write(address.get(), data.as_ptr(), data.len())
        }
    }

    pub fn i2c_try_write(&mut self, address: I2cAddress, data: &[u8]) -> Result<(), Error> {
        self.i2c_write(address, data)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove i2c"))
    }

    pub fn i2c_read(&mut self, address: I2cAddress, data: &mut [u8]) -> usize {
        unsafe {
            m5unified_sys::m5u_cardputer_grove_i2c_read(
                address.get(),
                data.as_mut_ptr(),
                data.len(),
            )
        }
    }

    pub fn i2c_try_read(&mut self, address: I2cAddress, data: &mut [u8]) -> Result<usize, Error> {
        let read = self.i2c_read(address, data);
        if read > 0 || data.is_empty() {
            Ok(read)
        } else {
            Err(Error::Unavailable("cardputer grove i2c"))
        }
    }

    pub fn i2c_try_read_exact(
        &mut self,
        address: I2cAddress,
        data: &mut [u8],
    ) -> Result<(), Error> {
        (self.i2c_try_read(address, data)? == data.len())
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove i2c"))
    }

    pub fn i2c_write_reg(&mut self, address: I2cAddress, reg: u8, data: &[u8]) -> bool {
        unsafe {
            m5unified_sys::m5u_cardputer_grove_i2c_write_reg(
                address.get(),
                reg,
                data.as_ptr(),
                data.len(),
            )
        }
    }

    pub fn i2c_try_write_reg(
        &mut self,
        address: I2cAddress,
        reg: u8,
        data: &[u8],
    ) -> Result<(), Error> {
        self.i2c_write_reg(address, reg, data)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove i2c"))
    }

    pub fn i2c_read_reg(&mut self, address: I2cAddress, reg: u8, data: &mut [u8]) -> usize {
        unsafe {
            m5unified_sys::m5u_cardputer_grove_i2c_read_reg(
                address.get(),
                reg,
                data.as_mut_ptr(),
                data.len(),
            )
        }
    }

    pub fn i2c_try_read_reg(
        &mut self,
        address: I2cAddress,
        reg: u8,
        data: &mut [u8],
    ) -> Result<usize, Error> {
        let read = self.i2c_read_reg(address, reg, data);
        if read > 0 || data.is_empty() {
            Ok(read)
        } else {
            Err(Error::Unavailable("cardputer grove i2c"))
        }
    }

    pub fn i2c_try_read_reg_exact(
        &mut self,
        address: I2cAddress,
        reg: u8,
        data: &mut [u8],
    ) -> Result<(), Error> {
        (self.i2c_try_read_reg(address, reg, data)? == data.len())
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove i2c"))
    }

    pub fn i2c_scan(&self) -> Vec<I2cAddress> {
        (0x08..=0x77)
            .filter_map(I2cAddress::new)
            .filter(|&address| self.i2c_probe(address))
            .collect()
    }

    pub fn gpio_pin_mode(&mut self, pin: GrovePin, mode: GpioMode) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_grove_gpio_pin_mode(pin.raw() as c_int, mode.raw()) }
    }

    pub fn gpio_try_pin_mode(&mut self, pin: GrovePin, mode: GpioMode) -> Result<(), Error> {
        self.gpio_pin_mode(pin, mode)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove gpio"))
    }

    pub fn gpio_write(&mut self, pin: GrovePin, high: bool) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_grove_gpio_write(pin.raw() as c_int, high) }
    }

    pub fn gpio_try_write(&mut self, pin: GrovePin, high: bool) -> Result<(), Error> {
        self.gpio_write(pin, high)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove gpio"))
    }

    pub fn gpio_read(&self, pin: GrovePin) -> Option<bool> {
        match unsafe { m5unified_sys::m5u_cardputer_grove_gpio_read(pin.raw() as c_int) } {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    pub fn gpio_try_read(&self, pin: GrovePin) -> Result<bool, Error> {
        self.gpio_read(pin)
            .ok_or(Error::Unavailable("cardputer grove gpio"))
    }

    pub fn analog_read(&self, pin: GrovePin) -> Option<u16> {
        let value = unsafe { m5unified_sys::m5u_gpio_analog_read(pin.raw() as c_int) };
        u16::try_from(value).ok()
    }

    pub fn analog_try_read(&self, pin: GrovePin) -> Result<u16, Error> {
        self.analog_read(pin)
            .ok_or(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_read_millivolts(&self, pin: GrovePin) -> Option<u16> {
        let value = unsafe { m5unified_sys::m5u_gpio_analog_read_millivolts(pin.raw() as c_int) };
        u16::try_from(value).ok()
    }

    pub fn analog_try_read_millivolts(&self, pin: GrovePin) -> Result<u16, Error> {
        self.analog_read_millivolts(pin)
            .ok_or(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_write(&mut self, pin: GrovePin, duty: u8) -> bool {
        unsafe { m5unified_sys::m5u_gpio_analog_write(pin.raw() as c_int, duty) }
    }

    pub fn analog_try_write(&mut self, pin: GrovePin, duty: u8) -> Result<(), Error> {
        self.analog_write(pin, duty)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_write_frequency(&mut self, pin: GrovePin, frequency_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_gpio_analog_write_frequency(pin.raw() as c_int, frequency_hz) }
    }

    pub fn analog_try_write_frequency(
        &mut self,
        pin: GrovePin,
        frequency_hz: u32,
    ) -> Result<(), Error> {
        validate_frequency_hz(frequency_hz)?;
        self.analog_write_frequency(pin, frequency_hz)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_write_resolution(&mut self, pin: GrovePin, resolution_bits: u8) -> bool {
        unsafe {
            m5unified_sys::m5u_gpio_analog_write_resolution(pin.raw() as c_int, resolution_bits)
        }
    }

    pub fn analog_try_write_resolution(
        &mut self,
        pin: GrovePin,
        resolution_bits: u8,
    ) -> Result<(), Error> {
        self.analog_write_resolution(pin, resolution_bits)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_write_config(&mut self, pin: GrovePin, config: AnalogOutputConfig) -> bool {
        self.analog_write_resolution(pin, config.resolution_bits)
            && self.analog_write_frequency(pin, config.frequency_hz)
            && self.analog_write(pin, config.duty)
    }

    pub fn analog_try_write_config(
        &mut self,
        pin: GrovePin,
        config: AnalogOutputConfig,
    ) -> Result<(), Error> {
        validate_analog_output_config(config)?;
        self.analog_write_config(pin, config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove analog"))
    }

    pub fn uart_begin(&mut self, baud: u32) -> bool {
        self.uart_begin_with_config(UartPins::CARDPUTER_GROVE, UartConfig::new(baud))
    }

    pub fn uart_begin_config(&mut self, config: UartConfig) -> bool {
        self.uart_begin_with_config(UartPins::CARDPUTER_GROVE, config)
    }

    pub fn uart_try_begin(&mut self, baud: u32) -> Result<(), Error> {
        self.uart_try_begin_config(UartConfig::new(baud))
    }

    pub fn uart_try_begin_config(&mut self, config: UartConfig) -> Result<(), Error> {
        validate_uart_config(config)?;
        self.uart_begin_config(config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove uart"))
    }

    pub fn uart_begin_with(&mut self, pins: UartPins, baud: u32) -> bool {
        self.uart_begin_with_config(pins, UartConfig::new(baud))
    }

    pub fn uart_begin_with_config(&mut self, pins: UartPins, config: UartConfig) -> bool {
        unsafe {
            m5unified_sys::m5u_cardputer_grove_uart_begin(
                pins.rx as c_int,
                pins.tx as c_int,
                config.baud,
            )
        }
    }

    pub fn uart_try_begin_with(&mut self, pins: UartPins, baud: u32) -> Result<(), Error> {
        self.uart_try_begin_with_config(pins, UartConfig::new(baud))
    }

    pub fn uart_try_begin_with_config(
        &mut self,
        pins: UartPins,
        config: UartConfig,
    ) -> Result<(), Error> {
        validate_uart_config(config)?;
        self.uart_begin_with_config(pins, config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove uart"))
    }

    pub fn uart_end(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_end() }
    }

    pub fn uart_available(&self) -> usize {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_available() }
    }

    pub fn uart_read(&mut self, data: &mut [u8]) -> usize {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_read(data.as_mut_ptr(), data.len()) }
    }

    pub fn uart_try_read(&mut self, data: &mut [u8]) -> Result<usize, Error> {
        let read = self.uart_read(data);
        if read > 0 || data.is_empty() {
            Ok(read)
        } else {
            Err(Error::Unavailable("cardputer grove uart"))
        }
    }

    pub fn uart_write(&mut self, data: &[u8]) -> usize {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_write(data.as_ptr(), data.len()) }
    }

    pub fn uart_try_write(&mut self, data: &[u8]) -> Result<usize, Error> {
        let written = self.uart_write(data);
        if written > 0 || data.is_empty() {
            Ok(written)
        } else {
            Err(Error::Unavailable("cardputer grove uart"))
        }
    }

    pub fn uart_try_write_all(&mut self, data: &[u8]) -> Result<(), Error> {
        (self.uart_try_write(data)? == data.len())
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove uart"))
    }

    pub fn uart_write_byte(&mut self, byte: u8) -> usize {
        self.uart_write(&[byte])
    }

    pub fn uart_try_write_byte(&mut self, byte: u8) -> Result<(), Error> {
        (self.uart_try_write(&[byte])? == 1)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove uart"))
    }

    pub fn uart_write_str(&mut self, text: &str) -> usize {
        self.uart_write(text.as_bytes())
    }

    pub fn uart_try_write_str(&mut self, text: &str) -> Result<usize, Error> {
        self.uart_try_write(text.as_bytes())
    }

    pub fn uart_try_write_str_all(&mut self, text: &str) -> Result<(), Error> {
        self.uart_try_write_all(text.as_bytes())
    }

    pub fn uart_flush(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_flush() }
    }
}

impl CardputerSpi {
    pub fn begin_with(&mut self, pins: SpiPins) -> bool {
        spi_begin(pins)
    }

    pub fn try_begin_with(&mut self, pins: SpiPins) -> Result<(), Error> {
        self.begin_with(pins)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer spi"))
    }

    pub fn begin_sd_bus(&mut self) -> bool {
        self.begin_with(SpiPins::CARDPUTER_SD)
    }

    pub fn try_begin_sd_bus(&mut self) -> Result<(), Error> {
        self.begin_sd_bus()
            .then_some(())
            .ok_or(Error::Unavailable("cardputer spi"))
    }

    pub fn end(&mut self) {
        spi_end()
    }

    pub fn transfer_byte(&mut self, value: u8, config: SpiConfig) -> u8 {
        spi_transfer_byte(value, config)
    }

    pub fn try_transfer_byte(&mut self, value: u8, config: SpiConfig) -> Result<u8, Error> {
        spi_try_transfer_byte(value, config)
    }

    pub fn transfer(&mut self, tx: &[u8], rx: &mut [u8], config: SpiConfig) -> bool {
        spi_transfer(tx, rx, config)
    }

    pub fn try_transfer(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
        config: SpiConfig,
    ) -> Result<(), Error> {
        validate_spi_transfer(tx, rx, config)?;
        self.transfer(tx, rx, config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer spi"))
    }

    pub fn read(&mut self, rx: &mut [u8], config: SpiConfig) -> bool {
        spi_read(rx, config)
    }

    pub fn try_read(&mut self, rx: &mut [u8], config: SpiConfig) -> Result<(), Error> {
        validate_spi_config(config)?;
        self.read(rx, config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer spi"))
    }

    pub fn write(&mut self, data: &[u8], config: SpiConfig) -> bool {
        spi_write(data, config)
    }

    pub fn try_write(&mut self, data: &[u8], config: SpiConfig) -> Result<(), Error> {
        validate_spi_config(config)?;
        self.write(data, config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer spi"))
    }

    pub fn try_write_all(&mut self, data: &[u8], config: SpiConfig) -> Result<(), Error> {
        validate_spi_config(config)?;
        self.write(data, config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer spi"))
    }
}

#[derive(Debug)]
pub struct Display;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DisplayFont {
    Font0 = 0,
    Font2 = 1,
    Font4 = 2,
    Font6 = 3,
    Font7 = 4,
    Font8 = 5,
    Font8x8C64 = 6,
    AsciiFont8x16 = 7,
    AsciiFont24x48 = 8,
    TomThumb = 9,
    FreeMono9pt7b = 10,
    FreeMono12pt7b = 11,
    FreeMono18pt7b = 12,
    FreeMono24pt7b = 13,
    FreeMonoBold9pt7b = 14,
    FreeMonoBold12pt7b = 15,
    FreeMonoBold18pt7b = 16,
    FreeMonoBold24pt7b = 17,
    FreeMonoOblique9pt7b = 18,
    FreeMonoOblique12pt7b = 19,
    FreeMonoOblique18pt7b = 20,
    FreeMonoOblique24pt7b = 21,
    FreeMonoBoldOblique9pt7b = 22,
    FreeMonoBoldOblique12pt7b = 23,
    FreeMonoBoldOblique18pt7b = 24,
    FreeMonoBoldOblique24pt7b = 25,
    FreeSans9pt7b = 26,
    FreeSans12pt7b = 27,
    FreeSans18pt7b = 28,
    FreeSans24pt7b = 29,
    FreeSansBold9pt7b = 30,
    FreeSansBold12pt7b = 31,
    FreeSansBold18pt7b = 32,
    FreeSansBold24pt7b = 33,
    FreeSansOblique9pt7b = 34,
    FreeSansOblique12pt7b = 35,
    FreeSansOblique18pt7b = 36,
    FreeSansOblique24pt7b = 37,
    FreeSansBoldOblique9pt7b = 38,
    FreeSansBoldOblique12pt7b = 39,
    FreeSansBoldOblique18pt7b = 40,
    FreeSansBoldOblique24pt7b = 41,
    FreeSerif9pt7b = 42,
    FreeSerif12pt7b = 43,
    FreeSerif18pt7b = 44,
    FreeSerif24pt7b = 45,
    FreeSerifItalic9pt7b = 46,
    FreeSerifItalic12pt7b = 47,
    FreeSerifItalic18pt7b = 48,
    FreeSerifItalic24pt7b = 49,
    FreeSerifBold9pt7b = 50,
    FreeSerifBold12pt7b = 51,
    FreeSerifBold18pt7b = 52,
    FreeSerifBold24pt7b = 53,
    FreeSerifBoldItalic9pt7b = 54,
    FreeSerifBoldItalic12pt7b = 55,
    FreeSerifBoldItalic18pt7b = 56,
    FreeSerifBoldItalic24pt7b = 57,
    OrbitronLight24 = 58,
    OrbitronLight32 = 59,
    RobotoThin24 = 60,
    Satisfy24 = 61,
    Yellowtail32 = 62,
    DejaVu9 = 63,
    DejaVu12 = 64,
    DejaVu18 = 65,
    DejaVu24 = 66,
    DejaVu40 = 67,
    DejaVu56 = 68,
    DejaVu72 = 69,
}

impl DisplayFont {
    pub const ALL: [Self; 70] = [
        Self::Font0,
        Self::Font2,
        Self::Font4,
        Self::Font6,
        Self::Font7,
        Self::Font8,
        Self::Font8x8C64,
        Self::AsciiFont8x16,
        Self::AsciiFont24x48,
        Self::TomThumb,
        Self::FreeMono9pt7b,
        Self::FreeMono12pt7b,
        Self::FreeMono18pt7b,
        Self::FreeMono24pt7b,
        Self::FreeMonoBold9pt7b,
        Self::FreeMonoBold12pt7b,
        Self::FreeMonoBold18pt7b,
        Self::FreeMonoBold24pt7b,
        Self::FreeMonoOblique9pt7b,
        Self::FreeMonoOblique12pt7b,
        Self::FreeMonoOblique18pt7b,
        Self::FreeMonoOblique24pt7b,
        Self::FreeMonoBoldOblique9pt7b,
        Self::FreeMonoBoldOblique12pt7b,
        Self::FreeMonoBoldOblique18pt7b,
        Self::FreeMonoBoldOblique24pt7b,
        Self::FreeSans9pt7b,
        Self::FreeSans12pt7b,
        Self::FreeSans18pt7b,
        Self::FreeSans24pt7b,
        Self::FreeSansBold9pt7b,
        Self::FreeSansBold12pt7b,
        Self::FreeSansBold18pt7b,
        Self::FreeSansBold24pt7b,
        Self::FreeSansOblique9pt7b,
        Self::FreeSansOblique12pt7b,
        Self::FreeSansOblique18pt7b,
        Self::FreeSansOblique24pt7b,
        Self::FreeSansBoldOblique9pt7b,
        Self::FreeSansBoldOblique12pt7b,
        Self::FreeSansBoldOblique18pt7b,
        Self::FreeSansBoldOblique24pt7b,
        Self::FreeSerif9pt7b,
        Self::FreeSerif12pt7b,
        Self::FreeSerif18pt7b,
        Self::FreeSerif24pt7b,
        Self::FreeSerifItalic9pt7b,
        Self::FreeSerifItalic12pt7b,
        Self::FreeSerifItalic18pt7b,
        Self::FreeSerifItalic24pt7b,
        Self::FreeSerifBold9pt7b,
        Self::FreeSerifBold12pt7b,
        Self::FreeSerifBold18pt7b,
        Self::FreeSerifBold24pt7b,
        Self::FreeSerifBoldItalic9pt7b,
        Self::FreeSerifBoldItalic12pt7b,
        Self::FreeSerifBoldItalic18pt7b,
        Self::FreeSerifBoldItalic24pt7b,
        Self::OrbitronLight24,
        Self::OrbitronLight32,
        Self::RobotoThin24,
        Self::Satisfy24,
        Self::Yellowtail32,
        Self::DejaVu9,
        Self::DejaVu12,
        Self::DejaVu18,
        Self::DejaVu24,
        Self::DejaVu40,
        Self::DejaVu56,
        Self::DejaVu72,
    ];

    pub fn from_raw(raw: i32) -> Option<Self> {
        let index = usize::try_from(raw).ok()?;
        Self::ALL.get(index).copied()
    }

    pub const fn raw(self) -> c_int {
        self as c_int
    }
}

impl Display {
    pub fn canvas(&mut self) -> Option<Canvas> {
        Canvas::for_display()
    }

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

    pub fn draw_bmp(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_display_draw_bmp(
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }

    pub fn try_draw_bmp(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_bmp(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("display bmp"))
    }

    pub fn draw_jpg(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_display_draw_jpg(
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }

    pub fn try_draw_jpg(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_jpg(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("display jpg"))
    }

    pub fn draw_png(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_display_draw_png(
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }

    pub fn try_draw_png(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_png(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("display png"))
    }

    pub fn push_image_rgb565(&mut self, rect: Rect, data: &[u16]) -> bool {
        let Some(pixel_count) = rect
            .w
            .checked_mul(rect.h)
            .and_then(|count| usize::try_from(count).ok())
        else {
            return false;
        };
        if rect.w <= 0 || rect.h <= 0 || data.len() < pixel_count {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_display_push_image_rgb565(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                data.as_ptr(),
                data.len(),
            )
        }
    }

    pub fn try_push_image_rgb565(&mut self, rect: Rect, data: &[u16]) -> Result<(), Error> {
        validate_rgb565_image(rect, data)?;
        self.push_image_rgb565(rect, data)
            .then_some(())
            .ok_or(Error::Unavailable("display rgb565 image"))
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_set_cursor(x as c_int, y as c_int) }
    }

    pub fn set_text_size(&mut self, size: i32) {
        unsafe { m5unified_sys::m5u_display_set_text_size(size as c_int) }
    }

    pub fn try_set_text_size(&mut self, size: i32) -> Result<(), Error> {
        validate_display_text_size(size)?;
        self.set_text_size(size);
        Ok(())
    }

    pub fn set_font(&mut self, font: DisplayFont) -> bool {
        unsafe { m5unified_sys::m5u_display_set_font(font.raw()) }
    }

    pub fn try_set_font(&mut self, font: DisplayFont) -> Result<(), Error> {
        self.set_font(font)
            .then_some(())
            .ok_or(Error::Unavailable("display font"))
    }

    pub fn show_font(&mut self, duration_ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_display_show_font(duration_ms) }
    }

    pub fn try_show_font(&mut self, duration_ms: u32) -> Result<(), Error> {
        self.show_font(duration_ms)
            .then_some(())
            .ok_or(Error::Unavailable("display font"))
    }

    pub fn unload_font(&mut self) {
        unsafe { m5unified_sys::m5u_display_unload_font() }
    }

    pub fn font_height_for(&self, font: DisplayFont) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_height_for(font.raw()) as i32 }
    }

    pub fn font_width_for(&self, font: DisplayFont) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_width_for(font.raw()) as i32 }
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

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_pixel(x, y, color) }
    }

    pub fn draw_point(&mut self, point: Point, color: u16) {
        self.draw_pixel(point.x, point.y, color)
    }

    pub fn draw_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_fast_hline(x, y, w, color) }
    }

    pub fn try_draw_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) -> Result<(), Error> {
        validate_display_length(w)?;
        self.draw_fast_hline(x, y, w, color);
        Ok(())
    }

    pub fn draw_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_fast_vline(x, y, h, color) }
    }

    pub fn try_draw_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) -> Result<(), Error> {
        validate_display_length(h)?;
        self.draw_fast_vline(x, y, h, color);
        Ok(())
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_rect(x, y, w, h, color) }
    }

    pub fn try_draw_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.draw_rect(rect.x, rect.y, rect.w, rect.h, color);
        Ok(())
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_rect(x, y, w, h, color) }
    }

    pub fn try_fill_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.fill_rect(rect.x, rect.y, rect.w, rect.h, color);
        Ok(())
    }

    pub fn fill_rect_alpha(&mut self, x: i32, y: i32, w: i32, h: i32, alpha: u8, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_rect_alpha(x, y, w, h, alpha, color) }
    }

    pub fn try_fill_rect_alpha(&mut self, rect: Rect, alpha: u8, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.fill_rect_alpha(rect.x, rect.y, rect.w, rect.h, alpha, color);
        Ok(())
    }

    pub fn draw_round_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_round_rect(x, y, w, h, r, color) }
    }

    pub fn try_draw_round_rect(
        &mut self,
        rect: Rect,
        radius: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_rect(rect)?;
        validate_display_radius(radius)?;
        self.draw_round_rect(rect.x, rect.y, rect.w, rect.h, radius, color);
        Ok(())
    }

    pub fn fill_round_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_round_rect(x, y, w, h, r, color) }
    }

    pub fn try_fill_round_rect(
        &mut self,
        rect: Rect,
        radius: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_rect(rect)?;
        validate_display_radius(radius)?;
        self.fill_round_rect(rect.x, rect.y, rect.w, rect.h, radius, color);
        Ok(())
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_circle(x, y, r, color) }
    }

    pub fn try_draw_circle(&mut self, x: i32, y: i32, r: i32, color: u16) -> Result<(), Error> {
        validate_display_radius(r)?;
        self.draw_circle(x, y, r, color);
        Ok(())
    }

    pub fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_circle(x, y, r, color) }
    }

    pub fn try_fill_circle(&mut self, x: i32, y: i32, r: i32, color: u16) -> Result<(), Error> {
        validate_display_radius(r)?;
        self.fill_circle(x, y, r, color);
        Ok(())
    }

    pub fn draw_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_ellipse(x, y, rx, ry, color) }
    }

    pub fn try_draw_ellipse(
        &mut self,
        x: i32,
        y: i32,
        rx: i32,
        ry: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_radii(rx, ry)?;
        self.draw_ellipse(x, y, rx, ry, color);
        Ok(())
    }

    pub fn fill_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_ellipse(x, y, rx, ry, color) }
    }

    pub fn try_fill_ellipse(
        &mut self,
        x: i32,
        y: i32,
        rx: i32,
        ry: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_radii(rx, ry)?;
        self.fill_ellipse(x, y, rx, ry, color);
        Ok(())
    }

    pub fn draw_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_draw_arc(
                center.x,
                center.y,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                color,
            )
        }
    }

    pub fn try_draw_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_arc(inner_radius, outer_radius, start_angle, end_angle)?;
        self.draw_arc(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            color,
        );
        Ok(())
    }

    pub fn fill_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_fill_arc(
                center.x,
                center.y,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                color,
            )
        }
    }

    pub fn try_fill_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_arc(inner_radius, outer_radius, start_angle, end_angle)?;
        self.fill_arc(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            color,
        );
        Ok(())
    }

    pub fn draw_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_triangle(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color)
        }
    }

    pub fn fill_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_triangle(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color)
        }
    }

    pub fn progress_bar(&mut self, rect: Rect, value: u8) {
        unsafe { m5unified_sys::m5u_display_progress_bar(rect.x, rect.y, rect.w, rect.h, value) }
    }

    pub fn try_progress_bar(&mut self, rect: Rect, value: u8) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.progress_bar(rect, value);
        Ok(())
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        unsafe { m5unified_sys::m5u_display_set_rotation(rotation) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonId {
    A,
    B,
    C,
    Pwr,
    Ext,
}

impl ButtonId {
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::A),
            1 => Some(Self::B),
            2 => Some(Self::C),
            3 => Some(Self::Pwr),
            4 => Some(Self::Ext),
            _ => None,
        }
    }

    pub const fn raw(self) -> c_int {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::Pwr => 3,
            Self::Ext => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonState {
    NoChange,
    Clicked,
    Hold,
    DecideClickCount,
    Other(i32),
}

impl ButtonState {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::NoChange,
            1 => Self::Clicked,
            2 => Self::Hold,
            3 => Self::DecideClickCount,
            raw => Self::Other(raw),
        }
    }

    pub const fn raw(self) -> Option<i32> {
        match self {
            Self::NoChange => Some(0),
            Self::Clicked => Some(1),
            Self::Hold => Some(2),
            Self::DecideClickCount => Some(3),
            Self::Other(_) => None,
        }
    }

    pub const fn raw_value(self) -> i32 {
        match self {
            Self::NoChange => 0,
            Self::Clicked => 1,
            Self::Hold => 2,
            Self::DecideClickCount => 3,
            Self::Other(raw) => raw,
        }
    }

    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Other(_))
    }

    pub const fn is_no_change(self) -> bool {
        matches!(self, Self::NoChange)
    }

    pub const fn is_clicked(self) -> bool {
        matches!(self, Self::Clicked)
    }

    pub const fn is_hold(self) -> bool {
        matches!(self, Self::Hold)
    }

    pub const fn is_decide_click_count(self) -> bool {
        matches!(self, Self::DecideClickCount)
    }

    pub const fn is_event(self) -> bool {
        matches!(self, Self::Clicked | Self::Hold | Self::DecideClickCount)
    }
}

#[derive(Debug)]
pub struct Buttons;

impl Buttons {
    pub fn button(&self, id: ButtonId) -> Button {
        Button { id }
    }

    pub fn a(&self) -> Button {
        self.button(ButtonId::A)
    }

    pub fn b(&self) -> Button {
        self.button(ButtonId::B)
    }

    pub fn c(&self) -> Button {
        self.button(ButtonId::C)
    }

    pub fn pwr(&self) -> Button {
        self.button(ButtonId::Pwr)
    }

    pub fn ext(&self) -> Button {
        self.button(ButtonId::Ext)
    }

    pub fn a_is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_a_is_pressed() }
    }

    pub fn a_was_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_a_was_pressed() }
    }

    pub fn a_was_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_a_was_released() }
    }

    pub fn b_is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_b_is_pressed() }
    }

    pub fn b_was_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_b_was_pressed() }
    }

    pub fn b_was_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_b_was_released() }
    }

    pub fn c_is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_c_is_pressed() }
    }

    pub fn c_was_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_c_was_pressed() }
    }

    pub fn c_was_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_btn_c_was_released() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Button {
    id: ButtonId,
}

impl Button {
    fn raw_id(&self) -> c_int {
        self.id.raw()
    }

    pub fn id(&self) -> ButtonId {
        self.id
    }

    pub fn is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_pressed(self.raw_id()) }
    }

    pub fn was_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_pressed(self.raw_id()) }
    }

    pub fn was_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_released(self.raw_id()) }
    }

    pub fn was_clicked(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_clicked(self.raw_id()) }
    }

    pub fn was_hold(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_hold(self.raw_id()) }
    }

    pub fn was_single_clicked(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_single_clicked(self.raw_id()) }
    }

    pub fn was_double_clicked(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_double_clicked(self.raw_id()) }
    }

    pub fn was_change_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_change_pressed(self.raw_id()) }
    }

    pub fn is_holding(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_holding(self.raw_id()) }
    }

    pub fn is_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_released(self.raw_id()) }
    }

    pub fn was_released_after_hold(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_released_after_hold(self.raw_id()) }
    }

    pub fn was_release_for(&self, ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_button_was_release_for(self.raw_id(), ms) }
    }

    pub fn pressed_for(&self, ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_button_pressed_for(self.raw_id(), ms) }
    }

    pub fn released_for(&self, ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_button_released_for(self.raw_id(), ms) }
    }

    pub fn was_decide_click_count(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_decide_click_count(self.raw_id()) }
    }

    pub fn click_count(&self) -> i32 {
        unsafe { m5unified_sys::m5u_button_get_click_count(self.raw_id()) as i32 }
    }

    pub fn set_debounce_thresh(&mut self, msec: u32) {
        unsafe { m5unified_sys::m5u_button_set_debounce_thresh(self.raw_id(), msec) }
    }

    pub fn try_set_debounce_thresh(&mut self, msec: u32) -> Result<(), Error> {
        self.set_debounce_thresh(msec);
        Ok(())
    }

    pub fn set_hold_thresh(&mut self, msec: u32) {
        unsafe { m5unified_sys::m5u_button_set_hold_thresh(self.raw_id(), msec) }
    }

    pub fn try_set_hold_thresh(&mut self, msec: u32) -> Result<(), Error> {
        self.set_hold_thresh(msec);
        Ok(())
    }

    pub fn set_raw_state(&mut self, msec: u32, press: bool) {
        unsafe { m5unified_sys::m5u_button_set_raw_state(self.raw_id(), msec, press) }
    }

    pub fn try_set_raw_state(&mut self, msec: u32, press: bool) -> Result<(), Error> {
        self.set_raw_state(msec, press);
        Ok(())
    }

    pub fn set_state(&mut self, msec: u32, state: ButtonState) {
        if let Some(state) = state.raw() {
            unsafe { m5unified_sys::m5u_button_set_state(self.raw_id(), msec, state) }
        }
    }

    pub fn try_set_state(&mut self, msec: u32, state: ButtonState) -> Result<(), Error> {
        let Some(state) = state.raw() else {
            return Err(Error::InvalidValue("button state"));
        };
        unsafe { m5unified_sys::m5u_button_set_state(self.raw_id(), msec, state) }
        Ok(())
    }

    pub fn state(&self) -> ButtonState {
        ButtonState::from_raw(unsafe { m5unified_sys::m5u_button_get_state(self.raw_id()) })
    }

    pub fn last_change(&self) -> u32 {
        unsafe { m5unified_sys::m5u_button_last_change(self.raw_id()) }
    }

    pub fn debounce_thresh(&self) -> u32 {
        unsafe { m5unified_sys::m5u_button_get_debounce_thresh(self.raw_id()) }
    }

    pub fn hold_thresh(&self) -> u32 {
        unsafe { m5unified_sys::m5u_button_get_hold_thresh(self.raw_id()) }
    }

    pub fn update_msec(&self) -> u32 {
        unsafe { m5unified_sys::m5u_button_get_update_msec(self.raw_id()) }
    }
}

#[derive(Debug)]
pub struct Mic;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct MicStats {
    pub min: i16,
    pub max: i16,
    pub peak: u16,
    pub mean: f32,
    pub rms: f32,
}

impl MicStats {
    pub const I16_FULL_SCALE: f32 = 32768.0;

    pub const fn dynamic_range(self) -> i32 {
        self.max as i32 - self.min as i32
    }

    pub fn peak_fraction(self) -> f32 {
        self.peak as f32 / Self::I16_FULL_SCALE
    }

    pub fn rms_fraction(self) -> f32 {
        self.rms / Self::I16_FULL_SCALE
    }

    pub fn peak_percent(self) -> f32 {
        self.peak_fraction() * 100.0
    }

    pub fn rms_percent(self) -> f32 {
        self.rms_fraction() * 100.0
    }

    pub fn is_silent(self) -> bool {
        self.peak == 0
    }
}

pub fn analyze_i16_samples(samples: &[i16]) -> Option<MicStats> {
    let (&first, rest) = samples.split_first()?;
    let mut min = first;
    let mut max = first;
    let mut peak = i32::from(first).unsigned_abs() as u16;
    let mut sum = i64::from(first);
    let mut sum_squares = {
        let sample = f64::from(first);
        sample * sample
    };

    for &sample in rest {
        min = min.min(sample);
        max = max.max(sample);
        peak = peak.max(i32::from(sample).unsigned_abs() as u16);
        sum += i64::from(sample);

        let sample = f64::from(sample);
        sum_squares += sample * sample;
    }

    let len = samples.len() as f64;
    Some(MicStats {
        min,
        max,
        peak,
        mean: (sum as f64 / len) as f32,
        rms: (sum_squares / len).sqrt() as f32,
    })
}

pub fn rms_i16_samples(samples: &[i16]) -> Option<f32> {
    analyze_i16_samples(samples).map(|stats| stats.rms)
}

impl Mic {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_mic_begin() }
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.begin()
            .then_some(())
            .ok_or(Error::Unavailable("microphone"))
    }

    pub fn record_i16(&mut self, buffer: &mut [i16]) -> bool {
        unsafe { m5unified_sys::m5u_mic_record_i16(buffer.as_mut_ptr(), buffer.len()) }
    }

    pub fn try_record_i16(&mut self, buffer: &mut [i16]) -> Result<(), Error> {
        validate_audio_data(buffer.len())?;
        self.record_i16(buffer)
            .then_some(())
            .ok_or(Error::Unavailable("microphone recording"))
    }

    pub fn rms(&mut self, buffer: &mut [i16]) -> Option<f32> {
        if !self.record_i16(buffer) {
            return None;
        }
        rms_i16_samples(buffer)
    }

    pub fn try_rms(&mut self, buffer: &mut [i16]) -> Result<f32, Error> {
        self.try_record_i16(buffer)?;
        Ok(rms_i16_samples(buffer).unwrap_or_default())
    }

    pub fn stats(&mut self, buffer: &mut [i16]) -> Option<MicStats> {
        if !self.record_i16(buffer) {
            return None;
        }
        analyze_i16_samples(buffer)
    }

    pub fn try_stats(&mut self, buffer: &mut [i16]) -> Result<MicStats, Error> {
        self.try_record_i16(buffer)?;
        Ok(analyze_i16_samples(buffer).unwrap_or_default())
    }
}

#[derive(Debug)]
pub struct Speaker;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpeakerConfig {
    pub pin_data_out: i32,
    pub pin_bck: i32,
    pub pin_ws: i32,
    pub sample_rate: u32,
    pub stereo: bool,
    pub buzzer: bool,
    pub use_dac: bool,
    pub dac_zero_level: u8,
    pub magnification: u8,
    pub dma_buf_len: usize,
    pub dma_buf_count: usize,
    pub task_priority: u8,
    pub task_pinned_core: u8,
    pub i2s_port: i32,
}

impl Default for SpeakerConfig {
    fn default() -> Self {
        Self {
            pin_data_out: -1,
            pin_bck: -1,
            pin_ws: -1,
            sample_rate: 48_000,
            stereo: false,
            buzzer: false,
            use_dac: false,
            dac_zero_level: 0,
            magnification: 16,
            dma_buf_len: 256,
            dma_buf_count: 8,
            task_priority: 2,
            task_pinned_core: u8::MAX,
            i2s_port: 0,
        }
    }
}

impl SpeakerConfig {
    pub const fn with_pins(mut self, pin_data_out: i32, pin_bck: i32, pin_ws: i32) -> Self {
        self.pin_data_out = pin_data_out;
        self.pin_bck = pin_bck;
        self.pin_ws = pin_ws;
        self
    }

    pub const fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub const fn with_stereo(mut self, stereo: bool) -> Self {
        self.stereo = stereo;
        self
    }

    pub const fn with_buzzer(mut self, buzzer: bool) -> Self {
        self.buzzer = buzzer;
        self
    }

    pub const fn with_dac(mut self, use_dac: bool) -> Self {
        self.use_dac = use_dac;
        self
    }

    pub const fn with_dac_zero_level(mut self, dac_zero_level: u8) -> Self {
        self.dac_zero_level = dac_zero_level;
        self
    }

    pub const fn with_magnification(mut self, magnification: u8) -> Self {
        self.magnification = magnification;
        self
    }

    pub const fn with_dma_buffer(mut self, len: usize, count: usize) -> Self {
        self.dma_buf_len = len;
        self.dma_buf_count = count;
        self
    }

    pub const fn with_task(mut self, priority: u8, pinned_core: u8) -> Self {
        self.task_priority = priority;
        self.task_pinned_core = pinned_core;
        self
    }

    pub const fn with_i2s_port(mut self, i2s_port: i32) -> Self {
        self.i2s_port = i2s_port;
        self
    }

    pub const fn pins(self) -> (i32, i32, i32) {
        (self.pin_data_out, self.pin_bck, self.pin_ws)
    }

    pub const fn sample_rate(self) -> u32 {
        self.sample_rate
    }

    pub const fn is_stereo(self) -> bool {
        self.stereo
    }

    pub const fn is_buzzer(self) -> bool {
        self.buzzer
    }

    pub const fn uses_dac(self) -> bool {
        self.use_dac
    }

    pub const fn dac_zero_level(self) -> u8 {
        self.dac_zero_level
    }

    pub const fn magnification(self) -> u8 {
        self.magnification
    }

    pub const fn dma_buffer(self) -> (usize, usize) {
        (self.dma_buf_len, self.dma_buf_count)
    }

    pub const fn task(self) -> (u8, u8) {
        (self.task_priority, self.task_pinned_core)
    }

    pub const fn i2s_port(self) -> i32 {
        self.i2s_port
    }
}

impl From<m5unified_sys::m5u_speaker_config_t> for SpeakerConfig {
    fn from(raw: m5unified_sys::m5u_speaker_config_t) -> Self {
        Self {
            pin_data_out: raw.pin_data_out,
            pin_bck: raw.pin_bck,
            pin_ws: raw.pin_ws,
            sample_rate: raw.sample_rate,
            stereo: raw.stereo,
            buzzer: raw.buzzer,
            use_dac: raw.use_dac,
            dac_zero_level: raw.dac_zero_level,
            magnification: raw.magnification,
            dma_buf_len: raw.dma_buf_len,
            dma_buf_count: raw.dma_buf_count,
            task_priority: raw.task_priority,
            task_pinned_core: raw.task_pinned_core,
            i2s_port: raw.i2s_port,
        }
    }
}

impl From<SpeakerConfig> for m5unified_sys::m5u_speaker_config_t {
    fn from(config: SpeakerConfig) -> Self {
        Self {
            pin_data_out: config.pin_data_out,
            pin_bck: config.pin_bck,
            pin_ws: config.pin_ws,
            sample_rate: config.sample_rate,
            stereo: config.stereo,
            buzzer: config.buzzer,
            use_dac: config.use_dac,
            dac_zero_level: config.dac_zero_level,
            magnification: config.magnification,
            dma_buf_len: config.dma_buf_len,
            dma_buf_count: config.dma_buf_count,
            task_priority: config.task_priority,
            task_pinned_core: config.task_pinned_core,
            i2s_port: config.i2s_port,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AudioPlaybackOptions {
    pub stereo: bool,
    pub repeat: u32,
    pub channel: Option<u8>,
    pub stop_current_sound: bool,
}

impl Default for AudioPlaybackOptions {
    fn default() -> Self {
        Self {
            stereo: false,
            repeat: 1,
            channel: None,
            stop_current_sound: false,
        }
    }
}

impl AudioPlaybackOptions {
    pub const fn new() -> Self {
        Self {
            stereo: false,
            repeat: 1,
            channel: None,
            stop_current_sound: false,
        }
    }

    pub const fn with_stereo(mut self, stereo: bool) -> Self {
        self.stereo = stereo;
        self
    }

    pub const fn with_repeat(mut self, repeat: u32) -> Self {
        self.repeat = repeat;
        self
    }

    pub const fn with_channel(mut self, channel: Option<u8>) -> Self {
        self.channel = channel;
        self
    }

    pub const fn with_stop_current_sound(mut self, stop_current_sound: bool) -> Self {
        self.stop_current_sound = stop_current_sound;
        self
    }

    pub const fn is_stereo(self) -> bool {
        self.stereo
    }

    pub const fn repeat(self) -> u32 {
        self.repeat
    }

    pub const fn channel(self) -> Option<u8> {
        self.channel
    }

    pub const fn stop_current_sound(self) -> bool {
        self.stop_current_sound
    }

    pub const fn targets_all_channels(self) -> bool {
        self.channel.is_none()
    }
}

impl Speaker {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_speaker_begin() }
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.begin()
            .then_some(())
            .ok_or(Error::Unavailable("speaker"))
    }

    pub fn set_volume(&mut self, volume: u8) {
        unsafe { m5unified_sys::m5u_speaker_set_volume(volume) }
    }

    pub fn tone(&mut self, frequency_hz: u32, duration_ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_speaker_tone(frequency_hz, duration_ms) }
    }

    pub fn try_tone(&mut self, frequency_hz: u32, duration_ms: u32) -> Result<(), Error> {
        validate_audio_frequency_hz(frequency_hz)?;
        self.tone(frequency_hz, duration_ms)
            .then_some(())
            .ok_or(Error::Unavailable("speaker tone"))
    }

    pub fn play_i16(&mut self, samples: &[i16], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_i16(samples.as_ptr(), samples.len(), sample_rate_hz)
        }
    }

    pub fn try_play_i16(&mut self, samples: &[i16], sample_rate_hz: u32) -> Result<(), Error> {
        validate_audio_data(samples.len())?;
        validate_audio_sample_rate(sample_rate_hz)?;
        self.play_i16(samples, sample_rate_hz)
            .then_some(())
            .ok_or(Error::Unavailable("speaker playback"))
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn components(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn magnitude_squared(self) -> f32 {
        self.x
            .mul_add(self.x, self.y.mul_add(self.y, self.z * self.z))
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn scale(self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x
            .mul_add(other.x, self.y.mul_add(other.y, self.z * other.z))
    }

    pub fn normalized(self) -> Option<Self> {
        let magnitude = self.magnitude();
        if magnitude > 0.0 && magnitude.is_finite() {
            Some(self.scale(1.0 / magnitude))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Imu;

impl Imu {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_begin() }
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.begin().then_some(()).ok_or(Error::Unavailable("imu"))
    }

    pub fn accel(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_accel(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn try_accel(&self) -> Result<Vec3, Error> {
        self.accel().ok_or(Error::Unavailable("imu accel"))
    }

    pub fn gyro(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_gyro(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn try_gyro(&self) -> Result<Vec3, Error> {
        self.gyro().ok_or(Error::Unavailable("imu gyro"))
    }

    pub fn mag(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_mag(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn try_mag(&self) -> Result<Vec3, Error> {
        self.mag().ok_or(Error::Unavailable("imu magnetometer"))
    }

    pub fn temperature_c(&self) -> Option<f32> {
        let mut temp = 0.0;
        let ok = unsafe { m5unified_sys::m5u_imu_get_temp_c(&mut temp) };
        ok.then_some(temp)
    }

    pub fn try_temperature_c(&self) -> Result<f32, Error> {
        self.temperature_c()
            .ok_or(Error::Unavailable("imu temperature"))
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct TouchPoint {
    pub x: i32,
    pub y: i32,
    pub size: u16,
    pub id: u8,
}

impl TouchPoint {
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            size: 0,
            id: 0,
        }
    }

    pub const fn with_size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    pub const fn with_id(mut self, id: u8) -> Self {
        self.id = id;
        self
    }

    pub const fn position(self) -> Point {
        Point::new(self.x, self.y)
    }

    pub const fn size(self) -> u16 {
        self.size
    }

    pub const fn id(self) -> u8 {
        self.id
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    Other(i32),
}

impl TouchState {
    pub fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Touch,
            2 => Self::TouchEnd,
            3 => Self::TouchBegin,
            5 => Self::Hold,
            6 => Self::HoldEnd,
            7 => Self::HoldBegin,
            9 => Self::Flick,
            10 => Self::FlickEnd,
            11 => Self::FlickBegin,
            13 => Self::Drag,
            14 => Self::DragEnd,
            15 => Self::DragBegin,
            raw => Self::Other(raw),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Touch => 1,
            Self::TouchEnd => 2,
            Self::TouchBegin => 3,
            Self::Hold => 5,
            Self::HoldEnd => 6,
            Self::HoldBegin => 7,
            Self::Flick => 9,
            Self::FlickEnd => 10,
            Self::FlickBegin => 11,
            Self::Drag => 13,
            Self::DragEnd => 14,
            Self::DragBegin => 15,
            Self::Other(raw) => raw,
        }
    }

    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    pub const fn is_touch(self) -> bool {
        matches!(self, Self::Touch | Self::TouchBegin | Self::TouchEnd)
    }

    pub const fn is_hold(self) -> bool {
        matches!(self, Self::Hold | Self::HoldBegin | Self::HoldEnd)
    }

    pub const fn is_flick(self) -> bool {
        matches!(self, Self::Flick | Self::FlickBegin | Self::FlickEnd)
    }

    pub const fn is_drag(self) -> bool {
        matches!(self, Self::Drag | Self::DragBegin | Self::DragEnd)
    }

    pub const fn is_begin(self) -> bool {
        matches!(
            self,
            Self::TouchBegin | Self::HoldBegin | Self::FlickBegin | Self::DragBegin
        )
    }

    pub const fn is_end(self) -> bool {
        matches!(
            self,
            Self::TouchEnd | Self::HoldEnd | Self::FlickEnd | Self::DragEnd
        )
    }
}

#[derive(Debug)]
pub struct Touch;

impl Touch {
    pub fn begin(&mut self) {
        unsafe { m5unified_sys::m5u_touch_begin() }
    }

    pub fn update(&mut self, msec: u32) {
        unsafe { m5unified_sys::m5u_touch_update(msec) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_touch_is_enabled() }
    }

    pub fn end(&mut self) {
        unsafe { m5unified_sys::m5u_touch_end() }
    }

    pub fn count(&self) -> usize {
        unsafe { m5unified_sys::m5u_touch_count() }.max(0) as usize
    }

    pub fn points(&self) -> Vec<TouchPoint> {
        (0..self.count())
            .filter_map(|index| self.point(index))
            .collect()
    }

    pub fn point(&self, index: usize) -> Option<TouchPoint> {
        let index = touch_index_to_c_int(index).ok()?;
        let (mut x, mut y) = (0, 0);
        let ok = unsafe { m5unified_sys::m5u_touch_get(index, &mut x, &mut y) };
        ok.then_some(TouchPoint {
            x,
            y,
            size: 0,
            id: 0,
        })
    }

    pub fn try_point(&self, index: usize) -> Result<TouchPoint, Error> {
        touch_index_to_c_int(index)?;
        self.point(index).ok_or(Error::Unavailable("touch point"))
    }

    pub fn primary_point(&self) -> Option<TouchPoint> {
        self.points().into_iter().next()
    }

    pub fn try_primary_point(&self) -> Result<TouchPoint, Error> {
        self.primary_point()
            .ok_or(Error::Unavailable("touch point"))
    }

    pub fn raw_point(&self, index: usize) -> Option<TouchPoint> {
        let index = touch_index_to_c_int(index).ok()?;
        let mut raw = m5unified_sys::m5u_touch_point_t::default();
        let ok = unsafe { m5unified_sys::m5u_touch_get_raw(index, &mut raw) };
        ok.then_some(TouchPoint {
            x: raw.x,
            y: raw.y,
            size: raw.size,
            id: raw.id,
        })
    }

    pub fn try_raw_point(&self, index: usize) -> Result<TouchPoint, Error> {
        touch_index_to_c_int(index)?;
        self.raw_point(index)
            .ok_or(Error::Unavailable("touch raw point"))
    }

    pub fn raw_points(&self) -> Vec<TouchPoint> {
        (0..self.count())
            .filter_map(|index| self.raw_point(index))
            .collect()
    }

    pub fn primary_raw_point(&self) -> Option<TouchPoint> {
        self.raw_point(0)
    }

    pub fn try_primary_raw_point(&self) -> Result<TouchPoint, Error> {
        self.primary_raw_point()
            .ok_or(Error::Unavailable("touch raw point"))
    }

    pub fn is_pressed(&self) -> bool {
        self.count() > 0
    }

    pub fn set_hold_thresh(&mut self, msec: u16) {
        unsafe { m5unified_sys::m5u_touch_set_hold_thresh(msec) }
    }

    pub fn try_set_hold_thresh(&mut self, msec: u16) -> Result<(), Error> {
        if msec == 0 {
            return Err(Error::InvalidValue("touch hold threshold"));
        }
        self.set_hold_thresh(msec);
        Ok(())
    }

    pub fn set_flick_thresh(&mut self, distance: u16) {
        unsafe { m5unified_sys::m5u_touch_set_flick_thresh(distance) }
    }

    pub fn try_set_flick_thresh(&mut self, distance: u16) -> Result<(), Error> {
        if distance == 0 {
            return Err(Error::InvalidValue("touch flick threshold"));
        }
        self.set_flick_thresh(distance);
        Ok(())
    }
}

fn touch_index_to_c_int(index: usize) -> Result<c_int, Error> {
    c_int::try_from(index).map_err(|_| Error::InvalidValue("touch index"))
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct RtcDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub weekday: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RtcWeekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl RtcWeekday {
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Sunday),
            1 => Some(Self::Monday),
            2 => Some(Self::Tuesday),
            3 => Some(Self::Wednesday),
            4 => Some(Self::Thursday),
            5 => Some(Self::Friday),
            6 => Some(Self::Saturday),
            _ => None,
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::Sunday => 0,
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
        }
    }
}

impl RtcDate {
    pub const fn new(year: i32, month: i32, day: i32, weekday: i32) -> Self {
        Self {
            year,
            month,
            day,
            weekday,
        }
    }

    pub const fn new_with_weekday(year: i32, month: i32, day: i32, weekday: RtcWeekday) -> Self {
        Self::new(year, month, day, weekday.raw())
    }

    pub const fn year(self) -> i32 {
        self.year
    }

    pub const fn month(self) -> i32 {
        self.month
    }

    pub const fn day(self) -> i32 {
        self.day
    }

    pub const fn raw_weekday(self) -> i32 {
        self.weekday
    }

    pub const fn ymd(self) -> (i32, i32, i32) {
        (self.year, self.month, self.day)
    }

    pub const fn weekday(self) -> Option<RtcWeekday> {
        RtcWeekday::from_raw(self.weekday)
    }

    pub const fn with_weekday(self, weekday: RtcWeekday) -> Self {
        Self {
            weekday: weekday.raw(),
            ..self
        }
    }

    pub const fn is_valid(self) -> bool {
        self.year >= 1900
            && self.month >= 1
            && self.month <= 12
            && self.day >= 1
            && self.day <= days_in_month(self.year, self.month)
            && self.weekday >= 0
            && self.weekday <= 6
    }

    pub const fn with_time(self, time: RtcTime) -> DateTime {
        DateTime {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: time.hour,
            minute: time.minute,
            second: time.second,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct RtcTime {
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

impl RtcTime {
    pub const fn new(hour: i32, minute: i32, second: i32) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    pub const fn hour(self) -> i32 {
        self.hour
    }

    pub const fn minute(self) -> i32 {
        self.minute
    }

    pub const fn second(self) -> i32 {
        self.second
    }

    pub const fn hms(self) -> (i32, i32, i32) {
        (self.hour, self.minute, self.second)
    }

    pub const fn is_valid(self) -> bool {
        self.hour >= 0
            && self.hour <= 23
            && self.minute >= 0
            && self.minute <= 59
            && self.second >= 0
            && self.second <= 59
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

impl DateTime {
    pub const fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub const fn year(self) -> i32 {
        self.year
    }

    pub const fn month(self) -> i32 {
        self.month
    }

    pub const fn day(self) -> i32 {
        self.day
    }

    pub const fn hour(self) -> i32 {
        self.hour
    }

    pub const fn minute(self) -> i32 {
        self.minute
    }

    pub const fn second(self) -> i32 {
        self.second
    }

    pub const fn ymd(self) -> (i32, i32, i32) {
        (self.year, self.month, self.day)
    }

    pub const fn hms(self) -> (i32, i32, i32) {
        (self.hour, self.minute, self.second)
    }

    pub const fn date(self, weekday: i32) -> RtcDate {
        RtcDate {
            year: self.year,
            month: self.month,
            day: self.day,
            weekday,
        }
    }

    pub const fn date_with_weekday(self, weekday: RtcWeekday) -> RtcDate {
        self.date(weekday.raw())
    }

    pub const fn time(self) -> RtcTime {
        RtcTime {
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        }
    }

    pub const fn is_valid(self) -> bool {
        self.date(0).is_valid() && self.time().is_valid()
    }
}

pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub const fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

#[derive(Debug)]
pub struct Rtc;

impl Rtc {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_rtc_begin() }
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.begin().then_some(()).ok_or(Error::Unavailable("rtc"))
    }

    pub fn volt_low(&self) -> bool {
        unsafe { m5unified_sys::m5u_rtc_get_volt_low() }
    }

    pub fn date(&self) -> Option<RtcDate> {
        let (mut year, mut month, mut day, mut weekday) = (0, 0, 0, 0);
        let ok = unsafe {
            m5unified_sys::m5u_rtc_get_date(&mut year, &mut month, &mut day, &mut weekday)
        };
        ok.then_some(RtcDate {
            year,
            month,
            day,
            weekday,
        })
    }

    pub fn try_date(&self) -> Result<RtcDate, Error> {
        self.date().ok_or(Error::Unavailable("rtc"))
    }

    pub fn time(&self) -> Option<RtcTime> {
        let (mut hour, mut minute, mut second) = (0, 0, 0);
        let ok = unsafe { m5unified_sys::m5u_rtc_get_time(&mut hour, &mut minute, &mut second) };
        ok.then_some(RtcTime {
            hour,
            minute,
            second,
        })
    }

    pub fn try_time(&self) -> Result<RtcTime, Error> {
        self.time().ok_or(Error::Unavailable("rtc"))
    }

    pub fn get_datetime(&self) -> Option<DateTime> {
        let (mut year, mut month, mut day, mut hour, mut minute, mut second) = (0, 0, 0, 0, 0, 0);
        let ok = unsafe {
            m5unified_sys::m5u_rtc_get_datetime(
                &mut year,
                &mut month,
                &mut day,
                &mut hour,
                &mut minute,
                &mut second,
            )
        };
        ok.then_some(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    pub fn try_get_datetime(&self) -> Result<DateTime, Error> {
        self.get_datetime().ok_or(Error::Unavailable("rtc"))
    }

    pub fn set_date(&mut self, date: RtcDate) -> bool {
        unsafe { m5unified_sys::m5u_rtc_set_date(date.year, date.month, date.day, date.weekday) }
    }

    pub fn try_set_date(&mut self, date: RtcDate) -> Result<(), Error> {
        if !date.is_valid() {
            return Err(Error::InvalidValue("rtc date"));
        }
        self.set_date(date)
            .then_some(())
            .ok_or(Error::Unavailable("rtc"))
    }

    pub fn set_time(&mut self, time: RtcTime) -> bool {
        unsafe { m5unified_sys::m5u_rtc_set_time(time.hour, time.minute, time.second) }
    }

    pub fn try_set_time(&mut self, time: RtcTime) -> Result<(), Error> {
        if !time.is_valid() {
            return Err(Error::InvalidValue("rtc time"));
        }
        self.set_time(time)
            .then_some(())
            .ok_or(Error::Unavailable("rtc"))
    }

    pub fn set_datetime(&mut self, datetime: DateTime) -> bool {
        unsafe {
            m5unified_sys::m5u_rtc_set_datetime(
                datetime.year,
                datetime.month,
                datetime.day,
                datetime.hour,
                datetime.minute,
                datetime.second,
            )
        }
    }

    pub fn try_set_datetime(&mut self, datetime: DateTime) -> Result<(), Error> {
        if !datetime.is_valid() {
            return Err(Error::InvalidValue("rtc datetime"));
        }
        self.set_datetime(datetime)
            .then_some(())
            .ok_or(Error::Unavailable("rtc"))
    }

    pub fn set_system_time_from_rtc(&mut self) {
        unsafe { m5unified_sys::m5u_rtc_set_system_time_from_rtc() }
    }

    pub fn try_set_system_time_from_rtc(&mut self) -> Result<(), Error> {
        self.set_system_time_from_rtc();
        Ok(())
    }

    pub fn set_alarm_irq_after(&mut self, seconds: i32) -> bool {
        unsafe { m5unified_sys::m5u_rtc_set_alarm_irq_after(seconds) }
    }

    pub fn try_set_alarm_irq_after(&mut self, seconds: i32) -> Result<(), Error> {
        if seconds < 0 {
            return Err(Error::InvalidValue("rtc alarm seconds"));
        }
        self.set_alarm_irq_after(seconds)
            .then_some(())
            .ok_or(Error::Unavailable("rtc alarm"))
    }

    pub fn irq_status(&self) -> bool {
        unsafe { m5unified_sys::m5u_rtc_get_irq_status() }
    }

    pub fn clear_irq(&mut self) {
        unsafe { m5unified_sys::m5u_rtc_clear_irq() }
    }

    pub fn try_clear_irq(&mut self) -> Result<(), Error> {
        self.clear_irq();
        Ok(())
    }

    pub fn disable_irq(&mut self) {
        unsafe { m5unified_sys::m5u_rtc_disable_irq() }
    }

    pub fn try_disable_irq(&mut self) -> Result<(), Error> {
        self.disable_irq();
        Ok(())
    }
}

#[derive(Debug)]
pub struct Power;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChargingState {
    Discharging,
    Charging,
    Unknown,
    Other(i32),
}

impl ChargingState {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Discharging,
            1 => Self::Charging,
            2 => Self::Unknown,
            raw => Self::Other(raw),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::Discharging => 0,
            Self::Charging => 1,
            Self::Unknown => 2,
            Self::Other(raw) => raw,
        }
    }

    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Other(_))
    }

    pub const fn is_discharging(self) -> bool {
        matches!(self, Self::Discharging)
    }

    pub const fn is_charging(self) -> bool {
        matches!(self, Self::Charging)
    }

    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerKeyState {
    None,
    LongPressed,
    ShortClicked,
    Both,
    Other(u8),
}

impl PowerKeyState {
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::LongPressed,
            2 => Self::ShortClicked,
            3 => Self::Both,
            raw => Self::Other(raw),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::None => 0,
            Self::LongPressed => 1,
            Self::ShortClicked => 2,
            Self::Both => 3,
            Self::Other(raw) => raw,
        }
    }

    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Other(_))
    }

    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    pub const fn is_long_pressed(self) -> bool {
        matches!(self, Self::LongPressed | Self::Both)
    }

    pub const fn is_short_clicked(self) -> bool {
        matches!(self, Self::ShortClicked | Self::Both)
    }

    pub const fn has_event(self) -> bool {
        matches!(self, Self::LongPressed | Self::ShortClicked | Self::Both)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PmicType {
    Unknown,
    Adc,
    Axp192,
    Ip5306,
    Axp2101,
    Other(i32),
}

impl PmicType {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Unknown,
            1 => Self::Adc,
            2 => Self::Axp192,
            3 => Self::Ip5306,
            4 => Self::Axp2101,
            raw => Self::Other(raw),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::Adc => 1,
            Self::Axp192 => 2,
            Self::Ip5306 => 3,
            Self::Axp2101 => 4,
            Self::Other(raw) => raw,
        }
    }

    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Other(_))
    }

    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub const fn is_axp(self) -> bool {
        matches!(self, Self::Axp192 | Self::Axp2101)
    }

    pub const fn supports_axp2101_irqs(self) -> bool {
        matches!(self, Self::Axp2101)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PowerStatus {
    pub battery_level: Option<u8>,
    pub battery_voltage_mv: Option<u16>,
    pub battery_current_ma: i32,
    pub charging_state: ChargingState,
    pub is_charging: bool,
    pub ext_output: bool,
    pub usb_output: bool,
    pub key_state: PowerKeyState,
    pub pmic_type: PmicType,
}

impl PowerStatus {
    pub const fn battery_percent(self) -> Option<u8> {
        self.battery_level
    }

    pub const fn battery_fraction(self) -> Option<f32> {
        match self.battery_level {
            Some(level) => Some(level as f32 / 100.0),
            None => None,
        }
    }

    pub const fn battery_voltage(self) -> Option<u16> {
        self.battery_voltage_mv
    }

    pub const fn has_battery_level(self) -> bool {
        self.battery_level.is_some()
    }

    pub const fn has_battery_voltage(self) -> bool {
        self.battery_voltage_mv.is_some()
    }

    pub const fn has_battery(self) -> bool {
        self.battery_level.is_some() || self.battery_voltage_mv.is_some()
    }

    pub const fn is_running_on_battery(self) -> bool {
        self.has_battery() && !self.is_charging
    }

    pub const fn has_power_key_event(self) -> bool {
        self.key_state.has_event()
    }
}

#[derive(Debug)]
pub struct Led;

impl Power {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_power_begin() }
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.begin()
            .then_some(())
            .ok_or(Error::Unavailable("power"))
    }

    pub fn battery_level(&self) -> Option<u8> {
        let level = unsafe { m5unified_sys::m5u_battery_level() };
        if (0..=100).contains(&level) {
            Some(level as u8)
        } else {
            None
        }
    }

    pub fn battery_voltage_mv(&self) -> Option<u16> {
        let mv = unsafe { m5unified_sys::m5u_battery_voltage_mv() };
        (mv >= 0).then_some(mv as u16)
    }

    pub fn battery_current_ma(&self) -> i32 {
        unsafe { m5unified_sys::m5u_battery_current_ma() }
    }

    pub fn charging_state(&self) -> ChargingState {
        ChargingState::from_raw(unsafe { m5unified_sys::m5u_power_charging_state() })
    }

    pub fn is_charging(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_is_charging() }
    }

    pub fn set_ext_output(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_power_set_ext_output(enable) }
    }

    pub fn try_set_ext_output(&mut self, enable: bool) -> Result<(), Error> {
        self.set_ext_output(enable);
        Ok(())
    }

    pub fn ext_output(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_get_ext_output() }
    }

    pub fn set_usb_output(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_power_set_usb_output(enable) }
    }

    pub fn try_set_usb_output(&mut self, enable: bool) -> Result<(), Error> {
        self.set_usb_output(enable);
        Ok(())
    }

    pub fn usb_output(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_get_usb_output() }
    }

    pub fn set_led(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_power_set_led(brightness) }
    }

    pub fn try_set_led(&mut self, brightness: u8) -> Result<(), Error> {
        self.set_led(brightness);
        Ok(())
    }

    pub fn power_off(&mut self) {
        unsafe { m5unified_sys::m5u_power_power_off() }
    }

    pub fn try_power_off(&mut self) -> Result<(), Error> {
        self.power_off();
        Ok(())
    }

    pub fn timer_sleep(&mut self, seconds: i32) {
        unsafe { m5unified_sys::m5u_power_timer_sleep(seconds) }
    }

    pub fn try_timer_sleep(&mut self, seconds: i32) -> Result<(), Error> {
        validate_sleep_seconds(seconds)?;
        self.timer_sleep(seconds);
        Ok(())
    }

    pub fn deep_sleep(&mut self, micro_seconds: u64, touch_wakeup: bool) {
        unsafe { m5unified_sys::m5u_power_deep_sleep(micro_seconds, touch_wakeup) }
    }

    pub fn try_deep_sleep(&mut self, micro_seconds: u64, touch_wakeup: bool) -> Result<(), Error> {
        validate_sleep_microseconds(micro_seconds)?;
        self.deep_sleep(micro_seconds, touch_wakeup);
        Ok(())
    }

    pub fn light_sleep(&mut self, micro_seconds: u64, touch_wakeup: bool) {
        unsafe { m5unified_sys::m5u_power_light_sleep(micro_seconds, touch_wakeup) }
    }

    pub fn try_light_sleep(&mut self, micro_seconds: u64, touch_wakeup: bool) -> Result<(), Error> {
        validate_sleep_microseconds(micro_seconds)?;
        self.light_sleep(micro_seconds, touch_wakeup);
        Ok(())
    }

    pub fn set_battery_charge(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_power_set_battery_charge(enable) }
    }

    pub fn try_set_battery_charge(&mut self, enable: bool) -> Result<(), Error> {
        self.set_battery_charge(enable);
        Ok(())
    }

    pub fn set_charge_current(&mut self, max_ma: u16) {
        unsafe { m5unified_sys::m5u_power_set_charge_current(max_ma) }
    }

    pub fn try_set_charge_current(&mut self, max_ma: u16) -> Result<(), Error> {
        validate_milliamps(max_ma)?;
        self.set_charge_current(max_ma);
        Ok(())
    }

    pub fn set_charge_voltage(&mut self, max_mv: u16) {
        unsafe { m5unified_sys::m5u_power_set_charge_voltage(max_mv) }
    }

    pub fn try_set_charge_voltage(&mut self, max_mv: u16) -> Result<(), Error> {
        validate_millivolts(max_mv)?;
        self.set_charge_voltage(max_mv);
        Ok(())
    }

    pub fn key_state(&self) -> PowerKeyState {
        PowerKeyState::from_raw(unsafe { m5unified_sys::m5u_power_get_key_state() })
    }

    pub fn set_vibration(&mut self, level: u8) {
        unsafe { m5unified_sys::m5u_power_set_vibration(level) }
    }

    pub fn try_set_vibration(&mut self, level: u8) -> Result<(), Error> {
        self.set_vibration(level);
        Ok(())
    }

    pub fn pmic_type(&self) -> PmicType {
        PmicType::from_raw(unsafe { m5unified_sys::m5u_power_get_type() })
    }

    pub fn status(&self) -> PowerStatus {
        PowerStatus {
            battery_level: self.battery_level(),
            battery_voltage_mv: self.battery_voltage_mv(),
            battery_current_ma: self.battery_current_ma(),
            charging_state: self.charging_state(),
            is_charging: self.is_charging(),
            ext_output: self.ext_output(),
            usb_output: self.usb_output(),
            key_state: self.key_state(),
            pmic_type: self.pmic_type(),
        }
    }
}

fn validate_sleep_seconds(seconds: i32) -> Result<(), Error> {
    if seconds <= 0 {
        Err(Error::InvalidValue("sleep seconds"))
    } else {
        Ok(())
    }
}

fn validate_sleep_microseconds(micro_seconds: u64) -> Result<(), Error> {
    if micro_seconds == 0 {
        Err(Error::InvalidValue("sleep microseconds"))
    } else {
        Ok(())
    }
}

fn validate_milliamps(max_ma: u16) -> Result<(), Error> {
    if max_ma == 0 {
        Err(Error::InvalidValue("milliamps"))
    } else {
        Ok(())
    }
}

fn validate_millivolts(max_mv: u16) -> Result<(), Error> {
    if max_mv == 0 {
        Err(Error::InvalidValue("millivolts"))
    } else {
        Ok(())
    }
}

impl Led {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_led_begin() }
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.begin().then_some(()).ok_or(Error::Unavailable("led"))
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_led_is_enabled() }
    }

    pub fn count(&self) -> usize {
        unsafe { m5unified_sys::m5u_led_count() }
    }

    pub fn display(&mut self) {
        unsafe { m5unified_sys::m5u_led_display() }
    }

    pub fn try_display(&mut self) -> Result<(), Error> {
        if !self.is_enabled() {
            return Err(Error::Unavailable("led"));
        }
        self.display();
        Ok(())
    }

    pub fn set_auto_display(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_led_set_auto_display(enable) }
    }

    pub fn try_set_auto_display(&mut self, enable: bool) -> Result<(), Error> {
        if !self.is_enabled() {
            return Err(Error::Unavailable("led"));
        }
        self.set_auto_display(enable);
        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_led_set_brightness(brightness) }
    }

    pub fn try_set_brightness(&mut self, brightness: u8) -> Result<(), Error> {
        if !self.is_enabled() {
            return Err(Error::Unavailable("led"));
        }
        self.set_brightness(brightness);
        Ok(())
    }

    pub fn set_color(&mut self, index: usize, rgb: u32) {
        unsafe { m5unified_sys::m5u_led_set_color(index, rgb) }
    }

    pub fn try_set_color(&mut self, index: usize, rgb: u32) -> Result<(), Error> {
        validate_led_index(self.count(), index)?;
        self.set_color(index, rgb);
        Ok(())
    }

    pub fn set_rgb_color(&mut self, index: usize, color: RgbColor) {
        self.set_color(index, color.raw())
    }

    pub fn try_set_rgb_color(&mut self, index: usize, color: RgbColor) -> Result<(), Error> {
        self.try_set_color(index, color.raw())
    }

    pub fn set_all_color(&mut self, rgb: u32) {
        unsafe { m5unified_sys::m5u_led_set_all_color(rgb) }
    }

    pub fn try_set_all_color(&mut self, rgb: u32) -> Result<(), Error> {
        if self.count() == 0 {
            return Err(Error::Unavailable("led"));
        }
        self.set_all_color(rgb);
        Ok(())
    }

    pub fn set_all_rgb_color(&mut self, color: RgbColor) {
        self.set_all_color(color.raw())
    }

    pub fn try_set_all_rgb_color(&mut self, color: RgbColor) -> Result<(), Error> {
        self.try_set_all_color(color.raw())
    }

    pub fn off(&mut self) {
        self.set_all_color(rgb::BLACK)
    }

    pub fn try_off(&mut self) -> Result<(), Error> {
        self.try_set_all_color(rgb::BLACK)
    }
}

fn validate_led_index(count: usize, index: usize) -> Result<(), Error> {
    if count == 0 {
        return Err(Error::Unavailable("led"));
    }
    if index >= count {
        return Err(Error::InvalidValue("led index"));
    }
    Ok(())
}

#[derive(Debug)]
pub struct Log;

impl Log {
    pub fn println(&self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_println(text.as_ptr()) }
        Ok(())
    }
}

pub fn sd_begin() -> bool {
    unsafe { m5unified_sys::m5u_sd_begin() }
}

pub fn sd_try_begin() -> Result<(), Error> {
    sd_begin().then_some(()).ok_or(Error::Unavailable("sd"))
}

pub fn sd_end() {
    unsafe { m5unified_sys::m5u_sd_end() }
}

pub fn sd_card_type() -> SdCardType {
    SdCardType::from_raw(unsafe { m5unified_sys::m5u_sd_card_type() as i32 })
}

pub fn sd_info() -> SdCardInfo {
    SdCardInfo {
        size_bytes: unsafe { m5unified_sys::m5u_sd_card_size_bytes() },
        total_bytes: unsafe { m5unified_sys::m5u_sd_total_bytes() },
        used_bytes: unsafe { m5unified_sys::m5u_sd_used_bytes() },
    }
}

pub fn sd_exists(path: &str) -> Result<bool, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    Ok(unsafe { m5unified_sys::m5u_sd_exists(path.as_ptr()) })
}

pub fn sd_file_size(path: &str) -> Result<u64, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    Ok(unsafe { m5unified_sys::m5u_sd_file_size(path.as_ptr()) })
}

pub fn sd_is_directory(path: &str) -> Result<bool, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    Ok(unsafe { m5unified_sys::m5u_sd_is_directory(path.as_ptr()) })
}

pub fn sd_read_file(path: &str, buffer: &mut [u8]) -> Result<usize, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    Ok(
        unsafe {
            m5unified_sys::m5u_sd_read_file(path.as_ptr(), buffer.as_mut_ptr(), buffer.len())
        },
    )
}

pub fn sd_try_read_file_exact(path: &str, buffer: &mut [u8]) -> Result<(), Error> {
    let read = sd_read_file(path, buffer)?;
    (read == buffer.len())
        .then_some(())
        .ok_or(Error::Unavailable("sd read"))
}

pub fn sd_write_file(path: &str, data: &[u8]) -> Result<usize, Error> {
    sd_write_file_with(path, data, false)
}

pub fn sd_try_write_file_all(path: &str, data: &[u8]) -> Result<(), Error> {
    let written = sd_write_file(path, data)?;
    (written == data.len())
        .then_some(())
        .ok_or(Error::Unavailable("sd write"))
}

pub fn sd_append_file(path: &str, data: &[u8]) -> Result<usize, Error> {
    sd_write_file_with(path, data, true)
}

pub fn sd_try_append_file_all(path: &str, data: &[u8]) -> Result<(), Error> {
    let written = sd_append_file(path, data)?;
    (written == data.len())
        .then_some(())
        .ok_or(Error::Unavailable("sd write"))
}

pub fn sd_remove_file(path: &str) -> Result<bool, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    Ok(unsafe { m5unified_sys::m5u_sd_remove(path.as_ptr()) })
}

pub fn sd_try_remove_file(path: &str) -> Result<(), Error> {
    sd_remove_file(path)?
        .then_some(())
        .ok_or(Error::Unavailable("sd remove"))
}

pub fn sd_mkdir(path: &str) -> Result<bool, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    Ok(unsafe { m5unified_sys::m5u_sd_mkdir(path.as_ptr()) })
}

pub fn sd_try_mkdir(path: &str) -> Result<(), Error> {
    sd_mkdir(path)?
        .then_some(())
        .ok_or(Error::Unavailable("sd mkdir"))
}

pub fn sd_rmdir(path: &str) -> Result<bool, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    Ok(unsafe { m5unified_sys::m5u_sd_rmdir(path.as_ptr()) })
}

pub fn sd_try_rmdir(path: &str) -> Result<(), Error> {
    sd_rmdir(path)?
        .then_some(())
        .ok_or(Error::Unavailable("sd rmdir"))
}

pub fn sd_rename(from_path: &str, to_path: &str) -> Result<bool, Error> {
    let from_path = CString::new(from_path).map_err(|_| Error::InvalidString)?;
    let to_path = CString::new(to_path).map_err(|_| Error::InvalidString)?;
    Ok(unsafe { m5unified_sys::m5u_sd_rename(from_path.as_ptr(), to_path.as_ptr()) })
}

pub fn sd_try_rename(from_path: &str, to_path: &str) -> Result<(), Error> {
    sd_rename(from_path, to_path)?
        .then_some(())
        .ok_or(Error::Unavailable("sd rename"))
}

pub fn sd_list_dir(path: &str, max_entries: usize) -> Result<Vec<CardputerSdDirEntry>, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    if max_entries == 0 {
        return Ok(Vec::new());
    }

    let mut raw_entries = vec![m5unified_sys::m5u_cardputer_sd_dir_entry_t::default(); max_entries];
    let count = unsafe {
        m5unified_sys::m5u_sd_list_dir(path.as_ptr(), raw_entries.as_mut_ptr(), raw_entries.len())
    }
    .min(raw_entries.len());

    let entries = raw_entries
        .iter()
        .take(count)
        .map(|entry| {
            let name = unsafe { CStr::from_ptr(entry.name.as_ptr()) }
                .to_string_lossy()
                .into_owned();
            CardputerSdDirEntry {
                name,
                is_directory: entry.is_directory,
                size: entry.size,
            }
        })
        .collect();

    Ok(entries)
}

fn sd_write_file_with(path: &str, data: &[u8], append: bool) -> Result<usize, Error> {
    let path = CString::new(path).map_err(|_| Error::InvalidString)?;
    Ok(unsafe {
        m5unified_sys::m5u_sd_write_file(path.as_ptr(), data.as_ptr(), data.len(), append)
    })
}

pub const DEFAULT_I2C_FREQUENCY_HZ: u32 = I2cConfig::DEFAULT_FREQUENCY_HZ;
pub const DEFAULT_SPI_FREQUENCY_HZ: u32 = 1_000_000;

pub fn i2c_begin(pins: I2cPins) -> bool {
    i2c_begin_config(pins, I2cConfig::default())
}

pub fn i2c_begin_config(pins: I2cPins, config: I2cConfig) -> bool {
    i2c_begin_with_config(pins, config)
}

pub fn i2c_try_begin(pins: I2cPins) -> Result<(), Error> {
    i2c_try_begin_config(pins, I2cConfig::default())
}

pub fn i2c_try_begin_config(pins: I2cPins, config: I2cConfig) -> Result<(), Error> {
    validate_i2c_config(config)?;
    i2c_begin_config(pins, config)
        .then_some(())
        .ok_or(Error::Unavailable("i2c"))
}

pub fn i2c_begin_with(pins: I2cPins, frequency_hz: u32) -> bool {
    i2c_begin_with_config(pins, I2cConfig::new(frequency_hz))
}

pub fn i2c_begin_with_config(pins: I2cPins, config: I2cConfig) -> bool {
    unsafe {
        m5unified_sys::m5u_i2c_begin(pins.sda as c_int, pins.scl as c_int, config.frequency_hz)
    }
}

pub fn i2c_try_begin_with(pins: I2cPins, frequency_hz: u32) -> Result<(), Error> {
    i2c_try_begin_with_config(pins, I2cConfig::new(frequency_hz))
}

pub fn i2c_try_begin_with_config(pins: I2cPins, config: I2cConfig) -> Result<(), Error> {
    validate_i2c_config(config)?;
    i2c_begin_with_config(pins, config)
        .then_some(())
        .ok_or(Error::Unavailable("i2c"))
}

pub fn i2c_end() {
    unsafe { m5unified_sys::m5u_i2c_end() }
}

pub fn i2c_probe(address: I2cAddress) -> bool {
    unsafe { m5unified_sys::m5u_i2c_probe(address.get()) }
}

pub fn i2c_write(address: I2cAddress, data: &[u8]) -> bool {
    unsafe { m5unified_sys::m5u_i2c_write(address.get(), data.as_ptr(), data.len()) }
}

pub fn i2c_try_write(address: I2cAddress, data: &[u8]) -> Result<(), Error> {
    i2c_write(address, data)
        .then_some(())
        .ok_or(Error::Unavailable("i2c"))
}

pub fn i2c_read(address: I2cAddress, data: &mut [u8]) -> usize {
    unsafe { m5unified_sys::m5u_i2c_read(address.get(), data.as_mut_ptr(), data.len()) }
}

pub fn i2c_try_read(address: I2cAddress, data: &mut [u8]) -> Result<usize, Error> {
    let read = i2c_read(address, data);
    if read > 0 || data.is_empty() {
        Ok(read)
    } else {
        Err(Error::Unavailable("i2c"))
    }
}

pub fn i2c_try_read_exact(address: I2cAddress, data: &mut [u8]) -> Result<(), Error> {
    (i2c_try_read(address, data)? == data.len())
        .then_some(())
        .ok_or(Error::Unavailable("i2c"))
}

pub fn i2c_write_reg(address: I2cAddress, reg: u8, data: &[u8]) -> bool {
    unsafe { m5unified_sys::m5u_i2c_write_reg(address.get(), reg, data.as_ptr(), data.len()) }
}

pub fn i2c_try_write_reg(address: I2cAddress, reg: u8, data: &[u8]) -> Result<(), Error> {
    i2c_write_reg(address, reg, data)
        .then_some(())
        .ok_or(Error::Unavailable("i2c"))
}

pub fn i2c_read_reg(address: I2cAddress, reg: u8, data: &mut [u8]) -> usize {
    unsafe { m5unified_sys::m5u_i2c_read_reg(address.get(), reg, data.as_mut_ptr(), data.len()) }
}

pub fn i2c_try_read_reg(address: I2cAddress, reg: u8, data: &mut [u8]) -> Result<usize, Error> {
    let read = i2c_read_reg(address, reg, data);
    if read > 0 || data.is_empty() {
        Ok(read)
    } else {
        Err(Error::Unavailable("i2c"))
    }
}

pub fn i2c_try_read_reg_exact(address: I2cAddress, reg: u8, data: &mut [u8]) -> Result<(), Error> {
    (i2c_try_read_reg(address, reg, data)? == data.len())
        .then_some(())
        .ok_or(Error::Unavailable("i2c"))
}

pub fn i2c_scan() -> Vec<I2cAddress> {
    (0x08..=0x77)
        .filter_map(I2cAddress::new)
        .filter(|&address| i2c_probe(address))
        .collect()
}

pub fn spi_begin(pins: SpiPins) -> bool {
    unsafe {
        m5unified_sys::m5u_spi_begin(
            pins.sck as c_int,
            pins.miso as c_int,
            pins.mosi as c_int,
            pins.cs as c_int,
        )
    }
}

pub fn spi_try_begin(pins: SpiPins) -> Result<(), Error> {
    spi_begin(pins)
        .then_some(())
        .ok_or(Error::Unavailable("spi"))
}

pub fn spi_end() {
    unsafe { m5unified_sys::m5u_spi_end() }
}

pub fn spi_transfer_byte(value: u8, config: SpiConfig) -> u8 {
    unsafe {
        m5unified_sys::m5u_spi_transfer_byte(
            value,
            config.frequency_hz,
            config.mode.raw(),
            config.bit_order.lsb_first(),
        )
    }
}

pub fn spi_try_transfer_byte(value: u8, config: SpiConfig) -> Result<u8, Error> {
    validate_spi_config(config)?;
    Ok(spi_transfer_byte(value, config))
}

pub fn spi_transfer(tx: &[u8], rx: &mut [u8], config: SpiConfig) -> bool {
    if tx.len() != rx.len() {
        return false;
    }

    unsafe {
        m5unified_sys::m5u_spi_transfer(
            tx.as_ptr(),
            rx.as_mut_ptr(),
            rx.len(),
            config.frequency_hz,
            config.mode.raw(),
            config.bit_order.lsb_first(),
        )
    }
}

pub fn spi_try_transfer(tx: &[u8], rx: &mut [u8], config: SpiConfig) -> Result<(), Error> {
    validate_spi_transfer(tx, rx, config)?;
    spi_transfer(tx, rx, config)
        .then_some(())
        .ok_or(Error::Unavailable("spi"))
}

pub fn spi_read(rx: &mut [u8], config: SpiConfig) -> bool {
    unsafe {
        m5unified_sys::m5u_spi_transfer(
            core::ptr::null(),
            rx.as_mut_ptr(),
            rx.len(),
            config.frequency_hz,
            config.mode.raw(),
            config.bit_order.lsb_first(),
        )
    }
}

pub fn spi_try_read(rx: &mut [u8], config: SpiConfig) -> Result<(), Error> {
    validate_spi_config(config)?;
    spi_read(rx, config)
        .then_some(())
        .ok_or(Error::Unavailable("spi"))
}

pub fn spi_write(data: &[u8], config: SpiConfig) -> bool {
    unsafe {
        m5unified_sys::m5u_spi_write(
            data.as_ptr(),
            data.len(),
            config.frequency_hz,
            config.mode.raw(),
            config.bit_order.lsb_first(),
        )
    }
}

pub fn spi_try_write(data: &[u8], config: SpiConfig) -> Result<(), Error> {
    validate_spi_config(config)?;
    spi_write(data, config)
        .then_some(())
        .ok_or(Error::Unavailable("spi"))
}

pub fn spi_try_write_all(data: &[u8], config: SpiConfig) -> Result<(), Error> {
    validate_spi_config(config)?;
    spi_write(data, config)
        .then_some(())
        .ok_or(Error::Unavailable("spi"))
}

pub fn uart_begin(pins: UartPins, baud: u32) -> bool {
    uart_begin_config(pins, UartConfig::new(baud))
}

pub fn uart_begin_config(pins: UartPins, config: UartConfig) -> bool {
    unsafe { m5unified_sys::m5u_uart_begin(pins.rx as c_int, pins.tx as c_int, config.baud) }
}

pub fn uart_try_begin(pins: UartPins, baud: u32) -> Result<(), Error> {
    uart_try_begin_config(pins, UartConfig::new(baud))
}

pub fn uart_try_begin_config(pins: UartPins, config: UartConfig) -> Result<(), Error> {
    validate_uart_config(config)?;
    uart_begin_config(pins, config)
        .then_some(())
        .ok_or(Error::Unavailable("uart"))
}

pub fn uart_end() {
    unsafe { m5unified_sys::m5u_uart_end() }
}

pub fn uart_available() -> usize {
    unsafe { m5unified_sys::m5u_uart_available() }
}

pub fn uart_read(data: &mut [u8]) -> usize {
    unsafe { m5unified_sys::m5u_uart_read(data.as_mut_ptr(), data.len()) }
}

pub fn uart_try_read(data: &mut [u8]) -> Result<usize, Error> {
    let read = uart_read(data);
    if read > 0 || data.is_empty() {
        Ok(read)
    } else {
        Err(Error::Unavailable("uart"))
    }
}

pub fn uart_write(data: &[u8]) -> usize {
    unsafe { m5unified_sys::m5u_uart_write(data.as_ptr(), data.len()) }
}

pub fn uart_try_write(data: &[u8]) -> Result<usize, Error> {
    let written = uart_write(data);
    if written > 0 || data.is_empty() {
        Ok(written)
    } else {
        Err(Error::Unavailable("uart"))
    }
}

pub fn uart_try_write_all(data: &[u8]) -> Result<(), Error> {
    (uart_try_write(data)? == data.len())
        .then_some(())
        .ok_or(Error::Unavailable("uart"))
}

pub fn uart_write_byte(byte: u8) -> usize {
    uart_write(&[byte])
}

pub fn uart_try_write_byte(byte: u8) -> Result<(), Error> {
    (uart_try_write(&[byte])? == 1)
        .then_some(())
        .ok_or(Error::Unavailable("uart"))
}

pub fn uart_write_str(text: &str) -> usize {
    uart_write(text.as_bytes())
}

pub fn uart_try_write_str(text: &str) -> Result<usize, Error> {
    uart_try_write(text.as_bytes())
}

pub fn uart_try_write_str_all(text: &str) -> Result<(), Error> {
    uart_try_write_all(text.as_bytes())
}

pub fn uart_flush() {
    unsafe { m5unified_sys::m5u_uart_flush() }
}

pub fn gpio_pin_mode(pin: GpioPin, mode: GpioMode) -> bool {
    unsafe { m5unified_sys::m5u_gpio_pin_mode(pin.raw() as c_int, mode.raw()) }
}

pub fn gpio_try_pin_mode(pin: GpioPin, mode: GpioMode) -> Result<(), Error> {
    gpio_pin_mode(pin, mode)
        .then_some(())
        .ok_or(Error::Unavailable("gpio"))
}

pub fn gpio_write(pin: GpioPin, high: bool) -> bool {
    unsafe { m5unified_sys::m5u_gpio_write(pin.raw() as c_int, high) }
}

pub fn gpio_try_write(pin: GpioPin, high: bool) -> Result<(), Error> {
    gpio_write(pin, high)
        .then_some(())
        .ok_or(Error::Unavailable("gpio"))
}

pub fn gpio_read(pin: GpioPin) -> Option<bool> {
    match unsafe { m5unified_sys::m5u_gpio_read(pin.raw() as c_int) } {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

pub fn gpio_try_read(pin: GpioPin) -> Result<bool, Error> {
    gpio_read(pin).ok_or(Error::Unavailable("gpio"))
}

pub fn analog_read(pin: GpioPin) -> Option<u16> {
    let value = unsafe { m5unified_sys::m5u_gpio_analog_read(pin.raw() as c_int) };
    u16::try_from(value).ok()
}

pub fn analog_try_read(pin: GpioPin) -> Result<u16, Error> {
    analog_read(pin).ok_or(Error::Unavailable("analog"))
}

pub fn analog_read_millivolts(pin: GpioPin) -> Option<u16> {
    let value = unsafe { m5unified_sys::m5u_gpio_analog_read_millivolts(pin.raw() as c_int) };
    u16::try_from(value).ok()
}

pub fn analog_try_read_millivolts(pin: GpioPin) -> Result<u16, Error> {
    analog_read_millivolts(pin).ok_or(Error::Unavailable("analog"))
}

pub fn analog_write(pin: GpioPin, duty: u8) -> bool {
    unsafe { m5unified_sys::m5u_gpio_analog_write(pin.raw() as c_int, duty) }
}

pub fn analog_try_write(pin: GpioPin, duty: u8) -> Result<(), Error> {
    analog_write(pin, duty)
        .then_some(())
        .ok_or(Error::Unavailable("analog"))
}

pub fn analog_write_frequency(pin: GpioPin, frequency_hz: u32) -> bool {
    unsafe { m5unified_sys::m5u_gpio_analog_write_frequency(pin.raw() as c_int, frequency_hz) }
}

pub fn analog_try_write_frequency(pin: GpioPin, frequency_hz: u32) -> Result<(), Error> {
    validate_frequency_hz(frequency_hz)?;
    analog_write_frequency(pin, frequency_hz)
        .then_some(())
        .ok_or(Error::Unavailable("analog"))
}

pub fn analog_write_resolution(pin: GpioPin, resolution_bits: u8) -> bool {
    unsafe { m5unified_sys::m5u_gpio_analog_write_resolution(pin.raw() as c_int, resolution_bits) }
}

pub fn analog_try_write_resolution(pin: GpioPin, resolution_bits: u8) -> Result<(), Error> {
    analog_write_resolution(pin, resolution_bits)
        .then_some(())
        .ok_or(Error::Unavailable("analog"))
}

pub fn analog_write_config(pin: GpioPin, config: AnalogOutputConfig) -> bool {
    analog_write_resolution(pin, config.resolution_bits)
        && analog_write_frequency(pin, config.frequency_hz)
        && analog_write(pin, config.duty)
}

pub fn analog_try_write_config(pin: GpioPin, config: AnalogOutputConfig) -> Result<(), Error> {
    validate_analog_output_config(config)?;
    analog_write_config(pin, config)
        .then_some(())
        .ok_or(Error::Unavailable("analog"))
}

fn validate_frequency_hz(frequency_hz: u32) -> Result<(), Error> {
    if frequency_hz == 0 {
        Err(Error::InvalidValue("frequency"))
    } else {
        Ok(())
    }
}

fn validate_i2c_config(config: I2cConfig) -> Result<(), Error> {
    validate_frequency_hz(config.frequency_hz)
}

fn validate_analog_output_config(config: AnalogOutputConfig) -> Result<(), Error> {
    validate_frequency_hz(config.frequency_hz)
}

fn validate_pwm_servo_config(config: PwmServoConfig) -> Result<(), Error> {
    validate_frequency_hz(config.frequency_hz)?;
    if config.pin < 0 {
        return Err(Error::InvalidValue("servo pin"));
    }
    if config.channel > 7 {
        return Err(Error::InvalidValue("servo channel"));
    }
    if config.timer > 3 {
        return Err(Error::InvalidValue("servo timer"));
    }
    if config.min_pulse_us == 0 || config.min_pulse_us >= config.max_pulse_us {
        return Err(Error::InvalidValue("servo pulse"));
    }
    if config.min_angle_tenths >= config.max_angle_tenths {
        return Err(Error::InvalidValue("servo angle"));
    }
    if config.neutral_angle_tenths < config.min_angle_tenths
        || config.neutral_angle_tenths > config.max_angle_tenths
    {
        return Err(Error::InvalidValue("servo neutral"));
    }
    Ok(())
}

fn validate_stackchan_pose(pose: StackChanPose) -> Result<(), Error> {
    if pose.pan_tenths < StackChanServoConfig::PAN_MIN_TENTHS
        || pose.pan_tenths > StackChanServoConfig::PAN_MAX_TENTHS
    {
        return Err(Error::InvalidValue("stackchan pan"));
    }
    if pose.tilt_tenths < StackChanServoConfig::TILT_MIN_TENTHS
        || pose.tilt_tenths > StackChanServoConfig::TILT_MAX_TENTHS
    {
        return Err(Error::InvalidValue("stackchan tilt"));
    }
    Ok(())
}

fn step_axis(current: i16, target: i16, step_tenths: u16) -> i16 {
    let step = i16::try_from(step_tenths).unwrap_or(i16::MAX);
    if current < target {
        current.saturating_add(step).min(target)
    } else if current > target {
        current.saturating_sub(step).max(target)
    } else {
        current
    }
}

fn clamp_f32(value: f32, min: f32, max: f32) -> f32 {
    if value.is_nan() {
        0.0
    } else if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn degrees_to_tenths(degrees: f32) -> i16 {
    (degrees * 10.0).round() as i16
}

fn validate_baud(baud: u32) -> Result<(), Error> {
    if baud == 0 {
        Err(Error::InvalidValue("baud"))
    } else {
        Ok(())
    }
}

fn validate_uart_config(config: UartConfig) -> Result<(), Error> {
    validate_baud(config.baud)
}

fn validate_spi_config(config: SpiConfig) -> Result<(), Error> {
    validate_frequency_hz(config.frequency_hz)
}

fn validate_spi_transfer(tx: &[u8], rx: &[u8], config: SpiConfig) -> Result<(), Error> {
    validate_spi_config(config)?;
    if tx.len() != rx.len() {
        Err(Error::InvalidValue("spi transfer length"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color565(pub u16);

impl Color565 {
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u16 & 0xf8) << 8) | ((g as u16 & 0xfc) << 3) | (b as u16 >> 3))
    }

    pub fn rgb888(r: u8, g: u8, b: u8) -> Self {
        Self(unsafe { m5unified_sys::m5u_display_color888(r, g, b) })
    }

    pub const fn raw(self) -> u16 {
        self.0
    }

    pub const fn red5(self) -> u8 {
        ((self.0 >> 11) & 0x1f) as u8
    }

    pub const fn green6(self) -> u8 {
        ((self.0 >> 5) & 0x3f) as u8
    }

    pub const fn blue5(self) -> u8 {
        (self.0 & 0x1f) as u8
    }

    pub const fn rgb565_components(self) -> (u8, u8, u8) {
        (self.red5(), self.green6(), self.blue5())
    }

    pub const fn to_rgb888(self) -> (u8, u8, u8) {
        (
            (self.red5() << 3) | (self.red5() >> 2),
            (self.green6() << 2) | (self.green6() >> 4),
            (self.blue5() << 3) | (self.blue5() >> 2),
        )
    }
}

impl From<u16> for Color565 {
    fn from(raw: u16) -> Self {
        Self::new(raw)
    }
}

impl From<Color565> for u16 {
    fn from(color: Color565) -> Self {
        color.raw()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn components(self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub const fn x(self) -> i32 {
        self.x
    }

    pub const fn y(self) -> i32 {
        self.y
    }

    pub const fn offset(self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub const fn delta_to(self, other: Self) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
    }

    pub const fn delta_point_to(self, other: Self) -> Self {
        Self::new(other.x - self.x, other.y - self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

impl Size {
    pub const ZERO: Self = Self { w: 0, h: 0 };

    pub const fn new(w: i32, h: i32) -> Self {
        Self { w, h }
    }

    pub const fn dimensions(self) -> (i32, i32) {
        (self.w, self.h)
    }

    pub const fn width(self) -> i32 {
        self.w
    }

    pub const fn height(self) -> i32 {
        self.h
    }

    pub const fn is_empty(self) -> bool {
        self.w <= 0 || self.h <= 0
    }

    pub const fn area(self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let area = (self.w as u64) * (self.h as u64);
        if area > usize::MAX as u64 {
            None
        } else {
            Some(area as usize)
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub const fn from_origin_size(origin: Point, size: Size) -> Self {
        Self {
            x: origin.x,
            y: origin.y,
            w: size.w,
            h: size.h,
        }
    }

    pub const fn origin(self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub const fn size(self) -> Size {
        Size {
            w: self.w,
            h: self.h,
        }
    }

    pub const fn is_empty(self) -> bool {
        self.size().is_empty()
    }

    pub const fn area(self) -> Option<usize> {
        self.size().area()
    }

    pub const fn right(self) -> i32 {
        self.x + self.w
    }

    pub const fn bottom(self) -> i32 {
        self.y + self.h
    }

    pub const fn contains(self, point: Point) -> bool {
        !self.is_empty()
            && point.x >= self.x
            && point.y >= self.y
            && point.x < self.right()
            && point.y < self.bottom()
    }

    pub const fn translate(self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            w: self.w,
            h: self.h,
        }
    }
}

fn validate_display_length(length: i32) -> Result<(), Error> {
    if length <= 0 {
        Err(Error::InvalidValue("display length"))
    } else {
        Ok(())
    }
}

fn validate_display_rect(rect: Rect) -> Result<(), Error> {
    if rect.w <= 0 || rect.h <= 0 {
        Err(Error::InvalidValue("display rect"))
    } else {
        Ok(())
    }
}

fn validate_display_radius(radius: i32) -> Result<(), Error> {
    if radius <= 0 {
        Err(Error::InvalidValue("display radius"))
    } else {
        Ok(())
    }
}

fn validate_display_radii(rx: i32, ry: i32) -> Result<(), Error> {
    if rx <= 0 || ry <= 0 {
        Err(Error::InvalidValue("display radii"))
    } else {
        Ok(())
    }
}

fn validate_display_arc(
    inner_radius: i32,
    outer_radius: i32,
    start_angle: f32,
    end_angle: f32,
) -> Result<(), Error> {
    if inner_radius < 0 || outer_radius <= 0 || outer_radius < inner_radius {
        return Err(Error::InvalidValue("display arc radii"));
    }
    if !start_angle.is_finite() || !end_angle.is_finite() {
        return Err(Error::InvalidValue("display arc angle"));
    }
    Ok(())
}

fn validate_display_window(xs: i32, ys: i32, xe: i32, ye: i32) -> Result<(), Error> {
    if xe < xs || ye < ys {
        Err(Error::InvalidValue("display window"))
    } else {
        Ok(())
    }
}

fn validate_display_text_size(size: i32) -> Result<(), Error> {
    if size <= 0 {
        Err(Error::InvalidValue("display text size"))
    } else {
        Ok(())
    }
}

fn validate_canvas_text_size(size: f32) -> Result<(), Error> {
    if !size.is_finite() || size <= 0.0 {
        Err(Error::InvalidValue("canvas text size"))
    } else {
        Ok(())
    }
}

fn validate_display_pivot(x: f32, y: f32) -> Result<(), Error> {
    if !x.is_finite() || !y.is_finite() {
        Err(Error::InvalidValue("display pivot"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DisplayResolution {
    pub logical_width: u16,
    pub logical_height: u16,
    pub refresh_rate: f32,
    pub output_width: u16,
    pub output_height: u16,
    pub scale_w: u8,
    pub scale_h: u8,
    pub pixel_clock: u32,
}

impl DisplayResolution {
    pub const DEFAULT_PIXEL_CLOCK: u32 = 74_250_000;

    pub const fn new(logical_width: u16, logical_height: u16) -> Self {
        Self {
            logical_width,
            logical_height,
            refresh_rate: 0.0,
            output_width: 0,
            output_height: 0,
            scale_w: 0,
            scale_h: 0,
            pixel_clock: Self::DEFAULT_PIXEL_CLOCK,
        }
    }

    pub const fn with_refresh_rate(mut self, refresh_rate: f32) -> Self {
        self.refresh_rate = refresh_rate;
        self
    }

    pub const fn with_output_size(mut self, output_width: u16, output_height: u16) -> Self {
        self.output_width = output_width;
        self.output_height = output_height;
        self
    }

    pub const fn with_scale(mut self, scale_w: u8, scale_h: u8) -> Self {
        self.scale_w = scale_w;
        self.scale_h = scale_h;
        self
    }

    pub const fn with_pixel_clock(mut self, pixel_clock: u32) -> Self {
        self.pixel_clock = pixel_clock;
        self
    }

    pub const fn logical_size(self) -> Size {
        Size {
            w: self.logical_width as i32,
            h: self.logical_height as i32,
        }
    }

    pub const fn refresh_rate(self) -> f32 {
        self.refresh_rate
    }

    pub const fn output_size(self) -> Option<Size> {
        if self.output_width == 0 && self.output_height == 0 {
            None
        } else {
            Some(Size {
                w: self.output_width as i32,
                h: self.output_height as i32,
            })
        }
    }

    pub const fn uses_auto_output_size(self) -> bool {
        self.output_width == 0 && self.output_height == 0
    }

    pub const fn scale(self) -> (u8, u8) {
        (self.scale_w, self.scale_h)
    }

    pub const fn uses_auto_scale(self) -> bool {
        self.scale_w == 0 && self.scale_h == 0
    }

    pub const fn pixel_clock(self) -> u32 {
        self.pixel_clock
    }

    pub fn validate(self) -> Result<(), Error> {
        validate_display_resolution(self)
    }
}

impl Default for DisplayResolution {
    fn default() -> Self {
        Self {
            logical_width: 0,
            logical_height: 0,
            refresh_rate: 0.0,
            output_width: 0,
            output_height: 0,
            scale_w: 0,
            scale_h: 0,
            pixel_clock: DisplayResolution::DEFAULT_PIXEL_CLOCK,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EpdMode {
    Quality,
    Text,
    Fast,
    Fastest,
    Unknown(i32),
}

impl EpdMode {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Quality,
            2 => Self::Text,
            3 => Self::Fast,
            4 => Self::Fastest,
            value => Self::Unknown(value),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::Quality => 1,
            Self::Text => 2,
            Self::Fast => 3,
            Self::Fastest => 4,
            Self::Unknown(value) => value,
        }
    }

    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Unknown(_))
    }

    pub const fn is_quality(self) -> bool {
        matches!(self, Self::Quality)
    }

    pub const fn is_text(self) -> bool {
        matches!(self, Self::Text)
    }

    pub const fn is_fast(self) -> bool {
        matches!(self, Self::Fast | Self::Fastest)
    }
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

impl TextDatum {
    pub const fn from_raw(raw: i32) -> Option<Self> {
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

    pub const fn raw(self) -> c_int {
        self as c_int
    }

    pub const fn horizontal_index(self) -> u8 {
        match self {
            Self::TopLeft | Self::MiddleLeft | Self::BottomLeft => 0,
            Self::TopCenter | Self::MiddleCenter | Self::BottomCenter => 1,
            Self::TopRight | Self::MiddleRight | Self::BottomRight => 2,
        }
    }

    pub const fn vertical_index(self) -> u8 {
        match self {
            Self::TopLeft | Self::TopCenter | Self::TopRight => 0,
            Self::MiddleLeft | Self::MiddleCenter | Self::MiddleRight => 1,
            Self::BottomLeft | Self::BottomCenter | Self::BottomRight => 2,
        }
    }

    pub const fn is_left(self) -> bool {
        self.horizontal_index() == 0
    }

    pub const fn is_horizontal_center(self) -> bool {
        self.horizontal_index() == 1
    }

    pub const fn is_right(self) -> bool {
        self.horizontal_index() == 2
    }

    pub const fn is_top(self) -> bool {
        self.vertical_index() == 0
    }

    pub const fn is_vertical_middle(self) -> bool {
        self.vertical_index() == 1
    }

    pub const fn is_bottom(self) -> bool {
        self.vertical_index() == 2
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ImageDrawOptions {
    pub x: i32,
    pub y: i32,
    pub max_width: i32,
    pub max_height: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub datum: TextDatum,
}

impl Default for ImageDrawOptions {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            max_width: 0,
            max_height: 0,
            offset_x: 0,
            offset_y: 0,
            scale_x: 1.0,
            scale_y: 0.0,
            datum: TextDatum::TopLeft,
        }
    }
}

impl ImageDrawOptions {
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            max_width: 0,
            max_height: 0,
            offset_x: 0,
            offset_y: 0,
            scale_x: 1.0,
            scale_y: 0.0,
            datum: TextDatum::TopLeft,
        }
    }

    pub const fn at(point: Point) -> Self {
        Self::new(point.x, point.y)
    }

    pub const fn origin(self) -> Point {
        Point::new(self.x, self.y)
    }

    pub const fn max_size(self) -> Option<Size> {
        if self.max_width <= 0 || self.max_height <= 0 {
            None
        } else {
            Some(Size::new(self.max_width, self.max_height))
        }
    }

    pub const fn offset(self) -> Point {
        Point::new(self.offset_x, self.offset_y)
    }

    pub const fn with_origin(mut self, point: Point) -> Self {
        self.x = point.x;
        self.y = point.y;
        self
    }

    pub const fn with_max_size(mut self, size: Size) -> Self {
        self.max_width = size.w;
        self.max_height = size.h;
        self
    }

    pub const fn with_offset(mut self, point: Point) -> Self {
        self.offset_x = point.x;
        self.offset_y = point.y;
        self
    }

    pub const fn with_scale(mut self, scale_x: f32, scale_y: f32) -> Self {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }

    pub const fn with_datum(mut self, datum: TextDatum) -> Self {
        self.datum = datum;
        self
    }
}

fn validate_image_data(data: &[u8]) -> Result<(), Error> {
    (!data.is_empty())
        .then_some(())
        .ok_or(Error::InvalidValue("image data"))
}

fn validate_rgb565_image(rect: Rect, data: &[u16]) -> Result<(), Error> {
    let Some(pixel_count) = rect
        .w
        .checked_mul(rect.h)
        .and_then(|count| usize::try_from(count).ok())
    else {
        return Err(Error::InvalidValue("rgb565 image rect"));
    };
    if rect.w <= 0 || rect.h <= 0 {
        return Err(Error::InvalidValue("rgb565 image rect"));
    }
    if data.len() < pixel_count {
        return Err(Error::InvalidValue("rgb565 image data"));
    }
    Ok(())
}

fn validate_display_resolution(resolution: DisplayResolution) -> Result<(), Error> {
    if !resolution.refresh_rate.is_finite() || resolution.refresh_rate < 0.0 {
        return Err(Error::InvalidValue("display refresh rate"));
    }
    if resolution.pixel_clock == 0 {
        return Err(Error::InvalidValue("display pixel clock"));
    }
    Ok(())
}

#[derive(Debug)]
pub struct Canvas {
    raw: NonNull<c_void>,
}

impl Canvas {
    fn for_display() -> Option<Self> {
        NonNull::new(unsafe { m5unified_sys::m5u_canvas_create_for_display() })
            .map(|raw| Self { raw })
    }

    fn for_cardputer_display() -> Option<Self> {
        NonNull::new(unsafe { m5unified_sys::m5u_canvas_create_for_cardputer_display() })
            .map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    pub fn create_sprite(&mut self, size: Size) -> bool {
        unsafe { m5unified_sys::m5u_canvas_create_sprite(self.raw(), size.w, size.h) }
    }

    pub fn try_create_sprite(&mut self, size: Size) -> Result<(), Error> {
        if size.w <= 0 || size.h <= 0 {
            return Err(Error::InvalidValue("canvas sprite size"));
        }
        self.create_sprite(size)
            .then_some(())
            .ok_or(Error::Unavailable("canvas sprite"))
    }

    pub fn delete_sprite(&mut self) {
        unsafe { m5unified_sys::m5u_canvas_delete_sprite(self.raw()) }
    }

    pub fn push_sprite(&mut self, point: Point) {
        unsafe { m5unified_sys::m5u_canvas_push_sprite(self.raw(), point.x, point.y) }
    }

    pub fn width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_canvas_width(self.raw()) as i32 }
    }

    pub fn height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_canvas_height(self.raw()) as i32 }
    }

    pub fn fill_screen(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_fill_screen(self.raw(), color) }
    }

    pub fn clear(&mut self) {
        self.fill_screen(colors::BLACK);
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_canvas_set_cursor(self.raw(), x, y) }
    }

    pub fn set_text_size(&mut self, size: f32) {
        unsafe { m5unified_sys::m5u_canvas_set_text_size(self.raw(), size) }
    }

    pub fn try_set_text_size(&mut self, size: f32) -> Result<(), Error> {
        validate_canvas_text_size(size)?;
        self.set_text_size(size);
        Ok(())
    }

    pub fn set_text_color(&mut self, fg: u16, bg: u16) {
        unsafe { m5unified_sys::m5u_canvas_set_text_color(self.raw(), fg, bg) }
    }

    pub fn set_text_scroll(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_canvas_set_text_scroll(self.raw(), enable) }
    }

    pub fn set_text_datum(&mut self, datum: TextDatum) {
        unsafe { m5unified_sys::m5u_canvas_set_text_datum(self.raw(), datum.raw()) }
    }

    pub fn text_datum(&self) -> Option<TextDatum> {
        TextDatum::from_raw(unsafe { m5unified_sys::m5u_canvas_get_text_datum(self.raw()) as i32 })
    }

    pub fn set_text_padding(&mut self, padding_x: u32) {
        unsafe { m5unified_sys::m5u_canvas_set_text_padding(self.raw(), padding_x) }
    }

    pub fn text_padding(&self) -> u32 {
        unsafe { m5unified_sys::m5u_canvas_get_text_padding(self.raw()) }
    }

    pub fn text_size_x(&self) -> u8 {
        unsafe { m5unified_sys::m5u_canvas_get_text_size_x(self.raw()) }
    }

    pub fn text_size_y(&self) -> u8 {
        unsafe { m5unified_sys::m5u_canvas_get_text_size_y(self.raw()) }
    }

    pub fn base_color(&self) -> u16 {
        unsafe { m5unified_sys::m5u_canvas_get_base_color(self.raw()) }
    }

    pub fn set_base_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_set_base_color(self.raw(), color) }
    }

    pub fn set_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_set_color(self.raw(), color) }
    }

    pub fn set_rgb_color(&mut self, r: u8, g: u8, b: u8) {
        unsafe { m5unified_sys::m5u_canvas_set_rgb_color(self.raw(), r, g, b) }
    }

    pub fn set_raw_color(&mut self, color: u32) {
        unsafe { m5unified_sys::m5u_canvas_set_raw_color(self.raw(), color) }
    }

    pub fn raw_color(&self) -> u32 {
        unsafe { m5unified_sys::m5u_canvas_get_raw_color(self.raw()) }
    }

    pub fn set_swap_bytes(&mut self, swap: bool) {
        unsafe { m5unified_sys::m5u_canvas_set_swap_bytes(self.raw(), swap) }
    }

    pub fn swap_bytes(&self) -> bool {
        unsafe { m5unified_sys::m5u_canvas_get_swap_bytes(self.raw()) }
    }

    pub fn set_font(&mut self, font: DisplayFont) -> bool {
        unsafe { m5unified_sys::m5u_canvas_set_font(self.raw(), font.raw()) }
    }

    pub fn try_set_font(&mut self, font: DisplayFont) -> Result<(), Error> {
        self.set_font(font)
            .then_some(())
            .ok_or(Error::Unavailable("canvas font"))
    }

    pub fn font_height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_canvas_font_height(self.raw()) as i32 }
    }

    pub fn font_width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_canvas_font_width(self.raw()) as i32 }
    }

    pub fn show_font(&mut self, duration_ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_canvas_show_font(self.raw(), duration_ms) }
    }

    pub fn try_show_font(&mut self, duration_ms: u32) -> Result<(), Error> {
        self.show_font(duration_ms)
            .then_some(())
            .ok_or(Error::Unavailable("canvas font"))
    }

    pub fn unload_font(&mut self) {
        unsafe { m5unified_sys::m5u_canvas_unload_font(self.raw()) }
    }

    pub fn text_width(&self, text: &str) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_canvas_text_width(self.raw(), text.as_ptr()) as i32 })
    }

    pub fn print(&mut self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_canvas_print(self.raw(), text.as_ptr()) }
        Ok(())
    }

    pub fn println(&mut self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_canvas_println(self.raw(), text.as_ptr()) }
        Ok(())
    }

    pub fn draw_center_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_canvas_draw_center_string(self.raw(), text.as_ptr(), x, y) as i32
        })
    }

    pub fn draw_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(
            unsafe {
                m5unified_sys::m5u_canvas_draw_string(self.raw(), text.as_ptr(), x, y) as i32
            },
        )
    }

    pub fn draw_line(&mut self, p0: Point, p1: Point, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_draw_line(self.raw(), p0.x, p0.y, p1.x, p1.y, color) }
    }

    pub fn draw_rect(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_draw_rect(self.raw(), rect.x, rect.y, rect.w, rect.h, color)
        }
    }

    pub fn try_draw_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.draw_rect(rect, color);
        Ok(())
    }

    pub fn fill_rect(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_fill_rect(self.raw(), rect.x, rect.y, rect.w, rect.h, color)
        }
    }

    pub fn try_fill_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.fill_rect(rect, color);
        Ok(())
    }

    pub fn draw_circle(&mut self, center: Point, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_draw_circle(self.raw(), center.x, center.y, radius, color)
        }
    }

    pub fn try_draw_circle(&mut self, center: Point, radius: i32, color: u16) -> Result<(), Error> {
        validate_display_radius(radius)?;
        self.draw_circle(center, radius, color);
        Ok(())
    }

    pub fn fill_circle(&mut self, center: Point, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_fill_circle(self.raw(), center.x, center.y, radius, color)
        }
    }

    pub fn try_fill_circle(&mut self, center: Point, radius: i32, color: u16) -> Result<(), Error> {
        validate_display_radius(radius)?;
        self.fill_circle(center, radius, color);
        Ok(())
    }

    pub fn draw_pixel(&mut self, point: Point, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_draw_pixel(self.raw(), point.x, point.y, color) }
    }

    pub fn read_pixel(&self, point: Point) -> u16 {
        unsafe { m5unified_sys::m5u_canvas_read_pixel(self.raw(), point.x, point.y) }
    }

    pub fn draw_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_draw_fast_hline(self.raw(), x, y, w, color) }
    }

    pub fn try_draw_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) -> Result<(), Error> {
        validate_display_length(w)?;
        self.draw_fast_hline(x, y, w, color);
        Ok(())
    }

    pub fn draw_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_draw_fast_vline(self.raw(), x, y, h, color) }
    }

    pub fn try_draw_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) -> Result<(), Error> {
        validate_display_length(h)?;
        self.draw_fast_vline(x, y, h, color);
        Ok(())
    }

    pub fn draw_round_rect(&mut self, rect: Rect, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_draw_round_rect(
                self.raw(),
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                radius,
                color,
            )
        }
    }

    pub fn try_draw_round_rect(
        &mut self,
        rect: Rect,
        radius: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_rect(rect)?;
        validate_display_radius(radius)?;
        self.draw_round_rect(rect, radius, color);
        Ok(())
    }

    pub fn fill_round_rect(&mut self, rect: Rect, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_fill_round_rect(
                self.raw(),
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                radius,
                color,
            )
        }
    }

    pub fn try_fill_round_rect(
        &mut self,
        rect: Rect,
        radius: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_rect(rect)?;
        validate_display_radius(radius)?;
        self.fill_round_rect(rect, radius, color);
        Ok(())
    }

    pub fn draw_ellipse(&mut self, center: Point, rx: i32, ry: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_draw_ellipse(self.raw(), center.x, center.y, rx, ry, color)
        }
    }

    pub fn try_draw_ellipse(
        &mut self,
        center: Point,
        rx: i32,
        ry: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_radii(rx, ry)?;
        self.draw_ellipse(center, rx, ry, color);
        Ok(())
    }

    pub fn fill_ellipse(&mut self, center: Point, rx: i32, ry: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_fill_ellipse(self.raw(), center.x, center.y, rx, ry, color)
        }
    }

    pub fn try_fill_ellipse(
        &mut self,
        center: Point,
        rx: i32,
        ry: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_radii(rx, ry)?;
        self.fill_ellipse(center, rx, ry, color);
        Ok(())
    }

    pub fn draw_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_canvas_draw_arc(
                self.raw(),
                center.x,
                center.y,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                color,
            )
        }
    }

    pub fn try_draw_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_arc(inner_radius, outer_radius, start_angle, end_angle)?;
        self.draw_arc(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            color,
        );
        Ok(())
    }

    pub fn fill_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_canvas_fill_arc(
                self.raw(),
                center.x,
                center.y,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                color,
            )
        }
    }

    pub fn try_fill_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_arc(inner_radius, outer_radius, start_angle, end_angle)?;
        self.fill_arc(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            color,
        );
        Ok(())
    }

    pub fn draw_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_draw_triangle(
                self.raw(),
                p0.x,
                p0.y,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
                color,
            )
        }
    }

    pub fn fill_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_fill_triangle(
                self.raw(),
                p0.x,
                p0.y,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
                color,
            )
        }
    }

    pub fn progress_bar(&mut self, rect: Rect, value: u8) {
        unsafe {
            m5unified_sys::m5u_canvas_progress_bar(
                self.raw(),
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                value,
            )
        }
    }

    pub fn try_progress_bar(&mut self, rect: Rect, value: u8) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.progress_bar(rect, value);
        Ok(())
    }

    pub fn text_length(&self, text: &str) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_canvas_text_length(self.raw(), text.as_ptr()) as i32 })
    }

    pub fn draw_char(&mut self, ch: char, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_canvas_draw_char(self.raw(), ch as u32, x, y) as i32 }
    }

    pub fn draw_number(&mut self, value: i32, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_canvas_draw_number(self.raw(), value, x, y) as i32 }
    }

    pub fn draw_float(&mut self, value: f32, decimals: u8, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_canvas_draw_float(self.raw(), value, decimals, x, y) as i32 }
    }

    pub fn draw_bmp(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_canvas_draw_bmp(
                self.raw(),
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }

    pub fn try_draw_bmp(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_bmp(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("canvas bmp"))
    }

    pub fn draw_jpg(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_canvas_draw_jpg(
                self.raw(),
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }

    pub fn try_draw_jpg(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_jpg(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("canvas jpg"))
    }

    pub fn draw_png(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_canvas_draw_png(
                self.raw(),
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }

    pub fn try_draw_png(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_png(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("canvas png"))
    }

    pub fn push_image_rgb565(&mut self, rect: Rect, data: &[u16]) -> bool {
        let Some(pixel_count) = rect
            .w
            .checked_mul(rect.h)
            .and_then(|count| usize::try_from(count).ok())
        else {
            return false;
        };
        if rect.w <= 0 || rect.h <= 0 || data.len() < pixel_count {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_canvas_push_image_rgb565(
                self.raw(),
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                data.as_ptr(),
                data.len(),
            )
        }
    }

    pub fn try_push_image_rgb565(&mut self, rect: Rect, data: &[u16]) -> Result<(), Error> {
        validate_rgb565_image(rect, data)?;
        self.push_image_rgb565(rect, data)
            .then_some(())
            .ok_or(Error::Unavailable("canvas rgb565 image"))
    }

    pub fn write_pixel(&mut self, point: Point, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_write_pixel(self.raw(), point.x, point.y, color) }
    }

    pub fn write_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_canvas_write_fast_vline(self.raw(), x, y, h, color) }
    }

    pub fn try_write_fast_vline(
        &mut self,
        x: i32,
        y: i32,
        h: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_length(h)?;
        self.write_fast_vline(x, y, h, color);
        Ok(())
    }

    pub fn set_addr_window(&mut self, rect: Rect) {
        unsafe {
            m5unified_sys::m5u_canvas_set_addr_window(self.raw(), rect.x, rect.y, rect.w, rect.h)
        }
    }

    pub fn try_set_addr_window(&mut self, rect: Rect) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_addr_window(rect);
        Ok(())
    }

    pub fn set_window(&mut self, xs: i32, ys: i32, xe: i32, ye: i32) {
        unsafe { m5unified_sys::m5u_canvas_set_window(self.raw(), xs, ys, xe, ye) }
    }

    pub fn try_set_window(&mut self, xs: i32, ys: i32, xe: i32, ye: i32) -> Result<(), Error> {
        validate_display_window(xs, ys, xe, ye)?;
        self.set_window(xs, ys, xe, ye);
        Ok(())
    }

    pub fn set_clip_rect(&mut self, rect: Rect) {
        unsafe {
            m5unified_sys::m5u_canvas_set_clip_rect(self.raw(), rect.x, rect.y, rect.w, rect.h)
        }
    }

    pub fn try_set_clip_rect(&mut self, rect: Rect) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_clip_rect(rect);
        Ok(())
    }

    pub fn clip_rect(&self) -> Rect {
        let mut x = 0;
        let mut y = 0;
        let mut w = 0;
        let mut h = 0;
        unsafe {
            m5unified_sys::m5u_canvas_get_clip_rect(self.raw(), &mut x, &mut y, &mut w, &mut h)
        }
        Rect { x, y, w, h }
    }

    pub fn clear_clip_rect(&mut self) {
        unsafe { m5unified_sys::m5u_canvas_clear_clip_rect(self.raw()) }
    }

    pub fn scroll(&mut self, dx: i32, dy: i32) {
        unsafe { m5unified_sys::m5u_canvas_scroll(self.raw(), dx, dy) }
    }

    pub fn set_scroll_rect(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_canvas_set_scroll_rect(
                self.raw(),
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                color,
            )
        }
    }

    pub fn try_set_scroll_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_scroll_rect(rect, color);
        Ok(())
    }

    pub fn scroll_rect(&self) -> Rect {
        let mut x = 0;
        let mut y = 0;
        let mut w = 0;
        let mut h = 0;
        unsafe {
            m5unified_sys::m5u_canvas_get_scroll_rect(self.raw(), &mut x, &mut y, &mut w, &mut h)
        }
        Rect { x, y, w, h }
    }

    pub fn clear_scroll_rect(&mut self) {
        unsafe { m5unified_sys::m5u_canvas_clear_scroll_rect(self.raw()) }
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe { m5unified_sys::m5u_canvas_delete(self.raw()) }
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
    fn raw(self) -> i32 {
        match self {
            Self::AtomDisplay => BoardKind::M5AtomDisplay.raw(),
            Self::UnitLcd => BoardKind::M5UnitLcd.raw(),
            Self::UnitOled => BoardKind::M5UnitOled.raw(),
            Self::UnitMiniOled => BoardKind::M5UnitMiniOled.raw(),
            Self::UnitGlass => BoardKind::M5UnitGlass.raw(),
            Self::UnitGlass2 => BoardKind::M5UnitGlass2.raw(),
            Self::UnitRca => BoardKind::M5UnitRca.raw(),
            Self::ModuleDisplay => BoardKind::M5ModuleDisplay.raw(),
            Self::ModuleRca => BoardKind::M5ModuleRca.raw(),
            Self::Raw(value) => value,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DisplayRef {
    index: i32,
}

impl DisplayRef {
    pub fn index(&self) -> usize {
        self.index as usize
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        unsafe { m5unified_sys::m5u_display_set_rotation_at(self.index, rotation) }
    }
    pub fn rotation(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_rotation_at(self.index) as i32 }
    }
    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_display_set_brightness_at(self.index, brightness) }
    }
    pub fn brightness(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_brightness_at(self.index) }
    }
    pub fn set_color_depth(&mut self, depth: u8) {
        unsafe { m5unified_sys::m5u_display_set_color_depth_at(self.index, depth) }
    }
    pub fn color_depth(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_color_depth_at(self.index) }
    }
    pub fn is_epd(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_is_epd_at(self.index) }
    }
    pub fn set_epd_mode(&mut self, mode: EpdMode) {
        unsafe { m5unified_sys::m5u_display_set_epd_mode_at(self.index, mode.raw()) }
    }
    pub fn epd_mode(&self) -> EpdMode {
        EpdMode::from_raw(unsafe { m5unified_sys::m5u_display_get_epd_mode_at(self.index) as i32 })
    }
    pub fn set_epd_fastest(&mut self) {
        self.set_epd_mode(EpdMode::Fastest)
    }
    pub fn set_resolution(&mut self, resolution: DisplayResolution) -> bool {
        unsafe {
            m5unified_sys::m5u_display_set_resolution_at(
                self.index,
                resolution.logical_width,
                resolution.logical_height,
                resolution.refresh_rate,
                resolution.output_width,
                resolution.output_height,
                resolution.scale_w,
                resolution.scale_h,
                resolution.pixel_clock,
            )
        }
    }
    pub fn try_set_resolution(&mut self, resolution: DisplayResolution) -> Result<(), Error> {
        validate_display_resolution(resolution)?;
        self.set_resolution(resolution)
            .then_some(())
            .ok_or(Error::Unavailable("display resolution"))
    }
    pub fn width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_width_at(self.index) as i32 }
    }
    pub fn height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_height_at(self.index) as i32 }
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
    pub fn display(&mut self) {
        unsafe { m5unified_sys::m5u_display_display_at(self.index) }
    }
    pub fn display_busy(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_display_busy_at(self.index) }
    }
    pub fn wait_display(&self) {
        unsafe { m5unified_sys::m5u_display_wait_display_at(self.index) }
    }
    pub fn sleep(&mut self) {
        unsafe { m5unified_sys::m5u_display_sleep_at(self.index) }
    }
    pub fn wakeup(&mut self) {
        unsafe { m5unified_sys::m5u_display_wakeup_at(self.index) }
    }
    pub fn power_save_on(&mut self) {
        unsafe { m5unified_sys::m5u_display_power_save_on_at(self.index) }
    }
    pub fn power_save_off(&mut self) {
        unsafe { m5unified_sys::m5u_display_power_save_off_at(self.index) }
    }
    pub fn power_save(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_display_power_save_at(self.index, enable) }
    }
    pub fn invert_display(&mut self, invert: bool) {
        unsafe { m5unified_sys::m5u_display_invert_display_at(self.index, invert) }
    }
    pub fn cursor_x(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_x_at(self.index) as i32 }
    }
    pub fn cursor_y(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_y_at(self.index) as i32 }
    }
    pub fn set_pivot(&mut self, x: f32, y: f32) {
        unsafe { m5unified_sys::m5u_display_set_pivot_at(self.index, x, y) }
    }
    pub fn try_set_pivot(&mut self, x: f32, y: f32) -> Result<(), Error> {
        validate_display_pivot(x, y)?;
        self.set_pivot(x, y);
        Ok(())
    }
    pub fn pivot(&self) -> (f32, f32) {
        unsafe {
            (
                m5unified_sys::m5u_display_get_pivot_x_at(self.index),
                m5unified_sys::m5u_display_get_pivot_y_at(self.index),
            )
        }
    }
    pub fn clear(&mut self) {
        self.fill_screen(colors::BLACK);
    }
    pub fn fill_screen(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_clear_at(self.index, color) }
    }
    pub fn draw_bmp(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_display_draw_bmp_at(
                self.index,
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }
    pub fn try_draw_bmp(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_bmp(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("display bmp"))
    }
    pub fn draw_jpg(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_display_draw_jpg_at(
                self.index,
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }
    pub fn try_draw_jpg(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_jpg(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("display jpg"))
    }
    pub fn draw_png(&mut self, data: &[u8], options: ImageDrawOptions) -> bool {
        if data.is_empty() {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_display_draw_png_at(
                self.index,
                data.as_ptr(),
                data.len(),
                options.x,
                options.y,
                options.max_width,
                options.max_height,
                options.offset_x,
                options.offset_y,
                options.scale_x,
                options.scale_y,
                options.datum.raw(),
            )
        }
    }
    pub fn try_draw_png(&mut self, data: &[u8], options: ImageDrawOptions) -> Result<(), Error> {
        validate_image_data(data)?;
        self.draw_png(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("display png"))
    }
    pub fn push_image_rgb565(&mut self, rect: Rect, data: &[u16]) -> bool {
        let Some(pixel_count) = rect
            .w
            .checked_mul(rect.h)
            .and_then(|count| usize::try_from(count).ok())
        else {
            return false;
        };
        if rect.w <= 0 || rect.h <= 0 || data.len() < pixel_count {
            return false;
        }
        unsafe {
            m5unified_sys::m5u_display_push_image_rgb565_at(
                self.index,
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                data.as_ptr(),
                data.len(),
            )
        }
    }
    pub fn try_push_image_rgb565(&mut self, rect: Rect, data: &[u16]) -> Result<(), Error> {
        validate_rgb565_image(rect, data)?;
        self.push_image_rgb565(rect, data)
            .then_some(())
            .ok_or(Error::Unavailable("display rgb565 image"))
    }
    pub fn set_cursor(&mut self, x: i32, y: i32) {
        unsafe { m5unified_sys::m5u_display_set_cursor_at(self.index, x, y) }
    }
    pub fn set_text_size(&mut self, size: i32) {
        unsafe { m5unified_sys::m5u_display_set_text_size_at(self.index, size) }
    }
    pub fn try_set_text_size(&mut self, size: i32) -> Result<(), Error> {
        validate_display_text_size(size)?;
        self.set_text_size(size);
        Ok(())
    }
    pub fn set_text_color(&mut self, fg: u16, bg: u16) {
        unsafe { m5unified_sys::m5u_display_set_text_color_at(self.index, fg, bg) }
    }
    pub fn set_text_datum(&mut self, datum: TextDatum) {
        unsafe { m5unified_sys::m5u_display_set_text_datum_at(self.index, datum as c_int) }
    }
    pub fn text_datum(&self) -> Option<TextDatum> {
        TextDatum::from_raw(unsafe {
            m5unified_sys::m5u_display_get_text_datum_at(self.index) as i32
        })
    }
    pub fn set_text_padding(&mut self, padding_x: u32) {
        unsafe { m5unified_sys::m5u_display_set_text_padding_at(self.index, padding_x) }
    }
    pub fn text_padding(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_text_padding_at(self.index) }
    }
    pub fn text_size_x(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_text_size_x_at(self.index) }
    }
    pub fn text_size_y(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_text_size_y_at(self.index) }
    }
    pub fn font_height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_height_at(self.index) as i32 }
    }
    pub fn font_width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_width_at(self.index) as i32 }
    }
    pub fn set_font(&mut self, font: DisplayFont) -> bool {
        unsafe { m5unified_sys::m5u_display_set_font_at(self.index, font.raw()) }
    }
    pub fn try_set_font(&mut self, font: DisplayFont) -> Result<(), Error> {
        self.set_font(font)
            .then_some(())
            .ok_or(Error::Unavailable("display font"))
    }
    pub fn show_font(&mut self, duration_ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_display_show_font_at(self.index, duration_ms) }
    }
    pub fn try_show_font(&mut self, duration_ms: u32) -> Result<(), Error> {
        self.show_font(duration_ms)
            .then_some(())
            .ok_or(Error::Unavailable("display font"))
    }
    pub fn unload_font(&mut self) {
        unsafe { m5unified_sys::m5u_display_unload_font_at(self.index) }
    }
    pub fn font_height_for(&self, font: DisplayFont) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_height_for(font.raw()) as i32 }
    }
    pub fn font_width_for(&self, font: DisplayFont) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_width_for(font.raw()) as i32 }
    }
    pub fn base_color(&self) -> u16 {
        unsafe { m5unified_sys::m5u_display_get_base_color_at(self.index) }
    }
    pub fn set_base_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_set_base_color_at(self.index, color) }
    }
    pub fn set_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_set_color_at(self.index, color) }
    }
    pub fn set_rgb_color(&mut self, r: u8, g: u8, b: u8) {
        unsafe { m5unified_sys::m5u_display_set_rgb_color_at(self.index, r, g, b) }
    }
    pub fn set_raw_color(&mut self, color: u32) {
        unsafe { m5unified_sys::m5u_display_set_raw_color_at(self.index, color) }
    }
    pub fn raw_color(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_raw_color_at(self.index) }
    }
    pub fn palette_count(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_palette_count_at(self.index) }
    }
    pub fn set_swap_bytes(&mut self, swap: bool) {
        unsafe { m5unified_sys::m5u_display_set_swap_bytes_at(self.index, swap) }
    }
    pub fn swap_bytes(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_get_swap_bytes_at(self.index) }
    }
    pub fn swap565(&self, r: u8, g: u8, b: u8) -> u16 {
        unsafe { m5unified_sys::m5u_display_swap565_at(self.index, r, g, b) }
    }
    pub fn swap888(&self, r: u8, g: u8, b: u8) -> u32 {
        unsafe { m5unified_sys::m5u_display_swap888_at(self.index, r, g, b) }
    }
    pub fn set_text_wrap(&mut self, wrap_x: bool, wrap_y: bool) {
        unsafe { m5unified_sys::m5u_display_set_text_wrap_at(self.index, wrap_x, wrap_y) }
    }
    pub fn color888(&self, r: u8, g: u8, b: u8) -> u16 {
        unsafe { m5unified_sys::m5u_display_color888_at(self.index, r, g, b) }
    }
    pub fn text_length(&self, text: &str) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_text_length_at(self.index, text.as_ptr()) as i32 })
    }
    pub fn text_width(&self, text: &str) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_text_width_at(self.index, text.as_ptr()) as i32 })
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
    pub fn draw_center_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_display_draw_center_string_at(self.index, text.as_ptr(), x, y) as i32
        })
    }
    pub fn draw_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe {
            m5unified_sys::m5u_display_draw_string_at(self.index, text.as_ptr(), x, y) as i32
        })
    }
    pub fn draw_char(&mut self, ch: char, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_char_at(self.index, ch as u32, x, y) as i32 }
    }
    pub fn draw_number(&mut self, value: i32, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_number_at(self.index, value, x, y) as i32 }
    }
    pub fn draw_float(&mut self, value: f32, decimals: u8, x: i32, y: i32) -> i32 {
        unsafe {
            m5unified_sys::m5u_display_draw_float_at(self.index, value, decimals, x, y) as i32
        }
    }
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_line_at(self.index, x0, y0, x1, y1, color) }
    }
    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_pixel_at(self.index, x, y, color) }
    }
    pub fn draw_point(&mut self, point: Point, color: u16) {
        self.draw_pixel(point.x, point.y, color)
    }
    pub fn read_pixel(&self, x: i32, y: i32) -> u16 {
        unsafe { m5unified_sys::m5u_display_read_pixel_at(self.index, x, y) }
    }
    pub fn read_point(&self, point: Point) -> u16 {
        self.read_pixel(point.x, point.y)
    }
    pub fn draw_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_fast_hline_at(self.index, x, y, w, color) }
    }
    pub fn try_draw_fast_hline(&mut self, x: i32, y: i32, w: i32, color: u16) -> Result<(), Error> {
        validate_display_length(w)?;
        self.draw_fast_hline(x, y, w, color);
        Ok(())
    }
    pub fn draw_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_fast_vline_at(self.index, x, y, h, color) }
    }
    pub fn try_draw_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) -> Result<(), Error> {
        validate_display_length(h)?;
        self.draw_fast_vline(x, y, h, color);
        Ok(())
    }
    pub fn draw_rect(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_rect_at(
                self.index, rect.x, rect.y, rect.w, rect.h, color,
            )
        }
    }
    pub fn try_draw_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.draw_rect(rect, color);
        Ok(())
    }
    pub fn fill_rect(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_rect_at(
                self.index, rect.x, rect.y, rect.w, rect.h, color,
            )
        }
    }
    pub fn try_fill_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.fill_rect(rect, color);
        Ok(())
    }
    pub fn fill_rect_alpha(&mut self, rect: Rect, alpha: u8, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_rect_alpha_at(
                self.index, rect.x, rect.y, rect.w, rect.h, alpha, color,
            )
        }
    }
    pub fn try_fill_rect_alpha(&mut self, rect: Rect, alpha: u8, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.fill_rect_alpha(rect, alpha, color);
        Ok(())
    }
    pub fn draw_round_rect(&mut self, rect: Rect, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_round_rect_at(
                self.index, rect.x, rect.y, rect.w, rect.h, radius, color,
            )
        }
    }
    pub fn try_draw_round_rect(
        &mut self,
        rect: Rect,
        radius: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_rect(rect)?;
        validate_display_radius(radius)?;
        self.draw_round_rect(rect, radius, color);
        Ok(())
    }
    pub fn fill_round_rect(&mut self, rect: Rect, radius: i32, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_round_rect_at(
                self.index, rect.x, rect.y, rect.w, rect.h, radius, color,
            )
        }
    }
    pub fn try_fill_round_rect(
        &mut self,
        rect: Rect,
        radius: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_rect(rect)?;
        validate_display_radius(radius)?;
        self.fill_round_rect(rect, radius, color);
        Ok(())
    }
    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_circle_at(self.index, x, y, r, color) }
    }
    pub fn try_draw_circle(&mut self, x: i32, y: i32, r: i32, color: u16) -> Result<(), Error> {
        validate_display_radius(r)?;
        self.draw_circle(x, y, r, color);
        Ok(())
    }
    pub fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_circle_at(self.index, x, y, r, color) }
    }
    pub fn try_fill_circle(&mut self, x: i32, y: i32, r: i32, color: u16) -> Result<(), Error> {
        validate_display_radius(r)?;
        self.fill_circle(x, y, r, color);
        Ok(())
    }
    pub fn draw_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_draw_ellipse_at(self.index, x, y, rx, ry, color) }
    }
    pub fn try_draw_ellipse(
        &mut self,
        x: i32,
        y: i32,
        rx: i32,
        ry: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_radii(rx, ry)?;
        self.draw_ellipse(x, y, rx, ry, color);
        Ok(())
    }
    pub fn fill_ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_fill_ellipse_at(self.index, x, y, rx, ry, color) }
    }
    pub fn try_fill_ellipse(
        &mut self,
        x: i32,
        y: i32,
        rx: i32,
        ry: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_radii(rx, ry)?;
        self.fill_ellipse(x, y, rx, ry, color);
        Ok(())
    }
    pub fn draw_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_draw_arc_at(
                self.index,
                center.x,
                center.y,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                color,
            )
        }
    }
    pub fn try_draw_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_arc(inner_radius, outer_radius, start_angle, end_angle)?;
        self.draw_arc(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            color,
        );
        Ok(())
    }
    pub fn fill_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) {
        unsafe {
            m5unified_sys::m5u_display_fill_arc_at(
                self.index,
                center.x,
                center.y,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                color,
            )
        }
    }
    pub fn try_fill_arc(
        &mut self,
        center: Point,
        inner_radius: i32,
        outer_radius: i32,
        start_angle: f32,
        end_angle: f32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_arc(inner_radius, outer_radius, start_angle, end_angle)?;
        self.fill_arc(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            color,
        );
        Ok(())
    }
    pub fn draw_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_draw_triangle_at(
                self.index, p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color,
            )
        }
    }
    pub fn fill_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_fill_triangle_at(
                self.index, p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, color,
            )
        }
    }
    pub fn progress_bar(&mut self, rect: Rect, value: u8) {
        unsafe {
            m5unified_sys::m5u_display_progress_bar_at(
                self.index, rect.x, rect.y, rect.w, rect.h, value,
            )
        }
    }
    pub fn try_progress_bar(&mut self, rect: Rect, value: u8) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.progress_bar(rect, value);
        Ok(())
    }
    pub fn write_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_pixel_at(self.index, x, y, color) }
    }
    pub fn write_point(&mut self, point: Point, color: u16) {
        self.write_pixel(point.x, point.y, color)
    }
    pub fn write_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_fast_vline_at(self.index, x, y, h, color) }
    }
    pub fn try_write_fast_vline(
        &mut self,
        x: i32,
        y: i32,
        h: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_length(h)?;
        self.write_fast_vline(x, y, h, color);
        Ok(())
    }
    pub fn set_addr_window(&mut self, rect: Rect) {
        unsafe {
            m5unified_sys::m5u_display_set_addr_window_at(
                self.index, rect.x, rect.y, rect.w, rect.h,
            )
        }
    }
    pub fn try_set_addr_window(&mut self, rect: Rect) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_addr_window(rect);
        Ok(())
    }
    pub fn set_window(&mut self, xs: i32, ys: i32, xe: i32, ye: i32) {
        unsafe { m5unified_sys::m5u_display_set_window_at(self.index, xs, ys, xe, ye) }
    }
    pub fn try_set_window(&mut self, xs: i32, ys: i32, xe: i32, ye: i32) -> Result<(), Error> {
        validate_display_window(xs, ys, xe, ye)?;
        self.set_window(xs, ys, xe, ye);
        Ok(())
    }
    pub fn set_clip_rect(&mut self, rect: Rect) {
        unsafe {
            m5unified_sys::m5u_display_set_clip_rect_at(self.index, rect.x, rect.y, rect.w, rect.h)
        }
    }
    pub fn try_set_clip_rect(&mut self, rect: Rect) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_clip_rect(rect);
        Ok(())
    }
    pub fn clip_rect(&self) -> Rect {
        let mut x = 0;
        let mut y = 0;
        let mut w = 0;
        let mut h = 0;
        unsafe {
            m5unified_sys::m5u_display_get_clip_rect_at(self.index, &mut x, &mut y, &mut w, &mut h)
        }
        Rect { x, y, w, h }
    }
    pub fn clear_clip_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_clip_rect_at(self.index) }
    }
    pub fn scroll(&mut self, dx: i32, dy: i32) {
        unsafe { m5unified_sys::m5u_display_scroll_at(self.index, dx, dy) }
    }
    pub fn set_text_scroll(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_display_set_text_scroll_at(self.index, enable) }
    }
    pub fn set_scroll_rect(&mut self, rect: Rect, color: u16) {
        unsafe {
            m5unified_sys::m5u_display_set_scroll_rect_at(
                self.index, rect.x, rect.y, rect.w, rect.h, color,
            )
        }
    }
    pub fn try_set_scroll_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_scroll_rect(rect, color);
        Ok(())
    }
    pub fn scroll_rect(&self) -> Rect {
        let mut x = 0;
        let mut y = 0;
        let mut w = 0;
        let mut h = 0;
        unsafe {
            m5unified_sys::m5u_display_get_scroll_rect_at(
                self.index, &mut x, &mut y, &mut w, &mut h,
            )
        }
        Rect { x, y, w, h }
    }
    pub fn clear_scroll_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_scroll_rect_at(self.index) }
    }
}

impl M5Unified {
    pub fn display_count(&self) -> usize {
        display_count()
    }

    pub fn display(&self, index: usize) -> Option<DisplayRef> {
        display_ref(index)
    }

    pub fn try_display(&self, index: usize) -> Result<DisplayRef, Error> {
        try_display_ref(index)
    }

    pub fn display_index(&self, kind: DisplayKind) -> Option<usize> {
        display_index(kind)
    }

    pub fn display_by_kind(&self, kind: DisplayKind) -> Option<DisplayRef> {
        display_ref_by_kind(kind)
    }

    pub fn try_display_by_kind(&self, kind: DisplayKind) -> Result<DisplayRef, Error> {
        try_display_ref_by_kind(kind)
    }
}

fn display_count() -> usize {
    unsafe { m5unified_sys::m5u_display_count().max(0) as usize }
}

fn display_ref(index: usize) -> Option<DisplayRef> {
    (index < display_count()).then_some(DisplayRef {
        index: index as i32,
    })
}

fn try_display_ref(index: usize) -> Result<DisplayRef, Error> {
    display_ref(index).ok_or(Error::InvalidValue("display index"))
}

fn display_index(kind: DisplayKind) -> Option<usize> {
    let index = unsafe { m5unified_sys::m5u_display_index_for_kind(kind.raw() as c_int) };
    (index >= 0).then_some(index as usize)
}

fn display_ref_by_kind(kind: DisplayKind) -> Option<DisplayRef> {
    display_index(kind).and_then(display_ref)
}

fn try_display_ref_by_kind(kind: DisplayKind) -> Result<DisplayRef, Error> {
    display_ref_by_kind(kind).ok_or(Error::Unavailable("display kind"))
}

impl Display {
    pub fn rotation(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_rotation() as i32 }
    }
    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_display_set_brightness(brightness) }
    }
    pub fn brightness(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_brightness() }
    }
    pub fn set_color_depth(&mut self, depth: u8) {
        unsafe { m5unified_sys::m5u_display_set_color_depth(depth) }
    }
    pub fn color_depth(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_color_depth() }
    }
    pub fn is_epd(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_is_epd() }
    }
    pub fn set_epd_mode(&mut self, mode: EpdMode) {
        unsafe { m5unified_sys::m5u_display_set_epd_mode(mode.raw()) }
    }
    pub fn epd_mode(&self) -> EpdMode {
        EpdMode::from_raw(unsafe { m5unified_sys::m5u_display_get_epd_mode() as i32 })
    }
    pub fn set_epd_fastest(&mut self) {
        unsafe { m5unified_sys::m5u_display_set_epd_fastest() }
    }
    pub fn set_resolution(&mut self, resolution: DisplayResolution) -> bool {
        unsafe {
            m5unified_sys::m5u_display_set_resolution(
                resolution.logical_width,
                resolution.logical_height,
                resolution.refresh_rate,
                resolution.output_width,
                resolution.output_height,
                resolution.scale_w,
                resolution.scale_h,
                resolution.pixel_clock,
            )
        }
    }
    pub fn try_set_resolution(&mut self, resolution: DisplayResolution) -> Result<(), Error> {
        validate_display_resolution(resolution)?;
        self.set_resolution(resolution)
            .then_some(())
            .ok_or(Error::Unavailable("display resolution"))
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
    pub fn sleep(&mut self) {
        unsafe { m5unified_sys::m5u_display_sleep() }
    }
    pub fn wakeup(&mut self) {
        unsafe { m5unified_sys::m5u_display_wakeup() }
    }
    pub fn power_save_on(&mut self) {
        unsafe { m5unified_sys::m5u_display_power_save_on() }
    }
    pub fn power_save_off(&mut self) {
        unsafe { m5unified_sys::m5u_display_power_save_off() }
    }
    pub fn power_save(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_display_power_save(enable) }
    }
    pub fn invert_display(&mut self, invert: bool) {
        unsafe { m5unified_sys::m5u_display_invert_display(invert) }
    }
    pub fn cursor_x(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_x() as i32 }
    }
    pub fn cursor_y(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_get_cursor_y() as i32 }
    }
    pub fn set_pivot(&mut self, x: f32, y: f32) {
        unsafe { m5unified_sys::m5u_display_set_pivot(x, y) }
    }
    pub fn try_set_pivot(&mut self, x: f32, y: f32) -> Result<(), Error> {
        validate_display_pivot(x, y)?;
        self.set_pivot(x, y);
        Ok(())
    }
    pub fn pivot(&self) -> (f32, f32) {
        unsafe {
            (
                m5unified_sys::m5u_display_get_pivot_x(),
                m5unified_sys::m5u_display_get_pivot_y(),
            )
        }
    }
    pub fn font_height(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_height() as i32 }
    }
    pub fn font_width(&self) -> i32 {
        unsafe { m5unified_sys::m5u_display_font_width() as i32 }
    }
    pub fn base_color(&self) -> u16 {
        unsafe { m5unified_sys::m5u_display_get_base_color() }
    }
    pub fn set_base_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_set_base_color(color) }
    }
    pub fn set_color(&mut self, color: u16) {
        unsafe { m5unified_sys::m5u_display_set_color(color) }
    }
    pub fn set_rgb_color(&mut self, r: u8, g: u8, b: u8) {
        unsafe { m5unified_sys::m5u_display_set_rgb_color(r, g, b) }
    }
    pub fn set_raw_color(&mut self, color: u32) {
        unsafe { m5unified_sys::m5u_display_set_raw_color(color) }
    }
    pub fn raw_color(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_raw_color() }
    }
    pub fn palette_count(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_palette_count() }
    }
    pub fn set_swap_bytes(&mut self, swap: bool) {
        unsafe { m5unified_sys::m5u_display_set_swap_bytes(swap) }
    }
    pub fn swap_bytes(&self) -> bool {
        unsafe { m5unified_sys::m5u_display_get_swap_bytes() }
    }
    pub fn swap565(&self, r: u8, g: u8, b: u8) -> u16 {
        unsafe { m5unified_sys::m5u_display_swap565(r, g, b) }
    }
    pub fn swap888(&self, r: u8, g: u8, b: u8) -> u32 {
        unsafe { m5unified_sys::m5u_display_swap888(r, g, b) }
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
    pub fn set_text_padding(&mut self, padding_x: u32) {
        unsafe { m5unified_sys::m5u_display_set_text_padding(padding_x) }
    }
    pub fn text_padding(&self) -> u32 {
        unsafe { m5unified_sys::m5u_display_get_text_padding() }
    }
    pub fn text_size_x(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_text_size_x() }
    }
    pub fn text_size_y(&self) -> u8 {
        unsafe { m5unified_sys::m5u_display_get_text_size_y() }
    }
    pub fn text_length(&self, text: &str) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_text_length(text.as_ptr()) as i32 })
    }
    pub fn text_width(&self, text: &str) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_text_width(text.as_ptr()) as i32 })
    }
    pub fn draw_center_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_draw_center_string(text.as_ptr(), x, y) as i32 })
    }
    pub fn draw_string(&mut self, text: &str, x: i32, y: i32) -> Result<i32, Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        Ok(unsafe { m5unified_sys::m5u_display_draw_string(text.as_ptr(), x, y) as i32 })
    }
    pub fn draw_char(&mut self, ch: char, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_char(ch as u32, x, y) as i32 }
    }
    pub fn draw_number(&mut self, value: i32, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_number(value, x, y) as i32 }
    }
    pub fn draw_float(&mut self, value: f32, decimals: u8, x: i32, y: i32) -> i32 {
        unsafe { m5unified_sys::m5u_display_draw_float(value, decimals, x, y) as i32 }
    }
    pub fn read_pixel(&self, x: i32, y: i32) -> u16 {
        unsafe { m5unified_sys::m5u_display_read_pixel(x, y) }
    }
    pub fn read_point(&self, point: Point) -> u16 {
        self.read_pixel(point.x, point.y)
    }
    pub fn write_pixel(&mut self, x: i32, y: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_pixel(x, y, color) }
    }
    pub fn write_point(&mut self, point: Point, color: u16) {
        self.write_pixel(point.x, point.y, color)
    }
    pub fn write_fast_vline(&mut self, x: i32, y: i32, h: i32, color: u16) {
        unsafe { m5unified_sys::m5u_display_write_fast_vline(x, y, h, color) }
    }
    pub fn try_write_fast_vline(
        &mut self,
        x: i32,
        y: i32,
        h: i32,
        color: u16,
    ) -> Result<(), Error> {
        validate_display_length(h)?;
        self.write_fast_vline(x, y, h, color);
        Ok(())
    }
    pub fn set_addr_window(&mut self, rect: Rect) {
        unsafe { m5unified_sys::m5u_display_set_addr_window(rect.x, rect.y, rect.w, rect.h) }
    }
    pub fn try_set_addr_window(&mut self, rect: Rect) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_addr_window(rect);
        Ok(())
    }
    pub fn set_window(&mut self, xs: i32, ys: i32, xe: i32, ye: i32) {
        unsafe { m5unified_sys::m5u_display_set_window(xs, ys, xe, ye) }
    }
    pub fn try_set_window(&mut self, xs: i32, ys: i32, xe: i32, ye: i32) -> Result<(), Error> {
        validate_display_window(xs, ys, xe, ye)?;
        self.set_window(xs, ys, xe, ye);
        Ok(())
    }
    pub fn set_clip_rect(&mut self, rect: Rect) {
        unsafe { m5unified_sys::m5u_display_set_clip_rect(rect.x, rect.y, rect.w, rect.h) }
    }
    pub fn try_set_clip_rect(&mut self, rect: Rect) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_clip_rect(rect);
        Ok(())
    }
    pub fn clip_rect(&self) -> Rect {
        let mut x = 0;
        let mut y = 0;
        let mut w = 0;
        let mut h = 0;
        unsafe { m5unified_sys::m5u_display_get_clip_rect(&mut x, &mut y, &mut w, &mut h) }
        Rect { x, y, w, h }
    }
    pub fn clear_clip_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_clip_rect() }
    }
    pub fn scroll(&mut self, dx: i32, dy: i32) {
        unsafe { m5unified_sys::m5u_display_scroll(dx, dy) }
    }
    pub fn set_text_scroll(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_display_set_text_scroll(enable) }
    }
    pub fn set_scroll_rect(&mut self, rect: Rect, color: u16) {
        unsafe { m5unified_sys::m5u_display_set_scroll_rect(rect.x, rect.y, rect.w, rect.h, color) }
    }
    pub fn try_set_scroll_rect(&mut self, rect: Rect, color: u16) -> Result<(), Error> {
        validate_display_rect(rect)?;
        self.set_scroll_rect(rect, color);
        Ok(())
    }
    pub fn scroll_rect(&self) -> Rect {
        let mut x = 0;
        let mut y = 0;
        let mut w = 0;
        let mut h = 0;
        unsafe { m5unified_sys::m5u_display_get_scroll_rect(&mut x, &mut y, &mut w, &mut h) }
        Rect { x, y, w, h }
    }
    pub fn clear_scroll_rect(&mut self) {
        unsafe { m5unified_sys::m5u_display_clear_scroll_rect() }
    }
    pub fn color888(&self, r: u8, g: u8, b: u8) -> u16 {
        unsafe { m5unified_sys::m5u_display_color888(r, g, b) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MicConfig {
    pub pin_data_in: i32,
    pub pin_bck: i32,
    pub pin_mck: i32,
    pub pin_ws: i32,
    pub sample_rate: u32,
    pub left_channel: bool,
    pub stereo: bool,
    pub over_sampling: u8,
    pub magnification: u8,
    pub noise_filter_level: u8,
    pub use_adc: bool,
    pub dma_buf_len: usize,
    pub dma_buf_count: usize,
    pub task_priority: u8,
    pub task_pinned_core: u8,
    pub i2s_port: i32,
}

impl Default for MicConfig {
    fn default() -> Self {
        m5unified_sys::m5u_mic_config_t::default().into()
    }
}

impl MicConfig {
    pub const fn with_pins(
        mut self,
        pin_data_in: i32,
        pin_bck: i32,
        pin_mck: i32,
        pin_ws: i32,
    ) -> Self {
        self.pin_data_in = pin_data_in;
        self.pin_bck = pin_bck;
        self.pin_mck = pin_mck;
        self.pin_ws = pin_ws;
        self
    }

    pub const fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub const fn with_left_channel(mut self, left_channel: bool) -> Self {
        self.left_channel = left_channel;
        self
    }

    pub const fn with_stereo(mut self, stereo: bool) -> Self {
        self.stereo = stereo;
        self
    }

    pub const fn with_over_sampling(mut self, over_sampling: u8) -> Self {
        self.over_sampling = over_sampling;
        self
    }

    pub const fn with_magnification(mut self, magnification: u8) -> Self {
        self.magnification = magnification;
        self
    }

    pub const fn with_noise_filter_level(mut self, noise_filter_level: u8) -> Self {
        self.noise_filter_level = noise_filter_level;
        self
    }

    pub const fn with_adc(mut self, use_adc: bool) -> Self {
        self.use_adc = use_adc;
        self
    }

    pub const fn with_dma_buffer(mut self, len: usize, count: usize) -> Self {
        self.dma_buf_len = len;
        self.dma_buf_count = count;
        self
    }

    pub const fn with_task(mut self, priority: u8, pinned_core: u8) -> Self {
        self.task_priority = priority;
        self.task_pinned_core = pinned_core;
        self
    }

    pub const fn with_i2s_port(mut self, i2s_port: i32) -> Self {
        self.i2s_port = i2s_port;
        self
    }

    pub const fn pins(self) -> (i32, i32, i32, i32) {
        (self.pin_data_in, self.pin_bck, self.pin_mck, self.pin_ws)
    }

    pub const fn sample_rate(self) -> u32 {
        self.sample_rate
    }

    pub const fn uses_left_channel(self) -> bool {
        self.left_channel
    }

    pub const fn is_stereo(self) -> bool {
        self.stereo
    }

    pub const fn over_sampling(self) -> u8 {
        self.over_sampling
    }

    pub const fn magnification(self) -> u8 {
        self.magnification
    }

    pub const fn noise_filter_level(self) -> u8 {
        self.noise_filter_level
    }

    pub const fn uses_adc(self) -> bool {
        self.use_adc
    }

    pub const fn dma_buffer(self) -> (usize, usize) {
        (self.dma_buf_len, self.dma_buf_count)
    }

    pub const fn task(self) -> (u8, u8) {
        (self.task_priority, self.task_pinned_core)
    }

    pub const fn i2s_port(self) -> i32 {
        self.i2s_port
    }
}

impl From<m5unified_sys::m5u_mic_config_t> for MicConfig {
    fn from(raw: m5unified_sys::m5u_mic_config_t) -> Self {
        Self {
            pin_data_in: raw.pin_data_in,
            pin_bck: raw.pin_bck,
            pin_mck: raw.pin_mck,
            pin_ws: raw.pin_ws,
            sample_rate: raw.sample_rate,
            left_channel: raw.left_channel,
            stereo: raw.stereo,
            over_sampling: raw.over_sampling,
            magnification: raw.magnification,
            noise_filter_level: raw.noise_filter_level,
            use_adc: raw.use_adc,
            dma_buf_len: raw.dma_buf_len,
            dma_buf_count: raw.dma_buf_count,
            task_priority: raw.task_priority,
            task_pinned_core: raw.task_pinned_core,
            i2s_port: raw.i2s_port,
        }
    }
}

impl From<MicConfig> for m5unified_sys::m5u_mic_config_t {
    fn from(config: MicConfig) -> Self {
        Self {
            pin_data_in: config.pin_data_in,
            pin_bck: config.pin_bck,
            pin_mck: config.pin_mck,
            pin_ws: config.pin_ws,
            sample_rate: config.sample_rate,
            left_channel: config.left_channel,
            stereo: config.stereo,
            over_sampling: config.over_sampling,
            magnification: config.magnification,
            noise_filter_level: config.noise_filter_level,
            use_adc: config.use_adc,
            dma_buf_len: config.dma_buf_len,
            dma_buf_count: config.dma_buf_count,
            task_priority: config.task_priority,
            task_pinned_core: config.task_pinned_core,
            i2s_port: config.i2s_port,
        }
    }
}

impl Mic {
    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_mic_is_enabled() }
    }
    pub fn is_recording(&self) -> bool {
        unsafe { m5unified_sys::m5u_mic_is_recording() }
    }
    pub fn end(&mut self) {
        unsafe { m5unified_sys::m5u_mic_end() }
    }
    pub fn record_i16_at(&mut self, buffer: &mut [i16], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_mic_record_i16_at(buffer.as_mut_ptr(), buffer.len(), sample_rate_hz)
        }
    }
    pub fn try_record_i16_at(
        &mut self,
        buffer: &mut [i16],
        sample_rate_hz: u32,
    ) -> Result<(), Error> {
        validate_audio_data(buffer.len())?;
        validate_audio_sample_rate(sample_rate_hz)?;
        self.record_i16_at(buffer, sample_rate_hz)
            .then_some(())
            .ok_or(Error::Unavailable("microphone recording"))
    }
    pub fn stats_at(&mut self, buffer: &mut [i16], sample_rate_hz: u32) -> Option<MicStats> {
        if !self.record_i16_at(buffer, sample_rate_hz) {
            return None;
        }
        analyze_i16_samples(buffer)
    }
    pub fn rms_at(&mut self, buffer: &mut [i16], sample_rate_hz: u32) -> Option<f32> {
        if !self.record_i16_at(buffer, sample_rate_hz) {
            return None;
        }
        rms_i16_samples(buffer)
    }
    pub fn try_rms_at(&mut self, buffer: &mut [i16], sample_rate_hz: u32) -> Result<f32, Error> {
        self.try_record_i16_at(buffer, sample_rate_hz)?;
        Ok(rms_i16_samples(buffer).unwrap_or_default())
    }
    pub fn try_stats_at(
        &mut self,
        buffer: &mut [i16],
        sample_rate_hz: u32,
    ) -> Result<MicStats, Error> {
        self.try_record_i16_at(buffer, sample_rate_hz)?;
        Ok(analyze_i16_samples(buffer).unwrap_or_default())
    }
    pub fn config(&self) -> MicConfig {
        self.try_config().unwrap_or_default()
    }
    pub fn try_config(&self) -> Result<MicConfig, Error> {
        let mut raw = m5unified_sys::m5u_mic_config_t::default();
        if unsafe { m5unified_sys::m5u_mic_get_config(&mut raw) } {
            Ok(raw.into())
        } else {
            Err(Error::Unavailable("microphone config"))
        }
    }
    pub fn set_config(&mut self, config: MicConfig) -> Result<(), Error> {
        validate_mic_config(config)?;
        let raw = m5unified_sys::m5u_mic_config_t::from(config);
        unsafe { m5unified_sys::m5u_mic_set_config(&raw) }
        Ok(())
    }
    pub fn noise_filter_level(&self) -> i32 {
        unsafe { m5unified_sys::m5u_mic_get_noise_filter_level() as i32 }
    }
    pub fn set_noise_filter_level(&mut self, level: u8) -> bool {
        unsafe { m5unified_sys::m5u_mic_set_noise_filter_level(c_int::from(level)) }
    }
    pub fn try_set_noise_filter_level(&mut self, level: u8) -> Result<(), Error> {
        self.set_noise_filter_level(level)
            .then_some(())
            .ok_or(Error::Unavailable("microphone noise filter"))
    }
}

impl Speaker {
    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_speaker_is_enabled() }
    }
    pub fn is_running(&self) -> bool {
        unsafe { m5unified_sys::m5u_speaker_is_running() }
    }
    pub fn end(&mut self) {
        unsafe { m5unified_sys::m5u_speaker_end() }
    }
    pub fn config(&self) -> SpeakerConfig {
        self.try_config().unwrap_or_default()
    }
    pub fn try_config(&self) -> Result<SpeakerConfig, Error> {
        let mut raw = m5unified_sys::m5u_speaker_config_t::default();
        if unsafe { m5unified_sys::m5u_speaker_get_config(&mut raw) } {
            Ok(raw.into())
        } else {
            Err(Error::Unavailable("speaker config"))
        }
    }
    pub fn set_config(&mut self, config: SpeakerConfig) {
        let raw = m5unified_sys::m5u_speaker_config_t::from(config);
        unsafe { m5unified_sys::m5u_speaker_set_config(&raw) }
    }
    pub fn try_set_config(&mut self, config: SpeakerConfig) -> Result<(), Error> {
        validate_speaker_config(config)?;
        self.set_config(config);
        Ok(())
    }
    pub fn volume(&self) -> u8 {
        unsafe { m5unified_sys::m5u_speaker_get_volume() }
    }
    pub fn tone_ex(&mut self, frequency_hz: f32, duration_ms: u32, channel: Option<u8>) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_tone_ex(
                frequency_hz,
                duration_ms,
                channel.map(i32::from).unwrap_or(-1),
            )
        }
    }
    pub fn try_tone_ex(
        &mut self,
        frequency_hz: f32,
        duration_ms: u32,
        channel: Option<u8>,
    ) -> Result<(), Error> {
        validate_audio_frequency_f32(frequency_hz)?;
        self.tone_ex(frequency_hz, duration_ms, channel)
            .then_some(())
            .ok_or(Error::Unavailable("speaker tone"))
    }
    pub fn play_u8(&mut self, samples: &[u8], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_u8(samples.as_ptr(), samples.len(), sample_rate_hz)
        }
    }
    pub fn try_play_u8(&mut self, samples: &[u8], sample_rate_hz: u32) -> Result<(), Error> {
        validate_audio_data(samples.len())?;
        validate_audio_sample_rate(sample_rate_hz)?;
        self.play_u8(samples, sample_rate_hz)
            .then_some(())
            .ok_or(Error::Unavailable("speaker playback"))
    }
    pub fn play_wav(&mut self, data: &[u8]) -> bool {
        unsafe { m5unified_sys::m5u_speaker_play_wav(data.as_ptr(), data.len()) }
    }
    pub fn try_play_wav(&mut self, data: &[u8]) -> Result<(), Error> {
        validate_audio_data(data.len())?;
        self.play_wav(data)
            .then_some(())
            .ok_or(Error::Unavailable("speaker playback"))
    }
    pub fn play_i16_with_options(
        &mut self,
        samples: &[i16],
        sample_rate_hz: u32,
        options: AudioPlaybackOptions,
    ) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_i16_ex(
                samples.as_ptr(),
                samples.len(),
                sample_rate_hz,
                options.stereo,
                options.repeat,
                options.channel.map(i32::from).unwrap_or(-1),
                options.stop_current_sound,
            )
        }
    }
    pub fn try_play_i16_with_options(
        &mut self,
        samples: &[i16],
        sample_rate_hz: u32,
        options: AudioPlaybackOptions,
    ) -> Result<(), Error> {
        validate_audio_data(samples.len())?;
        validate_audio_sample_rate(sample_rate_hz)?;
        validate_audio_playback_options(options)?;
        self.play_i16_with_options(samples, sample_rate_hz, options)
            .then_some(())
            .ok_or(Error::Unavailable("speaker playback"))
    }
    pub fn play_u8_with_options(
        &mut self,
        samples: &[u8],
        sample_rate_hz: u32,
        options: AudioPlaybackOptions,
    ) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_u8_ex(
                samples.as_ptr(),
                samples.len(),
                sample_rate_hz,
                options.stereo,
                options.repeat,
                options.channel.map(i32::from).unwrap_or(-1),
                options.stop_current_sound,
            )
        }
    }
    pub fn try_play_u8_with_options(
        &mut self,
        samples: &[u8],
        sample_rate_hz: u32,
        options: AudioPlaybackOptions,
    ) -> Result<(), Error> {
        validate_audio_data(samples.len())?;
        validate_audio_sample_rate(sample_rate_hz)?;
        validate_audio_playback_options(options)?;
        self.play_u8_with_options(samples, sample_rate_hz, options)
            .then_some(())
            .ok_or(Error::Unavailable("speaker playback"))
    }
    pub fn play_wav_with_options(&mut self, data: &[u8], options: AudioPlaybackOptions) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_wav_ex(
                data.as_ptr(),
                data.len(),
                options.repeat,
                options.channel.map(i32::from).unwrap_or(-1),
                options.stop_current_sound,
            )
        }
    }
    pub fn try_play_wav_with_options(
        &mut self,
        data: &[u8],
        options: AudioPlaybackOptions,
    ) -> Result<(), Error> {
        validate_audio_data(data.len())?;
        validate_audio_playback_options(options)?;
        self.play_wav_with_options(data, options)
            .then_some(())
            .ok_or(Error::Unavailable("speaker playback"))
    }
    pub fn is_playing(&self, channel: Option<u8>) -> bool {
        unsafe { m5unified_sys::m5u_speaker_is_playing(channel.map(i32::from).unwrap_or(-1)) }
    }
    pub fn playing_channels(&self) -> usize {
        unsafe { m5unified_sys::m5u_speaker_get_playing_channels() }
    }
    pub fn stop(&mut self, channel: Option<u8>) {
        unsafe { m5unified_sys::m5u_speaker_stop(channel.map(i32::from).unwrap_or(-1)) }
    }
    pub fn channel_volume(&self, channel: u8) -> u8 {
        unsafe { m5unified_sys::m5u_speaker_get_channel_volume(i32::from(channel)) }
    }
    pub fn set_channel_volume(&mut self, channel: u8, volume: u8) {
        unsafe { m5unified_sys::m5u_speaker_set_channel_volume(i32::from(channel), volume) }
    }
    pub fn set_all_channel_volume(&mut self, volume: u8) {
        unsafe { m5unified_sys::m5u_speaker_set_all_channel_volume(volume) }
    }
}

fn validate_audio_frequency_hz(frequency_hz: u32) -> Result<(), Error> {
    if frequency_hz == 0 {
        Err(Error::InvalidValue("audio frequency"))
    } else {
        Ok(())
    }
}

fn validate_audio_frequency_f32(frequency_hz: f32) -> Result<(), Error> {
    if !frequency_hz.is_finite() || frequency_hz <= 0.0 {
        Err(Error::InvalidValue("audio frequency"))
    } else {
        Ok(())
    }
}

fn validate_audio_sample_rate(sample_rate_hz: u32) -> Result<(), Error> {
    if sample_rate_hz == 0 {
        Err(Error::InvalidValue("audio sample rate"))
    } else {
        Ok(())
    }
}

fn validate_audio_data(len: usize) -> Result<(), Error> {
    if len == 0 {
        Err(Error::InvalidValue("audio data"))
    } else {
        Ok(())
    }
}

fn validate_audio_playback_options(options: AudioPlaybackOptions) -> Result<(), Error> {
    if options.repeat == 0 {
        Err(Error::InvalidValue("audio repeat"))
    } else {
        Ok(())
    }
}

fn validate_mic_config(config: MicConfig) -> Result<(), Error> {
    validate_audio_sample_rate(config.sample_rate)?;
    validate_audio_dma_buffers(config.dma_buf_len, config.dma_buf_count)
}

fn validate_speaker_config(config: SpeakerConfig) -> Result<(), Error> {
    validate_audio_sample_rate(config.sample_rate)?;
    validate_audio_dma_buffers(config.dma_buf_len, config.dma_buf_count)
}

fn validate_audio_dma_buffers(len: usize, count: usize) -> Result<(), Error> {
    if len == 0 {
        return Err(Error::InvalidValue("audio dma buffer length"));
    }
    if count == 0 {
        return Err(Error::InvalidValue("audio dma buffer count"));
    }
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImuKind {
    None,
    Unknown,
    Sh200q,
    Mpu6050,
    Mpu6886,
    Mpu9250,
    Bmi270,
    Raw(i32),
}

impl ImuKind {
    pub fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Unknown,
            2 => Self::Sh200q,
            3 => Self::Mpu6050,
            4 => Self::Mpu6886,
            5 => Self::Mpu9250,
            6 => Self::Bmi270,
            value => Self::Raw(value),
        }
    }

    pub fn raw(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Unknown => 1,
            Self::Sh200q => 2,
            Self::Mpu6050 => 3,
            Self::Mpu6886 => 4,
            Self::Mpu9250 => 5,
            Self::Bmi270 => 6,
            Self::Raw(value) => value,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ImuData {
    pub accel: Vec3,
    pub gyro: Vec3,
    pub mag: Option<Vec3>,
    pub temperature_c: Option<f32>,
}

impl ImuData {
    pub fn accel_magnitude(self) -> f32 {
        self.accel.magnitude()
    }

    pub fn gyro_magnitude(self) -> f32 {
        self.gyro.magnitude()
    }

    pub fn mag_magnitude(self) -> Option<f32> {
        self.mag.map(Vec3::magnitude)
    }

    pub const fn has_mag(self) -> bool {
        self.mag.is_some()
    }

    pub fn temperature_f(self) -> Option<f32> {
        self.temperature_c.map(|c| c.mul_add(9.0 / 5.0, 32.0))
    }

    pub fn temperature_k(self) -> Option<f32> {
        self.temperature_c.map(|c| c + 273.15)
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ImuCalibration {
    pub accel_strength: u8,
    pub gyro_strength: u8,
    pub mag_strength: u8,
}

impl ImuCalibration {
    pub const ZERO: Self = Self {
        accel_strength: 0,
        gyro_strength: 0,
        mag_strength: 0,
    };

    pub const fn new(accel_strength: u8, gyro_strength: u8, mag_strength: u8) -> Self {
        Self {
            accel_strength,
            gyro_strength,
            mag_strength,
        }
    }

    pub const fn with_accel_strength(mut self, accel_strength: u8) -> Self {
        self.accel_strength = accel_strength;
        self
    }

    pub const fn with_gyro_strength(mut self, gyro_strength: u8) -> Self {
        self.gyro_strength = gyro_strength;
        self
    }

    pub const fn with_mag_strength(mut self, mag_strength: u8) -> Self {
        self.mag_strength = mag_strength;
        self
    }

    pub const fn strengths(self) -> (u8, u8, u8) {
        (self.accel_strength, self.gyro_strength, self.mag_strength)
    }
}

pub const IMU_DATA_SLOTS: usize = 9;

impl Imu {
    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_imu_is_enabled() }
    }
    pub fn kind(&self) -> ImuKind {
        ImuKind::from_raw(unsafe { m5unified_sys::m5u_imu_get_type() as i32 })
    }
    pub fn update(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_update() }
    }
    pub fn try_update(&mut self) -> Result<(), Error> {
        self.update()
            .then_some(())
            .ok_or(Error::Unavailable("imu update"))
    }
    pub fn sleep(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_sleep() }
    }
    pub fn try_sleep(&mut self) -> Result<(), Error> {
        self.sleep()
            .then_some(())
            .ok_or(Error::Unavailable("imu sleep"))
    }
    pub fn data(&self) -> Option<ImuData> {
        Some(ImuData {
            accel: self.accel()?,
            gyro: self.gyro()?,
            mag: self.mag(),
            temperature_c: self.temperature_c(),
        })
    }
    pub fn try_data(&self) -> Result<ImuData, Error> {
        Ok(ImuData {
            accel: self.try_accel()?,
            gyro: self.try_gyro()?,
            mag: self.mag(),
            temperature_c: self.temperature_c(),
        })
    }
    pub fn load_offset_from_nvs(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_load_offset_from_nvs() }
    }
    pub fn try_load_offset_from_nvs(&mut self) -> Result<(), Error> {
        self.load_offset_from_nvs()
            .then_some(())
            .ok_or(Error::Unavailable("imu nvs offset load"))
    }
    pub fn save_offset_to_nvs(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_save_offset_to_nvs() }
    }
    pub fn try_save_offset_to_nvs(&mut self) -> Result<(), Error> {
        self.save_offset_to_nvs()
            .then_some(())
            .ok_or(Error::Unavailable("imu nvs offset save"))
    }
    pub fn clear_offset_data(&mut self) {
        unsafe { m5unified_sys::m5u_imu_clear_offset_data() }
    }
    pub fn offset_data(&self, index: usize) -> i32 {
        unsafe { m5unified_sys::m5u_imu_get_offset_data(index) }
    }
    pub fn try_offset_data(&self, index: usize) -> Result<i32, Error> {
        validate_imu_data_slot(index)?;
        Ok(self.offset_data(index))
    }
    pub fn offset_data_array(&self) -> [i32; IMU_DATA_SLOTS] {
        core::array::from_fn(|index| self.offset_data(index))
    }
    pub fn set_offset_data(&mut self, index: usize, value: i32) {
        unsafe { m5unified_sys::m5u_imu_set_offset_data(index, value) }
    }
    pub fn try_set_offset_data(&mut self, index: usize, value: i32) -> Result<(), Error> {
        validate_imu_data_slot(index)?;
        self.set_offset_data(index, value);
        Ok(())
    }
    pub fn set_offset_data_array(&mut self, values: [i32; IMU_DATA_SLOTS]) {
        for (index, value) in values.into_iter().enumerate() {
            self.set_offset_data(index, value);
        }
    }
    pub fn raw_data(&self, index: usize) -> i16 {
        unsafe { m5unified_sys::m5u_imu_get_raw_data(index) }
    }
    pub fn try_raw_data(&self, index: usize) -> Result<i16, Error> {
        validate_imu_data_slot(index)?;
        Ok(self.raw_data(index))
    }
    pub fn raw_data_array(&self) -> [i16; IMU_DATA_SLOTS] {
        core::array::from_fn(|index| self.raw_data(index))
    }
    pub fn set_int_pin_active_logic(&mut self, high_active: bool) -> bool {
        unsafe { m5unified_sys::m5u_imu_set_int_pin_active_logic(high_active) }
    }
    pub fn try_set_int_pin_active_logic(&mut self, high_active: bool) -> Result<(), Error> {
        self.set_int_pin_active_logic(high_active)
            .then_some(())
            .ok_or(Error::Unavailable("imu interrupt pin"))
    }
    pub fn set_calibration(&mut self, calibration: ImuCalibration) {
        unsafe {
            m5unified_sys::m5u_imu_set_calibration(
                calibration.accel_strength,
                calibration.gyro_strength,
                calibration.mag_strength,
            )
        }
    }
    pub fn try_set_calibration(&mut self, calibration: ImuCalibration) -> Result<(), Error> {
        self.set_calibration(calibration);
        Ok(())
    }
}

fn validate_imu_data_slot(index: usize) -> Result<(), Error> {
    if index < IMU_DATA_SLOTS {
        Ok(())
    } else {
        Err(Error::InvalidValue("imu data slot"))
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct TouchDetail {
    pub x: i32,
    pub y: i32,
    pub size: u16,
    pub id: u8,
    pub prev_x: i32,
    pub prev_y: i32,
    pub base_x: i32,
    pub base_y: i32,
    pub base_msec: u32,
    pub state: TouchState,
    pub is_pressed: bool,
    pub was_pressed: bool,
    pub is_released: bool,
    pub was_released: bool,
    pub was_clicked: bool,
    pub was_hold: bool,
    pub is_holding: bool,
    pub was_flick_start: bool,
    pub is_flicking: bool,
    pub was_flicked: bool,
    pub was_drag_start: bool,
    pub is_dragging: bool,
    pub was_dragged: bool,
    pub click_count: i32,
}

impl TouchDetail {
    pub const fn point(&self) -> TouchPoint {
        TouchPoint {
            x: self.x,
            y: self.y,
            size: self.size,
            id: self.id,
        }
    }

    pub const fn position(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub const fn previous_position(&self) -> Point {
        Point::new(self.prev_x, self.prev_y)
    }

    pub const fn base_position(&self) -> Point {
        Point::new(self.base_x, self.base_y)
    }

    pub const fn delta(&self) -> (i32, i32) {
        (self.x - self.prev_x, self.y - self.prev_y)
    }

    pub const fn delta_point(&self) -> Point {
        Point::new(self.x - self.prev_x, self.y - self.prev_y)
    }

    pub const fn distance(&self) -> (i32, i32) {
        (self.x - self.base_x, self.y - self.base_y)
    }

    pub const fn distance_point(&self) -> Point {
        Point::new(self.x - self.base_x, self.y - self.base_y)
    }
}

impl Touch {
    pub fn detail(&self, index: usize) -> Option<TouchDetail> {
        let index = touch_index_to_c_int(index).ok()?;
        let mut raw = m5unified_sys::m5u_touch_detail_t::default();
        let ok = unsafe { m5unified_sys::m5u_touch_get_detail(index, &mut raw) };
        ok.then_some(TouchDetail {
            x: raw.x,
            y: raw.y,
            size: raw.size,
            id: raw.id,
            prev_x: raw.prev_x,
            prev_y: raw.prev_y,
            base_x: raw.base_x,
            base_y: raw.base_y,
            base_msec: raw.base_msec,
            state: TouchState::from_raw(raw.state),
            is_pressed: raw.is_pressed,
            was_pressed: raw.was_pressed,
            is_released: raw.is_released,
            was_released: raw.was_released,
            was_clicked: raw.was_clicked,
            was_hold: raw.was_hold,
            is_holding: raw.is_holding,
            was_flick_start: raw.was_flick_start,
            is_flicking: raw.is_flicking,
            was_flicked: raw.was_flicked,
            was_drag_start: raw.was_drag_start,
            is_dragging: raw.is_dragging,
            was_dragged: raw.was_dragged,
            click_count: raw.click_count,
        })
    }

    pub fn try_detail(&self, index: usize) -> Result<TouchDetail, Error> {
        touch_index_to_c_int(index)?;
        self.detail(index).ok_or(Error::Unavailable("touch detail"))
    }

    pub fn details(&self) -> Vec<TouchDetail> {
        (0..self.count())
            .filter_map(|index| self.detail(index))
            .collect()
    }

    pub fn primary_detail(&self) -> Option<TouchDetail> {
        self.detail(0)
    }

    pub fn try_primary_detail(&self) -> Result<TouchDetail, Error> {
        self.primary_detail()
            .ok_or(Error::Unavailable("touch detail"))
    }
}

impl Rtc {
    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_rtc_is_enabled() }
    }
}

#[derive(Debug)]
pub struct Axp2101;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Axp2101IrqMask {
    pub raw: u64,
}

impl Axp2101IrqMask {
    pub const NONE: Self = Self { raw: 0 };
    pub const ALL: Self = Self { raw: u64::MAX };

    pub const fn from_raw(raw: u64) -> Self {
        Self { raw }
    }

    pub const fn raw(self) -> u64 {
        self.raw
    }

    pub const fn is_empty(self) -> bool {
        self.raw == 0
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.raw & other.raw) == other.raw
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            raw: self.raw | other.raw,
        }
    }

    pub const fn without(self, other: Self) -> Self {
        Self {
            raw: self.raw & !other.raw,
        }
    }
}

impl From<u64> for Axp2101IrqMask {
    fn from(raw: u64) -> Self {
        Self::from_raw(raw)
    }
}

impl From<Axp2101IrqMask> for u64 {
    fn from(mask: Axp2101IrqMask) -> Self {
        mask.raw()
    }
}

impl core::ops::BitOr for Axp2101IrqMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for Axp2101IrqMask {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl core::ops::BitAnd for Axp2101IrqMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            raw: self.raw & rhs.raw,
        }
    }
}

impl core::ops::BitAndAssign for Axp2101IrqMask {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl core::ops::Sub for Axp2101IrqMask {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.without(rhs)
    }
}

impl core::ops::SubAssign for Axp2101IrqMask {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.without(rhs);
    }
}

impl core::ops::Not for Axp2101IrqMask {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { raw: !self.raw }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Axp2101IrqStatus {
    pub raw: u64,
}

impl Axp2101IrqStatus {
    pub const fn from_raw(raw: u64) -> Self {
        Self { raw }
    }

    pub const fn raw(self) -> u64 {
        self.raw
    }

    pub const fn is_empty(self) -> bool {
        self.raw == 0
    }

    pub const fn any(self) -> bool {
        self.raw != 0
    }

    pub const fn contains(self, mask: Axp2101IrqMask) -> bool {
        (self.raw & mask.raw()) != 0
    }

    pub const fn as_mask(self) -> Axp2101IrqMask {
        Axp2101IrqMask::from_raw(self.raw)
    }

    pub fn battery_charger_under_temperature(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_is_bat_charger_under_temperature_irq() }
    }
    pub fn battery_charger_over_temperature(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_is_bat_charger_over_temperature_irq() }
    }
    pub fn vbus_insert(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_is_vbus_insert_irq() }
    }
    pub fn vbus_remove(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_is_vbus_remove_irq() }
    }
}

impl Power {
    pub fn axp2101(&self) -> Axp2101 {
        Axp2101
    }
}

impl Axp2101 {
    pub const IRQ_ALL: Axp2101IrqMask = Axp2101IrqMask::ALL;

    pub fn disable_irq(&self, mask: impl Into<Axp2101IrqMask>) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_disable_irq(mask.into().raw()) }
    }

    pub fn try_disable_irq(&self, mask: impl Into<Axp2101IrqMask>) -> Result<(), Error> {
        self.disable_irq(mask)
            .then_some(())
            .ok_or(Error::Unavailable("axp2101"))
    }

    pub fn enable_irq(&self, mask: impl Into<Axp2101IrqMask>) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_enable_irq(mask.into().raw()) }
    }

    pub fn try_enable_irq(&self, mask: impl Into<Axp2101IrqMask>) -> Result<(), Error> {
        self.enable_irq(mask)
            .then_some(())
            .ok_or(Error::Unavailable("axp2101"))
    }

    pub fn clear_irq_statuses(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_clear_irq_statuses() }
    }

    pub fn try_clear_irq_statuses(&self) -> Result<(), Error> {
        self.clear_irq_statuses()
            .then_some(())
            .ok_or(Error::Unavailable("axp2101"))
    }

    pub fn irq_statuses(&self) -> Axp2101IrqStatus {
        Axp2101IrqStatus::from_raw(unsafe { m5unified_sys::m5u_power_axp2101_get_irq_statuses() })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogLevel {
    None = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Verbose = 5,
}

impl LogLevel {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Error,
            2 => Self::Warn,
            3 => Self::Info,
            4 => Self::Debug,
            5 => Self::Verbose,
            _ => Self::Info,
        }
    }

    pub const fn raw(self) -> i32 {
        self as i32
    }

    pub const fn severity(self) -> u8 {
        self as u8
    }

    pub const fn is_enabled(self) -> bool {
        !matches!(self, Self::None)
    }

    pub const fn includes(self, message_level: Self) -> bool {
        message_level.severity() <= self.severity()
    }

    pub const fn is_at_least(self, minimum: Self) -> bool {
        self.severity() >= minimum.severity()
    }

    pub const fn is_verbose(self) -> bool {
        matches!(self, Self::Verbose)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogTarget {
    Serial,
    Display,
    Callback,
}

impl LogTarget {
    pub const ALL: [Self; 3] = [Self::Serial, Self::Display, Self::Callback];

    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Serial),
            1 => Some(Self::Display),
            2 => Some(Self::Callback),
            _ => None,
        }
    }

    pub const fn raw(self) -> c_int {
        match self {
            Self::Serial => 0,
            Self::Display => 1,
            Self::Callback => 2,
        }
    }

    pub const fn is_serial(self) -> bool {
        matches!(self, Self::Serial)
    }

    pub const fn is_display(self) -> bool {
        matches!(self, Self::Display)
    }

    pub const fn is_callback(self) -> bool {
        matches!(self, Self::Callback)
    }

    pub const fn supports_color(self) -> bool {
        matches!(self, Self::Serial | Self::Display)
    }
}

impl Log {
    pub fn print(&self, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_print(text.as_ptr()) }
        Ok(())
    }
    pub fn log(&self, level: LogLevel, text: &str) -> Result<(), Error> {
        let text = CString::new(text).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_level(level as c_int, text.as_ptr()) }
        Ok(())
    }

    pub fn error(&self, text: &str) -> Result<(), Error> {
        self.log(LogLevel::Error, text)
    }

    pub fn warn(&self, text: &str) -> Result<(), Error> {
        self.log(LogLevel::Warn, text)
    }

    pub fn info(&self, text: &str) -> Result<(), Error> {
        self.log(LogLevel::Info, text)
    }

    pub fn debug(&self, text: &str) -> Result<(), Error> {
        self.log(LogLevel::Debug, text)
    }

    pub fn verbose(&self, text: &str) -> Result<(), Error> {
        self.log(LogLevel::Verbose, text)
    }

    pub fn set_level(&mut self, target: LogTarget, level: LogLevel) {
        unsafe { m5unified_sys::m5u_log_set_level(target.raw(), level as c_int) }
    }

    pub fn level(&self, target: LogTarget) -> LogLevel {
        let raw = unsafe { m5unified_sys::m5u_log_get_level(target.raw()) };
        LogLevel::from_raw(raw as i32)
    }

    pub fn set_enable_color(&mut self, target: LogTarget, enable: bool) {
        unsafe { m5unified_sys::m5u_log_set_enable_color(target.raw(), enable) }
    }

    pub fn enable_color(&self, target: LogTarget) -> bool {
        unsafe { m5unified_sys::m5u_log_get_enable_color(target.raw()) }
    }

    pub fn set_suffix(&mut self, target: LogTarget, suffix: &str) -> Result<(), Error> {
        let suffix = CString::new(suffix).map_err(|_| Error::InvalidString)?;
        unsafe { m5unified_sys::m5u_log_set_suffix(target.raw(), suffix.as_ptr()) }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_dimensions_are_available_on_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(m5.display.width() > 0);
        assert!(m5.display.height() > 0);
        let origin = Point::new(4, 8);
        let size = Size::new(120, 80);
        let rect = Rect::from_origin_size(origin, size);
        assert_eq!(Point::ZERO, Point { x: 0, y: 0 });
        assert_eq!(origin.components(), (4, 8));
        assert_eq!(origin.x(), 4);
        assert_eq!(origin.y(), 8);
        assert_eq!(origin.offset(3, -2), Point::new(7, 6));
        assert_eq!(origin.delta_to(Point::new(7, 6)), (3, -2));
        assert_eq!(origin.delta_point_to(Point::new(7, 6)), Point::new(3, -2));
        assert_eq!(Size::ZERO, Size { w: 0, h: 0 });
        assert_eq!(size.dimensions(), (120, 80));
        assert_eq!(size.width(), 120);
        assert_eq!(size.height(), 80);
        assert!(!size.is_empty());
        assert_eq!(size.area(), Some(9_600));
        assert_eq!(Size::new(0, 80).area(), None);
        assert_eq!(rect, Rect::new(4, 8, 120, 80));
        assert_eq!(rect.origin(), origin);
        assert_eq!(rect.size(), size);
        assert_eq!(rect.right(), 124);
        assert_eq!(rect.bottom(), 88);
        assert_eq!(rect.area(), Some(9_600));
        assert!(rect.contains(Point::new(4, 8)));
        assert!(rect.contains(Point::new(123, 87)));
        assert!(!rect.contains(Point::new(124, 87)));
        assert_eq!(rect.translate(-4, 2), Rect::new(0, 10, 120, 80));
        let mut canvas = m5.canvas().expect("host stub canvas should be available");
        assert!(!canvas.create_sprite(Size { w: 120, h: 80 }));
        assert_eq!(
            canvas.try_create_sprite(Size { w: 0, h: 80 }),
            Err(Error::InvalidValue("canvas sprite size"))
        );
        assert_eq!(
            canvas.try_create_sprite(Size { w: 120, h: 80 }),
            Err(Error::Unavailable("canvas sprite"))
        );
        assert_eq!(canvas.width(), 0);
        assert_eq!(canvas.height(), 0);
        canvas.set_text_size(0.5);
        assert_eq!(canvas.try_set_text_size(0.5), Ok(()));
        assert_eq!(
            canvas.try_set_text_size(0.0),
            Err(Error::InvalidValue("canvas text size"))
        );
        assert_eq!(
            canvas.try_set_text_size(f32::NAN),
            Err(Error::InvalidValue("canvas text size"))
        );
        canvas.set_text_color(colors::GREEN, colors::BLACK);
        canvas.set_text_scroll(true);
        canvas.set_text_datum(TextDatum::MiddleCenter);
        assert_eq!(canvas.text_datum(), Some(TextDatum::TopLeft));
        canvas.set_text_padding(12);
        assert_eq!(canvas.text_padding(), 0);
        assert_eq!(canvas.text_size_x(), 1);
        assert_eq!(canvas.text_size_y(), 1);
        assert_eq!(canvas.base_color(), colors::BLACK);
        canvas.set_base_color(colors::BLUE);
        canvas.set_color(colors::WHITE);
        canvas.set_rgb_color(1, 2, 3);
        canvas.set_raw_color(0x1234);
        assert_eq!(canvas.raw_color(), 0);
        canvas.set_swap_bytes(true);
        assert!(!canvas.swap_bytes());
        assert!(canvas.set_font(DisplayFont::Font0));
        assert_eq!(canvas.try_set_font(DisplayFont::Font0), Ok(()));
        assert_eq!(canvas.font_width(), 8);
        assert_eq!(canvas.font_height(), 16);
        assert!(canvas.show_font(0));
        assert_eq!(canvas.try_show_font(0), Ok(()));
        canvas.unload_font();
        canvas.set_cursor(4, 5);
        canvas.clear();
        canvas.fill_screen(colors::BLACK);
        canvas.draw_line(Point { x: 0, y: 0 }, Point { x: 10, y: 10 }, colors::WHITE);
        canvas.draw_rect(
            Rect {
                x: 1,
                y: 2,
                w: 3,
                h: 4,
            },
            colors::GREEN,
        );
        assert_eq!(
            canvas.try_draw_rect(
                Rect {
                    x: 1,
                    y: 2,
                    w: 3,
                    h: 4,
                },
                colors::GREEN
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_draw_rect(
                Rect {
                    x: 1,
                    y: 2,
                    w: 3,
                    h: 0,
                },
                colors::GREEN
            ),
            Err(Error::InvalidValue("display rect"))
        );
        canvas.fill_rect(
            Rect {
                x: 2,
                y: 3,
                w: 4,
                h: 5,
            },
            colors::BLUE,
        );
        assert_eq!(
            canvas.try_fill_rect(
                Rect {
                    x: 2,
                    y: 3,
                    w: 4,
                    h: 5,
                },
                colors::BLUE
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_fill_rect(
                Rect {
                    x: 2,
                    y: 3,
                    w: -4,
                    h: 5,
                },
                colors::BLUE
            ),
            Err(Error::InvalidValue("display rect"))
        );
        canvas.draw_circle(Point { x: 4, y: 4 }, 3, colors::WHITE);
        assert_eq!(
            canvas.try_draw_circle(Point { x: 4, y: 4 }, 3, colors::WHITE),
            Ok(())
        );
        assert_eq!(
            canvas.try_draw_circle(Point { x: 4, y: 4 }, 0, colors::WHITE),
            Err(Error::InvalidValue("display radius"))
        );
        canvas.fill_circle(Point { x: 8, y: 8 }, 2, colors::RED);
        assert_eq!(
            canvas.try_fill_circle(Point { x: 8, y: 8 }, 2, colors::RED),
            Ok(())
        );
        assert_eq!(
            canvas.try_fill_circle(Point { x: 8, y: 8 }, -1, colors::RED),
            Err(Error::InvalidValue("display radius"))
        );
        canvas.draw_pixel(Point { x: 1, y: 1 }, colors::WHITE);
        assert_eq!(canvas.read_pixel(Point { x: 1, y: 1 }), colors::BLACK);
        canvas.draw_fast_hline(0, 2, 10, colors::GREEN);
        assert_eq!(canvas.try_draw_fast_hline(0, 2, 10, colors::GREEN), Ok(()));
        assert_eq!(
            canvas.try_draw_fast_hline(0, 2, 0, colors::GREEN),
            Err(Error::InvalidValue("display length"))
        );
        canvas.draw_fast_vline(2, 0, 10, colors::BLUE);
        assert_eq!(canvas.try_draw_fast_vline(2, 0, 10, colors::BLUE), Ok(()));
        assert_eq!(
            canvas.try_draw_fast_vline(2, 0, -1, colors::BLUE),
            Err(Error::InvalidValue("display length"))
        );
        canvas.draw_round_rect(
            Rect {
                x: 0,
                y: 0,
                w: 10,
                h: 8,
            },
            2,
            colors::YELLOW,
        );
        assert_eq!(
            canvas.try_draw_round_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 10,
                    h: 8,
                },
                2,
                colors::YELLOW
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_draw_round_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 8,
                },
                2,
                colors::YELLOW
            ),
            Err(Error::InvalidValue("display rect"))
        );
        canvas.fill_round_rect(
            Rect {
                x: 1,
                y: 1,
                w: 8,
                h: 6,
            },
            2,
            colors::CYAN,
        );
        assert_eq!(
            canvas.try_fill_round_rect(
                Rect {
                    x: 1,
                    y: 1,
                    w: 8,
                    h: 6,
                },
                2,
                colors::CYAN
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_fill_round_rect(
                Rect {
                    x: 1,
                    y: 1,
                    w: 8,
                    h: 6,
                },
                0,
                colors::CYAN
            ),
            Err(Error::InvalidValue("display radius"))
        );
        canvas.draw_ellipse(Point { x: 12, y: 12 }, 6, 3, colors::MAGENTA);
        assert_eq!(
            canvas.try_draw_ellipse(Point { x: 12, y: 12 }, 6, 3, colors::MAGENTA),
            Ok(())
        );
        assert_eq!(
            canvas.try_draw_ellipse(Point { x: 12, y: 12 }, 0, 3, colors::MAGENTA),
            Err(Error::InvalidValue("display radii"))
        );
        canvas.fill_ellipse(Point { x: 14, y: 14 }, 4, 2, colors::ORANGE);
        assert_eq!(
            canvas.try_fill_ellipse(Point { x: 14, y: 14 }, 4, 2, colors::ORANGE),
            Ok(())
        );
        assert_eq!(
            canvas.try_fill_ellipse(Point { x: 14, y: 14 }, 4, -1, colors::ORANGE),
            Err(Error::InvalidValue("display radii"))
        );
        canvas.draw_arc(Point { x: 20, y: 20 }, 3, 7, 0.0, 90.0, colors::WHITE);
        assert_eq!(
            canvas.try_draw_arc(Point { x: 20, y: 20 }, 3, 7, 0.0, 90.0, colors::WHITE),
            Ok(())
        );
        assert_eq!(
            canvas.try_draw_arc(Point { x: 20, y: 20 }, 8, 7, 0.0, 90.0, colors::WHITE),
            Err(Error::InvalidValue("display arc radii"))
        );
        canvas.fill_arc(Point { x: 24, y: 24 }, 2, 6, 90.0, 180.0, colors::BLUE);
        assert_eq!(
            canvas.try_fill_arc(Point { x: 24, y: 24 }, 2, 6, 90.0, 180.0, colors::BLUE),
            Ok(())
        );
        assert_eq!(
            canvas.try_fill_arc(Point { x: 24, y: 24 }, 2, 6, f32::NAN, 180.0, colors::BLUE),
            Err(Error::InvalidValue("display arc angle"))
        );
        canvas.draw_triangle(
            Point { x: 0, y: 0 },
            Point { x: 5, y: 0 },
            Point { x: 2, y: 5 },
            colors::GREEN,
        );
        canvas.fill_triangle(
            Point { x: 6, y: 0 },
            Point { x: 11, y: 0 },
            Point { x: 8, y: 5 },
            colors::RED,
        );
        canvas.progress_bar(
            Rect {
                x: 0,
                y: 20,
                w: 30,
                h: 4,
            },
            64,
        );
        assert_eq!(
            canvas.try_progress_bar(
                Rect {
                    x: 0,
                    y: 20,
                    w: 30,
                    h: 4,
                },
                64
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_progress_bar(
                Rect {
                    x: 0,
                    y: 20,
                    w: 30,
                    h: 0,
                },
                64
            ),
            Err(Error::InvalidValue("display rect"))
        );
        canvas.write_pixel(Point { x: 0, y: 0 }, colors::WHITE);
        canvas.write_fast_vline(0, 0, 10, colors::WHITE);
        assert_eq!(canvas.try_write_fast_vline(0, 0, 10, colors::WHITE), Ok(()));
        assert_eq!(
            canvas.try_write_fast_vline(0, 0, 0, colors::WHITE),
            Err(Error::InvalidValue("display length"))
        );
        canvas.set_addr_window(Rect {
            x: 0,
            y: 0,
            w: 20,
            h: 10,
        });
        assert_eq!(
            canvas.try_set_addr_window(Rect {
                x: 0,
                y: 0,
                w: 20,
                h: 10,
            }),
            Ok(())
        );
        assert_eq!(
            canvas.try_set_addr_window(Rect {
                x: 0,
                y: 0,
                w: -1,
                h: 10,
            }),
            Err(Error::InvalidValue("display rect"))
        );
        canvas.set_window(0, 0, 19, 9);
        assert_eq!(canvas.try_set_window(0, 0, 19, 9), Ok(()));
        assert_eq!(
            canvas.try_set_window(20, 0, 19, 9),
            Err(Error::InvalidValue("display window"))
        );
        canvas.set_clip_rect(Rect {
            x: 1,
            y: 2,
            w: 20,
            h: 10,
        });
        assert_eq!(
            canvas.try_set_clip_rect(Rect {
                x: 1,
                y: 2,
                w: 20,
                h: 10,
            }),
            Ok(())
        );
        assert_eq!(
            canvas.try_set_clip_rect(Rect {
                x: 1,
                y: 2,
                w: 20,
                h: 0,
            }),
            Err(Error::InvalidValue("display rect"))
        );
        assert_eq!(canvas.clip_rect(), Rect::default());
        canvas.clear_clip_rect();
        canvas.scroll(0, 2);
        canvas.set_scroll_rect(
            Rect {
                x: 0,
                y: 0,
                w: 24,
                h: 12,
            },
            colors::BLACK,
        );
        assert_eq!(
            canvas.try_set_scroll_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 24,
                    h: 12,
                },
                colors::BLACK
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_set_scroll_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: -1,
                    h: 12,
                },
                colors::BLACK
            ),
            Err(Error::InvalidValue("display rect"))
        );
        assert_eq!(canvas.scroll_rect(), Rect::default());
        canvas.clear_scroll_rect();
        assert_eq!(canvas.text_length("abc"), Ok(24));
        assert_eq!(canvas.draw_char('A', 0, 0), 8);
        assert_eq!(canvas.draw_number(42, 0, 0), 0);
        assert_eq!(canvas.draw_float(1.25, 2, 0, 0), 0);
        let image_options = ImageDrawOptions::at(Point::new(0, 0)).with_scale(1.0, 1.0);
        assert!(!canvas.draw_bmp(&[], image_options));
        assert_eq!(
            canvas.try_draw_bmp(&[], image_options),
            Err(Error::InvalidValue("image data"))
        );
        assert!(!canvas.draw_jpg(&[], ImageDrawOptions::default()));
        assert_eq!(
            canvas.try_draw_jpg(&[], ImageDrawOptions::default()),
            Err(Error::InvalidValue("image data"))
        );
        assert!(!canvas.draw_png(&[], ImageDrawOptions::default()));
        assert_eq!(
            canvas.try_draw_png(&[], ImageDrawOptions::default()),
            Err(Error::InvalidValue("image data"))
        );
        assert!(!canvas.push_image_rgb565(
            Rect {
                x: 0,
                y: 0,
                w: 2,
                h: 2,
            },
            &[colors::BLACK, colors::WHITE, colors::RED, colors::GREEN],
        ));
        assert_eq!(
            canvas.try_push_image_rgb565(
                Rect {
                    x: 0,
                    y: 0,
                    w: 2,
                    h: 2,
                },
                &[colors::BLACK, colors::WHITE, colors::RED],
            ),
            Err(Error::InvalidValue("rgb565 image data"))
        );
        assert_eq!(
            canvas.try_push_image_rgb565(
                Rect {
                    x: 0,
                    y: 0,
                    w: 2,
                    h: 2,
                },
                &[colors::BLACK, colors::WHITE, colors::RED, colors::GREEN],
            ),
            Err(Error::Unavailable("canvas rgb565 image"))
        );
        assert_eq!(canvas.text_width("abc"), Ok(24));
        assert_eq!(canvas.print("canvas"), Ok(()));
        assert_eq!(canvas.println("sprite"), Ok(()));
        assert_eq!(canvas.draw_center_string("abc", 20, 10), Ok(24));
        assert_eq!(canvas.draw_string("abc", 0, 0), Ok(3));
        assert_eq!(
            canvas.draw_center_string("bad\0canvas", 0, 0),
            Err(Error::InvalidString)
        );
        assert_eq!(canvas.print("bad\0canvas"), Err(Error::InvalidString));
        canvas.push_sprite(Point { x: 0, y: 0 });
        canvas.delete_sprite();
        let red = Color565::from_rgb888(255, 0, 0);
        assert_eq!(red.raw(), colors::RED);
        assert_eq!(u16::from(red), colors::RED);
        assert_eq!(Color565::from(colors::GREEN), Color565::new(colors::GREEN));
        assert_eq!(Color565::new(colors::BLUE).rgb565_components(), (0, 0, 31));
        assert_eq!(Color565::new(colors::WHITE).to_rgb888(), (255, 255, 255));
        assert_eq!(Color565::rgb888(255, 0, 0).raw(), colors::RED);
        assert_eq!(m5.display.font_width(), 8);
        assert_eq!(m5.display.font_height(), 16);
        assert_eq!(DisplayFont::ALL.len(), 70);
        assert_eq!(DisplayFont::from_raw(0), Some(DisplayFont::Font0));
        assert_eq!(DisplayFont::from_raw(69), Some(DisplayFont::DejaVu72));
        assert_eq!(DisplayFont::from_raw(70), None);
        assert_eq!(DisplayFont::from_raw(-1), None);
        assert_eq!(DisplayFont::FreeMonoBoldOblique24pt7b.raw(), 25);
        assert!(m5.display.set_font(DisplayFont::Font0));
        assert_eq!(m5.display.try_set_font(DisplayFont::Font0), Ok(()));
        assert!(m5.display.set_font(DisplayFont::FreeMonoBoldOblique24pt7b));
        assert!(m5.display.show_font(0));
        assert_eq!(m5.display.try_show_font(0), Ok(()));
        assert_eq!(m5.display.font_width_for(DisplayFont::Font0), 8);
        assert_eq!(m5.display.font_height_for(DisplayFont::Font0), 16);
        m5.display.unload_font();
        assert_eq!(ImageDrawOptions::default().datum, TextDatum::TopLeft);
        let image_options = ImageDrawOptions::at(Point::new(1, 2))
            .with_max_size(Size::new(10, 10))
            .with_offset(Point::new(3, 4))
            .with_scale(1.0, 1.0)
            .with_datum(TextDatum::MiddleCenter);
        assert_eq!(image_options.origin(), Point::new(1, 2));
        assert_eq!(image_options.max_size(), Some(Size::new(10, 10)));
        assert_eq!(image_options.offset(), Point::new(3, 4));
        assert_eq!(
            ImageDrawOptions::new(0, 0).with_origin(Point::new(1, 2)),
            ImageDrawOptions::at(Point::new(1, 2))
        );
        assert_eq!(ImageDrawOptions::default().max_size(), None);
        assert!(!m5.display.draw_bmp(&[], ImageDrawOptions::default()));
        assert_eq!(
            m5.display.try_draw_bmp(&[], ImageDrawOptions::default()),
            Err(Error::InvalidValue("image data"))
        );
        assert!(!m5
            .display
            .draw_jpg(&[0xff, 0xd8], ImageDrawOptions::default()));
        assert_eq!(
            m5.display
                .try_draw_jpg(&[0xff, 0xd8], ImageDrawOptions::default()),
            Err(Error::Unavailable("display jpg"))
        );
        assert!(!m5
            .display
            .draw_png(&[0x89, b'P', b'N', b'G'], image_options));
        assert!(!m5.display.push_image_rgb565(
            Rect {
                x: 0,
                y: 0,
                w: 2,
                h: 2,
            },
            &[colors::RED; 3]
        ));
        assert_eq!(
            m5.display.try_push_image_rgb565(
                Rect {
                    x: 0,
                    y: 0,
                    w: 2,
                    h: 2,
                },
                &[colors::RED; 3]
            ),
            Err(Error::InvalidValue("rgb565 image data"))
        );
        assert!(!m5.display.push_image_rgb565(
            Rect {
                x: 0,
                y: 0,
                w: 2,
                h: 2,
            },
            &[colors::RED; 4]
        ));
        assert_eq!(
            m5.display.try_push_image_rgb565(
                Rect {
                    x: 0,
                    y: 0,
                    w: 2,
                    h: 2,
                },
                &[colors::RED; 4]
            ),
            Err(Error::Unavailable("display rgb565 image"))
        );
        assert_eq!(m5.display.brightness(), 0);
        m5.display.set_brightness(128);
        m5.display.set_color_depth(16);
        assert_eq!(m5.display.color_depth(), 16);
        assert!(!m5.display.is_epd());
        assert_eq!(m5.display.epd_mode(), EpdMode::Unknown(0));
        assert_eq!(EpdMode::from_raw(1), EpdMode::Quality);
        assert_eq!(EpdMode::from_raw(4), EpdMode::Fastest);
        assert_eq!(EpdMode::from_raw(99), EpdMode::Unknown(99));
        assert_eq!(EpdMode::Fast.raw(), 3);
        assert_eq!(EpdMode::Unknown(99).raw(), 99);
        assert!(EpdMode::Quality.is_known());
        assert!(EpdMode::Quality.is_quality());
        assert!(EpdMode::Text.is_text());
        assert!(EpdMode::Fast.is_fast());
        assert!(EpdMode::Fastest.is_fast());
        assert!(!EpdMode::Unknown(99).is_known());
        m5.display.set_epd_mode(EpdMode::Quality);
        m5.display.set_epd_mode(EpdMode::Text);
        m5.display.set_epd_mode(EpdMode::Fast);
        m5.display.set_epd_fastest();
        assert!(!m5.display.set_resolution(DisplayResolution {
            logical_width: 320,
            logical_height: 240,
            refresh_rate: 60.0,
            ..DisplayResolution::default()
        }));
        assert_eq!(
            m5.display.try_set_resolution(DisplayResolution {
                logical_width: 320,
                logical_height: 240,
                refresh_rate: 60.0,
                ..DisplayResolution::default()
            }),
            Err(Error::Unavailable("display resolution"))
        );
        assert_eq!(
            m5.display.try_set_resolution(DisplayResolution {
                refresh_rate: f32::NAN,
                ..DisplayResolution::default()
            }),
            Err(Error::InvalidValue("display refresh rate"))
        );
        assert_eq!(
            m5.display.try_set_resolution(DisplayResolution {
                refresh_rate: -1.0,
                ..DisplayResolution::default()
            }),
            Err(Error::InvalidValue("display refresh rate"))
        );
        assert_eq!(
            m5.display.try_set_resolution(DisplayResolution {
                pixel_clock: 0,
                ..DisplayResolution::default()
            }),
            Err(Error::InvalidValue("display pixel clock"))
        );
        assert_eq!(m5.display.cursor_x(), 0);
        assert_eq!(m5.display.cursor_y(), 0);
        m5.display.set_pivot(12.0, 34.0);
        assert_eq!(m5.display.try_set_pivot(12.0, 34.0), Ok(()));
        assert_eq!(
            m5.display.try_set_pivot(f32::NAN, 34.0),
            Err(Error::InvalidValue("display pivot"))
        );
        assert_eq!(
            m5.display.try_set_pivot(12.0, f32::INFINITY),
            Err(Error::InvalidValue("display pivot"))
        );
        assert_eq!(m5.display.pivot(), (0.0, 0.0));
        m5.display.set_base_color(colors::BLACK);
        m5.display.set_color(colors::WHITE);
        m5.display.set_rgb_color(255, 0, 0);
        m5.display.set_raw_color(0x00FF00);
        assert_eq!(m5.display.raw_color(), 0);
        assert_eq!(m5.display.palette_count(), 0);
        m5.display.set_swap_bytes(true);
        assert!(!m5.display.swap_bytes());
        assert_eq!(m5.display.swap565(255, 0, 0), 0x00F8);
        assert_eq!(m5.display.swap888(0x12, 0x34, 0x56), 0x563412);
        m5.display.invert_display(false);
        m5.display.power_save(true);
        m5.display.power_save_on();
        m5.display.power_save_off();
        m5.display.sleep();
        m5.display.wakeup();
        assert_eq!(m5.display.text_datum(), Some(TextDatum::TopLeft));
        assert_eq!(TextDatum::from_raw(5), Some(TextDatum::MiddleCenter));
        assert_eq!(TextDatum::from_raw(99), None);
        assert_eq!(TextDatum::BottomRight.raw(), 10);
        assert_eq!(TextDatum::MiddleCenter.horizontal_index(), 1);
        assert_eq!(TextDatum::MiddleCenter.vertical_index(), 1);
        assert!(TextDatum::TopLeft.is_left());
        assert!(TextDatum::TopCenter.is_horizontal_center());
        assert!(TextDatum::TopRight.is_right());
        assert!(TextDatum::TopLeft.is_top());
        assert!(TextDatum::MiddleLeft.is_vertical_middle());
        assert!(TextDatum::BottomLeft.is_bottom());
        m5.display.set_text_datum(TextDatum::MiddleCenter);
        m5.display.set_text_padding(12);
        assert_eq!(m5.display.text_padding(), 0);
        assert_eq!(m5.display.text_size_x(), 1);
        assert_eq!(m5.display.text_size_y(), 1);
        m5.display.set_text_size(2);
        assert_eq!(m5.display.try_set_text_size(2), Ok(()));
        assert_eq!(
            m5.display.try_set_text_size(0),
            Err(Error::InvalidValue("display text size"))
        );
        assert_eq!(
            m5.display.try_set_text_size(-1),
            Err(Error::InvalidValue("display text size"))
        );
        assert_eq!(m5.display.text_length("abc"), Ok(24));
        assert_eq!(m5.display.text_width("abc"), Ok(24));
        assert_eq!(m5.display.draw_center_string("abc", 160, 120), Ok(24));
        assert_eq!(m5.display.draw_char('A', 0, 0), 8);
        assert_eq!(m5.display.draw_number(42, 0, 0), 0);
        assert_eq!(m5.display.draw_float(1.25, 2, 0, 0), 0);
        m5.display.draw_pixel(1, 1, colors::WHITE);
        m5.display.draw_point(Point::new(2, 2), colors::GREEN);
        assert_eq!(m5.display.read_pixel(1, 1), colors::BLACK);
        assert_eq!(m5.display.read_point(Point::new(2, 2)), colors::BLACK);
        m5.display.write_point(Point::new(3, 3), colors::BLUE);
        m5.display.draw_fast_hline(0, 2, 10, colors::RED);
        assert_eq!(
            m5.display.try_draw_fast_hline(0, 2, 10, colors::RED),
            Ok(())
        );
        assert_eq!(
            m5.display.try_draw_fast_hline(0, 2, 0, colors::RED),
            Err(Error::InvalidValue("display length"))
        );
        m5.display.draw_fast_vline(2, 0, 10, colors::GREEN);
        assert_eq!(
            m5.display.try_draw_fast_vline(2, 0, 10, colors::GREEN),
            Ok(())
        );
        assert_eq!(
            m5.display.try_draw_fast_vline(2, 0, -1, colors::GREEN),
            Err(Error::InvalidValue("display length"))
        );
        assert_eq!(
            m5.display.try_draw_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 12,
                    h: 10,
                },
                colors::WHITE
            ),
            Ok(())
        );
        assert_eq!(
            m5.display.try_fill_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 10,
                },
                colors::WHITE
            ),
            Err(Error::InvalidValue("display rect"))
        );
        m5.display.set_addr_window(Rect {
            x: 0,
            y: 0,
            w: 12,
            h: 10,
        });
        assert_eq!(
            m5.display.try_set_addr_window(Rect {
                x: 0,
                y: 0,
                w: 12,
                h: 10,
            }),
            Ok(())
        );
        m5.display.set_window(0, 0, 11, 9);
        assert_eq!(m5.display.try_set_window(0, 0, 11, 9), Ok(()));
        assert_eq!(
            m5.display.try_set_window(12, 0, 11, 9),
            Err(Error::InvalidValue("display window"))
        );
        assert_eq!(
            m5.display.try_set_window(0, 10, 11, 9),
            Err(Error::InvalidValue("display window"))
        );
        m5.display.set_clip_rect(Rect {
            x: 1,
            y: 2,
            w: 30,
            h: 40,
        });
        assert_eq!(
            m5.display.try_set_clip_rect(Rect {
                x: 1,
                y: 2,
                w: 30,
                h: 40,
            }),
            Ok(())
        );
        assert_eq!(
            m5.display.try_set_clip_rect(Rect {
                x: 1,
                y: 2,
                w: 30,
                h: 0,
            }),
            Err(Error::InvalidValue("display rect"))
        );
        assert_eq!(
            m5.display.clip_rect(),
            Rect {
                x: 0,
                y: 0,
                w: 320,
                h: 240,
            }
        );
        m5.display.clear_clip_rect();
        m5.display.scroll(0, 2);
        m5.display.set_text_scroll(true);
        m5.display.set_scroll_rect(
            Rect {
                x: 0,
                y: 0,
                w: 80,
                h: 40,
            },
            colors::BLACK,
        );
        assert_eq!(
            m5.display.try_set_scroll_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 80,
                    h: 40,
                },
                colors::BLACK
            ),
            Ok(())
        );
        assert_eq!(
            m5.display.try_progress_bar(
                Rect {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 40,
                },
                50
            ),
            Err(Error::InvalidValue("display rect"))
        );
        assert_eq!(m5.display.scroll_rect(), Rect::default());
        m5.display.clear_scroll_rect();
        m5.display.fill_rect_alpha(0, 0, 10, 10, 128, colors::BLUE);
        assert_eq!(
            m5.display.try_fill_rect_alpha(
                Rect {
                    x: 0,
                    y: 0,
                    w: 10,
                    h: 10,
                },
                128,
                colors::BLUE
            ),
            Ok(())
        );
        assert_eq!(
            m5.display.try_fill_rect_alpha(
                Rect {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 10,
                },
                128,
                colors::BLUE
            ),
            Err(Error::InvalidValue("display rect"))
        );
        m5.display.draw_round_rect(0, 0, 20, 10, 3, colors::YELLOW);
        assert_eq!(
            m5.display.try_draw_round_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 20,
                    h: 10,
                },
                3,
                colors::YELLOW
            ),
            Ok(())
        );
        assert_eq!(
            m5.display.try_draw_round_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 20,
                    h: 0,
                },
                3,
                colors::YELLOW
            ),
            Err(Error::InvalidValue("display rect"))
        );
        m5.display.fill_round_rect(0, 0, 20, 10, 3, colors::CYAN);
        assert_eq!(
            m5.display.try_fill_round_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 20,
                    h: 10,
                },
                3,
                colors::CYAN
            ),
            Ok(())
        );
        assert_eq!(
            m5.display.try_fill_round_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 20,
                    h: 10,
                },
                -1,
                colors::CYAN
            ),
            Err(Error::InvalidValue("display radius"))
        );
        m5.display.draw_circle(20, 20, 8, colors::WHITE);
        assert_eq!(m5.display.try_draw_circle(20, 20, 8, colors::WHITE), Ok(()));
        assert_eq!(
            m5.display.try_draw_circle(20, 20, 0, colors::WHITE),
            Err(Error::InvalidValue("display radius"))
        );
        m5.display.fill_circle(20, 20, 7, colors::BLUE);
        assert_eq!(m5.display.try_fill_circle(20, 20, 7, colors::BLUE), Ok(()));
        assert_eq!(
            m5.display.try_fill_circle(20, 20, -1, colors::BLUE),
            Err(Error::InvalidValue("display radius"))
        );
        m5.display.draw_ellipse(20, 20, 8, 4, colors::MAGENTA);
        assert_eq!(
            m5.display.try_draw_ellipse(20, 20, 8, 4, colors::MAGENTA),
            Ok(())
        );
        assert_eq!(
            m5.display.try_draw_ellipse(20, 20, 0, 4, colors::MAGENTA),
            Err(Error::InvalidValue("display radii"))
        );
        m5.display.fill_ellipse(20, 20, 8, 4, colors::ORANGE);
        assert_eq!(
            m5.display.try_fill_ellipse(20, 20, 8, 4, colors::ORANGE),
            Ok(())
        );
        assert_eq!(
            m5.display.try_fill_ellipse(20, 20, 8, -1, colors::ORANGE),
            Err(Error::InvalidValue("display radii"))
        );
        m5.display
            .draw_arc(Point { x: 20, y: 20 }, 4, 8, 0.0, 90.0, colors::WHITE);
        assert_eq!(
            m5.display
                .try_draw_arc(Point { x: 20, y: 20 }, 4, 8, 0.0, 90.0, colors::WHITE),
            Ok(())
        );
        assert_eq!(
            m5.display
                .try_draw_arc(Point { x: 20, y: 20 }, -1, 8, 0.0, 90.0, colors::WHITE),
            Err(Error::InvalidValue("display arc radii"))
        );
        m5.display
            .fill_arc(Point { x: 20, y: 20 }, 4, 8, 90.0, 180.0, colors::WHITE);
        assert_eq!(
            m5.display
                .try_fill_arc(Point { x: 20, y: 20 }, 4, 8, 90.0, 180.0, colors::WHITE),
            Ok(())
        );
        assert_eq!(
            m5.display.try_fill_arc(
                Point { x: 20, y: 20 },
                4,
                8,
                90.0,
                f32::INFINITY,
                colors::WHITE
            ),
            Err(Error::InvalidValue("display arc angle"))
        );
        m5.display.draw_triangle(
            Point { x: 0, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 5, y: 10 },
            colors::WHITE,
        );
        m5.display.fill_triangle(
            Point { x: 0, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 5, y: 10 },
            colors::WHITE,
        );
        m5.display.progress_bar(
            Rect {
                x: 0,
                y: 0,
                w: 100,
                h: 8,
            },
            66,
        );
    }

    #[test]
    fn invalid_strings_are_rejected_before_ffi() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert_eq!(m5.display.print("bad\0string"), Err(Error::InvalidString));
        assert_eq!(
            m5.display.draw_center_string("bad\0string", 0, 0),
            Err(Error::InvalidString)
        );
        assert_eq!(
            m5.display.text_width("bad\0string"),
            Err(Error::InvalidString)
        );
        assert_eq!(sd_exists("bad\0path"), Err(Error::InvalidString));
        assert_eq!(sd_file_size("bad\0path"), Err(Error::InvalidString));
        let mut buffer = [0_u8; 4];
        assert_eq!(
            sd_read_file("bad\0path", &mut buffer),
            Err(Error::InvalidString)
        );
        assert_eq!(
            sd_try_read_file_exact("bad\0path", &mut buffer),
            Err(Error::InvalidString)
        );
        assert_eq!(sd_write_file("bad\0path", b"x"), Err(Error::InvalidString));
        assert_eq!(
            sd_try_write_file_all("bad\0path", b"x"),
            Err(Error::InvalidString)
        );
        assert_eq!(sd_append_file("bad\0path", b"x"), Err(Error::InvalidString));
        assert_eq!(
            sd_try_append_file_all("bad\0path", b"x"),
            Err(Error::InvalidString)
        );
        assert_eq!(sd_remove_file("bad\0path"), Err(Error::InvalidString));
        assert_eq!(sd_try_remove_file("bad\0path"), Err(Error::InvalidString));
        assert_eq!(sd_mkdir("bad\0path"), Err(Error::InvalidString));
        assert_eq!(sd_try_mkdir("bad\0path"), Err(Error::InvalidString));
        assert_eq!(sd_rmdir("bad\0path"), Err(Error::InvalidString));
        assert_eq!(sd_try_rmdir("bad\0path"), Err(Error::InvalidString));
        assert_eq!(sd_rename("bad\0path", "/new"), Err(Error::InvalidString));
        assert_eq!(
            sd_try_rename("bad\0path", "/new"),
            Err(Error::InvalidString)
        );
        assert_eq!(sd_rename("/old", "bad\0path"), Err(Error::InvalidString));
        assert_eq!(
            sd_try_rename("/old", "bad\0path"),
            Err(Error::InvalidString)
        );
        assert_eq!(sd_list_dir("bad\0path", 4), Err(Error::InvalidString));
    }

    #[test]
    fn generic_sd_api_uses_host_stubs() {
        let mut buffer = [0_u8; 16];

        assert!(!sd_begin());
        assert_eq!(sd_try_begin(), Err(Error::Unavailable("sd")));
        sd_end();
        assert_eq!(sd_card_type(), SdCardType::None);
        assert_eq!(SdCardType::from_raw(0), SdCardType::None);
        assert_eq!(SdCardType::from_raw(1), SdCardType::Mmc);
        assert_eq!(SdCardType::from_raw(2), SdCardType::Sd);
        assert_eq!(SdCardType::from_raw(3), SdCardType::Sdhc);
        assert_eq!(SdCardType::from_raw(99), SdCardType::Unknown(99));
        assert_eq!(SdCardType::Sdhc.raw(), 3);
        assert_eq!(SdCardType::Unknown(99).raw(), 99);
        assert!(SdCardType::None.is_absent());
        assert!(!SdCardType::None.is_present());
        assert!(SdCardType::Mmc.is_present());
        assert!(SdCardType::Mmc.is_mmc());
        assert!(SdCardType::Sd.is_sd());
        assert!(SdCardType::Sd.is_standard_capacity_sd());
        assert!(SdCardType::Sdhc.is_sd());
        assert!(SdCardType::Sdhc.is_high_capacity_sd());
        assert!(SdCardType::Unknown(99).is_unknown());
        assert_eq!(sd_info(), SdCardInfo::default());
        let sd_info = SdCardInfo {
            size_bytes: 64 * SdCardInfo::BYTES_PER_MEBIBYTE,
            total_bytes: 32 * SdCardInfo::BYTES_PER_MEBIBYTE,
            used_bytes: 8 * SdCardInfo::BYTES_PER_MEBIBYTE,
        };
        assert!(sd_info.has_capacity());
        assert_eq!(sd_info.free_bytes(), 24 * SdCardInfo::BYTES_PER_MEBIBYTE);
        assert_eq!(sd_info.size_mebibytes(), 64);
        assert_eq!(sd_info.total_mebibytes(), 32);
        assert_eq!(sd_info.used_mebibytes(), 8);
        assert_eq!(sd_info.free_mebibytes(), 24);
        assert_eq!(sd_info.used_fraction(), Some(0.25));
        assert_eq!(sd_info.used_percent(), Some(25.0));
        assert!(!SdCardInfo::default().has_capacity());
        assert_eq!(SdCardInfo::default().used_percent(), None);
        assert_eq!(
            SdCardInfo {
                size_bytes: 0,
                total_bytes: 10,
                used_bytes: 20,
            }
            .free_bytes(),
            0
        );
        assert_eq!(sd_exists("/music.wav"), Ok(false));
        assert_eq!(sd_file_size("/music.wav"), Ok(0));
        assert_eq!(sd_is_directory("/music.wav"), Ok(false));
        assert_eq!(sd_read_file("/music.wav", &mut buffer), Ok(0));
        assert_eq!(
            sd_try_read_file_exact("/music.wav", &mut buffer),
            Err(Error::Unavailable("sd read"))
        );
        let mut empty: [u8; 0] = [];
        assert_eq!(sd_try_read_file_exact("/music.wav", &mut empty), Ok(()));
        assert_eq!(sd_write_file("/music.wav", b"hello"), Ok(0));
        assert_eq!(
            sd_try_write_file_all("/music.wav", b"hello"),
            Err(Error::Unavailable("sd write"))
        );
        assert_eq!(sd_try_write_file_all("/music.wav", b""), Ok(()));
        assert_eq!(sd_append_file("/music.wav", b"\n"), Ok(0));
        assert_eq!(
            sd_try_append_file_all("/music.wav", b"\n"),
            Err(Error::Unavailable("sd write"))
        );
        assert_eq!(sd_try_append_file_all("/music.wav", b""), Ok(()));
        assert_eq!(sd_remove_file("/music.wav"), Ok(false));
        assert_eq!(
            sd_try_remove_file("/music.wav"),
            Err(Error::Unavailable("sd remove"))
        );
        assert_eq!(sd_mkdir("/m5rs"), Ok(false));
        assert_eq!(sd_try_mkdir("/m5rs"), Err(Error::Unavailable("sd mkdir")));
        assert_eq!(sd_rmdir("/m5rs"), Ok(false));
        assert_eq!(sd_try_rmdir("/m5rs"), Err(Error::Unavailable("sd rmdir")));
        assert_eq!(sd_rename("/old", "/new"), Ok(false));
        assert_eq!(
            sd_try_rename("/old", "/new"),
            Err(Error::Unavailable("sd rename"))
        );
        assert_eq!(sd_list_dir("/", 4), Ok(Vec::new()));
        assert_eq!(sd_list_dir("/", 0), Ok(Vec::new()));
        assert_eq!(sd_is_directory("bad\0path"), Err(Error::InvalidString));
    }

    #[test]
    fn generic_i2c_api_uses_host_stubs() {
        let mut buffer = [0_u8; 4];
        let address = I2cAddress::new(0x42).expect("valid 7-bit address");
        let pins = I2cPins::new(21, 22);
        let config = I2cConfig::default().with_frequency_hz(I2cConfig::FAST_FREQUENCY_HZ);

        assert_eq!(pins.pins(), (21, 22));
        assert_eq!(pins.sda(), 21);
        assert_eq!(pins.scl(), 22);
        assert_eq!(I2cConfig::DEFAULT_FREQUENCY_HZ, 100_000);
        assert_eq!(DEFAULT_I2C_FREQUENCY_HZ, I2cConfig::DEFAULT_FREQUENCY_HZ);
        assert_eq!(I2cConfig::default().frequency_hz(), 100_000);
        assert!(I2cConfig::default().is_standard_mode());
        assert_eq!(config.frequency_hz(), 400_000);
        assert!(config.is_fast_mode());
        assert!(!config.is_standard_mode());
        assert_eq!(I2cAddress::from_7bit(0x42), Some(address));
        assert_eq!(I2cAddress::from_7bit(0x80), None);
        assert_eq!(I2cAddress::from_8bit(0x84), address);
        assert_eq!(address.get(), 0x42);
        assert_eq!(address.raw(), 0x42);
        assert_eq!(address.as_7bit(), 0x42);
        assert_eq!(address.write_address_8bit(), 0x84);
        assert_eq!(address.read_address_8bit(), 0x85);
        assert!(address.is_non_reserved());
        assert!(!address.is_reserved());
        assert!(I2cAddress::new(0x00)
            .expect("valid reserved address")
            .is_reserved());
        assert!(I2cAddress::new(0x78)
            .expect("valid reserved address")
            .is_reserved());
        assert_eq!(
            I2cPins::from_pin_names(PinName::PORT_A_SDA, PinName::PORT_A_SCL),
            None
        );
        assert_eq!(I2cPins::port_a(), None);
        assert!(!i2c_begin(pins));
        assert!(!i2c_begin_with(pins, 400_000));
        assert_eq!(i2c_try_begin(pins), Err(Error::Unavailable("i2c")));
        assert_eq!(
            i2c_try_begin_with(pins, 400_000),
            Err(Error::Unavailable("i2c"))
        );
        assert_eq!(
            i2c_try_begin_with(pins, 0),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!i2c_begin_config(pins, config));
        assert_eq!(
            i2c_try_begin_config(pins, config),
            Err(Error::Unavailable("i2c"))
        );
        assert_eq!(
            i2c_try_begin_config(pins, I2cConfig::new(0)),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!i2c_begin_with_config(pins, config));
        assert_eq!(
            i2c_try_begin_with_config(pins, config),
            Err(Error::Unavailable("i2c"))
        );
        assert_eq!(
            i2c_try_begin_with_config(pins, I2cConfig::new(0)),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!i2c_probe(address));
        assert!(!i2c_write(address, &[1, 2]));
        assert_eq!(
            i2c_try_write(address, &[1, 2]),
            Err(Error::Unavailable("i2c"))
        );
        assert_eq!(i2c_read(address, &mut buffer), 0);
        assert_eq!(
            i2c_try_read(address, &mut buffer),
            Err(Error::Unavailable("i2c"))
        );
        assert_eq!(
            i2c_try_read_exact(address, &mut buffer),
            Err(Error::Unavailable("i2c"))
        );
        assert!(!i2c_write_reg(address, 0x00, &[1, 2]));
        assert_eq!(
            i2c_try_write_reg(address, 0x00, &[1, 2]),
            Err(Error::Unavailable("i2c"))
        );
        assert_eq!(i2c_read_reg(address, 0x00, &mut buffer), 0);
        assert_eq!(
            i2c_try_read_reg(address, 0x00, &mut buffer),
            Err(Error::Unavailable("i2c"))
        );
        assert_eq!(
            i2c_try_read_reg_exact(address, 0x00, &mut buffer),
            Err(Error::Unavailable("i2c"))
        );
        assert!(i2c_scan().is_empty());
        i2c_end();
    }

    #[test]
    fn stackchan_motion_contract_clamps_like_mcp() {
        let command = StackChanMove::from_mcp(12.25, 91.0, -5);
        assert_eq!(command.x_tenths, 123);
        assert_eq!(command.y_tenths, StackChanMotionContract::Y_MAX_TENTHS);
        assert_eq!(
            command.speed_percent,
            StackChanMotionContract::SPEED_MIN_PERCENT
        );

        let command = StackChanMove::new(180, -10, 150);
        assert_eq!(command.x_tenths, StackChanMotionContract::X_MAX_TENTHS);
        assert_eq!(command.y_tenths, StackChanMotionContract::Y_MIN_TENTHS);
        assert_eq!(
            command.speed_percent,
            StackChanMotionContract::SPEED_MAX_PERCENT
        );
        assert_eq!(
            command.bsp_y_tenths(),
            StackChanMotionContract::Y_HARDWARE_MIN_TENTHS
        );
        assert_eq!(command.bsp_speed(), 1000);

        let command = StackChanMove::from_tenths(-2000, 900, 42);
        assert_eq!(command.x_tenths, StackChanMotionContract::X_MIN_TENTHS);
        assert_eq!(command.y_tenths, StackChanMotionContract::Y_MAX_TENTHS);
        assert_eq!(
            command.bsp_y_tenths(),
            StackChanMotionContract::Y_HARDWARE_MAX_TENTHS
        );
        assert_eq!(command.bsp_speed(), 420);
    }

    #[test]
    fn stackchan_bsp_motion_uses_host_stub_boundary() {
        assert!(matches!(
            StackChanBspMotion::begin(),
            Err(Error::Unavailable("stackchan bsp motion"))
        ));
    }

    #[test]
    fn generic_uart_api_uses_host_stubs() {
        let mut buffer = [0_u8; 8];
        let pins = UartPins::new(16, 17);
        let config = UartConfig::default().with_baud(230_400);

        assert_eq!(pins.pins(), (16, 17));
        assert_eq!(pins.rx(), 16);
        assert_eq!(pins.tx(), 17);
        assert_eq!(UartConfig::DEFAULT_BAUD, 115_200);
        assert_eq!(UartConfig::default().baud(), 115_200);
        assert!(UartConfig::default().is_default_baud());
        assert_eq!(config.baud(), 230_400);
        assert!(!config.is_default_baud());
        assert_eq!(
            UartPins::from_pin_names(PinName::PORT_C_RXD, PinName::PORT_C_TXD),
            None
        );
        assert_eq!(UartPins::port_c(), None);
        assert_eq!(UartPins::port_d(), None);
        assert_eq!(UartPins::port_e(), None);
        assert!(!uart_begin(pins, 115_200));
        assert_eq!(
            uart_try_begin(pins, 115_200),
            Err(Error::Unavailable("uart"))
        );
        assert_eq!(uart_try_begin(pins, 0), Err(Error::InvalidValue("baud")));
        assert!(!uart_begin_config(pins, config));
        assert_eq!(
            uart_try_begin_config(pins, config),
            Err(Error::Unavailable("uart"))
        );
        assert_eq!(
            uart_try_begin_config(pins, UartConfig::new(0)),
            Err(Error::InvalidValue("baud"))
        );
        assert_eq!(uart_available(), 0);
        assert_eq!(uart_read(&mut buffer), 0);
        assert_eq!(uart_try_read(&mut buffer), Err(Error::Unavailable("uart")));
        assert_eq!(uart_write(b"ping"), 0);
        assert_eq!(uart_try_write(b"ping"), Err(Error::Unavailable("uart")));
        assert_eq!(uart_try_write_all(b"ping"), Err(Error::Unavailable("uart")));
        assert_eq!(uart_write_byte(b'\n'), 0);
        assert_eq!(uart_try_write_byte(b'\n'), Err(Error::Unavailable("uart")));
        assert_eq!(uart_write_str("pong"), 0);
        assert_eq!(uart_try_write_str("pong"), Err(Error::Unavailable("uart")));
        assert_eq!(
            uart_try_write_str_all("pong"),
            Err(Error::Unavailable("uart"))
        );
        uart_flush();
        uart_end();
    }

    #[test]
    fn generic_spi_api_uses_host_stubs() {
        let mut rx = [0_u8; 3];
        let tx = [1_u8, 2, 3];
        let pins = SpiPins::new(18, 19, 23, 5);
        let config = SpiConfig::default()
            .with_frequency_hz(2_000_000)
            .with_mode(SpiMode::Mode3)
            .with_bit_order(SpiBitOrder::LsbFirst);

        assert_eq!(pins.pins(), (18, 19, 23, 5));
        assert_eq!(pins.sck(), 18);
        assert_eq!(pins.miso(), 19);
        assert_eq!(pins.mosi(), 23);
        assert_eq!(pins.cs(), 5);
        assert_eq!(
            SpiPins::from_pin_names(
                PinName::SD_SPI_SCLK,
                PinName::SD_SPI_CIPO,
                PinName::SD_SPI_COPI,
                PinName::SD_SPI_SS
            ),
            None
        );
        assert_eq!(SpiPins::sd(), None);
        assert_eq!(SpiConfig::default().frequency_hz, DEFAULT_SPI_FREQUENCY_HZ);
        assert_eq!(SpiConfig::default().mode, SpiMode::Mode0);
        assert_eq!(SpiConfig::default().bit_order, SpiBitOrder::MsbFirst);
        assert_eq!(config.frequency_hz(), 2_000_000);
        assert_eq!(config.mode(), SpiMode::Mode3);
        assert_eq!(config.bit_order(), SpiBitOrder::LsbFirst);
        assert!(config.is_lsb_first());
        assert!(!config.is_msb_first());
        assert_eq!(
            SpiConfig::new(2_000_000, SpiMode::Mode3, SpiBitOrder::LsbFirst),
            config
        );
        assert_eq!(SpiBitOrder::from_raw(0), Some(SpiBitOrder::MsbFirst));
        assert_eq!(SpiBitOrder::from_raw(1), Some(SpiBitOrder::LsbFirst));
        assert_eq!(SpiBitOrder::from_raw(2), None);
        assert_eq!(SpiBitOrder::from_lsb_first(false), SpiBitOrder::MsbFirst);
        assert_eq!(SpiBitOrder::from_lsb_first(true), SpiBitOrder::LsbFirst);
        assert_eq!(SpiBitOrder::LsbFirst.raw(), 1);
        assert!(SpiBitOrder::MsbFirst.msb_first());
        assert!(!SpiBitOrder::MsbFirst.lsb_first());
        assert!(SpiBitOrder::LsbFirst.lsb_first());
        assert_eq!(SpiMode::from_raw(0), Some(SpiMode::Mode0));
        assert_eq!(SpiMode::from_raw(3), Some(SpiMode::Mode3));
        assert_eq!(SpiMode::from_raw(4), None);
        assert_eq!(SpiMode::Mode3.raw(), 3);
        assert!(!SpiMode::Mode0.clock_polarity());
        assert!(!SpiMode::Mode0.clock_phase());
        assert!(!SpiMode::Mode1.clock_polarity());
        assert!(SpiMode::Mode1.clock_phase());
        assert!(SpiMode::Mode2.clock_polarity());
        assert!(!SpiMode::Mode2.clock_phase());
        assert!(SpiMode::Mode3.clock_polarity());
        assert!(SpiMode::Mode3.clock_phase());
        assert!(!spi_begin(pins));
        assert_eq!(spi_try_begin(pins), Err(Error::Unavailable("spi")));
        assert_eq!(spi_transfer_byte(0xff, config), 0);
        assert_eq!(spi_try_transfer_byte(0xff, config), Ok(0));
        assert_eq!(
            spi_try_transfer_byte(
                0xff,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!spi_transfer(&tx, &mut rx, config));
        assert_eq!(
            spi_try_transfer(&tx, &mut rx, config),
            Err(Error::Unavailable("spi"))
        );
        assert_eq!(
            spi_try_transfer(
                &tx,
                &mut rx,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert_eq!(rx, [0, 0, 0]);
        assert!(!spi_read(&mut rx, config));
        assert_eq!(
            spi_try_read(&mut rx, config),
            Err(Error::Unavailable("spi"))
        );
        assert_eq!(
            spi_try_read(
                &mut rx,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!spi_write(&tx, config));
        assert_eq!(spi_try_write(&tx, config), Err(Error::Unavailable("spi")));
        assert_eq!(
            spi_try_write_all(&tx, config),
            Err(Error::Unavailable("spi"))
        );
        assert_eq!(
            spi_try_write(
                &tx,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert_eq!(
            spi_try_write_all(
                &tx,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!spi_transfer(&tx, &mut rx[..2], config));
        assert_eq!(
            spi_try_transfer(&tx, &mut rx[..2], config),
            Err(Error::InvalidValue("spi transfer length"))
        );
        spi_end();
    }

    #[test]
    fn generic_gpio_api_uses_host_stubs() {
        let pin = GpioPin::new(5);
        let analog_config = AnalogOutputConfig::new(128)
            .with_frequency_hz(2_000)
            .with_resolution_bits(10);

        assert_eq!(GpioPin::new(5).raw(), 5);
        assert_eq!(pin.pin(), 5);
        assert_eq!(GpioPin::from_pin_name(PinName::PORT_B_IN), None);
        assert_eq!(GpioMode::from_raw(0), Some(GpioMode::Input));
        assert_eq!(GpioMode::from_raw(3), Some(GpioMode::InputPulldown));
        assert_eq!(GpioMode::from_raw(4), None);
        assert_eq!(GpioMode::InputPullup.raw(), 2);
        assert_eq!(AnalogOutputConfig::DEFAULT_FREQUENCY_HZ, 1_000);
        assert_eq!(AnalogOutputConfig::DEFAULT_RESOLUTION_BITS, 8);
        assert_eq!(AnalogOutputConfig::default().duty(), 0);
        assert_eq!(AnalogOutputConfig::new(128).duty(), 128);
        assert_eq!(analog_config.duty(), 128);
        assert_eq!(analog_config.frequency_hz(), 2_000);
        assert_eq!(analog_config.resolution_bits(), 10);
        assert!(AnalogOutputConfig::default().is_default_frequency());
        assert!(AnalogOutputConfig::default().is_default_resolution());
        assert!(!analog_config.is_default_frequency());
        assert!(!analog_config.is_default_resolution());
        assert_eq!(
            AnalogOutputConfig::default().with_duty(64),
            AnalogOutputConfig::new(64)
        );
        assert!(GpioMode::Input.is_input());
        assert!(GpioMode::InputPullup.is_input());
        assert!(GpioMode::InputPulldown.is_input());
        assert!(!GpioMode::Output.is_input());
        assert!(GpioMode::Output.is_output());
        assert!(GpioMode::InputPullup.has_pullup());
        assert!(GpioMode::InputPulldown.has_pulldown());
        assert!(GpioMode::InputPullup.has_pull());
        assert!(!GpioMode::Input.has_pull());
        assert!(!gpio_pin_mode(pin, GpioMode::Input));
        assert!(!gpio_pin_mode(pin, GpioMode::Output));
        assert!(!gpio_pin_mode(pin, GpioMode::InputPullup));
        assert!(!gpio_pin_mode(pin, GpioMode::InputPulldown));
        assert_eq!(
            gpio_try_pin_mode(pin, GpioMode::Input),
            Err(Error::Unavailable("gpio"))
        );
        assert!(!gpio_write(pin, true));
        assert_eq!(gpio_try_write(pin, true), Err(Error::Unavailable("gpio")));
        assert_eq!(gpio_read(pin), None);
        assert_eq!(gpio_try_read(pin), Err(Error::Unavailable("gpio")));
        assert_eq!(analog_read(pin), None);
        assert_eq!(analog_try_read(pin), Err(Error::Unavailable("analog")));
        assert_eq!(analog_read_millivolts(pin), None);
        assert_eq!(
            analog_try_read_millivolts(pin),
            Err(Error::Unavailable("analog"))
        );
        assert!(!analog_write(pin, 128));
        assert_eq!(
            analog_try_write(pin, 128),
            Err(Error::Unavailable("analog"))
        );
        assert!(!analog_write_frequency(pin, 1_000));
        assert_eq!(
            analog_try_write_frequency(pin, 1_000),
            Err(Error::Unavailable("analog"))
        );
        assert_eq!(
            analog_try_write_frequency(pin, 0),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!analog_write_resolution(pin, 8));
        assert_eq!(
            analog_try_write_resolution(pin, 8),
            Err(Error::Unavailable("analog"))
        );
        assert!(!analog_write_config(pin, analog_config));
        assert_eq!(
            analog_try_write_config(pin, analog_config),
            Err(Error::Unavailable("analog"))
        );
        assert_eq!(
            analog_try_write_config(pin, AnalogOutputConfig::new(128).with_frequency_hz(0)),
            Err(Error::InvalidValue("frequency"))
        );
    }

    #[test]
    fn mic_rms_uses_recorded_buffer() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        let mut buffer = [0_i16; 8];
        let stats = analyze_i16_samples(&[-3, 4]).expect("non-empty samples should analyze");

        assert_eq!(analyze_i16_samples(&[]), None);
        assert_eq!(rms_i16_samples(&[]), None);
        assert_eq!(stats.min, -3);
        assert_eq!(stats.max, 4);
        assert_eq!(stats.peak, 4);
        assert!((stats.mean - 0.5).abs() < f32::EPSILON);
        assert!((stats.rms - 3.535_534).abs() < 0.000_1);
        assert_eq!(stats.dynamic_range(), 7);
        assert_eq!(stats.peak_fraction(), 4.0 / MicStats::I16_FULL_SCALE);
        assert_eq!(stats.peak_percent(), 100.0 * 4.0 / MicStats::I16_FULL_SCALE);
        assert!(
            (stats.rms_fraction() - (stats.rms / MicStats::I16_FULL_SCALE)).abs() < f32::EPSILON
        );
        assert!((stats.rms_percent() - (stats.rms_fraction() * 100.0)).abs() < f32::EPSILON);
        assert!(!stats.is_silent());
        assert!(MicStats::default().is_silent());
        assert_eq!(
            analyze_i16_samples(&[i16::MIN])
                .expect("non-empty samples should analyze")
                .peak,
            32_768
        );

        assert_eq!(m5.mic.try_begin(), Ok(()));
        assert!(m5.mic.begin());
        assert!(m5.mic.is_enabled());
        assert!(!m5.mic.is_recording());
        assert_eq!(m5.mic.config(), MicConfig::default());
        assert_eq!(m5.mic.try_config(), Ok(MicConfig::default()));
        assert_eq!(m5.mic.noise_filter_level(), 0);
        assert!(m5.mic.set_noise_filter_level(4));
        assert_eq!(m5.mic.try_set_noise_filter_level(4), Ok(()));
        let mic_config = MicConfig::default()
            .with_sample_rate(22_050)
            .with_left_channel(true)
            .with_over_sampling(4)
            .with_noise_filter_level(2);
        assert_eq!(m5.mic.set_config(mic_config), Ok(()));
        assert_eq!(
            mic_config,
            MicConfig {
                sample_rate: 22_050,
                left_channel: true,
                over_sampling: 4,
                noise_filter_level: 2,
                ..MicConfig::default()
            }
        );
        assert_eq!(
            mic_config.pins(),
            (
                MicConfig::default().pin_data_in,
                MicConfig::default().pin_bck,
                MicConfig::default().pin_mck,
                MicConfig::default().pin_ws,
            )
        );
        assert_eq!(mic_config.sample_rate(), 22_050);
        assert!(mic_config.uses_left_channel());
        assert!(!mic_config.is_stereo());
        assert_eq!(mic_config.over_sampling(), 4);
        assert_eq!(
            mic_config.magnification(),
            MicConfig::default().magnification
        );
        assert_eq!(mic_config.noise_filter_level(), 2);
        assert_eq!(mic_config.uses_adc(), MicConfig::default().use_adc);
        assert_eq!(
            mic_config.dma_buffer(),
            (
                MicConfig::default().dma_buf_len,
                MicConfig::default().dma_buf_count
            )
        );
        assert_eq!(
            mic_config.task(),
            (
                MicConfig::default().task_priority,
                MicConfig::default().task_pinned_core
            )
        );
        assert_eq!(mic_config.i2s_port(), MicConfig::default().i2s_port);
        assert_eq!(
            m5.mic.set_config(MicConfig::default().with_sample_rate(0)),
            Err(Error::InvalidValue("audio sample rate"))
        );
        assert_eq!(
            m5.mic
                .set_config(MicConfig::default().with_dma_buffer(0, 8)),
            Err(Error::InvalidValue("audio dma buffer length"))
        );
        assert_eq!(
            m5.mic
                .set_config(MicConfig::default().with_dma_buffer(256, 0)),
            Err(Error::InvalidValue("audio dma buffer count"))
        );
        assert_eq!(m5.mic.try_record_i16(&mut buffer), Ok(()));
        assert!(m5.mic.record_i16_at(&mut buffer, 22_050));
        assert_eq!(m5.mic.try_record_i16_at(&mut buffer, 22_050), Ok(()));
        assert_eq!(m5.mic.rms(&mut buffer), Some(0.0));
        assert_eq!(m5.mic.try_rms(&mut buffer), Ok(0.0));
        assert_eq!(m5.mic.stats(&mut buffer), Some(MicStats::default()));
        assert_eq!(m5.mic.try_stats(&mut buffer), Ok(MicStats::default()));
        assert_eq!(m5.mic.rms_at(&mut buffer, 22_050), Some(0.0));
        assert_eq!(m5.mic.try_rms_at(&mut buffer, 22_050), Ok(0.0));
        assert_eq!(
            m5.mic.stats_at(&mut buffer, 22_050),
            Some(MicStats::default())
        );
        assert_eq!(
            m5.mic.try_stats_at(&mut buffer, 22_050),
            Ok(MicStats::default())
        );

        let mut empty: [i16; 0] = [];
        assert_eq!(
            m5.mic.try_record_i16(&mut empty),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(
            m5.mic.try_rms(&mut empty),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(
            m5.mic.try_stats(&mut empty),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(
            m5.mic.try_record_i16_at(&mut empty, 22_050),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(
            m5.mic.try_record_i16_at(&mut buffer, 0),
            Err(Error::InvalidValue("audio sample rate"))
        );
        assert_eq!(
            m5.mic.try_rms_at(&mut empty, 22_050),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(
            m5.mic.try_rms_at(&mut buffer, 0),
            Err(Error::InvalidValue("audio sample rate"))
        );
        assert_eq!(
            m5.mic.try_stats_at(&mut empty, 22_050),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(
            m5.mic.try_stats_at(&mut buffer, 0),
            Err(Error::InvalidValue("audio sample rate"))
        );
        m5.mic.end();
    }

    #[test]
    fn speaker_config_and_playback_options_use_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        let config = m5.speaker.config();

        assert_eq!(config, SpeakerConfig::default());
        assert_eq!(m5.speaker.try_config(), Ok(SpeakerConfig::default()));
        assert_eq!(m5.speaker.try_begin(), Ok(()));
        assert!(m5.speaker.begin());
        assert!(m5.speaker.is_enabled());
        assert!(!m5.speaker.is_running());
        assert_eq!(m5.speaker.volume(), 64);
        assert_eq!(m5.speaker.playing_channels(), 0);
        let speaker_config = SpeakerConfig::default()
            .with_sample_rate(44_100)
            .with_stereo(true)
            .with_dma_buffer(256, 4);
        m5.speaker.set_config(speaker_config);
        assert_eq!(
            speaker_config,
            SpeakerConfig {
                sample_rate: 44_100,
                stereo: true,
                dma_buf_count: 4,
                ..SpeakerConfig::default()
            }
        );
        assert_eq!(speaker_config.pins(), (-1, -1, -1));
        assert_eq!(speaker_config.sample_rate(), 44_100);
        assert!(speaker_config.is_stereo());
        assert!(!speaker_config.is_buzzer());
        assert!(!speaker_config.uses_dac());
        assert_eq!(speaker_config.dac_zero_level(), 0);
        assert_eq!(speaker_config.magnification(), 16);
        assert_eq!(speaker_config.dma_buffer(), (256, 4));
        assert_eq!(speaker_config.task(), (2, u8::MAX));
        assert_eq!(speaker_config.i2s_port(), 0);
        assert_eq!(m5.speaker.try_set_config(speaker_config), Ok(()));
        assert_eq!(
            m5.speaker
                .try_set_config(SpeakerConfig::default().with_sample_rate(0)),
            Err(Error::InvalidValue("audio sample rate"))
        );
        assert_eq!(
            m5.speaker
                .try_set_config(SpeakerConfig::default().with_dma_buffer(0, 8)),
            Err(Error::InvalidValue("audio dma buffer length"))
        );
        assert_eq!(
            m5.speaker
                .try_set_config(SpeakerConfig::default().with_dma_buffer(256, 0)),
            Err(Error::InvalidValue("audio dma buffer count"))
        );
        m5.speaker.set_volume(32);
        assert_eq!(
            m5.speaker.try_tone(0, 10),
            Err(Error::InvalidValue("audio frequency"))
        );
        assert_eq!(m5.speaker.try_tone(440, 10), Ok(()));
        assert!(m5.speaker.tone(440, 10));
        assert_eq!(
            m5.speaker.try_tone_ex(0.0, 10, Some(1)),
            Err(Error::InvalidValue("audio frequency"))
        );
        assert_eq!(
            m5.speaker.try_tone_ex(f32::NAN, 10, Some(1)),
            Err(Error::InvalidValue("audio frequency"))
        );
        assert_eq!(m5.speaker.try_tone_ex(880.0, 10, Some(1)), Ok(()));
        assert!(m5.speaker.tone_ex(880.0, 10, Some(1)));
        assert_eq!(
            m5.speaker.try_play_i16(&[], 22_050),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(
            m5.speaker.try_play_i16(&[0], 0),
            Err(Error::InvalidValue("audio sample rate"))
        );
        assert_eq!(m5.speaker.try_play_i16(&[0, 1, -1], 22_050), Ok(()));
        assert!(m5.speaker.play_i16(&[0, 1, -1], 22_050));
        assert_eq!(
            m5.speaker.try_play_u8(&[], 22_050),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(
            m5.speaker.try_play_u8(&[0], 0),
            Err(Error::InvalidValue("audio sample rate"))
        );
        assert_eq!(m5.speaker.try_play_u8(&[0, 128, 255], 22_050), Ok(()));
        assert!(m5.speaker.play_u8(&[0, 128, 255], 22_050));
        assert_eq!(
            m5.speaker.try_play_wav(b""),
            Err(Error::InvalidValue("audio data"))
        );
        assert_eq!(m5.speaker.try_play_wav(b"RIFF"), Ok(()));
        assert!(m5.speaker.play_wav(b"RIFF"));

        let options = AudioPlaybackOptions::new()
            .with_stereo(true)
            .with_repeat(2)
            .with_channel(Some(1))
            .with_stop_current_sound(true);
        let invalid_repeat = options.with_repeat(0);
        assert_eq!(AudioPlaybackOptions::default(), AudioPlaybackOptions::new());
        assert_eq!(
            options,
            AudioPlaybackOptions {
                stereo: true,
                repeat: 2,
                channel: Some(1),
                stop_current_sound: true,
            }
        );
        assert!(options.is_stereo());
        assert_eq!(options.repeat(), 2);
        assert_eq!(options.channel(), Some(1));
        assert!(options.stop_current_sound());
        assert!(!options.targets_all_channels());
        assert!(AudioPlaybackOptions::default().targets_all_channels());
        assert_eq!(
            m5.speaker
                .try_play_i16_with_options(&[0], 44_100, invalid_repeat),
            Err(Error::InvalidValue("audio repeat"))
        );
        assert_eq!(
            m5.speaker
                .try_play_i16_with_options(&[0, 1, -1], 44_100, options),
            Ok(())
        );
        assert!(m5
            .speaker
            .play_i16_with_options(&[0, 1, -1], 44_100, options));
        assert_eq!(
            m5.speaker
                .try_play_u8_with_options(&[0, 128, 255], 44_100, options),
            Ok(())
        );
        assert!(m5
            .speaker
            .play_u8_with_options(&[0, 128, 255], 44_100, options));
        assert_eq!(
            m5.speaker.try_play_wav_with_options(b"RIFF", options),
            Ok(())
        );
        assert!(m5.speaker.play_wav_with_options(b"RIFF", options));
        assert!(!m5.speaker.is_playing(None));
        assert!(!m5.speaker.is_playing(Some(1)));
        assert_eq!(m5.speaker.channel_volume(0), 255);
        m5.speaker.set_channel_volume(0, 64);
        m5.speaker.set_all_channel_volume(64);
        m5.speaker.stop(Some(1));
        m5.speaker.stop(None);
        m5.speaker.end();
    }

    #[test]
    fn imu_api_uses_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");

        assert_eq!(m5.imu.try_begin(), Ok(()));
        assert!(m5.imu.begin());
        assert!(m5.imu.is_enabled());
        assert_eq!(m5.imu.kind(), ImuKind::None);
        assert_eq!(ImuKind::from_raw(1), ImuKind::Unknown);
        assert_eq!(ImuKind::from_raw(2), ImuKind::Sh200q);
        assert_eq!(ImuKind::from_raw(3), ImuKind::Mpu6050);
        assert_eq!(ImuKind::from_raw(4), ImuKind::Mpu6886);
        assert_eq!(ImuKind::from_raw(5), ImuKind::Mpu9250);
        assert_eq!(ImuKind::from_raw(6), ImuKind::Bmi270);
        assert_eq!(ImuKind::from_raw(99), ImuKind::Raw(99));
        assert_eq!(ImuKind::Bmi270.raw(), 6);
        assert_eq!(ImuKind::Raw(99).raw(), 99);
        let unit_z = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3::ZERO, Vec3::default());
        assert_eq!(unit_z.components(), [0.0, 0.0, 1.0]);
        assert_eq!(unit_z.magnitude_squared(), 1.0);
        assert_eq!(unit_z.magnitude(), 1.0);
        assert_eq!(unit_z.scale(2.0), Vec3::new(0.0, 0.0, 2.0));
        assert_eq!(unit_z.dot(Vec3::new(0.0, 2.0, 3.0)), 3.0);
        assert_eq!(unit_z.normalized(), Some(unit_z));
        assert_eq!(Vec3::ZERO.normalized(), None);
        assert!(m5.imu.update());
        assert_eq!(m5.imu.try_update(), Ok(()));
        assert_eq!(m5.imu.accel(), Some(unit_z));
        assert_eq!(m5.imu.try_accel(), Ok(unit_z));
        assert_eq!(m5.imu.gyro(), Some(Vec3::default()));
        assert_eq!(m5.imu.try_gyro(), Ok(Vec3::default()));
        assert_eq!(m5.imu.mag(), None);
        assert_eq!(
            m5.imu.try_mag(),
            Err(Error::Unavailable("imu magnetometer"))
        );
        assert_eq!(m5.imu.temperature_c(), Some(25.0));
        assert_eq!(m5.imu.try_temperature_c(), Ok(25.0));
        assert_eq!(
            m5.imu.data(),
            Some(ImuData {
                accel: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0
                },
                gyro: Vec3::default(),
                mag: None,
                temperature_c: Some(25.0),
            })
        );
        assert_eq!(
            m5.imu.try_data(),
            Ok(ImuData {
                accel: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0
                },
                gyro: Vec3::default(),
                mag: None,
                temperature_c: Some(25.0),
            })
        );
        let imu_data = m5.imu.data().expect("host stub should expose imu data");
        assert_eq!(imu_data.accel_magnitude(), 1.0);
        assert_eq!(imu_data.gyro_magnitude(), 0.0);
        assert_eq!(imu_data.mag_magnitude(), None);
        assert!(!imu_data.has_mag());
        assert_eq!(imu_data.temperature_f(), Some(77.0));
        assert_eq!(imu_data.temperature_k(), Some(298.15));
        assert!(!m5.imu.sleep());
        assert_eq!(m5.imu.try_sleep(), Err(Error::Unavailable("imu sleep")));
        assert!(!m5.imu.load_offset_from_nvs());
        assert_eq!(
            m5.imu.try_load_offset_from_nvs(),
            Err(Error::Unavailable("imu nvs offset load"))
        );
        assert!(!m5.imu.save_offset_to_nvs());
        assert_eq!(
            m5.imu.try_save_offset_to_nvs(),
            Err(Error::Unavailable("imu nvs offset save"))
        );
        m5.imu.clear_offset_data();
        assert_eq!(m5.imu.offset_data(0), 0);
        assert_eq!(m5.imu.try_offset_data(0), Ok(0));
        assert_eq!(m5.imu.offset_data_array(), [0; IMU_DATA_SLOTS]);
        m5.imu.set_offset_data(0, 123);
        assert_eq!(m5.imu.try_set_offset_data(0, 456), Ok(()));
        m5.imu.set_offset_data_array([1; IMU_DATA_SLOTS]);
        assert_eq!(m5.imu.raw_data(0), 0);
        assert_eq!(m5.imu.try_raw_data(0), Ok(0));
        assert_eq!(m5.imu.raw_data_array(), [0; IMU_DATA_SLOTS]);
        assert_eq!(
            m5.imu.try_offset_data(IMU_DATA_SLOTS),
            Err(Error::InvalidValue("imu data slot"))
        );
        assert_eq!(
            m5.imu.try_set_offset_data(IMU_DATA_SLOTS, 1),
            Err(Error::InvalidValue("imu data slot"))
        );
        assert_eq!(
            m5.imu.try_raw_data(IMU_DATA_SLOTS),
            Err(Error::InvalidValue("imu data slot"))
        );
        assert!(!m5.imu.set_int_pin_active_logic(true));
        assert_eq!(
            m5.imu.try_set_int_pin_active_logic(true),
            Err(Error::Unavailable("imu interrupt pin"))
        );
        assert_eq!(ImuCalibration::ZERO, ImuCalibration::default());
        let calibration = ImuCalibration::new(1, 2, 3)
            .with_accel_strength(4)
            .with_gyro_strength(5)
            .with_mag_strength(6);
        assert_eq!(calibration.strengths(), (4, 5, 6));
        m5.imu.set_calibration(calibration);
        assert_eq!(m5.imu.try_set_calibration(calibration), Ok(()));
    }

    #[test]
    fn led_color_api_is_available_on_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");

        assert!(!m5.led.begin());
        assert_eq!(m5.led.try_begin(), Err(Error::Unavailable("led")));
        assert!(!m5.led.is_enabled());
        assert_eq!(m5.led.count(), 0);
        m5.led.set_brightness(32);
        assert_eq!(
            m5.led.try_set_brightness(32),
            Err(Error::Unavailable("led"))
        );
        m5.led.set_auto_display(false);
        assert_eq!(
            m5.led.try_set_auto_display(false),
            Err(Error::Unavailable("led"))
        );
        m5.led.set_color(0, rgb::BLUE);
        assert_eq!(
            m5.led.try_set_color(0, rgb::BLUE),
            Err(Error::Unavailable("led"))
        );
        let magenta = RgbColor::from_rgb888(255, 0, 255);
        assert_eq!(magenta, RgbColor::MAGENTA);
        assert_eq!(magenta.raw(), rgb::MAGENTA);
        assert_eq!(magenta.rgb888_components(), (255, 0, 255));
        assert_eq!(RgbColor::new(0xff00_ff00), RgbColor::GREEN);
        assert_eq!(RgbColor::from(rgb::BLUE).blue(), 255);
        assert_eq!(u32::from(RgbColor::RED), rgb::RED);
        m5.led.set_rgb_color(0, RgbColor::BLUE);
        assert_eq!(
            m5.led.try_set_rgb_color(0, RgbColor::BLUE),
            Err(Error::Unavailable("led"))
        );
        m5.led.set_all_color(rgb::RED);
        assert_eq!(
            m5.led.try_set_all_color(rgb::RED),
            Err(Error::Unavailable("led"))
        );
        m5.led.set_all_rgb_color(RgbColor::RED);
        assert_eq!(
            m5.led.try_set_all_rgb_color(RgbColor::RED),
            Err(Error::Unavailable("led"))
        );
        m5.led.display();
        assert_eq!(m5.led.try_display(), Err(Error::Unavailable("led")));
        m5.led.off();
        assert_eq!(m5.led.try_off(), Err(Error::Unavailable("led")));
        assert_eq!(validate_led_index(1, 0), Ok(()));
        assert_eq!(
            validate_led_index(1, 1),
            Err(Error::InvalidValue("led index"))
        );
    }

    #[test]
    fn button_event_api_is_available_on_host_stubs() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        let mut button = m5.buttons.a();

        assert_eq!(ButtonId::from_raw(0), Some(ButtonId::A));
        assert_eq!(ButtonId::from_raw(4), Some(ButtonId::Ext));
        assert_eq!(ButtonId::from_raw(5), None);
        assert_eq!(ButtonId::Pwr.raw(), 3);
        assert_eq!(ButtonState::from_raw(1), ButtonState::Clicked);
        assert_eq!(ButtonState::from_raw(99), ButtonState::Other(99));
        assert_eq!(ButtonState::Clicked.raw(), Some(1));
        assert_eq!(ButtonState::Other(99).raw(), None);
        assert_eq!(ButtonState::Other(99).raw_value(), 99);
        assert!(ButtonState::Clicked.is_known());
        assert!(!ButtonState::Other(99).is_known());
        assert!(ButtonState::NoChange.is_no_change());
        assert!(ButtonState::Clicked.is_clicked());
        assert!(ButtonState::Hold.is_hold());
        assert!(ButtonState::DecideClickCount.is_decide_click_count());
        assert!(ButtonState::Clicked.is_event());
        assert!(!ButtonState::NoChange.is_event());
        assert!(!ButtonState::Other(99).is_event());
        assert_eq!(button.id(), ButtonId::A);
        assert_eq!(m5.buttons.button(ButtonId::Ext).id(), ButtonId::Ext);
        assert_eq!(m5.buttons.pwr().id(), ButtonId::Pwr);
        assert!(!m5.buttons.a_is_pressed());
        assert!(!m5.buttons.a_was_pressed());
        assert!(!m5.buttons.a_was_released());
        assert!(!m5.buttons.b_is_pressed());
        assert!(!m5.buttons.b_was_pressed());
        assert!(!m5.buttons.b_was_released());
        assert!(!m5.buttons.c_is_pressed());
        assert!(!m5.buttons.c_was_pressed());
        assert!(!m5.buttons.c_was_released());
        assert!(!button.is_pressed());
        assert!(!button.is_released());
        assert!(!button.was_pressed());
        assert!(!button.was_released());
        assert!(!button.was_clicked());
        assert!(!button.was_hold());
        assert!(!button.was_single_clicked());
        assert!(!button.was_double_clicked());
        assert!(!button.was_change_pressed());
        assert!(!button.is_holding());
        assert!(!button.was_released_after_hold());
        assert!(!button.was_release_for(500));
        assert!(!button.pressed_for(500));
        assert!(!button.released_for(500));
        assert!(!button.was_decide_click_count());
        assert_eq!(button.click_count(), 0);
        assert_eq!(button.state(), ButtonState::NoChange);
        assert_eq!(button.last_change(), 0);
        assert_eq!(button.debounce_thresh(), 0);
        assert_eq!(button.hold_thresh(), 0);
        assert_eq!(button.update_msec(), 0);
        button.set_debounce_thresh(20);
        assert_eq!(button.try_set_debounce_thresh(20), Ok(()));
        button.set_hold_thresh(500);
        assert_eq!(button.try_set_hold_thresh(500), Ok(()));
        button.set_raw_state(1, false);
        assert_eq!(button.try_set_raw_state(1, false), Ok(()));
        button.set_state(2, ButtonState::Clicked);
        assert_eq!(button.try_set_state(2, ButtonState::Clicked), Ok(()));
        button.set_state(3, ButtonState::Other(99));
        assert_eq!(
            button.try_set_state(3, ButtonState::Other(99)),
            Err(Error::InvalidValue("button state"))
        );
    }

    #[test]
    fn power_management_api_uses_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");

        assert!(!m5.power.begin());
        assert_eq!(m5.power.try_begin(), Err(Error::Unavailable("power")));
        assert_eq!(m5.power.battery_level(), Some(100));
        assert_eq!(m5.power.battery_voltage_mv(), Some(4200));
        assert_eq!(m5.power.battery_current_ma(), 0);
        assert_eq!(m5.power.charging_state(), ChargingState::Discharging);
        assert_eq!(ChargingState::from_raw(1), ChargingState::Charging);
        assert_eq!(ChargingState::from_raw(7), ChargingState::Other(7));
        assert_eq!(ChargingState::Charging.raw(), 1);
        assert_eq!(ChargingState::Other(7).raw(), 7);
        assert!(ChargingState::Charging.is_known());
        assert!(!ChargingState::Other(7).is_known());
        assert!(ChargingState::Discharging.is_discharging());
        assert!(ChargingState::Charging.is_charging());
        assert!(ChargingState::Unknown.is_unknown());
        assert!(!m5.power.is_charging());
        assert!(!m5.power.ext_output());
        assert!(!m5.power.usb_output());
        assert_eq!(m5.power.key_state(), PowerKeyState::None);
        assert_eq!(PowerKeyState::from_raw(2), PowerKeyState::ShortClicked);
        assert_eq!(PowerKeyState::from_raw(9), PowerKeyState::Other(9));
        assert_eq!(PowerKeyState::ShortClicked.raw(), 2);
        assert_eq!(PowerKeyState::Other(9).raw(), 9);
        assert!(PowerKeyState::ShortClicked.is_known());
        assert!(!PowerKeyState::Other(9).is_known());
        assert!(PowerKeyState::None.is_none());
        assert!(PowerKeyState::LongPressed.is_long_pressed());
        assert!(PowerKeyState::ShortClicked.is_short_clicked());
        assert!(PowerKeyState::Both.is_long_pressed());
        assert!(PowerKeyState::Both.is_short_clicked());
        assert!(PowerKeyState::Both.has_event());
        assert!(!PowerKeyState::None.has_event());
        assert_eq!(m5.power.pmic_type(), PmicType::Unknown);
        assert_eq!(PmicType::from_raw(4), PmicType::Axp2101);
        assert_eq!(PmicType::from_raw(42), PmicType::Other(42));
        assert_eq!(PmicType::Axp2101.raw(), 4);
        assert_eq!(PmicType::Other(42).raw(), 42);
        assert!(PmicType::Axp2101.is_known());
        assert!(!PmicType::Other(42).is_known());
        assert!(PmicType::Unknown.is_unknown());
        assert!(PmicType::Axp192.is_axp());
        assert!(PmicType::Axp2101.is_axp());
        assert!(PmicType::Axp2101.supports_axp2101_irqs());
        assert!(!PmicType::Axp192.supports_axp2101_irqs());
        let status = m5.power.status();
        assert_eq!(
            status,
            PowerStatus {
                battery_level: Some(100),
                battery_voltage_mv: Some(4200),
                battery_current_ma: 0,
                charging_state: ChargingState::Discharging,
                is_charging: false,
                ext_output: false,
                usb_output: false,
                key_state: PowerKeyState::None,
                pmic_type: PmicType::Unknown,
            }
        );
        assert_eq!(status.battery_percent(), Some(100));
        assert_eq!(status.battery_fraction(), Some(1.0));
        assert_eq!(status.battery_voltage(), Some(4200));
        assert!(status.has_battery_level());
        assert!(status.has_battery_voltage());
        assert!(status.has_battery());
        assert!(status.is_running_on_battery());
        assert!(!status.has_power_key_event());
        let axp = m5.power.axp2101();
        let mask = Axp2101IrqMask::from_raw(0b1010);
        let status = Axp2101IrqStatus::from_raw(0b1000);
        let mut assigned_mask = Axp2101IrqMask::from_raw(0b0010);
        assert_eq!(Axp2101::IRQ_ALL, Axp2101IrqMask::ALL);
        assert_eq!(Axp2101IrqMask::NONE.raw(), 0);
        assert!(Axp2101IrqMask::NONE.is_empty());
        assert_eq!(mask.raw(), 0b1010);
        assert!(mask.contains(Axp2101IrqMask::from_raw(0b0010)));
        assert_eq!(mask.union(Axp2101IrqMask::from_raw(0b0100)).raw(), 0b1110);
        assert_eq!((mask | Axp2101IrqMask::from_raw(0b0100)).raw(), 0b1110);
        assigned_mask |= Axp2101IrqMask::from_raw(0b1000);
        assert_eq!(assigned_mask, mask);
        assert_eq!((mask & Axp2101IrqMask::from_raw(0b1000)).raw(), 0b1000);
        assigned_mask &= Axp2101IrqMask::from_raw(0b0010);
        assert_eq!(assigned_mask.raw(), 0b0010);
        assert_eq!(mask.without(Axp2101IrqMask::from_raw(0b1000)).raw(), 0b0010);
        assert_eq!((mask - Axp2101IrqMask::from_raw(0b1000)).raw(), 0b0010);
        assigned_mask = mask;
        assigned_mask -= Axp2101IrqMask::from_raw(0b1000);
        assert_eq!(assigned_mask.raw(), 0b0010);
        assert_eq!((!Axp2101IrqMask::NONE).raw(), u64::MAX);
        assert_eq!(u64::from(mask), 0b1010);
        assert_eq!(Axp2101IrqMask::from(0b1010), mask);
        assert!(status.any());
        assert!(!status.is_empty());
        assert_eq!(status.raw(), 0b1000);
        assert!(status.contains(Axp2101IrqMask::from_raw(0b1000)));
        assert_eq!(status.as_mask(), Axp2101IrqMask::from_raw(0b1000));
        assert!(!axp.disable_irq(mask));
        assert_eq!(
            axp.try_disable_irq(mask),
            Err(Error::Unavailable("axp2101"))
        );
        assert!(!axp.enable_irq(mask));
        assert_eq!(axp.try_enable_irq(mask), Err(Error::Unavailable("axp2101")));
        assert!(!axp.clear_irq_statuses());
        assert_eq!(
            axp.try_clear_irq_statuses(),
            Err(Error::Unavailable("axp2101"))
        );
        assert_eq!(axp.irq_statuses(), Axp2101IrqStatus::default());
        assert!(!axp.irq_statuses().battery_charger_under_temperature());
        assert!(!axp.irq_statuses().battery_charger_over_temperature());
        assert!(!axp.irq_statuses().vbus_insert());
        assert!(!axp.irq_statuses().vbus_remove());
        m5.power.set_ext_output(true);
        assert_eq!(m5.power.try_set_ext_output(true), Ok(()));
        m5.power.set_usb_output(true);
        assert_eq!(m5.power.try_set_usb_output(true), Ok(()));
        m5.power.set_led(8);
        assert_eq!(m5.power.try_set_led(8), Ok(()));
        m5.power.set_battery_charge(true);
        assert_eq!(m5.power.try_set_battery_charge(true), Ok(()));
        m5.power.set_charge_current(200);
        assert_eq!(m5.power.try_set_charge_current(200), Ok(()));
        assert_eq!(
            m5.power.try_set_charge_current(0),
            Err(Error::InvalidValue("milliamps"))
        );
        m5.power.set_charge_voltage(4100);
        assert_eq!(m5.power.try_set_charge_voltage(4100), Ok(()));
        assert_eq!(
            m5.power.try_set_charge_voltage(0),
            Err(Error::InvalidValue("millivolts"))
        );
        m5.power.set_vibration(0);
        assert_eq!(m5.power.try_set_vibration(0), Ok(()));
        m5.power.light_sleep(0, false);
        assert_eq!(m5.power.try_light_sleep(1, false), Ok(()));
        assert_eq!(
            m5.power.try_light_sleep(0, false),
            Err(Error::InvalidValue("sleep microseconds"))
        );
        m5.power.deep_sleep(0, false);
        assert_eq!(m5.power.try_deep_sleep(1, false), Ok(()));
        assert_eq!(
            m5.power.try_deep_sleep(0, false),
            Err(Error::InvalidValue("sleep microseconds"))
        );
        m5.power.timer_sleep(0);
        assert_eq!(m5.power.try_timer_sleep(1), Ok(()));
        assert_eq!(
            m5.power.try_timer_sleep(0),
            Err(Error::InvalidValue("sleep seconds"))
        );
        assert_eq!(
            m5.power.try_timer_sleep(-1),
            Err(Error::InvalidValue("sleep seconds"))
        );
        m5.power.power_off();
        assert_eq!(m5.power.try_power_off(), Ok(()));
    }

    #[test]
    fn pin_lookup_uses_typed_names_and_host_stubs() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        let cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");

        assert_eq!(PinName::PORT_A_SDA.raw(), Some(3));
        assert_eq!(PinName::SD_SPI_CIPO.raw(), Some(14));
        assert_eq!(PinName::MBusPin(1).raw(), Some(18));
        assert_eq!(PinName::MBusPin(30).raw(), Some(47));
        assert_eq!(PinName::MBusPin(31).raw(), None);
        assert_eq!(m5.pin(PinName::RgbLed), None);
        assert_eq!(cardputer.pin(PinName::PORT_A_SCL), None);
    }

    #[test]
    fn board_detection_uses_typed_kind_and_host_stubs() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        let cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");

        assert_eq!(m5.board(), BoardKind::Unknown);
        assert_eq!(cardputer.board(), BoardKind::Unknown);
        assert_eq!(BoardKind::from_raw(14), BoardKind::M5Cardputer);
        assert_eq!(BoardKind::M5Cardputer.raw(), 14);
        assert!(BoardKind::M5Cardputer.is_cardputer());
        assert!(!BoardKind::M5AtomS3.is_cardputer());
        assert_eq!(BoardKind::from_raw(999), BoardKind::Raw(999));
        assert_eq!(
            DisplayKind::AtomDisplay.raw(),
            BoardKind::M5AtomDisplay.raw()
        );
        assert_eq!(
            DisplayKind::ModuleDisplay.raw(),
            BoardKind::M5ModuleDisplay.raw()
        );
    }

    #[test]
    fn startup_config_can_be_passed_to_begin() {
        let speaker = ExternalSpeakerConfig::HAT_SPK | ExternalSpeakerConfig::ATOMIC_ECHO;
        let mut speaker_assigned = ExternalSpeakerConfig::HAT_SPK;
        speaker_assigned |= ExternalSpeakerConfig::ATOMIC_ECHO;
        let mut speaker_intersection = speaker;
        speaker_intersection &= ExternalSpeakerConfig::HAT_SPK;
        let mut speaker_without_echo = speaker;
        speaker_without_echo -= ExternalSpeakerConfig::ATOMIC_ECHO;
        let display = ExternalDisplayConfig::NONE
            | ExternalDisplayConfig::ATOM_DISPLAY
            | ExternalDisplayConfig::UNIT_LCD;
        let mut display_assigned = ExternalDisplayConfig::NONE;
        display_assigned |= ExternalDisplayConfig::ATOM_DISPLAY;
        display_assigned |= ExternalDisplayConfig::UNIT_LCD;
        let mut display_intersection = display;
        display_intersection &= ExternalDisplayConfig::UNIT_LCD;
        let mut display_without_atom = display;
        display_without_atom -= ExternalDisplayConfig::ATOM_DISPLAY;
        let config = M5Config::default()
            .with_serial_baudrate(0)
            .with_clear_display(false)
            .with_output_power(false)
            .with_pmic_button(false)
            .with_internal_imu(false)
            .with_internal_rtc(false)
            .with_internal_mic(false)
            .with_internal_speaker(false)
            .with_external_imu(true)
            .with_external_rtc(true)
            .with_disable_rtc_irq(false)
            .with_led_brightness(32)
            .with_external_speaker(speaker)
            .with_external_display(display);
        let raw = m5unified_sys::m5u_config_t::from(config);

        assert_eq!(raw.serial_baudrate, 0);
        assert!(!raw.clear_display);
        assert!(!raw.output_power);
        assert!(!raw.pmic_button);
        assert!(!raw.internal_imu);
        assert!(!raw.internal_rtc);
        assert!(!raw.internal_mic);
        assert!(!raw.internal_spk);
        assert!(raw.external_imu);
        assert!(raw.external_rtc);
        assert!(!raw.disable_rtc_irq);
        assert_eq!(raw.led_brightness, 32);
        assert!(speaker.contains(ExternalSpeakerConfig::HAT_SPK));
        assert!(speaker.contains(ExternalSpeakerConfig::ATOMIC_ECHO));
        assert!(speaker.any());
        assert!(!speaker.is_empty());
        assert!(ExternalSpeakerConfig::NONE.is_empty());
        assert_eq!(ExternalSpeakerConfig::ALL.without(speaker).bits(), 0x001b);
        assert_eq!(speaker_assigned, speaker);
        assert_eq!(speaker_intersection, ExternalSpeakerConfig::HAT_SPK);
        assert_eq!(speaker_without_echo, ExternalSpeakerConfig::HAT_SPK);
        assert_eq!(
            (speaker - ExternalSpeakerConfig::ATOMIC_ECHO).bits(),
            ExternalSpeakerConfig::HAT_SPK.bits()
        );
        assert_eq!(
            (speaker & ExternalSpeakerConfig::HAT_SPK).bits(),
            ExternalSpeakerConfig::HAT_SPK.bits()
        );
        assert_eq!(raw.external_speaker_value, (1 << 2) | (1 << 5));
        assert!(display.contains(ExternalDisplayConfig::ATOM_DISPLAY));
        assert!(display.contains(ExternalDisplayConfig::UNIT_LCD));
        assert!(display.any());
        assert!(!display.is_empty());
        assert!(ExternalDisplayConfig::NONE.is_empty());
        assert_eq!(display_assigned, display);
        assert_eq!(display_intersection, ExternalDisplayConfig::UNIT_LCD);
        assert_eq!(display_without_atom, ExternalDisplayConfig::UNIT_LCD);
        assert_eq!(
            (display - ExternalDisplayConfig::ATOM_DISPLAY).bits(),
            ExternalDisplayConfig::UNIT_LCD.bits()
        );
        assert_eq!(
            (display & ExternalDisplayConfig::UNIT_LCD).bits(),
            ExternalDisplayConfig::UNIT_LCD.bits()
        );
        assert_eq!(raw.external_display_value, (1 << 1) | (1 << 4));
        assert_eq!(M5Config::default().external_display.bits(), 0xffff);
        assert_eq!(M5Config::default().external_speaker.bits(), 0);

        let _m5 = M5Unified::begin_with_config(config).expect("host stub begin should succeed");
        let _cardputer =
            Cardputer::begin_with_config(config).expect("host stub cardputer begin should succeed");
        let _cardputer_no_keyboard = Cardputer::begin_with_config_and_keyboard(config, false)
            .expect("host stub cardputer begin should succeed");
    }

    #[test]
    fn timing_helpers_are_available_on_host_stubs() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        let cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");

        assert_eq!(m5.millis(), 0);
        assert_eq!(m5.micros(), 0);
        assert_eq!(m5.update_msec(), 0);
        assert_eq!(cardputer.millis(), 0);
        assert_eq!(cardputer.micros(), 0);
        assert_eq!(cardputer.update_msec(), 0);
        assert_eq!(millis(), 0);
        assert_eq!(micros(), 0);
        assert_eq!(update_msec(), 0);
    }

    #[test]
    fn rtc_api_uses_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        let date = RtcDate::new(2026, 5, 21, 4);
        let time = RtcTime::new(12, 34, 56);
        let datetime = DateTime::new(2026, 5, 21, 12, 34, 56);
        let typed_date = RtcDate::new_with_weekday(2026, 5, 21, RtcWeekday::Thursday);

        assert!(m5.rtc.is_enabled());
        assert_eq!(m5.rtc.try_begin(), Ok(()));
        assert!(m5.rtc.begin());
        assert!(!m5.rtc.volt_low());
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2100));
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2025, 2), 28);
        assert_eq!(days_in_month(2026, 13), 0);
        assert_eq!(RtcWeekday::from_raw(0), Some(RtcWeekday::Sunday));
        assert_eq!(RtcWeekday::from_raw(6), Some(RtcWeekday::Saturday));
        assert_eq!(RtcWeekday::from_raw(7), None);
        assert_eq!(RtcWeekday::Thursday.raw(), 4);
        assert!(date.is_valid());
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 5);
        assert_eq!(date.day(), 21);
        assert_eq!(date.raw_weekday(), 4);
        assert_eq!(date.ymd(), (2026, 5, 21));
        assert_eq!(typed_date, date);
        assert_eq!(date.weekday(), Some(RtcWeekday::Thursday));
        assert_eq!(RtcDate::new(2026, 5, 21, 99).weekday(), None);
        assert_eq!(
            date.with_weekday(RtcWeekday::Friday),
            RtcDate::new(2026, 5, 21, 5)
        );
        assert!(time.is_valid());
        assert_eq!(time.hour(), 12);
        assert_eq!(time.minute(), 34);
        assert_eq!(time.second(), 56);
        assert_eq!(time.hms(), (12, 34, 56));
        assert!(datetime.is_valid());
        assert_eq!(datetime.year(), 2026);
        assert_eq!(datetime.month(), 5);
        assert_eq!(datetime.day(), 21);
        assert_eq!(datetime.hour(), 12);
        assert_eq!(datetime.minute(), 34);
        assert_eq!(datetime.second(), 56);
        assert_eq!(datetime.ymd(), (2026, 5, 21));
        assert_eq!(datetime.hms(), (12, 34, 56));
        assert_eq!(date.with_time(time), datetime);
        assert_eq!(datetime.date(4), date);
        assert_eq!(datetime.date_with_weekday(RtcWeekday::Thursday), date);
        assert_eq!(datetime.time(), time);
        assert!(!RtcDate::new(2026, 2, 29, 0).is_valid());
        assert!(!RtcDate::new(2024, 2, 29, 7).is_valid());
        assert!(!RtcTime::new(24, 0, 0).is_valid());
        assert!(!DateTime::new(2026, 5, 21, 12, 60, 0).is_valid());
        assert_eq!(
            m5.rtc.date(),
            Some(RtcDate {
                year: 2026,
                month: 1,
                day: 1,
                weekday: 4,
            })
        );
        assert_eq!(
            m5.rtc.try_date(),
            Ok(RtcDate {
                year: 2026,
                month: 1,
                day: 1,
                weekday: 4,
            })
        );
        assert_eq!(
            m5.rtc.time(),
            Some(RtcTime {
                hour: 0,
                minute: 0,
                second: 0,
            })
        );
        assert_eq!(
            m5.rtc.try_time(),
            Ok(RtcTime {
                hour: 0,
                minute: 0,
                second: 0,
            })
        );
        assert_eq!(
            m5.rtc.get_datetime(),
            Some(DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
            })
        );
        assert_eq!(
            m5.rtc.try_get_datetime(),
            Ok(DateTime {
                year: 2026,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
            })
        );
        assert!(m5.rtc.set_date(date));
        assert_eq!(m5.rtc.try_set_date(date), Ok(()));
        assert_eq!(
            m5.rtc.try_set_date(RtcDate::new(2026, 2, 29, 0)),
            Err(Error::InvalidValue("rtc date"))
        );
        assert!(m5.rtc.set_time(time));
        assert_eq!(m5.rtc.try_set_time(time), Ok(()));
        assert_eq!(
            m5.rtc.try_set_time(RtcTime::new(24, 0, 0)),
            Err(Error::InvalidValue("rtc time"))
        );
        assert!(m5.rtc.set_datetime(datetime));
        assert_eq!(m5.rtc.try_set_datetime(datetime), Ok(()));
        assert_eq!(
            m5.rtc
                .try_set_datetime(DateTime::new(2026, 5, 21, 12, 60, 0)),
            Err(Error::InvalidValue("rtc datetime"))
        );
        m5.rtc.set_system_time_from_rtc();
        assert_eq!(m5.rtc.try_set_system_time_from_rtc(), Ok(()));
        assert!(!m5.rtc.set_alarm_irq_after(60));
        assert_eq!(
            m5.rtc.try_set_alarm_irq_after(60),
            Err(Error::Unavailable("rtc alarm"))
        );
        assert_eq!(
            m5.rtc.try_set_alarm_irq_after(-1),
            Err(Error::InvalidValue("rtc alarm seconds"))
        );
        assert!(!m5.rtc.irq_status());
        m5.rtc.clear_irq();
        assert_eq!(m5.rtc.try_clear_irq(), Ok(()));
        m5.rtc.disable_irq();
        assert_eq!(m5.rtc.try_disable_irq(), Ok(()));
    }

    #[test]
    fn touch_button_height_helpers_use_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");

        m5.touch.begin();
        m5.touch.update(0);
        assert!(!m5.touch.is_enabled());
        assert_eq!(m5.touch.count(), 0);
        assert!(!m5.touch.is_pressed());
        assert!(m5.touch.points().is_empty());
        assert_eq!(m5.touch.point(0), None);
        assert_eq!(
            m5.touch.try_point(0),
            Err(Error::Unavailable("touch point"))
        );
        assert_eq!(m5.touch.point(usize::MAX), None);
        assert_eq!(
            m5.touch.try_point(usize::MAX),
            Err(Error::InvalidValue("touch index"))
        );
        assert_eq!(m5.touch.primary_point(), None);
        assert_eq!(
            m5.touch.try_primary_point(),
            Err(Error::Unavailable("touch point"))
        );
        assert_eq!(m5.touch.raw_point(0), None);
        assert_eq!(
            m5.touch.try_raw_point(0),
            Err(Error::Unavailable("touch raw point"))
        );
        assert!(m5.touch.raw_points().is_empty());
        assert_eq!(m5.touch.primary_raw_point(), None);
        assert_eq!(
            m5.touch.try_primary_raw_point(),
            Err(Error::Unavailable("touch raw point"))
        );
        assert_eq!(m5.touch.raw_point(usize::MAX), None);
        assert_eq!(
            m5.touch.try_raw_point(usize::MAX),
            Err(Error::InvalidValue("touch index"))
        );
        assert_eq!(m5.touch.detail(0), None);
        assert_eq!(
            m5.touch.try_detail(0),
            Err(Error::Unavailable("touch detail"))
        );
        assert_eq!(m5.touch.detail(usize::MAX), None);
        assert_eq!(
            m5.touch.try_detail(usize::MAX),
            Err(Error::InvalidValue("touch index"))
        );
        assert!(m5.touch.details().is_empty());
        assert_eq!(m5.touch.primary_detail(), None);
        assert_eq!(
            m5.touch.try_primary_detail(),
            Err(Error::Unavailable("touch detail"))
        );
        m5.touch.set_hold_thresh(500);
        assert_eq!(m5.touch.try_set_hold_thresh(500), Ok(()));
        assert_eq!(
            m5.touch.try_set_hold_thresh(0),
            Err(Error::InvalidValue("touch hold threshold"))
        );
        m5.touch.set_flick_thresh(8);
        assert_eq!(m5.touch.try_set_flick_thresh(8), Ok(()));
        assert_eq!(
            m5.touch.try_set_flick_thresh(0),
            Err(Error::InvalidValue("touch flick threshold"))
        );
        m5.touch.end();

        assert_eq!(TouchState::from_raw(0), TouchState::None);
        assert_eq!(TouchState::from_raw(15), TouchState::DragBegin);
        assert_eq!(TouchState::from_raw(99), TouchState::Other(99));
        assert_eq!(TouchState::None.raw(), 0);
        assert_eq!(TouchState::DragBegin.raw(), 15);
        assert_eq!(TouchState::Other(99).raw(), 99);
        assert!(TouchState::None.is_none());
        assert!(TouchState::TouchBegin.is_touch());
        assert!(TouchState::TouchBegin.is_begin());
        assert!(TouchState::TouchEnd.is_end());
        assert!(TouchState::Hold.is_hold());
        assert!(TouchState::Flick.is_flick());
        assert!(TouchState::Drag.is_drag());
        assert!(!TouchState::Other(99).is_drag());
        let point = TouchPoint::new(12, 34).with_size(5).with_id(2);
        assert_eq!(point.position(), Point::new(12, 34));
        assert_eq!(point.size(), 5);
        assert_eq!(point.id(), 2);
        let detail = TouchDetail {
            x: 3,
            y: 7,
            prev_x: 1,
            prev_y: 4,
            base_x: -1,
            base_y: 2,
            ..TouchDetail::default()
        };
        assert_eq!(detail.point(), TouchPoint::new(3, 7));
        assert_eq!(detail.position(), Point::new(3, 7));
        assert_eq!(detail.previous_position(), Point::new(1, 4));
        assert_eq!(detail.base_position(), Point::new(-1, 2));
        assert_eq!(detail.delta(), (2, 3));
        assert_eq!(detail.delta_point(), Point::new(2, 3));
        assert_eq!(detail.distance(), (4, 5));
        assert_eq!(detail.distance_point(), Point::new(4, 5));

        m5.set_touch_button_height(48);
        m5.set_touch_button_height_by_ratio(4);
        cardputer.set_touch_button_height(32);
        cardputer.set_touch_button_height_by_ratio(8);
        set_touch_button_height(16);
        set_touch_button_height_by_ratio(2);

        assert_eq!(m5.touch_button_height(), 0);
        assert_eq!(cardputer.touch_button_height(), 0);
        assert_eq!(touch_button_height(), 0);
    }

    #[test]
    fn log_configuration_uses_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");

        assert!(m5.set_log_display(0));
        assert!(cardputer.set_log_display(0));
        assert_eq!(m5.log.print("plain log"), Ok(()));
        assert_eq!(m5.log.println("plain log line"), Ok(()));
        assert_eq!(m5.log.log(LogLevel::Info, "leveled log"), Ok(()));
        assert_eq!(LogLevel::from_raw(0), LogLevel::None);
        assert_eq!(LogLevel::from_raw(3), LogLevel::Info);
        assert_eq!(LogLevel::from_raw(4), LogLevel::Debug);
        assert_eq!(LogLevel::from_raw(99), LogLevel::Info);
        assert_eq!(LogLevel::Verbose.raw(), 5);
        assert_eq!(LogLevel::Debug.severity(), 4);
        assert!(!LogLevel::None.is_enabled());
        assert!(LogLevel::Info.is_enabled());
        assert!(LogLevel::Debug.includes(LogLevel::Error));
        assert!(LogLevel::Debug.includes(LogLevel::Info));
        assert!(!LogLevel::Info.includes(LogLevel::Debug));
        assert!(LogLevel::Verbose.is_at_least(LogLevel::Debug));
        assert!(LogLevel::Verbose.is_verbose());
        assert!(!LogLevel::Debug.is_verbose());
        assert_eq!(LogTarget::from_raw(0), Some(LogTarget::Serial));
        assert_eq!(LogTarget::from_raw(2), Some(LogTarget::Callback));
        assert_eq!(LogTarget::from_raw(3), None);
        assert_eq!(LogTarget::Display.raw(), 1);
        assert_eq!(
            LogTarget::ALL,
            [LogTarget::Serial, LogTarget::Display, LogTarget::Callback]
        );
        assert!(LogTarget::Serial.is_serial());
        assert!(LogTarget::Display.is_display());
        assert!(LogTarget::Callback.is_callback());
        assert!(LogTarget::Serial.supports_color());
        assert!(LogTarget::Display.supports_color());
        assert!(!LogTarget::Callback.supports_color());
        assert_eq!(m5.log.error("error log"), Ok(()));
        assert_eq!(m5.log.warn("warn log"), Ok(()));
        assert_eq!(m5.log.info("info log"), Ok(()));
        assert_eq!(m5.log.debug("debug log"), Ok(()));
        assert_eq!(m5.log.verbose("verbose log"), Ok(()));
        assert_eq!(m5.log.print("bad\0log"), Err(Error::InvalidString));
        assert_eq!(m5.log.println("bad\0log"), Err(Error::InvalidString));
        assert_eq!(
            m5.log.log(LogLevel::Info, "bad\0log"),
            Err(Error::InvalidString)
        );
        assert_eq!(m5.log.error("bad\0log"), Err(Error::InvalidString));
        m5.log.set_level(LogTarget::Serial, LogLevel::Debug);
        m5.log.set_enable_color(LogTarget::Display, true);
        assert_eq!(m5.log.level(LogTarget::Serial), LogLevel::Info);
        assert!(!m5.log.enable_color(LogTarget::Display));
        assert_eq!(m5.log.set_suffix(LogTarget::Serial, "\n"), Ok(()));
        assert_eq!(
            m5.log.set_suffix(LogTarget::Serial, "bad\0suffix"),
            Err(Error::InvalidString)
        );
        assert_eq!(m5.try_set_log_display(0), Ok(()));
        assert_eq!(cardputer.try_set_log_display(0), Ok(()));
        assert_eq!(
            m5.try_set_log_display(1),
            Err(Error::InvalidValue("display index"))
        );
    }

    #[test]
    fn primary_display_selection_uses_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");

        assert!(m5.set_primary_display(0));
        assert_eq!(m5.try_set_primary_display(0), Ok(()));
        assert!(!m5.set_primary_display(1));
        assert_eq!(
            m5.try_set_primary_display(1),
            Err(Error::Unavailable("display"))
        );
        assert!(!m5.set_primary_display_kind(DisplayKind::AtomDisplay));
        assert_eq!(
            m5.try_set_primary_display_kind(DisplayKind::AtomDisplay),
            Err(Error::Unavailable("display kind"))
        );
        assert_eq!(m5.display_count(), 1);
        let mut display = m5.display(0).expect("host stub display 0 should exist");
        assert_eq!(display.index(), 0);
        assert_eq!(m5.try_display(0).map(|display| display.index()), Ok(0));
        assert_eq!(m5.try_display(1), Err(Error::InvalidValue("display index")));
        assert!(m5.display_by_kind(DisplayKind::ModuleDisplay).is_none());
        assert_eq!(
            m5.try_display_by_kind(DisplayKind::ModuleDisplay),
            Err(Error::Unavailable("display kind"))
        );
        assert_eq!(display.width(), 320);
        assert_eq!(display.height(), 240);
        display.set_rotation(1);
        assert_eq!(display.rotation(), 0);
        display.set_brightness(128);
        assert_eq!(display.brightness(), 0);
        display.set_color_depth(16);
        assert_eq!(display.color_depth(), 16);
        assert!(!display.is_epd());
        display.set_epd_mode(EpdMode::Fast);
        display.set_epd_fastest();
        assert_eq!(display.epd_mode(), EpdMode::Unknown(0));
        let resolution = DisplayResolution::new(320, 240)
            .with_refresh_rate(60.0)
            .with_output_size(640, 480)
            .with_scale(2, 2)
            .with_pixel_clock(DisplayResolution::DEFAULT_PIXEL_CLOCK);
        assert_eq!(resolution.logical_size(), Size::new(320, 240));
        assert_eq!(resolution.refresh_rate(), 60.0);
        assert_eq!(resolution.output_size(), Some(Size::new(640, 480)));
        assert!(!resolution.uses_auto_output_size());
        assert_eq!(resolution.scale(), (2, 2));
        assert!(!resolution.uses_auto_scale());
        assert_eq!(
            resolution.pixel_clock(),
            DisplayResolution::DEFAULT_PIXEL_CLOCK
        );
        assert_eq!(resolution.validate(), Ok(()));
        assert_eq!(DisplayResolution::default().output_size(), None);
        assert!(DisplayResolution::default().uses_auto_output_size());
        assert!(DisplayResolution::default().uses_auto_scale());
        assert_eq!(
            DisplayResolution::new(320, 240)
                .with_refresh_rate(f32::NAN)
                .validate(),
            Err(Error::InvalidValue("display refresh rate"))
        );
        assert_eq!(
            DisplayResolution::new(320, 240)
                .with_pixel_clock(0)
                .validate(),
            Err(Error::InvalidValue("display pixel clock"))
        );
        assert!(!display.set_resolution(DisplayResolution::default()));
        assert_eq!(
            display.try_set_resolution(DisplayResolution::default()),
            Err(Error::Unavailable("display resolution"))
        );
        assert_eq!(
            display.try_set_resolution(DisplayResolution {
                refresh_rate: f32::INFINITY,
                ..DisplayResolution::default()
            }),
            Err(Error::InvalidValue("display refresh rate"))
        );
        assert_eq!(
            display.try_set_resolution(DisplayResolution {
                pixel_clock: 0,
                ..DisplayResolution::default()
            }),
            Err(Error::InvalidValue("display pixel clock"))
        );
        display.start_write();
        display.end_write();
        display.transaction(|display| display.draw_pixel(1, 1, colors::WHITE));
        display.transaction(|display| display.draw_point(Point::new(2, 2), colors::GREEN));
        display.display();
        assert!(!display.display_busy());
        display.wait_display();
        display.sleep();
        display.wakeup();
        display.power_save_on();
        display.power_save_off();
        display.power_save(true);
        display.invert_display(false);
        assert_eq!(display.cursor_x(), 0);
        assert_eq!(display.cursor_y(), 0);
        display.set_pivot(10.0, 11.0);
        assert_eq!(display.try_set_pivot(10.0, 11.0), Ok(()));
        assert_eq!(
            display.try_set_pivot(f32::NEG_INFINITY, 11.0),
            Err(Error::InvalidValue("display pivot"))
        );
        assert_eq!(
            display.try_set_pivot(10.0, f32::NAN),
            Err(Error::InvalidValue("display pivot"))
        );
        assert_eq!(display.pivot(), (0.0, 0.0));
        display.clear();
        display.fill_screen(colors::WHITE);
        assert!(!display.draw_bmp(&[], ImageDrawOptions::default()));
        assert_eq!(
            display.try_draw_bmp(&[], ImageDrawOptions::default()),
            Err(Error::InvalidValue("image data"))
        );
        assert!(!display.draw_jpg(&[0xff, 0xd8], ImageDrawOptions::default()));
        assert_eq!(
            display.try_draw_jpg(&[0xff, 0xd8], ImageDrawOptions::default()),
            Err(Error::Unavailable("display jpg"))
        );
        let image_options = ImageDrawOptions::new(2, 3).with_max_size(Size::new(8, 8));
        assert!(!display.draw_png(&[0x89, b'P', b'N', b'G'], image_options));
        assert_eq!(
            display.try_draw_png(&[0x89, b'P', b'N', b'G'], image_options),
            Err(Error::Unavailable("display png"))
        );
        assert!(!display.push_image_rgb565(
            Rect {
                x: 0,
                y: 0,
                w: 2,
                h: 2,
            },
            &[colors::RED]
        ));
        assert_eq!(
            display.try_push_image_rgb565(
                Rect {
                    x: 0,
                    y: 0,
                    w: 2,
                    h: 2,
                },
                &[colors::RED]
            ),
            Err(Error::InvalidValue("rgb565 image data"))
        );
        assert!(!display.push_image_rgb565(
            Rect {
                x: 0,
                y: 0,
                w: 1,
                h: 1,
            },
            &[colors::RED]
        ));
        assert_eq!(
            display.try_push_image_rgb565(
                Rect {
                    x: 0,
                    y: 0,
                    w: 1,
                    h: 1,
                },
                &[colors::RED]
            ),
            Err(Error::Unavailable("display rgb565 image"))
        );
        display.set_cursor(4, 5);
        display.set_text_size(2);
        display.set_text_color(colors::BLACK, colors::WHITE);
        display.set_text_datum(TextDatum::TopLeft);
        assert_eq!(display.text_datum(), Some(TextDatum::TopLeft));
        display.set_text_padding(12);
        assert_eq!(display.text_padding(), 0);
        assert_eq!(display.text_size_x(), 1);
        assert_eq!(display.text_size_y(), 1);
        assert_eq!(display.try_set_text_size(2), Ok(()));
        assert_eq!(
            display.try_set_text_size(0),
            Err(Error::InvalidValue("display text size"))
        );
        assert_eq!(
            display.try_set_text_size(-1),
            Err(Error::InvalidValue("display text size"))
        );
        assert_eq!(display.font_width(), 8);
        assert_eq!(display.font_height(), 16);
        assert!(display.set_font(DisplayFont::Font0));
        assert_eq!(display.try_set_font(DisplayFont::Font0), Ok(()));
        assert!(display.set_font(DisplayFont::DejaVu24));
        assert!(display.show_font(0));
        assert_eq!(display.try_show_font(0), Ok(()));
        display.unload_font();
        assert_eq!(display.font_width_for(DisplayFont::Font0), 8);
        assert_eq!(display.font_height_for(DisplayFont::Font0), 16);
        assert_eq!(display.base_color(), 0);
        display.set_base_color(colors::BLACK);
        display.set_color(colors::WHITE);
        display.set_rgb_color(255, 0, 0);
        display.set_raw_color(0x1234);
        assert_eq!(display.raw_color(), 0);
        assert_eq!(display.palette_count(), 0);
        display.set_swap_bytes(true);
        assert!(!display.swap_bytes());
        assert_eq!(display.color888(255, 0, 0), colors::RED);
        assert_eq!(display.swap565(255, 0, 0), 0x00f8);
        assert_eq!(display.swap888(1, 2, 3), 0x030201);
        display.set_text_wrap(true, false);
        assert_eq!(display.text_length("abc"), Ok(24));
        assert_eq!(display.text_width("abc"), Ok(24));
        assert_eq!(display.print("display 0"), Ok(()));
        assert_eq!(display.println("display 0"), Ok(()));
        assert_eq!(display.draw_center_string("abc", 160, 120), Ok(24));
        assert_eq!(display.draw_string("display 0", 6, 7), Ok(0));
        assert_eq!(display.draw_char('A', 0, 0), 8);
        assert_eq!(display.draw_number(42, 0, 0), 0);
        assert_eq!(display.draw_float(1.25, 2, 0, 0), 0);
        assert_eq!(
            display.text_width("bad\0display"),
            Err(Error::InvalidString)
        );
        assert_eq!(display.print("bad\0display"), Err(Error::InvalidString));
        assert_eq!(display.println("bad\0display"), Err(Error::InvalidString));
        assert_eq!(
            display.draw_center_string("bad\0display", 6, 7),
            Err(Error::InvalidString)
        );
        assert_eq!(
            display.draw_string("bad\0display", 6, 7),
            Err(Error::InvalidString)
        );
        display.draw_line(0, 0, 10, 10, colors::GREEN);
        display.draw_pixel(5, 5, colors::BLUE);
        display.draw_point(Point::new(6, 6), colors::GREEN);
        assert_eq!(display.read_pixel(5, 5), colors::BLACK);
        assert_eq!(display.read_point(Point::new(6, 6)), colors::BLACK);
        display.draw_fast_hline(0, 6, 10, colors::RED);
        assert_eq!(display.try_draw_fast_hline(0, 6, 10, colors::RED), Ok(()));
        assert_eq!(
            display.try_draw_fast_hline(0, 6, 0, colors::RED),
            Err(Error::InvalidValue("display length"))
        );
        display.draw_fast_vline(6, 0, 10, colors::GREEN);
        assert_eq!(display.try_draw_fast_vline(6, 0, 10, colors::GREEN), Ok(()));
        display.draw_rect(
            Rect {
                x: 1,
                y: 2,
                w: 3,
                h: 4,
            },
            colors::BLACK,
        );
        assert_eq!(
            display.try_draw_rect(
                Rect {
                    x: 1,
                    y: 2,
                    w: 3,
                    h: 4,
                },
                colors::BLACK
            ),
            Ok(())
        );
        display.fill_rect(
            Rect {
                x: 1,
                y: 2,
                w: 3,
                h: 4,
            },
            colors::RED,
        );
        assert_eq!(
            display.try_fill_rect(
                Rect {
                    x: 1,
                    y: 2,
                    w: -3,
                    h: 4,
                },
                colors::RED
            ),
            Err(Error::InvalidValue("display rect"))
        );
        display.fill_rect_alpha(
            Rect {
                x: 2,
                y: 3,
                w: 4,
                h: 5,
            },
            128,
            colors::BLUE,
        );
        assert_eq!(
            display.try_fill_rect_alpha(
                Rect {
                    x: 2,
                    y: 3,
                    w: 4,
                    h: 5,
                },
                128,
                colors::BLUE
            ),
            Ok(())
        );
        assert_eq!(
            display.try_fill_rect_alpha(
                Rect {
                    x: 2,
                    y: 3,
                    w: 4,
                    h: 0,
                },
                128,
                colors::BLUE
            ),
            Err(Error::InvalidValue("display rect"))
        );
        display.draw_round_rect(
            Rect {
                x: 2,
                y: 3,
                w: 12,
                h: 13,
            },
            3,
            colors::GREEN,
        );
        assert_eq!(
            display.try_draw_round_rect(
                Rect {
                    x: 2,
                    y: 3,
                    w: 12,
                    h: 13,
                },
                3,
                colors::GREEN
            ),
            Ok(())
        );
        assert_eq!(
            display.try_draw_round_rect(
                Rect {
                    x: 2,
                    y: 3,
                    w: -12,
                    h: 13,
                },
                3,
                colors::GREEN
            ),
            Err(Error::InvalidValue("display rect"))
        );
        display.fill_round_rect(
            Rect {
                x: 3,
                y: 4,
                w: 10,
                h: 11,
            },
            2,
            colors::RED,
        );
        assert_eq!(
            display.try_fill_round_rect(
                Rect {
                    x: 3,
                    y: 4,
                    w: 10,
                    h: 11,
                },
                2,
                colors::RED
            ),
            Ok(())
        );
        assert_eq!(
            display.try_fill_round_rect(
                Rect {
                    x: 3,
                    y: 4,
                    w: 10,
                    h: 11,
                },
                0,
                colors::RED
            ),
            Err(Error::InvalidValue("display radius"))
        );
        display.draw_circle(10, 10, 6, colors::WHITE);
        assert_eq!(display.try_draw_circle(10, 10, 6, colors::WHITE), Ok(()));
        assert_eq!(
            display.try_draw_circle(10, 10, 0, colors::WHITE),
            Err(Error::InvalidValue("display radius"))
        );
        display.fill_circle(10, 10, 5, colors::BLUE);
        assert_eq!(display.try_fill_circle(10, 10, 5, colors::BLUE), Ok(()));
        assert_eq!(
            display.try_fill_circle(10, 10, -1, colors::BLUE),
            Err(Error::InvalidValue("display radius"))
        );
        display.draw_ellipse(12, 12, 6, 4, colors::GREEN);
        assert_eq!(
            display.try_draw_ellipse(12, 12, 6, 4, colors::GREEN),
            Ok(())
        );
        assert_eq!(
            display.try_draw_ellipse(12, 12, -1, 4, colors::GREEN),
            Err(Error::InvalidValue("display radii"))
        );
        display.fill_ellipse(12, 12, 5, 3, colors::RED);
        assert_eq!(display.try_fill_ellipse(12, 12, 5, 3, colors::RED), Ok(()));
        assert_eq!(
            display.try_fill_ellipse(12, 12, 5, 0, colors::RED),
            Err(Error::InvalidValue("display radii"))
        );
        display.draw_arc(Point { x: 20, y: 20 }, 4, 8, 0.0, 180.0, colors::WHITE);
        assert_eq!(
            display.try_draw_arc(Point { x: 20, y: 20 }, 4, 8, 0.0, 180.0, colors::WHITE),
            Ok(())
        );
        assert_eq!(
            display.try_draw_arc(Point { x: 20, y: 20 }, 4, 0, 0.0, 180.0, colors::WHITE),
            Err(Error::InvalidValue("display arc radii"))
        );
        display.fill_arc(Point { x: 20, y: 20 }, 2, 5, 180.0, 360.0, colors::BLUE);
        assert_eq!(
            display.try_fill_arc(Point { x: 20, y: 20 }, 2, 5, 180.0, 360.0, colors::BLUE),
            Ok(())
        );
        assert_eq!(
            display.try_fill_arc(
                Point { x: 20, y: 20 },
                2,
                5,
                f32::NEG_INFINITY,
                360.0,
                colors::BLUE
            ),
            Err(Error::InvalidValue("display arc angle"))
        );
        display.draw_triangle(
            Point { x: 0, y: 0 },
            Point { x: 8, y: 0 },
            Point { x: 4, y: 8 },
            colors::GREEN,
        );
        display.fill_triangle(
            Point { x: 1, y: 1 },
            Point { x: 7, y: 1 },
            Point { x: 4, y: 7 },
            colors::RED,
        );
        display.progress_bar(
            Rect {
                x: 0,
                y: 20,
                w: 80,
                h: 8,
            },
            42,
        );
        assert_eq!(
            display.try_progress_bar(
                Rect {
                    x: 0,
                    y: 20,
                    w: 80,
                    h: 8,
                },
                42
            ),
            Ok(())
        );
        display.write_pixel(7, 7, colors::WHITE);
        display.write_point(Point::new(8, 8), colors::BLUE);
        display.write_fast_vline(8, 0, 10, colors::BLACK);
        assert_eq!(
            display.try_write_fast_vline(8, 0, 10, colors::BLACK),
            Ok(())
        );
        assert_eq!(
            display.try_write_fast_vline(8, 0, 0, colors::BLACK),
            Err(Error::InvalidValue("display length"))
        );
        display.set_addr_window(Rect {
            x: 0,
            y: 0,
            w: 12,
            h: 10,
        });
        assert_eq!(
            display.try_set_addr_window(Rect {
                x: 0,
                y: 0,
                w: 12,
                h: 10,
            }),
            Ok(())
        );
        display.set_window(0, 0, 11, 9);
        assert_eq!(display.try_set_window(0, 0, 11, 9), Ok(()));
        assert_eq!(
            display.try_set_window(12, 0, 11, 9),
            Err(Error::InvalidValue("display window"))
        );
        assert_eq!(
            display.try_set_window(0, 10, 11, 9),
            Err(Error::InvalidValue("display window"))
        );
        display.set_clip_rect(Rect {
            x: 2,
            y: 3,
            w: 20,
            h: 21,
        });
        assert_eq!(
            display.try_set_clip_rect(Rect {
                x: 2,
                y: 3,
                w: 20,
                h: 21,
            }),
            Ok(())
        );
        assert_eq!(
            display.clip_rect(),
            Rect {
                x: 0,
                y: 0,
                w: 320,
                h: 240,
            }
        );
        display.clear_clip_rect();
        display.scroll(0, 2);
        display.set_text_scroll(true);
        display.set_scroll_rect(
            Rect {
                x: 1,
                y: 2,
                w: 30,
                h: 31,
            },
            colors::BLACK,
        );
        assert_eq!(
            display.try_set_scroll_rect(
                Rect {
                    x: 1,
                    y: 2,
                    w: 30,
                    h: 0,
                },
                colors::BLACK
            ),
            Err(Error::InvalidValue("display rect"))
        );
        assert_eq!(display.scroll_rect(), Rect::default());
        display.clear_scroll_rect();
        assert!(m5.display(1).is_none());
        assert!(cardputer.set_primary_display(0));
        assert_eq!(cardputer.try_set_primary_display(0), Ok(()));
        assert_eq!(cardputer.display_count(), 1);
        assert!(cardputer.display(0).is_some());
        assert!(cardputer.display(1).is_none());
        assert_eq!(
            cardputer.try_display(0).map(|display| display.index()),
            Ok(0)
        );
        assert_eq!(
            cardputer.try_display(1),
            Err(Error::InvalidValue("display index"))
        );
        assert_eq!(cardputer.display_index(DisplayKind::ModuleDisplay), None);
        assert!(cardputer
            .display_by_kind(DisplayKind::ModuleDisplay)
            .is_none());
        assert_eq!(
            cardputer.try_display_by_kind(DisplayKind::ModuleDisplay),
            Err(Error::Unavailable("display kind"))
        );
        assert_eq!(
            cardputer.try_set_primary_display_kind(DisplayKind::ModuleDisplay),
            Err(Error::Unavailable("display kind"))
        );
    }

    #[test]
    fn cardputer_keyboard_state_is_available_on_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");
        let mut canvas = cardputer
            .canvas()
            .expect("host stub Cardputer canvas should be available");
        let state = cardputer
            .keyboard
            .state()
            .expect("host stub keyboard state should be available");

        assert_eq!(cardputer.button_a.id(), ButtonId::A);
        assert!(!canvas.create_sprite(Size { w: 80, h: 40 }));
        assert!(canvas.set_font(DisplayFont::FreeSerifBoldItalic18pt7b));
        assert_eq!(canvas.font_width(), 8);
        assert_eq!(canvas.font_height(), 16);
        assert!(canvas.show_font(0));
        assert_eq!(canvas.try_show_font(0), Ok(()));
        canvas.unload_font();
        canvas.set_text_datum(TextDatum::TopCenter);
        assert_eq!(canvas.text_datum(), Some(TextDatum::TopLeft));
        canvas.set_text_padding(4);
        assert_eq!(canvas.text_padding(), 0);
        assert_eq!(canvas.base_color(), colors::BLACK);
        canvas.set_base_color(colors::RED);
        canvas.set_color(colors::GREEN);
        canvas.set_rgb_color(4, 5, 6);
        canvas.set_raw_color(0x5678);
        assert_eq!(canvas.raw_color(), 0);
        canvas.set_swap_bytes(true);
        assert!(!canvas.swap_bytes());
        canvas.clear();
        canvas.fill_rect(
            Rect {
                x: 0,
                y: 0,
                w: 12,
                h: 8,
            },
            colors::BLACK,
        );
        assert_eq!(
            canvas.try_fill_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 12,
                    h: 8,
                },
                colors::BLACK
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_fill_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 8,
                },
                colors::BLACK
            ),
            Err(Error::InvalidValue("display rect"))
        );
        canvas.draw_circle(Point { x: 6, y: 6 }, 2, colors::GREEN);
        assert_eq!(
            canvas.try_draw_circle(Point { x: 6, y: 6 }, 2, colors::GREEN),
            Ok(())
        );
        assert_eq!(
            canvas.try_draw_circle(Point { x: 6, y: 6 }, 0, colors::GREEN),
            Err(Error::InvalidValue("display radius"))
        );
        canvas.draw_pixel(Point { x: 1, y: 1 }, colors::WHITE);
        assert_eq!(canvas.read_pixel(Point { x: 1, y: 1 }), colors::BLACK);
        canvas.draw_round_rect(
            Rect {
                x: 0,
                y: 0,
                w: 12,
                h: 8,
            },
            2,
            colors::GREEN,
        );
        assert_eq!(
            canvas.try_draw_round_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 12,
                    h: 8,
                },
                2,
                colors::GREEN
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_draw_round_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 12,
                    h: 8,
                },
                0,
                colors::GREEN
            ),
            Err(Error::InvalidValue("display radius"))
        );
        canvas.fill_triangle(
            Point { x: 2, y: 7 },
            Point { x: 6, y: 2 },
            Point { x: 10, y: 7 },
            colors::GREEN,
        );
        canvas.progress_bar(
            Rect {
                x: 0,
                y: 10,
                w: 20,
                h: 4,
            },
            50,
        );
        assert_eq!(
            canvas.try_progress_bar(
                Rect {
                    x: 0,
                    y: 10,
                    w: 20,
                    h: 4,
                },
                50
            ),
            Ok(())
        );
        assert_eq!(
            canvas.try_progress_bar(
                Rect {
                    x: 0,
                    y: 10,
                    w: -1,
                    h: 4,
                },
                50
            ),
            Err(Error::InvalidValue("display rect"))
        );
        canvas.write_pixel(Point { x: 0, y: 0 }, colors::WHITE);
        canvas.write_fast_vline(0, 0, 8, colors::WHITE);
        assert_eq!(canvas.try_write_fast_vline(0, 0, 8, colors::WHITE), Ok(()));
        assert_eq!(
            canvas.try_write_fast_vline(0, 0, -1, colors::WHITE),
            Err(Error::InvalidValue("display length"))
        );
        canvas.set_addr_window(Rect {
            x: 0,
            y: 0,
            w: 20,
            h: 10,
        });
        assert_eq!(
            canvas.try_set_addr_window(Rect {
                x: 0,
                y: 0,
                w: 20,
                h: 10,
            }),
            Ok(())
        );
        canvas.set_window(0, 0, 19, 9);
        assert_eq!(canvas.try_set_window(0, 0, 19, 9), Ok(()));
        assert_eq!(
            canvas.try_set_window(0, 10, 19, 9),
            Err(Error::InvalidValue("display window"))
        );
        canvas.set_clip_rect(Rect {
            x: 0,
            y: 0,
            w: 20,
            h: 10,
        });
        assert_eq!(
            canvas.try_set_clip_rect(Rect {
                x: 0,
                y: 0,
                w: 20,
                h: 10,
            }),
            Ok(())
        );
        assert_eq!(canvas.clip_rect(), Rect::default());
        canvas.clear_clip_rect();
        canvas.scroll(0, 1);
        canvas.set_scroll_rect(
            Rect {
                x: 0,
                y: 0,
                w: 20,
                h: 8,
            },
            colors::BLACK,
        );
        assert_eq!(
            canvas.try_set_scroll_rect(
                Rect {
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 8,
                },
                colors::BLACK
            ),
            Err(Error::InvalidValue("display rect"))
        );
        assert_eq!(canvas.scroll_rect(), Rect::default());
        canvas.clear_scroll_rect();
        assert_eq!(canvas.draw_char('C', 0, 0), 8);
        assert_eq!(canvas.draw_number(7, 0, 0), 0);
        assert_eq!(canvas.println("cardputer"), Ok(()));
        canvas.push_sprite(Point { x: 4, y: 4 });
        assert!(!cardputer.keyboard.is_pressed());
        assert_eq!(cardputer.keyboard.pressed_count(), 0);
        assert_eq!(cardputer.keyboard.key_at(0, 0), None);
        assert_eq!(
            cardputer.keyboard.try_key_at(0, 0),
            Err(Error::Unavailable("cardputer keyboard key"))
        );
        assert_eq!(cardputer.keyboard.key_value_at(0, 0), None);
        assert_eq!(
            cardputer.keyboard.try_key_value_at(0, 0),
            Err(Error::Unavailable("cardputer keyboard key"))
        );
        assert_eq!(
            cardputer.keyboard.try_state(),
            Ok(CardputerKeyboardState::default())
        );
        assert_eq!(cardputer.keyboard.word_lossy(), Some(String::new()));
        assert_eq!(cardputer.keyboard.try_word_lossy(), Ok(String::new()));
        assert_eq!(cardputer.keyboard.word_bytes(), Some(Vec::new()));
        assert_eq!(cardputer.keyboard.try_word_bytes(), Ok(Vec::new()));
        assert_eq!(cardputer.keyboard.has_word(), Some(false));
        assert_eq!(cardputer.keyboard.try_has_word(), Ok(false));
        assert_eq!(cardputer.keyboard.contains_word_byte(b'a'), Some(false));
        assert_eq!(cardputer.keyboard.try_contains_word_byte(b'a'), Ok(false));
        assert_eq!(cardputer.keyboard.hid_keys(), Some(Vec::new()));
        assert_eq!(cardputer.keyboard.try_hid_keys(), Ok(Vec::new()));
        assert_eq!(cardputer.keyboard.first_hid_key(), None);
        assert_eq!(cardputer.keyboard.try_first_hid_key(), Ok(None));
        assert_eq!(cardputer.keyboard.contains_hid_key(4), Some(false));
        assert_eq!(cardputer.keyboard.try_contains_hid_key(4), Ok(false));
        assert_eq!(cardputer.keyboard.modifier_keys(), Some(Vec::new()));
        assert_eq!(cardputer.keyboard.try_modifier_keys(), Ok(Vec::new()));
        assert_eq!(cardputer.keyboard.first_modifier_key(), None);
        assert_eq!(cardputer.keyboard.try_first_modifier_key(), Ok(None));
        assert_eq!(cardputer.keyboard.modifiers(), Some(0));
        assert_eq!(cardputer.keyboard.try_modifiers(), Ok(0));
        assert_eq!(cardputer.keyboard.contains_modifier_key(2), Some(false));
        assert_eq!(cardputer.keyboard.try_contains_modifier_key(2), Ok(false));
        assert_eq!(cardputer.keyboard.first_word_char(), None);
        assert_eq!(cardputer.keyboard.try_first_word_char(), Ok(None));
        assert!(!cardputer.keyboard.is_char_pressed('a'));
        assert_eq!(cardputer.keyboard.try_is_char_pressed('a'), Ok(false));
        assert!(!cardputer.keyboard.is_char_pressed('\u{00e9}'));
        assert_eq!(
            cardputer.keyboard.try_is_char_pressed('\u{00e9}'),
            Err(Error::InvalidValue("cardputer keyboard key"))
        );
        assert!(!cardputer.keyboard.is_char_pressed('\0'));
        assert_eq!(
            cardputer.keyboard.try_is_char_pressed('\0'),
            Err(Error::InvalidValue("cardputer keyboard key"))
        );
        assert_eq!(
            cardputer.keyboard.key_at(CardputerKeyboard::COLUMNS, 0),
            None
        );
        assert_eq!(cardputer.keyboard.key_at(0, CardputerKeyboard::ROWS), None);
        assert_eq!(
            cardputer.keyboard.try_key_at(CardputerKeyboard::COLUMNS, 0),
            Err(Error::InvalidValue("cardputer keyboard position"))
        );
        assert_eq!(
            cardputer.keyboard.try_key_at(0, CardputerKeyboard::ROWS),
            Err(Error::InvalidValue("cardputer keyboard position"))
        );
        assert_eq!(
            cardputer
                .keyboard
                .key_value_at(CardputerKeyboard::COLUMNS, 0),
            None
        );
        assert_eq!(
            cardputer
                .keyboard
                .try_key_value_at(CardputerKeyboard::COLUMNS, 0),
            Err(Error::InvalidValue("cardputer keyboard position"))
        );
        assert_eq!(
            CardputerKeyboardState::new(),
            CardputerKeyboardState::default()
        );
        assert_eq!(state, CardputerKeyboardState::default());
        assert!(state.is_empty());
        assert!(!state.has_word());
        assert!(!state.has_hid_keys());
        assert!(!state.has_modifier_keys());
        assert!(!state.has_modifiers());
        assert_eq!(state.word_utf8(), Ok(""));
        assert_eq!(state.word_lossy(), "");
        assert_eq!(state.first_word_byte(), None);
        assert_eq!(state.first_word_char(), None);
        assert_eq!(state.first_hid_key(), None);
        assert_eq!(state.first_modifier_key(), None);
        assert!(!state.contains_word_byte(b'a'));
        assert!(!state.contains_hid_key(4));
        assert!(!state.contains_modifier_key(2));

        let state = CardputerKeyboardState {
            shift: true,
            ..CardputerKeyboardState::new()
                .with_word(b"A".to_vec())
                .with_hid_keys([4])
                .with_modifier_keys([2])
                .with_modifiers(2)
        };
        assert!(!state.is_empty());
        assert!(state.has_word());
        assert!(state.has_hid_keys());
        assert!(state.has_modifier_keys());
        assert!(state.has_modifiers());
        assert_eq!(state.word_utf8(), Ok("A"));
        assert_eq!(state.word_lossy(), "A");
        assert_eq!(state.first_word_byte(), Some(b'A'));
        assert_eq!(state.first_word_char(), Some('A'));
        assert_eq!(state.first_hid_key(), Some(4));
        assert_eq!(state.first_modifier_key(), Some(2));
        assert!(state.contains_word_byte(b'A'));
        assert!(state.contains_hid_key(4));
        assert!(state.contains_modifier_key(2));

        let value = CardputerKeyValue::new(b'a', b'A');
        assert_eq!(value.bytes(), (b'a', b'A'));
        assert!(!value.is_empty());
        assert_eq!(value.first_char(), Some('a'));
        assert_eq!(value.second_char(), Some('A'));
        assert_eq!(value.chars(), (Some('a'), Some('A')));
        assert!(CardputerKeyValue::new(0, 0).is_empty());
        assert_eq!(CardputerKeyValue::new(0, 0xff).first_char(), None);
        assert_eq!(CardputerKeyValue::new(0, 0xff).second_char(), None);
    }

    #[test]
    fn cardputer_sd_reports_absent_on_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");

        assert!(!cardputer.sd.begin());
        assert_eq!(
            cardputer.sd.try_begin(),
            Err(Error::Unavailable("cardputer sd"))
        );
        assert_eq!(
            cardputer
                .sd
                .try_begin_with(CardputerSdPins::BUILTIN, CardputerSd::DEFAULT_FREQUENCY_HZ),
            Err(Error::Unavailable("cardputer sd"))
        );
        assert_eq!(
            cardputer.sd.try_begin_with(CardputerSdPins::BUILTIN, 0),
            Err(Error::InvalidValue("frequency"))
        );
        cardputer.sd.end();
        assert_eq!(cardputer.sd.card_type(), SdCardType::None);
        assert_eq!(cardputer.sd.info(), SdCardInfo::default());
    }

    #[test]
    fn cardputer_sd_file_helpers_use_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");
        let mut buffer = [0_u8; 16];

        assert_eq!(cardputer.sd.exists("/m5rs.txt"), Ok(false));
        assert_eq!(cardputer.sd.file_size("/m5rs.txt"), Ok(0));
        assert_eq!(cardputer.sd.is_directory("/m5rs.txt"), Ok(false));
        assert_eq!(cardputer.sd.list_dir("/", 4), Ok(Vec::new()));
        assert_eq!(cardputer.sd.list_dir("/", 0), Ok(Vec::new()));
        let file_entry = CardputerSdDirEntry::file("music.wav", 4096);
        let dir_entry = CardputerSdDirEntry::directory("samples");
        assert_eq!(file_entry.name(), "music.wav");
        assert!(file_entry.is_file());
        assert!(!file_entry.is_directory());
        assert_eq!(file_entry.size_bytes(), 4096);
        assert_eq!(file_entry.extension(), Some("wav"));
        assert_eq!(file_entry.file_stem(), "music");
        assert!(file_entry.has_extension("WAV"));
        assert!(dir_entry.is_directory());
        assert!(!dir_entry.is_file());
        assert_eq!(dir_entry.extension(), None);
        assert_eq!(dir_entry.file_stem(), "samples");
        assert!(!dir_entry.has_extension("wav"));
        assert_eq!(cardputer.sd.read_file("/m5rs.txt", &mut buffer), Ok(0));
        assert_eq!(
            cardputer.sd.try_read_file_exact("/m5rs.txt", &mut buffer),
            Err(Error::Unavailable("cardputer sd read"))
        );
        let mut empty: [u8; 0] = [];
        assert_eq!(
            cardputer.sd.try_read_file_exact("/m5rs.txt", &mut empty),
            Ok(())
        );
        assert_eq!(cardputer.sd.write_file("/m5rs.txt", b"hello"), Ok(0));
        assert_eq!(
            cardputer.sd.try_write_file_all("/m5rs.txt", b"hello"),
            Err(Error::Unavailable("cardputer sd write"))
        );
        assert_eq!(cardputer.sd.try_write_file_all("/m5rs.txt", b""), Ok(()));
        assert_eq!(cardputer.sd.append_file("/m5rs.txt", b"\n"), Ok(0));
        assert_eq!(
            cardputer.sd.try_append_file_all("/m5rs.txt", b"\n"),
            Err(Error::Unavailable("cardputer sd write"))
        );
        assert_eq!(cardputer.sd.try_append_file_all("/m5rs.txt", b""), Ok(()));
        assert_eq!(cardputer.sd.remove_file("/m5rs.txt"), Ok(false));
        assert_eq!(
            cardputer.sd.try_remove_file("/m5rs.txt"),
            Err(Error::Unavailable("cardputer sd remove"))
        );
        assert_eq!(cardputer.sd.mkdir("/m5rs"), Ok(false));
        assert_eq!(
            cardputer.sd.try_mkdir("/m5rs"),
            Err(Error::Unavailable("cardputer sd mkdir"))
        );
        assert_eq!(cardputer.sd.rmdir("/m5rs"), Ok(false));
        assert_eq!(
            cardputer.sd.try_rmdir("/m5rs"),
            Err(Error::Unavailable("cardputer sd rmdir"))
        );
        assert_eq!(
            cardputer.sd.rename("/m5rs.txt", "/m5rs-renamed.txt"),
            Ok(false)
        );
        assert_eq!(
            cardputer.sd.try_rename("/m5rs.txt", "/m5rs-renamed.txt"),
            Err(Error::Unavailable("cardputer sd rename"))
        );
        assert_eq!(cardputer.sd.exists("/bad\0path"), Err(Error::InvalidString));
        assert_eq!(
            cardputer.sd.file_size("/bad\0path"),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.is_directory("/bad\0path"),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.list_dir("/bad\0path", 4),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.try_read_file_exact("/bad\0path", &mut buffer),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.try_write_file_all("/bad\0path", b"x"),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.try_append_file_all("/bad\0path", b"x"),
            Err(Error::InvalidString)
        );
        assert_eq!(cardputer.sd.mkdir("/bad\0path"), Err(Error::InvalidString));
        assert_eq!(
            cardputer.sd.try_mkdir("/bad\0path"),
            Err(Error::InvalidString)
        );
        assert_eq!(cardputer.sd.rmdir("/bad\0path"), Err(Error::InvalidString));
        assert_eq!(
            cardputer.sd.try_rmdir("/bad\0path"),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.try_remove_file("/bad\0path"),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.rename("/bad\0path", "/m5rs-renamed.txt"),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.try_rename("/bad\0path", "/m5rs-renamed.txt"),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.rename("/m5rs.txt", "/bad\0path"),
            Err(Error::InvalidString)
        );
        assert_eq!(
            cardputer.sd.try_rename("/m5rs.txt", "/bad\0path"),
            Err(Error::InvalidString)
        );
    }

    #[test]
    fn cardputer_ir_reports_unavailable_on_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");
        let frame = NecFrame::new(0x1111, 0x34);
        let repeated = frame.with_repeats(2);

        assert_eq!(frame.address(), 0x1111);
        assert_eq!(frame.command(), 0x34);
        assert_eq!(frame.repeat_count(), 0);
        assert_eq!(frame.components(), (0x1111, 0x34, 0));
        assert!(!frame.has_repeats());
        assert_eq!(
            repeated,
            NecFrame {
                address: 0x1111,
                command: 0x34,
                repeats: 2
            }
        );
        assert_eq!(repeated.components(), (0x1111, 0x34, 2));
        assert!(repeated.has_repeats());
        assert_eq!(
            frame.with_address(0x2222).with_command(0x56),
            NecFrame::new(0x2222, 0x56)
        );
        assert!(!cardputer.ir.begin());
        assert_eq!(
            cardputer.ir.try_begin(),
            Err(Error::Unavailable("cardputer ir"))
        );
        assert_eq!(
            cardputer.ir.try_begin_on_pin(CardputerIr::BUILTIN_TX_PIN),
            Err(Error::Unavailable("cardputer ir"))
        );
        assert!(!cardputer.ir.send_nec(frame));
        assert_eq!(
            cardputer.ir.try_send_nec(frame),
            Err(Error::Unavailable("cardputer ir"))
        );
    }

    #[test]
    fn cardputer_grove_i2c_reports_absent_on_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");
        let address = I2cAddress::new(0x42).expect("valid 7-bit address");
        let mut buffer = [0_u8; 4];
        let config = I2cConfig::default().with_frequency_hz(I2cConfig::FAST_FREQUENCY_HZ);

        assert!(!cardputer.grove.i2c_begin());
        assert_eq!(
            cardputer.grove.i2c_try_begin(),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert!(!cardputer.grove.i2c_begin_config(config));
        assert_eq!(
            cardputer.grove.i2c_try_begin_config(config),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert_eq!(
            cardputer.grove.i2c_try_begin_config(I2cConfig::new(0)),
            Err(Error::InvalidValue("frequency"))
        );
        assert_eq!(
            cardputer.grove.i2c_try_begin_with(
                I2cPins::CARDPUTER_GROVE,
                CardputerGrove::DEFAULT_I2C_FREQUENCY_HZ
            ),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert_eq!(
            cardputer
                .grove
                .i2c_try_begin_with(I2cPins::CARDPUTER_GROVE, 0),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!cardputer
            .grove
            .i2c_begin_with_config(I2cPins::CARDPUTER_GROVE, config));
        assert_eq!(
            cardputer
                .grove
                .i2c_try_begin_with_config(I2cPins::CARDPUTER_GROVE, config),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert_eq!(
            cardputer
                .grove
                .i2c_try_begin_with_config(I2cPins::CARDPUTER_GROVE, I2cConfig::new(0)),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!cardputer.grove.i2c_probe(address));
        assert!(!cardputer.grove.i2c_write(address, &[1, 2]));
        assert_eq!(
            cardputer.grove.i2c_try_write(address, &[1, 2]),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert_eq!(cardputer.grove.i2c_read(address, &mut buffer), 0);
        assert_eq!(
            cardputer.grove.i2c_try_read(address, &mut buffer),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert_eq!(
            cardputer.grove.i2c_try_read_exact(address, &mut buffer),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert!(!cardputer.grove.i2c_write_reg(address, 0x00, &[1, 2]));
        assert_eq!(
            cardputer.grove.i2c_try_write_reg(address, 0x00, &[1, 2]),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert_eq!(cardputer.grove.i2c_read_reg(address, 0x00, &mut buffer), 0);
        assert_eq!(
            cardputer.grove.i2c_try_read_reg(address, 0x00, &mut buffer),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        assert_eq!(
            cardputer
                .grove
                .i2c_try_read_reg_exact(address, 0x00, &mut buffer),
            Err(Error::Unavailable("cardputer grove i2c"))
        );
        cardputer.grove.i2c_end();
        assert!(cardputer.grove.i2c_scan().is_empty());
        assert_eq!(I2cAddress::new(0x80), None);
        assert_eq!(address.raw(), 0x42);
        assert!(address.is_non_reserved());
        assert_eq!(address.write_address_8bit(), 0x84);
        assert_eq!(address.read_address_8bit(), 0x85);
    }

    #[test]
    fn cardputer_grove_gpio_reports_unavailable_on_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");

        assert_eq!(GrovePin::from_raw(1), GrovePin::G1);
        assert_eq!(GrovePin::from_raw(2), GrovePin::G2);
        assert_eq!(GrovePin::from_raw(99), GrovePin::Raw(99));
        assert_eq!(GrovePin::G1.raw(), 1);
        assert_eq!(GrovePin::G2.raw(), 2);
        assert!(GrovePin::G1.is_builtin());
        assert!(GrovePin::G1.is_g1());
        assert!(GrovePin::G2.is_g2());
        assert!(GrovePin::Raw(99).is_raw());
        assert!(!GrovePin::G1.is_raw());
        assert!(!GrovePin::Raw(99).is_builtin());
        assert!(!cardputer.grove.gpio_pin_mode(GrovePin::G1, GpioMode::Input));
        assert_eq!(
            cardputer
                .grove
                .gpio_try_pin_mode(GrovePin::G1, GpioMode::Input),
            Err(Error::Unavailable("cardputer grove gpio"))
        );
        assert!(!cardputer.grove.gpio_write(GrovePin::G2, true));
        assert_eq!(
            cardputer.grove.gpio_try_write(GrovePin::G2, true),
            Err(Error::Unavailable("cardputer grove gpio"))
        );
        assert_eq!(cardputer.grove.gpio_read(GrovePin::G1), None);
        assert_eq!(
            cardputer.grove.gpio_try_read(GrovePin::G1),
            Err(Error::Unavailable("cardputer grove gpio"))
        );
    }

    #[test]
    fn cardputer_grove_analog_reports_unavailable_on_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");
        let analog_config = AnalogOutputConfig::new(128)
            .with_frequency_hz(2_000)
            .with_resolution_bits(10);

        assert_eq!(cardputer.grove.analog_read(GrovePin::G1), None);
        assert_eq!(
            cardputer.grove.analog_try_read(GrovePin::G1),
            Err(Error::Unavailable("cardputer grove analog"))
        );
        assert_eq!(cardputer.grove.analog_read_millivolts(GrovePin::G2), None);
        assert_eq!(
            cardputer.grove.analog_try_read_millivolts(GrovePin::G2),
            Err(Error::Unavailable("cardputer grove analog"))
        );
        assert!(!cardputer.grove.analog_write(GrovePin::G2, 128));
        assert_eq!(
            cardputer.grove.analog_try_write(GrovePin::G2, 128),
            Err(Error::Unavailable("cardputer grove analog"))
        );
        assert!(!cardputer.grove.analog_write_frequency(GrovePin::G2, 1_000));
        assert_eq!(
            cardputer
                .grove
                .analog_try_write_frequency(GrovePin::G2, 1_000),
            Err(Error::Unavailable("cardputer grove analog"))
        );
        assert_eq!(
            cardputer.grove.analog_try_write_frequency(GrovePin::G2, 0),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!cardputer.grove.analog_write_resolution(GrovePin::G2, 8));
        assert_eq!(
            cardputer.grove.analog_try_write_resolution(GrovePin::G2, 8),
            Err(Error::Unavailable("cardputer grove analog"))
        );
        assert!(!cardputer
            .grove
            .analog_write_config(GrovePin::G2, analog_config));
        assert_eq!(
            cardputer
                .grove
                .analog_try_write_config(GrovePin::G2, analog_config),
            Err(Error::Unavailable("cardputer grove analog"))
        );
        assert_eq!(
            cardputer.grove.analog_try_write_config(
                GrovePin::G2,
                AnalogOutputConfig::new(128).with_frequency_hz(0)
            ),
            Err(Error::InvalidValue("frequency"))
        );
    }

    #[test]
    fn cardputer_grove_uart_reports_unavailable_on_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");
        let mut buffer = [0_u8; 8];
        let config = UartConfig::default().with_baud(230_400);

        assert_eq!(UartPins::CARDPUTER_GROVE.rx, 1);
        assert_eq!(UartPins::CARDPUTER_GROVE.tx, 2);
        assert!(!cardputer.grove.uart_begin(115_200));
        assert_eq!(
            cardputer.grove.uart_try_begin(115_200),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(
            cardputer.grove.uart_try_begin(0),
            Err(Error::InvalidValue("baud"))
        );
        assert!(!cardputer.grove.uart_begin_config(config));
        assert_eq!(
            cardputer.grove.uart_try_begin_config(config),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(
            cardputer.grove.uart_try_begin_config(UartConfig::new(0)),
            Err(Error::InvalidValue("baud"))
        );
        assert_eq!(
            cardputer
                .grove
                .uart_try_begin_with(UartPins::CARDPUTER_GROVE, 115_200),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(
            cardputer
                .grove
                .uart_try_begin_with(UartPins::CARDPUTER_GROVE, 0),
            Err(Error::InvalidValue("baud"))
        );
        assert!(!cardputer
            .grove
            .uart_begin_with_config(UartPins::CARDPUTER_GROVE, config));
        assert_eq!(
            cardputer
                .grove
                .uart_try_begin_with_config(UartPins::CARDPUTER_GROVE, config),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(
            cardputer
                .grove
                .uart_try_begin_with_config(UartPins::CARDPUTER_GROVE, UartConfig::new(0)),
            Err(Error::InvalidValue("baud"))
        );
        assert_eq!(cardputer.grove.uart_available(), 0);
        assert_eq!(cardputer.grove.uart_read(&mut buffer), 0);
        assert_eq!(
            cardputer.grove.uart_try_read(&mut buffer),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(cardputer.grove.uart_write(b"ping"), 0);
        assert_eq!(
            cardputer.grove.uart_try_write(b"ping"),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(
            cardputer.grove.uart_try_write_all(b"ping"),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(cardputer.grove.uart_write_byte(b'\n'), 0);
        assert_eq!(
            cardputer.grove.uart_try_write_byte(b'\n'),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(cardputer.grove.uart_write_str("pong"), 0);
        assert_eq!(
            cardputer.grove.uart_try_write_str("pong"),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        assert_eq!(
            cardputer.grove.uart_try_write_str_all("pong"),
            Err(Error::Unavailable("cardputer grove uart"))
        );
        cardputer.grove.uart_flush();
        cardputer.grove.uart_end();
    }

    #[test]
    fn cardputer_spi_reports_unavailable_on_host_stubs() {
        let mut cardputer = Cardputer::begin().expect("host stub cardputer begin should succeed");
        let pins = CardputerSdPins::BUILTIN.spi_pins();
        let config = SpiConfig::default().with_frequency_hz(DEFAULT_SPI_FREQUENCY_HZ);
        let tx = [0x9f, 0x00, 0x00];
        let mut rx = [0_u8; 3];

        assert_eq!(
            CardputerSdPins::new(40, 39, 14, 12),
            CardputerSdPins::BUILTIN
        );
        assert_eq!(CardputerSdPins::BUILTIN.pins(), (40, 39, 14, 12));
        assert_eq!(CardputerSdPins::BUILTIN.sck(), 40);
        assert_eq!(CardputerSdPins::BUILTIN.miso(), 39);
        assert_eq!(CardputerSdPins::BUILTIN.mosi(), 14);
        assert_eq!(CardputerSdPins::BUILTIN.cs(), 12);
        assert_eq!(SpiPins::from_cardputer_sd(CardputerSdPins::BUILTIN), pins);
        assert_eq!(pins, SpiPins::CARDPUTER_SD);
        assert_eq!(pins.pins(), CardputerSdPins::BUILTIN.pins());
        assert_eq!(pins.sck(), CardputerSdPins::BUILTIN.sck());
        assert_eq!(pins.miso(), CardputerSdPins::BUILTIN.miso());
        assert_eq!(pins.mosi(), CardputerSdPins::BUILTIN.mosi());
        assert_eq!(pins.cs(), CardputerSdPins::BUILTIN.cs());
        assert!(!cardputer.spi.begin_sd_bus());
        assert_eq!(
            cardputer.spi.try_begin_sd_bus(),
            Err(Error::Unavailable("cardputer spi"))
        );
        assert!(!cardputer.spi.begin_with(pins));
        assert_eq!(
            cardputer.spi.try_begin_with(pins),
            Err(Error::Unavailable("cardputer spi"))
        );
        assert_eq!(cardputer.spi.transfer_byte(0xff, config), 0);
        assert_eq!(cardputer.spi.try_transfer_byte(0xff, config), Ok(0));
        assert_eq!(
            cardputer.spi.try_transfer_byte(
                0xff,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!cardputer.spi.transfer(&tx, &mut rx, config));
        assert_eq!(
            cardputer.spi.try_transfer(&tx, &mut rx, config),
            Err(Error::Unavailable("cardputer spi"))
        );
        assert_eq!(
            cardputer.spi.try_transfer(
                &tx,
                &mut rx,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert_eq!(rx, [0, 0, 0]);
        assert!(!cardputer.spi.read(&mut rx, config));
        assert_eq!(
            cardputer.spi.try_read(&mut rx, config),
            Err(Error::Unavailable("cardputer spi"))
        );
        assert_eq!(
            cardputer.spi.try_read(
                &mut rx,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!cardputer.spi.write(&tx, config));
        assert_eq!(
            cardputer.spi.try_write(&tx, config),
            Err(Error::Unavailable("cardputer spi"))
        );
        assert_eq!(
            cardputer.spi.try_write_all(&tx, config),
            Err(Error::Unavailable("cardputer spi"))
        );
        assert_eq!(
            cardputer.spi.try_write(
                &tx,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert_eq!(
            cardputer.spi.try_write_all(
                &tx,
                SpiConfig {
                    frequency_hz: 0,
                    ..config
                }
            ),
            Err(Error::InvalidValue("frequency"))
        );
        assert!(!cardputer.spi.transfer(&tx, &mut rx[..2], config));
        assert_eq!(
            cardputer.spi.try_transfer(&tx, &mut rx[..2], config),
            Err(Error::InvalidValue("spi transfer length"))
        );
        cardputer.spi.end();
    }
}
