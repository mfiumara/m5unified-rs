//! Power-management, PMIC, sleep, and AXP2101 helpers.
//!
//! The safe wrapper exposes battery and VBUS readings, output controls, sleep
//! timers, vibration, external-port power, and direct PMIC-specific helpers.

use crate::{Date, Time};

#[derive(Debug)]
pub struct Power;

impl Power {
    /// Initialize the power-management backend.
    ///
    /// `M5Unified::begin` calls this as part of normal startup; this helper is
    /// exposed for parity with M5Unified and advanced reinitialization flows.
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_power_begin() }
    }

    /// Return the detected power-management IC type.
    pub fn pmic_type(&self) -> PowerType {
        PowerType::from_raw(unsafe { m5unified_sys::m5u_power_get_type() as i32 })
    }

    /// Return the battery level as a percentage when the board reports one.
    pub fn battery_level(&self) -> Option<u8> {
        let level = unsafe { m5unified_sys::m5u_battery_level() };
        if (0..=100).contains(&level) {
            Some(level as u8)
        } else {
            None
        }
    }

    /// Return the battery voltage in millivolts when supported.
    pub fn battery_voltage_mv(&self) -> Option<u16> {
        let mv = unsafe { m5unified_sys::m5u_battery_voltage_mv() };
        (mv >= 0).then_some(mv as u16)
    }

    /// Return VBUS voltage in millivolts when the board supports it.
    pub fn vbus_voltage_mv(&self) -> Option<u16> {
        let mv = unsafe { m5unified_sys::m5u_power_get_vbus_voltage_mv() };
        (mv >= 0).then_some(mv as u16)
    }

    /// Return battery current in milliamps.
    ///
    /// Positive values indicate charge current and negative values indicate
    /// discharge current.
    pub fn battery_current_ma(&self) -> i32 {
        unsafe { m5unified_sys::m5u_power_get_battery_current_ma() as i32 }
    }

    /// Return external-port voltage in millivolts for the selected port mask.
    pub fn ext_voltage_mv(&self, port_mask: ExtPortMask) -> f32 {
        unsafe { m5unified_sys::m5u_power_get_ext_voltage_mv(port_mask.bits()) }
    }

    /// Return external-port current in milliamps for the selected port mask.
    pub fn ext_current_ma(&self, port_mask: ExtPortMask) -> f32 {
        unsafe { m5unified_sys::m5u_power_get_ext_current_ma(port_mask.bits()) }
    }

    /// Return the explicit charging state reported by M5Unified.
    pub fn charge_state(&self) -> ChargeState {
        ChargeState::from_raw(unsafe { m5unified_sys::m5u_power_get_charge_state() as i32 })
    }

    /// Return whether M5Unified reports that the battery is charging.
    pub fn is_charging(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_is_charging() }
    }

    /// Set the board power LED brightness where the board supports it.
    pub fn set_led(&mut self, brightness: u8) {
        unsafe { m5unified_sys::m5u_power_set_led(brightness) }
    }

    /// Set external port output for the selected ports.
    pub fn set_ext_output(&mut self, enable: bool, port_mask: ExtPortMask) {
        unsafe { m5unified_sys::m5u_power_set_ext_output(enable, port_mask.bits()) }
    }

    /// Compatibility alias for M5Unified's deprecated `setExtPower` helper.
    ///
    /// This applies the same default port mask as M5Unified's C++ API.
    pub fn set_ext_power(&mut self, enable: bool) {
        self.set_ext_output(enable, ExtPortMask::ALL)
    }

    /// Return whether external port output is enabled.
    pub fn ext_output(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_get_ext_output() }
    }

    /// Set main USB power output where the board supports it.
    pub fn set_usb_output(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_power_set_usb_output(enable) }
    }

    /// Return whether main USB power output is enabled.
    pub fn usb_output(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_get_usb_output() }
    }

    /// Enable or disable battery charging where the PMIC supports it.
    pub fn set_battery_charge(&mut self, enable: bool) {
        unsafe { m5unified_sys::m5u_power_set_battery_charge(enable) }
    }

    /// Set the PMIC charge-current target in milliamps.
    pub fn set_charge_current_ma(&mut self, max_ma: u16) {
        unsafe { m5unified_sys::m5u_power_set_charge_current(max_ma) }
    }

    /// Set the PMIC charge-voltage target in millivolts.
    pub fn set_charge_voltage_mv(&mut self, max_mv: u16) {
        unsafe { m5unified_sys::m5u_power_set_charge_voltage(max_mv) }
    }

    /// Return the PMIC key state.
    ///
    /// M5Unified reports `0` for none, `1` for long press, `2` for short click,
    /// and `3` for both. On supported PMICs this read clears the latched state.
    pub fn key_state(&self) -> u8 {
        unsafe { m5unified_sys::m5u_power_get_key_state() }
    }

    /// Configure external-port bus output where the board supports it.
    pub fn set_ext_port_bus_config(&mut self, config: ExtPortBusConfig) {
        let raw = config.to_raw();
        unsafe { m5unified_sys::m5u_power_set_ext_port_bus_config(&raw) }
    }

    /// Set vibration motor strength where the board supports it.
    pub fn set_vibration(&mut self, level: u8) {
        unsafe { m5unified_sys::m5u_power_set_vibration(level) }
    }

    /// Power the board off through M5Unified.
    pub fn power_off(&mut self) {
        unsafe { m5unified_sys::m5u_power_power_off() }
    }

    /// Enter timer sleep and wake after the given number of seconds.
    pub fn timer_sleep_seconds(&mut self, seconds: i32) {
        unsafe { m5unified_sys::m5u_power_timer_sleep_seconds(seconds) }
    }

    /// Enter timer sleep and wake at the given RTC time.
    ///
    /// As in M5Unified, seconds are ignored by the underlying implementation.
    pub fn timer_sleep_time(&mut self, time: Time) {
        let raw = time.to_raw();
        unsafe { m5unified_sys::m5u_power_timer_sleep_time(&raw) }
    }

    /// Enter timer sleep and wake at the given RTC date and time.
    ///
    /// As in M5Unified, the year, month, and seconds fields are ignored by the
    /// underlying implementation.
    pub fn timer_sleep_date_time(&mut self, date: Date, time: Time) {
        let date = date.to_raw();
        let time = time.to_raw();
        unsafe { m5unified_sys::m5u_power_timer_sleep_date_time(&date, &time) }
    }

    /// Enter ESP32 deep sleep, optionally enabling touch wakeup.
    pub fn deep_sleep_us(&mut self, micro_seconds: u64, touch_wakeup: bool) {
        unsafe { m5unified_sys::m5u_power_deep_sleep_us(micro_seconds, touch_wakeup) }
    }

    /// Enter ESP32 light sleep, optionally enabling touch wakeup.
    pub fn light_sleep_us(&mut self, micro_seconds: u64, touch_wakeup: bool) {
        unsafe { m5unified_sys::m5u_power_light_sleep_us(micro_seconds, touch_wakeup) }
    }

    pub fn axp2101(&self) -> Axp2101 {
        Axp2101
    }

    pub fn aw32001(&self) -> Aw32001 {
        Aw32001
    }

    pub fn bq27220(&self) -> Bq27220 {
        Bq27220
    }

    pub fn ina226(&self) -> Ina226 {
        Ina226
    }

    pub fn ip5306(&self) -> Ip5306 {
        Ip5306
    }

    pub fn py32pmic(&self) -> Py32Pmic {
        Py32Pmic
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExtPortBusConfig {
    pub voltage_mv: u16,
    pub current_limit_ma: u8,
    pub enable: bool,
    pub direction_output: bool,
}

impl ExtPortBusConfig {
    fn to_raw(self) -> m5unified_sys::m5u_power_ext_port_bus_t {
        m5unified_sys::m5u_power_ext_port_bus_t {
            voltage_mv: self.voltage_mv,
            current_limit_ma: self.current_limit_ma,
            enable: self.enable,
            direction_output: self.direction_output,
        }
    }
}

impl Default for ExtPortBusConfig {
    fn default() -> Self {
        Self {
            voltage_mv: 5_000,
            current_limit_ma: 0,
            enable: false,
            direction_output: false,
        }
    }
}

/// M5Unified PMIC type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PowerType {
    Unknown,
    Adc,
    Axp192,
    Ip5306,
    Axp2101,
    Aw32001,
    Py32Pmic,
    M5Pm1,
    Raw(i32),
}

impl PowerType {
    /// Convert the raw M5Unified PMIC discriminant into a Rust enum.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Unknown,
            1 => Self::Adc,
            2 => Self::Axp192,
            3 => Self::Ip5306,
            4 => Self::Axp2101,
            5 => Self::Aw32001,
            6 => Self::Py32Pmic,
            7 => Self::M5Pm1,
            other => Self::Raw(other),
        }
    }

    /// Return the raw M5Unified PMIC discriminant.
    pub const fn raw(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::Adc => 1,
            Self::Axp192 => 2,
            Self::Ip5306 => 3,
            Self::Axp2101 => 4,
            Self::Aw32001 => 5,
            Self::Py32Pmic => 6,
            Self::M5Pm1 => 7,
            Self::Raw(raw) => raw,
        }
    }
}

/// Battery charge state reported by M5Unified.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChargeState {
    Discharging,
    Charging,
    Unknown,
    Raw(i32),
}

impl ChargeState {
    /// Convert the raw M5Unified charging-state discriminant into a Rust enum.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Discharging,
            1 => Self::Charging,
            2 => Self::Unknown,
            other => Self::Raw(other),
        }
    }

    /// Return the raw M5Unified charging-state discriminant.
    pub const fn raw(self) -> i32 {
        match self {
            Self::Discharging => 0,
            Self::Charging => 1,
            Self::Unknown => 2,
            Self::Raw(raw) => raw,
        }
    }
}

/// Charge state reported directly by the AXP2101 PMIC.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axp2101ChargeStatus {
    Unavailable,
    Discharging,
    Standby,
    Charging,
    Raw(i32),
}

impl Axp2101ChargeStatus {
    /// Convert the raw M5Unified AXP2101 status into a Rust enum.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            -2 => Self::Unavailable,
            -1 => Self::Discharging,
            0 => Self::Standby,
            1 => Self::Charging,
            other => Self::Raw(other),
        }
    }

    /// Return the raw M5Unified AXP2101 status.
    pub const fn raw(self) -> i32 {
        match self {
            Self::Unavailable => -2,
            Self::Discharging => -1,
            Self::Standby => 0,
            Self::Charging => 1,
            Self::Raw(raw) => raw,
        }
    }
}

/// Latched AXP2101 PEK button press state.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axp2101PekPress {
    None,
    Long,
    Short,
    Both,
    Raw(u8),
}

impl Axp2101PekPress {
    /// Convert the raw M5Unified AXP2101 PEK state into a Rust enum.
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Long,
            2 => Self::Short,
            3 => Self::Both,
            other => Self::Raw(other),
        }
    }

    /// Return the raw M5Unified AXP2101 PEK state.
    pub const fn raw(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Long => 1,
            Self::Short => 2,
            Self::Both => 3,
            Self::Raw(raw) => raw,
        }
    }
}

/// Charge state reported directly by the AW32001 charger.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Aw32001ChargeStatus {
    Unknown,
    NotCharging,
    PreCharge,
    Charging,
    ChargeDone,
    Raw(i32),
}

impl Aw32001ChargeStatus {
    /// Convert the raw M5Unified AW32001 charge status into a Rust enum.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            -1 => Self::Unknown,
            0 => Self::NotCharging,
            1 => Self::PreCharge,
            2 => Self::Charging,
            3 => Self::ChargeDone,
            other => Self::Raw(other),
        }
    }

    /// Return the raw M5Unified AW32001 charge status.
    pub const fn raw(self) -> i32 {
        match self {
            Self::Unknown => -1,
            Self::NotCharging => 0,
            Self::PreCharge => 1,
            Self::Charging => 2,
            Self::ChargeDone => 3,
            Self::Raw(raw) => raw,
        }
    }
}

/// Latched PY32 PMIC power-key press state.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Py32PmicPekPress {
    None,
    Short,
    Raw(u8),
}

impl Py32PmicPekPress {
    /// Convert the raw M5Unified PY32 PMIC PEK state into a Rust enum.
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::None,
            2 => Self::Short,
            other => Self::Raw(other),
        }
    }

    /// Return the raw M5Unified PY32 PMIC PEK state.
    pub const fn raw(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Short => 2,
            Self::Raw(raw) => raw,
        }
    }
}

/// INA226 sample averaging rate.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ina226Sampling {
    Rate1,
    Rate4,
    Rate16,
    Rate64,
    Rate128,
    Rate256,
    Rate512,
    Rate1024,
    Raw(u8),
}

impl Ina226Sampling {
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Rate1,
            1 => Self::Rate4,
            2 => Self::Rate16,
            3 => Self::Rate64,
            4 => Self::Rate128,
            5 => Self::Rate256,
            6 => Self::Rate512,
            7 => Self::Rate1024,
            other => Self::Raw(other),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::Rate1 => 0,
            Self::Rate4 => 1,
            Self::Rate16 => 2,
            Self::Rate64 => 3,
            Self::Rate128 => 4,
            Self::Rate256 => 5,
            Self::Rate512 => 6,
            Self::Rate1024 => 7,
            Self::Raw(raw) => raw,
        }
    }
}

/// INA226 bus or shunt conversion time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ina226ConversionTime {
    Us140,
    Us204,
    Us332,
    Us588,
    Us1100,
    Us2116,
    Us4156,
    Us8244,
    Raw(u8),
}

impl Ina226ConversionTime {
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Us140,
            1 => Self::Us204,
            2 => Self::Us332,
            3 => Self::Us588,
            4 => Self::Us1100,
            5 => Self::Us2116,
            6 => Self::Us4156,
            7 => Self::Us8244,
            other => Self::Raw(other),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::Us140 => 0,
            Self::Us204 => 1,
            Self::Us332 => 2,
            Self::Us588 => 3,
            Self::Us1100 => 4,
            Self::Us2116 => 5,
            Self::Us4156 => 6,
            Self::Us8244 => 7,
            Self::Raw(raw) => raw,
        }
    }
}

/// INA226 operation mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ina226Mode {
    PowerDown,
    ShuntVoltageSingle,
    BusVoltageSingle,
    ShuntAndBusSingle,
    ShuntVoltage,
    BusVoltage,
    ShuntAndBus,
    Raw(u8),
}

impl Ina226Mode {
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::PowerDown,
            1 => Self::ShuntVoltageSingle,
            2 => Self::BusVoltageSingle,
            3 => Self::ShuntAndBusSingle,
            5 => Self::ShuntVoltage,
            6 => Self::BusVoltage,
            7 => Self::ShuntAndBus,
            other => Self::Raw(other),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::PowerDown => 0,
            Self::ShuntVoltageSingle => 1,
            Self::BusVoltageSingle => 2,
            Self::ShuntAndBusSingle => 3,
            Self::ShuntVoltage => 5,
            Self::BusVoltage => 6,
            Self::ShuntAndBus => 7,
            Self::Raw(raw) => raw,
        }
    }
}

/// INA226 power monitor configuration.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ina226Config {
    pub shunt_res_ohm: f32,
    pub max_expected_current_a: f32,
    pub sampling_rate: Ina226Sampling,
    pub shunt_conversion_time: Ina226ConversionTime,
    pub bus_conversion_time: Ina226ConversionTime,
    pub mode: Ina226Mode,
}

impl Ina226Config {
    fn to_raw(self) -> m5unified_sys::m5u_power_ina226_config_t {
        m5unified_sys::m5u_power_ina226_config_t {
            shunt_res: self.shunt_res_ohm,
            max_expected_current: self.max_expected_current_a,
            sampling_rate: self.sampling_rate.raw(),
            shunt_conversion_time: self.shunt_conversion_time.raw(),
            bus_conversion_time: self.bus_conversion_time.raw(),
            mode: self.mode.raw(),
        }
    }
}

impl Default for Ina226Config {
    fn default() -> Self {
        Self {
            shunt_res_ohm: 0.1,
            max_expected_current_a: 2.0,
            sampling_rate: Ina226Sampling::Rate16,
            shunt_conversion_time: Ina226ConversionTime::Us1100,
            bus_conversion_time: Ina226ConversionTime::Us1100,
            mode: Ina226Mode::ShuntAndBus,
        }
    }
}

/// External port mask for boards with independently switchable power outputs.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExtPortMask(u16);

impl ExtPortMask {
    pub const NONE: Self = Self(0);
    pub const PA: Self = Self(1 << 0);
    pub const PB1: Self = Self(1 << 1);
    pub const PB2: Self = Self(1 << 2);
    pub const PC1: Self = Self(1 << 3);
    pub const PC2: Self = Self(1 << 4);
    pub const USB: Self = Self(1 << 5);
    pub const PWR485: Self = Self(1 << 6);
    pub const PWRCAN: Self = Self(1 << 7);
    pub const MAIN: Self = Self(1 << 15);
    pub const ALL: Self = Self(0x00FF);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }
}

impl Default for ExtPortMask {
    fn default() -> Self {
        Self::NONE
    }
}

impl core::ops::BitOr for ExtPortMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExtPortMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExtPortMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign for ExtPortMask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

#[derive(Debug)]
pub struct Aw32001;

impl Aw32001 {
    /// Initialize the direct AW32001 backend when this board has one.
    pub fn begin(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_aw32001_begin() }
    }

    /// Enable or disable battery charging through the AW32001.
    pub fn set_battery_charge(&self, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_power_aw32001_set_battery_charge(enable) }
    }

    /// Set the AW32001 charge-current target in milliamps.
    pub fn set_charge_current_ma(&self, max_ma: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_aw32001_set_charge_current(max_ma) }
    }

    /// Set the AW32001 charge-voltage target in millivolts.
    pub fn set_charge_voltage_mv(&self, max_mv: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_aw32001_set_charge_voltage(max_mv) }
    }

    /// Return whether the AW32001 reports that the battery is charging.
    pub fn is_charging(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_aw32001_is_charging() }
    }

    /// Return the configured AW32001 charge current in milliamps.
    pub fn charge_current_ma(&self) -> Option<u16> {
        let current = unsafe { m5unified_sys::m5u_power_aw32001_get_charge_current() };
        (current != 0).then_some(current)
    }

    /// Return the configured AW32001 charge voltage in millivolts.
    pub fn charge_voltage_mv(&self) -> Option<u16> {
        let voltage = unsafe { m5unified_sys::m5u_power_aw32001_get_charge_voltage() };
        (voltage != 0).then_some(voltage)
    }

    /// Return the direct AW32001 charge status.
    pub fn charge_status(&self) -> Aw32001ChargeStatus {
        Aw32001ChargeStatus::from_raw(unsafe {
            m5unified_sys::m5u_power_aw32001_get_charge_status()
        })
    }
}

#[derive(Debug)]
pub struct Bq27220;

impl Bq27220 {
    /// Initialize the direct BQ27220 battery gauge backend when this board has one.
    pub fn begin(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_bq27220_begin() }
    }

    /// Return battery current in milliamps.
    pub fn current_ma(&self) -> i16 {
        unsafe { m5unified_sys::m5u_power_bq27220_get_current_ma() }
    }

    /// Return battery voltage in millivolts.
    pub fn voltage_mv(&self) -> i16 {
        unsafe { m5unified_sys::m5u_power_bq27220_get_voltage_mv() }
    }

    /// Return battery current in amps.
    pub fn current_a(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_bq27220_get_current_a() }
    }

    /// Return battery voltage in volts.
    pub fn voltage_v(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_bq27220_get_voltage_v() }
    }
}

#[derive(Debug)]
pub struct Ina226;

impl Ina226 {
    /// Initialize the direct INA226 power monitor backend when this board has one.
    pub fn begin(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_ina226_begin() }
    }

    /// Configure INA226 sampling, conversion, and calibration parameters.
    pub fn configure(&self, config: Ina226Config) -> bool {
        let raw = config.to_raw();
        unsafe { m5unified_sys::m5u_power_ina226_config(&raw) }
    }

    /// Return bus voltage in volts.
    pub fn bus_voltage_v(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_ina226_get_bus_voltage_v() }
    }

    /// Return shunt voltage in volts.
    pub fn shunt_voltage_v(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_ina226_get_shunt_voltage_v() }
    }

    /// Return shunt current in amps.
    pub fn shunt_current_a(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_ina226_get_shunt_current_a() }
    }

    /// Return measured power in watts.
    pub fn power_w(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_ina226_get_power_w() }
    }
}

#[derive(Debug)]
pub struct Ip5306;

impl Ip5306 {
    /// Initialize the direct IP5306 backend when this board has one.
    pub fn begin(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_ip5306_begin() }
    }

    /// Return the direct IP5306 battery level as a percentage when available.
    pub fn battery_level(&self) -> Option<u8> {
        let level = unsafe { m5unified_sys::m5u_power_ip5306_get_battery_level() };
        if (0..=100).contains(&level) {
            Some(level as u8)
        } else {
            None
        }
    }

    /// Enable or disable battery charging through the IP5306.
    pub fn set_battery_charge(&self, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_power_ip5306_set_battery_charge(enable) }
    }

    /// Set the IP5306 charge-current target in milliamps.
    pub fn set_charge_current_ma(&self, max_ma: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_ip5306_set_charge_current(max_ma) }
    }

    /// Set the IP5306 charge-voltage target in millivolts.
    pub fn set_charge_voltage_mv(&self, max_mv: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_ip5306_set_charge_voltage(max_mv) }
    }

    /// Return whether the IP5306 reports that the battery is charging.
    pub fn is_charging(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_ip5306_is_charging() }
    }

    /// Set whether the IP5306 keeps boost output alive under low load.
    pub fn set_power_boost_keep_on(&self, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_power_ip5306_set_power_boost_keep_on(enable) }
    }
}

#[derive(Debug)]
pub struct Py32Pmic;

impl Py32Pmic {
    /// Initialize the direct PY32 PMIC backend when this board has one.
    pub fn begin(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_py32pmic_begin() }
    }

    /// Enable or disable external output through the PY32 PMIC.
    pub fn set_ext_output(&self, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_power_py32pmic_set_ext_output(enable) }
    }

    /// Enable or disable battery charging through the PY32 PMIC.
    pub fn set_battery_charge(&self, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_power_py32pmic_set_battery_charge(enable) }
    }

    /// Set the PY32 PMIC charge-current target in milliamps.
    ///
    /// M5Unified currently exposes this as unsupported and returns `false`.
    pub fn set_charge_current_ma(&self, max_ma: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_py32pmic_set_charge_current(max_ma) }
    }

    /// Set the PY32 PMIC charge-voltage target in millivolts.
    ///
    /// M5Unified currently exposes this as unsupported and returns `false`.
    pub fn set_charge_voltage_mv(&self, max_mv: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_py32pmic_set_charge_voltage(max_mv) }
    }

    /// Return whether the PY32 PMIC reports that the battery is charging.
    ///
    /// M5Unified currently returns `false` for this direct PY32 PMIC helper.
    pub fn is_charging(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_py32pmic_is_charging() }
    }

    /// Return the configured PY32 PMIC charge current in milliamps.
    pub fn charge_current_ma(&self) -> Option<u16> {
        let current = unsafe { m5unified_sys::m5u_power_py32pmic_get_charge_current() };
        (current != 0).then_some(current)
    }

    /// Return the configured PY32 PMIC charge voltage in millivolts.
    pub fn charge_voltage_mv(&self) -> Option<u16> {
        let voltage = unsafe { m5unified_sys::m5u_power_py32pmic_get_charge_voltage() };
        (voltage != 0).then_some(voltage)
    }

    /// Return the latched PY32 PMIC PEK press state.
    ///
    /// On supported PMICs this read clears the latched short-press state.
    pub fn pek_press(&self) -> Py32PmicPekPress {
        Py32PmicPekPress::from_raw(unsafe { m5unified_sys::m5u_power_py32pmic_get_pek_press() })
    }

    /// Power the board off through the PY32 PMIC.
    pub fn power_off(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_py32pmic_power_off() }
    }
}

#[derive(Debug)]
pub struct Axp2101;

impl Axp2101 {
    const LDO_KIND_ALDO: i32 = 0;
    const LDO_KIND_BLDO: i32 = 1;
    const LDO_KIND_DLDO: i32 = 2;

    pub const IRQ_ALL: u64 = u64::MAX;
    pub const IRQ_BAT_WORK_UNDER_TEMP: u64 = 1 << 0;
    pub const IRQ_BAT_WORK_OVER_TEMP: u64 = 1 << 1;
    pub const IRQ_BAT_CHG_UNDER_TEMP: u64 = 1 << 2;
    pub const IRQ_BAT_CHG_OVER_TEMP: u64 = 1 << 3;
    pub const IRQ_GAUGE_NEW_SOC: u64 = 1 << 4;
    pub const IRQ_GAUGE_WDT_TIMEOUT: u64 = 1 << 5;
    pub const IRQ_WARNING_LEVEL1: u64 = 1 << 6;
    pub const IRQ_WARNING_LEVEL2: u64 = 1 << 7;
    pub const IRQ_PKEY_POSITIVE_EDGE: u64 = 1 << 8;
    pub const IRQ_PKEY_NEGATIVE_EDGE: u64 = 1 << 9;
    pub const IRQ_PKEY_LONG_PRESS: u64 = 1 << 10;
    pub const IRQ_PKEY_SHORT_PRESS: u64 = 1 << 11;
    pub const IRQ_BAT_REMOVE: u64 = 1 << 12;
    pub const IRQ_BAT_INSERT: u64 = 1 << 13;
    pub const IRQ_VBUS_REMOVE: u64 = 1 << 14;
    pub const IRQ_VBUS_INSERT: u64 = 1 << 15;
    pub const IRQ_BAT_OVER_VOLTAGE: u64 = 1 << 16;
    pub const IRQ_CHARGER_TIMER: u64 = 1 << 17;
    pub const IRQ_DIE_OVER_TEMP: u64 = 1 << 18;
    pub const IRQ_BAT_CHG_START: u64 = 1 << 19;
    pub const IRQ_BAT_CHG_DONE: u64 = 1 << 20;
    pub const IRQ_BATFET_OVER_CURR: u64 = 1 << 21;
    pub const IRQ_LDO_OVER_CURR: u64 = 1 << 22;
    pub const IRQ_WDT_EXPIRE: u64 = 1 << 23;

    /// Initialize the direct AXP2101 backend when this board has one.
    pub fn begin(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_begin() }
    }

    /// Return the direct AXP2101 battery level as a percentage when available.
    pub fn battery_level(&self) -> Option<u8> {
        let level = unsafe { m5unified_sys::m5u_power_axp2101_get_battery_level() };
        if (0..=100).contains(&level) {
            Some(level as u8)
        } else {
            None
        }
    }

    /// Enable or disable battery charging through the AXP2101.
    pub fn set_battery_charge(&self, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_set_battery_charge(enable) }
    }

    /// Set the AXP2101 pre-charge current target in milliamps.
    pub fn set_pre_charge_current_ma(&self, max_ma: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_set_pre_charge_current(max_ma) }
    }

    /// Set the AXP2101 charge-current target in milliamps.
    pub fn set_charge_current_ma(&self, max_ma: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_set_charge_current(max_ma) }
    }

    /// Set the AXP2101 charge-voltage target in millivolts.
    pub fn set_charge_voltage_mv(&self, max_mv: u16) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_set_charge_voltage(max_mv) }
    }

    /// Return the direct AXP2101 charge status.
    pub fn charge_status(&self) -> Axp2101ChargeStatus {
        Axp2101ChargeStatus::from_raw(unsafe {
            m5unified_sys::m5u_power_axp2101_get_charge_status()
        })
    }

    /// Return whether the AXP2101 reports that the battery is charging.
    pub fn is_charging(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_is_charging() }
    }

    fn set_ldo(&self, kind: i32, channel: i32, voltage_mv: Option<u16>) -> bool {
        let voltage_mv = voltage_mv.map(i32::from).unwrap_or(-1);
        unsafe { m5unified_sys::m5u_power_axp2101_set_ldo(kind, channel, voltage_mv) }
    }

    fn ldo_enabled(&self, kind: i32, channel: i32) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_get_ldo_enabled(kind, channel) }
    }

    /// Set ALDO1 output in millivolts, or disable it with `None`.
    pub fn set_aldo1_mv(&self, voltage_mv: Option<u16>) -> bool {
        self.set_ldo(Self::LDO_KIND_ALDO, 1, voltage_mv)
    }

    /// Set ALDO2 output in millivolts, or disable it with `None`.
    pub fn set_aldo2_mv(&self, voltage_mv: Option<u16>) -> bool {
        self.set_ldo(Self::LDO_KIND_ALDO, 2, voltage_mv)
    }

    /// Set ALDO3 output in millivolts, or disable it with `None`.
    pub fn set_aldo3_mv(&self, voltage_mv: Option<u16>) -> bool {
        self.set_ldo(Self::LDO_KIND_ALDO, 3, voltage_mv)
    }

    /// Set ALDO4 output in millivolts, or disable it with `None`.
    pub fn set_aldo4_mv(&self, voltage_mv: Option<u16>) -> bool {
        self.set_ldo(Self::LDO_KIND_ALDO, 4, voltage_mv)
    }

    /// Set BLDO1 output in millivolts, or disable it with `None`.
    pub fn set_bldo1_mv(&self, voltage_mv: Option<u16>) -> bool {
        self.set_ldo(Self::LDO_KIND_BLDO, 1, voltage_mv)
    }

    /// Set BLDO2 output in millivolts, or disable it with `None`.
    pub fn set_bldo2_mv(&self, voltage_mv: Option<u16>) -> bool {
        self.set_ldo(Self::LDO_KIND_BLDO, 2, voltage_mv)
    }

    /// Set DLDO1 output in millivolts, or disable it with `None`.
    pub fn set_dldo1_mv(&self, voltage_mv: Option<u16>) -> bool {
        self.set_ldo(Self::LDO_KIND_DLDO, 1, voltage_mv)
    }

    /// Set DLDO2 output in millivolts, or disable it with `None`.
    pub fn set_dldo2_mv(&self, voltage_mv: Option<u16>) -> bool {
        self.set_ldo(Self::LDO_KIND_DLDO, 2, voltage_mv)
    }

    /// Return whether ALDO1 is enabled.
    pub fn aldo1_enabled(&self) -> bool {
        self.ldo_enabled(Self::LDO_KIND_ALDO, 1)
    }

    /// Return whether ALDO2 is enabled.
    pub fn aldo2_enabled(&self) -> bool {
        self.ldo_enabled(Self::LDO_KIND_ALDO, 2)
    }

    /// Return whether ALDO3 is enabled.
    pub fn aldo3_enabled(&self) -> bool {
        self.ldo_enabled(Self::LDO_KIND_ALDO, 3)
    }

    /// Return whether ALDO4 is enabled.
    pub fn aldo4_enabled(&self) -> bool {
        self.ldo_enabled(Self::LDO_KIND_ALDO, 4)
    }

    /// Return whether BLDO1 is enabled.
    pub fn bldo1_enabled(&self) -> bool {
        self.ldo_enabled(Self::LDO_KIND_BLDO, 1)
    }

    /// Return whether BLDO2 is enabled.
    pub fn bldo2_enabled(&self) -> bool {
        self.ldo_enabled(Self::LDO_KIND_BLDO, 2)
    }

    /// Power the board off through the AXP2101.
    pub fn power_off(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_power_off() }
    }

    /// Enable or disable the AXP2101 ADC channels.
    pub fn set_adc_state(&self, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_set_adc_state(enable) }
    }

    /// Set the AXP2101 ADC rate.
    ///
    /// M5Unified currently accepts the value but does not program a register.
    pub fn set_adc_rate(&self, rate: u8) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_set_adc_rate(rate) }
    }

    /// Enable or disable AXP2101 backup output.
    ///
    /// M5Unified currently exposes this as a no-op on AXP2101.
    pub fn set_backup(&self, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_set_backup(enable) }
    }

    /// Return whether ACIN is present.
    pub fn is_acin(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_is_acin() }
    }

    /// Return whether VBUS is present.
    pub fn is_vbus(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_is_vbus() }
    }

    /// Return whether the AXP2101 reports a battery present.
    pub fn battery_present(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_get_bat_state() }
    }

    /// Return battery voltage in volts.
    pub fn battery_voltage_v(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_battery_voltage_v() }
    }

    /// Return battery discharge current in milliamps.
    pub fn battery_discharge_current_ma(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_battery_discharge_current_ma() }
    }

    /// Return battery charge current in milliamps.
    pub fn battery_charge_current_ma(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_battery_charge_current_ma() }
    }

    /// Return battery power in milliwatts.
    pub fn battery_power_mw(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_battery_power_mw() }
    }

    /// Return ACIN voltage in volts.
    pub fn acin_voltage_v(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_acin_voltage_v() }
    }

    /// Return ACIN current in milliamps.
    pub fn acin_current_ma(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_acin_current_ma() }
    }

    /// Return VBUS voltage in volts.
    pub fn vbus_voltage_v(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_vbus_voltage_v() }
    }

    /// Return VBUS current in milliamps.
    pub fn vbus_current_ma(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_vbus_current_ma() }
    }

    /// Return TS pin voltage in volts.
    pub fn ts_voltage_v(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_ts_voltage_v() }
    }

    /// Return APS voltage in volts.
    pub fn aps_voltage_v(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_aps_voltage_v() }
    }

    /// Return internal PMIC temperature in degrees Celsius.
    pub fn internal_temperature_c(&self) -> f32 {
        unsafe { m5unified_sys::m5u_power_axp2101_get_internal_temperature_c() }
    }

    /// Return the latched AXP2101 PEK press state.
    ///
    /// On supported PMICs this read clears the latched state.
    pub fn pek_press(&self) -> Axp2101PekPress {
        Axp2101PekPress::from_raw(unsafe { m5unified_sys::m5u_power_axp2101_get_pek_press() })
    }

    pub fn disable_irq(&self, mask: u64) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_disable_irq(mask) }
    }

    pub fn enable_irq(&self, mask: u64) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_enable_irq(mask) }
    }

    pub fn clear_irq_statuses(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_axp2101_clear_irq_statuses() }
    }

    pub fn irq_statuses(&self) -> Axp2101IrqStatus {
        Axp2101IrqStatus::from_status_register_order(unsafe {
            m5unified_sys::m5u_power_axp2101_get_irq_statuses()
        })
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Axp2101IrqStatus {
    pub raw: u64,
}

impl Axp2101IrqStatus {
    fn from_status_register_order(raw: u64) -> Self {
        Self {
            raw: ((raw >> 16) & 0xFF) | (raw & 0xFF00) | ((raw & 0xFF) << 16),
        }
    }

    pub fn contains(&self, mask: u64) -> bool {
        self.raw & mask != 0
    }

    pub fn is_empty(&self) -> bool {
        self.raw == 0
    }

    pub fn battery_work_under_temperature(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_WORK_UNDER_TEMP)
    }

    pub fn battery_work_over_temperature(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_WORK_OVER_TEMP)
    }

    pub fn battery_charger_under_temperature(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_CHG_UNDER_TEMP)
    }

    pub fn battery_charger_over_temperature(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_CHG_OVER_TEMP)
    }

    pub fn gauge_new_soc(&self) -> bool {
        self.contains(Axp2101::IRQ_GAUGE_NEW_SOC)
    }

    pub fn gauge_watchdog_timeout(&self) -> bool {
        self.contains(Axp2101::IRQ_GAUGE_WDT_TIMEOUT)
    }

    pub fn warning_level1(&self) -> bool {
        self.contains(Axp2101::IRQ_WARNING_LEVEL1)
    }

    pub fn warning_level2(&self) -> bool {
        self.contains(Axp2101::IRQ_WARNING_LEVEL2)
    }

    pub fn pkey_positive_edge(&self) -> bool {
        self.contains(Axp2101::IRQ_PKEY_POSITIVE_EDGE)
    }

    pub fn pkey_negative_edge(&self) -> bool {
        self.contains(Axp2101::IRQ_PKEY_NEGATIVE_EDGE)
    }

    pub fn pkey_long_press(&self) -> bool {
        self.contains(Axp2101::IRQ_PKEY_LONG_PRESS)
    }

    pub fn pkey_short_press(&self) -> bool {
        self.contains(Axp2101::IRQ_PKEY_SHORT_PRESS)
    }

    pub fn battery_remove(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_REMOVE)
    }

    pub fn battery_insert(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_INSERT)
    }

    pub fn vbus_insert(&self) -> bool {
        self.contains(Axp2101::IRQ_VBUS_INSERT)
    }

    pub fn vbus_remove(&self) -> bool {
        self.contains(Axp2101::IRQ_VBUS_REMOVE)
    }

    pub fn battery_over_voltage(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_OVER_VOLTAGE)
    }

    pub fn charger_timer_expired(&self) -> bool {
        self.contains(Axp2101::IRQ_CHARGER_TIMER)
    }

    pub fn die_over_temperature(&self) -> bool {
        self.contains(Axp2101::IRQ_DIE_OVER_TEMP)
    }

    pub fn battery_charge_start(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_CHG_START)
    }

    pub fn battery_charge_done(&self) -> bool {
        self.contains(Axp2101::IRQ_BAT_CHG_DONE)
    }

    pub fn batfet_over_current(&self) -> bool {
        self.contains(Axp2101::IRQ_BATFET_OVER_CURR)
    }

    pub fn ldo_over_current(&self) -> bool {
        self.contains(Axp2101::IRQ_LDO_OVER_CURR)
    }

    pub fn watchdog_expired(&self) -> bool {
        self.contains(Axp2101::IRQ_WDT_EXPIRE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axp2101_status_register_order_is_normalized_to_irq_masks() {
        let upstream_raw = (0b0000_0101_u64 << 16) | (0b1000_0000_u64 << 8) | 0b0100_0000_u64;
        let status = Axp2101IrqStatus::from_status_register_order(upstream_raw);

        assert!(status.battery_work_under_temperature());
        assert!(status.battery_charger_under_temperature());
        assert!(status.vbus_insert());
        assert!(status.ldo_over_current());
        assert!(!status.battery_work_over_temperature());
    }
}
