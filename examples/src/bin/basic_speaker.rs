use m5unified::{DisplayFont, EpdMode, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Speaker")?;
    m5.display.set_font(DisplayFont::DejaVu18);
    m5.display.set_epd_mode(EpdMode::Fastest);
    let mut cfg = m5.speaker.config();
    if !cfg.use_dac && !cfg.buzzer {
        cfg.sample_rate = 192_000;
    }
    cfg.magnification = 16;
    m5.speaker.set_config(cfg)?;
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
