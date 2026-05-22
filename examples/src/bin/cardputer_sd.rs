use m5unified::{colors, Cardputer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::WHITE, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer SD")?;

    let mounted = cardputer.sd.try_begin().is_ok();
    let info = cardputer.sd.info();

    cardputer.display.set_cursor(4, 32);
    cardputer.display.println(&format!("mounted: {mounted}"))?;
    cardputer.display.set_cursor(4, 56);
    cardputer
        .display
        .println(&format!("type: {:?}", cardputer.sd.card_type()))?;
    cardputer.display.set_cursor(4, 80);
    cardputer
        .display
        .println(&format!("size: {} MiB", info.size_mebibytes()))?;

    Ok(())
}
