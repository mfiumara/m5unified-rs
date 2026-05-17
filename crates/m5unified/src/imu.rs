#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
        ImuKind::Unknown(unsafe { m5unified_sys::m5u_imu_get_type() as i32 })
    }

    pub fn update(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_imu_update() }
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImuKind {
    Unknown(i32),
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ImuData {
    pub usec: u32,
    pub accel: Vec3,
    pub gyro: Vec3,
    pub mag: Vec3,
    pub temperature_c: Option<f32>,
}
