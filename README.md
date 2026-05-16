# m5unified-rs

Rust bindings for [M5Unified](https://github.com/m5stack/M5Unified).

This repository is planned as a Cargo workspace containing two crates:

- `m5unified-sys` — raw/unsafe Rust bindings to a tiny C ABI shim over the C++ M5Unified API.
- `m5unified` — safe ergonomic Rust wrapper around `m5unified-sys`.

## Initial goal

The first milestone is intentionally small:

1. Build M5Unified/M5GFX as ESP-IDF components inside a Rust ESP-IDF project.
2. Call `M5.begin()` from Rust through a C shim.
3. Draw basic text or fill the screen from Rust.
4. Read M5StickS3 buttons from Rust.
5. Add microphone recording after display/button are proven.

This is not intended to bind the full C++ API directly. The Rust side should bind plain `extern "C"` functions exposed by a small C++ shim.

## Repository layout

```text
crates/
  m5unified-sys/   raw bindings + native C/C++ shim
  m5unified/       safe Rust wrapper
examples/          host-checkable Rust ports of upstream M5Unified examples
firmware/          ESP-IDF Rust firmware spikes for real hardware validation
```

## Status

The workspace now has a host-checkable Rust API surface for the upstream example categories:

- display drawing/text
- buttons
- microphone/speaker
- IMU
- touch
- RTC
- power/battery
- logging
- SD-card boundary

The C++ shim declares the matching C ABI. Host builds use no-op stubs so examples compile without hardware. ESP-IDF builds now have a native component scaffold in [`crates/m5unified-sys/native`](crates/m5unified-sys/native), plus a first firmware package in [`firmware/hello-display`](firmware/hello-display) that consumes that shim as an ESP-IDF component for M5StickS3-class hardware validation.

## Examples

Rust translations/smoke ports of every upstream M5Unified example directory live in the `examples` workspace package. See [`examples/README.md`](examples/README.md) for the upstream-to-Rust mapping.

```bash
bash scripts/check-host.sh
cargo run -p m5unified-examples --bin basic_displays
```

## Firmware spike

The first ESP-IDF Rust firmware package lives in [`firmware/hello-display`](firmware/hello-display). It is excluded from the host workspace because it requires the esp-rs `xtensa-esp32s3-espidf` toolchain.

If Cargo reports `custom toolchain 'esp' ... is not installed`, install the esp-rs toolchain first:

```bash
cargo +stable install espup
espup install
. ~/export-esp.sh
cargo +stable install espflash
```

Use `+stable` for the install commands, or run them outside `firmware/hello-display`, because that directory's `rust-toolchain.toml` selects the not-yet-installed `esp` toolchain.

On macOS, if the ESP-IDF build later reports missing native build tools, install:

```bash
brew install cmake ninja dfu-util ccache
```

```bash
cd firmware/hello-display
cargo build --target xtensa-esp32s3-espidf
espflash flash --monitor target/xtensa-esp32s3-espidf/debug/m5unified-hello-display
```

Expected hardware behavior: the display shows `hello from rust`; Button A/B presses change the screen.

## Plan

See [`docs/plans/2026-05-15-m5unified-rs-roadmap.md`](docs/plans/2026-05-15-m5unified-rs-roadmap.md) for the implementation roadmap and Codex handoff plan.
