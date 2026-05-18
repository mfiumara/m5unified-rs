//! Power-management, PMIC, sleep, and AXP2101 helpers.
//!
//! The safe wrapper exposes battery and VBUS readings, output controls, sleep
//! timers, vibration, external-port power, and a focused AXP2101 IRQ surface.

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
pub struct Axp2101;

impl Axp2101 {
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
