use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Displays")?;
    let w = m5.display.width();
    let h = m5.display.height();
    m5.display.draw_rect(0, 0, w, h, colors::WHITE);
    m5.display.draw_line(0, 0, w - 1, h - 1, colors::RED);
    m5.display.draw_line(w - 1, 0, 0, h - 1, colors::GREEN);
    m5.display.fill_circle(w / 2, h / 2, 24, colors::BLUE);
    m5.display.set_cursor(8, h / 2 + 36);
    m5.display.println("Shapes through safe Rust API")?;
    Ok(())
}
