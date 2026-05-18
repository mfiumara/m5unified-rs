use crate::Error;

fn channel_to_raw(channel: Option<u8>) -> i32 {
    channel.map(i32::from).unwrap_or(-1)
}

#[derive(Debug)]
pub struct Mic;

impl Mic {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_mic_begin() }
    }

    pub fn is_running(&self) -> bool {
        unsafe { m5unified_sys::m5u_mic_is_running() }
    }

    pub fn record_i16(&mut self, buffer: &mut [i16]) -> bool {
        unsafe { m5unified_sys::m5u_mic_record_i16(buffer.as_mut_ptr(), buffer.len()) }
    }

    pub fn record_u8(&mut self, buffer: &mut [u8]) -> bool {
        unsafe { m5unified_sys::m5u_mic_record_u8(buffer.as_mut_ptr(), buffer.len()) }
    }

    pub fn rms(&mut self, buffer: &mut [i16]) -> Option<f32> {
        if !self.record_i16(buffer) || buffer.is_empty() {
            return None;
        }
        let sum_sq: f32 = buffer.iter().map(|&s| (s as f32) * (s as f32)).sum();
        Some((sum_sq / buffer.len() as f32).sqrt())
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_mic_is_enabled() }
    }

    pub fn is_recording(&self) -> bool {
        unsafe { m5unified_sys::m5u_mic_is_recording() }
    }

    pub fn recording_state(&self) -> AudioQueueState {
        AudioQueueState::from_raw(unsafe { m5unified_sys::m5u_mic_recording_state() })
    }

    pub fn end(&mut self) {
        unsafe { m5unified_sys::m5u_mic_end() }
    }

    pub fn set_sample_rate(&mut self, sample_rate_hz: u32) {
        unsafe { m5unified_sys::m5u_mic_set_sample_rate(sample_rate_hz) }
    }

    pub fn record_i16_at(&mut self, buffer: &mut [i16], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_mic_record_i16_at(buffer.as_mut_ptr(), buffer.len(), sample_rate_hz)
        }
    }

    pub fn record_u8_at(&mut self, buffer: &mut [u8], sample_rate_hz: u32) -> bool {
        self.record_u8_with_options(
            buffer,
            RecordingOptions {
                sample_rate_hz,
                stereo: false,
            },
        )
    }

    pub fn record_i16_with_options(
        &mut self,
        buffer: &mut [i16],
        options: RecordingOptions,
    ) -> bool {
        unsafe {
            m5unified_sys::m5u_mic_record_i16_ex(
                buffer.as_mut_ptr(),
                buffer.len(),
                options.sample_rate_hz,
                options.stereo,
            )
        }
    }

    pub fn record_u8_with_options(&mut self, buffer: &mut [u8], options: RecordingOptions) -> bool {
        unsafe {
            m5unified_sys::m5u_mic_record_u8_ex(
                buffer.as_mut_ptr(),
                buffer.len(),
                options.sample_rate_hz,
                options.stereo,
            )
        }
    }

    pub fn config(&self) -> MicConfig {
        let mut raw = m5unified_sys::m5u_mic_config_t::default();
        let ok = unsafe { m5unified_sys::m5u_mic_get_config(&mut raw) };
        if ok {
            MicConfig::from_raw(raw)
        } else {
            MicConfig::default()
        }
    }

    pub fn set_config(&mut self, config: MicConfig) -> Result<(), Error> {
        let raw = config.to_raw();
        unsafe { m5unified_sys::m5u_mic_set_config(&raw) }
            .then_some(())
            .ok_or(Error::Unavailable("microphone config"))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MicConfig {
    pub pin_data_in: i32,
    pub pin_bck: i32,
    pub pin_mck: i32,
    pub pin_ws: i32,
    pub sample_rate: u32,
    pub left_channel: bool,
    pub stereo: bool,
    pub over_sampling: u8,
    pub magnification: u8,
    pub noise_filter_level: u8,
    pub use_adc: bool,
    pub dma_buf_len: usize,
    pub dma_buf_count: usize,
    pub task_priority: u8,
    pub task_pinned_core: u8,
    pub i2s_port: i32,
}

impl MicConfig {
    fn from_raw(raw: m5unified_sys::m5u_mic_config_t) -> Self {
        Self {
            pin_data_in: raw.pin_data_in,
            pin_bck: raw.pin_bck,
            pin_mck: raw.pin_mck,
            pin_ws: raw.pin_ws,
            sample_rate: raw.sample_rate,
            left_channel: raw.left_channel != 0,
            stereo: raw.stereo != 0,
            over_sampling: raw.over_sampling,
            magnification: raw.magnification,
            noise_filter_level: raw.noise_filter_level,
            use_adc: raw.use_adc != 0,
            dma_buf_len: raw.dma_buf_len,
            dma_buf_count: raw.dma_buf_count,
            task_priority: raw.task_priority,
            task_pinned_core: raw.task_pinned_core,
            i2s_port: raw.i2s_port,
        }
    }

    fn to_raw(self) -> m5unified_sys::m5u_mic_config_t {
        m5unified_sys::m5u_mic_config_t {
            pin_data_in: self.pin_data_in,
            pin_bck: self.pin_bck,
            pin_mck: self.pin_mck,
            pin_ws: self.pin_ws,
            sample_rate: self.sample_rate,
            left_channel: u8::from(self.left_channel),
            stereo: u8::from(self.stereo),
            over_sampling: self.over_sampling,
            magnification: self.magnification,
            noise_filter_level: self.noise_filter_level,
            use_adc: u8::from(self.use_adc),
            dma_buf_len: self.dma_buf_len,
            dma_buf_count: self.dma_buf_count,
            task_priority: self.task_priority,
            task_pinned_core: self.task_pinned_core,
            i2s_port: self.i2s_port,
        }
    }
}

impl Default for MicConfig {
    fn default() -> Self {
        Self::from_raw(m5unified_sys::m5u_mic_config_t::default())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RecordingOptions {
    pub sample_rate_hz: u32,
    pub stereo: bool,
}

impl Default for RecordingOptions {
    fn default() -> Self {
        Self {
            sample_rate_hz: 16_000,
            stereo: false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AudioQueueState {
    Idle,
    Active,
    Full,
    Unknown(usize),
}

impl AudioQueueState {
    fn from_raw(raw: usize) -> Self {
        match raw {
            0 => Self::Idle,
            1 => Self::Active,
            2 => Self::Full,
            other => Self::Unknown(other),
        }
    }

    pub fn is_active(self) -> bool {
        !matches!(self, Self::Idle)
    }
}

#[derive(Debug)]
pub struct Speaker;

impl Speaker {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_speaker_begin() }
    }

    pub fn is_running(&self) -> bool {
        unsafe { m5unified_sys::m5u_speaker_is_running() }
    }

    pub fn set_volume(&mut self, volume: u8) {
        unsafe { m5unified_sys::m5u_speaker_set_volume(volume) }
    }

    pub fn tone(&mut self, frequency_hz: u32, duration_ms: u32) -> bool {
        unsafe { m5unified_sys::m5u_speaker_tone(frequency_hz, duration_ms) }
    }

    pub fn play_i16(&mut self, samples: &[i16], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_i16(samples.as_ptr(), samples.len(), sample_rate_hz)
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { m5unified_sys::m5u_speaker_is_enabled() }
    }

    pub fn end(&mut self) {
        unsafe { m5unified_sys::m5u_speaker_end() }
    }

    pub fn volume(&self) -> u8 {
        unsafe { m5unified_sys::m5u_speaker_get_volume() }
    }

    pub fn config(&self) -> SpeakerConfig {
        let mut raw = m5unified_sys::m5u_speaker_config_t::default();
        let ok = unsafe { m5unified_sys::m5u_speaker_get_config(&mut raw) };
        if ok {
            SpeakerConfig::from_raw(raw)
        } else {
            SpeakerConfig::default()
        }
    }

    pub fn set_config(&mut self, config: SpeakerConfig) -> Result<(), Error> {
        let raw = config.to_raw();
        unsafe { m5unified_sys::m5u_speaker_set_config(&raw) }
            .then_some(())
            .ok_or(Error::Unavailable("speaker config"))
    }

    pub fn tone_ex(&mut self, frequency_hz: f32, duration_ms: u32, channel: Option<u8>) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_tone_ex(frequency_hz, duration_ms, channel_to_raw(channel))
        }
    }

    pub fn tone_with_options(&mut self, frequency_hz: f32, options: ToneOptions) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_tone_options(
                frequency_hz,
                options.duration_ms,
                channel_to_raw(options.channel),
                options.stop_current_sound,
            )
        }
    }

    pub fn tone_with_raw(
        &mut self,
        frequency_hz: f32,
        data: &[u8],
        options: ToneOptions,
        stereo: bool,
    ) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_tone_full(
                frequency_hz,
                options.duration_ms,
                channel_to_raw(options.channel),
                options.stop_current_sound,
                data.as_ptr(),
                data.len(),
                stereo,
            )
        }
    }

    pub fn play_u8(&mut self, samples: &[u8], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_u8(samples.as_ptr(), samples.len(), sample_rate_hz)
        }
    }

    pub fn play_u8_with_options(&mut self, samples: &[u8], options: RawPlaybackOptions) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_u8_ex(
                samples.as_ptr(),
                samples.len(),
                options.sample_rate_hz,
                options.stereo,
                options.repeat,
                channel_to_raw(options.channel),
                options.stop_current_sound,
            )
        }
    }

    pub fn play_i8_with_options(&mut self, samples: &[i8], options: RawPlaybackOptions) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_i8_ex(
                samples.as_ptr(),
                samples.len(),
                options.sample_rate_hz,
                options.stereo,
                options.repeat,
                channel_to_raw(options.channel),
                options.stop_current_sound,
            )
        }
    }

    pub fn play_i16_with_options(&mut self, samples: &[i16], options: RawPlaybackOptions) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_i16_ex(
                samples.as_ptr(),
                samples.len(),
                options.sample_rate_hz,
                options.stereo,
                options.repeat,
                channel_to_raw(options.channel),
                options.stop_current_sound,
            )
        }
    }

    pub fn play_wav(&mut self, data: &[u8]) -> bool {
        unsafe { m5unified_sys::m5u_speaker_play_wav(data.as_ptr(), data.len()) }
    }

    pub fn play_wav_with_options(&mut self, data: &[u8], options: WavPlaybackOptions) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_wav_ex(
                data.as_ptr(),
                data.len(),
                options.repeat,
                channel_to_raw(options.channel),
                options.stop_current_sound,
            )
        }
    }

    pub fn is_playing(&self, channel: Option<u8>) -> bool {
        unsafe { m5unified_sys::m5u_speaker_is_playing(channel_to_raw(channel)) }
    }

    pub fn playing_channels(&self) -> usize {
        unsafe { m5unified_sys::m5u_speaker_playing_channels() }
    }

    pub fn channel_playing_state(&self, channel: u8) -> AudioQueueState {
        AudioQueueState::from_raw(unsafe {
            m5unified_sys::m5u_speaker_channel_playing_state(i32::from(channel))
        })
    }

    pub fn stop(&mut self, channel: Option<u8>) {
        unsafe { m5unified_sys::m5u_speaker_stop(channel_to_raw(channel)) }
    }

    pub fn channel_volume(&self, channel: u8) -> u8 {
        unsafe { m5unified_sys::m5u_speaker_get_channel_volume(i32::from(channel)) }
    }

    pub fn set_channel_volume(&mut self, channel: u8, volume: u8) {
        unsafe { m5unified_sys::m5u_speaker_set_channel_volume(i32::from(channel), volume) }
    }

    pub fn set_all_channel_volume(&mut self, volume: u8) {
        unsafe { m5unified_sys::m5u_speaker_set_all_channel_volume(volume) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ToneOptions {
    pub duration_ms: u32,
    pub channel: Option<u8>,
    pub stop_current_sound: bool,
}

impl Default for ToneOptions {
    fn default() -> Self {
        Self {
            duration_ms: u32::MAX,
            channel: None,
            stop_current_sound: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RawPlaybackOptions {
    pub sample_rate_hz: u32,
    pub stereo: bool,
    pub repeat: u32,
    pub channel: Option<u8>,
    pub stop_current_sound: bool,
}

impl Default for RawPlaybackOptions {
    fn default() -> Self {
        Self {
            sample_rate_hz: 44_100,
            stereo: false,
            repeat: 1,
            channel: None,
            stop_current_sound: false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WavPlaybackOptions {
    pub repeat: u32,
    pub channel: Option<u8>,
    pub stop_current_sound: bool,
}

impl Default for WavPlaybackOptions {
    fn default() -> Self {
        Self {
            repeat: 1,
            channel: None,
            stop_current_sound: false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpeakerConfig {
    pub pin_data_out: i32,
    pub pin_bck: i32,
    pub pin_mck: i32,
    pub pin_ws: i32,
    pub sample_rate: u32,
    pub stereo: bool,
    pub buzzer: bool,
    pub use_dac: bool,
    pub dac_zero_level: u8,
    pub magnification: u8,
    pub dma_buf_len: usize,
    pub dma_buf_count: usize,
    pub task_priority: u8,
    pub task_pinned_core: u8,
    pub i2s_port: i32,
}

impl SpeakerConfig {
    fn from_raw(raw: m5unified_sys::m5u_speaker_config_t) -> Self {
        Self {
            pin_data_out: raw.pin_data_out,
            pin_bck: raw.pin_bck,
            pin_mck: raw.pin_mck,
            pin_ws: raw.pin_ws,
            sample_rate: raw.sample_rate,
            stereo: raw.stereo != 0,
            buzzer: raw.buzzer != 0,
            use_dac: raw.use_dac != 0,
            dac_zero_level: raw.dac_zero_level,
            magnification: raw.magnification,
            dma_buf_len: raw.dma_buf_len,
            dma_buf_count: raw.dma_buf_count,
            task_priority: raw.task_priority,
            task_pinned_core: raw.task_pinned_core,
            i2s_port: raw.i2s_port,
        }
    }

    fn to_raw(self) -> m5unified_sys::m5u_speaker_config_t {
        m5unified_sys::m5u_speaker_config_t {
            pin_data_out: self.pin_data_out,
            pin_bck: self.pin_bck,
            pin_mck: self.pin_mck,
            pin_ws: self.pin_ws,
            sample_rate: self.sample_rate,
            stereo: u8::from(self.stereo),
            buzzer: u8::from(self.buzzer),
            use_dac: u8::from(self.use_dac),
            dac_zero_level: self.dac_zero_level,
            magnification: self.magnification,
            dma_buf_len: self.dma_buf_len,
            dma_buf_count: self.dma_buf_count,
            task_priority: self.task_priority,
            task_pinned_core: self.task_pinned_core,
            i2s_port: self.i2s_port,
        }
    }
}

impl Default for SpeakerConfig {
    fn default() -> Self {
        Self::from_raw(m5unified_sys::m5u_speaker_config_t::default())
    }
}
