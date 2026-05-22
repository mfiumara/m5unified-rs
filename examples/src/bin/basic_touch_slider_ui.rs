use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Touch/SliderUI")?;
    let y = m5.display.height() - 40;
    m5.display
        .draw_rect(20, y, m5.display.width() - 40, 20, colors::WHITE);
    let value = m5
        .touch
        .detail(0)
        .filter(|detail| detail.is_pressed())
        .map(|detail| detail.x)
        .unwrap_or(20);
    m5.display.fill_rect(
        20,
        y,
        value.clamp(0, m5.display.width() - 40),
        20,
        colors::CYAN,
    );
    Ok(())
}
