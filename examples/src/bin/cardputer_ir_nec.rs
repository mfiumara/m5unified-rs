use m5unified::{colors, Cardputer, NecFrame};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::WHITE, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer IR NEC")?;

    let ready = cardputer.ir.try_begin().is_ok();
    let sent = cardputer
        .ir
        .try_send_nec(NecFrame::new(0x1111, 0x34))
        .is_ok();

    cardputer.display.set_cursor(4, 32);
    cardputer.display.println(&format!("ready: {ready}"))?;
    cardputer.display.set_cursor(4, 56);
    cardputer.display.println(&format!("sent: {sent}"))?;

    Ok(())
}
