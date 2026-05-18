use m5unified::M5Unified;
use m5unified_examples::{banner, unavailable_integration, ExampleResult, OptionalIntegration};

const A2DP: OptionalIntegration = OptionalIntegration::new(
    "Bluetooth A2DP",
    "bluetooth-a2dp",
    "Bluetooth stack selection belongs in the app crate; decoded PCM should call Speaker::play_i16.",
);

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Bluetooth_with_ESP32A2DP")?;
    unavailable_integration(&mut m5, A2DP)
}
