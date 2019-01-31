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
///* a mini-compatibility library for math functions.
#![feature(libc)]
use libc::*;
use crate::ck::def::defe::*;
use crate::sys::*;
pub type __u_char = c_uchar;
pub type __u_short = c_ushort;
pub type __u_int = c_uint;
pub type __u_long = c_ulong;
pub type __int8_t = c_schar;
pub type __uint8_t = c_uchar;
pub type __int16_t = c_short;
pub type __uint16_t = c_ushort;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type __int64_t = c_long;
pub type __uint64_t = c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = c_long;
pub type __u_quad_t = c_ulong;
pub type __intmax_t = c_long;
pub type __uintmax_t = c_ulong;
pub type __dev_t = c_ulong;
pub type __uid_t = c_uint;
pub type __gid_t = c_uint;
pub type __ino_t = c_ulong;
pub type __ino64_t = c_ulong;
pub type __mode_t = c_uint;
pub type __nlink_t = c_ulong;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type __pid_t = c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [c_int; 2usize],
}
pub type __clock_t = c_long;
pub type __rlim_t = c_ulong;
pub type __rlim64_t = c_ulong;
pub type __id_t = c_uint;
pub type __time_t = c_long;
pub type __useconds_t = c_uint;
pub type __suseconds_t = c_long;
pub type __daddr_t = c_int;
pub type __key_t = c_int;
pub type __clockid_t = c_int;
pub type __timer_t = *mut c_void;
pub type __blksize_t = c_long;
pub type __blkcnt_t = c_long;
pub type __blkcnt64_t = c_long;
pub type __fsblkcnt_t = c_ulong;
pub type __fsblkcnt64_t = c_ulong;
pub type __fsfilcnt_t = c_ulong;
pub type __fsfilcnt64_t = c_ulong;
pub type __fsword_t = c_long;
pub type __ssize_t = c_long;
pub type __syscall_slong_t = c_long;
pub type __syscall_ulong_t = c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut c_char;
pub type __intptr_t = c_long;
pub type __socklen_t = c_uint;
pub type __sig_atomic_t = c_int;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
pub type float_t = f32;
pub type double_t = f64;
extern "C" {
    pub fn __fpclassify(__value: f64) -> c_int;
}
extern "C" {
    pub fn __signbit(__value: f64) -> c_int;
}
extern "C" {
    pub fn __isinf(__value: f64) -> c_int;
}
extern "C" {
    pub fn __finite(__value: f64) -> c_int;
}
extern "C" {
    pub fn __isnan(__value: f64) -> c_int;
}
extern "C" {
    pub fn __iseqsig(__x: f64, __y: f64) -> c_int;
}
extern "C" {
    pub fn __issignaling(__value: f64) -> c_int;
}
extern "C" {
    pub fn acos(__x: f64) -> f64;
}
extern "C" {
    pub fn __acos(__x: f64) -> f64;
}
extern "C" {
    pub fn asin(__x: f64) -> f64;
}
extern "C" {
    pub fn __asin(__x: f64) -> f64;
}
extern "C" {
    pub fn atan(__x: f64) -> f64;
}
extern "C" {
    pub fn __atan(__x: f64) -> f64;
}
extern "C" {
    pub fn atan2(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn __atan2(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn cos(__x: f64) -> f64;
}
extern "C" {
    pub fn __cos(__x: f64) -> f64;
}
extern "C" {
    pub fn sin(__x: f64) -> f64;
}
extern "C" {
    pub fn __sin(__x: f64) -> f64;
}
extern "C" {
    pub fn tan(__x: f64) -> f64;
}
extern "C" {
    pub fn __tan(__x: f64) -> f64;
}
extern "C" {
    pub fn cosh(__x: f64) -> f64;
}
extern "C" {
    pub fn __cosh(__x: f64) -> f64;
}
extern "C" {
    pub fn sinh(__x: f64) -> f64;
}
extern "C" {
    pub fn __sinh(__x: f64) -> f64;
}
extern "C" {
    pub fn tanh(__x: f64) -> f64;
}
extern "C" {
    pub fn __tanh(__x: f64) -> f64;
}
extern "C" {
    pub fn acosh(__x: f64) -> f64;
}
extern "C" {
    pub fn __acosh(__x: f64) -> f64;
}
extern "C" {
    pub fn asinh(__x: f64) -> f64;
}
extern "C" {
    pub fn __asinh(__x: f64) -> f64;
}
extern "C" {
    pub fn atanh(__x: f64) -> f64;
}
extern "C" {
    pub fn __atanh(__x: f64) -> f64;
}
extern "C" {
    pub fn exp(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp(__x: f64) -> f64;
}
extern "C" {
    pub fn frexp(__x: f64, __exponent: *mut c_int) -> f64;
}
extern "C" {
    pub fn __frexp(__x: f64, __exponent: *mut c_int) -> f64;
}
extern "C" {
    pub fn ldexp(__x: f64, __exponent: c_int) -> f64;
}
extern "C" {
    pub fn __ldexp(__x: f64, __exponent: c_int) -> f64;
}
extern "C" {
    pub fn log(__x: f64) -> f64;
}
extern "C" {
    pub fn __log(__x: f64) -> f64;
}
extern "C" {
    pub fn log10(__x: f64) -> f64;
}
extern "C" {
    pub fn __log10(__x: f64) -> f64;
}
extern "C" {
    pub fn modf(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn __modf(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn expm1(__x: f64) -> f64;
}
extern "C" {
    pub fn __expm1(__x: f64) -> f64;
}
extern "C" {
    pub fn log1p(__x: f64) -> f64;
}
extern "C" {
    pub fn __log1p(__x: f64) -> f64;
}
extern "C" {
    pub fn logb(__x: f64) -> f64;
}
extern "C" {
    pub fn __logb(__x: f64) -> f64;
}
extern "C" {
    pub fn exp2(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp2(__x: f64) -> f64;
}
extern "C" {
    pub fn log2(__x: f64) -> f64;
}
extern "C" {
    pub fn __log2(__x: f64) -> f64;
}
extern "C" {
    pub fn pow(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __pow(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn sqrt(__x: f64) -> f64;
}
extern "C" {
    pub fn __sqrt(__x: f64) -> f64;
}
extern "C" {
    pub fn hypot(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __hypot(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn cbrt(__x: f64) -> f64;
}
extern "C" {
    pub fn __cbrt(__x: f64) -> f64;
}
extern "C" {
    pub fn ceil(__x: f64) -> f64;
}
extern "C" {
    pub fn __ceil(__x: f64) -> f64;
}
extern "C" {
    pub fn fabs(__x: f64) -> f64;
}
extern "C" {
    pub fn __fabs(__x: f64) -> f64;
}
extern "C" {
    pub fn floor(__x: f64) -> f64;
}
extern "C" {
    pub fn __floor(__x: f64) -> f64;
}
extern "C" {
    pub fn fmod(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmod(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn copysign(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __copysign(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nan(__tagb: *const c_char) -> f64;
}
extern "C" {
    pub fn __nan(__tagb: *const c_char) -> f64;
}
extern "C" {
    pub fn erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn rint(__x: f64) -> f64;
}
extern "C" {
    pub fn __rint(__x: f64) -> f64;
}
extern "C" {
    pub fn nextafter(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nextafter(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nexttoward(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nexttoward(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn remainder(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __remainder(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn scalbn(__x: f64, __n: c_int) -> f64;
}
extern "C" {
    pub fn __scalbn(__x: f64, __n: c_int) -> f64;
}
extern "C" {
    pub fn ilogb(__x: f64) -> c_int;
}
extern "C" {
    pub fn __ilogb(__x: f64) -> c_int;
}
extern "C" {
    pub fn scalbln(__x: f64, __n: c_long) -> f64;
}
extern "C" {
    pub fn __scalbln(__x: f64, __n: c_long) -> f64;
}
extern "C" {
    pub fn nearbyint(__x: f64) -> f64;
}
extern "C" {
    pub fn __nearbyint(__x: f64) -> f64;
}
extern "C" {
    pub fn round(__x: f64) -> f64;
}
extern "C" {
    pub fn __round(__x: f64) -> f64;
}
extern "C" {
    pub fn trunc(__x: f64) -> f64;
}
extern "C" {
    pub fn __trunc(__x: f64) -> f64;
}
extern "C" {
    pub fn remquo(__x: f64, __y: f64, __quo: *mut c_int) -> f64;
}
extern "C" {
    pub fn __remquo(__x: f64, __y: f64, __quo: *mut c_int) -> f64;
}
extern "C" {
    pub fn lrint(__x: f64) -> c_long;
}
extern "C" {
    pub fn __lrint(__x: f64) -> c_long;
}
extern "C" {
    pub fn llrint(__x: f64) -> c_longlong;
}
extern "C" {
    pub fn __llrint(__x: f64) -> c_longlong;
}
extern "C" {
    pub fn lround(__x: f64) -> c_long;
}
extern "C" {
    pub fn __lround(__x: f64) -> c_long;
}
extern "C" {
    pub fn llround(__x: f64) -> c_longlong;
}
extern "C" {
    pub fn __llround(__x: f64) -> c_longlong;
}
extern "C" {
    pub fn fdim(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fdim(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmax(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmax(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmin(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmin(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fma(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn __fma(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn __fpclassifyf(__value: f32) -> c_int;
}
extern "C" {
    pub fn __signbitf(__value: f32) -> c_int;
}
extern "C" {
    pub fn __isinff(__value: f32) -> c_int;
}
extern "C" {
    pub fn __finitef(__value: f32) -> c_int;
}
extern "C" {
    pub fn __isnanf(__value: f32) -> c_int;
}
extern "C" {
    pub fn __iseqsigf(__x: f32, __y: f32) -> c_int;
}
extern "C" {
    pub fn __issignalingf(__value: f32) -> c_int;
}
extern "C" {
    pub fn acosf(__x: f32) -> f32;
}
extern "C" {
    pub fn __acosf(__x: f32) -> f32;
}
extern "C" {
    pub fn asinf(__x: f32) -> f32;
}
extern "C" {
    pub fn __asinf(__x: f32) -> f32;
}
extern "C" {
    pub fn atanf(__x: f32) -> f32;
}
extern "C" {
    pub fn __atanf(__x: f32) -> f32;
}
extern "C" {
    pub fn atan2f(__y: f32, __x: f32) -> f32;
}
extern "C" {
    pub fn __atan2f(__y: f32, __x: f32) -> f32;
}
extern "C" {
    pub fn cosf(__x: f32) -> f32;
}
extern "C" {
    pub fn __cosf(__x: f32) -> f32;
}
extern "C" {
    pub fn sinf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sinf(__x: f32) -> f32;
}
extern "C" {
    pub fn tanf(__x: f32) -> f32;
}
extern "C" {
    pub fn __tanf(__x: f32) -> f32;
}
extern "C" {
    pub fn coshf(__x: f32) -> f32;
}
extern "C" {
    pub fn __coshf(__x: f32) -> f32;
}
extern "C" {
    pub fn sinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn tanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __tanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn acoshf(__x: f32) -> f32;
}
extern "C" {
    pub fn __acoshf(__x: f32) -> f32;
}
extern "C" {
    pub fn asinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __asinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn atanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __atanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn expf(__x: f32) -> f32;
}
extern "C" {
    pub fn __expf(__x: f32) -> f32;
}
extern "C" {
    pub fn frexpf(__x: f32, __exponent: *mut c_int) -> f32;
}
extern "C" {
    pub fn __frexpf(__x: f32, __exponent: *mut c_int) -> f32;
}
extern "C" {
    pub fn ldexpf(__x: f32, __exponent: c_int) -> f32;
}
extern "C" {
    pub fn __ldexpf(__x: f32, __exponent: c_int) -> f32;
}
extern "C" {
    pub fn logf(__x: f32) -> f32;
}
extern "C" {
    pub fn __logf(__x: f32) -> f32;
}
extern "C" {
    pub fn log10f(__x: f32) -> f32;
}
extern "C" {
    pub fn __log10f(__x: f32) -> f32;
}
extern "C" {
    pub fn modff(__x: f32, __iptr: *mut f32) -> f32;
}
extern "C" {
    pub fn __modff(__x: f32, __iptr: *mut f32) -> f32;
}
extern "C" {
    pub fn expm1f(__x: f32) -> f32;
}
extern "C" {
    pub fn __expm1f(__x: f32) -> f32;
}
extern "C" {
    pub fn log1pf(__x: f32) -> f32;
}
extern "C" {
    pub fn __log1pf(__x: f32) -> f32;
}
extern "C" {
    pub fn logbf(__x: f32) -> f32;
}
extern "C" {
    pub fn __logbf(__x: f32) -> f32;
}
extern "C" {
    pub fn exp2f(__x: f32) -> f32;
}
extern "C" {
    pub fn __exp2f(__x: f32) -> f32;
}
extern "C" {
    pub fn log2f(__x: f32) -> f32;
}
extern "C" {
    pub fn __log2f(__x: f32) -> f32;
}
extern "C" {
    pub fn powf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __powf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn sqrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sqrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn hypotf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __hypotf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn cbrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn __cbrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn ceilf(__x: f32) -> f32;
}
extern "C" {
    pub fn __ceilf(__x: f32) -> f32;
}
extern "C" {
    pub fn fabsf(__x: f32) -> f32;
}
extern "C" {
    pub fn __fabsf(__x: f32) -> f32;
}
extern "C" {
    pub fn floorf(__x: f32) -> f32;
}
extern "C" {
    pub fn __floorf(__x: f32) -> f32;
}
extern "C" {
    pub fn fmodf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmodf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn copysignf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __copysignf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn nanf(__tagb: *const c_char) -> f32;
}
extern "C" {
    pub fn __nanf(__tagb: *const c_char) -> f32;
}
extern "C" {
    pub fn erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn __erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn rintf(__x: f32) -> f32;
}
extern "C" {
    pub fn __rintf(__x: f32) -> f32;
}
extern "C" {
    pub fn nextafterf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __nextafterf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn nexttowardf(__x: f32, __y: f64) -> f32;
}
extern "C" {
    pub fn __nexttowardf(__x: f32, __y: f64) -> f32;
}
extern "C" {
    pub fn remainderf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __remainderf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn scalbnf(__x: f32, __n: c_int) -> f32;
}
extern "C" {
    pub fn __scalbnf(__x: f32, __n: c_int) -> f32;
}
extern "C" {
    pub fn ilogbf(__x: f32) -> c_int;
}
extern "C" {
    pub fn __ilogbf(__x: f32) -> c_int;
}
extern "C" {
    pub fn scalblnf(__x: f32, __n: c_long) -> f32;
}
extern "C" {
    pub fn __scalblnf(__x: f32, __n: c_long) -> f32;
}
extern "C" {
    pub fn nearbyintf(__x: f32) -> f32;
}
extern "C" {
    pub fn __nearbyintf(__x: f32) -> f32;
}
extern "C" {
    pub fn roundf(__x: f32) -> f32;
}
extern "C" {
    pub fn __roundf(__x: f32) -> f32;
}
extern "C" {
    pub fn truncf(__x: f32) -> f32;
}
extern "C" {
    pub fn __truncf(__x: f32) -> f32;
}
extern "C" {
    pub fn remquof(__x: f32, __y: f32, __quo: *mut c_int) -> f32;
}
extern "C" {
    pub fn __remquof(__x: f32, __y: f32, __quo: *mut c_int) -> f32;
}
extern "C" {
    pub fn lrintf(__x: f32) -> c_long;
}
extern "C" {
    pub fn __lrintf(__x: f32) -> c_long;
}
extern "C" {
    pub fn llrintf(__x: f32) -> c_longlong;
}
extern "C" {
    pub fn __llrintf(__x: f32) -> c_longlong;
}
extern "C" {
    pub fn lroundf(__x: f32) -> c_long;
}
extern "C" {
    pub fn __lroundf(__x: f32) -> c_long;
}
extern "C" {
    pub fn llroundf(__x: f32) -> c_longlong;
}
extern "C" {
    pub fn __llroundf(__x: f32) -> c_longlong;
}
extern "C" {
    pub fn fdimf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fdimf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fmaxf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmaxf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fminf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fminf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
extern "C" {
    pub fn __fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
extern "C" {
    pub fn __fpclassifyl(__value: f64) -> c_int;
}
extern "C" {
    pub fn __signbitl(__value: f64) -> c_int;
}
extern "C" {
    pub fn __isinfl(__value: f64) -> c_int;
}
extern "C" {
    pub fn __finitel(__value: f64) -> c_int;
}
extern "C" {
    pub fn __isnanl(__value: f64) -> c_int;
}
extern "C" {
    pub fn __iseqsigl(__x: f64, __y: f64) -> c_int;
}
extern "C" {
    pub fn __issignalingl(__value: f64) -> c_int;
}
extern "C" {
    pub fn acosl(__x: f64) -> f64;
}
extern "C" {
    pub fn __acosl(__x: f64) -> f64;
}
extern "C" {
    pub fn asinl(__x: f64) -> f64;
}
extern "C" {
    pub fn __asinl(__x: f64) -> f64;
}
extern "C" {
    pub fn atanl(__x: f64) -> f64;
}
extern "C" {
    pub fn __atanl(__x: f64) -> f64;
}
extern "C" {
    pub fn atan2l(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn __atan2l(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn cosl(__x: f64) -> f64;
}
extern "C" {
    pub fn __cosl(__x: f64) -> f64;
}
extern "C" {
    pub fn sinl(__x: f64) -> f64;
}
extern "C" {
    pub fn __sinl(__x: f64) -> f64;
}
extern "C" {
    pub fn tanl(__x: f64) -> f64;
}
extern "C" {
    pub fn __tanl(__x: f64) -> f64;
}
extern "C" {
    pub fn coshl(__x: f64) -> f64;
}
extern "C" {
    pub fn __coshl(__x: f64) -> f64;
}
extern "C" {
    pub fn sinhl(__x: f64) -> f64;
}
extern "C" {
    pub fn __sinhl(__x: f64) -> f64;
}
extern "C" {
    pub fn tanhl(__x: f64) -> f64;
}
extern "C" {
    pub fn __tanhl(__x: f64) -> f64;
}
extern "C" {
    pub fn acoshl(__x: f64) -> f64;
}
extern "C" {
    pub fn __acoshl(__x: f64) -> f64;
}
extern "C" {
    pub fn asinhl(__x: f64) -> f64;
}
extern "C" {
    pub fn __asinhl(__x: f64) -> f64;
}
extern "C" {
    pub fn atanhl(__x: f64) -> f64;
}
extern "C" {
    pub fn __atanhl(__x: f64) -> f64;
}
extern "C" {
    pub fn expl(__x: f64) -> f64;
}
extern "C" {
    pub fn __expl(__x: f64) -> f64;
}
extern "C" {
    pub fn frexpl(__x: f64, __exponent: *mut c_int) -> f64;
}
extern "C" {
    pub fn __frexpl(__x: f64, __exponent: *mut c_int) -> f64;
}
extern "C" {
    pub fn ldexpl(__x: f64, __exponent: c_int) -> f64;
}
extern "C" {
    pub fn __ldexpl(__x: f64, __exponent: c_int) -> f64;
}
extern "C" {
    pub fn logl(__x: f64) -> f64;
}
extern "C" {
    pub fn __logl(__x: f64) -> f64;
}
extern "C" {
    pub fn log10l(__x: f64) -> f64;
}
extern "C" {
    pub fn __log10l(__x: f64) -> f64;
}
extern "C" {
    pub fn modfl(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn __modfl(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn expm1l(__x: f64) -> f64;
}
extern "C" {
    pub fn __expm1l(__x: f64) -> f64;
}
extern "C" {
    pub fn log1pl(__x: f64) -> f64;
}
extern "C" {
    pub fn __log1pl(__x: f64) -> f64;
}
extern "C" {
    pub fn logbl(__x: f64) -> f64;
}
extern "C" {
    pub fn __logbl(__x: f64) -> f64;
}
extern "C" {
    pub fn exp2l(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp2l(__x: f64) -> f64;
}
extern "C" {
    pub fn log2l(__x: f64) -> f64;
}
extern "C" {
    pub fn __log2l(__x: f64) -> f64;
}
extern "C" {
    pub fn powl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __powl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn sqrtl(__x: f64) -> f64;
}
extern "C" {
    pub fn __sqrtl(__x: f64) -> f64;
}
extern "C" {
    pub fn hypotl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __hypotl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn cbrtl(__x: f64) -> f64;
}
extern "C" {
    pub fn __cbrtl(__x: f64) -> f64;
}
extern "C" {
    pub fn ceill(__x: f64) -> f64;
}
extern "C" {
    pub fn __ceill(__x: f64) -> f64;
}
extern "C" {
    pub fn fabsl(__x: f64) -> f64;
}
extern "C" {
    pub fn __fabsl(__x: f64) -> f64;
}
extern "C" {
    pub fn floorl(__x: f64) -> f64;
}
extern "C" {
    pub fn __floorl(__x: f64) -> f64;
}
extern "C" {
    pub fn fmodl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmodl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn copysignl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __copysignl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nanl(__tagb: *const c_char) -> f64;
}
extern "C" {
    pub fn __nanl(__tagb: *const c_char) -> f64;
}
extern "C" {
    pub fn erfl(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erfl(arg1: f64) -> f64;
}
extern "C" {
    pub fn erfcl(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erfcl(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn __lgammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn tgammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn __tgammal(arg1: f64) -> f64;
}
extern "C" {
    pub fn rintl(__x: f64) -> f64;
}
extern "C" {
    pub fn __rintl(__x: f64) -> f64;
}
extern "C" {
    pub fn nextafterl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nextafterl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nexttowardl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nexttowardl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn remainderl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __remainderl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn scalbnl(__x: f64, __n: c_int) -> f64;
}
extern "C" {
    pub fn __scalbnl(__x: f64, __n: c_int) -> f64;
}
extern "C" {
    pub fn ilogbl(__x: f64) -> c_int;
}
extern "C" {
    pub fn __ilogbl(__x: f64) -> c_int;
}
extern "C" {
    pub fn scalblnl(__x: f64, __n: c_long) -> f64;
}
extern "C" {
    pub fn __scalblnl(__x: f64, __n: c_long) -> f64;
}
extern "C" {
    pub fn nearbyintl(__x: f64) -> f64;
}
extern "C" {
    pub fn __nearbyintl(__x: f64) -> f64;
}
extern "C" {
    pub fn roundl(__x: f64) -> f64;
}
extern "C" {
    pub fn __roundl(__x: f64) -> f64;
}
extern "C" {
    pub fn truncl(__x: f64) -> f64;
}
extern "C" {
    pub fn __truncl(__x: f64) -> f64;
}
extern "C" {
    pub fn remquol(__x: f64, __y: f64, __quo: *mut c_int) -> f64;
}
extern "C" {
    pub fn __remquol(__x: f64, __y: f64, __quo: *mut c_int) -> f64;
}
extern "C" {
    pub fn lrintl(__x: f64) -> c_long;
}
extern "C" {
    pub fn __lrintl(__x: f64) -> c_long;
}
extern "C" {
    pub fn llrintl(__x: f64) -> c_longlong;
}
extern "C" {
    pub fn __llrintl(__x: f64) -> c_longlong;
}
extern "C" {
    pub fn lroundl(__x: f64) -> c_long;
}
extern "C" {
    pub fn __lroundl(__x: f64) -> c_long;
}
extern "C" {
    pub fn llroundl(__x: f64) -> c_longlong;
}
extern "C" {
    pub fn __llroundl(__x: f64) -> c_longlong;
}
extern "C" {
    pub fn fdiml(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fdiml(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmaxl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmaxl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fminl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fminl(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmal(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn __fmal(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn mtof(f: f64) -> f64;
}
extern "C" {
    pub fn ftom(f: f64) -> f64;
}
extern "C" {
    pub fn powtodb(f: f64) -> f64;
}
extern "C" {
    pub fn rmstodb(f: f64) -> f64;
}
extern "C" {
    pub fn dbtopow(f: f64) -> f64;
}
extern "C" {
    pub fn dbtorms(f: f64) -> f64;
}
extern "C" {
    pub fn nextpow2(i: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn ensurepow2(i: c_ulong) -> c_ulong;
}
