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
extern crate libc;
use libc::*;
use crate::dts::*;
///* Rust equivalent is f32
pub type t_CKTIME = c_float;
pub type t_CKDUR = f64;
pub type t_CKFLOAT = f64;
pub type t_CKDOUBLE = f64;
pub type t_CKSINGLE = f32;
//TODO Using 64-bit signed int, check @ compile time
pub type t_CKINT = i64;
pub type t_CKUINT = u64;
pub type t_CKBOOL = u64;
///* Should this be empty enum Void, (), or c_void?
pub type t_CKVOID = ();
pub type nullptr_t = *const c_void;
pub type t_CKVOIDPTR = *mut nullptr_t;
pub type t_CKBYTE = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t_CKCOMPLEX {
    pub re: t_CKFLOAT,
    pub im: t_CKFLOAT,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t_CKPOLAR {
    pub modulus: t_CKFLOAT,
    pub phase: t_CKFLOAT,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t_CKVEC3 {
    pub x: t_CKFLOAT,
    pub y: t_CKFLOAT,
    pub z: t_CKFLOAT,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t_CKVEC4 {
    pub x: t_CKFLOAT,
    pub y: t_CKFLOAT,
    pub z: t_CKFLOAT,
    pub w: t_CKFLOAT,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t_CKVECTOR {
    pub N: t_CKUINT,
    pub values: *mut t_CKFLOAT,
}
pub const sz_TIME: t_CKTIME = 0.0;
pub const sz_DUR: t_CKDUR = 0.0;
pub const sz_FLOAT: t_CKFLOAT = 0.0;
pub const sz_DOUBLE: t_CKDOUBLE = 0.0;
pub const sz_SINGLE: t_CKSINGLE = 0.0;
pub const sz_INT: t_CKUINT = 0;
pub const sz_UINT: t_CKUINT = 0;
pub const sz_BOOL: t_CKBOOL = 0;
pub const sz_BYTE: t_CKBYTE = 0;
pub type sz_VOIDPTR = c_void;
pub type sz_COMPLEX = f64;
pub type sz_POLAR = t_CKPOLAR;
pub type sz_VEC3 = t_CKVEC3;
pub type sz_VEC4 = t_CKVEC4;
pub type sz_VECTOR = t_CKVECTOR;
pub const sz_VOID: u32 = 0;
pub const sz_WORD: u32 = 4;
pub const kindof_VOID: u32 = 0;
pub const kindof_INT: u32 = 1;
pub const kindof_FLOAT: u32 = 2;
pub const kindof_COMPLEX: u32 = 3;
pub const kindof_VEC3: u32 = 4;
pub const kindof_VEC4: u32 = 5;
pub const SAMPLE: f64 = 0.0;
pub type t_CKSAMPLE = SAMPLE;
pub const SILENCE: f64 = 0.0;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const ONE_PI: f64 = 3.141592653589793;
pub const TWO_PI: f64 = 6.283185307179586;
pub const SQRT2: f64 = 1.4142135623730951;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t_CKCOMPLEX_SAMPLE {
    pub re: SAMPLE,
    pub im: SAMPLE,
}