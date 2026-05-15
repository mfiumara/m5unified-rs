use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/MP3_with_ESP8266Audio")?;
    m5.display
        .println("Rust sample boundary: feed decoded PCM to Speaker::play_i16")?;
    Ok(())
}
