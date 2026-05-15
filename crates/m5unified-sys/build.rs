fn main() {
    // Placeholder build script.
    // The real implementation will compile/link a C++ shim against M5Unified
    // and M5GFX as ESP-IDF components.
    println!("cargo:rerun-if-changed=native/m5u_shim.h");
    println!("cargo:rerun-if-changed=native/m5u_shim.cpp");
}
