fn main() {
    # dependencies
    println!("cargo:rustc-link-search=../kjsl_c_lib/cmake-build-debug/");
    println!("cargo:rustc-link-lib=static=kjsl_c_lib");
}
