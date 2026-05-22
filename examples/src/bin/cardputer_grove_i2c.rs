use m5unified::{colors, Cardputer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::WHITE, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer Grove")?;

    let ready = cardputer.grove.i2c_try_begin().is_ok();
    let probe_address = m5unified::I2cAddress::new(0x42).expect("valid 7-bit address");
    let mut buffer = [0_u8; 2];
    let raw_write = cardputer.grove.i2c_try_write(probe_address, &[0x00]);
    let raw_read = cardputer.grove.i2c_try_read(probe_address, &mut buffer);
    let devices = if ready {
        cardputer.grove.i2c_scan()
    } else {
        Vec::new()
    };

    cardputer.display.set_cursor(4, 32);
    cardputer.display.println(&format!("ready: {ready}"))?;
    cardputer.display.set_cursor(4, 56);
    cardputer
        .display
        .println(&format!("devices: {}", devices.len()))?;
    cardputer.display.set_cursor(4, 80);
    cardputer
        .display
        .println(&format!("raw: {}/{raw_read:?}", raw_write.is_ok()))?;
    cardputer.grove.i2c_end();

    Ok(())
}
