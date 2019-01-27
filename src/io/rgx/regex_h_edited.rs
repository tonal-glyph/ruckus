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
use crate::regex::tre_h_edited::*;
use libc::*;
//* tre regex header
pub const TRE_REGEX_H: u32 = 1;
pub const TRE_H: u32 = 1;
pub const HAVE_ALLOCA: u32 = 1;
pub const HAVE_ALLOCA_H: u32 = 1;
pub const HAVE_SYS_TYPES_H: u32 = 1;
pub const HAVE_WCHAR_H: u32 = 1;
pub const TRE_APPROX: u32 = 1;
pub const TRE_USE_ALLOCA: u32 = 1;
pub const TRE_VERSION: &'static [u8; 14usize] = b"@TRE_VERSION@\0";
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
pub const REG_EXTENDED: u32 = 1;
pub const REG_ICASE: u32 = 2;
pub const REG_NEWLINE: u32 = 4;
pub const REG_NOSUB: u32 = 8;
pub const REG_BASIC: u32 = 0;
pub const REG_LITERAL: u32 = 16;
pub const REG_RIGHT_ASSOC: u32 = 32;
pub const REG_UNGREEDY: u32 = 64;
pub const REG_NOTBOL: u32 = 1;
pub const REG_NOTEOL: u32 = 2;
pub const REG_APPROX_MATCHER: u32 = 4;
pub const REG_BACKTRACKING_MATCHER: u32 = 8;
pub const REG_NOSPEC: u32 = 16;
pub const RE_DUP_MAX: u32 = 255;
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
pub type regoff_t = c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct regex_t {
    pub re_nsub: usize,
    pub value: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub const reg_errcode_t_REG_OK: reg_errcode_t = 0;
pub const reg_errcode_t_REG_NOMATCH: reg_errcode_t = 1;
pub const reg_errcode_t_REG_BADPAT: reg_errcode_t = 2;
pub const reg_errcode_t_REG_ECOLLATE: reg_errcode_t = 3;
pub const reg_errcode_t_REG_ECTYPE: reg_errcode_t = 4;
pub const reg_errcode_t_REG_EESCAPE: reg_errcode_t = 5;
pub const reg_errcode_t_REG_ESUBREG: reg_errcode_t = 6;
pub const reg_errcode_t_REG_EBRACK: reg_errcode_t = 7;
pub const reg_errcode_t_REG_EPAREN: reg_errcode_t = 8;
pub const reg_errcode_t_REG_EBRACE: reg_errcode_t = 9;
pub const reg_errcode_t_REG_BADBR: reg_errcode_t = 10;
pub const reg_errcode_t_REG_ERANGE: reg_errcode_t = 11;
pub const reg_errcode_t_REG_ESPACE: reg_errcode_t = 12;
pub const reg_errcode_t_REG_BADRPT: reg_errcode_t = 13;
pub type reg_errcode_t = u32;
extern "C" {
    pub fn tre_regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
}
extern "C" {
    pub fn tre_regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn tre_regerror(
        errcode: c_int,
        preg: *const regex_t,
        errbuf: *mut c_char,
        errbuf_size: usize,
    ) -> usize;
}
extern "C" {
    pub fn tre_regfree(preg: *mut regex_t);
}
extern "C" {
    pub fn tre_regncomp(
        preg: *mut regex_t,
        regex: *const c_char,
        len: usize,
        cflags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn tre_regnexec(
        preg: *const regex_t,
        string: *const c_char,
        len: usize,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct regaparams_t {
    pub cost_ins: c_int,
    pub cost_del: c_int,
    pub cost_subst: c_int,
    pub max_cost: c_int,
    pub max_ins: c_int,
    pub max_del: c_int,
    pub max_subst: c_int,
    pub max_err: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct regamatch_t {
    pub nmatch: usize,
    pub pmatch: *mut regmatch_t,
    pub cost: c_int,
    pub num_ins: c_int,
    pub num_del: c_int,
    pub num_subst: c_int,
}
extern "C" {
    pub fn tre_regaexec(
        preg: *const regex_t,
        string: *const c_char,
        match_: *mut regamatch_t,
        params: regaparams_t,
        eflags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn tre_reganexec(
        preg: *const regex_t,
        string: *const c_char,
        len: usize,
        match_: *mut regamatch_t,
        params: regaparams_t,
        eflags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn tre_regaparams_default(params: *mut regaparams_t);
}
pub type tre_char_t = c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_str_source {
    pub get_next_char: Option<
        unsafe extern "C" fn(
            c: *mut tre_char_t,
            pos_add: *mut c_uint,
            context: *mut c_void,
        ) -> c_int,
    >,
    pub rewind: Option<unsafe extern "C" fn(pos: usize, context: *mut c_void)>,
    pub compare: Option<
        unsafe extern "C" fn(pos1: usize, pos2: usize, len: usize, context: *mut c_void) -> c_int,
    >,
    pub context: *mut c_void,
}
extern "C" {
    pub fn tre_reguexec(
        preg: *const regex_t,
        string: *const tre_str_source,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn tre_version() -> *mut c_char;
}
extern "C" {
    pub fn tre_config(query: c_int, result: *mut c_void) -> c_int;
}
pub const TRE_CONFIG_APPROX: _bindgen_ty_1 = 0;
pub const TRE_CONFIG_WCHAR: _bindgen_ty_1 = 1;
pub const TRE_CONFIG_MULTIBYTE: _bindgen_ty_1 = 2;
pub const TRE_CONFIG_SYSTEM_ABI: _bindgen_ty_1 = 3;
pub const TRE_CONFIG_VERSION: _bindgen_ty_1 = 4;
pub type _bindgen_ty_1 = u32;
extern "C" {
    pub fn tre_have_backrefs(preg: *const regex_t) -> c_int;
}
extern "C" {
    pub fn tre_have_approx(preg: *const regex_t) -> c_int;
}
