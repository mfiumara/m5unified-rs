use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Imu")?;
    if m5.imu.try_begin().is_err() {
        m5.display.println("imu unavailable")?;
        return Ok(());
    }
    if let Ok(data) = m5.imu.try_data() {
        m5.display.println(&format!(
            "accel: {:.2}, {:.2}, {:.2} |g|={:.2}",
            data.accel.x,
            data.accel.y,
            data.accel.z,
            data.accel_magnitude()
        ))?;
        m5.display.println(&format!(
            "gyro: {:.2}, {:.2}, {:.2} |dps|={:.2}",
            data.gyro.x,
            data.gyro.y,
            data.gyro.z,
            data.gyro_magnitude()
        ))?;
        if let Some(temp) = data.temperature_c {
            m5.display.println(&format!("temp: {temp:.1} C"))?;
        }
    }
    let raw = m5.imu.raw_data_array();
    m5.display
        .println(&format!("raw[0..3]: {}, {}, {}", raw[0], raw[1], raw[2]))?;
    Ok(())
}
