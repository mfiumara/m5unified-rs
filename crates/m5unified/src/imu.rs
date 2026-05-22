//! IMU data, calibration, and axis-order helpers.
//!
//! This module wraps M5Unified's IMU class with typed vectors, sensor masks,
//! sensor kind detection, and safe accessors for combined IMU data.

use core::ffi::c_int;

use crate::system::Board;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImuAxis {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
    Raw(i32),
}

impl ImuAxis {
    pub const fn raw(self) -> i32 {
        match self {
            Self::XPos => 0,
            Self::XNeg => 1,
            Self::YPos => 2,
            Self::YNeg => 3,
            Self::ZPos => 4,
            Self::ZNeg => 5,
            Self::Raw(raw) => raw,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ImuSensorMask(u8);

impl ImuSensorMask {
    pub const NONE: Self = Self(0);
    pub const ACCEL: Self = Self(1 << 0);
    pub const GYRO: Self = Self(1 << 1);
    pub const MAG: Self = Self(1 << 2);

    pub const fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u8 {
        self.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

#[derive(Debug)]
pub struct Imu;

impl Imu {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_begin() }
    }

    pub fn begin_for_board(&mut self, board: Board) -> bool {
        unsafe { m5unified_sys::m5u_imu_begin_for_board(board.raw()) }
    }

    pub fn init(&mut self) -> bool {
        self.begin()
    }

    pub fn init_for_board(&mut self, board: Board) -> bool {
        self.begin_for_board(board)
    }

    pub fn ak8963(&self) -> ImuDevice {
        ImuDevice::new(ImuDeviceKind::Ak8963)
    }

    pub fn bmm150(&self) -> ImuDevice {
        ImuDevice::new(ImuDeviceKind::Bmm150)
    }

    pub fn bmi270(&self) -> ImuDevice {
        ImuDevice::new(ImuDeviceKind::Bmi270)
    }

    pub fn mpu6886(&self) -> ImuDevice {
        ImuDevice::new(ImuDeviceKind::Mpu6886)
    }

    pub fn sh200q(&self) -> ImuDevice {
        ImuDevice::new(ImuDeviceKind::Sh200q)
    }

    pub fn accel(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_accel(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn accel_data(&self) -> Option<Vec3> {
        self.accel()
    }

    pub fn gyro(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_gyro(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn gyro_data(&self) -> Option<Vec3> {
        self.gyro()
    }

    pub fn mag(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_mag(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn gyro_mag(&self) -> Option<Vec3> {
        self.mag()
    }

    pub fn temperature_c(&self) -> Option<f32> {
        let mut temp = 0.0;
        let ok = unsafe { m5unified_sys::m5u_imu_get_temp_c(&mut temp) };
        ok.then_some(temp)
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_imu_is_enabled() }
    }

    pub fn kind(&self) -> ImuKind {
        ImuKind::from_raw(unsafe { m5unified_sys::m5u_imu_get_type() as i32 })
    }

    pub fn update(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_update() }
    }

    pub fn update_mask(&mut self) -> ImuSensorMask {
        let raw = unsafe { m5unified_sys::m5u_imu_update_mask() };
        ImuSensorMask::from_raw(raw.max(0) as u8)
    }

    pub fn data(&self) -> Option<ImuData> {
        let mut raw = m5unified_sys::m5u_imu_data_t::default();
        let ok = unsafe { m5unified_sys::m5u_imu_get_data(&mut raw) };
        ok.then(|| ImuData {
            usec: raw.usec,
            accel: Vec3 {
                x: raw.accel_x,
                y: raw.accel_y,
                z: raw.accel_z,
            },
            gyro: Vec3 {
                x: raw.gyro_x,
                y: raw.gyro_y,
                z: raw.gyro_z,
            },
            mag: Vec3 {
                x: raw.mag_x,
                y: raw.mag_y,
                z: raw.mag_z,
            },
            temperature_c: self.temperature_c(),
        })
    }

    pub fn load_offset_from_nvs(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_load_offset_from_nvs() }
    }

    pub fn save_offset_to_nvs(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_save_offset_to_nvs() }
    }

    pub fn offset_data(&self, index: i32) -> f32 {
        unsafe { m5unified_sys::m5u_imu_get_offset_data(index) }
    }

    pub fn set_calibration(&mut self, x: f32, y: f32, z: f32) {
        unsafe { m5unified_sys::m5u_imu_set_calibration(x, y, z) }
    }

    pub fn sleep(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_sleep() }
    }

    pub fn set_clock_hz(&mut self, freq: u32) {
        unsafe { m5unified_sys::m5u_imu_set_clock(freq) }
    }

    pub fn set_axis_order(&mut self, axis0: ImuAxis, axis1: ImuAxis, axis2: ImuAxis) -> bool {
        unsafe { m5unified_sys::m5u_imu_set_axis_order(axis0.raw(), axis1.raw(), axis2.raw()) }
    }

    pub fn set_axis_order_right_handed(&mut self, axis0: ImuAxis, axis1: ImuAxis) -> bool {
        unsafe { m5unified_sys::m5u_imu_set_axis_order_right_handed(axis0.raw(), axis1.raw()) }
    }

    pub fn set_axis_order_left_handed(&mut self, axis0: ImuAxis, axis1: ImuAxis) -> bool {
        unsafe { m5unified_sys::m5u_imu_set_axis_order_left_handed(axis0.raw(), axis1.raw()) }
    }

    pub fn set_int_pin_active_logic(&mut self, level: bool) -> bool {
        unsafe { m5unified_sys::m5u_imu_set_int_pin_active_logic(level) }
    }

    pub fn set_calibration_strength(&mut self, accel: u8, gyro: u8, mag: u8) {
        unsafe { m5unified_sys::m5u_imu_set_calibration_strength(accel, gyro, mag) }
    }

    pub fn clear_offset_data(&mut self) {
        unsafe { m5unified_sys::m5u_imu_clear_offset_data() }
    }

    pub fn set_offset_data(&mut self, index: usize, value: i32) {
        unsafe { m5unified_sys::m5u_imu_set_offset_data(index, value) }
    }

    pub fn offset_data_i32(&self, index: usize) -> i32 {
        unsafe { m5unified_sys::m5u_imu_get_offset_data_i32(index) }
    }

    pub fn raw_data(&self, index: usize) -> i16 {
        unsafe { m5unified_sys::m5u_imu_get_raw_data(index) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImuKind {
    None,
    Sh200q,
    Mpu6050,
    Mpu6886,
    Mpu9250,
    Bmi270,
    Unknown(i32),
}

impl ImuKind {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::None,
            2 => Self::Sh200q,
            3 => Self::Mpu6050,
            4 => Self::Mpu6886,
            5 => Self::Mpu9250,
            6 => Self::Bmi270,
            raw => Self::Unknown(raw),
        }
    }

    pub const fn raw(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Unknown(raw) => raw,
            Self::Sh200q => 2,
            Self::Mpu6050 => 3,
            Self::Mpu6886 => 4,
            Self::Mpu9250 => 5,
            Self::Bmi270 => 6,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ImuData {
    pub usec: u32,
    pub accel: Vec3,
    pub gyro: Vec3,
    pub mag: Vec3,
    pub temperature_c: Option<f32>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImuDeviceKind {
    Ak8963,
    Bmm150,
    Bmi270,
    Mpu6886,
    Sh200q,
}

impl ImuDeviceKind {
    const fn raw(self) -> c_int {
        match self {
            Self::Ak8963 => 0,
            Self::Bmm150 => 1,
            Self::Bmi270 => 2,
            Self::Mpu6886 => 3,
            Self::Sh200q => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct RawVec3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl RawVec3 {
    pub const fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    pub fn scaled(self, resolution: f32) -> Vec3 {
        Vec3 {
            x: self.x as f32 * resolution,
            y: self.y as f32 * resolution,
            z: self.z as f32 * resolution,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ImuRawData {
    pub sensor_mask: ImuSensorMask,
    pub accel: RawVec3,
    pub gyro: RawVec3,
    pub mag: RawVec3,
    pub temp_adc: i16,
}

impl ImuRawData {
    fn from_raw(raw: m5unified_sys::m5u_imu_raw_data_t) -> Self {
        Self {
            sensor_mask: ImuSensorMask::from_raw(raw.sensor_mask),
            accel: RawVec3::new(raw.accel_x, raw.accel_y, raw.accel_z),
            gyro: RawVec3::new(raw.gyro_x, raw.gyro_y, raw.gyro_z),
            mag: RawVec3::new(raw.mag_x, raw.mag_y, raw.mag_z),
            temp_adc: raw.temp,
        }
    }

    pub const fn has_accel(self) -> bool {
        self.sensor_mask.contains(ImuSensorMask::ACCEL)
    }

    pub const fn has_gyro(self) -> bool {
        self.sensor_mask.contains(ImuSensorMask::GYRO)
    }

    pub const fn has_mag(self) -> bool {
        self.sensor_mask.contains(ImuSensorMask::MAG)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ImuConvertParams {
    pub accel_res: f32,
    pub gyro_res: f32,
    pub mag_res: f32,
    pub temp_res: f32,
    pub temp_offset: f32,
}

impl ImuConvertParams {
    fn from_raw(raw: m5unified_sys::m5u_imu_convert_param_t) -> Self {
        Self {
            accel_res: raw.accel_res,
            gyro_res: raw.gyro_res,
            mag_res: raw.mag_res,
            temp_res: raw.temp_res,
            temp_offset: raw.temp_offset,
        }
    }

    pub fn temperature_c(self, adc: i16) -> f32 {
        adc as f32 * self.temp_res + self.temp_offset
    }
}

impl Default for ImuConvertParams {
    fn default() -> Self {
        Self::from_raw(m5unified_sys::m5u_imu_convert_param_t::default())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ImuDevice {
    kind: ImuDeviceKind,
}

impl ImuDevice {
    const fn new(kind: ImuDeviceKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> ImuDeviceKind {
        self.kind
    }

    pub fn begin(&mut self) -> ImuSensorMask {
        let raw = unsafe { m5unified_sys::m5u_imu_device_begin(self.kind.raw()) };
        ImuSensorMask::from_raw(raw.max(0) as u8)
    }

    pub fn init(&mut self) -> ImuSensorMask {
        self.begin()
    }

    pub fn raw_data(&self) -> Option<ImuRawData> {
        let mut raw = m5unified_sys::m5u_imu_raw_data_t::default();
        unsafe { m5unified_sys::m5u_imu_device_get_raw_data(self.kind.raw(), &mut raw) }
            .then_some(ImuRawData::from_raw(raw))
    }

    pub fn convert_params(&self) -> Option<ImuConvertParams> {
        let mut raw = m5unified_sys::m5u_imu_convert_param_t::default();
        unsafe { m5unified_sys::m5u_imu_device_get_convert_param(self.kind.raw(), &mut raw) }
            .then_some(ImuConvertParams::from_raw(raw))
    }

    pub fn data(&self) -> Option<ImuData> {
        let raw = self.raw_data()?;
        let params = self.convert_params().unwrap_or_default();
        Some(ImuData {
            usec: 0,
            accel: raw.accel.scaled(params.accel_res),
            gyro: raw.gyro.scaled(params.gyro_res),
            mag: raw.mag.scaled(params.mag_res),
            temperature_c: self.temperature_adc().map(|adc| params.temperature_c(adc)),
        })
    }

    pub fn temperature_adc(&self) -> Option<i16> {
        let mut adc = 0;
        unsafe { m5unified_sys::m5u_imu_device_get_temp_adc(self.kind.raw(), &mut adc) }
            .then_some(adc)
    }

    pub fn temperature_c(&self) -> Option<f32> {
        let adc = self.temperature_adc()?;
        self.convert_params()
            .map(|params| params.temperature_c(adc))
    }

    pub fn sleep(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_device_sleep(self.kind.raw()) }
    }

    pub fn set_int_pin_active_logic(&mut self, level: bool) -> bool {
        unsafe { m5unified_sys::m5u_imu_device_set_int_pin_active_logic(self.kind.raw(), level) }
    }

    pub fn who_am_i(&self) -> Option<u8> {
        let raw = unsafe { m5unified_sys::m5u_imu_device_who_am_i(self.kind.raw()) };
        (raw >= 0).then_some(raw as u8)
    }
}
