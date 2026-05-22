use m5unified::{colors, Cardputer, GrovePin};

fn main() -> Result<(), m5unified::Error> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::CYAN, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer ADC")?;

    let g1_raw = cardputer.grove.analog_try_read(GrovePin::G1);
    let g1_mv = cardputer.grove.analog_try_read_millivolts(GrovePin::G1);
    let pwm_resolution = cardputer.grove.analog_try_write_resolution(GrovePin::G2, 8);
    let pwm_frequency = cardputer
        .grove
        .analog_try_write_frequency(GrovePin::G2, 1_000);
    let pwm_written = cardputer.grove.analog_try_write(GrovePin::G2, 128);

    cardputer.display.set_cursor(4, 32);
    cardputer.display.println(&format!("G1 raw: {g1_raw:?}"))?;
    cardputer.display.set_cursor(4, 56);
    cardputer.display.println(&format!("G1 mV: {g1_mv:?}"))?;
    cardputer.display.set_cursor(4, 80);
    cardputer
        .display
        .println(&format!("PWM res: {}", pwm_resolution.is_ok()))?;
    cardputer.display.set_cursor(4, 104);
    cardputer
        .display
        .println(&format!("PWM freq: {}", pwm_frequency.is_ok()))?;
    cardputer.display.set_cursor(4, 128);
    cardputer
        .display
        .println(&format!("PWM write: {}", pwm_written.is_ok()))?;

    Ok(())
}
