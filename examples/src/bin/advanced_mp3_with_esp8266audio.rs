use m5unified::M5Unified;
use m5unified_examples::{banner, unavailable_integration, ExampleResult, OptionalIntegration};

const MP3_DECODER: OptionalIntegration = OptionalIntegration::new(
    "ESP8266Audio MP3",
    "mp3-decoder",
    "ESP8266Audio is not part of m5unified; use an app-level decoder and feed PCM to Speaker.",
);

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/MP3_with_ESP8266Audio")?;
    unavailable_integration(&mut m5, MP3_DECODER)
}
