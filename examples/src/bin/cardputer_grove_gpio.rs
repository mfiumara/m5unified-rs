use m5unified::{colors, Cardputer, GpioMode, GrovePin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::WHITE, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer GPIO")?;

    let mode_set = cardputer
        .grove
        .gpio_try_pin_mode(GrovePin::G1, GpioMode::InputPullup);
    let g1 = cardputer.grove.gpio_try_read(GrovePin::G1);

    cardputer.display.set_cursor(4, 32);
    cardputer
        .display
        .println(&format!("mode: {}", mode_set.is_ok()))?;
    cardputer.display.set_cursor(4, 56);
    cardputer.display.println(&format!("G1: {g1:?}"))?;

    Ok(())
}
