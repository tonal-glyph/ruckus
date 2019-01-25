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
use crate::chuck_carrier_h_edited::*;
use crate::chuck_def_h_edited::*;
use crate::chuck_dl_h_edited::*;
///* ChucK import for Synthesis ToolKit (STK)
///*                        by Perry Cook and Gary Scavone
use libc::*;
pub fn DLLQUERY() {
    xform_query(QUERY: *mut Chuck_DL_Query);
}
pub fn CKBOOL() {
    stk_detach(carrier: *mut Chuck_Carrier);
}
pub fn main() {
    DLLQUERY();
    CKBOOL();
}
