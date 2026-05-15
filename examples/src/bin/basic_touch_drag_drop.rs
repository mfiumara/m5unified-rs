use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Touch/DragDrop")?;
    finite_loop(&mut m5, 3, |m5, _| {
        for point in m5.touch.points() {
            m5.display.fill_circle(point.x, point.y, 8, colors::YELLOW);
        }
        Ok(())
    })
}
