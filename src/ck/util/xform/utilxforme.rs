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
use crate::sys::*;
///* transform utilities
use libc::*;
extern "C" {
    pub fn hanning(window: *mut f64, length: c_ulong);
}
extern "C" {
    pub fn hamming(window: *mut f64, length: c_ulong);
}
extern "C" {
    pub fn blackman(window: *mut f64, length: c_ulong);
}
extern "C" {
    pub fn bartlett(window: *mut f64, length: c_ulong);
}
extern "C" {
    pub fn apply_window(data: *mut f64, window: *mut f64, length: c_ulong);
}
extern "C" {
    pub fn rfft(x: *mut f64, N: c_long, forward: c_uint);
}
extern "C" {
    pub fn cfft(x: *mut f64, NC: c_long, forward: c_uint);
}
extern "C" {
    pub fn the_dct(x: *mut f64, N: c_ulong, out: *mut f64, Nout: c_ulong);
}
extern "C" {
    pub fn the_dct_matrix(out: *mut f64, N: c_ulong);
}
extern "C" {
    pub fn the_inverse_dct(x: *mut f64, N: c_ulong, out: *mut f64, Nout: c_ulong);
}
extern "C" {
    pub fn the_inverse_dct_matrix(out: *mut f64, N: c_ulong);
}
extern "C" {
    pub fn the_dct_now(
        x: *mut f64,
        matrix: *mut f64,
        N: c_ulong,
        out: *mut f64,
        Nout: c_ulong,
    );
}
extern "C" {
    pub fn the_inverse_dct_now(
        x: *mut f64,
        matrix: *mut f64,
        N: c_ulong,
        out: *mut f64,
        Nout: c_ulong,
    );
}
