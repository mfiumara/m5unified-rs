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

    pub fn accel(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_accel(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn gyro(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_gyro(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
    }

    pub fn mag(&self) -> Option<Vec3> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let ok = unsafe { m5unified_sys::m5u_imu_get_mag(&mut x, &mut y, &mut z) };
        ok.then_some(Vec3 { x, y, z })
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
