use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Touch/DragDrop")?;
    finite_loop(&mut m5, 3, |m5, _| {
        for detail in m5.touch.details() {
            if detail.is_dragging() {
                m5.display.draw_line(
                    detail.prev_x,
                    detail.prev_y,
                    detail.x,
                    detail.y,
                    colors::YELLOW,
                );
            } else if detail.is_pressed() {
                m5.display.fill_circle(detail.x, detail.y, 8, colors::CYAN);
            }
            if detail.was_released() {
                let x = detail.base_x.min(detail.x);
                let y = detail.base_y.min(detail.y);
                m5.display.draw_rect(
                    x,
                    y,
                    detail.distance_x().abs(),
                    detail.distance_y().abs(),
                    colors::WHITE,
                );
            }
        }
        Ok(())
    })
}
