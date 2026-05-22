cmake_minimum_required(VERSION 3.16)

# ESP-IDF component wrapper for the Rust-facing M5Unified C ABI shim.
#
# This component is meant to be copied or symlinked into an esp-idf-sys project
# component directory, or included by a consuming ESP-IDF CMake build. It keeps
# the Rust FFI boundary small: Rust links to the flat C functions declared in
# m5u_shim.h, while the default C++ source calls the real M5Unified API.
#
# Firmware builds should use the real shim. The stub source is available only
# for host-side C ABI checks that intentionally do not link M5Unified.
# Set M5UNIFIED_RS_ENABLE_CARDPUTER=ON from the consuming project to compile the
# optional M5Cardputer keyboard path. The M5Cardputer Arduino library must also
# be available to that build.
option(M5UNIFIED_RS_USE_HOST_STUB "Build the no-op host C ABI shim instead of the firmware shim" OFF)

set(M5UNIFIED_RS_EXTRA_REQUIRES)
if(M5UNIFIED_RS_ENABLE_CARDPUTER)
    list(APPEND M5UNIFIED_RS_EXTRA_REQUIRES M5Cardputer)
endif()

if(M5UNIFIED_RS_USE_HOST_STUB)
    set(M5UNIFIED_RS_SHIM_SRCS "m5u_shim_stub.cpp")
    set(M5UNIFIED_RS_COMPONENT_REQUIRES "")
else()
    set(M5UNIFIED_RS_SHIM_SRCS "m5u_shim.cpp")
    set(M5UNIFIED_RS_COMPONENT_REQUIRES M5Unified M5GFX esp_timer driver ${M5UNIFIED_RS_EXTRA_REQUIRES})
endif()

idf_component_register(
    SRCS ${M5UNIFIED_RS_SHIM_SRCS}
    INCLUDE_DIRS "."
    REQUIRES ${M5UNIFIED_RS_COMPONENT_REQUIRES}
)

if(NOT M5UNIFIED_RS_USE_HOST_STUB)
    target_compile_definitions(${COMPONENT_LIB} PRIVATE M5UNIFIED_RS_USE_REAL_M5UNIFIED=1)
    if(M5UNIFIED_RS_ENABLE_CARDPUTER)
        target_compile_definitions(${COMPONENT_LIB} PRIVATE M5UNIFIED_RS_USE_REAL_M5CARDPUTER=1)
    endif()
endif()
target_compile_features(${COMPONENT_LIB} PRIVATE cxx_std_17)
