//! Safe Rust wrapper for a small M5Unified C ABI surface.
//!
//! The API is intentionally shaped around M5Unified's common examples while
//! keeping Rust call sites safe and host-checkable. Hardware calls are provided
//! by `m5unified-sys`; on non-ESP-IDF targets that crate supplies no-op stubs so
//! examples compile in CI.
//!
//! # Example
//!
//! ```no_run
//! use m5unified::{colors, M5Unified};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut m5 = M5Unified::begin()?;
//!
//!     m5.display.fill_screen(colors::BLACK);
//!     m5.display.set_text_size(2);
//!     m5.display.println("hello from Rust")?;
//!
//!     loop {
//!         m5.update();
//!         if m5.buttons.a().was_pressed() {
//!             m5.display.println("Button A")?;
//!         }
//!         m5.delay_ms(16);
//!     }
//! }
//! ```

mod audio;
mod buttons;
mod config;
mod display;
mod error;
mod imu;
mod led;
mod log;
mod power;
mod rtc;
mod sd;
mod system;
mod touch;

pub use audio::{Mic, MicConfig, Speaker, SpeakerConfig};
pub use buttons::{Button, ButtonId, Buttons};
pub use config::{ExternalDisplayConfig, ExternalSpeakerConfig, M5UnifiedConfig};
pub use display::{
    colors, Color565, Display, DisplayFont, DisplayKind, DisplayRef, EpdMode, Point, Rect, Size,
    TextDatum,
};
pub use error::Error;
pub use imu::{Imu, ImuData, ImuKind, Vec3};
pub use led::{Led, LedColor};
pub use log::{Log, LogLevel, LogTarget, RawLogCallback};
pub use power::{Axp2101, Axp2101IrqStatus, Power};
pub use rtc::{DateTime, Rtc};
pub use sd::{sd_begin, sd_begin_with_config, sd_end, sd_is_mounted, SdSpiConfig, SD_MOUNT_PATH};
pub use system::{Board, PinName};
pub use touch::{Touch, TouchDetail, TouchPoint};

/// Top-level handle for M5Unified-backed board features.
#[derive(Debug)]
pub struct M5Unified {
    pub display: Display,
    pub buttons: Buttons,
    pub mic: Mic,
    pub speaker: Speaker,
    pub imu: Imu,
    pub touch: Touch,
    pub rtc: Rtc,
    pub power: Power,
    pub led: Led,
    pub log: Log,
}

impl M5Unified {
    /// Initialize M5Unified and return a board handle.
    pub fn begin() -> Result<Self, Error> {
        let ok = unsafe { m5unified_sys::m5u_begin() };
        Self::from_begin_result(ok)
    }

    pub fn begin_with_config(config: &M5UnifiedConfig) -> Result<Self, Error> {
        let raw = config.to_raw();
        let ok = unsafe { m5unified_sys::m5u_begin_with_config(&raw) };
        Self::from_begin_result(ok)
    }

    fn from_begin_result(ok: bool) -> Result<Self, Error> {
        if !ok {
            return Err(Error::BeginFailed);
        }

        Ok(Self {
            display: Display,
            buttons: Buttons,
            mic: Mic,
            speaker: Speaker,
            imu: Imu,
            touch: Touch,
            rtc: Rtc,
            power: Power,
            led: Led,
            log: Log,
        })
    }

    /// Poll/update M5Unified internals, including button edge state.
    pub fn update(&mut self) {
        unsafe { m5unified_sys::m5u_update() }
    }

    /// Delay execution. On host builds this is currently a no-op.
    pub fn delay_ms(&self, ms: u32) {
        unsafe { m5unified_sys::m5u_delay_ms(ms) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_dimensions_are_available_on_host_stubs() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(m5.display.width() > 0);
        assert!(m5.display.height() > 0);
    }

    #[test]
    fn invalid_strings_are_rejected_before_ffi() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert_eq!(m5.display.print("bad\0string"), Err(Error::InvalidString));
    }

    #[test]
    fn mic_rms_uses_recorded_buffer() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        let mut buffer = [0_i16; 8];
        assert_eq!(m5.mic.rms(&mut buffer), Some(0.0));
    }

    #[test]
    fn audio_config_helpers_compile_on_host() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");

        let mut mic = m5.mic.config();
        assert_eq!(mic.sample_rate, 16_000);
        mic.dma_buf_count = 3;
        mic.dma_buf_len = 256;
        mic.noise_filter_level = 8;
        assert_eq!(m5.mic.set_config(mic), Ok(()));

        let mut speaker = m5.speaker.config();
        assert_eq!(speaker.sample_rate, 48_000);
        speaker.sample_rate = 96_000;
        speaker.dma_buf_count = 20;
        assert_eq!(m5.speaker.set_config(speaker), Ok(()));
    }

    #[test]
    fn imu_combined_data_uses_host_stub() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        let data = m5.imu.data().expect("host stub imu data should exist");
        assert_eq!(data.usec, 0);
        assert_eq!(data.accel.z, 1.0);
        assert_eq!(data.gyro, Vec3::default());
        assert_eq!(data.mag, Vec3::default());
        assert_eq!(data.temperature_c, Some(25.0));
    }

    #[test]
    fn led_host_stub_reports_disabled() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(!m5.led.is_enabled());
        assert_eq!(m5.led.count(), 0);
        m5.led.set_all_color(LedColor::RED);
    }

    #[test]
    fn system_host_stub_reports_unknown_board_and_no_pins() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert_eq!(m5.board(), Board::Unknown);
        assert_eq!(m5.get_pin(PinName::PORT_A_SDA), None);
        assert!(m5.set_primary_display(0));
        assert!(!m5.set_primary_display_type(DisplayKind::ModuleDisplay));
        m5.set_touch_button_height(32);
        assert_eq!(m5.touch_button_height(), 0);
    }

    #[test]
    fn begin_with_config_uses_host_stub() {
        let config = M5UnifiedConfig {
            led_brightness: 32,
            external_imu: true,
            external_rtc: true,
            ..M5UnifiedConfig::default()
        };
        let m5 = M5Unified::begin_with_config(&config).expect("host stub begin should succeed");
        assert_eq!(m5.display.width(), 320);
    }

    #[test]
    fn display_and_rtc_example_helpers_compile_on_host() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(m5.display.set_font(DisplayFont::Ascii8x16));
        assert!(m5.display.set_font(DisplayFont::LgfxJapanGothic12));
        assert!(m5.display.set_font(DisplayFont::DejaVu18));
        m5.display.set_text_scroll(true);
        m5.display.set_epd_mode(EpdMode::Fastest);
        m5.rtc.set_system_time_from_rtc();
    }

    #[test]
    fn log_configuration_helpers_compile_on_host() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(m5.log.set_enable_color(LogTarget::Serial, true));
        assert!(m5.log.enable_color(LogTarget::Serial));
        assert!(m5.log.set_log_level(LogTarget::Display, LogLevel::Debug));
        assert!(m5.log.log_level(LogTarget::Display).is_some());
        assert_eq!(m5.log.set_suffix(LogTarget::Callback, ""), Ok(true));
        assert!(m5.log.clear_callback());
    }

    #[test]
    fn sd_helpers_compile_on_host() {
        let config = SdSpiConfig {
            pin_sclk: 18,
            pin_mosi: 23,
            pin_miso: 19,
            pin_cs: 4,
            ..SdSpiConfig::default()
        };
        assert_eq!(SD_MOUNT_PATH, "/sdcard");
        assert!(!sd_begin());
        assert!(!sd_begin_with_config(&config));
        assert!(!sd_is_mounted());
        sd_end();
    }

    #[test]
    fn axp2101_irq_helpers_compile_on_host() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        let axp = m5.power.axp2101();
        let mask = Axp2101::IRQ_BAT_CHG_UNDER_TEMP | Axp2101::IRQ_VBUS_INSERT;
        assert!(!axp.disable_irq(Axp2101::IRQ_ALL));
        assert!(!axp.enable_irq(mask));
        assert!(!axp.clear_irq_statuses());
        let status = axp.irq_statuses();
        assert_eq!(status.raw, 0);
        assert!(!status.battery_charger_under_temperature());
        assert!(!status.vbus_insert());
    }
}
