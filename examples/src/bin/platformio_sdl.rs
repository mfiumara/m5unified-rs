use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "PlatformIO_SDL")?;
    m5.display
        .println("Host stubs provide SDL-like compile-time smoke coverage")?;
    Ok(())
}
