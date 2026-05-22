use m5unified::{LogLevel, LogTarget, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/LogOutput")?;
    m5.try_set_log_display(0)?;
    m5.log.set_level(LogTarget::Serial, LogLevel::Debug);
    m5.log.set_enable_color(LogTarget::Display, false);
    m5.log.set_suffix(LogTarget::Serial, "\n")?;
    m5.log.println("log line from Rust wrapper")?;
    m5.log
        .log(LogLevel::Info, "info log through Rust wrapper")?;
    m5.display.println("also displayed on screen")?;
    Ok(())
}
