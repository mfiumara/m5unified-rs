use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Advanced/WebRadio_with_ESP8266Audio")?;
    m5.display
        .println("Network streaming belongs in app crate; audio sink is Speaker")?;
    Ok(())
}
