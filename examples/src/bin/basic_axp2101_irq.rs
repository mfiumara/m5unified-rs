use m5unified::{Axp2101, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Axp2101/IRQ")?;
    m5.display
        .println(&format!("battery={:?}%", m5.power.battery_level()))?;
    m5.display
        .println(&format!("voltage={:?}mV", m5.power.battery_voltage_mv()))?;
    m5.display
        .println(&format!("charging={}", m5.power.is_charging()))?;

    let axp = m5.power.axp2101();
    m5.display
        .println(&format!("axp charge={:?}", axp.charge_status()))?;
    m5.display
        .println(&format!("axp batt={:?}%", axp.battery_level()))?;
    m5.display
        .println(&format!("axp batt={:.3}V", axp.battery_voltage_v()))?;
    m5.display.println(&format!("axp vbus={}", axp.is_vbus()))?;
    m5.display
        .println(&format!("axp pek={:?}", axp.pek_press()))?;

    let irq_mask = Axp2101::IRQ_BAT_CHG_UNDER_TEMP
        | Axp2101::IRQ_BAT_CHG_OVER_TEMP
        | Axp2101::IRQ_VBUS_INSERT
        | Axp2101::IRQ_VBUS_REMOVE;
    let configured =
        axp.disable_irq(Axp2101::IRQ_ALL) && axp.clear_irq_statuses() && axp.enable_irq(irq_mask);
    m5.display
        .println(&format!("axp2101 irq configured={configured}"))?;

    let status = axp.irq_statuses();
    if status.battery_charger_under_temperature() {
        m5.display.println("BatUnderTempCharge")?;
    }
    if status.battery_charger_over_temperature() {
        m5.display.println("BatOverTempCharge")?;
    }
    if status.vbus_insert() {
        m5.display.println("Usb inserted")?;
    }
    if status.vbus_remove() {
        m5.display.println("Usb removed")?;
    }

    Ok(())
}
