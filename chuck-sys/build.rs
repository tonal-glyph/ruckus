extern crate dotenv;
extern crate pkg_config;
// extern crate bindgen;
// extern crate cc;
// #[macro_use]
// extern crate cfg_if;
// use std::fs;
// use std::path::{Path, PathBuf};
// use std::process::Command;
// use std::io::Write;
// Add C++ files (from Edouard Belval's https://github.com/Belval/seal-rs)
// fn add_cpp_files(builder: &mut cc::Build, path: &Path) {
//     for e in path.read_dir().unwrap() {
//         let e = e.unwrap();
//         let path = e.path();
//         if e.file_type().unwrap().is_dir() {
//         } else if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
//             builder.file(&path);
//         }
//     }
// }
// // Add C files
// fn add_c_files(builder: &mut cc::Build, path: &Path) {
//     for e in path.read_dir().unwrap() {
//         let e = e.unwrap();
//         let path = e.path();
//         if e.file_type().unwrap().is_dir() {
//         } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
//             builder.file(&path);
//         }
//     }
// }
/// Generate bindings.rs from ChucK C wrapper API
// fn unixbind() {
//     let bindings = bindgen::Builder::default()
//         // .conservative_inline_namespaces()
//         // .trust_clang_mangling(false)
//         // .enable_cxx_namespaces()
//         .enable_function_attribute_detection()
//         .generate_block(true)
//         .generate_inline_functions(true)
//         .derive_default(true)
//         .impl_debug(true)
//         .impl_partialeq(true)
//         .derive_default(true)
//         .derive_eq(true)
//         .derive_hash(true)
//         .derive_ord(true)
//         // .whitelist_function("ChucK::.*")
//         .opaque_type("std::.*")
//         .header("./src/c_chuck.h")
//         .clang_arg("-B")
//         .clang_arg("--sysroot")
//         .clang_arg("-D__PLATFORM_LINUX__")
//         .clang_arg("-DHAVE_CONFIG_H")
//         .clang_arg("-DHAVE_LIBPTHREAD")
//         .clang_arg("-DHAVE_POLL")
//         .clang_arg("-DENABLE_THREADS")
//         .clang_arg("-D__CK_SNDFILE_NATIVE__")
//         .clang_arg("-D__LINUX_ALSA__")
//         .clang_arg("-DUSE_ALSA")
//         .clang_arg("-DUSE_DLTRICK_ALSA")
//         .clang_arg("-fPIC")
//         .clang_arg("-fno-strict-aliasing")
//         .clang_arg("-fstack-protector-strong")
//         .clang_arg("-fno-plt")
//         .clang_arg("-I./src/core/")
//         .clang_arg("-I./src/host/")
//         .clang_arg("-I./src/core/lo")
//         .clang_arg("-I./src/core/regex")
//         .clang_arg("-O3")
//         .clang_arg("-mtune=generic")
//         .clang_arg("-march=native")
//         .clang_arg("-pipe")
//         .generate()
//         .expect("Unable to generate bindings");
//     let _out_path = PathBuf::from("./src/");
//     let bindings_string = bindings.to_string();
//     let mut file = fs::File::create("./src/bindings.rs").unwrap();
//     let _write = match file.write_all(bindings_string.as_bytes()) {
//         Ok(r) => r,
//         Err(e) => panic!("Unable to write bindings file: {}", e)
//     };
//     let _sync = match file.sync_data() {
//         Ok(r) => r,
//         Err(e) => panic!("Unable to sync: {}", e)
//     };
// }
/// Build libchuck as a static or dynamic library (default dynamic)
// fn unixbuild() {
//     // Compile C/C++ sources
//     dotenv::dotenv().ok(); // get variables from .env file
//     // Command::new("bison")
//     //     .args(&["-dv", "-b", "src/core/chuck", "./src/core/chuck.y"])
//     //     .output()
//     //     .expect("failed to run bison on parser");
//     // Command::new("flex")
//     //     .args(&["-osrc/core/chuck.yy.c", "chuck.lex"])
//     //     .output()
//     //     .expect("failed to run flex on lexer");
//     // Command::new("gcc")
//     //     .args(&["-B --sysroot", "-D__PLATFORM_LINUX__", "-DHAVE_CONFIG_H", "-DHAVE_LIBPTHREAD", "-DHAVE_POLL", "-D__CK_SNDFILE_NATIVE__", "-D__LINUX_ALSA__", "-DUSE_ALSA", "-DUSE_DLTRICK_ALSA", "-fPIC", "-fno-strict-aliasing", "-fstack-protector-strong", "-fno-plt", "-I./src", "-I./src/core", "-I./src/core/lo", "-O3", "-mtune=generic", "-march=native", "-pipe", "./src/core/*.c", "./src/core/lo/*.c"])
//     //     .output()
//     //     .expect("failed to build C files");
//     // Command::new("g++")
//     //     .args(&["-B --sysroot", "-D__PLATFORM_LINUX__", "-DHAVE_CONFIG_H", "-DHAVE_LIBPTHREAD", "-DHAVE_POLL", "-D__CK_SNDFILE_NATIVE__", "-D__LINUX_ALSA__", "-DUSE_ALSA", "-DUSE_DLTRICK_ALSA", "-fPIC", "-fno-strict-aliasing", "-fstack-protector-strong", "-fno-plt", "-I./src", "-I./src/core", "-I./src/core/lo", "-O3", "-mtune=generic", "-march=native", "-pipe", "./src/c_chuck.cpp", "./src/core/*.cpp"])
//     //     .output()
//     //     .expect("failed to build C++ files");
//     let mut builder = cc::Build::new();
//     builder.include("./src");
//     builder.include("./src/core");
//     builder.include("./src/core/lo");
//     // builder.include("./src/core/regex");
//     // builder.include("./src/host");
//     // builder.include("./src/host/RtAudio");
//     builder.flag_if_supported("-B");
//     builder.flag_if_supported("--sysroot");
//     builder.flag_if_supported("-D__PLATFORM_LINUX__");
//     builder.flag_if_supported("-DHAVE_CONFIG_H");
//     builder.flag_if_supported("-DHAVE_POLL");
//     builder.flag_if_supported("-DHAVE_LIBPTHREAD");
//     builder.flag_if_supported("-DENABLE_THREADS");
//     builder.flag_if_supported("-D__CK_SNDFILE_NATIVE__");
//     builder.flag_if_supported("-D__LINUX_ALSA__");
//     builder.flag_if_supported("-DUSE_ALSA");
//     builder.flag_if_supported("-DUSE_DLTRICK_ALSA");
//     builder.flag_if_supported("-fPIC");
//     builder.flag_if_supported("-fno-strict-aliasing");
//     builder.flag_if_supported("-fstacore-protector-strong");
//     builder.flag_if_supported("-fno-plt");
//     builder.flag_if_supported("-fpermissive");
//     builder.flag_if_supported("-O3");
//     builder.flag_if_supported("-mtune=generic");
//     builder.flag_if_supported("-march=native");
//     builder.flag_if_supported("-pipe");
//     // let src_path = Path::new("./src/");
//     // let core_path = Path::new("./src/core/");
//     // let lo_path = Path::new("./src/core/lo/");
//     // let host_path = Path::new("./src/host/");
//     // let rtaudio_path = Path::new("./src/host/RtAudio/");
//     // add_c_files(&mut builder, core_path);
//     // add_c_files(&mut builder, lo_path);
//     // add_cpp_files(&mut builder, core_path);
//     // add_cpp_files(&mut builder, src_path);
//     // add_cpp_files(&mut builder, host_path);
//     // add_cpp_files(&mut builder, rtaudio_path);
//     // builder.file("./src/core/util_math.c");
//     // builder.file("./src/core/util_network.c");
//     // builder.file("./src/core/util_raw.c");
//     // builder.file("./src/core/util_sndfile.c");
//     // builder.file("./src/core/util_xforms.c");
//     builder.cpp(true);
//     builder.cpp_link_stdlib("stdc++");
//     builder.file("./src/c_chuck.cpp"); // C wrapper
//     builder.file("./src/core/chuck_absyn.cpp");
//     builder.file("./src/core/chuck_carrier.cpp");
//     builder.file("./src/core/chuck_compile.cpp");
//     builder.file("./src/core/chuck_dl.cpp");
//     builder.file("./src/core/chuck_emit.cpp");
//     builder.file("./src/core/chuck_errmsg.cpp");
//     builder.file("./src/core/chuck_frame.cpp");
//     builder.file("./src/core/chuck_instr.cpp");
//     builder.file("./src/core/chuck_io.cpp");
//     builder.file("./src/core/chuck_lang.cpp");
//     builder.file("./src/core/chuck_oo.cpp");
//     builder.file("./src/core/chuck_otf.cpp");
//     builder.file("./src/core/chuck_parse.cpp");
//     builder.file("./src/core/chuck_scan.cpp");
//     builder.file("./src/core/chuck_shell.cpp");
//     builder.file("./src/core/chuck_stats.cpp");
//     builder.file("./src/core/chuck_symbol.cpp");
//     builder.file("./src/core/chuck_table.cpp");
//     builder.file("./src/core/chuck_type.cpp");
//     builder.file("./src/core/chuck_ugen.cpp");
//     builder.file("./src/core/chuck_utils.cpp");
//     builder.file("./src/core/chuck_vm.cpp");
//     builder.file("./src/core/chuck.cpp");
//     builder.file("./src/core/hidio_sdl.cpp");
//     builder.file("./src/core/midiio_rtmidi.cpp");
//     builder.file("./src/core/rtmidi.cpp");
//     builder.file("./src/core/uana_extract.cpp");
//     builder.file("./src/core/uana_xform.cpp");
//     builder.file("./src/core/ugen_filter.cpp");
//     builder.file("./src/core/ugen_osc.cpp");
//     builder.file("./src/core/ugen_stk.cpp");
//     builder.file("./src/core/ugen_xxx.cpp");
//     builder.file("./src/core/ulib_machine.cpp");
//     builder.file("./src/core/ulib_math.cpp");
//     builder.file("./src/core/ulib_opsc.cpp");
//     builder.file("./src/core/ulib_regex.cpp");
//     builder.file("./src/core/ulib_std.cpp");
//     builder.file("./src/core/util_buffers.cpp");
//     builder.file("./src/core/util_console.cpp");
//     builder.file("./src/core/util_hid.cpp");
//     builder.file("./src/core/util_opsc.cpp");
//     builder.file("./src/core/util_serial.cpp");
//     builder.file("./src/core/util_string.cpp");
//     builder.file("./src/core/util_thread.cpp");
//     #[cfg(feature = "static")]
//     builder.compile("libchuck.a");
//     #[cfg(feature = "dynamic")]
//     builder.compile("libchuck.so")
// }
// fn winbind() {
//     println!("Windows bindings unimplemented.")
//     // cfg_if! {
//     //     if #[cfg(feature = "static")] {
//     //         builder.compile("libchuck.lib");
//     //     }
//     //     if #[cfg(feature = "dynamic")] {
//     //         builder.compile("libchuck.dll")
//     //     }
//     // }
// }
// fn winbuild() {
//     println!("Windows build unimplemented.")
// }
// fn osxbind() {
//     println!("OSX bindings unimplemented.")
// }
// fn osxbuild() {
//     println!("OSX build unimplemented.")
// }

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(feature = "static")]
    println!("the C runtime will be statically linked");
    #[cfg(feature = "dynamic")]
    println!("the C runtime will be dynamically linked");
    dotenv::dotenv().ok();
    pkg_config::Config::new()
        .atleast_version("0.1.0")
        .statik(true)
        .probe("chuck")
        .unwrap();
    // #[cfg(unix)]
    // unixbind();
    // #[cfg(unix)]
    // unixbuild();
    // #[cfg(win)]
    // winbind();
    // #[cfg(win)]
    // winbuild();
    // #[cfg(osx)]
    // osxbind();
    // #[cfg(osx)]
    // osxbuild();
}
