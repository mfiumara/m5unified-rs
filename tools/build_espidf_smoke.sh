#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLES_DIR="${ROOT}/examples"
TARGET="${ESP_IDF_TARGET_TRIPLE:-xtensa-esp32s3-espidf}"
SMOKE_BINS="${ESPIDF_SMOKE_BINS:-hello_display}"
CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"

export PYTHONPATH="${EXAMPLES_DIR}/tools/python${PYTHONPATH:+:${PYTHONPATH}}"
export CMAKE_BUILD_PARALLEL_LEVEL="${CMAKE_BUILD_PARALLEL_LEVEL:-1}"

cd "${EXAMPLES_DIR}"

for bin in ${SMOKE_BINS}; do
  cargo build --jobs "${CARGO_BUILD_JOBS}" --bin "${bin}" --target "${TARGET}"
done
