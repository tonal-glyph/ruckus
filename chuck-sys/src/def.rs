//! basic ChucK types from `src/core/chuck_def.h`
use libc;

// types
/// f64
pub type t_CKTIME = f64;
/// f64
pub type t_CKDUR = f64;
/// f64
pub type t_CKFLOAT = f64;
/// f64
pub type t_CKDOUBLE = f64;
/// f32
pub type t_CKSINGLE = f32;
/// f64
pub type t_CKINT = f64;
/// u64
pub type t_CKUINT = u64;
/// bool
pub type t_CKBOOL = bool;
/// u8
pub type t_CKBYTE = u8;
/// c_void
pub type t_CKVOID = libc::c_void;
/// *mut c_void
pub type t_CKVOIDPTR = *mut libc::c_void;
/// f32
pub type SAMPLE = f32;
/// f32
pub type SILENCE = f32;
/// *mut c_char
pub type c_str = *mut libc::c_char;
/// const c_char
pub const c_constr: libc::c_char = 0;
/// π
pub const ONE_PI: f64 = 3.14159265358979323846;
/// 2π
pub const TWO_PI: f64 = 2.0 * ONE_PI;
/// square root
pub const SQRT2: f64 = 1.41421356237309504880;

// bool
/// one
pub const TRUE: u8 = 1;
/// zero
pub const FALSE: u8 = 0;

// complex types
/// f64,f64
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX {
    pub re: f64,
    pub im: f64,
}
/// f64,f64
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX_SAMPLE {
    pub re: f64,
    pub im: f64,
}
// polar type
/// f64,f64
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKPOLAR {
    pub modulus: f64,
    pub phase: f64,
}
// vector types
/// f64,f64,f64
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
/// f64,f64,f64,f64
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
/// c_ulong,f64
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVECTOR {
    pub N: libc::c_ulong,
    pub values: f64,
}
