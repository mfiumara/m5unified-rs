//! FEETECH SCSCL bus servo driver for StackChan head movement.
//!
//! Wraps the C shim that talks to StackChan's two SCSCL smart servos over
//! UART.  On non-ESP targets the shim is a no-op, so this compiles for host
//! checks but does nothing.
//!
//! # Servo layout
//! | Axis  | Servo ID | Default zero (raw) | Range           |
//! |-------|----------|--------------------|-----------------|
//! | Yaw   | 1        | 460                | −128 ° … +128 ° |
//! | Pitch | 2        | 620                | 0 ° … +90 °     |
//!
//! Zero positions are persisted in ESP32 NVS so calibration survives reboots.
//! Use [`StackChanServos::calibrate_from_current_pos`] to set the home position
//! while the servos are at the desired neutral angle, then call
//! [`StackChanServos::save_calibration`] to write it to NVS.

use crate::Error;

/// Servo ID for the yaw (left–right) axis.
pub const SERVO_YAW: u8 = 1;
/// Servo ID for the pitch (up–down) axis.
pub const SERVO_PITCH: u8 = 2;

/// Factory-default zero position for yaw (raw SCSCL units, 0–1023).
pub const YAW_ZERO_DEFAULT: u16 = 460;
/// Factory-default zero position for pitch (raw SCSCL units, 0–1023).
pub const PITCH_ZERO_DEFAULT: u16 = 620;

const NVS_NS: &[u8] = b"servo\0";
const NVS_KEY_YAW: &[u8] = b"zero_pos_1\0";
const NVS_KEY_PITCH: &[u8] = b"zero_pos_2\0";

/// Converts a degree angle to a SCSCL raw position value.
///
/// One SCSCL raw unit = 0.3125 ° (= 5/16 °), so raw = zero + angle × 16/5.
fn angle_to_raw(angle_deg: i32, zero_raw: u16) -> u16 {
    let raw = zero_raw as i32 + (angle_deg as f32 * 16.0 / 5.0) as i32;
    raw.clamp(0, 1023) as u16
}

/// Converts a SCSCL raw position back to degrees relative to a zero position.
fn raw_to_angle(raw: i32, zero_raw: u16) -> f32 {
    (raw - zero_raw as i32) as f32 * 5.0 / 16.0
}

fn nvs_read_zero(key: &[u8], default: u16) -> u16 {
    let mut val: i32 = 0;
    let ok = unsafe {
        m5unified_sys::m5u_nvs_read_i32(
            NVS_NS.as_ptr() as *const core::ffi::c_char,
            key.as_ptr() as *const core::ffi::c_char,
            &mut val,
        )
    };
    if ok && (0..=1023).contains(&val) {
        val as u16
    } else {
        default
    }
}

fn nvs_write_zero(key: &[u8], val: u16) -> bool {
    unsafe {
        m5unified_sys::m5u_nvs_write_i32(
            NVS_NS.as_ptr() as *const core::ffi::c_char,
            key.as_ptr() as *const core::ffi::c_char,
            val as i32,
        )
    }
}

/// Handle to the two StackChan head servos with NVS-backed calibration.
///
/// Drop releases the UART bus.
pub struct StackChanServos {
    yaw_zero: u16,
    pitch_zero: u16,
}

impl StackChanServos {
    /// Initialize with default Core S3 pins (TX=6, RX=7) at 1 Mbps.
    /// Loads calibration from NVS; falls back to factory defaults if absent.
    pub fn new() -> Result<Self, Error> {
        Self::new_with_pins(-1, -1)
    }

    /// Initialize with explicit TX/RX GPIO pins.  Pass `0` for `baud_rate`
    /// to use the StackChan default of 1 000 000.
    /// Loads calibration from NVS; falls back to factory defaults if absent.
    pub fn new_with_pins(tx_pin: i32, rx_pin: i32) -> Result<Self, Error> {
        let ok = unsafe { m5unified_sys::m5u_servo_init(tx_pin, rx_pin, 0) };
        if !ok {
            return Err(Error::BeginFailed);
        }
        let yaw_zero = nvs_read_zero(NVS_KEY_YAW, YAW_ZERO_DEFAULT);
        let pitch_zero = nvs_read_zero(NVS_KEY_PITCH, PITCH_ZERO_DEFAULT);
        Ok(Self { yaw_zero, pitch_zero })
    }

    /// Current yaw zero position (raw units, 0–1023).
    pub fn yaw_zero(&self) -> u16 { self.yaw_zero }

    /// Current pitch zero position (raw units, 0–1023).
    pub fn pitch_zero(&self) -> u16 { self.pitch_zero }

    /// Move the yaw servo to `angle_deg` degrees from its calibrated home.
    ///
    /// `time_ms` = 0 means "as fast as possible"; `speed` = 0 means no limit.
    pub fn move_yaw(&mut self, angle_deg: i32, time_ms: u16, speed: u16) -> bool {
        let raw = angle_to_raw(angle_deg, self.yaw_zero);
        unsafe { m5unified_sys::m5u_servo_write_raw_pos(SERVO_YAW, raw, time_ms, speed) }
    }

    /// Move the pitch servo to `angle_deg` degrees (0 = level, positive = up).
    pub fn move_pitch(&mut self, angle_deg: i32, time_ms: u16, speed: u16) -> bool {
        let raw = angle_to_raw(angle_deg, self.pitch_zero);
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

    /// Enable or disable torque on a single servo.
    pub fn set_torque_single(&mut self, id: u8, enable: bool) {
        unsafe { m5unified_sys::m5u_servo_enable_torque(id, enable); }
    }

    /// Read the current raw position of a servo (0–1023).
    /// Returns `None` on timeout or bus error.
    pub fn read_raw_pos(&mut self, id: u8) -> Option<i32> {
        let v = unsafe { m5unified_sys::m5u_servo_read_raw_pos(id) };
        if v < 0 { None } else { Some(v) }
    }

    /// Read the current angle of a servo relative to its calibrated home.
    /// Returns `None` on bus error.
    pub fn read_angle(&mut self, id: u8) -> Option<f32> {
        let raw = self.read_raw_pos(id)?;
        let zero = if id == SERVO_YAW { self.yaw_zero } else { self.pitch_zero };
        Some(raw_to_angle(raw, zero))
    }

    /// Set the in-memory zero position for `id` to the servo's current raw position.
    ///
    /// Call [`save_calibration`] afterwards to persist to NVS.
    /// Returns the new zero raw value, or `None` on bus error.
    pub fn calibrate_from_current_pos(&mut self, id: u8) -> Option<u16> {
        let raw = self.read_raw_pos(id)?;
        let raw = raw.clamp(0, 1023) as u16;
        if id == SERVO_YAW {
            self.yaw_zero = raw;
        } else {
            self.pitch_zero = raw;
        }
        Some(raw)
    }

    /// Write the current in-memory zero positions to NVS.
    /// Returns `true` if both writes succeed.
    pub fn save_calibration(&self) -> bool {
        nvs_write_zero(NVS_KEY_YAW, self.yaw_zero)
            & nvs_write_zero(NVS_KEY_PITCH, self.pitch_zero)
    }

    /// Reset both zero positions to factory defaults in memory and NVS.
    pub fn reset_calibration(&mut self) -> bool {
        self.yaw_zero = YAW_ZERO_DEFAULT;
        self.pitch_zero = PITCH_ZERO_DEFAULT;
        self.save_calibration()
    }
}

impl Drop for StackChanServos {
    fn drop(&mut self) {
        unsafe { m5unified_sys::m5u_servo_deinit() }
    }
}
