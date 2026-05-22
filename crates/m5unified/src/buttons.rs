//! Button handles and event state helpers.
//!
//! The indexed wrapper covers the upstream A/B/C/EXT/PWR buttons and common
//! press, release, click, hold, and timing predicates.

use core::ffi::c_int;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonId {
    A,
    B,
    C,
    Pwr,
    Ext,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonState {
    NoChange,
    Clicked,
    Hold,
    DecideClickCount,
    Raw(u8),
}

impl ButtonState {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::NoChange,
            1 => Self::Clicked,
            2 => Self::Hold,
            3 => Self::DecideClickCount,
            raw => Self::Raw(raw),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::NoChange => 0,
            Self::Clicked => 1,
            Self::Hold => 2,
            Self::DecideClickCount => 3,
            Self::Raw(raw) => raw,
        }
    }
}

#[derive(Debug)]
pub struct Buttons;

impl Buttons {
    pub fn button(&self, id: ButtonId) -> Button {
        Button { id }
    }

    pub fn get(&self, index: usize) -> Option<Button> {
        let id = match index {
            0 => ButtonId::A,
            1 => ButtonId::B,
            2 => ButtonId::C,
            3 => ButtonId::Ext,
            4 => ButtonId::Pwr,
            _ => return None,
        };
        Some(self.button(id))
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
            ButtonId::Ext => 3,
            ButtonId::Pwr => 4,
        }
    }

    pub fn is_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_pressed(self.raw_id()) }
    }

    pub fn is_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_released(self.raw_id()) }
    }

    pub fn was_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_pressed(self.raw_id()) }
    }

    pub fn was_released(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_released(self.raw_id()) }
    }

    pub fn was_released_after_hold(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_released_after_hold(self.raw_id()) }
    }

    pub fn was_clicked(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_clicked(self.raw_id()) }
    }

    pub fn was_single_clicked(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_single_clicked(self.raw_id()) }
    }

    pub fn was_double_clicked(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_double_clicked(self.raw_id()) }
    }

    pub fn was_hold(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_hold(self.raw_id()) }
    }

    pub fn is_holding(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_is_holding(self.raw_id()) }
    }

    pub fn was_change_pressed(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_change_pressed(self.raw_id()) }
    }

    pub fn was_decide_click_count(&self) -> bool {
        unsafe { m5unified_sys::m5u_button_was_decide_click_count(self.raw_id()) }
    }

    #[deprecated(note = "use was_decide_click_count")]
    pub fn was_decied_click_count(&self) -> bool {
        self.was_decide_click_count()
    }

    pub fn click_count(&self) -> i32 {
        unsafe { m5unified_sys::m5u_button_get_click_count(self.raw_id()) as i32 }
    }

    pub fn was_release_for_ms(&self, ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_button_was_release_for(self.raw_id(), ms) }
    }

    #[deprecated(note = "use was_release_for_ms")]
    pub fn was_releasefor_ms(&self, ms: u32) -> bool {
        self.was_release_for_ms(ms)
    }

    pub fn pressed_for_ms(&self, ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_button_pressed_for(self.raw_id(), ms) }
    }

    pub fn released_for_ms(&self, ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_button_released_for(self.raw_id(), ms) }
    }

    pub fn set_debounce_thresh_ms(&self, ms: u32) {
        unsafe { m5unified_sys::m5u_button_set_debounce_thresh(self.raw_id(), ms) }
    }

    pub fn set_hold_thresh_ms(&self, ms: u32) {
        unsafe { m5unified_sys::m5u_button_set_hold_thresh(self.raw_id(), ms) }
    }

    pub fn set_raw_state(&self, msec: u32, pressed: bool) {
        unsafe { m5unified_sys::m5u_button_set_raw_state(self.raw_id(), msec, pressed) }
    }

    pub fn set_state_at(&self, msec: u32, state: ButtonState) {
        unsafe { m5unified_sys::m5u_button_set_state(self.raw_id(), msec, state.raw()) }
    }

    pub fn state(&self) -> ButtonState {
        unsafe { ButtonState::from_raw(m5unified_sys::m5u_button_get_state(self.raw_id())) }
    }

    pub fn last_change_ms(&self) -> u32 {
        unsafe { m5unified_sys::m5u_button_last_change(self.raw_id()) }
    }

    pub fn debounce_thresh_ms(&self) -> u32 {
        unsafe { m5unified_sys::m5u_button_get_debounce_thresh(self.raw_id()) }
    }

    pub fn hold_thresh_ms(&self) -> u32 {
        unsafe { m5unified_sys::m5u_button_get_hold_thresh(self.raw_id()) }
    }

    pub fn update_msec(&self) -> u32 {
        unsafe { m5unified_sys::m5u_button_get_update_msec(self.raw_id()) }
    }
}
