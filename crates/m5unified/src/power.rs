#[derive(Debug)]
pub struct Power;

impl Power {
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

    pub fn is_charging(&self) -> bool {
        unsafe { m5unified_sys::m5u_power_is_charging() }
    }

    pub fn axp2101(&self) -> Axp2101 {
        Axp2101
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
