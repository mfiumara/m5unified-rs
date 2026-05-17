use m5unified::{DateTime, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Rtc")?;
    let _ = m5.rtc.set_datetime(DateTime {
        year: 2026,
        month: 5,
        day: 15,
        hour: 12,
        minute: 0,
        second: 0,
    });
    m5.rtc.set_system_time_from_rtc();
    if let Some(dt) = m5.rtc.get_datetime() {
        m5.display.println(&format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second
        ))?;
    }
    Ok(())
}
