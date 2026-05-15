use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Test/build_test")?;
    m5.display.println("build matrix smoke sample")?;
    Ok(())
}
