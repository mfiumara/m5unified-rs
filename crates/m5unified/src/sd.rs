//! SD-card mounting and file helpers.
//!
//! The native shim mounts an SD card into ESP-IDF's FAT VFS. [`SdCard`] offers
//! a small RAII wrapper for mount/unmount and file reads through standard Rust
//! filesystem APIs.

use core::ffi::c_int;
use std::path::{Path, PathBuf};

use crate::Error;

/// FAT VFS mount path used by the native SD SPI helper.
pub const SD_MOUNT_PATH: &str = "/sdcard";

/// Mount an SD card using the board SD SPI pins reported by M5Unified.
pub fn sd_begin() -> bool {
    unsafe { m5unified_sys::m5u_sd_begin() }
}

/// Mount an SD card using explicit SPI pins and mount options.
pub fn sd_begin_with_config(config: &SdSpiConfig) -> bool {
    let raw = config.to_raw();
    unsafe { m5unified_sys::m5u_sd_begin_spi(&raw) }
}

/// Return whether this shim has an SD card mounted at [`SD_MOUNT_PATH`].
pub fn sd_is_mounted() -> bool {
    unsafe { m5unified_sys::m5u_sd_is_mounted() }
}

/// Unmount the SD card mounted by [`sd_begin`] or [`sd_begin_with_config`].
pub fn sd_end() {
    unsafe { m5unified_sys::m5u_sd_end() }
}

/// Mounted SD card handle.
///
/// Dropping this value unmounts the card through the native shim.
#[derive(Debug)]
pub struct SdCard;

impl SdCard {
    /// Mount an SD card using the board SD SPI pins reported by M5Unified.
    pub fn mount() -> Result<Self, Error> {
        sd_begin()
            .then_some(Self)
            .ok_or(Error::Unavailable("sd card"))
    }

    /// Mount an SD card using explicit SPI pins and mount options.
    pub fn mount_with_config(config: &SdSpiConfig) -> Result<Self, Error> {
        sd_begin_with_config(config)
            .then_some(Self)
            .ok_or(Error::Unavailable("sd card"))
    }

    /// Return whether this shim has an SD card mounted at [`SD_MOUNT_PATH`].
    pub fn is_mounted(&self) -> bool {
        sd_is_mounted()
    }

    /// Return the FAT VFS mount path used by the native SD SPI helper.
    pub const fn mount_path(&self) -> &'static str {
        SD_MOUNT_PATH
    }

    /// Convert a card-relative path into an absolute path under [`SD_MOUNT_PATH`].
    pub fn path_for(path: impl AsRef<Path>) -> PathBuf {
        let path = path.as_ref();
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            Path::new(SD_MOUNT_PATH).join(path)
        }
    }

    /// Read a file from the mounted SD card.
    ///
    /// Absolute paths are used unchanged. Relative paths are resolved under
    /// [`SD_MOUNT_PATH`].
    pub fn read(&self, path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
        std::fs::read(Self::path_for(path))
    }
}

impl Drop for SdCard {
    fn drop(&mut self) {
        sd_end();
    }
}

/// SPI configuration for mounting an SD card through ESP-IDF's SDSPI driver.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SdSpiConfig {
    /// SPI clock pin.
    pub pin_sclk: i32,
    /// SPI MOSI/COPI pin.
    pub pin_mosi: i32,
    /// SPI MISO/CIPO pin.
    pub pin_miso: i32,
    /// SD card chip-select pin.
    pub pin_cs: i32,
    /// ESP-IDF SPI host id. Use `-1` for `SDSPI_HOST_DEFAULT`.
    pub host_id: i32,
    /// SD SPI frequency in kHz. Use `0` for ESP-IDF's default.
    pub frequency_khz: u32,
    /// Maximum concurrently open files.
    pub max_files: i32,
    /// Format the FAT filesystem if mounting fails.
    pub format_if_mount_failed: bool,
}

impl SdSpiConfig {
    fn to_raw(self) -> m5unified_sys::m5u_sd_spi_config_t {
        m5unified_sys::m5u_sd_spi_config_t {
            pin_sclk: self.pin_sclk as c_int,
            pin_mosi: self.pin_mosi as c_int,
            pin_miso: self.pin_miso as c_int,
            pin_cs: self.pin_cs as c_int,
            host_id: self.host_id as c_int,
            frequency_khz: self.frequency_khz,
            max_files: self.max_files as c_int,
            format_if_mount_failed: u8::from(self.format_if_mount_failed),
        }
    }
}

impl Default for SdSpiConfig {
    fn default() -> Self {
        let raw = m5unified_sys::m5u_sd_spi_config_t::default();
        Self {
            pin_sclk: raw.pin_sclk,
            pin_mosi: raw.pin_mosi,
            pin_miso: raw.pin_miso,
            pin_cs: raw.pin_cs,
            host_id: raw.host_id,
            frequency_khz: raw.frequency_khz,
            max_files: raw.max_files,
            format_if_mount_failed: raw.format_if_mount_failed != 0,
        }
    }
}
