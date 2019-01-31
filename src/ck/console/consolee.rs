#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_mut
)]
#![feature(libc)]
use libc;
use crate::ck::def::defe::*;
use crate::ck::shell::shelle::*;
///* chuck command line console
extern "C" {
    pub static Chuck_Console: Chuck_Shell_UI;
}
fn main () {
    Chuck_Shell_UI_init(this: *mut c_void);
}
