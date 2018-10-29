#![allow(dead_code)]
#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![allow(unused_imports)]

extern crate bindgen;
extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    env::set_var("CC_x86_64-unknown-linux-gnu", "clang");
    env::set_var("CXX_x86_64-unknown-linux-gnu", "clang");
    env::set_var("HOST_CC", "clang");
    env::set_var("LD", "clang");
    env::set_var("OPT_LEVEL", "3");
    env::set_var("CFLAGS_x86_64-unknown-linux-gnu", "-O3 -march=native -fpermissive -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__PLATFORM_LINUX__ -O3 -fno-strict-aliasing -D__CK_SNDFILE_NATIVE__ -D__LINUX_PULSE__");
    env::set_var("CXXFLAGS_x86_64-unknown-linux-gnu", "-O3 -march=native -fpermissive -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__PLATFORM_LINUX__ -O3 -fno-strict-aliasing -D__CK_SNDFILE_NATIVE__ -D__LINUX_PULSE__");
    env::set_var(
        "LDFLAGS_x86_64-unknown-linux-gnu",
        "-lasound -ljack -lc++ -ldl -lm -lsndfile -lpthread -lpulse-simple -lpulse",
    );
    println!("cargo:include=chuck-lib");
    cc::Build::new()
        .file("chuck-lib/chuck.cpp")
        .file("chuck-lib/chuck_vm.cpp")
        .compile("chucklib");
    let bindings = bindgen::Builder::default()
        .header("chuck-lib/chuck.h")
        .header("chuck-lib/chuck_vm.h")
        .generate()
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
