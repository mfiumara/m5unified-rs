#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/../examples"

# The Rust crate declares ESP-IDF symbols through the C ABI. The actual
# definitions are provided by the local ESP-IDF component in examples/components/.
CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CMAKE_BUILD_PARALLEL_LEVEL="${CMAKE_BUILD_PARALLEL_LEVEL:-1}" \
  cargo build --jobs "${CARGO_BUILD_JOBS}" --bin hello_display --target xtensa-esp32s3-espidf
