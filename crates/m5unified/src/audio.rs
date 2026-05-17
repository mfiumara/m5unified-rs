use core::ffi::c_int;

use crate::Error;

#[derive(Debug)]
pub struct Mic;

impl Mic {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_mic_begin() }
    }

    pub fn record_i16(&mut self, buffer: &mut [i16]) -> bool {
        unsafe { m5unified_sys::m5u_mic_record_i16(buffer.as_mut_ptr(), buffer.len()) }
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

    pub fn end(&mut self) {
        unsafe { m5unified_sys::m5u_mic_end() }
    }

    pub fn record_i16_at(&mut self, buffer: &mut [i16], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_mic_record_i16_at(buffer.as_mut_ptr(), buffer.len(), sample_rate_hz)
        }
    }

    pub fn config(&self) -> MicConfig {
        MicConfig {
            noise_filter_level: unsafe { m5unified_sys::m5u_mic_get_noise_filter_level() as i32 },
        }
    }

    pub fn set_config(&mut self, config: MicConfig) -> Result<(), Error> {
        unsafe { m5unified_sys::m5u_mic_set_noise_filter_level(config.noise_filter_level as c_int) }
            .then_some(())
            .ok_or(Error::Unavailable("microphone config"))
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct MicConfig {
    pub noise_filter_level: i32,
}

#[derive(Debug)]
pub struct Speaker;

impl Speaker {
    pub fn begin(&mut self) -> bool {
        unsafe { m5unified_sys::m5u_speaker_begin() }
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

    pub fn tone_ex(&mut self, frequency_hz: f32, duration_ms: u32, channel: Option<u8>) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_tone_ex(
                frequency_hz,
                duration_ms,
                channel.map(i32::from).unwrap_or(-1),
            )
        }
    }

    pub fn play_u8(&mut self, samples: &[u8], sample_rate_hz: u32) -> bool {
        unsafe {
            m5unified_sys::m5u_speaker_play_u8(samples.as_ptr(), samples.len(), sample_rate_hz)
        }
    }

    pub fn play_wav(&mut self, data: &[u8]) -> bool {
        unsafe { m5unified_sys::m5u_speaker_play_wav(data.as_ptr(), data.len()) }
    }

    pub fn is_playing(&self, channel: Option<u8>) -> bool {
        unsafe { m5unified_sys::m5u_speaker_is_playing(channel.map(i32::from).unwrap_or(-1)) }
    }

    pub fn stop(&mut self, channel: Option<u8>) {
        unsafe { m5unified_sys::m5u_speaker_stop(channel.map(i32::from).unwrap_or(-1)) }
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
