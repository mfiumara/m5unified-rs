use m5unified::M5Unified;
use m5unified_examples::{banner, unavailable_integration, ExampleResult, OptionalIntegration};

const AQUESTALK: OptionalIntegration = OptionalIntegration::new(
    "AquesTalk",
    "aquestalk",
    "AquesTalk licensing/binary support is external; synthesize PCM then call Speaker::play_i16.",
);

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Speak_with_AquesTalk")?;
    m5.speaker.begin();
    m5.speaker.tone(880, 80);
    unavailable_integration(&mut m5, AQUESTALK)
}
