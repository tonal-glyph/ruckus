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
extern crate chuck_sys;
extern crate cpal;
extern crate midir;
extern crate rosc;
extern crate sdl2;
extern crate serde_osc;
extern crate serial;
extern crate sndfile_sys;
use chuck_sys as sys;
use serde_osc as so;
use sndfile_sys as sf;
use sys::def::*;
