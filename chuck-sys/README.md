# chuck-sys

## Strongly-timed, Concurrent, and On-the-fly Music Programming Language

[![Build Status](https://travis-ci.org/tonal-glyph/chuck-sys.svg?branch=master)](https://travis-ci.org/tonal-glyph/chuck-sys)

This crate is a project to create Rust bindings to the ChucK creative coding language. ChucK was created by Dr. Ge Wang of Stanford University's Center for Computer Research in Music and Acoustics (or CCRMA, pronounced "karma"). Perry Cook and Spencer Salazar also played significant roles in it's development. All authors are documented in the original C/C++ code. Building on the tradition of other audio languages like SuperCollider and FAUST, ChucK is a strongly-typed, strongly-timed object-oriented language. It is designed to be concise with a familiar C-like syntax, and to be useful for teaching programming concepts. For more information about ChucK, please see the [reference implementation](https://github.com/ccrma/chuck) or the [homepage](http://chuck.stanford.edu)

## installation

You'll have to pardon me as this is a little convoluted right now. Eventually I must automate this for sanity reasons.

1. Clone the repository and `cd ruckus/chuck-sys/src`.

2. Run one of the following: `make linux-alsa`, `make linux-pulse`, `make linux-jack`, `make osx`, `make cygwin`, or `make win32`. Once the binary is built, run `sudo make install`. Now you have `chuck`! If you don't do this step, you must manually run lex/flex and bison/yacc to generate the lexer and parser sources or the next step probably won't work right.

3. Next, `cd ../build` and `cmake ..` with the generator of your choice, I prefer Ninja for its speed. Running make/ninja/whatever should build `libchuck.a`.

4. Copy `libchuck.a` to `/usr/local/lib` or your system's equivalent, copy the header files in `src/core` to `/usr/local/include/chuck`, and copy all header files in `src/core/lo` and `src/core/regex' to those subfolders in `/usr/local/include/chuck`. Copy `chuck.pc` to your PKG_CONFIG_PATH (`/usr/local/lib/pkgconfig`).

5. Now you should be able to run `cargo build` and this bindings crate will link to libchuck.a and give you raw ChucK power!!!

## development dependencies

chuck-sys development uses a few 'bread and butter' tools nearly always found on Linux, MacOSX and MingW installations:

- automake or [just](https://github.com/casey/just) as the task runner
- cmake for generating compile_commands.json and building libchuck static library
- ctags (optional) (I prefer [ptags](https://github.com/dalance/ptags))
- GCC/Clang, right now I'm focusing on GCC. Testing Clang and Visual Studio will come later.
- lex/flex for building the language lexer. Most will probably use flex.
- bison/yacc for building the language parser. Most will probably use bison.
- pkg-config for linking to the static library

These tools are easy to install with a package manager if they're missing from your system. In addition, the following Rust crates are used to run the build tasks.

- [bindgen](https://github.com/rust-lang/rust-bindgen) - generates Rust bindings to the C wrapper
- ~~[cc](https://github.com/alexcrichton/cc-rs) - Building C/C++ code into libchuck binary~~
- [just](https://github.com/casey/just) - a task runner inspired by automake

## a word about libchuck

libchuck is brand new in the world of ChucK, having been added to the language in the 1.4.0.0 release. The goal is to make it easier to embed ChucK and spawn multiple VMs very quickly. The single-header approach has thus been implemented, that header being `src/core/chuck.h` or an even more stripped down version `design/chuck-lib/chuck.h`. This repository uses the former one and has two goals:

- Automate building libchuck
- Binding to Rust

As such it has been heavily refactored, mainly to help me navigate a large codebase without repetitive stress injuries. I fixed some inconsistent whitespace and indentation, removed a lot of blank lines, etc. You are encouraged to use the canonical source unless you have these same goals and are okay with the changes I've made. My goal is to keep the C/C++ sources compiling throughout, even if I have to grab the original file that's having problems and start over.

## status

To help out please see the [Issues](https://github.com/tonal-glyph/ruckus/issues) tab and tonal glyph's [project board](https://github.com/orgs/tonal-glyph/projects/2).

## todo

- [ ] test binding layouts
- [ ] generate windows bindings
- [ ] benchmark VM creation, maybe experiment with crossbeam? ChucK is already concurrent, but there's always room for improvement...

## license

This bindings library is licensed under GPL2 to match the licensing on the original library.

These open-source projects help ChucK work, they all have their own respective licenses:

- [liblo](https://github.com/radarsat1/liblo) for open sound control - by Stephen Sinclair
- [libsndfile](https://github.com/erikd/libsndfile/) for audio file format handling - by Erik de Castro Lopo
- [osc-kit](https://github.com/CNMAT/CNMAT-OSC) for open sound control - by [CNMAT](http://cnmat.berkeley.edu/)
- [rtaudio](https://github.com/thestk/rtaudio) for audio io - by Gary Scavone
- [rtmidi](https://github.com/thestk/rtmidi) for midi io - by Gary Scavone
- [sdl](https://www.libsdl.org) for HID io - by Sam Latinga, et al.
- [synthesis toolkit](https://github.com/thestk/stk) for algorithms in the std lib - by Perry Cook & Gary Scavone
- [tiger-compiler](https://www.cs.princeton.edu/~appel/modern/c/project.html) for language features -  by Andrew Appel
- [tre-regex](https://github.com/laurikari/tre) -  by Ville Laurikari. (Windows only)
