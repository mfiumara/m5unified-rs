use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Button")?;
    finite_loop(&mut m5, 5, |m5, _| {
        let a = m5.buttons.a();
        let b = m5.buttons.b();
        let c = m5.buttons.c();
        let color = if a.is_pressed() {
            colors::RED
        } else if b.was_pressed() {
            colors::GREEN
        } else if c.was_released() {
            colors::BLUE
        } else {
            colors::DARK_GREY
        };
        m5.display.fill_rect(0, 60, 160, 40, color);
        m5.display.set_cursor(0, 110);
        m5.display.println("A/B/C states sampled")?;
        Ok(())
    })
}
