# ruck

`ruck` is a Rust wrapper around the ChucK command-line interface. It's also the [name](https://github.com/alltom/ruck) of another ChucK related project written in Ruby.

All ruck does is pass its arguments onto ChucK. It always adds the `--caution-to-the-wind` flag to allow Rust <=> ChucK interoperability. This flag allows the user to call the `Std.system()` function in ChucK code and use system commands (i.e. a Rust executable). If you call `cargo run -- src/default.ck` this will be demonstrated.

For more information on ChucK, including installation, see its [homepage](http://chuck.stanford.edu).

## goals

- [x] - Call ChucK from Rust code
- [x] - Call Rust from ChucK code

## platform

Right now Linux is the main focus of this small tool, but it's simple enough that it should be adaptable for OSX and Windows.