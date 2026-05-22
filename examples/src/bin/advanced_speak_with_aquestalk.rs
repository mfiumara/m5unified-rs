use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Speak_with_AquesTalk")?;
    let _ = m5.speaker.try_begin();
    let _ = m5.speaker.try_tone(880, 80);
    m5.display
        .println("TTS engines can synthesize PCM then call Speaker::play_i16")?;
    Ok(())
}
