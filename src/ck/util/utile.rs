#![allow(
    dead_code,
    improper_ctypes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_mut
)]
#![feature(libc)]
use crate::chuck_def_h_edited::*;
use crate::dts::*;
///* common utils, adapted from Tiger Compiler Andrew Appel
use libc::*;
pub type U_boolList = *mut U_boolList_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct U_boolList_ {
    pub head: c_ulong,
    pub tail: U_boolList,
}
extern "C" {
    pub fn checked_malloc(size: c_int) -> *mut c_void;
}
extern "C" {
    pub fn cc_str(arg1: *mut c_char) -> c_str;
}
extern "C" {
    pub fn U_BoolList(head: c_ulong, tail: U_boolList) -> U_boolList;
}
