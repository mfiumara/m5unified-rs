//! M5Cardputer-oriented convenience wrapper.
//!
//! The Cardputer handle shares the safe M5Unified display, audio, power, and
//! button wrappers, then adds small helpers for the keyboard, built-in SD slot,
//! IR transmitter, Grove port, and raw SPI experiments used by the examples.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::{
    sd_begin_with_config, sd_end, sd_is_mounted, Button, ButtonId, Buttons, Display, Error, Led,
    Log, M5UnifiedConfig, Mic, Power, SdSpiConfig, Speaker, SD_MOUNT_PATH,
};

#[derive(Debug)]
pub struct Cardputer {
    pub display: Display,
    pub buttons: Buttons,
    pub button_a: Button,
    pub keyboard: CardputerKeyboard,
    pub sd: CardputerSd,
    pub ir: CardputerIr,
    pub grove: CardputerGrove,
    pub spi: CardputerSpi,
    pub mic: Mic,
    pub speaker: Speaker,
    pub power: Power,
    pub led: Led,
    pub log: Log,
}

impl Cardputer {
    pub fn begin() -> Result<Self, Error> {
        Self::begin_with_keyboard(true)
    }

    pub fn begin_with_config(config: &M5UnifiedConfig) -> Result<Self, Error> {
        Self::begin_with_config_and_keyboard(config, true)
    }

    pub fn begin_with_keyboard(enable_keyboard: bool) -> Result<Self, Error> {
        Self::from_begin_result(unsafe { m5unified_sys::m5u_cardputer_begin(enable_keyboard) })
    }

    pub fn begin_with_config_and_keyboard(
        config: &M5UnifiedConfig,
        enable_keyboard: bool,
    ) -> Result<Self, Error> {
        let raw = config.to_raw();
        Self::from_begin_result(unsafe {
            m5unified_sys::m5u_cardputer_begin_with_config(&raw, enable_keyboard)
        })
    }

    fn from_begin_result(ok: bool) -> Result<Self, Error> {
        if !ok {
            return Err(Error::BeginFailed);
        }

        let buttons = Buttons;
        Ok(Self {
            display: Display,
            button_a: buttons.button(ButtonId::A),
            buttons,
            keyboard: CardputerKeyboard,
            sd: CardputerSd,
            ir: CardputerIr,
            grove: CardputerGrove,
            spi: CardputerSpi,
            mic: Mic,
            speaker: Speaker,
            power: Power,
            led: Led,
            log: Log,
        })
    }

    pub fn update(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_update() }
    }

    pub fn delay_ms(&self, ms: u32) {
        unsafe { m5unified_sys::m5u_delay_ms(ms) }
    }
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

impl CardputerKeyboardState {
    pub fn new() -> Self {
        Self::default()
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

    pub fn word_utf8(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.word)
    }

    pub fn word_lossy(&self) -> String {
        String::from_utf8_lossy(&self.word).into_owned()
    }

    pub fn first_word_char(&self) -> Option<char> {
        self.word_utf8().ok()?.chars().next()
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct CardputerKeyValue {
    pub first: u8,
    pub second: u8,
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

    pub fn key_at(&self, x: u8, y: u8) -> Option<u8> {
        if x >= Self::COLUMNS || y >= Self::ROWS {
            return None;
        }
        let key = unsafe { m5unified_sys::m5u_cardputer_keyboard_get_key(x, y) };
        (key != 0).then_some(key)
    }

    pub fn try_key_at(&self, x: u8, y: u8) -> Result<u8, Error> {
        self.key_at(x, y)
            .ok_or(Error::Unavailable("cardputer keyboard position"))
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
        self.key_value_at(x, y)
            .ok_or(Error::Unavailable("cardputer keyboard position"))
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

    pub fn capslocked(&self) -> bool {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_capslocked() }
    }

    pub fn set_capslocked(&mut self, locked: bool) {
        unsafe { m5unified_sys::m5u_cardputer_keyboard_set_capslocked(locked) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CardputerSdPins {
    pub sck: i32,
    pub miso: i32,
    pub mosi: i32,
    pub cs: i32,
}

impl CardputerSdPins {
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
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::None)
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
}

impl CardputerSd {
    pub const DEFAULT_FREQUENCY_HZ: u32 = 25_000_000;

    pub fn begin(&mut self) -> bool {
        self.try_begin().is_ok()
    }

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.try_begin_with(CardputerSdPins::BUILTIN, Self::DEFAULT_FREQUENCY_HZ)
    }

    pub fn begin_with(&mut self, pins: CardputerSdPins, frequency_hz: u32) -> bool {
        self.try_begin_with(pins, frequency_hz).is_ok()
    }

    pub fn try_begin_with(
        &mut self,
        pins: CardputerSdPins,
        frequency_hz: u32,
    ) -> Result<(), Error> {
        let config = SdSpiConfig {
            pin_sclk: pins.sck,
            pin_mosi: pins.mosi,
            pin_miso: pins.miso,
            pin_cs: pins.cs,
            frequency_khz: frequency_hz / 1_000,
            ..SdSpiConfig::default()
        };
        sd_begin_with_config(&config)
            .then_some(())
            .ok_or(Error::Unavailable("cardputer sd"))
    }

    pub fn is_mounted(&self) -> bool {
        sd_is_mounted()
    }

    pub fn card_type(&self) -> SdCardType {
        if self.is_mounted() {
            SdCardType::Unknown(0)
        } else {
            SdCardType::None
        }
    }

    pub fn info(&self) -> SdCardInfo {
        SdCardInfo::default()
    }

    pub fn end(&mut self) {
        sd_end();
    }

    pub fn exists(&self, path: &str) -> Result<bool, Error> {
        Ok(cardputer_sd_path(path).exists())
    }

    pub fn file_size(&self, path: &str) -> Result<u64, Error> {
        Ok(fs::metadata(cardputer_sd_path(path))
            .map_err(|_| Error::Unavailable("cardputer sd file"))?
            .len())
    }

    pub fn is_directory(&self, path: &str) -> Result<bool, Error> {
        Ok(fs::metadata(cardputer_sd_path(path))
            .map_err(|_| Error::Unavailable("cardputer sd path"))?
            .is_dir())
    }

    pub fn list_dir(
        &self,
        path: &str,
        max_entries: usize,
    ) -> Result<Vec<CardputerSdDirEntry>, Error> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(cardputer_sd_path(path))
            .map_err(|_| Error::Unavailable("cardputer sd dir"))?
            .take(max_entries)
        {
            let entry = entry.map_err(|_| Error::Unavailable("cardputer sd dir"))?;
            let metadata = entry
                .metadata()
                .map_err(|_| Error::Unavailable("cardputer sd dir"))?;
            entries.push(CardputerSdDirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                is_directory: metadata.is_dir(),
                size: if metadata.is_dir() { 0 } else { metadata.len() },
            });
        }
        Ok(entries)
    }

    pub fn read_file(&self, path: &str, buffer: &mut [u8]) -> Result<usize, Error> {
        File::open(cardputer_sd_path(path))
            .and_then(|mut file| file.read(buffer))
            .map_err(|_| Error::Unavailable("cardputer sd file"))
    }

    pub fn write_file(&self, path: &str, data: &[u8]) -> Result<usize, Error> {
        write_cardputer_sd_file(path, data, false)
    }

    pub fn append_file(&self, path: &str, data: &[u8]) -> Result<usize, Error> {
        write_cardputer_sd_file(path, data, true)
    }

    pub fn remove_file(&self, path: &str) -> Result<bool, Error> {
        fs::remove_file(cardputer_sd_path(path))
            .map(|_| true)
            .map_err(|_| Error::Unavailable("cardputer sd file"))
    }

    pub fn mkdir(&self, path: &str) -> Result<bool, Error> {
        fs::create_dir_all(cardputer_sd_path(path))
            .map(|_| true)
            .map_err(|_| Error::Unavailable("cardputer sd dir"))
    }

    pub fn rmdir(&self, path: &str) -> Result<bool, Error> {
        fs::remove_dir(cardputer_sd_path(path))
            .map(|_| true)
            .map_err(|_| Error::Unavailable("cardputer sd dir"))
    }

    pub fn rename(&self, from_path: &str, to_path: &str) -> Result<bool, Error> {
        fs::rename(cardputer_sd_path(from_path), cardputer_sd_path(to_path))
            .map(|_| true)
            .map_err(|_| Error::Unavailable("cardputer sd file"))
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
}

impl CardputerIr {
    pub const BUILTIN_TX_PIN: i32 = 44;

    pub fn try_begin(&mut self) -> Result<(), Error> {
        self.try_begin_on_pin(Self::BUILTIN_TX_PIN)
    }

    pub fn try_begin_on_pin(&mut self, pin: i32) -> Result<(), Error> {
        unsafe { m5unified_sys::m5u_cardputer_ir_begin(pin) }
            .then_some(())
            .ok_or(Error::Unavailable("cardputer ir"))
    }

    pub fn send_nec(&mut self, frame: NecFrame) -> bool {
        self.try_send_nec(frame).is_ok()
    }

    pub fn try_send_nec(&mut self, frame: NecFrame) -> Result<(), Error> {
        unsafe {
            m5unified_sys::m5u_cardputer_ir_send_nec(frame.address, frame.command, frame.repeats)
        }
        .then_some(())
        .ok_or(Error::Unavailable("cardputer ir"))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct I2cAddress(u8);

impl I2cAddress {
    pub const MAX_7BIT: u8 = 0x7f;

    pub const fn new(raw: u8) -> Option<Self> {
        if raw <= Self::MAX_7BIT {
            Some(Self(raw))
        } else {
            None
        }
    }

    pub const fn raw(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GrovePin {
    G1,
    G2,
    Raw(i32),
}

impl GrovePin {
    pub const fn raw(self) -> i32 {
        match self {
            Self::G1 => 1,
            Self::G2 => 2,
            Self::Raw(pin) => pin,
        }
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
    pub const fn raw(self) -> i32 {
        match self {
            Self::Input => 0,
            Self::Output => 1,
            Self::InputPullup => 2,
            Self::InputPulldown => 3,
        }
    }
}

impl CardputerGrove {
    pub const DEFAULT_I2C_FREQUENCY_HZ: u32 = 100_000;
    pub const I2C_SDA: i32 = 2;
    pub const I2C_SCL: i32 = 1;

    pub fn i2c_try_begin(&mut self) -> Result<(), Error> {
        self.i2c_try_begin_with_frequency(Self::DEFAULT_I2C_FREQUENCY_HZ)
    }

    pub fn i2c_try_begin_with_frequency(&mut self, frequency_hz: u32) -> Result<(), Error> {
        unsafe {
            m5unified_sys::m5u_cardputer_grove_i2c_begin(Self::I2C_SDA, Self::I2C_SCL, frequency_hz)
        }
        .then_some(())
        .ok_or(Error::Unavailable("cardputer grove i2c"))
    }

    pub fn i2c_end(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_grove_i2c_end() }
    }

    pub fn i2c_scan(&self) -> Vec<I2cAddress> {
        (0..=I2cAddress::MAX_7BIT)
            .filter_map(|address| {
                unsafe { m5unified_sys::m5u_cardputer_grove_i2c_probe(address) }
                    .then(|| I2cAddress::new(address))
                    .flatten()
            })
            .collect()
    }

    pub fn i2c_try_write(&mut self, address: I2cAddress, data: &[u8]) -> Result<(), Error> {
        unsafe {
            m5unified_sys::m5u_cardputer_grove_i2c_write(address.raw(), data.as_ptr(), data.len())
        }
        .then_some(())
        .ok_or(Error::Unavailable("cardputer grove i2c"))
    }

    pub fn i2c_try_read(&mut self, address: I2cAddress, buffer: &mut [u8]) -> Result<usize, Error> {
        let read = unsafe {
            m5unified_sys::m5u_cardputer_grove_i2c_read(
                address.raw(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        if read == buffer.len() {
            Ok(read)
        } else {
            Err(Error::Unavailable("cardputer grove i2c"))
        }
    }

    pub fn gpio_try_pin_mode(&mut self, pin: GrovePin, mode: GpioMode) -> Result<(), Error> {
        unsafe { m5unified_sys::m5u_cardputer_grove_gpio_pin_mode(pin.raw(), mode.raw()) }
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove gpio"))
    }

    pub fn gpio_try_read(&self, pin: GrovePin) -> Result<bool, Error> {
        match unsafe { m5unified_sys::m5u_cardputer_grove_gpio_read(pin.raw()) } {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::Unavailable("cardputer grove gpio")),
        }
    }

    pub fn gpio_try_write(&mut self, pin: GrovePin, high: bool) -> Result<(), Error> {
        unsafe { m5unified_sys::m5u_cardputer_grove_gpio_write(pin.raw(), high) }
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove gpio"))
    }

    pub fn analog_read_millivolts(&self, pin: GrovePin) -> Option<u16> {
        self.analog_try_read_millivolts(pin).ok()
    }

    pub fn analog_try_read(&self, _pin: GrovePin) -> Result<u16, Error> {
        Err(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_try_read_millivolts(&self, _pin: GrovePin) -> Result<u16, Error> {
        Err(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_try_write(&mut self, _pin: GrovePin, _duty: u8) -> Result<(), Error> {
        Err(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_try_write_frequency(
        &mut self,
        _pin: GrovePin,
        _frequency_hz: u32,
    ) -> Result<(), Error> {
        Err(Error::Unavailable("cardputer grove analog"))
    }

    pub fn analog_try_write_resolution(
        &mut self,
        _pin: GrovePin,
        _resolution_bits: u8,
    ) -> Result<(), Error> {
        Err(Error::Unavailable("cardputer grove analog"))
    }

    pub fn uart_try_begin(&mut self, baud: u32) -> Result<(), Error> {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_begin(Self::I2C_SCL, Self::I2C_SDA, baud) }
            .then_some(())
            .ok_or(Error::Unavailable("cardputer grove uart"))
    }

    pub fn uart_end(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_end() }
    }

    pub fn uart_available(&self) -> usize {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_available() }
    }

    pub fn uart_try_read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let read = unsafe {
            m5unified_sys::m5u_cardputer_grove_uart_read(buffer.as_mut_ptr(), buffer.len())
        };
        (read > 0)
            .then_some(read)
            .ok_or(Error::Unavailable("cardputer grove uart"))
    }

    pub fn uart_try_write_all(&mut self, data: &[u8]) -> Result<usize, Error> {
        let written =
            unsafe { m5unified_sys::m5u_cardputer_grove_uart_write(data.as_ptr(), data.len()) };
        (written == data.len())
            .then_some(written)
            .ok_or(Error::Unavailable("cardputer grove uart"))
    }

    pub fn uart_try_write_str(&mut self, text: &str) -> Result<usize, Error> {
        self.uart_try_write_all(text.as_bytes())
    }

    pub fn uart_flush(&mut self) {
        unsafe { m5unified_sys::m5u_cardputer_grove_uart_flush() }
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
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum SpiBitOrder {
    #[default]
    MsbFirst,
    LsbFirst,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum SpiMode {
    #[default]
    Mode0,
    Mode1,
    Mode2,
    Mode3,
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
            frequency_hz: 1_000_000,
            mode: SpiMode::Mode0,
            bit_order: SpiBitOrder::MsbFirst,
        }
    }
}

impl SpiConfig {
    pub const fn with_mode(mut self, mode: SpiMode) -> Self {
        self.mode = mode;
        self
    }

    pub const fn with_bit_order(mut self, bit_order: SpiBitOrder) -> Self {
        self.bit_order = bit_order;
        self
    }
}

impl CardputerSpi {
    pub fn try_begin_with(&mut self, _pins: SpiPins) -> Result<(), Error> {
        Err(Error::Unavailable("cardputer spi"))
    }

    pub fn end(&mut self) {}

    pub fn try_transfer(
        &mut self,
        _tx: &[u8],
        _rx: &mut [u8],
        _config: SpiConfig,
    ) -> Result<(), Error> {
        Err(Error::Unavailable("cardputer spi"))
    }

    pub fn try_write(&mut self, _data: &[u8], _config: SpiConfig) -> Result<(), Error> {
        Err(Error::Unavailable("cardputer spi"))
    }
}

fn cardputer_sd_path(path: &str) -> PathBuf {
    let path = path.trim_start_matches('/');
    if path.is_empty() {
        Path::new(SD_MOUNT_PATH).to_path_buf()
    } else {
        Path::new(SD_MOUNT_PATH).join(path)
    }
}

fn write_cardputer_sd_file(path: &str, data: &[u8], append: bool) -> Result<usize, Error> {
    let mut options = OpenOptions::new();
    options.create(true).write(true);
    if append {
        options.append(true);
    } else {
        options.truncate(true);
    }
    options
        .open(cardputer_sd_path(path))
        .and_then(|mut file| file.write(data))
        .map_err(|_| Error::Unavailable("cardputer sd file"))
}

fn ascii_byte_to_char(byte: u8) -> Option<char> {
    if byte == 0 || byte == 0xff || !byte.is_ascii() {
        None
    } else {
        Some(byte as char)
    }
}
