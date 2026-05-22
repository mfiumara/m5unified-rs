use m5unified::{M5Unified, SdCard};
use m5unified_examples::{banner, ExampleResult};

const WAV_FILE: &str = "sample.wav";

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Speaker_SD_wav_file")?;
    m5.speaker.begin();

    match SdCard::mount() {
        Ok(sd) => {
            m5.display
                .println(&format!("{} mounted", sd.mount_path()))?;
            match sd.read(WAV_FILE) {
                Ok(data) => {
                    if m5.speaker.play_wav(&data) {
                        m5.display.println(&format!("playing {WAV_FILE}"))?;
                    } else {
                        m5.display.println("speaker rejected WAV data")?;
                    }
                }
                Err(err) => {
                    m5.display.println(&format!("{WAV_FILE}: {err}"))?;
                }
            }
        }
        Err(_) => {
            m5.display.println("SD unavailable in host stub")?;
        }
    }

    Ok(())
}
