use m5unified::{ExtPortMask, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Test/build_test")?;
    m5.display.println("build matrix smoke sample")?;

    let _ = m5.power.pmic_type();
    let _ = m5.power.battery_level();
    let _ = m5.power.battery_voltage_mv();
    let _ = m5.power.vbus_voltage_mv();
    let _ = m5.power.battery_current_ma();
    let _ = m5.power.charge_state();
    let _ = m5.power.is_charging();
    let _ = m5.power.key_state();

    m5.power.set_led(64);
    m5.power
        .set_ext_output(true, ExtPortMask::PA | ExtPortMask::PB1);
    m5.power.set_ext_power(true);
    m5.power.set_usb_output(true);
    m5.power.set_battery_charge(true);
    m5.power.set_charge_current_ma(500);
    m5.power.set_charge_voltage_mv(4_200);
    m5.power.set_vibration(0);

    Ok(())
}
