use std::{thread, time::Duration};

use anyhow::Result;
use m5unified::{colors, M5Unified};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let mut m5 = M5Unified::begin()?;
    m5.display.fill_screen(colors::BLACK);
    m5.display.set_text_color(colors::GREEN, colors::BLACK);
    m5.display.set_text_size(2);
    m5.display.set_cursor(8, 16);
    m5.display.println("hello from rust")?;
    m5.display.set_cursor(8, 44);
    m5.display.println("M5Unified via C shim")?;

    loop {
        m5.update();
        if m5.buttons.a().was_pressed() {
            m5.display.fill_screen(colors::BLUE);
            m5.display.set_cursor(8, 16);
            m5.display.println("Button A pressed")?;
        }
        if m5.buttons.b().was_pressed() {
            m5.display.fill_screen(colors::GREEN);
            m5.display.set_cursor(8, 16);
            m5.display.println("Button B pressed")?;
        }
        thread::sleep(Duration::from_millis(50));
    }
}
