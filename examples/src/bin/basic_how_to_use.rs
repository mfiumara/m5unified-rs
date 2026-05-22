use m5unified::{
    analog_try_read, analog_try_read_millivolts, analog_try_write, analog_try_write_frequency,
    analog_try_write_resolution, colors, gpio_try_pin_mode, gpio_try_read, i2c_end, i2c_scan,
    i2c_try_begin, rgb, spi_end, spi_transfer_byte, spi_try_begin, spi_try_write, uart_end,
    uart_flush, uart_try_begin, uart_try_write_str_all, GpioMode, GpioPin, I2cPins, M5Unified,
    PinName, SpiConfig, SpiPins, UartPins,
};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/HowToUse")?;
    let _ = m5.led.try_begin();
    let _ = m5.led.try_set_brightness(24);
    let _ = m5.led.try_set_auto_display(true);
    let _ = m5.led.try_set_color(0, rgb::GREEN);
    let _ = m5.led.try_set_all_color(rgb::GREEN);
    m5.display
        .println("Call M5Unified::begin(), update(), then use modules.")?;
    m5.display.println(&format!("board: {:?}", m5.board()))?;
    m5.display.println(&format!(
        "leds: {} enabled: {}",
        m5.led.count(),
        m5.led.is_enabled()
    ))?;
    m5.display.println(&format!(
        "millis: {} update: {}",
        m5.millis(),
        m5.update_msec()
    ))?;
    m5.display.println(&format!(
        "Port A SDA pin: {:?}",
        m5.pin(PinName::PORT_A_SDA)
    ))?;
    let i2c_ready = match I2cPins::port_a() {
        Some(pins) => i2c_try_begin(pins).is_ok(),
        None => false,
    };
    let i2c_devices = if i2c_ready { i2c_scan().len() } else { 0 };
    m5.display
        .println(&format!("i2c: {i2c_ready} devices: {i2c_devices}"))?;
    i2c_end();
    let uart_ready = match UartPins::port_c() {
        Some(pins) => uart_try_begin(pins, 115_200).is_ok(),
        None => false,
    };
    if uart_ready {
        let _ = uart_try_write_str_all("m5unified-rs\r\n");
        uart_flush();
    }
    m5.display.println(&format!("uart: {uart_ready}"))?;
    uart_end();
    let gpio_status = match GpioPin::from_pin_name(PinName::PORT_B_IN) {
        Some(pin) => {
            let configured = gpio_try_pin_mode(pin, GpioMode::Input).is_ok();
            format!(
                "gpio: {configured} {:?} adc: {:?}/{:?}mV",
                gpio_try_read(pin),
                analog_try_read(pin),
                analog_try_read_millivolts(pin)
            )
        }
        None => "gpio: unavailable".to_owned(),
    };
    m5.display.println(&gpio_status)?;
    let pwm_ready = match GpioPin::from_pin_name(PinName::PORT_B_OUT) {
        Some(pin) => {
            let configured = analog_try_write_frequency(pin, 1_000).is_ok()
                && analog_try_write_resolution(pin, 8).is_ok();
            configured && analog_try_write(pin, 64).is_ok()
        }
        None => false,
    };
    m5.display.println(&format!("pwm: {pwm_ready}"))?;
    let spi_ready = match SpiPins::sd() {
        Some(pins) => spi_try_begin(pins).is_ok(),
        None => false,
    };
    let spi_echo = if spi_ready {
        let _ = spi_try_write(&[0xff], SpiConfig::default());
        spi_transfer_byte(0xff, SpiConfig::default())
    } else {
        0
    };
    m5.display
        .println(&format!("spi: {spi_ready} echo: {spi_echo}"))?;
    spi_end();
    finite_loop(&mut m5, 3, |m5, i| {
        m5.display.set_cursor(0, 48 + (i as i32 * 16));
        m5.display.set_text_color(colors::GREEN, colors::BLACK);
        m5.display.println(&format!("tick {i}"))?;
        Ok(())
    })?;
    let _ = m5.led.try_off();
    Ok(())
}
