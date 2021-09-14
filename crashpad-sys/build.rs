fn link_crashpad_lib(module: &str) {
    println!("cargo:rustc-link-lib=static=crashpad_{}", module);
}

fn link_crashpad_3rd(module: &str) {
    println!("cargo:rustc-link-lib=static={}", module);
}

fn build_crashpad() {
    println!("cargo:rerun-if-changed=crashpad/");

    let dst = cmake::build("crashpad");

    #[cfg(not(target_os = "macos"))]
    link_crashpad_lib("compat");
    link_crashpad_lib("tools");
    link_crashpad_3rd("mini_chromium");
    link_crashpad_lib("util");
    link_crashpad_lib("client");
    link_crashpad_lib("snapshot");
    link_crashpad_lib("minidump");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib32", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
}

fn bindgen_crashpad_wrapper() {
    let out_dir_str = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir_str);

    let mut config = bindgen::CodegenConfig::empty();
    config.insert(bindgen::CodegenConfig::FUNCTIONS);

    bindgen::Builder::default()
        .with_codegen_config(config)
        .header("wrapper/lib.h")
        // Ref: https://github.com/rust-lang-nursery/rust-bindgen/issues/550
        .blocklist_type("max_align_t")
        .ctypes_prefix("libc")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn build_crashpad_wrapper() {
    println!("cargo:rerun-if-changed=wrapper/");

    cc::Build::new()
        .cpp(true)
        .include("crashpad")
        .include("crashpad/third_party/mini_chromium")
        .include("crashpad/third_party/mini_chromium/mini_chromium")
        .file("wrapper/lib.cc")
        // Ref: crashpad/CMakeLists.txt
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-std:c++14")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-wd4100")
        .compile("libcrashpad_wrapper.a");
}

fn main() {
    #[cfg(target_os = "macos")]
    panic!(
        "Doesn't support macOS because I don't have macOS. \
         Contributions are welcome!"
    );

    println!("cargo:rerun-if-changed=build.rs");
    bindgen_crashpad_wrapper();
    build_crashpad_wrapper();
    build_crashpad();
}
