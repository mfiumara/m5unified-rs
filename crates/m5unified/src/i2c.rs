use core::ffi::c_int;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct I2cBus {
    kind: I2cBusKind,
}

impl I2cBus {
    pub const INTERNAL: Self = Self {
        kind: I2cBusKind::Internal,
    };
    pub const EXTERNAL: Self = Self {
        kind: I2cBusKind::External,
    };

    pub fn set_port(&mut self, port_num: i32, pin_sda: i32, pin_scl: i32) {
        unsafe { m5unified_sys::m5u_i2c_set_port(self.raw(), port_num, pin_sda, pin_scl) }
    }

    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_i2c_begin(self.raw()) }
    }

    pub fn begin_with_port(&mut self, port_num: i32, pin_sda: i32, pin_scl: i32) -> bool {
        unsafe { m5unified_sys::m5u_i2c_begin_with_port(self.raw(), port_num, pin_sda, pin_scl) }
    }

    pub fn release(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_i2c_release(self.raw()) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_i2c_is_enabled(self.raw()) }
    }

    pub fn port(&self) -> Option<i32> {
        let port = unsafe { m5unified_sys::m5u_i2c_get_port(self.raw()) };
        (port >= 0).then_some(port)
    }

    pub fn sda_pin(&self) -> Option<u8> {
        let pin = unsafe { m5unified_sys::m5u_i2c_get_sda(self.raw()) };
        (pin >= 0).then_some(pin as u8)
    }

    pub fn scl_pin(&self) -> Option<u8> {
        let pin = unsafe { m5unified_sys::m5u_i2c_get_scl(self.raw()) };
        (pin >= 0).then_some(pin as u8)
    }

    pub fn start(&mut self, address: u8, read: bool, freq_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_i2c_start(self.raw(), address, read, freq_hz) }
    }

    pub fn restart(&mut self, address: u8, read: bool, freq_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_i2c_restart(self.raw(), address, read, freq_hz) }
    }

    pub fn stop(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_i2c_stop(self.raw()) }
    }

    pub fn write_byte(&mut self, data: u8) -> bool {
        unsafe { m5unified_sys::m5u_i2c_write_byte(self.raw(), data) }
    }

    pub fn write(&mut self, data: &[u8]) -> bool {
        unsafe { m5unified_sys::m5u_i2c_write(self.raw(), data.as_ptr(), data.len()) }
    }

    pub fn read(&mut self, result: &mut [u8], last_nack: bool) -> bool {
        unsafe {
            m5unified_sys::m5u_i2c_read(self.raw(), result.as_mut_ptr(), result.len(), last_nack)
        }
    }

    pub fn write_register(&mut self, address: u8, reg: u8, data: &[u8], freq_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_i2c_write_register(
                self.raw(),
                address,
                reg,
                data.as_ptr(),
                data.len(),
                freq_hz,
            )
        }
    }

    pub fn read_register(&mut self, address: u8, reg: u8, result: &mut [u8], freq_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_i2c_read_register(
                self.raw(),
                address,
                reg,
                result.as_mut_ptr(),
                result.len(),
                freq_hz,
            )
        }
    }

    pub fn write_register8(&mut self, address: u8, reg: u8, data: u8, freq_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_i2c_write_register8(self.raw(), address, reg, data, freq_hz) }
    }

    pub fn read_register8(&mut self, address: u8, reg: u8, freq_hz: u32) -> u8 {
        unsafe { m5unified_sys::m5u_i2c_read_register8(self.raw(), address, reg, freq_hz) }
    }

    pub fn bit_on(&mut self, address: u8, reg: u8, data: u8, freq_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_i2c_bit_on(self.raw(), address, reg, data, freq_hz) }
    }

    pub fn bit_off(&mut self, address: u8, reg: u8, data: u8, freq_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_i2c_bit_off(self.raw(), address, reg, data, freq_hz) }
    }

    pub fn scan(&self, freq_hz: u32) -> [bool; 120] {
        let mut result = [false; 120];
        unsafe { m5unified_sys::m5u_i2c_scan(self.raw(), result.as_mut_ptr(), freq_hz) };
        result
    }

    pub fn scan_address(&self, address: u8, freq_hz: u32) -> bool {
        unsafe { m5unified_sys::m5u_i2c_scan_address(self.raw(), address, freq_hz) }
    }

    fn raw(self) -> c_int {
        self.kind as c_int
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum I2cBusKind {
    Internal = 0,
    External = 1,
}
