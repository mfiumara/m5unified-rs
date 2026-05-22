use m5unified::{DateTime, M5Unified};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Rtc")?;
    let _ = m5.rtc.set_datetime(DateTime {
        year: 2026,
        month: 5,
        day: 15,
        weekday: None,
        hour: 12,
        minute: 0,
        second: 0,
    });
    m5.rtc.set_system_time_from_rtc();
    m5.display
        .println(&format!("rtc volt_low={}", m5.rtc.volt_low()))?;
    if let Some(dt) = m5.rtc.get_datetime() {
        m5.display.println(&format!(
            "{:04}-{:02}-{:02} {:?} {:02}:{:02}:{:02}",
            dt.year, dt.month, dt.day, dt.weekday, dt.hour, dt.minute, dt.second
        ))?;
    }
    let _ = m5.rtc.set_timer_irq_ms(1_000);
    let _ = m5.rtc.set_alarm_irq_after_seconds(1);
    let _ = m5.rtc.irq_status();
    m5.rtc.clear_irq();
    m5.rtc.disable_irq();
    Ok(())
}
