use crate::Board;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5UnifiedConfig {
    pub serial_baudrate: u32,
    pub external_speaker: ExternalSpeakerConfig,
    pub external_display: ExternalDisplayConfig,
    pub clear_display: bool,
    pub output_power: bool,
    pub pmic_button: bool,
    pub internal_imu: bool,
    pub internal_rtc: bool,
    pub internal_mic: bool,
    pub internal_spk: bool,
    pub external_imu: bool,
    pub external_rtc: bool,
    pub disable_rtc_irq: bool,
    pub led_brightness: u8,
    pub fallback_board: Option<Board>,
}

impl M5UnifiedConfig {
    pub(crate) fn to_raw(&self) -> m5unified_sys::m5u_config_t {
        m5unified_sys::m5u_config_t {
            serial_baudrate: self.serial_baudrate,
            external_speaker_value: self.external_speaker.bits(),
            external_display_value: self.external_display.bits(),
            clear_display: u8::from(self.clear_display),
            output_power: u8::from(self.output_power),
            pmic_button: u8::from(self.pmic_button),
            internal_imu: u8::from(self.internal_imu),
            internal_rtc: u8::from(self.internal_rtc),
            internal_mic: u8::from(self.internal_mic),
            internal_spk: u8::from(self.internal_spk),
            external_imu: u8::from(self.external_imu),
            external_rtc: u8::from(self.external_rtc),
            disable_rtc_irq: u8::from(self.disable_rtc_irq),
            led_brightness: self.led_brightness,
            fallback_board: self.fallback_board.map(Board::raw).unwrap_or(-1),
        }
    }
}

impl Default for M5UnifiedConfig {
    fn default() -> Self {
        Self {
            serial_baudrate: 0,
            external_speaker: ExternalSpeakerConfig::default(),
            external_display: ExternalDisplayConfig::default(),
            clear_display: true,
            output_power: true,
            pmic_button: true,
            internal_imu: true,
            internal_rtc: true,
            internal_mic: true,
            internal_spk: true,
            external_imu: false,
            external_rtc: false,
            disable_rtc_irq: true,
            led_brightness: 0,
            fallback_board: None,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ExternalSpeakerConfig {
    pub module_display: bool,
    pub module_rca: bool,
    pub hat_spk: bool,
    pub atomic_spk: bool,
    pub hat_spk2: bool,
    pub atomic_echo: bool,
}

impl ExternalSpeakerConfig {
    fn bits(self) -> u8 {
        u8::from(self.module_display)
            | (u8::from(self.module_rca) << 1)
            | (u8::from(self.hat_spk) << 2)
            | (u8::from(self.atomic_spk) << 3)
            | (u8::from(self.hat_spk2) << 4)
            | (u8::from(self.atomic_echo) << 5)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExternalDisplayConfig {
    pub module_display: bool,
    pub atom_display: bool,
    pub unit_oled: bool,
    pub unit_mini_oled: bool,
    pub unit_lcd: bool,
    pub unit_glass: bool,
    pub unit_glass2: bool,
    pub unit_rca: bool,
    pub module_rca: bool,
}

impl ExternalDisplayConfig {
    fn bits(self) -> u16 {
        u16::from(self.module_display)
            | (u16::from(self.atom_display) << 1)
            | (u16::from(self.unit_oled) << 2)
            | (u16::from(self.unit_mini_oled) << 3)
            | (u16::from(self.unit_lcd) << 4)
            | (u16::from(self.unit_glass) << 5)
            | (u16::from(self.unit_glass2) << 6)
            | (u16::from(self.unit_rca) << 7)
            | (u16::from(self.module_rca) << 8)
            | 0xFE00
    }
}

impl Default for ExternalDisplayConfig {
    fn default() -> Self {
        Self {
            module_display: true,
            atom_display: true,
            unit_oled: true,
            unit_mini_oled: true,
            unit_lcd: true,
            unit_glass: true,
            unit_glass2: true,
            unit_rca: true,
            module_rca: true,
        }
    }
}
