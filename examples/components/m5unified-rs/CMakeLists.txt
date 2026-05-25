# Reuse the repository's native shim as an ESP-IDF component without copying it.
# Paths are relative to this component directory:
# examples/components/m5unified-rs
set(M5UNIFIED_RS_NATIVE_DIR "${CMAKE_CURRENT_LIST_DIR}/../../../crates/m5unified-sys/native")

set(M5UNIFIED_RS_COMPONENT_REQUIRES M5Unified M5GFX esp_timer driver fatfs sdmmc)
if(IDF_VERSION_MAJOR GREATER 5 OR (IDF_VERSION_MAJOR EQUAL 5 AND IDF_VERSION_MINOR GREATER_EQUAL 3))
    list(APPEND M5UNIFIED_RS_COMPONENT_REQUIRES esp_driver_sdspi esp_driver_spi)
endif()

idf_component_register(
    SRCS "${M5UNIFIED_RS_NATIVE_DIR}/m5u_shim.cpp"
    INCLUDE_DIRS "${M5UNIFIED_RS_NATIVE_DIR}"
    REQUIRES ${M5UNIFIED_RS_COMPONENT_REQUIRES}
)
