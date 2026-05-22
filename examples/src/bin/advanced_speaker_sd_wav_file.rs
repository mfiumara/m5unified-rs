use m5unified::{
    sd_append_file, sd_card_type, sd_end, sd_exists, sd_file_size, sd_info, sd_list_dir, sd_mkdir,
    sd_read_file, sd_remove_file, sd_rename, sd_rmdir, sd_try_begin, sd_write_file, M5Unified,
};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/Speaker_SD_wav_file")?;
    if sd_try_begin().is_ok() {
        let info = sd_info();
        let exists = sd_exists("/music.wav")?;
        let size = sd_file_size("/music.wav")?;
        let dir_created = sd_mkdir("/m5rs")?;
        let written = sd_write_file("/m5rs-sd-smoke.txt", b"hello")?;
        let appended = sd_append_file("/m5rs-sd-smoke.txt", b"\n")?;
        let renamed = sd_rename("/m5rs-sd-smoke.txt", "/m5rs-sd-renamed.txt")?;
        let entries = sd_list_dir("/", 4)?;
        let wav_entries = entries
            .iter()
            .filter(|entry| entry.is_file() && entry.extension() == Some("wav"))
            .count();
        let mut buffer = [0_u8; 16];
        let read = sd_read_file("/m5rs-sd-renamed.txt", &mut buffer)?;
        let removed = sd_remove_file("/m5rs-sd-renamed.txt")?;
        let dir_removed = sd_rmdir("/m5rs")?;
        m5.display
            .println("SD mounted; stream WAV frames to speaker.play_i16()")?;
        m5.display.println(&format!(
            "card: {:?} used: {}/{}",
            sd_card_type(),
            info.used_bytes,
            info.total_bytes
        ))?;
        m5.display
            .println(&format!("music.wav: {exists} size: {size}"))?;
        m5.display.println(&format!(
            "smoke: {written}+{appended}/{renamed}/{read}/{removed}"
        ))?;
        m5.display.println(&format!(
            "dir: {dir_created}/{dir_removed} wav: {wav_entries}/{}",
            entries.len()
        ))?;
        sd_end();
    } else {
        m5.display.println("SD unavailable in host stub")?;
    }
    Ok(())
}
