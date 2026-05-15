use m5unified::{colors, M5Unified};
use m5unified_examples::{banner, finite_loop, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/HowToUse")?;
    m5.display
        .println("Call M5Unified::begin(), update(), then use modules.")?;
    finite_loop(&mut m5, 3, |m5, i| {
        m5.display.set_cursor(0, 48 + (i as i32 * 16));
        m5.display.set_text_color(colors::GREEN, colors::BLACK);
        m5.display.println(&format!("tick {i}"))?;
        Ok(())
    })
}
