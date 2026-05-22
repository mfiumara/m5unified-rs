use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Speaker")?;
    if m5.speaker.try_begin().is_ok() {
        m5.speaker.set_volume(96);
        for note in [440, 494, 523, 587, 659] {
            let _ = m5.speaker.try_tone(note, 120);
        }
        let silence = [0_i16; 128];
        let _ = m5.speaker.try_play_i16(&silence, 16_000);
        m5.display.println("tone sequence queued")?;
    } else {
        m5.display.println("speaker unavailable")?;
    }
    Ok(())
}
