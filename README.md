# m5unified-rs

Rust bindings for [M5Unified](https://github.com/m5stack/M5Unified), the
board support library used across M5Stack ESP32 devices.

The project publishes two crates:

- [`m5unified`](https://docs.rs/m5unified) - safe Rust API for common
  M5Unified display, button, microphone, speaker, IMU, touch, RTC, power, log,
  and SD-card operations.
- [`m5unified-sys`](https://docs.rs/m5unified-sys) - raw FFI declarations plus
  the C/C++ ESP-IDF component shim used by the safe crate.

The binding strategy is intentionally narrow: Rust calls a plain `extern "C"`
shim instead of trying to bind the full M5Unified C++ class surface directly.

## Install

For applications, depend on the safe wrapper:

```toml
[dependencies]
m5unified = "0.3"
```

Use `m5unified-sys` directly only when writing lower-level bindings or firmware
integration code that needs the raw C ABI.

## Quick Start

```rust
use m5unified::{colors, M5Unified};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut m5 = M5Unified::begin()?;

    m5.display.fill_screen(colors::BLACK);
    m5.display.set_text_size(2);
    m5.display.println("hello from Rust")?;

    loop {
        m5.update();
        if m5.buttons.a().was_pressed() {
            m5.display.println("Button A")?;
        }
        m5.delay_ms(16);
    }
}
```

## Target Support

On ESP-IDF targets, `m5unified-sys` declares the C ABI that is implemented by
the native shim in this repository. Firmware projects should include that shim
as an ESP-IDF component so the Rust crate links against the real M5Unified and
M5GFX libraries.

On non-ESP-IDF host targets, `m5unified-sys` provides no-op Rust stubs. This
keeps the safe wrapper and translated examples buildable in CI and on developer
machines without M5Stack hardware. Host stubs are for compile-time checking, not
hardware simulation.

## API Surface

The current wrapper covers the API used by the translated upstream examples:

- startup configuration through `M5UnifiedConfig` and `M5Unified::begin_with_config`
- display drawing, text, `core::fmt::Write` formatting, color, fonts, EPD modes, scrolling, transactions, and indexed multi-display drawing
- button press, release, hold, click, click-count, timing, threshold, and state helpers
- microphone recording, recording state, sample-rate, configuration helpers, and simple RMS calculation
- speaker configuration, running/playing state, tone, PCM, WAV, channel, repeat, and volume controls
- IMU acceleration, gyro, temperature, sensor masks, axis order, calibration, raw data, and NVS offsets
- touch points, touch detail state, flick/drag state helpers, and touch thresholds
- RTC date/time, date-only/time-only, low-voltage and IRQ helpers, system-time sync, power PMIC type, battery/VBUS/external-port readings, sleep/power-off, output/charge controls, AXP2101 IRQ mask/status helpers, LED control/color batches/type queries, logging dump/path/configuration/callbacks, `core::fmt::Write` log formatting, and SD SPI mount helpers
- internal/external I2C bus setup, raw transfers, register helpers, device helpers, bit helpers, and address scanning
- named board identity, timing helpers, pin lookup, primary/log display selection, and touch-button sizing

This is not a complete M5Unified port yet. Missing APIs should be added through
the C ABI shim first, then wrapped by `m5unified`.

## Repository Layout

```text
crates/
  m5unified-sys/   raw bindings and native C/C++ shim
  m5unified/       safe Rust wrapper
examples/          host-checkable Rust ports of upstream M5Unified examples
docs/              example mapping, project plans, and release notes
```

## Examples

Rust translations and smoke ports of upstream M5Unified examples live in the
repository's `examples` workspace package. They are intentionally not published
as a crate.

```bash
bash scripts/check-host.sh
cargo run -p m5unified-examples --bin basic_displays
```

See the repository docs for the upstream-to-Rust example mapping and firmware
bring-up instructions.

## On-Device Hello Display

The `examples` package includes `hello_display`, a M5StickS3 smoke sample that
can be built for `xtensa-esp32s3-espidf` while the rest of the workspace remains
host-checkable through stubs.

Install the esp-rs tools before building for hardware:

```bash
cargo +stable install espup
espup install
. ~/export-esp.sh
cargo +stable install espflash
```

Then build and flash the sample:

```bash
bash tools/build_espidf_smoke.sh
espflash flash --monitor target/xtensa-esp32s3-espidf/debug/hello_display
```

Expected hardware behavior: the display shows `hello from rust`; Button A/B
presses change the screen.

Record hardware runs in `docs/examples/hardware-verification.md`.

## Release Checks

```bash
python3 tools/check_examples_manifest.py
python3 tools/check_no_sys_in_examples.py
bash scripts/check-host.sh
bash tools/build_espidf_smoke.sh
cargo package -p m5unified-sys
cargo publish -p m5unified-sys --dry-run
```

Publish `m5unified-sys` before `m5unified`, because the safe crate depends on
the exact sys crate version through crates.io. Package and dry-run `m5unified`
after the sys crate upload has propagated. See `docs/publishing.md` in the
repository for the full release checklist.

## License

Licensed under either MIT or Apache-2.0, at your option.
