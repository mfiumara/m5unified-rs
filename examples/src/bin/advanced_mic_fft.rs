use m5unified::{colors, DisplayFont, EpdMode, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Mic_FFT")?;
    m5.display.set_font(DisplayFont::Ascii8x16);
    m5.display.set_epd_mode(EpdMode::Fastest);
    let mut cfg = m5.mic.config();
    cfg.sample_rate = 16_000;
    cfg.dma_buf_count = 3;
    cfg.dma_buf_len = 256;
    cfg.over_sampling = 1;
    cfg.noise_filter_level = 0;
    cfg.magnification = if cfg.use_adc { 16 } else { 1 };
    m5.mic.set_config(cfg)?;
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
