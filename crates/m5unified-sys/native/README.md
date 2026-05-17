# Native ESP-IDF component scaffold

This directory contains the C ABI shim that lets Rust call the real C++ M5Unified API on ESP-IDF targets.

## Files

- `m5u_shim.h` — flat C ABI declarations consumed by `m5unified-sys`.
- `m5u_shim.cpp` — firmware C++ implementation that forwards to the real `M5Unified` API.
- `m5u_shim_stub.cpp` — optional no-op implementation for host-side C ABI checks that intentionally do not link `M5Unified`.
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
      m5u_shim_stub.cpp
      m5u_shim.h
```

Then build the firmware for ESP32-S3 with the Rust ESP-IDF toolchain. Firmware builds use `m5u_shim.cpp` by default and fail loudly if `M5Unified` or `M5GFX` are not wired into the ESP-IDF component graph.

For host-side C ABI checks that need a C++ object without M5Unified, configure the component with `M5UNIFIED_RS_USE_HOST_STUB=ON` so CMake selects `m5u_shim_stub.cpp` instead. The normal Rust host tests use Rust-side stubs and do not compile either C++ file.

## Current limitation

This is a component scaffold, not a fully automated Cargo-to-ESP-IDF linkage path yet. The next step is an actual firmware package that vendors/symlinks this component and proves `basic_displays` or a smaller display/button sample on M5StickS3 hardware.
