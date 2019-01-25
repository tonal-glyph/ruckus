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
use crate::chuck_dl_h_edited::*;
///* class library for `Math`
use libc::*;
pub fn DLLQUERY() {
    libmath_query(QUERY: *mut Chuck_DL_Query);
}
pub fn main() {
    // impl
    CK_DLL_SFUN(sin_impl);
    CK_DLL_SFUN(cos_impl);
    CK_DLL_SFUN(tan_impl);
    CK_DLL_SFUN(cot_impl);
    CK_DLL_SFUN(asin_impl);
    CK_DLL_SFUN(acos_impl);
    CK_DLL_SFUN(atan_impl);
    CK_DLL_SFUN(atan2_impl);
    CK_DLL_SFUN(sinh_impl);
    CK_DLL_SFUN(cosh_impl);
    CK_DLL_SFUN(tanh_impl);
    CK_DLL_SFUN(hypot_impl);
    CK_DLL_SFUN(pow_impl);
    CK_DLL_SFUN(sqrt_impl);
    CK_DLL_SFUN(exp_impl);
    CK_DLL_SFUN(log_impl);
    CK_DLL_SFUN(log2_impl);
    CK_DLL_SFUN(log10_impl);
    CK_DLL_SFUN(floor_impl);
    CK_DLL_SFUN(ceil_impl);
    CK_DLL_SFUN(round_impl);
    CK_DLL_SFUN(trunc_impl);
    CK_DLL_SFUN(fmod_impl);
    CK_DLL_SFUN(remainder_impl);
    CK_DLL_SFUN(min_impl);
    CK_DLL_SFUN(max_impl);
    CK_DLL_SFUN(isinf_impl);
    CK_DLL_SFUN(isnan_impl);
    CK_DLL_SFUN(floatMax_impl);
    CK_DLL_SFUN(intMax_impl);
    CK_DLL_SFUN(nextpow2_impl);
    CK_DLL_SFUN(ensurepow2_impl);
    CK_DLL_SFUN(re_impl);
    CK_DLL_SFUN(im_impl);
    CK_DLL_SFUN(modulus_impl);
    CK_DLL_SFUN(phase_impl);
    CK_DLL_SFUN(rtop_impl);
    CK_DLL_SFUN(ptor_impl);
    CK_DLL_SFUN(random_impl);
    CK_DLL_SFUN(randomf_impl);
    CK_DLL_SFUN(random2f_impl);
    CK_DLL_SFUN(random2_impl);
    CK_DLL_SFUN(srandom_impl);
    CK_DLL_SFUN(gauss_impl);
    // max for random functions
    pub const CK_RANDOM_MAX: u64 = 0x7fffffff;
}
