# Native ESP-IDF component scaffold

This directory contains the C ABI shim that lets Rust call the real C++ M5Unified API on ESP-IDF targets.

## Files

- `m5u_shim.h` — flat C ABI declarations consumed by `m5unified-sys`.
- `m5u_shim.cpp` — C++ implementation that forwards to `M5Unified` when compiled with `M5UNIFIED_RS_USE_REAL_M5UNIFIED`.
- `CMakeLists.txt` — ESP-IDF component wrapper for the shim.
- `idf_component.yml` — ESP-IDF component-manager dependencies for M5Unified/M5GFX.

## Intended integration

For the first on-device spike, copy or symlink this directory into an `esp-idf-sys` firmware project as a component, for example:

```text
firmware/
  Cargo.toml
  build.rs
  sdkconfig.defaults
  components/
    m5unified-rs-shim/
      CMakeLists.txt
      idf_component.yml
      m5u_shim.cpp
      m5u_shim.h
```

Then build the firmware for ESP32-S3 with the Rust ESP-IDF toolchain and set:

```bash
export M5UNIFIED_RS_USE_REAL_M5UNIFIED=1
```

The `m5unified-sys` build script emits a `m5unified_rs_real_m5unified` cfg when that environment variable is present on an ESP-IDF target. The C++ component itself defines `M5UNIFIED_RS_USE_REAL_M5UNIFIED=1` for `m5u_shim.cpp`, causing the shim to include `<M5Unified.h>` and call real hardware APIs.

## Current limitation

This is a component scaffold, not a fully automated Cargo-to-ESP-IDF linkage path yet. The next step is an actual firmware package that vendors/symlinks this component and proves `basic_displays` or a smaller display/button sample on M5StickS3 hardware.
