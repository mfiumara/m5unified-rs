use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Microphone")?;
    if m5.mic.try_begin().is_err() {
        m5.display.println("mic unavailable")?;
        return Ok(());
    }
    let mut samples = [0_i16; 256];
    finite_loop(&mut m5, 4, |m5, i| {
        let stats = m5.mic.try_stats(&mut samples).unwrap_or_default();
        let width = ((stats.rms_fraction() * m5.display.width() as f32) as i32)
            .clamp(0, m5.display.width());
        m5.display
            .fill_rect(0, 70 + i as i32 * 12, width, 8, colors::GREEN);
        m5.log.println(&format!(
            "mic rms={:.2}% peak={:.2}% mean={:.2}",
            stats.rms_percent(),
            stats.peak_percent(),
            stats.mean
        ))?;
        Ok(())
    })
}
