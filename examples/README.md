# Examples

This package contains Rust translations/smoke ports of the upstream M5Unified examples at:

<https://github.com/m5stack/M5Unified/tree/master/examples>

They compile against the local `m5unified` Rust API. On non-ESP-IDF host targets, `m5unified-sys` provides no-op host stubs so `cargo check --workspace --examples` can validate the Rust API shape without hardware. On ESP-IDF targets, the same samples are intended to call through the C ABI shim into M5Unified.

## Upstream mapping

- `examples/Basic/HowToUse` → `basic_how_to_use`
- `examples/Basic/Button` → `basic_button`
- `examples/Basic/Displays` → `basic_displays`
- `examples/Basic/Microphone` → `basic_microphone`
- `examples/Basic/Speaker` → `basic_speaker`
- `examples/Basic/Imu` → `basic_imu`
- `examples/Basic/Touch/DragDrop` → `basic_touch_drag_drop`
- `examples/Basic/Touch/SliderUI` → `basic_touch_slider_ui`
- `examples/Basic/Rtc` → `basic_rtc`
- `examples/Basic/LogOutput` → `basic_log_output`
- `examples/Basic/Axp2101/IRQ/IRQExample` → `basic_axp2101_irq`
- `examples/Advanced/Mic_FFT` → `advanced_mic_fft`
- `examples/Advanced/Speaker_SD_wav_file` → `advanced_speaker_sd_wav_file`
- `examples/Advanced/MP3_with_ESP8266Audio` → `advanced_mp3_with_esp8266audio`
- `examples/Advanced/WebRadio_with_ESP8266Audio` → `advanced_webradio_with_esp8266audio`
- `examples/Advanced/Bluetooth_with_ESP32A2DP` → `advanced_bluetooth_with_esp32a2dp`
- `examples/Advanced/Speak_with_AquesTalk` → `advanced_speak_with_aquestalk`
- `examples/PlatformIO_SDL/src` → `platformio_sdl`
- `examples/Test/build_test` → `test_build_test`

## Run/check

```bash
cargo check --workspace --examples --bins --tests
cargo run -p m5unified-examples --bin basic_displays
```

The advanced network/Bluetooth/codec examples currently define the Rust API boundary and compile-time sample shape. They intentionally leave codec/network stack selection to the application crate while routing display/speaker/control operations through `m5unified`.
