fn main() {
    println!("cargo:rerun-if-changed=native/CMakeLists.txt");
    println!("cargo:rerun-if-changed=native/idf_component.yml");
    println!("cargo:rerun-if-changed=native/m5u_shim.h");
    println!("cargo:rerun-if-changed=native/m5u_shim.cpp");
    println!("cargo:rerun-if-changed=native/m5u_shim_stub.cpp");
}
