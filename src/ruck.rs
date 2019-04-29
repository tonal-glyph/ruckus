#![deny(missing_docs)]
// Copyright (c) 2019 Andrew Prentice
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruck` is a Rust wrapper around the ChucK command line. All it does is pass its
//! arguments onto ChucK. It depends on having ChucK installed and in your PATH. The
//! undocumented `--caution-to-the-wind` flag is always passed to allow interoperability
//! with Rust, as this allows using the Std.system() function from ChucK's standard library
//! which is used to run system commands.

use std::env;
use std::process::Command;

/// Handles one or more args passed to chuck binary
pub fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.push("--caution-to-the-wind".to_string());
    Command::new("chuck")
        .args(&args[1..])
        .spawn()
        .expect("failed to run chuck")
        .wait()
        .expect("failed to wait for subprocess");
}

#[cfg(test)]
mod tests {
    #[test]
    fn echo() {
        Command::new("ruck")
            .arg("./tests/echo.ck")
            .spawn()
            .expect("failed to run ruck")
            .wait()
            .expect("failt to wait for subprocess");
    }
}
