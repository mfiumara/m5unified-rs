# M5Unified Cardputer keyboard firmware spike

This firmware package is the first Cardputer-first hardware validation target for the Rust bindings. It is separate from the host-checkable workspace because it targets `xtensa-esp32s3-espidf` and requires the esp-rs toolchain plus M5Stack Arduino components.

Behavior:

- initializes `M5Cardputer` through the Rust `m5unified::Cardputer` wrapper;
- draws status text through the shared C ABI shim;
- polls `M5Cardputer.update()`;
- reads `M5Cardputer.Keyboard.keysState()` through Rust;
- initializes the built-in microSD slot and shows mount status;
- initializes the built-in IR transmitter and sends a sample NEC frame on Button A;
- initializes the HY2.0-4P Grove I2C bus and shows detected device count;
- appends printable keyboard input to the display;
- clears the input line on Enter;
- deletes one character on Backspace;
- toggles caps lock with Button A.

The `components/m5unified-rs` component includes the repository's native shim and defines `M5UNIFIED_RS_USE_REAL_M5CARDPUTER=1`, which enables the optional `M5Cardputer` C++ path. It also defines `M5UNIFIED_RS_USE_ARDUINO_GPIO=1` for Grove GPIO, `M5UNIFIED_RS_USE_ARDUINO_SD=1` for the built-in microSD slot, `M5UNIFIED_RS_USE_ARDUINO_SERIAL=1` for Grove UART, `M5UNIFIED_RS_USE_ARDUINO_WIRE=1` for the Grove I2C boundary, and `M5UNIFIED_RS_USE_ARDUINO_IRREMOTE=1` for the built-in IR transmitter.

## Build

Install the esp-rs toolchain first:

```bash
cargo +stable install espup
espup install
. ~/export-esp.sh
cargo +stable install espflash
```

Then build from this directory:

```bash
cargo build --target xtensa-esp32s3-espidf
```

This target depends on the M5Cardputer and Arduino-IRremote libraries being available to the ESP-IDF/Arduino build. M5Stack's current Cardputer documentation shows the PlatformIO dependency as `M5Cardputer=https://github.com/m5stack/M5Cardputer`; the IR example also uses Arduino-IRremote's `IrSender`.

## Flash

```bash
espflash flash --monitor target/xtensa-esp32s3-espidf/debug/m5unified-cardputer-keyboard
```

Expected hardware result on M5Cardputer or Cardputer-Adv: the screen shows the Rust keyboard prompt plus SD, IR, and Grove I2C status, typed printable keys appear on the display, Backspace deletes, Enter clears, and Button A toggles caps lock while sending a sample NEC IR frame.
