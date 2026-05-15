use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Bluetooth_with_ESP32A2DP")?;
    m5.display
        .println("Bluetooth A2DP integration should provide PCM to Speaker")?;
    Ok(())
}
