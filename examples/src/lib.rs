//! Shared helpers for translated M5Unified Rust samples.

use m5unified::{colors, M5Unified};

pub type ExampleResult = Result<(), Box<dyn std::error::Error>>;

pub fn banner(m5: &mut M5Unified, title: &str) -> ExampleResult {
    m5.display.fill_screen(colors::BLACK);
    m5.display.set_cursor(0, 0);
    m5.display.set_text_size(2);
    m5.display.set_text_color(colors::WHITE, colors::BLACK);
    m5.display.println(title)?;
    m5.display.println("m5unified-rs translated sample")?;
    Ok(())
}

pub fn finite_loop<F>(m5: &mut M5Unified, iterations: usize, mut f: F) -> ExampleResult
where
    F: FnMut(&mut M5Unified, usize) -> ExampleResult,
{
    for i in 0..iterations {
        m5.update();
        f(m5, i)?;
        m5.delay_ms(10);
    }
    Ok(())
}
