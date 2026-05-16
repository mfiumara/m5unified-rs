# M5Unified hello-display firmware spike

This is the first real ESP-IDF Rust firmware package for the repo. It is intentionally separate from the host-checkable `m5unified-examples` package because it targets `xtensa-esp32s3-espidf` and requires the esp-rs toolchain.

Behavior:

- initializes `M5Unified` through the Rust `m5unified` crate;
- draws `hello from rust` through the C ABI shim;
- polls `M5.update()`;
- changes the display when Button A or Button B is pressed.

The `components/m5unified-rs` ESP-IDF component references `crates/m5unified-sys/native/m5u_shim.cpp` by relative path, so the firmware consumes the same native shim scaffold used by the Rust API instead of carrying a copied shim.

## Build

Install the esp-rs toolchain first. If Cargo reports `custom toolchain 'esp' ... is not installed`, install it with `espup`:

```bash
cargo +stable install espup
espup install
. ~/export-esp.sh
cargo +stable install espflash
```

Run the install commands with `+stable` (or from outside this firmware directory) because this directory's `rust-toolchain.toml` intentionally selects the not-yet-installed `esp` toolchain.

On macOS, if the ESP-IDF build later complains about missing host tools, install the common prerequisites:

```bash
brew install cmake ninja dfu-util ccache
```

ESP-IDF v5.3.x may fail if Homebrew's `python3` points at a too-new Python such as 3.14. If `.embuild` fails while creating `idf5.3_py3.14_env`, install Python 3.12 and put its `python3` shim first on `PATH` before rebuilding:

```bash
brew install python@3.12
export PATH="$(brew --prefix python@3.12)/libexec/bin:$PATH"
python3 --version  # should print Python 3.12.x
rm -rf .embuild/espressif/python_env
cargo build --target xtensa-esp32s3-espidf
```

If `ensurepip` fails with `pyexpat`/`libexpat` symbol errors, repair Homebrew's Python/expat linkage or use a non-Homebrew Python:

```bash
brew reinstall expat python@3.12
export PATH="$(brew --prefix python@3.12)/libexec/bin:$PATH"
export DYLD_LIBRARY_PATH="$(brew --prefix expat)/lib:${DYLD_LIBRARY_PATH:-}"
python3 -c 'import pyexpat; print(pyexpat.EXPAT_VERSION)'
rm -rf .embuild/espressif/python_env
cargo build --target xtensa-esp32s3-espidf
```

If Homebrew Python still imports `/usr/lib/libexpat.1.dylib`, use a pyenv Python instead:

```bash
brew install pyenv
pyenv install 3.11.9
export PATH="$(pyenv root)/versions/3.11.9/bin:$PATH"
python3 -c 'import pyexpat; print(pyexpat.EXPAT_VERSION)'
rm -rf .embuild/espressif/python_env
cargo build --target xtensa-esp32s3-espidf
```

Then from this directory run:

```bash
cargo build --target xtensa-esp32s3-espidf
```

## Flash

```bash
espflash flash --monitor target/xtensa-esp32s3-espidf/debug/m5unified-hello-display
```

Expected hardware result on M5StickS3-class hardware: the screen shows `hello from rust`; pressing Button A or B changes the screen text/background.
