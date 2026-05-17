use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Microphone")?;
    let mut cfg = m5.mic.config();
    cfg.noise_filter_level = cfg.noise_filter_level.wrapping_add(8);
    cfg.sample_rate = 16_000;
    m5.mic.set_config(cfg)?;
    if !m5.mic.begin() {
        m5.display.println("mic unavailable")?;
        return Ok(());
    }
    let mut samples = [0_i16; 256];
    finite_loop(&mut m5, 4, |m5, i| {
        let rms = m5.mic.rms(&mut samples).unwrap_or_default();
        let width = (rms as i32).clamp(0, m5.display.width());
        m5.display
            .fill_rect(0, 70 + i as i32 * 12, width, 8, colors::GREEN);
        m5.log.println(&format!("mic rms={rms:.2}"))?;
        Ok(())
    })
}
