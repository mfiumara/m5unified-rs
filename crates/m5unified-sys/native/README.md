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

Cardputer firmware should also make the M5Cardputer Arduino library available to ESP-IDF/Arduino and define `M5UNIFIED_RS_USE_REAL_M5CARDPUTER=1`. Define `M5UNIFIED_RS_USE_ARDUINO_GPIO=1` when Arduino GPIO helpers are available and the firmware wants Grove GPIO helpers. Define `M5UNIFIED_RS_USE_ARDUINO_SD=1` when the Arduino `SPI`/`SD` libraries are available and the firmware wants the Cardputer microSD mount, status, and file helpers. Define `M5UNIFIED_RS_USE_ARDUINO_SERIAL=1` when Arduino `Serial1` is available and the firmware wants Grove UART helpers. Define `M5UNIFIED_RS_USE_ARDUINO_WIRE=1` when Arduino `Wire` is available and the firmware wants Grove I2C helpers. Define `M5UNIFIED_RS_USE_ARDUINO_IRREMOTE=1` when Arduino-IRremote is available and the firmware wants NEC IR transmit helpers. The repository's `firmware/cardputer-keyboard` package shows the intended CMake wiring for that path.

## Current limitation

This is a component scaffold, not a fully automated Cargo-to-ESP-IDF linkage path yet. The firmware packages under `firmware/` consume the shim by relative path and are the hardware validation targets.
