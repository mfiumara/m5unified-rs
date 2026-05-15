# m5unified-rs Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.
> **For Codex:** Work task-by-task, commit after each completed task, and keep the scope intentionally small until real M5StickS3 hardware validates the path.

**Goal:** Build a Rust workspace containing `m5unified-sys` raw bindings and `m5unified` safe wrappers so Rust/Cargo projects can use selected M5Unified features on M5StickS3.

**Architecture:** Do not bind M5Unified's C++ classes directly. Compile a tiny C++ shim against M5Unified/M5GFX as ESP-IDF components, expose plain `extern "C"` functions, bind those in `m5unified-sys`, then wrap them safely in `m5unified`.

**Tech Stack:** Rust 2021, Cargo workspace, ESP-IDF Rust ecosystem, M5Unified, M5GFX, C++ shim, M5StickS3 / ESP32-S3, `espflash` for on-device validation.

---

## Current repo state

The repo currently contains an initial skeleton:

```text
Cargo.toml
README.md
LICENSE
.gitignore
crates/
  m5unified-sys/
    Cargo.toml
    build.rs
    src/lib.rs
    native/m5u_shim.h
    native/m5u_shim.cpp
  m5unified/
    Cargo.toml
    src/lib.rs
examples/README.md
```

The placeholder shim currently returns stub values. The next milestone is to make it compile and run against real M5Unified on M5StickS3.

---

## Guiding decisions from the design discussion

- Develop `m5unified-sys` and `m5unified` in the same repo as separate publishable crates.
- Keep `m5unified-sys` raw, unsafe, and boring.
- Put ergonomics, `Result`, Rust enums, ownership conventions, and safe buffer APIs in `m5unified`.
- Start with M5StickS3 only. Do not attempt to support the full M5Stack board universe yet.
- First public API should be tiny:
  - `M5Unified::begin()`
  - display fill/cursor/print
  - button A/B state
  - mic begin/record i16 buffer
  - battery percentage
- Avoid a direct C++ binding generator strategy. Use a hand-written C ABI shim.
- Treat M5Unified source and the Arduino voice transmitter repo as hardware documentation where needed.

---

## Phase 0: Local development hygiene

### Task 0.1: Add rustfmt configuration

**Objective:** Make formatting expectations explicit for future agents/Codex.

**Files:**
- Create: `rustfmt.toml`

**Steps:**

1. Create `rustfmt.toml`:

```toml
edition = "2021"
max_width = 100
newline_style = "Unix"
```

2. Run formatting when Rust is installed:

```bash
cargo fmt --all
```

Expected: all Rust files formatted.

3. Commit:

```bash
git add rustfmt.toml crates/**/*.rs
 git commit -m "chore: add rustfmt configuration"
```

### Task 0.2: Add a basic CI workflow

**Objective:** Add a lightweight GitHub Actions workflow that catches syntax/formatting issues for host-buildable code.

**Files:**
- Create: `.github/workflows/ci.yml`

**Steps:**

1. Create workflow:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --all -- --check
      - run: cargo check --workspace
      - run: cargo clippy --workspace -- -D warnings
```

2. Run locally if Rust is available:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace -- -D warnings
```

3. Commit:

```bash
git add .github/workflows/ci.yml
 git commit -m "ci: add basic workspace checks"
```

**Note:** This CI only verifies the placeholder host-buildable skeleton. ESP-IDF hardware builds may need a dedicated job later.

---

## Phase 1: Make the workspace host-check clean

### Task 1.1: Decide whether host builds should link the shim or use docs-only placeholders

**Objective:** Avoid accidental unresolved extern link failures in host CI while preserving embedded intent.

**Files:**
- Modify: `crates/m5unified-sys/src/lib.rs`
- Modify: `crates/m5unified-sys/build.rs`
- Modify: `crates/m5unified-sys/Cargo.toml`

**Recommended approach:** Add a feature flag split:

- default feature: `stub` — pure Rust fallback symbols for host checking/docs.
- embedded feature: `esp-idf` — compile/link real C++ shim.

Example `Cargo.toml`:

```toml
[features]
default = ["stub"]
stub = []
esp-idf = []
```

In `src/lib.rs`, gate extern declarations:

```rust
#[cfg(feature = "esp-idf")]
mod ffi {
    use core::ffi::{c_char, c_int};

    extern "C" {
        pub fn m5u_begin() -> bool;
        pub fn m5u_update();
        pub fn m5u_display_fill_screen(color: u16);
        pub fn m5u_display_set_cursor(x: c_int, y: c_int);
        pub fn m5u_display_print(text: *const c_char);
        pub fn m5u_btn_a_is_pressed() -> bool;
        pub fn m5u_btn_b_was_pressed() -> bool;
        pub fn m5u_mic_begin() -> bool;
        pub fn m5u_mic_record_i16(buffer: *mut i16, samples: usize) -> bool;
        pub fn m5u_battery_level() -> c_int;
    }
}
```

Then expose safe raw wrappers or re-export the extern functions. For `stub`, define Rust functions with the same public surface returning defaults.

**Verification:**

```bash
cargo check --workspace
cargo check --workspace --no-default-features --features m5unified-sys/stub
```

Expected: host checks pass without ESP-IDF installed.

**Commit:**

```bash
git add crates/m5unified-sys
 git commit -m "chore: add host-checkable sys stubs"
```

### Task 1.2: Add unit tests for safe wrapper behavior that does not need hardware

**Objective:** Lock in simple wrapper semantics before hardware work.

**Files:**
- Modify: `crates/m5unified/src/lib.rs`

**Tests to add:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_print_rejects_interior_nul() {
        let mut display = Display;
        assert_eq!(display.print("hello\0world"), Err(Error::InvalidString));
    }

    #[test]
    fn battery_level_returns_none_for_stub_negative_value() {
        let power = Power;
        assert_eq!(power.battery_level(), None);
    }
}
```

**Verification:**

```bash
cargo test --workspace
```

Expected: tests pass on host.

**Commit:**

```bash
git add crates/m5unified/src/lib.rs
 git commit -m "test: cover basic safe wrapper behavior"
```

---

## Phase 2: Wire real ESP-IDF/M5Unified component build

### Task 2.1: Research esp-idf-sys native component integration

**Objective:** Determine the least-pain way to include M5Unified and M5GFX ESP-IDF components in a Rust ESP-IDF app.

**Files:**
- Create: `docs/research/esp-idf-component-integration.md`

**Questions to answer:**

- How does `esp-idf-sys` discover extra ESP-IDF components?
- Should `m5unified-sys` vendor M5Unified/M5GFX as git submodules, download them in `build.rs`, or expect the app to provide components?
- Can `embuild` compile a small C++ source file in the crate directly?
- How should component versions be pinned?

**Suggested sources:**

- `esp-rs/esp-idf-template`
- `esp-rs/esp-idf-sys`
- `m5stack/M5Unified` `idf_component.yml` and `CMakeLists.txt`
- `m5stack/M5GFX`

**Deliverable:** short markdown recommendation with one chosen path and rejected alternatives.

**Commit:**

```bash
git add docs/research/esp-idf-component-integration.md
 git commit -m "docs: research esp-idf component integration"
```

### Task 2.2: Add version-pinned component metadata

**Objective:** Pin M5Unified/M5GFX dependency strategy so builds are reproducible.

**Files:**
- Create or modify depending on chosen strategy:
  - `crates/m5unified-sys/idf_component.yml`, or
  - `crates/m5unified-sys/native/CMakeLists.txt`, or
  - `.gitmodules` and `vendor/` paths

**Recommended initial strategy:** Prefer ESP-IDF component manager metadata if it works from a Rust project; otherwise vendor as git submodules.

**Verification:**

```bash
git status --short
```

Expected: metadata files only; no huge generated build output.

**Commit:**

```bash
git add crates/m5unified-sys .gitmodules vendor docs/research/esp-idf-component-integration.md
 git commit -m "build: pin M5Unified component dependencies"
```

### Task 2.3: Replace placeholder shim with real M5Unified calls

**Objective:** Make `native/m5u_shim.cpp` call actual M5Unified APIs.

**Files:**
- Modify: `crates/m5unified-sys/native/m5u_shim.cpp`
- Modify: `crates/m5unified-sys/native/m5u_shim.h` if signatures change

**Target implementation:**

```cpp
#include "m5u_shim.h"
#include <M5Unified.h>

extern "C" {

bool m5u_begin(void) {
    auto cfg = M5.config();
    M5.begin(cfg);
    return true;
}

void m5u_update(void) {
    M5.update();
}

void m5u_display_fill_screen(uint16_t color) {
    M5.Display.fillScreen(color);
}

void m5u_display_set_cursor(int x, int y) {
    M5.Display.setCursor(x, y);
}

void m5u_display_print(const char* text) {
    if (text) {
        M5.Display.print(text);
    }
}

bool m5u_btn_a_is_pressed(void) {
    return M5.BtnA.isPressed();
}

bool m5u_btn_b_was_pressed(void) {
    return M5.BtnB.wasPressed();
}

bool m5u_mic_begin(void) {
    M5.Speaker.end();
    return M5.Mic.begin();
}

bool m5u_mic_record_i16(int16_t* buffer, size_t samples) {
    if (!buffer || samples == 0) {
        return false;
    }
    return M5.Mic.record(buffer, samples);
}

int m5u_battery_level(void) {
    return M5.Power.getBatteryLevel();
}

}
```

**Verification:**

Run the embedded build command identified in Task 2.1.

Expected: compile succeeds or fails with a concrete dependency/config issue documented in the commit/notes.

**Commit:**

```bash
git add crates/m5unified-sys/native
 git commit -m "feat: call M5Unified from C shim"
```

---

## Phase 3: Create real on-device examples

### Task 3.1: Add `hello-display` example crate

**Objective:** Prove Rust can initialize M5Unified and draw text on the M5StickS3 screen.

**Files:**
- Create: `examples/hello-display/Cargo.toml`
- Create: `examples/hello-display/src/main.rs`
- Modify: root `Cargo.toml` if examples are workspace members

**Example behavior:**

- call `M5Unified::begin()`
- fill screen black
- set cursor
- print `hello from rust`
- loop calling `update()`

**Pseudo-code:**

```rust
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let mut m5 = m5unified::M5Unified::begin()?;
    m5.display.fill_screen(0x0000);
    m5.display.set_cursor(10, 10);
    m5.display.print("hello from rust")?;

    loop {
        m5.update();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
```

**Verification:**

```bash
cd examples/hello-display
cargo build --target xtensa-esp32s3-espidf
espflash flash --monitor target/xtensa-esp32s3-espidf/debug/hello-display
```

Expected: M5StickS3 screen displays `hello from rust`.

**Commit:**

```bash
git add examples/hello-display Cargo.toml
 git commit -m "feat: add hello display example"
```

### Task 3.2: Add button example

**Objective:** Prove `M5.update()` and button state work through the wrapper.

**Files:**
- Create: `examples/button/Cargo.toml`
- Create: `examples/button/src/main.rs`

**Example behavior:**

- show default screen
- while Button A is pressed, fill screen green or print `A pressed`
- when Button B was pressed, print `B pressed`

**Verification:**

Flash to M5StickS3 and press buttons.

Expected: display changes according to button state.

**Commit:**

```bash
git add examples/button
 git commit -m "feat: add button example"
```

### Task 3.3: Add mic level example

**Objective:** Prove microphone recording works through the wrapper before attempting voice streaming.

**Files:**
- Create: `examples/mic-level/Cargo.toml`
- Create: `examples/mic-level/src/main.rs`
- Modify: `crates/m5unified/src/lib.rs` if wrapper needs an RMS helper or safer mic errors

**Example behavior:**

- initialize M5
- initialize mic
- record small `i16` buffers
- compute RMS or peak amplitude
- print level to serial and optionally draw a basic bar on display

**RMS helper:**

```rust
fn rms_i16(samples: &[i16]) -> f32 {
    let sum_sq: f32 = samples
        .iter()
        .map(|&s| {
            let s = s as f32;
            s * s
        })
        .sum();
    (sum_sq / samples.len().max(1) as f32).sqrt()
}
```

**Verification:**

Flash and speak/tap near the mic.

Expected: serial/display level changes with sound.

**Commit:**

```bash
git add examples/mic-level crates/m5unified/src/lib.rs
 git commit -m "feat: add microphone level example"
```

---

## Phase 4: Improve wrapper API after hardware proof

### Task 4.1: Replace boolean mic errors with `Result`

**Objective:** Make safe APIs expressive once real failure modes are known.

**Files:**
- Modify: `crates/m5unified/src/lib.rs`

**Approach:**

Add variants such as:

```rust
pub enum Error {
    BeginFailed,
    InvalidString,
    MicBeginFailed,
    MicRecordFailed,
}
```

Change mic methods:

```rust
pub fn begin(&mut self) -> Result<(), Error>;
pub fn record_i16(&mut self, buffer: &mut [i16]) -> Result<(), Error>;
```

**Verification:**

```bash
cargo test --workspace
```

Flash `examples/mic-level` again.

**Commit:**

```bash
git add crates/m5unified/src/lib.rs examples/mic-level
 git commit -m "refactor: return results from microphone wrapper"
```

### Task 4.2: Add basic colors and display convenience methods

**Objective:** Make examples readable without overbuilding a graphics API.

**Files:**
- Modify: `crates/m5unified/src/lib.rs`

**Add:**

```rust
pub mod color {
    pub const BLACK: u16 = 0x0000;
    pub const WHITE: u16 = 0xffff;
    pub const RED: u16 = 0xf800;
    pub const GREEN: u16 = 0x07e0;
    pub const BLUE: u16 = 0x001f;
}
```

**Verification:**

Use constants in examples and re-flash `hello-display`.

**Commit:**

```bash
git add crates/m5unified/src/lib.rs examples
 git commit -m "feat: add basic display color constants"
```

---

## Phase 5: Voice assistant integration boundary

### Task 5.1: Document how this repo relates to the voice assistant repo

**Objective:** Keep `m5unified-rs` focused while making the parallel project path obvious.

**Files:**
- Modify: `README.md`
- Create: `docs/voice-assistant-integration.md`

**Content:**

- This repo provides board bindings/wrappers only.
- The actual M5StickS3 voice assistant should live in a separate repo.
- Voice assistant repo should depend on `m5unified` via git/path initially.
- Initial voice assistant milestones:
  1. flash Rust hello-world
  2. button push-to-talk
  3. WiFi connect
  4. send test message
  5. mic level
  6. stream PCM chunks
  7. backend Whisper transcription

**Commit:**

```bash
git add README.md docs/voice-assistant-integration.md
 git commit -m "docs: describe voice assistant integration path"
```

---

## Hardware validation checklist

For every on-device example, record:

- Board: M5StickS3
- USB serial port
- ESP-IDF version
- Rust target: `xtensa-esp32s3-espidf`
- Exact command run
- Whether display worked
- Whether buttons worked
- Whether mic worked
- Serial monitor output
- Any required boot/upload button sequence

Create or update:

```text
docs/hardware/m5sticks3-validation.md
```

Commit validation logs separately:

```bash
git add docs/hardware/m5sticks3-validation.md
 git commit -m "docs: record M5StickS3 hardware validation"
```

---

## Open questions

- Should `m5unified-sys` vendor M5Unified/M5GFX, use ESP-IDF component manager dependencies, or expect app-level components?
- Can this be made usable from normal `esp-idf-template` apps without custom CMake glue?
- Which M5StickS3 display controller/panel settings does M5Unified auto-detect, and do we need any explicit config?
- What is the exact microphone sample format returned by `M5.Mic.record()` on StickS3?
- Does calling `M5.Speaker.end()` before `M5.Mic.begin()` remain necessary for StickS3 mic use?
- Is `getBatteryLevel()` reliable on StickS3, or should voltage/status be exposed too?

---

## Definition of done for v0.1.0

A minimal `0.1.0` is ready when:

- `cargo check --workspace` passes on host.
- `hello-display` builds and runs on M5StickS3.
- Button example builds and button state works on M5StickS3.
- Mic level example builds and responds to sound on M5StickS3.
- README documents install/build/flash steps.
- `m5unified-sys` and `m5unified` crate descriptions are accurate and warn that API is experimental.

Do not publish before at least display + button are validated on hardware.
