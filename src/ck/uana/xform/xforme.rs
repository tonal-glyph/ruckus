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
///* unit analyzer transforms
#![feature(libc)]
use libc::*;
use crate::ck::dynl::dynle::*;
pub fn DLLQUERY() {
    xform_query(query: *mut Chuck_DL_Query);
}
pub fn main() {
    DLLQUERY();
}
