use m5unified::{DateTime, M5Unified, RtcDate, RtcTime};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Rtc")?;
    if m5.rtc.try_begin().is_err() {
        m5.display.println("rtc unavailable")?;
        return Ok(());
    }
    let date = RtcDate::new(2026, 5, 15, 5);
    let time = RtcTime::new(12, 0, 0);
    let datetime = date.with_time(time);
    let _ = m5.rtc.try_set_datetime(datetime);
    m5.display.println(&format!(
        "valid date={} time={} dt={}",
        date.is_valid(),
        time.is_valid(),
        DateTime::new(2026, 5, 15, 12, 0, 0).is_valid()
    ))?;
    if let Ok(dt) = m5.rtc.try_get_datetime() {
        m5.display.println(&format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second
        ))?;
    }
    Ok(())
}
