//! FEETECH SCSCL bus servo driver for StackChan head movement.
//!
//! Wraps the C shim that talks to StackChan's two SCSCL smart servos over
//! UART.  On non-ESP targets the shim is a no-op, so this compiles for host
//! checks but does nothing.
//!
//! # Servo layout
//! | Axis  | Servo ID | Zero (raw) | Range           |
//! |-------|----------|-----------|-----------------|
//! | Yaw   | 1        | 460       | −128 ° … +128 ° |
//! | Pitch | 2        | 620       | 0 ° … +90 °     |

use crate::Error;

/// Servo ID for the yaw (left–right) axis.
pub const SERVO_YAW: u8 = 1;
/// Servo ID for the pitch (up–down) axis.
pub const SERVO_PITCH: u8 = 2;

const YAW_ZERO_RAW: u16 = 460;
const PITCH_ZERO_RAW: u16 = 620;

/// Converts a degree angle to a SCSCL raw position value.
///
/// One SCSCL raw unit ≈ 0.3125 ° (5/16 °), so raw = zero + angle × 16/5.
fn angle_to_raw(angle_deg: i32, zero_raw: u16) -> u16 {
    let raw = zero_raw as i32 + (angle_deg as f32 * 16.0 / 5.0) as i32;
    raw.clamp(0, 1023) as u16
}

/// Handle to the two StackChan head servos.
///
/// Drop releases the UART bus.
pub struct StackChanServos {
    _private: (),
}

impl StackChanServos {
    /// Initialize with default Core S3 pins (TX=6, RX=7) at 1 Mbps.
    pub fn new() -> Result<Self, Error> {
        Self::new_with_pins(-1, -1)
    }

    /// Initialize with explicit TX/RX GPIO pins.  Pass `0` for `baud_rate`
    /// to use the StackChan default of 1 000 000.
    pub fn new_with_pins(tx_pin: i32, rx_pin: i32) -> Result<Self, Error> {
        let ok = unsafe { m5unified_sys::m5u_servo_init(tx_pin, rx_pin, 0) };
        if ok {
            Ok(Self { _private: () })
        } else {
            Err(Error::BeginFailed)
        }
    }

    /// Move the yaw servo to `angle_deg` degrees.
    ///
    /// `time_ms` = 0 means "as fast as possible"; `speed` = 0 means no limit.
    pub fn move_yaw(&mut self, angle_deg: i32, time_ms: u16, speed: u16) -> bool {
        let raw = angle_to_raw(angle_deg, YAW_ZERO_RAW);
        unsafe { m5unified_sys::m5u_servo_write_raw_pos(SERVO_YAW, raw, time_ms, speed) }
    }

    /// Move the pitch servo to `angle_deg` degrees (0 = level, positive = tilt up).
    pub fn move_pitch(&mut self, angle_deg: i32, time_ms: u16, speed: u16) -> bool {
        let raw = angle_to_raw(angle_deg, PITCH_ZERO_RAW);
        unsafe { m5unified_sys::m5u_servo_write_raw_pos(SERVO_PITCH, raw, time_ms, speed) }
    }

    /// Move both servos simultaneously.
    pub fn move_both(
        &mut self,
        yaw_deg: i32,
        pitch_deg: i32,
        time_ms: u16,
        speed: u16,
    ) -> bool {
        self.move_yaw(yaw_deg, time_ms, speed) & self.move_pitch(pitch_deg, time_ms, speed)
    }

    /// Enable or disable torque on both servos.
    pub fn set_torque(&mut self, enable: bool) {
        unsafe {
            m5unified_sys::m5u_servo_enable_torque(SERVO_YAW, enable);
            m5unified_sys::m5u_servo_enable_torque(SERVO_PITCH, enable);
        }
    }

    /// Read the current raw position of a servo.  Returns `None` on timeout.
    pub fn read_raw_pos(&mut self, id: u8) -> Option<i32> {
        let v = unsafe { m5unified_sys::m5u_servo_read_raw_pos(id) };
        if v < 0 { None } else { Some(v) }
    }
}

impl Drop for StackChanServos {
    fn drop(&mut self) {
        unsafe { m5unified_sys::m5u_servo_deinit() }
    }
}
