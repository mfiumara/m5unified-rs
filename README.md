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

Early skeleton. APIs are placeholders and will change.
