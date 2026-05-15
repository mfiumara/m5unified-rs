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
examples/          on-device examples, added as support lands
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

The C++ shim declares the matching C ABI. Host builds use no-op stubs so examples compile without hardware; ESP-IDF builds still need the real M5Unified/M5GFX component integration enabled in `m5unified-sys/build.rs`.

## Examples

Rust translations/smoke ports of every upstream M5Unified example directory live in the `examples` workspace package. See [`examples/README.md`](examples/README.md) for the upstream-to-Rust mapping.

```bash
bash scripts/check-host.sh
cargo run -p m5unified-examples --bin basic_displays
```

## Plan

See [`docs/plans/2026-05-15-m5unified-rs-roadmap.md`](docs/plans/2026-05-15-m5unified-rs-roadmap.md) for the implementation roadmap and Codex handoff plan.
