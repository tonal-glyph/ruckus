#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX {
    pub re: f64,
    pub im: f64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKPOLAR {
    pub modulus: f64,
    pub phase: f64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct t_CKVECTOR {
    pub N: ::std::os::raw::c_ulong,
    pub values: *mut f64,
}
impl Default for t_CKVECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type t_CKTIME = *mut f64;
pub type t_CKDUR = *mut f64;
pub type t_CKFLOAT = *mut f64;
pub type t_CKDOUBLE = *mut f64;
pub type t_CKSINGLE = *mut f32;
pub type t_CKINT = *mut f64;
pub type t_CKUINT = *mut u64;
pub type t_CKBOOL = *mut bool;
pub type t_CKBYTE = *mut u8;
pub type t_CKVOID = ::std::os::raw::c_void;
pub type t_CKVOIDPTR = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX_SAMPLE {
    pub re: f64,
    pub im: f64,
}
