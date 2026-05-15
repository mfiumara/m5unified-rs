use std::env;

fn main() {
    println!("cargo:rerun-if-changed=native/m5u_shim.h");
    println!("cargo:rerun-if-changed=native/m5u_shim.cpp");
    println!("cargo:rerun-if-env-changed=M5UNIFIED_SYS_REQUIRE_REAL");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("native/m5u_shim.cpp")
        .include("native")
        .flag_if_supported("-std=c++17");

    if env::var_os("M5UNIFIED_SYS_REQUIRE_REAL").is_some() {
        build.define("M5U_REQUIRE_REAL_M5UNIFIED", None);
    }

    build.compile("m5u_shim");
}
