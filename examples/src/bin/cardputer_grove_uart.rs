use m5unified::{colors, Cardputer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::WHITE, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer UART")?;

    let ready = cardputer.grove.uart_try_begin(115_200).is_ok();
    let written = cardputer.grove.uart_try_write_str("hello from cardputer");
    let newline = cardputer.grove.uart_try_write_all(b"\r\n");
    cardputer.grove.uart_flush();
    let available = cardputer.grove.uart_available();

    cardputer.display.set_cursor(4, 32);
    cardputer.display.println(&format!("ready: {ready}"))?;
    cardputer.display.set_cursor(4, 56);
    cardputer
        .display
        .println(&format!("tx: {written:?}+{}", newline.is_ok()))?;
    cardputer.display.set_cursor(4, 80);
    cardputer.display.println(&format!("rx: {available}"))?;
    cardputer.grove.uart_end();

    Ok(())
}
