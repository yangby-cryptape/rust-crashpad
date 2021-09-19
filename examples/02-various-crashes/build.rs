fn bindgen_libcdemo() {
    let out_dir_str = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir_str);

    let mut config = bindgen::CodegenConfig::empty();
    config.insert(bindgen::CodegenConfig::FUNCTIONS);

    bindgen::Builder::default()
        .with_codegen_config(config)
        .header("c/lib.h")
        .ctypes_prefix("libc")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn build_libcdemo() {
    println!("cargo:rerun-if-changed=c/");
    cc::Build::new().file("c/lib.c").compile("libcdemo.a");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    bindgen_libcdemo();
    build_libcdemo();
}
