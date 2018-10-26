#![allow(dead_code)]
#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![allow(unused_imports)]

extern crate bindgen;
extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    env::set_var("CC", "clang");
    env::set_var("CXX", "clang");
    env::set_var("LD", "clang");
    env::set_var("CFLAGS_x86_64-unknown-linux-gnu", "-std=c++17 -O3 -march=native -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__PLATFORM_LINUX__ -O3 -fno-strict-aliasing -D__CK_SNDFILE_NATIVE__ -D__LINUX_PULSE__");
    env::set_var("CXXFLAGS_x86_64-unknown-linux-gnu", "-std=c++17 -O3 -march=native -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__PLATFORM_LINUX__ -O3 -fno-strict-aliasing -D__CK_SNDFILE_NATIVE__ -D__LINUX_PULSE__");
    env::set_var("LDFLAGS_x86_64-unknown-linux-gnu", "-lasound -ljack -lstdc++ -ldl -lm -lsndfile -lpthread -lpulse-simple -lpulse");
    println!("cargo:include=chuck/src/core");
    cc::Build::new()
        .file("chuck/src/core/chuck.cpp")
        .file("chuck/src/core/chuck_vm.cpp")
        .compile("chucklib");
    let bindings = bindgen::Builder::default()
        .header("chuck/src/core/chuck.hpp")
        .generate()
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
