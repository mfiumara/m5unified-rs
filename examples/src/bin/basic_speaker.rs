use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Speaker")?;
    if m5.speaker.begin() {
        m5.speaker.set_volume(96);
        for note in [440, 494, 523, 587, 659] {
            m5.speaker.tone(note, 120);
        }
        let silence = [0_i16; 128];
        m5.speaker.play_i16(&silence, 16_000);
        m5.display.println("tone sequence queued")?;
    } else {
        m5.display.println("speaker unavailable")?;
    }
    Ok(())
}
