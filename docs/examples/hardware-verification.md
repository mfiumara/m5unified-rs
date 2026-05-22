# Hardware Verification

This file records real ESP-IDF firmware and on-device checks. Host stubs prove
API shape only; they do not prove hardware behavior.

## Build Smoke

Command:

```bash
bash tools/build_espidf_smoke.sh
```

Defaults:

- target: `xtensa-esp32s3-espidf`
- binaries: `hello_display`
- override target with `ESP_IDF_TARGET_TRIPLE`
- override binaries with `ESPIDF_SMOKE_BINS`, for example
  `ESPIDF_SMOKE_BINS="hello_display basic_button"`

The command requires the esp-rs ESP-IDF environment, `~/export-esp.sh`, and the
component manager dependencies declared by `examples/components/m5unified-rs`.

## On-Device Matrix

| Date | Board | Target | Binary | Build | Flash/run | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| 2026-05-18 | M5StickS3-class ESP32-S3 | `xtensa-esp32s3-espidf` | `hello_display` | pending in current environment | pending hardware | Expected display text: `hello from rust`; Button A/B redraw status text. |

## Recording A Run

For each hardware run, record:

- board model and revision if known
- Rust target triple
- command used to build
- `espflash` command used to flash or monitor
- visible display, touch, button, audio, SD, IMU, RTC, and power behavior
- failures, boot logs, or limitations
