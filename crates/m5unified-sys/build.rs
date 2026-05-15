use std::env;

fn main() {
    println!("cargo:rerun-if-changed=native/m5u_shim.h");
    println!("cargo:rerun-if-changed=native/m5u_shim.cpp");
    println!("cargo:rerun-if-env-changed=M5UNIFIED_RS_USE_REAL_M5UNIFIED");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let use_real = env::var_os("M5UNIFIED_RS_USE_REAL_M5UNIFIED").is_some();

    if target_os == "espidf" && use_real {
        // The actual ESP-IDF component wiring is expected to be supplied by the
        // consuming esp-idf-sys project/CMake configuration. We still expose a
        // cfg so Rust code and docs can distinguish real-hardware builds from
        // host-stub checks.
        println!("cargo:rustc-cfg=m5unified_rs_real_m5unified");
        println!("cargo:warning=M5UNIFIED_RS_USE_REAL_M5UNIFIED is set; ensure native/m5u_shim.cpp is compiled as an ESP-IDF C++ component with M5Unified/M5GFX available.");
    } else if target_os == "espidf" {
        println!("cargo:warning=building for ESP-IDF without M5UNIFIED_RS_USE_REAL_M5UNIFIED; link native/m5u_shim.cpp and M5Unified/M5GFX before flashing.");
    }
}
