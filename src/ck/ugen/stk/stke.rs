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
///* ChucK import for Synthesis ToolKit (STK)
///*                        by Perry Cook and Gary Scavone
#![feature(libc)]
use libc::*;
use crate::ck::carry::carrye::*;
use crate::ck::def::defe::*;
use crate::ck::dynl::dynle::*;
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
