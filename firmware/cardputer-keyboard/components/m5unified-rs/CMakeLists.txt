# Reuse the repository's native shim as an ESP-IDF component without copying it.
# Paths are relative to this component directory:
# firmware/cardputer-keyboard/components/m5unified-rs
set(M5UNIFIED_RS_NATIVE_DIR "${CMAKE_CURRENT_LIST_DIR}/../../../../crates/m5unified-sys/native")

idf_component_register(
    SRCS "${M5UNIFIED_RS_NATIVE_DIR}/m5u_shim.cpp"
    INCLUDE_DIRS "${M5UNIFIED_RS_NATIVE_DIR}"
    REQUIRES M5Unified M5GFX M5Cardputer IRremote esp_timer driver
)

target_compile_definitions(
    ${COMPONENT_LIB}
    PRIVATE
        M5UNIFIED_RS_USE_REAL_M5UNIFIED=1
        M5UNIFIED_RS_USE_REAL_M5CARDPUTER=1
        M5UNIFIED_RS_USE_ARDUINO_GPIO=1
        M5UNIFIED_RS_USE_ARDUINO_SD=1
        M5UNIFIED_RS_USE_ARDUINO_SERIAL=1
        M5UNIFIED_RS_USE_ARDUINO_WIRE=1
        M5UNIFIED_RS_USE_ARDUINO_IRREMOTE=1
)
target_compile_features(${COMPONENT_LIB} PRIVATE cxx_std_17)
