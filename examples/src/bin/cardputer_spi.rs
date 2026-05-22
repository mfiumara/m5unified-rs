use m5unified::{colors, Cardputer, CardputerSdPins, SpiBitOrder, SpiConfig, SpiMode};

fn main() -> Result<(), m5unified::Error> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::CYAN, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer SPI")?;

    let pins = CardputerSdPins::BUILTIN.spi_pins();
    let config = SpiConfig::default()
        .with_mode(SpiMode::Mode0)
        .with_bit_order(SpiBitOrder::MsbFirst);
    let ready = cardputer.spi.try_begin_with(pins).is_ok();
    let mut rx = [0_u8; 3];
    let transferred = cardputer
        .spi
        .try_transfer(&[0x9f, 0x00, 0x00], &mut rx, config);
    let wrote = cardputer.spi.try_write(&[0x00], config);
    cardputer.spi.end();

    cardputer.display.set_cursor(4, 32);
    cardputer.display.println(&format!("ready: {ready}"))?;
    cardputer.display.set_cursor(4, 56);
    cardputer
        .display
        .println(&format!("transfer: {}", transferred.is_ok()))?;
    cardputer.display.set_cursor(4, 80);
    cardputer.display.println(&format!("rx: {rx:02x?}"))?;
    cardputer.display.set_cursor(4, 104);
    cardputer
        .display
        .println(&format!("write: {}", wrote.is_ok()))?;

    Ok(())
}
