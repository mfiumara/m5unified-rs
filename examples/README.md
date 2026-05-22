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
- `M5Cardputer/examples/Basic/Keyboard` → `cardputer_keyboard`
- `M5Cardputer/examples/Basic/SDCard` → `cardputer_sd`
- `M5Cardputer SDCard file read/write boundary` → `cardputer_sd_file`
- `M5Cardputer/examples/Basic/IR_NEC` → `cardputer_ir_nec`
- `M5Cardputer HY2.0-4P Grove I2C boundary` → `cardputer_grove_i2c`
- `M5Cardputer HY2.0-4P Grove GPIO boundary` → `cardputer_grove_gpio`
- `M5Cardputer HY2.0-4P Grove analog/PWM boundary` → `cardputer_grove_analog`
- `M5Cardputer raw SPI boundary` → `cardputer_spi`
- `M5Cardputer HY2.0-4P Grove UART boundary` → `cardputer_grove_uart`
- `M5Stack StackChan generic PWM fallback` → `stackchan_servo`

## Run/check

```bash
bash scripts/check-host.sh
cargo run
cargo run -p m5unified-examples --bin basic_displays
cargo run --bin basic_button
cargo run -p m5unified-examples --bin stackchan_servo
```

From the workspace root, plain `cargo run` launches `basic_how_to_use` as the default host-checkable smoke example. Use `cargo run --bin <name>` for any specific translated upstream example listed above.

The advanced network/Bluetooth/codec examples currently define the Rust API boundary and compile-time sample shape. They intentionally leave codec/network stack selection to the application crate while routing display/speaker/control operations through `m5unified`.

`stackchan_servo` is scoped to the generic PWM pan/tilt fallback useful for custom Stack-chan style builds: attach two servos, move to neutral/look poses, and run a small smooth sweep. Official Stack-chan CoreS3 bodies are not Port A PWM devices; MCP-compatible firmware should use StackChan-BSP Motion with `M5StackChan.begin()`/`update()`, VM_EN/IO-expander power setup, yaw `x` -128..128, pitch `y` 0..90 with hardware clamp 5..85, speed percent mapped to BSP speed * 10, and home/nod/shake/status endpoints.
