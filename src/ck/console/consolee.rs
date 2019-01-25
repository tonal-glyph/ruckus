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
use crate::chuck_def_h_edited::*;
use crate::chuck_shell_h_edited::*;
///* chuck command line console
use libc;
extern "C" {
    pub static Chuck_Console: Chuck_Shell_UI;
}
// crate::chuck_shell_h_edited::Chuck_Shell_UI_init(this: *mut c_void);
