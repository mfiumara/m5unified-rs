#[derive(Debug)]
pub struct Power;

impl Power {
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

    /// Set vibration motor strength where the board supports it.
    pub fn set_vibration(&mut self, level: u8) {
        unsafe { m5unified_sys::m5u_power_set_vibration(level) }
    }

    pub fn axp2101(&self) -> Axp2101 {
        Axp2101
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
    pub const IRQ_BAT_CHG_UNDER_TEMP: u64 = 1 << 2;
    pub const IRQ_BAT_CHG_OVER_TEMP: u64 = 1 << 3;
    pub const IRQ_VBUS_REMOVE: u64 = 1 << 14;
    pub const IRQ_VBUS_INSERT: u64 = 1 << 15;

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
        Axp2101IrqStatus {
            raw: unsafe { m5unified_sys::m5u_power_axp2101_get_irq_statuses() },
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Axp2101IrqStatus {
    pub raw: u64,
}

impl Axp2101IrqStatus {
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
