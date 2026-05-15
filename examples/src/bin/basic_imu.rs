use m5unified::M5Unified;
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Imu")?;
    if !m5.imu.begin() {
        m5.display.println("imu unavailable")?;
        return Ok(());
    }
    if let Some(accel) = m5.imu.accel() {
        m5.display.println(&format!(
            "accel: {:.2}, {:.2}, {:.2}",
            accel.x, accel.y, accel.z
        ))?;
    }
    if let Some(gyro) = m5.imu.gyro() {
        m5.display.println(&format!(
            "gyro: {:.2}, {:.2}, {:.2}",
            gyro.x, gyro.y, gyro.z
        ))?;
    }
    if let Some(temp) = m5.imu.temperature_c() {
        m5.display.println(&format!("temp: {temp:.1} C"))?;
    }
    Ok(())
}
