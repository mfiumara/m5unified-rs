use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate should be nested under repo root")
        .to_path_buf()
}

#[test]
fn hello_display_example_scaffold_consumes_native_component() {
    let root = repo_root();
    let examples = root.join("examples");

    assert!(examples.join("Cargo.toml").exists());
    assert!(examples.join("src/bin/hello_display.rs").exists());
    assert!(examples.join(".cargo/config.toml").exists());
    assert!(root.join("tools/build_espidf_smoke.sh").exists());
    assert!(root.join("docs/examples/hardware-verification.md").exists());
    assert!(examples
        .join("components/m5unified-rs/CMakeLists.txt")
        .exists());
    assert!(examples
        .join("components/m5unified-rs/idf_component.yml")
        .exists());
    assert!(root.join("components_esp32s3.lock").exists());

    let cargo = fs::read_to_string(examples.join("Cargo.toml")).expect("read examples manifest");
    assert!(cargo.contains("m5unified"));
    assert!(cargo.contains("esp-idf-sys"));
    assert!(cargo.contains("cfg(target_os = \"espidf\")"));

    let config =
        fs::read_to_string(examples.join(".cargo/config.toml")).expect("read cargo config");
    assert!(config.contains("xtensa-esp32s3-espidf"));
    assert!(config.contains("ESP_IDF_SYS_ROOT_CRATE"));
    assert!(config.contains("ESP_IDF_SDKCONFIG_DEFAULTS"));
    assert!(config.contains("ldproxy"));

    let cmake = fs::read_to_string(examples.join("components/m5unified-rs/CMakeLists.txt"))
        .expect("read examples component cmake");
    assert!(cmake.contains("crates/m5unified-sys/native"));
    assert!(cmake.contains("m5u_shim.cpp"));
    assert!(cmake.contains("fatfs"));
    assert!(cmake.contains("sdmmc"));
    assert!(!cmake.contains("M5UNIFIED_RS_USE_REAL_M5UNIFIED"));

    let main_rs =
        fs::read_to_string(examples.join("src/bin/hello_display.rs")).expect("read example main");
    assert!(main_rs.contains("M5Unified::begin"));
    assert!(main_rs.contains("hello from rust"));
    assert!(main_rs.contains("was_pressed"));
    assert!(main_rs.contains("link_patches"));

    let smoke_script =
        fs::read_to_string(root.join("tools/build_espidf_smoke.sh")).expect("read smoke script");
    assert!(smoke_script.contains("xtensa-esp32s3-espidf"));
    assert!(smoke_script.contains("ESPIDF_SMOKE_BINS"));
    assert!(smoke_script.contains("hello_display"));

    let hardware_doc = fs::read_to_string(root.join("docs/examples/hardware-verification.md"))
        .expect("read hardware verification doc");
    assert!(hardware_doc.contains("tools/build_espidf_smoke.sh"));
    assert!(hardware_doc.contains("hello_display"));
    assert!(hardware_doc.contains("pending hardware"));
}

#[test]
fn root_workspace_includes_examples_without_firmware_exclude() {
    let root_manifest =
        fs::read_to_string(repo_root().join("Cargo.toml")).expect("read root manifest");
    assert!(root_manifest.contains("\"examples\""));
    assert!(!root_manifest.contains("firmware/hello-display"));
}
