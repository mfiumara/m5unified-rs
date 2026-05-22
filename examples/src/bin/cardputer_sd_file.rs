use m5unified::{colors, Cardputer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::WHITE, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer SD file")?;

    let mounted = cardputer.sd.begin();
    let dir_created = cardputer.sd.mkdir("/m5rs")?;
    let path = "/m5rs.txt";
    let renamed_path = "/m5rs-renamed.txt";
    let written = cardputer.sd.write_file(path, b"hello from rust")?;
    let appended = cardputer.sd.append_file(path, b"\ncardputer")?;
    let renamed = cardputer.sd.rename(path, renamed_path)?;
    let exists = cardputer.sd.exists(path)?;
    let renamed_exists = cardputer.sd.exists(renamed_path)?;
    let size = cardputer.sd.file_size(renamed_path)?;
    let root_is_dir = cardputer.sd.is_directory("/")?;
    let root_entries = cardputer.sd.list_dir("/", 4)?;
    let root_files = root_entries.iter().filter(|entry| entry.is_file()).count();

    let mut buffer = [0_u8; 64];
    let read = cardputer.sd.read_file(renamed_path, &mut buffer)?;
    let text = String::from_utf8_lossy(&buffer[..read]);
    let removed = cardputer.sd.remove_file(renamed_path)?;
    let dir_removed = cardputer.sd.rmdir("/m5rs")?;

    cardputer.display.set_cursor(4, 32);
    cardputer.display.println(&format!("mounted: {mounted}"))?;
    cardputer.display.set_cursor(4, 56);
    cardputer.display.println(&format!("exists: {exists}"))?;
    cardputer.display.set_cursor(4, 80);
    cardputer
        .display
        .println(&format!("write: {written}+{appended}"))?;
    cardputer.display.set_cursor(4, 104);
    cardputer
        .display
        .println(&format!("renamed: {renamed}/{renamed_exists}"))?;
    cardputer.display.set_cursor(4, 128);
    cardputer.display.println(&format!("read: {read}"))?;
    cardputer.display.set_cursor(4, 152);
    cardputer.display.println(text.trim_end())?;
    cardputer.display.set_cursor(4, 176);
    cardputer
        .display
        .println(&format!("dir: {dir_created}/{dir_removed}"))?;
    cardputer.display.set_cursor(4, 200);
    cardputer
        .display
        .println(&format!("size: {size} dir: {root_is_dir}"))?;
    cardputer.display.set_cursor(4, 224);
    cardputer
        .display
        .println(&format!("entries: {root_files}/{}", root_entries.len()))?;
    cardputer.display.set_cursor(4, 248);
    cardputer.display.println(&format!("removed: {removed}"))?;
    cardputer.sd.end();

    Ok(())
}
