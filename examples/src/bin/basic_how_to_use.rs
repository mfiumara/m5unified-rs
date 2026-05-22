use m5unified::{colors, DisplayKind, EpdMode, LedColor, M5Unified, M5UnifiedConfig, PinName};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let cfg = M5UnifiedConfig {
        led_brightness: 64,
        external_imu: true,
        external_rtc: true,
        ..M5UnifiedConfig::default()
    };
    let mut m5 = M5Unified::begin_with_config(&cfg)?;
    banner(&mut m5, "Basic/HowToUse")?;
    m5.display
        .println("Call M5Unified::begin(), update(), then use modules.")?;
    m5.set_touch_button_height(32);
    m5.set_log_display_index(0);
    let _ = m5.set_primary_display_type(DisplayKind::ModuleDisplay);
    m5.display.set_epd_mode(EpdMode::Fastest);
    let _board = m5.board();
    let _port_a_sda = m5.get_pin(PinName::PORT_A_SDA);
    for index in 0..m5.display_count() {
        if let Some(mut display) = m5.display(index) {
            display.transaction(|display| {
                display.write_pixel(0, 0, colors::WHITE);
                display.draw_pixel(1, 1, colors::GREEN);
            });
            display.print(&format!("Display {index}\n"))?;
        }
    }
    if m5.led.is_enabled() {
        m5.led.set_brightness(64);
        m5.led.set_all_color(LedColor::BLUE);
    }
    finite_loop(&mut m5, 3, |m5, i| {
        m5.display.set_cursor(0, 48 + (i as i32 * 16));
        m5.display.set_text_color(colors::GREEN, colors::BLACK);
        m5.display.println(&format!("tick {i}"))?;
        if m5.led.is_enabled() {
            let color = match i % 3 {
                0 => LedColor::RED,
                1 => LedColor::GREEN,
                _ => LedColor::BLUE,
            };
            m5.led.set_all_color(color);
        }
        Ok(())
    })
}
