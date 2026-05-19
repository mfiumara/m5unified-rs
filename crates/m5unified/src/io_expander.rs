//! IO expander helpers for boards with upstream `M5.getIOExpander()` support
//! and direct PI4IOE5V6408 control.

use crate::M5Unified;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IoExpander {
    index: usize,
}

impl IoExpander {
    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn available(&self) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_available(self.index) }
    }

    /// Set pin direction. `true` means output and `false` means input.
    pub fn set_direction(&self, pin: u8, output: bool) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_set_direction(self.index, pin, output) }
    }

    pub fn enable_pull(&self, pin: u8, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_enable_pull(self.index, pin, enable) }
    }

    /// Set pull direction. `true` means pull-up and `false` means pull-down.
    pub fn set_pull_mode(&self, pin: u8, pull_up: bool) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_set_pull_mode(self.index, pin, pull_up) }
    }

    pub fn set_high_impedance(&self, pin: u8, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_set_high_impedance(self.index, pin, enable) }
    }

    pub fn write_value(&self, pin: u8) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_get_write_value(self.index, pin) }
    }

    pub fn digital_write(&self, pin: u8, level: bool) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_digital_write(self.index, pin, level) }
    }

    pub fn digital_read(&self, pin: u8) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_digital_read(self.index, pin) }
    }

    pub fn reset_irq(&self) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_reset_irq(self.index) }
    }

    pub fn disable_irq(&self) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_disable_irq(self.index) }
    }

    pub fn enable_irq(&self) -> bool {
        unsafe { m5unified_sys::m5u_io_expander_enable_irq(self.index) }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Pi4ioe5v6408;

impl Pi4ioe5v6408 {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_begin() }
    }

    /// Set pin direction. `true` means output and `false` means input.
    pub fn set_direction(&self, pin: u8, output: bool) -> bool {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_set_direction(pin, output) }
    }

    pub fn enable_pull(&self, pin: u8, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_enable_pull(pin, enable) }
    }

    /// Set pull direction. `true` means pull-up and `false` means pull-down.
    pub fn set_pull_mode(&self, pin: u8, pull_up: bool) -> bool {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_set_pull_mode(pin, pull_up) }
    }

    pub fn set_high_impedance(&self, pin: u8, enable: bool) -> bool {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_set_high_impedance(pin, enable) }
    }

    pub fn write_value(&self, pin: u8) -> bool {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_get_write_value(pin) }
    }

    pub fn digital_write(&self, pin: u8, level: bool) -> bool {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_digital_write(pin, level) }
    }

    pub fn digital_read(&self, pin: u8) -> bool {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_digital_read(pin) }
    }

    pub fn reset_irq(&self) {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_reset_irq() }
    }

    pub fn disable_irq(&self) {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_disable_irq() }
    }

    pub fn enable_irq(&self) {
        unsafe { m5unified_sys::m5u_pi4ioe5v6408_enable_irq() }
    }
}

impl M5Unified {
    pub fn io_expander(&self, index: usize) -> IoExpander {
        IoExpander { index }
    }

    pub fn pi4ioe5v6408(&self) -> Pi4ioe5v6408 {
        Pi4ioe5v6408
    }
}
