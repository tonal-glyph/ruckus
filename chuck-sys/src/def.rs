//! basic ChucK types from `chuck/src/core/chuck_def.h`
use libc::{
    c_char,
    c_double,
    c_float,
    c_long,
    c_ulong,
    c_void
};

// UDT
/// c_double
pub type t_CKTIME = c_double;
/// c_double
pub type t_CKDUR = c_double;
/// c_double
pub type t_CKFLOAT = c_double;
/// c_double
pub type t_CKDOUBLE = c_double;
/// c_float
pub type t_CKSINGLE = c_float;
/// c_double
pub type t_CKINT = c_long;
/// c_ulong
pub type t_CKUINT = c_ulong;
/// bool
pub type t_CKBOOL = bool;
/// c_char
pub type t_CKBYTE = c_char;
/// c_void
pub type t_CKVOID = c_void;
/// *mut c_void
pub type t_CKVOIDPTR = *mut c_void;
/// c_float
pub type SAMPLE = c_float;
/// c_float
pub type SILENCE = c_float;
/// *mut c_char
pub type c_str = *mut c_char;
/// *const c_char
pub type c_constr = *const c_char;
/// π
pub const ONE_PI: f64 = ::std::f64::consts::PI; // use native
/// 2π
pub const TWO_PI: f64 = 2.0 * ONE_PI;
/// square root
pub const SQRT2: f64 = 1.41421356237309504880;

// bool
/// one
pub const TRUE: u8 = 1;
/// zero
pub const FALSE: u8 = 0;
pub type NULL = ();
pub type t_CKSAMPLE = SAMPLE;

// complex UDT
/// c_double,c_double
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX {
    pub re: c_double,
    pub im: c_double,
}
/// c_double,c_double
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX_SAMPLE {
    pub re: c_double,
    pub im: c_double,
}
// polar type
/// c_double,c_double
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKPOLAR {
    pub modulus: c_double,
    pub phase: c_double,
}
// vector types
/// c_double,c_double,c_double
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC3 {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}
/// c_double,c_double,c_double,c_double
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC4 {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}
/// c_ulong,c_double
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVECTOR {
    pub N: c_ulong,
    pub values: c_double,
}
