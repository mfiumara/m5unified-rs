use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Touch/DragDrop")?;
    finite_loop(&mut m5, 3, |m5, _| {
        if let Ok(point) = m5.touch.try_primary_point() {
            let position = point.position();
            m5.display
                .fill_circle(position.x, position.y, 8, colors::YELLOW);
        }
        Ok(())
    })
}
