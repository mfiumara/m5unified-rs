use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use m5unified::{colors, M5Unified};

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    println!("hello-display boot: before M5Unified::begin");
    let _ = io::stdout().flush();

    let mut m5 = M5Unified::begin()?;
    println!(
        "M5Unified started: display={}x{}, rotation={}",
        m5.display.width(),
        m5.display.height(),
        m5.display.rotation()
    );
    let _ = io::stdout().flush();

    m5.display.set_brightness(255);
    m5.display.fill_screen(colors::RED);
    thread::sleep(Duration::from_millis(250));
    m5.display.fill_screen(colors::GREEN);
    thread::sleep(Duration::from_millis(250));
    m5.display.fill_screen(colors::BLUE);
    thread::sleep(Duration::from_millis(250));

    m5.display.fill_screen(colors::BLACK);
    m5.display.set_text_color(colors::GREEN, colors::BLACK);
    m5.display.set_text_size(2);
    m5.display.set_cursor(8, 16);
    m5.display.println("hello from rust")?;
    m5.display.set_cursor(8, 44);
    m5.display.println("M5Unified via C shim")?;

    let mut heartbeat_at = Instant::now();
    let mut heartbeat = 0u32;

    loop {
        m5.update();
        if heartbeat_at.elapsed() >= Duration::from_secs(1) {
            heartbeat = heartbeat.wrapping_add(1);
            println!(
                "heartbeat {heartbeat}: display={}x{}",
                m5.display.width(),
                m5.display.height()
            );
            let _ = io::stdout().flush();
            heartbeat_at = Instant::now();
        }
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
