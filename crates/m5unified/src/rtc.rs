//! RTC date/time and alarm helpers.
//!
//! This module provides typed date, time, and date-time structs plus wrappers
//! for RTC status, setters, system-time sync, timer IRQs, and alarm IRQs.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub weekday: Option<u8>,
}

impl Date {
    fn from_raw(raw: m5unified_sys::m5u_rtc_datetime_t) -> Self {
        Self {
            year: raw.year,
            month: raw.month,
            day: raw.day,
            weekday: (0..=6).contains(&raw.weekday).then_some(raw.weekday as u8),
        }
    }

    pub(crate) fn to_raw(self) -> m5unified_sys::m5u_rtc_datetime_t {
        m5unified_sys::m5u_rtc_datetime_t {
            year: self.year,
            month: self.month,
            day: self.day,
            weekday: self.weekday.map(i32::from).unwrap_or(255),
            ..m5unified_sys::m5u_rtc_datetime_t::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Time {
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

impl Time {
    fn from_raw(raw: m5unified_sys::m5u_rtc_datetime_t) -> Self {
        Self {
            hour: raw.hour,
            minute: raw.minute,
            second: raw.second,
        }
    }

    pub(crate) fn to_raw(self) -> m5unified_sys::m5u_rtc_datetime_t {
        m5unified_sys::m5u_rtc_datetime_t {
            hour: self.hour,
            minute: self.minute,
            second: self.second,
            ..m5unified_sys::m5u_rtc_datetime_t::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub weekday: Option<u8>,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

impl DateTime {
    pub fn date(self) -> Date {
        Date {
            year: self.year,
            month: self.month,
            day: self.day,
            weekday: self.weekday,
        }
    }

    pub fn time(self) -> Time {
        Time {
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        }
    }

    pub fn from_date_time(date: Date, time: Time) -> Self {
        Self {
            year: date.year,
            month: date.month,
            day: date.day,
            weekday: date.weekday,
            hour: time.hour,
            minute: time.minute,
            second: time.second,
        }
    }

    fn from_raw(raw: m5unified_sys::m5u_rtc_datetime_t) -> Self {
        Self {
            year: raw.year,
            month: raw.month,
            day: raw.day,
            weekday: (0..=6).contains(&raw.weekday).then_some(raw.weekday as u8),
            hour: raw.hour,
            minute: raw.minute,
            second: raw.second,
        }
    }

    fn to_raw(self) -> m5unified_sys::m5u_rtc_datetime_t {
        m5unified_sys::m5u_rtc_datetime_t {
            year: self.year,
            month: self.month,
            day: self.day,
            weekday: self.weekday.map(i32::from).unwrap_or(255),
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        }
    }
}

#[derive(Debug)]
pub struct Rtc;

impl Rtc {
    pub fn get_datetime(&self) -> Option<DateTime> {
        let mut raw = m5unified_sys::m5u_rtc_datetime_t::default();
        unsafe { m5unified_sys::m5u_rtc_get_datetime_detail(&mut raw) }
            .then_some(DateTime::from_raw(raw))
    }

    pub fn get_date(&self) -> Option<Date> {
        let mut raw = m5unified_sys::m5u_rtc_datetime_t::default();
        unsafe { m5unified_sys::m5u_rtc_get_date_detail(&mut raw) }.then_some(Date::from_raw(raw))
    }

    pub fn get_time(&self) -> Option<Time> {
        let mut raw = m5unified_sys::m5u_rtc_datetime_t::default();
        unsafe { m5unified_sys::m5u_rtc_get_time_detail(&mut raw) }.then_some(Time::from_raw(raw))
    }

    pub fn set_datetime(&mut self, datetime: DateTime) -> bool {
        let raw = datetime.to_raw();
        unsafe { m5unified_sys::m5u_rtc_set_datetime_detail(&raw) }
    }

    pub fn set_date(&mut self, date: Date) -> bool {
        let raw = date.to_raw();
        unsafe { m5unified_sys::m5u_rtc_set_date_detail(&raw) }
    }

    pub fn set_time(&mut self, time: Time) -> bool {
        let raw = time.to_raw();
        unsafe { m5unified_sys::m5u_rtc_set_time_detail(&raw) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_rtc_is_enabled() }
    }

    pub fn volt_low(&self) -> bool {
        unsafe { m5unified_sys::m5u_rtc_get_volt_low() }
    }

    pub fn set_system_time_from_rtc(&mut self) {
        unsafe { m5unified_sys::m5u_rtc_set_system_time_from_rtc() }
    }

    pub fn set_timer_irq_ms(&mut self, timer_msec: u32) -> u32 {
        unsafe { m5unified_sys::m5u_rtc_set_timer_irq(timer_msec) }
    }

    pub fn set_alarm_irq_after_seconds(&mut self, after_seconds: i32) -> i32 {
        unsafe { m5unified_sys::m5u_rtc_set_alarm_irq_after_seconds(after_seconds) }
    }

    pub fn set_alarm_irq(&mut self, datetime: DateTime) -> i32 {
        let raw = datetime.to_raw();
        unsafe { m5unified_sys::m5u_rtc_set_alarm_irq_datetime(&raw) }
    }

    pub fn set_alarm_irq_time(&mut self, time: Time) -> i32 {
        let raw = time.to_raw();
        unsafe { m5unified_sys::m5u_rtc_set_alarm_irq_time(&raw) }
    }

    pub fn irq_status(&self) -> bool {
        unsafe { m5unified_sys::m5u_rtc_get_irq_status() }
    }

    pub fn clear_irq(&mut self) {
        unsafe { m5unified_sys::m5u_rtc_clear_irq() }
    }

    pub fn disable_irq(&mut self) {
        unsafe { m5unified_sys::m5u_rtc_disable_irq() }
    }
}
