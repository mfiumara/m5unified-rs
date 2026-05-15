use m5unified::M5Unified;
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
    Ok(())
}
