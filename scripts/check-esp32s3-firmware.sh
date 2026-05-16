#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/../firmware/hello-display"

# The Rust crate declares ESP-IDF symbols through the C ABI. The actual
# definitions are provided by the local ESP-IDF component in components/.
M5UNIFIED_RS_USE_REAL_M5UNIFIED=1 CMAKE_BUILD_PARALLEL_LEVEL="${CMAKE_BUILD_PARALLEL_LEVEL:-1}" \
  cargo build --target xtensa-esp32s3-espidf
