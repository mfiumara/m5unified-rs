use core::ffi::c_int;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonId {
    A,
    B,
    C,
    Pwr,
    Ext,
}

#[derive(Debug)]
pub struct Buttons;

impl Buttons {
    pub fn button(&self, id: ButtonId) -> Button {
        Button { id }
    }

    pub fn a(&self) -> Button {
        self.button(ButtonId::A)
    }

    pub fn b(&self) -> Button {
        self.button(ButtonId::B)
    }

    pub fn c(&self) -> Button {
        self.button(ButtonId::C)
    }

    pub fn pwr(&self) -> Button {
        self.button(ButtonId::Pwr)
    }

    pub fn ext(&self) -> Button {
        self.button(ButtonId::Ext)
    }

    pub fn a_is_pressed(&self) -> bool {
        self.a().is_pressed()
    }

    pub fn b_was_pressed(&self) -> bool {
        self.b().was_pressed()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Button {
    id: ButtonId,
}

impl Button {
    fn raw_id(&self) -> c_int {
        match self.id {
            ButtonId::A => 0,
            ButtonId::B => 1,
            ButtonId::C => 2,
            ButtonId::Pwr => 3,
            ButtonId::Ext => 4,
        }
    }

    pub fn is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_pressed(self.raw_id()) }
    }

    pub fn was_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_pressed(self.raw_id()) }
    }

    pub fn was_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_released(self.raw_id()) }
    }

    pub fn was_clicked(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_clicked(self.raw_id()) }
    }

    pub fn was_hold(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_hold(self.raw_id()) }
    }

    pub fn is_holding(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_holding(self.raw_id()) }
    }

    pub fn was_decide_click_count(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_decide_click_count(self.raw_id()) }
    }

    pub fn click_count(&self) -> i32 {
        unsafe { m5unified_sys::m5u_button_get_click_count(self.raw_id()) as i32 }
    }
}
