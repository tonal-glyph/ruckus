extern crate cmake;
extern crate dotenv;
extern crate bindgen;
// extern crate cc;
// #[macro_use]
// extern crate cfg_if;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::Write;
use cmake::Config;
/// Generate ffi.rs from ChucK C wrapper API
fn unixbind() {
    let bindings = bindgen::Builder::default()
        .conservative_inline_namespaces()
        .trust_clang_mangling(false)
        .enable_cxx_namespaces()
        .enable_function_attribute_detection()
        .generate_block(true)
        .generate_inline_functions(true)
        .derive_default(true)
        .impl_debug(true)
        .impl_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .whitelist_function("Chuck_Carrier::.*")
        .whitelist_function("ChucK::.*")
        .whitelist_function("ck_fprintf_stderr")
        .whitelist_function("EM_.*")
        .whitelist_function("stk_detach")
        .whitelist_type("Chuck_Carrier::.*")
        .whitelist_type("Chuck_Compiler::.*")
        .whitelist_type("Chuck_IO_Serial::.*")
        .whitelist_type("CHUCK_PARAM.*")
        .whitelist_type("Chuck_VM::.*")
        .whitelist_type("ChucK::.*")
        .whitelist_type("CK_LOG.*")
        .whitelist_type("ck_param_type")
        .whitelist_type("HidInManager::.*")
        .whitelist_type("MAXPATHLEN")
        .whitelist_type("t_CK.*")
        .whitelist_var("Chuck_Carrier::.*")
        .whitelist_var("ChucK::.*")
        .whitelist_var("MAXPATHLEN")
        .whitelist_var("t_CK.*")
        .opaque_type("std::.*")
        .header("./c_chuck.h")
        .clang_arg("-B")
        .clang_arg("--sysroot")
        .clang_arg("-D__PLATFORM_LINUX__")
        .clang_arg("-DHAVE_CONFIG_H")
        .clang_arg("-DHAVE_LIBPTHREAD")
        .clang_arg("-DHAVE_POLL")
        .clang_arg("-DENABLE_THREADS")
        .clang_arg("-D__CHUCK_NO_MAIN__") // empty int main()
        .clang_arg("-D__CK_SNDFILE_NATIVE__")
        .clang_arg("-D__LINUX_ALSA__")
        .clang_arg("-fPIC")
        .clang_arg("-fno-strict-aliasing")
        .clang_arg("-fstack-protector-strong")
        .clang_arg("-fno-plt")
        .clang_arg("-I../chuck/src/core/")
        .clang_arg("-I../chuck/src/host/")
        .clang_arg("-I../chuck/src/core/lo")
        // .clang_arg("-I../chuck/src/core/regex") #windows only
        .clang_arg("-O3")
        .clang_arg("-mtune=generic")
        .clang_arg("-march=native")
        .clang_arg("-pipe")
        .generate()
        .expect("Unable to generate bindings");
    let _out_path = PathBuf::from("./src/");
    let bindings_string = bindings.to_string();
    let mut file = fs::File::create("./src/ffi.rs").unwrap();
    let _write = match file.write_all(bindings_string.as_bytes()) {
        Ok(r) => r,
        Err(e) => panic!("Unable to write bindings file: {}", e)
    };
    let _sync = match file.sync_data() {
        Ok(r) => r,
        Err(e) => panic!("Unable to sync: {}", e)
    };
}
/// Build libchuck as a static or dynamic library (default static)
fn unixbuild() {
    // Compile C/C++ sources
    dotenv::dotenv().ok(); // get variables from .env file
    Command::new("bison") // build parser (chuck.tab.c, chuck.tab.h)
        .args(&["-dv", "-b", "../chuck/src/core/chuck", "../chuck/src/core/chuck.y"])
        .output()
        .expect("failed to run bison on parser");
    Command::new("flex") // build lexer (chuck.yy.c)
        .args(&["-o ../chuck/src/core/chuck.yy.c", "chuck.lex"])
        .output()
        .expect("failed to run flex on lexer");
    // Makefile version
    Command::new("make") // build chuck binary
        .args(&["-f", "../chuck/src/makefile", "linux-alsa"])
        .output()
        .expect("failed to run make");
    // let _dst = Config::new("libchuck")
    //     .generator("Ninja") // for speed
    //     .build();
}

fn main() {
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock")
    );
    #[cfg(feature = "stat")]
    println!("the C runtime will be statically linked");
    #[cfg(feature = "dyn")]
    println!("the C runtime will be dynamically linked");
    let dst = Config::new("libchuck")
        .define("CMAKE_INSTALL_PREFIX", "/tmp")
        .cflag("-D__PLATFORM_LINUX__")
        .cflag("-D__CK_SNDFILE_NATIVE__")
        .no_build_target(true)
        .generator("Ninja") // for speed
        .build();
    dotenv::dotenv().ok();
    #[cfg(unix)]
    unixbind();
    #[cfg(unix)]
    unixbuild();
    println!("cargo:rustc-link-search=native={}", dst.display());
    #[cfg(feature = "stat")]
    println!("cargo:rustc-link-lib=static=libchuck");
    #[cfg(feature = "dyn")]
    println!("cargo:rustc-link-lib=dynamic=libchuck");
}
