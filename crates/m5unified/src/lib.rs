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
mod i2c;
mod imu;
mod led;
mod log;
mod power;
mod rtc;
mod sd;
mod system;
mod touch;

pub use audio::{
    AudioQueueState, Mic, MicConfig, RawPlaybackOptions, RecordingOptions, Speaker, SpeakerConfig,
    ToneOptions, WavPlaybackOptions,
};
pub use buttons::{Button, ButtonId, ButtonState, Buttons};
pub use config::{ExternalDisplayConfig, ExternalSpeakerConfig, M5UnifiedConfig};
pub use display::{
    colors, Color565, Display, DisplayFont, DisplayKind, DisplayRef, EpdMode, Point, Rect, Size,
    TextDatum,
};
pub use error::Error;
pub use i2c::{I2cBus, I2cDevice};
pub use imu::{Imu, ImuAxis, ImuData, ImuKind, ImuSensorMask, Vec3};
pub use led::{Led, LedColor, LedType};
pub use log::{Log, LogLevel, LogTarget, RawLogCallback};
pub use power::{
    Axp2101, Axp2101IrqStatus, ChargeState, ExtPortBusConfig, ExtPortMask, Power, PowerType,
};
pub use rtc::{Date, DateTime, Rtc, Time};
pub use sd::{
    sd_begin, sd_begin_with_config, sd_end, sd_is_mounted, SdCard, SdSpiConfig, SD_MOUNT_PATH,
};
pub(crate) use system::raw_display_kinds;
pub use system::{Board, PinName};
pub use touch::{Touch, TouchDetail, TouchPoint, TouchState};

/// Top-level handle for M5Unified-backed board features.
#[derive(Debug)]
pub struct M5Unified {
    pub display: Display,
    pub buttons: Buttons,
    pub mic: Mic,
    pub speaker: Speaker,
    pub in_i2c: I2cBus,
    pub ex_i2c: I2cBus,
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
            in_i2c: I2cBus::INTERNAL,
            ex_i2c: I2cBus::EXTERNAL,
            imu: Imu,
            touch: Touch,
            rtc: Rtc,
            power: Power,
            led: Led,
            log: Log,
        })
    }

    /// Alias for the primary display, matching upstream's `M5.Lcd` naming.
    pub fn lcd(&self) -> &Display {
        &self.display
    }

    /// Mutable alias for the primary display, matching upstream's `M5.Lcd`.
    pub fn lcd_mut(&mut self) -> &mut Display {
        &mut self.display
    }

    pub fn button(&self, index: usize) -> Option<Button> {
        self.buttons.get(index)
    }

    pub fn buttons(&self, index: usize) -> Option<Button> {
        self.button(index)
    }

    pub fn displays(&self, index: usize) -> Option<DisplayRef> {
        self.display(index)
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
    use core::fmt::Write as _;

    use super::*;

    #[test]
    fn display_dimensions_are_available_on_host_stubs() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(m5.display.width() > 0);
        assert!(m5.display.height() > 0);
        assert_eq!(m5.lcd().width(), m5.display.width());
        assert_eq!(m5.lcd_mut().height(), m5.display.height());
        assert!(m5.button(0).is_some());
        assert!(m5.buttons(0).is_some());
        assert!(m5.displays(0).is_some());
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
        m5.mic.set_sample_rate(24_000);
        assert!(!m5.mic.is_running());
        assert_eq!(m5.mic.recording_state(), AudioQueueState::Idle);
        let mut rec_u8 = [0_u8; 8];
        assert!(m5.mic.record_u8(&mut rec_u8));
        assert!(m5.mic.record_u8_at(&mut rec_u8, 24_000));
        assert!(m5.mic.record_i16_with_options(
            &mut [0_i16; 8],
            RecordingOptions {
                sample_rate_hz: 24_000,
                stereo: true,
            }
        ));

        let mut speaker = m5.speaker.config();
        assert_eq!(speaker.sample_rate, 48_000);
        speaker.sample_rate = 96_000;
        speaker.dma_buf_count = 20;
        assert_eq!(m5.speaker.set_config(speaker), Ok(()));
        assert!(!m5.speaker.is_running());
        assert_eq!(m5.speaker.playing_channels(), 0);
        assert_eq!(m5.speaker.channel_playing_state(0), AudioQueueState::Idle);
        assert!(m5.speaker.tone_with_options(
            440.0,
            ToneOptions {
                duration_ms: 100,
                channel: Some(1),
                stop_current_sound: false,
            }
        ));
        assert!(m5
            .speaker
            .tone_with_raw(440.0, &[0x80; 16], ToneOptions::default(), false));
        assert!(m5.speaker.play_i8_with_options(
            &[0_i8; 8],
            RawPlaybackOptions {
                sample_rate_hz: 22_050,
                stereo: false,
                repeat: 2,
                channel: Some(2),
                stop_current_sound: true,
            }
        ));
        assert!(m5
            .speaker
            .play_i16_with_options(&[0_i16; 8], RawPlaybackOptions::default()));
        assert!(m5
            .speaker
            .play_u8_with_options(&[0_u8; 8], RawPlaybackOptions::default()));
        assert!(m5
            .speaker
            .play_wav_with_options(&[0_u8; 44], WavPlaybackOptions::default()));
    }

    #[test]
    fn imu_combined_data_uses_host_stub() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert_eq!(m5.imu.kind(), ImuKind::None);
        assert!(m5.imu.init());
        assert!(!m5.imu.begin_for_board(Board::M5Stack));
        assert!(!m5.imu.init_for_board(Board::M5Stack));
        assert!(m5.imu.update());
        assert!(m5.imu.update_mask().is_empty());
        let data = m5.imu.data().expect("host stub imu data should exist");
        assert_eq!(data.usec, 0);
        assert_eq!(data.accel.z, 1.0);
        assert_eq!(data.gyro, Vec3::default());
        assert_eq!(data.mag, Vec3::default());
        assert_eq!(data.temperature_c, Some(25.0));
        assert_eq!(m5.imu.accel_data().unwrap().z, 1.0);
        assert_eq!(m5.imu.gyro_data(), Some(Vec3::default()));
        assert_eq!(m5.imu.gyro_mag(), Some(Vec3::default()));
        assert!(m5
            .imu
            .set_axis_order(ImuAxis::XPos, ImuAxis::YPos, ImuAxis::ZPos));
        assert!(m5
            .imu
            .set_axis_order_right_handed(ImuAxis::XPos, ImuAxis::YPos));
        assert!(m5
            .imu
            .set_axis_order_left_handed(ImuAxis::XPos, ImuAxis::YPos));
        assert!(m5.imu.set_int_pin_active_logic(true));
        m5.imu.set_clock_hz(400_000);
        m5.imu.set_calibration_strength(1, 2, 3);
        m5.imu.clear_offset_data();
        m5.imu.set_offset_data(0, 123);
        assert_eq!(m5.imu.offset_data_i32(0), 0);
        assert_eq!(m5.imu.raw_data(0), 0);
        assert!(m5.imu.sleep());

        let mask = ImuSensorMask::from_raw(ImuSensorMask::ACCEL.raw() | ImuSensorMask::GYRO.raw());
        assert!(mask.contains(ImuSensorMask::ACCEL));
        assert!(mask.contains(ImuSensorMask::GYRO));
        assert!(!mask.contains(ImuSensorMask::MAG));
        assert_eq!(ImuKind::Bmi270.raw(), 6);
    }

    #[test]
    fn led_host_stub_reports_disabled() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(!m5.led.is_enabled());
        assert_eq!(m5.led.count(), 0);
        assert_eq!(m5.led.led_type(0), LedType::Unknown);
        m5.led.set_all_color(LedColor::RED);
        m5.led
            .set_colors(0, &[LedColor::RED, LedColor::GREEN, LedColor::BLUE]);
    }

    #[test]
    fn system_host_stub_reports_unknown_board_and_no_pins() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert_eq!(m5.board(), Board::Unknown);
        assert_eq!(m5.get_pin(PinName::PORT_A_SDA), None);
        assert!(m5.set_primary_display(0));
        assert!(!m5.set_primary_display_type(DisplayKind::ModuleDisplay));
        assert!(
            !m5.set_primary_display_types(&[DisplayKind::UnitOled, DisplayKind::ModuleDisplay,])
        );
        m5.set_touch_button_height(32);
        m5.set_log_display_types(&[DisplayKind::UnitOled, DisplayKind::ModuleDisplay]);
        assert_eq!(m5.touch_button_height(), 0);
        assert_eq!(m5.millis(), 0);
        assert_eq!(m5.micros(), 0);
        assert_eq!(m5.update_msec(), 0);
        assert_eq!(Board::from_raw(1), Board::M5Stack);
        assert_eq!(Board::from_raw(199), Board::M5ModuleDisplay);
        assert_eq!(Board::from_raw(999), Board::Raw(999));
        assert_eq!(Board::M5AtomS3Lite.raw(), 137);
        assert_eq!(DisplayKind::ModuleDisplay.raw(), 199);
        assert_eq!(
            m5.display_index_any(&[DisplayKind::UnitOled, DisplayKind::ModuleDisplay]),
            None
        );
        assert!(!m5.in_i2c.is_enabled());
        assert_eq!(m5.in_i2c.port(), None);
        assert_eq!(m5.in_i2c.sda_pin(), None);
        assert_eq!(m5.in_i2c.scl_pin(), None);
        m5.ex_i2c.set_port(0, 1, 2);
        assert!(!m5.ex_i2c.begin());
        assert!(!m5.ex_i2c.begin_with_port(0, 1, 2));
        assert!(!m5.ex_i2c.start(0x42, false, 100_000));
        assert!(!m5.ex_i2c.restart(0x42, true, 100_000));
        assert!(!m5.ex_i2c.write_byte(0xAA));
        assert!(!m5.ex_i2c.write(&[1, 2, 3]));
        assert!(!m5.ex_i2c.read(&mut [0_u8; 2], true));
        assert!(!m5.ex_i2c.write_register(0x42, 0x10, &[1], 100_000));
        assert!(!m5.ex_i2c.read_register(0x42, 0x10, &mut [0_u8; 2], 100_000));
        assert!(!m5.ex_i2c.write_register8(0x42, 0x10, 1, 100_000));
        assert_eq!(m5.ex_i2c.read_register8(0x42, 0x10, 100_000), 0);
        assert!(!m5.ex_i2c.bit_on(0x42, 0x10, 0x01, 100_000));
        assert!(!m5.ex_i2c.bit_off(0x42, 0x10, 0x01, 100_000));
        assert!(!m5.ex_i2c.scan_address(0x42, 100_000));
        assert_eq!(m5.ex_i2c.scan(100_000), [false; 120]);
        assert!(!m5.ex_i2c.stop());
        assert!(!m5.ex_i2c.release());

        let mut device = I2cDevice::external(0x42, 100_000);
        assert_eq!(device.address(), 0x42);
        assert_eq!(device.clock_hz(), 100_000);
        assert!(!device.is_bus_enabled());
        device.set_address(0x43);
        device.set_clock(400_000);
        device.set_port(I2cBus::INTERNAL);
        assert_eq!(device.address(), 0x43);
        assert_eq!(device.clock_hz(), 400_000);
        assert_eq!(device.bus(), I2cBus::INTERNAL);
        assert!(!device.write_register8(0x10, 0x01));
        assert_eq!(device.read_register8(0x10), 0);
        assert!(!device.write_register8_array(&[0x10, 0x01, 0x11, 0x02]));
        assert!(!device.write_register8_array(&[0x10]));
        assert!(!device.write_register8_pairs(&[(0x10, 0x01)]));
        assert!(!device.write_register(0x10, &[1, 2]));
        assert!(!device.read_register(0x10, &mut [0_u8; 2]));
        assert!(!device.bit_on(0x10, 0x01));
        assert!(!device.bit_off(0x10, 0x01));
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
        assert_eq!(config.to_raw().fallback_board, -1);

        let config = M5UnifiedConfig {
            fallback_board: Some(Board::M5AtomS3Lite),
            ..M5UnifiedConfig::default()
        };
        assert_eq!(config.to_raw().fallback_board, 137);
    }

    #[test]
    fn button_state_helpers_compile_on_host() {
        let m5 = M5Unified::begin().expect("host stub begin should succeed");
        let button = m5.buttons.a();

        assert!(m5.buttons.get(0).is_some());
        assert!(m5.buttons.get(4).is_some());
        assert!(m5.buttons.get(5).is_none());
        assert_eq!(ButtonState::from_raw(0), ButtonState::NoChange);
        assert_eq!(ButtonState::Raw(9).raw(), 9);
        assert!(!button.is_pressed());
        assert!(button.is_released());
        assert!(!button.was_pressed());
        assert!(!button.was_released());
        assert!(!button.was_released_after_hold());
        assert!(!button.was_clicked());
        assert!(!button.was_single_clicked());
        assert!(!button.was_double_clicked());
        assert!(!button.was_hold());
        assert!(!button.is_holding());
        assert!(!button.was_change_pressed());
        assert!(!button.was_decide_click_count());
        #[allow(deprecated)]
        let was_decied_click_count = button.was_decied_click_count();
        assert!(!was_decied_click_count);
        assert_eq!(button.click_count(), 0);
        assert!(!button.was_release_for_ms(10));
        #[allow(deprecated)]
        let was_releasefor_ms = button.was_releasefor_ms(10);
        assert!(!was_releasefor_ms);
        assert!(!button.pressed_for_ms(10));
        assert!(button.released_for_ms(10));
        button.set_debounce_thresh_ms(12);
        button.set_hold_thresh_ms(600);
        button.set_raw_state(20, true);
        button.set_state_at(30, ButtonState::Clicked);
        assert_eq!(button.state(), ButtonState::NoChange);
        assert_eq!(button.last_change_ms(), 0);
        assert_eq!(button.debounce_thresh_ms(), 10);
        assert_eq!(button.hold_thresh_ms(), 500);
        assert_eq!(button.update_msec(), 0);
    }

    #[test]
    fn touch_state_helpers_compile_on_host() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(!m5.touch.is_enabled());
        assert!(!m5.touch.is_pressed());
        assert_eq!(m5.touch.count(), 0);
        assert!(m5.touch.points().is_empty());
        assert_eq!(m5.touch.raw_point(0), None);
        assert!(m5.touch.raw_points().is_empty());
        assert_eq!(m5.touch.detail(0), None);
        assert!(m5.touch.details().is_empty());
        m5.touch.set_hold_thresh_ms(500);
        m5.touch.set_flick_thresh_px(8);

        assert_eq!(TouchState::from_raw(0b1111), TouchState::DragBegin);
        assert_eq!(TouchState::Raw(0x80).raw(), 0x80);
        assert!(TouchState::TouchBegin.is_pressed());
        assert!(TouchState::TouchEnd.was_released());
        assert!(TouchState::TouchEnd.was_clicked());
        assert!(TouchState::HoldBegin.is_holding());
        assert!(TouchState::HoldBegin.was_hold());
        assert!(TouchState::FlickBegin.was_flick_start());
        assert!(TouchState::FlickBegin.is_flicking());
        assert!(TouchState::FlickEnd.was_flicked());
        assert!(TouchState::DragBegin.was_drag_start());
        assert!(TouchState::DragBegin.is_dragging());
        assert!(TouchState::DragEnd.was_dragged());

        let detail = TouchDetail {
            x: 12,
            y: 15,
            prev_x: 10,
            prev_y: 11,
            base_x: 4,
            base_y: 5,
            state: TouchState::DragBegin,
            is_pressed: true,
            was_pressed: true,
            was_clicked: false,
            was_released: false,
            was_hold: false,
            is_holding: true,
            click_count: 2,
            ..TouchDetail::default()
        };
        assert!(detail.is_pressed());
        assert!(detail.was_pressed());
        assert!(!detail.was_clicked());
        assert!(!detail.was_released());
        assert!(detail.is_holding());
        assert!(!detail.was_hold());
        assert_eq!(detail.click_count(), 2);
        assert_eq!(detail.delta_x(), 2);
        assert_eq!(detail.delta_y(), 4);
        assert_eq!(detail.delta(), (2, 4));
        assert_eq!(detail.distance_x(), 8);
        assert_eq!(detail.distance_y(), 10);
        assert_eq!(detail.distance(), (8, 10));
        assert!(detail.was_drag_start());
        assert!(detail.is_dragging());
    }

    #[test]
    fn display_and_rtc_example_helpers_compile_on_host() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(m5.display.set_font(DisplayFont::Ascii8x16));
        assert!(m5.display.set_font(DisplayFont::LgfxJapanGothic12));
        assert!(m5.display.set_font(DisplayFont::DejaVu18));
        m5.display.set_text_scroll(true);
        m5.display.set_epd_mode(EpdMode::Fastest);
        assert!(write!(&mut m5.display, "primary value={}", 7).is_ok());
        m5.rtc.set_system_time_from_rtc();
        assert!(!m5.rtc.volt_low());
        let date = m5.rtc.get_date().expect("host stub date should exist");
        let time = m5.rtc.get_time().expect("host stub time should exist");
        let datetime = DateTime::from_date_time(date, time);
        assert_eq!(datetime.date(), date);
        assert_eq!(datetime.time(), time);
        assert!(m5.rtc.set_date(date));
        assert!(m5.rtc.set_time(time));
        assert_eq!(m5.rtc.set_timer_irq_ms(250), 250);
        assert_eq!(m5.rtc.set_alarm_irq_after_seconds(2), 2);
        assert_eq!(m5.rtc.set_alarm_irq_time(time), 0);
        assert!(!m5.rtc.irq_status());
        m5.rtc.clear_irq();
        m5.rtc.disable_irq();

        let mut display = m5.display(0).expect("host stub display should exist");
        assert_eq!(display.width(), 320);
        assert_eq!(display.height(), 240);
        assert_eq!(display.rotation(), 0);
        display.clear();
        display.fill_screen(colors::BLACK);
        display.set_cursor(0, 0);
        display.set_text_size(1);
        display.set_text_color(colors::WHITE, colors::BLACK);
        display.set_rotation(1);
        display.set_color(colors::YELLOW);
        assert!(write!(&mut display, "indexed value={}", 3).is_ok());
        assert_eq!(display.println("indexed display"), Ok(()));
        assert_eq!(display.draw_string("indexed", 0, 0), Ok(0));
        display.transaction(|display| {
            display.write_pixel(0, 0, colors::WHITE);
            display.draw_pixel(1, 1, colors::RED);
            display.draw_line(0, 0, 8, 8, colors::GREEN);
            display.draw_rect(0, 0, 6, 6, colors::CYAN);
            display.fill_rect(0, 0, 4, 4, colors::BLUE);
            display.draw_circle(8, 8, 3, colors::MAGENTA);
            display.fill_circle(8, 8, 2, colors::GREEN);
        });
    }

    #[test]
    fn log_configuration_helpers_compile_on_host() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(m5.log.set_enable_color(LogTarget::Serial, true));
        assert!(m5.log.enable_color(LogTarget::Serial));
        assert!(m5.log.set_log_level(LogTarget::Display, LogLevel::Debug));
        assert!(m5.log.log_level(LogTarget::Display).is_some());
        assert_eq!(m5.log.set_suffix(LogTarget::Callback, ""), Ok(true));
        assert_eq!(
            Log::path_to_file_name("/tmp/example/source.cpp"),
            Ok("source.cpp".to_string())
        );
        m5.log.println_empty();
        assert!(write!(&mut m5.log, "formatted log {}", 42).is_ok());
        m5.log.dump(&[0, 1, 2, 3], LogLevel::Info);
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
        assert_eq!(
            SdCard::path_for("sound.wav"),
            std::path::PathBuf::from("/sdcard/sound.wav")
        );
        assert_eq!(
            SdCard::path_for("/tmp/sound.wav"),
            std::path::PathBuf::from("/tmp/sound.wav")
        );
        assert!(!sd_begin());
        assert!(!sd_begin_with_config(&config));
        assert!(!sd_is_mounted());
        assert!(SdCard::mount().is_err());
        assert!(SdCard::mount_with_config(&config).is_err());
        sd_end();
    }

    #[test]
    fn axp2101_irq_helpers_compile_on_host() {
        let mut m5 = M5Unified::begin().expect("host stub begin should succeed");
        assert!(!m5.power.begin());
        assert_eq!(m5.power.pmic_type(), PowerType::Unknown);
        assert_eq!(m5.power.charge_state(), ChargeState::Unknown);
        assert_eq!(m5.power.vbus_voltage_mv(), None);
        assert_eq!(m5.power.battery_current_ma(), 0);
        assert_eq!(m5.power.ext_voltage_mv(ExtPortMask::PA), 0.0);
        assert_eq!(m5.power.ext_current_ma(ExtPortMask::PA), 0.0);
        assert_eq!(m5.power.key_state(), 0);
        m5.power.set_led(32);
        m5.power
            .set_ext_output(true, ExtPortMask::PA | ExtPortMask::PB1);
        m5.power.set_ext_power(true);
        assert!(!m5.power.ext_output());
        m5.power.set_usb_output(true);
        assert!(!m5.power.usb_output());
        m5.power.set_battery_charge(true);
        m5.power.set_charge_current_ma(500);
        m5.power.set_charge_voltage_mv(4_200);
        m5.power.set_vibration(0);
        m5.power.set_ext_port_bus_config(ExtPortBusConfig {
            voltage_mv: 5_000,
            current_limit_ma: 100,
            enable: true,
            direction_output: true,
        });
        m5.power.timer_sleep_seconds(0);
        m5.power.timer_sleep_time(Time {
            hour: 6,
            minute: 30,
            second: 0,
        });
        m5.power.timer_sleep_date_time(
            Date {
                year: 2026,
                month: 5,
                day: 18,
                weekday: Some(1),
            },
            Time {
                hour: 6,
                minute: 30,
                second: 0,
            },
        );
        m5.power.deep_sleep_us(0, false);
        m5.power.light_sleep_us(0, false);
        m5.power.power_off();
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
