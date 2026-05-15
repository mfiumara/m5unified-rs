use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Mic_FFT")?;
    let mut samples = [0_i16; 256];
    let rms = m5.mic.rms(&mut samples).unwrap_or_default();
    for bin in 0..16 {
        let height = ((rms as i32 / 128) + bin).clamp(1, 80);
        m5.display
            .fill_rect(8 + bin * 12, 160 - height, 8, height, colors::GREEN);
    }
    m5.display
        .println("placeholder spectrum until DSP crate is selected")?;
    Ok(())
}
