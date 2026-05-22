use std::fs;
use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn native_component_cmake_registers_the_shim_and_m5unified_dependencies() {
    let cmake_path = crate_root().join("native/CMakeLists.txt");
    let cmake = fs::read_to_string(&cmake_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", cmake_path.display()));

    assert!(cmake.contains("m5u_shim.cpp"));
    assert!(cmake.contains("m5u_shim_stub.cpp"));
    assert!(cmake.contains("M5UNIFIED_RS_USE_HOST_STUB"));
    assert!(!cmake.contains("M5UNIFIED_RS_USE_REAL_M5UNIFIED"));
    assert!(cmake.contains("M5Unified"));
    assert!(cmake.contains("M5GFX"));
    assert!(cmake.contains("idf_component_register"));
}

#[test]
fn native_component_manifest_declares_managed_dependencies() {
    let manifest_path = crate_root().join("native/idf_component.yml");
    let manifest = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", manifest_path.display()));

    assert!(manifest.contains("m5stack/M5Unified"));
    assert!(manifest.contains("m5stack/M5GFX"));
}
