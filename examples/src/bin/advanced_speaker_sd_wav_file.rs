use m5unified::{sd_begin, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Speaker_SD_wav_file")?;
    if sd_begin() {
        m5.display
            .println("SD mounted; stream WAV frames to speaker.play_i16()")?;
    } else {
        m5.display.println("SD unavailable in host stub")?;
    }
    Ok(())
}
