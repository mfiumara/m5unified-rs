# Complete M5Unified Examples Rust Port Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.

**Goal:** Port every upstream example under `M5Unified/examples` into Rust samples that compile against this repository's own Rust API, expanding `m5unified-sys` and the safe `m5unified` crate only as needed.

**Architecture:** Keep the C++ boundary small and stable: `m5unified-sys` exposes a plain C ABI implemented by `native/m5u_shim.cpp`, while `m5unified` exposes the ergonomic Rust API used by all examples. Host builds use Rust stubs for CI compilation; ESP-IDF builds link the real M5Unified/M5GFX components.

**Tech Stack:** Rust 2021, Cargo workspace, ESP-IDF Rust, M5Unified/M5GFX, hand-written C++ shim, host stubs, translated examples in `examples/src/bin`, firmware smoke builds under `firmware/`.

---

## Current state snapshot

The workspace already has both layers:

- `crates/m5unified-sys`: raw unsafe shim bindings plus host stubs.
- `crates/m5unified`: a safe Rust wrapper crate with `M5Unified`, `Display`, `Buttons`, `Speaker`, `Mic`, `Imu`, `Touch`, `Rtc`, `Power`, and `Log` types.
- `examples`: Rust sample binaries already named after the upstream examples.
- `examples/src/bin/hello_display.rs`: a proven ESP-IDF smoke sample inside the examples package.

So this repo is **not only `-sys` anymore**. It has a real Rust API crate, but that API is still thin and example-driven. It is good enough for early display/button/mic/speaker/IMU/touch/RTC/power demos, not yet complete enough to faithfully express all upstream examples.

## Upstream example inventory

As of the inventory run, upstream has 18 `.ino` examples:

- `Advanced/Bluetooth_with_ESP32A2DP/Bluetooth_with_ESP32A2DP.ino`
- `Advanced/MP3_with_ESP8266Audio/MP3_with_ESP8266Audio.ino`
- `Advanced/Mic_FFT/Mic_FFT.ino`
- `Advanced/Speak_with_AquesTalk/Speak_with_AquesTalk.ino`
- `Advanced/Speaker_SD_wav_file/Speaker_SD_wav_file.ino`
- `Advanced/WebRadio_with_ESP8266Audio/WebRadio_with_ESP8266Audio.ino`
- `Basic/Axp2101/IRQ/IRQExample/IRQExample.ino`
- `Basic/Button/Button.ino`
- `Basic/Displays/Displays.ino`
- `Basic/HowToUse/HowToUse.ino`
- `Basic/Imu/Imu.ino`
- `Basic/LogOutput/LogOutput.ino`
- `Basic/Microphone/Microphone.ino`
- `Basic/Rtc/Rtc.ino`
- `Basic/Speaker/Speaker.ino`
- `Basic/Touch/DragDrop/DragDrop.ino`
- `Basic/Touch/SliderUI/SliderUI.ino`
- `Test/build_test/build_test.ino`

The Rust repo already has corresponding `examples/src/bin/*.rs` names. Completion means those files compile cleanly and use `m5unified`, not direct `m5unified-sys` calls or placeholder-only APIs.

---

## Definition of done

A feature/example is complete when:

1. The Rust example compiles on host with `cargo check --workspace`.
2. The Rust example uses the safe `m5unified` API, not `m5unified-sys` directly.
3. Any new unsafe FFI is isolated in `m5unified-sys` and has matching host stubs.
4. The safe wrapper encodes failures as `Result`, `Option`, or typed status values instead of raw sentinel values.
5. Hardware-specific examples are either:
   - implemented fully, or
   - explicitly feature-gated with a clear `Unavailable` path and documented limitation.
6. At least one ESP-IDF firmware smoke target proves the shim still links against real M5Unified.

---

## Phase 1: Make completion measurable

### Task 1.1: Add an upstream example manifest

**Objective:** Track every upstream example, its Rust target, status, and required API surface.

**Files:**
- Create: `docs/examples/upstream-examples.toml`
- Create: `docs/examples/README.md`

**Implementation:** Add entries like:

```toml
[[example]]
upstream = "Basic/Button/Button.ino"
rust_bin = "basic_button"
status = "compiles"
category = "basic"
requires = ["display", "buttons", "log"]
notes = "Needs full click/hold button state, including PWR and EXT."
```

Allowed statuses:

- `not_started`
- `skeleton`
- `host_compiles`
- `espidf_compiles`
- `hardware_verified`
- `blocked`

**Verification:**

```bash
cargo check --workspace
```

Expected: no source code behavior changes; workspace still checks.

### Task 1.2: Add a manifest linter

**Objective:** Fail CI if an upstream example is missing a Rust bin or manifest entry.

**Files:**
- Create: `tools/check_examples_manifest.py`
- Modify: `.github/workflows/ci.yml` if present, otherwise document command in `README.md`

**Implementation:** Script should:

1. Read `docs/examples/upstream-examples.toml`.
2. List `examples/src/bin/*.rs`.
3. Assert every manifest `rust_bin` exists.
4. Assert every existing example bin appears in the manifest unless listed as `extra = true`.

**Verification:**

```bash
python3 tools/check_examples_manifest.py
cargo check --workspace
```

### Task 1.3: Add a generated API gap report

**Objective:** Keep a living source-of-truth for C++ methods used by upstream examples and Rust support status.

**Files:**
- Create: `tools/inventory_upstream_examples.py`
- Create/update: `docs/examples/api-gap-report.md`

**Implementation:** Script should clone or read a local `vendor/M5Unified` checkout and extract rough occurrences of:

- `M5.<subsystem>.<method>(...)`
- `M5.<method>(...)`
- `M5.Displays(i).<method>(...)`
- `M5.Touch.getDetail(...).<method>(...)`
- direct classes like `M5Canvas`, `BluetoothA2DPSink`, `AudioGeneratorMP3`, `AquesTalk`

**Verification:**

```bash
python3 tools/inventory_upstream_examples.py --upstream /tmp/M5Unified-examples-inventory > docs/examples/api-gap-report.md
```

---

## Phase 2: Stabilize crate boundaries

### Task 2.1: Document the two-crate contract

**Objective:** Prevent accidental growth of the unsafe API into user-facing examples.

**Files:**
- Modify: `crates/m5unified-sys/src/lib.rs`
- Modify: `crates/m5unified/src/lib.rs`
- Modify: `README.md`

**Rules:**

- `m5unified-sys`:
  - raw, unsafe, C-like names, no ergonomics;
  - mirrors `native/m5u_shim.h` exactly;
  - must always include host stubs for new functions.
- `m5unified`:
  - safe, idiomatic names;
  - no public unsafe required for normal examples;
  - owns strings/buffers/types and converts at FFI boundary.
- Examples:
  - depend on `m5unified` only.

**Verification:**

```bash
cargo check --workspace
```

### Task 2.2: Add a no-direct-sys check for examples

**Objective:** Enforce that examples only use `m5unified`.

**Files:**
- Create: `tools/check_no_sys_in_examples.py`

**Implementation:** Fail if `examples/src/bin/*.rs` contains `m5unified_sys`.

**Verification:**

```bash
python3 tools/check_no_sys_in_examples.py
```

### Task 2.3: Split `m5unified/src/lib.rs` into modules

**Objective:** Keep the Rust API maintainable as the example surface grows.

**Files:**
- Modify: `crates/m5unified/src/lib.rs`
- Create: `crates/m5unified/src/display.rs`
- Create: `crates/m5unified/src/buttons.rs`
- Create: `crates/m5unified/src/audio.rs`
- Create: `crates/m5unified/src/imu.rs`
- Create: `crates/m5unified/src/touch.rs`
- Create: `crates/m5unified/src/rtc.rs`
- Create: `crates/m5unified/src/power.rs`
- Create: `crates/m5unified/src/log.rs`
- Create: `crates/m5unified/src/error.rs`

**Verification:**

```bash
cargo test --workspace
```

---

## Phase 3: Complete the Basic examples first

### Task 3.1: Complete display basics

**Objective:** Support `Basic/HowToUse`, `Basic/Displays`, `Basic/Button`, `Basic/Speaker`, touch demos, and log demos without display workarounds.

**Files:**
- Modify: `crates/m5unified-sys/native/m5u_shim.h`
- Modify: `crates/m5unified-sys/native/m5u_shim.cpp`
- Modify: `crates/m5unified-sys/src/lib.rs`
- Modify/Create: `crates/m5unified/src/display.rs`

**Add raw shim functions:**

- `m5u_display_get_rotation() -> int`
- `m5u_display_set_brightness(uint8_t)`
- `m5u_display_set_epd_fastest()`
- `m5u_display_start_write()`
- `m5u_display_end_write()`
- `m5u_display_display()`
- `m5u_display_display_busy() -> bool`
- `m5u_display_wait_display()`
- `m5u_display_get_cursor_y() -> int`
- `m5u_display_font_height() -> int`
- `m5u_display_get_base_color() -> uint16_t`
- `m5u_display_set_color(uint16_t)`
- `m5u_display_set_text_wrap(bool x, bool y)`
- `m5u_display_set_text_datum(int datum)`
- `m5u_display_draw_string(const char*, int x, int y) -> int`
- `m5u_display_printf` should **not** be exposed directly; format in Rust then call print/draw_string.
- `m5u_display_write_pixel(int x, int y, uint16_t color)`
- `m5u_display_write_fast_vline(int x, int y, int h, uint16_t color)`
- `m5u_display_set_clip_rect(int x, int y, int w, int h)`
- `m5u_display_clear_clip_rect()`
- `m5u_display_color888(uint8_t r, uint8_t g, uint8_t b) -> uint16_t`

**Add safe Rust wrappers:**

- `Display::rotation() -> i32`
- `Display::set_brightness(u8)`
- `Display::transaction(|display| ...)`
- `Display::draw_string(&str, x, y) -> Result<i32, Error>`
- `Display::write_pixel(...)`
- `Display::write_fast_vline(...)`
- `Display::set_clip_rect(...)`
- `Display::clear_clip_rect()`
- `Color::rgb888(r, g, b)` or `Display::color888(...)`

**Verification:**

```bash
cargo test -p m5unified
cargo check --workspace
```

### Task 3.2: Complete button state API

**Objective:** Support all button events used by upstream: PWR, A, B, C, EXT; pressed, released, clicked, hold, holding, decide-click-count, click count.

**Files:**
- Modify shim/header/sys bindings.
- Modify/Create: `crates/m5unified/src/buttons.rs`
- Modify examples: `basic_button.rs`, `basic_how_to_use.rs`, `basic_log_output.rs`, `basic_speaker.rs`

**Add raw shim functions:**

Use an indexed button API instead of one C function per button/state:

```c
bool m5u_button_is_pressed(int button);
bool m5u_button_was_pressed(int button);
bool m5u_button_was_released(int button);
bool m5u_button_was_clicked(int button);
bool m5u_button_was_hold(int button);
bool m5u_button_is_holding(int button);
bool m5u_button_was_decide_click_count(int button);
int  m5u_button_get_click_count(int button);
```

Button IDs:

- `0 = A`
- `1 = B`
- `2 = C`
- `3 = Pwr`
- `4 = Ext`

Keep old A/B/C raw functions temporarily for compatibility, but update safe Rust to use the indexed path.

**Verification:**

```bash
cargo test -p m5unified
cargo check --workspace
```

### Task 3.3: Complete power and AXP2101 IRQ API

**Objective:** Support `Basic/Axp2101/IRQ/IRQExample` and battery display in `HowToUse`.

**Files:**
- Modify shim/header/sys bindings.
- Create: `crates/m5unified/src/axp2101.rs` or include under `power.rs`.
- Modify: `examples/src/bin/basic_axp2101_irq.rs`

**Add safe Rust model:**

```rust
pub struct Axp2101;
pub struct IrqStatus { ... }
```

Expose only the IRQs used by the example first:

- battery charger under-temperature
- battery charger over-temperature
- VBUS insert
- VBUS remove

**Verification:**

```bash
cargo check -p m5unified-examples --bin basic_axp2101_irq
cargo check --workspace
```

### Task 3.4: Complete RTC/log wrappers

**Objective:** Support `Basic/Rtc` and `Basic/LogOutput` without placeholder logging.

**Files:**
- Modify: `crates/m5unified/src/rtc.rs`
- Modify: `crates/m5unified/src/log.rs`
- Modify examples: `basic_rtc.rs`, `basic_log_output.rs`

**API additions:**

- `Rtc::is_enabled() -> bool`
- `Log::print(&str) -> Result<(), Error>`
- `Log::println(&str) -> Result<(), Error>`
- `Log::log(level, &str) -> Result<(), Error>`
- `LogLevel` enum: Error/Warn/Info/Debug/Verbose

Skip callback and color/suffix support unless needed for semantic parity; document those as intentionally simplified.

**Verification:**

```bash
cargo check -p m5unified-examples --bin basic_rtc --bin basic_log_output
```

### Task 3.5: Complete touch detail API

**Objective:** Support DragDrop, SliderUI, and touch handling in advanced examples.

**Files:**
- Modify shim/header/sys bindings.
- Modify/Create: `crates/m5unified/src/touch.rs`
- Modify examples: `basic_touch_drag_drop.rs`, `basic_touch_slider_ui.rs`

**Add touch detail struct across C boundary:**

```c
typedef struct {
  int x;
  int y;
  int prev_x;
  int prev_y;
  bool is_pressed;
  bool was_pressed;
  bool was_released;
  bool was_clicked;
  bool was_hold;
  bool is_holding;
  int click_count;
} m5u_touch_detail_t;

bool m5u_touch_get_detail(int index, m5u_touch_detail_t* out);
```

**Safe Rust:**

```rust
pub struct TouchDetail { ... }
impl TouchDetail {
    pub fn delta(&self) -> (i32, i32) { ... }
}
```

**Verification:**

```bash
cargo check -p m5unified-examples --bin basic_touch_drag_drop --bin basic_touch_slider_ui
```

### Task 3.6: Complete IMU basic/calibration API

**Objective:** Support `Basic/Imu` including type display and offset/calibration flow.

**Files:**
- Modify shim/header/sys bindings.
- Modify/Create: `crates/m5unified/src/imu.rs`
- Modify: `examples/src/bin/basic_imu.rs`

**API additions:**

- `Imu::is_enabled() -> bool`
- `Imu::kind() -> ImuKind`
- `Imu::update() -> bool`
- `Imu::data() -> ImuData { accel, gyro, mag, temp }`
- `Imu::load_offset_from_nvs() -> bool`
- `Imu::save_offset_to_nvs() -> bool`
- `Imu::offset_data(index) -> Option<f32>`
- `Imu::set_calibration(...)`

**Verification:**

```bash
cargo check -p m5unified-examples --bin basic_imu
```

### Task 3.7: Complete microphone basic API

**Objective:** Support `Basic/Microphone` and prepare for `Advanced/Mic_FFT`.

**Files:**
- Modify shim/header/sys bindings.
- Modify/Create: `crates/m5unified/src/audio.rs`
- Modify: `examples/src/bin/basic_microphone.rs`

**API additions:**

- `Mic::is_enabled() -> bool`
- `Mic::is_recording() -> bool`
- `Mic::end()`
- `Mic::config() -> MicConfig`
- `Mic::set_config(MicConfig) -> Result<(), Error>`
- `Mic::record_i16_at(&mut [i16], sample_rate_hz: u32) -> bool`
- `MicConfig::noise_filter_level`
- optional port pin fields for advanced FFT

**Verification:**

```bash
cargo check -p m5unified-examples --bin basic_microphone
```

### Task 3.8: Complete speaker basic API

**Objective:** Support `Basic/Speaker` and audio playback controls.

**Files:**
- Modify shim/header/sys bindings.
- Modify/Create: `crates/m5unified/src/audio.rs`
- Modify: `examples/src/bin/basic_speaker.rs`

**API additions:**

- `Speaker::is_enabled() -> bool`
- `Speaker::end()`
- `Speaker::is_playing(channel: Option<u8>) -> bool`
- `Speaker::stop(channel: Option<u8>)`
- `Speaker::volume() -> u8`
- `Speaker::set_volume(u8)`
- `Speaker::channel_volume(channel: u8) -> u8`
- `Speaker::set_channel_volume(channel: u8, volume: u8)`
- `Speaker::set_all_channel_volume(u8)`
- `Speaker::tone_ex(frequency_hz: f32, duration_ms: Option<u32>, channel: Option<u8>)`
- `Speaker::play_raw_u8(...)`
- `Speaker::play_raw_i16(...)`
- `Speaker::play_wav(...)`

**Verification:**

```bash
cargo check -p m5unified-examples --bin basic_speaker
```

### Task 3.9: Complete multi-display API enough for `Basic/Displays`

**Objective:** Support multiple display enumeration and primary display selection.

**Files:**
- Modify shim/header/sys bindings.
- Modify/Create: `crates/m5unified/src/display.rs`
- Modify: `examples/src/bin/basic_displays.rs`

**API additions:**

- `M5Unified::display_count() -> usize`
- `M5Unified::display(index) -> Option<DisplayRef>`
- `M5Unified::primary_display_type(...)`
- `M5Unified::display_index(DisplayKind) -> Option<usize>`
- `DisplayKind` enum for module/atom/unit displays used by example.

**C ABI strategy:** Use indexed functions instead of exposing C++ display objects:

```c
int  m5u_display_count(void);
int  m5u_display_index_for_kind(int kind);
int  m5u_display_width_at(int index);
void m5u_display_print_at(int index, const char* text);
...
```

**Verification:**

```bash
cargo check -p m5unified-examples --bin basic_displays
```

### Task 3.10: Finish `Basic/HowToUse` as integration sample

**Objective:** Use all completed basic APIs in one larger sample.

**Files:**
- Modify: `examples/src/bin/basic_how_to_use.rs`

**Verification:**

```bash
cargo check -p m5unified-examples --bin basic_how_to_use
cargo check --workspace
```

---

## Phase 4: Advanced examples and optional integrations

### Task 4.1: Add SD and WAV file support

**Objective:** Support `Advanced/Speaker_SD_wav_file`.

**Files:**
- Modify: `crates/m5unified-sys/native/m5u_shim.cpp`
- Modify: `crates/m5unified/src/storage.rs`
- Modify: `examples/src/bin/advanced_speaker_sd_wav_file.rs`

**API additions:**

- `Sd::begin() -> Result<Sd, Error>`
- `Sd::read(path: &str, buf: &mut [u8]) -> Result<usize, Error>` or document use of Rust/ESP-IDF filesystem once mounted.
- `Speaker::play_wav_bytes(&[u8]) -> Result<(), Error>`

**Note:** Current `m5u_sd_begin()` is stubbed to `false`; this task must decide whether to wire Arduino SD component support, ESP-IDF VFS/FAT, or mark SD as feature-gated.

### Task 4.2: Add FFT-ready microphone/display support

**Objective:** Support `Advanced/Mic_FFT` at compile level with a Rust FFT crate or simple placeholder visualization.

**Files:**
- Modify: `examples/Cargo.toml`
- Modify: `examples/src/bin/advanced_mic_fft.rs`
- Potentially modify `audio.rs`, `display.rs`

**Decision:** Prefer a pure Rust FFT crate for the Rust example. Keep M5Unified shim focused on mic capture and display drawing.

### Task 4.3: Decide strategy for MP3/WebRadio examples

**Objective:** Avoid pretending Arduino-only libraries are part of the M5Unified Rust binding.

**Examples:**

- `Advanced/MP3_with_ESP8266Audio`
- `Advanced/WebRadio_with_ESP8266Audio`

**Decision options:**

1. Port behavior using Rust ESP-IDF networking + Rust MP3 decoder + `Speaker::play_raw_i16`.
2. Add optional C++ adapter layer to wrap ESP8266Audio classes.
3. Mark examples as `blocked`/`feature-gated` with clear explanation.

**Recommendation:** Option 1 for long-term Rust quality, but implement compile-level skeleton first. Do not pollute `m5unified` with ESP8266Audio bindings.

### Task 4.4: Decide strategy for Bluetooth A2DP example

**Objective:** Support `Advanced/Bluetooth_with_ESP32A2DP` semantics without making M5Unified own Bluetooth.

**Decision:** Treat Bluetooth as an optional example-level dependency, not a core `m5unified` API. The core API only needs `Speaker::play_raw_i16` with appropriate buffer behavior.

### Task 4.5: Decide strategy for AquesTalk example

**Objective:** Support `Advanced/Speak_with_AquesTalk` semantics.

**Decision:** Keep TTS generation outside `m5unified`. The M5Unified Rust API only needs `Speaker::play_raw_*` and display logging. The example may be feature-gated if AquesTalk licensing/binary support is not practical.

---

## Phase 5: ESP-IDF build validation

### Task 5.1: Promote examples into firmware smoke targets

**Objective:** Ensure host compilation is not the only signal.

**Files:**
- Create: `firmware/basic-button/`
- Create: `firmware/basic-speaker/`
- Create: `firmware/basic-microphone/`
- Create: `firmware/basic-imu/`
- Create: `firmware/basic-touch/`

Each firmware target should depend on local `m5unified` and reuse the same logic as the corresponding example where possible.

**Verification:**

```bash
cd firmware/basic-button
cargo build --target xtensa-esp32s3-espidf
```

### Task 5.2: Add an ESP-IDF build script

**Objective:** Run all firmware smoke builds consistently.

**Files:**
- Create: `tools/build_espidf_smoke.sh`

**Verification:**

```bash
bash tools/build_espidf_smoke.sh
```

### Task 5.3: Add hardware verification notes

**Objective:** Track what has actually been run on an M5StickS3/M5StickC Plus2-class board.

**Files:**
- Create: `docs/examples/hardware-verification.md`

Include:

- board model
- target triple
- firmware target
- expected visible/audio behavior
- observed result
- date
- known limitations

---

## Phase 6: Polish the Rust API

### Task 6.1: Replace bool-heavy APIs with Result/Option where useful

**Objective:** Make API feel Rusty, not like C++ through a thin skin.

**Examples:**

- `Speaker::begin() -> Result<(), Error>` instead of `bool`.
- `Mic::begin() -> Result<(), Error>`.
- `Imu::begin() -> Result<(), Error>`.
- `Rtc::get_datetime() -> Result<DateTime, Error>` if unavailable/error distinction matters.

Use compatibility methods only if examples already depend on old names.

### Task 6.2: Add typed colors and geometry

**Objective:** Avoid endless `i32, i32, i32, i32, u16` signatures.

**Types:**

```rust
pub struct Color565(pub u16);
pub struct Point { pub x: i32, pub y: i32 }
pub struct Size { pub w: i32, pub h: i32 }
pub struct Rect { pub x: i32, pub y: i32, pub w: i32, pub h: i32 }
```

Keep raw convenience methods for examples if readability benefits.

### Task 6.3: Add docs and doctests for public API families

**Objective:** Make the crate usable outside the examples.

**Files:**
- Module-level docs for display/buttons/audio/imu/touch/rtc/power/log.
- README examples using safe API only.

**Verification:**

```bash
cargo test --doc -p m5unified
```

---

## Recommended implementation order

1. Manifest/lint/report tooling.
2. Crate boundary cleanup and module split.
3. Display API expansion.
4. Indexed full button API.
5. Speaker controls.
6. Mic controls.
7. Touch detail.
8. RTC/log/power/AXP2101.
9. IMU calibration/data.
10. Multi-display.
11. Finish all Basic examples.
12. SD/WAV advanced example.
13. Mic FFT advanced example.
14. Bluetooth/MP3/WebRadio/AquesTalk strategy and feature gates.
15. ESP-IDF smoke builds.
16. Rust API polish.

## Risks and policy decisions

- **Do not bind arbitrary C++ classes directly.** If an upstream example uses `M5Canvas`, `AudioGeneratorMP3`, or `BluetoothA2DPSink`, decide whether to model the behavior in Rust or wrap it behind a tiny purpose-specific C ABI.
- **Keep host stubs honest enough for compile tests, not fake hardware simulators.** They should return deterministic defaults and fill buffers safely.
- **Board support starts with ESP32-S3/M5StickS3-style hardware.** Other boards can compile where possible, but runtime availability should be explicit.
- **Advanced examples may be feature-gated.** Completion means the repo has a compiling Rust sample and documented path, not necessarily that third-party Arduino libraries are fully re-bound.

## Final acceptance command set

```bash
python3 tools/check_examples_manifest.py
python3 tools/check_no_sys_in_examples.py
cargo fmt --all -- --check
cargo test --workspace
cargo check --workspace
bash tools/build_espidf_smoke.sh
```

Expected: all pass, except hardware/ESP-IDF smoke builds may be documented as requiring the ESP-IDF environment and supported board connected for flash/run validation.
