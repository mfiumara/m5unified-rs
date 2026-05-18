use m5unified::M5Unified;
use m5unified_examples::{banner, unavailable_integration, ExampleResult, OptionalIntegration};

const WEBRADIO: OptionalIntegration = OptionalIntegration::new(
    "ESP8266Audio WebRadio",
    "webradio-mp3",
    "Networking and MP3 decoding are app-level dependencies; stream PCM to Speaker.",
);

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/WebRadio_with_ESP8266Audio")?;
    unavailable_integration(&mut m5, WEBRADIO)
}
