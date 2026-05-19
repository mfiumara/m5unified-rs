//! Rust port of M5Unified's `M5Timer` callback scheduler.

pub struct M5Timer {
    timers: Vec<TimerInfo>,
    active_count: usize,
}

impl M5Timer {
    pub const MAX_TIMERS: usize = 10;

    pub fn new() -> Self {
        Self {
            timers: (0..Self::MAX_TIMERS)
                .map(|_| TimerInfo::default())
                .collect(),
            active_count: 0,
        }
    }

    /// Run due timers using M5Unified's millisecond clock.
    pub fn run(&mut self) {
        self.run_at_millis(unsafe { m5unified_sys::m5u_millis() });
    }

    /// Call a function every interval for the specified number of times.
    ///
    /// Pass `0` for `times` to run until the timer is deleted.
    pub fn set_timer<F>(&mut self, interval_ms: u32, function: F, times: u32) -> Option<usize>
    where
        F: FnMut() + 'static,
    {
        if self.active_count >= Self::MAX_TIMERS {
            return None;
        }

        for (id, timer) in self.timers.iter_mut().enumerate() {
            if timer.has_callback() {
                continue;
            }
            timer.set(interval_ms, Box::new(function), times, unsafe {
                m5unified_sys::m5u_millis()
            });
            self.active_count += 1;
            return Some(id);
        }

        None
    }

    /// Call a function repeatedly at the given interval.
    pub fn set_interval<F>(&mut self, interval_ms: u32, function: F) -> Option<usize>
    where
        F: FnMut() + 'static,
    {
        self.set_timer(interval_ms, function, 0)
    }

    /// Call a function once after the given interval.
    pub fn set_timeout<F>(&mut self, interval_ms: u32, function: F) -> Option<usize>
    where
        F: FnMut() + 'static,
    {
        self.set_timer(interval_ms, function, 1)
    }

    pub fn delete_timer(&mut self, id: usize) {
        if let Some(timer) = self.timers.get_mut(id) {
            if timer.has_callback() {
                timer.clear();
                self.active_count = self.active_count.saturating_sub(1);
            }
        }
    }

    pub fn restart_timer(&mut self, id: usize) {
        if let Some(timer) = self.timers.get_mut(id) {
            timer.previous_ms = unsafe { m5unified_sys::m5u_millis() };
        }
    }

    pub fn enable(&mut self, id: usize) {
        if let Some(timer) = self.timers.get_mut(id) {
            timer.enabled = true;
        }
    }

    pub fn disable(&mut self, id: usize) {
        if let Some(timer) = self.timers.get_mut(id) {
            timer.enabled = false;
        }
    }

    pub fn toggle(&mut self, id: usize) {
        if let Some(timer) = self.timers.get_mut(id) {
            timer.enabled = !timer.enabled;
        }
    }

    pub fn is_enabled(&self, id: usize) -> bool {
        self.timers.get(id).is_some_and(|timer| timer.enabled)
    }

    pub fn num_timers(&self) -> usize {
        self.active_count
    }

    pub fn num_available_timers(&self) -> usize {
        Self::MAX_TIMERS - self.active_count
    }

    fn run_at_millis(&mut self, current_ms: u32) {
        for timer in &mut self.timers {
            if !timer.has_callback() {
                continue;
            }
            if !timer.run(current_ms) {
                timer.clear();
                self.active_count = self.active_count.saturating_sub(1);
            }
        }
    }
}

impl Default for M5Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for M5Timer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("M5Timer")
            .field("num_timers", &self.num_timers())
            .field("num_available_timers", &self.num_available_timers())
            .finish()
    }
}

#[derive(Default)]
struct TimerInfo {
    callback: Option<Box<dyn FnMut()>>,
    previous_ms: u32,
    interval_ms: u32,
    remaining: u32,
    enabled: bool,
}

impl TimerInfo {
    fn has_callback(&self) -> bool {
        self.callback.is_some()
    }

    fn set(&mut self, interval_ms: u32, callback: Box<dyn FnMut()>, times: u32, current_ms: u32) {
        self.callback = Some(callback);
        self.interval_ms = interval_ms;
        self.remaining = times;
        self.enabled = true;
        self.previous_ms = current_ms;
    }

    fn clear(&mut self) {
        self.callback = None;
        self.enabled = false;
        self.interval_ms = 0;
        self.remaining = 0;
    }

    fn run(&mut self, current_ms: u32) -> bool {
        if current_ms.wrapping_sub(self.previous_ms) < self.interval_ms {
            return true;
        }

        self.previous_ms = self.previous_ms.wrapping_add(self.interval_ms);

        if self.enabled {
            if let Some(callback) = self.callback.as_mut() {
                callback();
            }
            if self.remaining != 0 {
                self.remaining -= 1;
                return self.remaining != 0;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn timeout_runs_once_and_releases_slot() {
        let calls = Rc::new(Cell::new(0));
        let observed = calls.clone();
        let mut timer = M5Timer::new();

        let id = timer
            .set_timeout(5, move || observed.set(observed.get() + 1))
            .expect("timer slot should be available");

        assert_eq!(id, 0);
        assert_eq!(timer.num_timers(), 1);
        timer.run_at_millis(4);
        assert_eq!(calls.get(), 0);
        timer.run_at_millis(5);
        assert_eq!(calls.get(), 1);
        assert_eq!(timer.num_timers(), 0);
        assert!(!timer.is_enabled(id));
    }

    #[test]
    fn interval_enable_disable_toggle_and_delete_match_upstream_semantics() {
        let calls = Rc::new(Cell::new(0));
        let observed = calls.clone();
        let mut timer = M5Timer::new();

        let id = timer
            .set_interval(10, move || observed.set(observed.get() + 1))
            .expect("timer slot should be available");

        assert!(timer.is_enabled(id));
        timer.run_at_millis(10);
        timer.run_at_millis(20);
        assert_eq!(calls.get(), 2);

        timer.disable(id);
        timer.run_at_millis(30);
        assert_eq!(calls.get(), 2);
        assert_eq!(timer.num_timers(), 1);

        timer.toggle(id);
        assert!(timer.is_enabled(id));
        timer.run_at_millis(40);
        assert_eq!(calls.get(), 3);

        timer.delete_timer(id);
        assert_eq!(timer.num_timers(), 0);
        assert_eq!(timer.num_available_timers(), M5Timer::MAX_TIMERS);
    }

    #[test]
    fn rejects_more_than_max_timers() {
        let mut timer = M5Timer::new();

        for _ in 0..M5Timer::MAX_TIMERS {
            assert!(timer.set_interval(1, || {}).is_some());
        }
        assert_eq!(timer.num_available_timers(), 0);
        assert!(timer.set_interval(1, || {}).is_none());
    }
}
