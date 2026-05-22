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
- display enumeration and primary-display selection
- Cardputer keyboard input
- Cardputer microSD mount/status and file helpers
- Cardputer IR NEC transmit boundary
- Cardputer Grove I2C boundary
- Cardputer Grove GPIO boundary
- Cardputer Grove analog/PWM boundary
- Cardputer raw SPI boundary
- Cardputer Grove UART boundary
- runtime board detection
- timing helpers
- buttons
- board-aware pin lookup
- microphone/speaker
- IMU
- touch
- touch button geometry
- RTC
- power/battery
- RGB LED
- logging
- log target configuration
- SD-card boundary

The C++ shim declares the matching C ABI. Host builds use no-op stubs so examples compile without hardware. ESP-IDF builds now have a native component scaffold in [`crates/m5unified-sys/native`](crates/m5unified-sys/native), plus a first firmware package in [`firmware/hello-display`](firmware/hello-display) that consumes that shim as an ESP-IDF component for M5StickS3-class hardware validation.

## Examples

Rust translations/smoke ports of every upstream M5Unified example directory live in the `examples` workspace package. See [`examples/README.md`](examples/README.md) for the upstream-to-Rust mapping.

```bash
bash scripts/check-host.sh
cargo run -p m5unified-examples --bin basic_displays
cargo run -p m5unified-examples --bin cardputer_keyboard
cargo run -p m5unified-examples --bin cardputer_sd
cargo run -p m5unified-examples --bin cardputer_sd_file
cargo run -p m5unified-examples --bin cardputer_ir_nec
cargo run -p m5unified-examples --bin cardputer_grove_i2c
cargo run -p m5unified-examples --bin cardputer_grove_gpio
cargo run -p m5unified-examples --bin cardputer_grove_analog
cargo run -p m5unified-examples --bin cardputer_spi
cargo run -p m5unified-examples --bin cardputer_grove_uart
```

## Firmware spike

The first ESP-IDF Rust firmware package lives in [`firmware/hello-display`](firmware/hello-display). It is excluded from the host workspace because it requires the esp-rs `xtensa-esp32s3-espidf` toolchain.

The first Cardputer-specific firmware package lives in [`firmware/cardputer-keyboard`](firmware/cardputer-keyboard). It enables the optional `M5Cardputer` shim path and validates display output plus keyboard input from Rust.

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

If the ESP-IDF install fails while creating an environment named like `idf5.3_py3.14_env`, Homebrew's `python3` is too new for this ESP-IDF version. Use Python 3.12 for the build:

```bash
brew install python@3.12
export PATH="$(brew --prefix python@3.12)/libexec/bin:$PATH"
python3 --version  # should print Python 3.12.x
rm -rf firmware/hello-display/.embuild/espressif/python_env
```

If Python 3.12 then fails in `ensurepip` with a `pyexpat`/`libexpat` symbol error, repair Homebrew's expat linkage or use pyenv:

```bash
brew reinstall expat python@3.12
export PATH="$(brew --prefix python@3.12)/libexec/bin:$PATH"
export DYLD_LIBRARY_PATH="$(brew --prefix expat)/lib:${DYLD_LIBRARY_PATH:-}"
python3 -c 'import pyexpat; print(pyexpat.EXPAT_VERSION)'
rm -rf firmware/hello-display/.embuild/espressif/python_env
```

Fallback:

```bash
brew install pyenv
pyenv install 3.11.9
export PATH="$(pyenv root)/versions/3.11.9/bin:$PATH"
python3 -c 'import pyexpat; print(pyexpat.EXPAT_VERSION)'
rm -rf firmware/hello-display/.embuild/espressif/python_env
```

```bash
cd firmware/hello-display
cargo build --target xtensa-esp32s3-espidf
espflash flash --monitor target/xtensa-esp32s3-espidf/debug/m5unified-hello-display
```

Expected hardware behavior: the display shows `hello from rust`; Button A/B presses change the screen.

For the Cardputer keyboard firmware:

```bash
cd firmware/cardputer-keyboard
cargo build --target xtensa-esp32s3-espidf
espflash flash --monitor target/xtensa-esp32s3-espidf/debug/m5unified-cardputer-keyboard
```

Expected hardware behavior: typed keys appear on the display, Backspace deletes one character, Enter clears the input line, Button A toggles caps lock and sends a sample NEC IR frame, and the screen reports whether the built-in microSD slot, IR transmitter, and Grove I2C bus initialized.

## Cardputer support

Cardputer-specific Rust APIs start at `m5unified::Cardputer`. The first supported board-specific surfaces are the keyboard, built-in microSD slot, IR NEC transmit boundary, and Grove I2C/GPIO/UART boundaries:

```rust
let mut cardputer = m5unified::Cardputer::begin()?;
cardputer.update();
if let Some(state) = cardputer.keyboard.state() {
    let typed = state.word_lossy();
}
let sd_mounted = cardputer.sd.begin();
let _written = cardputer.sd.write_file("/m5rs.txt", b"hello")?;
let ir_ready = cardputer.ir.begin();
let grove_ready = cardputer.grove.i2c_try_begin().is_ok();
let uart_ready = cardputer.grove.uart_try_begin(115_200).is_ok();
let grove_adc = cardputer.grove.analog_read(m5unified::GrovePin::G1);
let spi_ready = cardputer.spi.begin_with(m5unified::SpiPins::CARDPUTER_SD);
cardputer.led.set_all_color(m5unified::rgb::GREEN);
let port_a_sda = cardputer.pin(m5unified::PinName::PORT_A_SDA);
let board = cardputer.board();
```

The generic shim still builds against plain M5Unified by default. To compile the optional C++ Cardputer path for firmware, make the `M5Cardputer` library available to ESP-IDF/Arduino and set `M5UNIFIED_RS_ENABLE_CARDPUTER=ON` for the native component so it defines `M5UNIFIED_RS_USE_REAL_M5CARDPUTER`.

## Stack-chan motion contract

Stack-chan motion helpers mirror the MCP/firmware contract used by `stackchan-mcp`: `x` yaw is -128..128 degrees, `y` pitch is 0..90 degrees, and `speed` is 0..100 percent. `StackChanMove` clamps that public contract and converts the official StackChan-BSP Motion boundary to 0.1 degree units, hardware pitch clamp 5..85 degrees, and BSP speed units of `speed * 10`.

Official Stack-chan CoreS3 hardware is not generic PWM on Port A. It requires StackChan-BSP Motion initialization (`M5StackChan.begin()`/`update()`), board power setup including VM_EN/IO expander handling, and Motion calls for move/home/nod/shake/status. The Rust C ABI exposes `StackChanBspMotion` hooks for firmware that enables `M5UNIFIED_RS_ENABLE_STACKCHAN_BSP=ON` and links StackChan-BSP; host builds and firmware without that option return `Error::Unavailable("stackchan bsp motion")`.

Generic two-axis PWM fallback control is available through `m5unified::StackChanPwmServos` for custom non-official builds. It attaches two LEDC PWM channels, writes pan/tilt angles in tenths of a degree, returns to neutral, and can step smoothly between poses.

```rust
let mut m5 = m5unified::M5Unified::begin()?;
let command = m5unified::StackChanMove::from_mcp(0.0, 45.0, 50);
assert_eq!(command.bsp_speed(), 500);

let mut servos = m5unified::StackChanPwmServos::attach_pwm_pins(
    m5unified::PwmServoPins::CORES3_PORT_A,
)?;
servos.move_to(command)?;
servos.write_pose(m5unified::StackChanPose::LEFT)?;
servos.smooth_move_to(m5unified::StackChanPose::NEUTRAL, 50, 20, |ms| m5.delay_ms(ms))?;
```

`examples/src/bin/stackchan_servo.rs` is the generic PWM fallback example only. Official Stack-chan MCP firmware should keep the BSP Motion path and expose `/move`, `/home`, `/nod`, `/shake`, and `/servo/status` at the firmware/MCP layer.

## Plans and publishing

- [`docs/plans/2026-05-15-m5unified-rs-roadmap.md`](docs/plans/2026-05-15-m5unified-rs-roadmap.md) contains the original implementation roadmap.
- [`docs/plans/2026-05-16-complete-m5unified-examples.md`](docs/plans/2026-05-16-complete-m5unified-examples.md) tracks the full upstream-example parity plan.
- [`docs/examples/upstream-examples.toml`](docs/examples/upstream-examples.toml) maps upstream examples to Rust bins.
- [`docs/publishing.md`](docs/publishing.md) documents the crates.io release order for `m5unified-sys` and `m5unified`.
