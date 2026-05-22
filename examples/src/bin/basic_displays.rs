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

    for index in 0..m5.display_count() {
        if let Some(mut display) = m5.display(index) {
            let w = display.width();
            let h = display.height();
            let text_size = (display.height() / 60).max(1);
            display.clear();
            display.set_cursor(4, 4);
            display.set_text_size(text_size);
            display.set_text_color(colors::WHITE, colors::BLACK);
            display.println(&format!("No.{index}"))?;
            display.transaction(|display| {
                display.set_color(colors::CYAN);
                display.write_pixel(0, 0, colors::WHITE);
                display.draw_pixel(1, 1, colors::YELLOW);
                display.draw_line(0, h - 1, w - 1, 0, colors::RED);
                display.draw_rect(0, 0, w, h, colors::WHITE);
                display.fill_rect(2, 2, 8, 8, colors::DARK_GREY);
                display.draw_circle(w / 2, h / 2, 12, colors::GREEN);
                display.fill_circle(16, 16, 6, colors::CYAN);
            });
        }
    }

    Ok(())
}
