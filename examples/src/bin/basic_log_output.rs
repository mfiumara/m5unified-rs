use m5unified::{LogLevel, LogTarget, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/LogOutput")?;
    m5.display.set_text_scroll(true);
    m5.log.set_log_level(LogTarget::Serial, LogLevel::Verbose);
    m5.log.set_log_level(LogTarget::Display, LogLevel::Debug);
    m5.log.set_enable_color(LogTarget::Serial, true);
    m5.log.set_enable_color(LogTarget::Display, true);
    let _ = m5.log.set_suffix(LogTarget::Serial, "\n")?;
    let _ = m5.log.set_suffix(LogTarget::Display, "\n")?;
    m5.log.println("log line from Rust wrapper")?;
    m5.log
        .log(LogLevel::Debug, "debug line from Rust wrapper")?;
    m5.display.println("also displayed on screen")?;
    Ok(())
}
