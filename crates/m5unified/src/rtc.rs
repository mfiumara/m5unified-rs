#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

#[derive(Debug)]
pub struct Rtc;

impl Rtc {
    pub fn get_datetime(&self) -> Option<DateTime> {
        let (mut year, mut month, mut day, mut hour, mut minute, mut second) = (0, 0, 0, 0, 0, 0);
        let ok = unsafe {
            m5unified_sys::m5u_rtc_get_datetime(
                &mut year,
                &mut month,
                &mut day,
                &mut hour,
                &mut minute,
                &mut second,
            )
        };
        ok.then_some(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    pub fn set_datetime(&mut self, datetime: DateTime) -> bool {
        unsafe {
            m5unified_sys::m5u_rtc_set_datetime(
                datetime.year,
                datetime.month,
                datetime.day,
                datetime.hour,
                datetime.minute,
                datetime.second,
            )
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_rtc_is_enabled() }
    }

    pub fn set_system_time_from_rtc(&mut self) {
        unsafe { m5unified_sys::m5u_rtc_set_system_time_from_rtc() }
    }
}
