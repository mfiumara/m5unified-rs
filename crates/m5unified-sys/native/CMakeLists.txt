cmake_minimum_required(VERSION 3.16)

# ESP-IDF component wrapper for the Rust-facing M5Unified C ABI shim.
#
# This component is meant to be copied or symlinked into an esp-idf-sys project
# component directory, or included by a consuming ESP-IDF CMake build. It keeps
# the Rust FFI boundary small: Rust links to the flat C functions declared in
# m5u_shim.h, while this C++ source calls the real M5Unified API.

idf_component_register(
    SRCS "m5u_shim.cpp"
    INCLUDE_DIRS "."
    REQUIRES M5Unified M5GFX esp_timer driver
)

target_compile_definitions(${COMPONENT_LIB} PRIVATE M5UNIFIED_RS_USE_REAL_M5UNIFIED=1)
target_compile_features(${COMPONENT_LIB} PRIVATE cxx_std_17)
