use m5unified::{Axp2101IrqMask, M5Unified};
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
    let mask = Axp2101IrqMask::ALL;
    m5.display
        .println(&format!("irq enable={}", axp.try_enable_irq(mask).is_ok()))?;
    let statuses = axp.irq_statuses();
    m5.display.println(&format!(
        "irq raw=0x{:016x} any={}",
        statuses.raw(),
        statuses.any()
    ))?;
    m5.display.println(&format!(
        "irq clear={}",
        axp.try_clear_irq_statuses().is_ok()
    ))?;
    let _ = axp.try_disable_irq(mask);
    Ok(())
}
