use core::f32::consts::PI;

use m5unified::{colors, DisplayFont, EpdMode, M5Unified, RecordingOptions};
use m5unified_examples::{banner, ExampleResult};

const SAMPLE_RATE_HZ: u32 = 16_000;
const SAMPLE_COUNT: usize = 256;
const SPECTRUM_BINS: usize = 16;
const BAR_WIDTH: i32 = 8;
const BAR_GAP: i32 = 4;
const GRAPH_BOTTOM: i32 = 160;
const GRAPH_HEIGHT: i32 = 88;

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Mic_FFT")?;
    m5.display.set_font(DisplayFont::Ascii8x16);
    m5.display.set_epd_mode(EpdMode::Fastest);
    let mut cfg = m5.mic.config();
    cfg.sample_rate = SAMPLE_RATE_HZ;
    cfg.dma_buf_count = 3;
    cfg.dma_buf_len = SAMPLE_COUNT;
    cfg.over_sampling = 1;
    cfg.noise_filter_level = 0;
    cfg.magnification = if cfg.use_adc { 16 } else { 1 };
    m5.mic.set_config(cfg)?;

    let mut samples = [0_i16; SAMPLE_COUNT];
    if m5.mic.record_i16_with_options(
        &mut samples,
        RecordingOptions {
            sample_rate_hz: SAMPLE_RATE_HZ,
            stereo: false,
        },
    ) {
        let spectrum = spectrum_bins(&samples);
        draw_spectrum(&mut m5, &spectrum);
    } else {
        m5.display.println("microphone unavailable")?;
    }

    Ok(())
}

fn spectrum_bins(samples: &[i16]) -> [f32; SPECTRUM_BINS] {
    let mut bins = [0.0; SPECTRUM_BINS];
    if samples.is_empty() {
        return bins;
    }

    let sample_count = samples.len() as f32;
    for (bin_index, output) in bins.iter_mut().enumerate() {
        let harmonic = (bin_index + 1) as f32;
        let mut real = 0.0;
        let mut imag = 0.0;

        for (sample_index, &sample) in samples.iter().enumerate() {
            let phase = 2.0 * PI * harmonic * sample_index as f32 / sample_count;
            let (sin, cos) = phase.sin_cos();
            let value = sample as f32;
            real += value * cos;
            imag -= value * sin;
        }

        *output = real.hypot(imag) / sample_count;
    }

    bins
}

fn draw_spectrum(m5: &mut M5Unified, spectrum: &[f32; SPECTRUM_BINS]) {
    let peak = spectrum.iter().copied().fold(1.0_f32, f32::max);

    for (index, magnitude) in spectrum.iter().copied().enumerate() {
        let scaled = (magnitude / peak).sqrt();
        let height = (scaled * GRAPH_HEIGHT as f32)
            .round()
            .clamp(1.0, GRAPH_HEIGHT as f32) as i32;
        let x = 8 + index as i32 * (BAR_WIDTH + BAR_GAP);
        let color = if index < SPECTRUM_BINS / 3 {
            colors::CYAN
        } else if index < SPECTRUM_BINS * 2 / 3 {
            colors::GREEN
        } else {
            colors::YELLOW
        };
        m5.display
            .fill_rect(x, GRAPH_BOTTOM - height, BAR_WIDTH, height, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silent_samples_have_zero_spectrum() {
        let samples = [0_i16; SAMPLE_COUNT];
        assert!(spectrum_bins(&samples).iter().all(|&bin| bin == 0.0));
    }

    #[test]
    fn sine_wave_peaks_at_matching_bin() {
        let harmonic = 3.0;
        let mut samples = [0_i16; SAMPLE_COUNT];
        for (index, sample) in samples.iter_mut().enumerate() {
            let phase = 2.0 * PI * harmonic * index as f32 / SAMPLE_COUNT as f32;
            *sample = (phase.sin() * 1000.0) as i16;
        }

        let spectrum = spectrum_bins(&samples);
        let (peak_index, peak) = spectrum
            .iter()
            .copied()
            .enumerate()
            .max_by(|left, right| left.1.total_cmp(&right.1))
            .expect("spectrum should not be empty");

        assert_eq!(peak_index, 2);
        assert!(peak > spectrum[0] * 4.0);
    }
}
