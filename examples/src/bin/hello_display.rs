use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use m5unified::{colors, M5Unified};
use m5unified_examples::ExampleResult;

#[cfg(target_os = "espidf")]
fn link_platform_patches() {
    esp_idf_sys::link_patches();
}

#[cfg(not(target_os = "espidf"))]
fn link_platform_patches() {}

fn draw_boot_text(m5: &mut M5Unified) -> ExampleResult {
    m5.display.fill_screen(colors::BLACK);
    m5.display.set_text_color(colors::GREEN, colors::BLACK);
    m5.display.set_text_size(2);
    m5.display.set_cursor(8, 16);
    m5.display.println("hello from rust")?;

    let line_gap = (m5.display.font_height() / 2).max(4);
    let second_line_y = m5.display.cursor_y() + line_gap;
    m5.display.set_cursor(8, second_line_y);
    m5.display.println("M5Unified via examples")?;
    Ok(())
}

fn main() -> ExampleResult {
    link_platform_patches();

    println!("hello-display example boot: before M5Unified::begin");
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
    m5.delay_ms(250);
    m5.display.fill_screen(colors::GREEN);
    m5.delay_ms(250);
    m5.display.fill_screen(colors::BLUE);
    m5.delay_ms(250);

    draw_boot_text(&mut m5)?;

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
        m5.delay_ms(50);
    }
}
