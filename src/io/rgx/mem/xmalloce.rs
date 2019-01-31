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
///* regex malloc
use libc::*;
pub const _XMALLOC_H: u32 = 1;
pub const _SYS_TYPES_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 28;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __USE_EXTERN_INLINES: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _STDLIB_H: u32 = 1;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type u_int8_t = c_uchar;
pub type u_int16_t = c_ushort;
pub type u_int32_t = c_uint;
pub type u_int64_t = c_ulong;
pub type register_t = c_long;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
extern "C" {
    pub fn xmalloc_impl(
        size: usize,
        file: *const c_char,
        line: c_int,
        func: *const c_char,
    ) -> *mut c_void;
}
extern "C" {
    pub fn xcalloc_impl(
        nmemb: usize,
        size: usize,
        file: *const c_char,
        line: c_int,
        func: *const c_char,
    ) -> *mut c_void;
}
extern "C" {
    pub fn xfree_impl(ptr: *mut c_void, file: *const c_char, line: c_int, func: *const c_char);
}
extern "C" {
    pub fn xrealloc_impl(
        ptr: *mut c_void,
        new_size: usize,
        file: *const c_char,
        line: c_int,
        func: *const c_char,
    ) -> *mut c_void;
}
extern "C" {
    pub fn xmalloc_dump_leaks() -> c_int;
}
extern "C" {
    pub fn xmalloc_configure(fail_after: c_int);
}
pub type wchar_t = c_int;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: c_int,
    pub rem: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: c_long,
    pub rem: c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: c_longlong,
    pub rem: c_longlong,
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const c_char) -> c_int;
}
extern "C" {
    pub fn atol(__nptr: *const c_char) -> c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const c_char) -> c_longlong;
}
extern "C" {
    pub fn strtod(__nptr: *const c_char, __endptr: *mut c_char) -> f64;
}
extern "C" {
    pub fn strtof(__nptr: *const c_char, __endptr: *mut c_char) -> f32;
}
extern "C" {
    pub fn strtold(__nptr: *const c_char, __endptr: *mut c_char) -> f64;
}
extern "C" {
    pub fn strtol(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_long;
}
extern "C" {
    pub fn strtoul(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_ulong;
}
extern "C" {
    pub fn strtoll(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __base: c_int,
    ) -> c_ulonglong;
}
extern "C" {
    pub fn rand() -> c_int;
}
extern "C" {
    pub fn srand(__seed: c_uint);
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn realloc(__ptr: *mut c_void, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn free(__ptr: *mut c_void);
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: Option<unsafe extern "C" fn()>) -> c_int;
}
extern "C" {
    pub fn at_quick_exit(__func: Option<unsafe extern "C" fn()>) -> c_int;
}
extern "C" {
    pub fn exit(__status: c_int);
}
extern "C" {
    pub fn quick_exit(__status: c_int);
}
extern "C" {
    pub fn _Exit(__status: c_int);
}
extern "C" {
    pub fn getenv(__name: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn system(__command: *const c_char) -> c_int;
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void) -> c_int>;
extern "C" {
    pub fn bsearch(
        __key: *const c_void,
        __base: *const c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn qsort(__base: *mut c_void, __nmemb: usize, __size: usize, __compar: __compar_fn_t);
}
extern "C" {
    pub fn abs(__x: c_int) -> c_int;
}
extern "C" {
    pub fn labs(__x: c_long) -> c_long;
}
extern "C" {
    pub fn llabs(__x: c_longlong) -> c_longlong;
}
extern "C" {
    pub fn div(__numer: c_int, __denom: c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: c_long, __denom: c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(__numer: c_longlong, __denom: c_longlong) -> lldiv_t;
}
extern "C" {
    pub fn mblen(__s: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn mbtowc(__pwc: *mut wchar_t, __s: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut c_char, __wchar: wchar_t) -> c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
