# Reuse the repository's native shim as an ESP-IDF component without copying it.
# Paths are relative to this component directory:
# firmware/hello-display/components/m5unified-rs
set(M5UNIFIED_RS_NATIVE_DIR "${CMAKE_CURRENT_LIST_DIR}/../../../../crates/m5unified-sys/native")

idf_component_register(
    SRCS "${M5UNIFIED_RS_NATIVE_DIR}/m5u_shim.cpp"
    INCLUDE_DIRS "${M5UNIFIED_RS_NATIVE_DIR}"
    REQUIRES M5Unified M5GFX esp_timer driver
)
