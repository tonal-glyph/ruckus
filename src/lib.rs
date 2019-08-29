#![allow(
    dead_code,
    improper_ctypes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_doc_comments,
    unused_imports,
    unused_mut
)]
#![deny(missing_docs)]
// Copyright (c) 2019 Andrew Prentice
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `ruckus` is a creative coding toolkit for the ChucK language. It includes:
//! * **ruckus** - lib - This library, for handling ChucK VMs
//! * **ruck** - binary - A simple command line wrapper for the chuck binary
extern crate chuck_sys;
use chuck_sys::*;