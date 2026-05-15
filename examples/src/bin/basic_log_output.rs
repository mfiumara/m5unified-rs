use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/LogOutput")?;
    m5.log.println("log line from Rust wrapper")?;
    m5.display.println("also displayed on screen")?;
    Ok(())
}
