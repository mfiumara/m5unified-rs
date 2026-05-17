use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate should be nested under repo root")
        .to_path_buf()
}

#[test]
fn hello_display_firmware_scaffold_consumes_native_component() {
    let root = repo_root();
    let firmware = root.join("firmware/hello-display");

    assert!(firmware.join("Cargo.toml").exists());
    assert!(firmware.join("src/main.rs").exists());
    assert!(firmware.join(".cargo/config.toml").exists());
    assert!(firmware
        .join("components/m5unified-rs/CMakeLists.txt")
        .exists());
    assert!(firmware
        .join("components/m5unified-rs/idf_component.yml")
        .exists());

    let cargo = fs::read_to_string(firmware.join("Cargo.toml")).expect("read firmware manifest");
    assert!(cargo.contains("m5unified"));
    assert!(cargo.contains("esp-idf-sys"));

    let config =
        fs::read_to_string(firmware.join(".cargo/config.toml")).expect("read cargo config");
    assert!(config.contains("xtensa-esp32s3-espidf"));
    assert!(config.contains("ldproxy"));

    let cmake = fs::read_to_string(firmware.join("components/m5unified-rs/CMakeLists.txt"))
        .expect("read firmware component cmake");
    assert!(cmake.contains("crates/m5unified-sys/native"));
    assert!(cmake.contains("m5u_shim.cpp"));
    assert!(!cmake.contains("M5UNIFIED_RS_USE_REAL_M5UNIFIED"));

    let main_rs = fs::read_to_string(firmware.join("src/main.rs")).expect("read firmware main");
    assert!(main_rs.contains("M5Unified::begin"));
    assert!(main_rs.contains("hello from rust"));
    assert!(main_rs.contains("was_pressed"));
}

#[test]
fn root_workspace_excludes_target_specific_firmware() {
    let root_manifest =
        fs::read_to_string(repo_root().join("Cargo.toml")).expect("read root manifest");
    assert!(root_manifest.contains("exclude"));
    assert!(root_manifest.contains("firmware/hello-display"));
}
