use m5unified::{sd_begin, sd_end, sd_is_mounted, M5Unified, SD_MOUNT_PATH};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Speaker_SD_wav_file")?;
    if sd_begin() {
        let state = if sd_is_mounted() {
            "mounted"
        } else {
            "not mounted"
        };
        m5.display.println(&format!("{SD_MOUNT_PATH} {state}"))?;
        sd_end();
    } else {
        m5.display.println("SD unavailable in host stub")?;
    }
    Ok(())
}
