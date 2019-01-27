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
///* tre regex match utilities
use libc::*;
pub const TRE_INTERNAL_H: u32 = 1;
pub const _CTYPE_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
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
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const TRE_H: u32 = 1;
pub const HAVE_ALLOCA: u32 = 1;
pub const HAVE_ALLOCA_H: u32 = 1;
pub const HAVE_SYS_TYPES_H: u32 = 1;
pub const HAVE_WCHAR_H: u32 = 1;
pub const TRE_APPROX: u32 = 1;
pub const TRE_USE_ALLOCA: u32 = 1;
pub const TRE_VERSION: &'static [u8; 14usize] = b"@TRE_VERSION@\0";
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const __PTHREAD_MUTEX_LOCK_ELISION: u32 = 1;
pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND: u32 = 0;
pub const __PTHREAD_MUTEX_USE_UNION: u32 = 0;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
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
pub const TRE_CHAR_MAX: u32 = 255;
pub const TRE_MB_CUR_MAX: u32 = 1;
pub const STRF: &'static [u8; 2usize] = b"s\0";
pub const ASSERT_AT_BOL: u32 = 1;
pub const ASSERT_AT_EOL: u32 = 2;
pub const ASSERT_CHAR_CLASS: u32 = 4;
pub const ASSERT_CHAR_CLASS_NEG: u32 = 8;
pub const ASSERT_AT_BOW: u32 = 16;
pub const ASSERT_AT_EOW: u32 = 32;
pub const ASSERT_AT_WB: u32 = 64;
pub const ASSERT_AT_WB_NEG: u32 = 128;
pub const ASSERT_BACKREF: u32 = 256;
pub const ASSERT_LAST: u32 = 256;
pub const TRE_PARAM_UNSET: i32 = -1;
pub const TRE_PARAM_DEFAULT: i32 = -2;
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
pub const _ISupper: _bindgen_ty_1 = 256;
pub const _ISlower: _bindgen_ty_1 = 512;
pub const _ISalpha: _bindgen_ty_1 = 1024;
pub const _ISdigit: _bindgen_ty_1 = 2048;
pub const _ISxdigit: _bindgen_ty_1 = 4096;
pub const _ISspace: _bindgen_ty_1 = 8192;
pub const _ISprint: _bindgen_ty_1 = 16384;
pub const _ISgraph: _bindgen_ty_1 = 32768;
pub const _ISblank: _bindgen_ty_1 = 1;
pub const _IScntrl: _bindgen_ty_1 = 2;
pub const _ISpunct: _bindgen_ty_1 = 4;
pub const _ISalnum: _bindgen_ty_1 = 8;
pub type _bindgen_ty_1 = u32;
extern "C" {
    pub fn __ctype_b_loc() -> *mut *const c_ushort;
}
extern "C" {
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn isalnum(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isalpha(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn iscntrl(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isdigit(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn islower(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isgraph(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isprint(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn ispunct(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isspace(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isupper(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isxdigit(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn tolower(__c: c_int) -> c_int;
}
extern "C" {
    pub fn toupper(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isblank(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isascii(__c: c_int) -> c_int;
}
extern "C" {
    pub fn toascii(__c: c_int) -> c_int;
}
extern "C" {
    pub fn _toupper(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn _tolower(arg1: c_int) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const c_ushort,
    pub __ctype_tolower: *const c_int,
    pub __ctype_toupper: *const c_int,
    pub __names: [*const c_char; 13usize],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn isalnum_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isalpha_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iscntrl_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isdigit_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn islower_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isgraph_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isprint_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn ispunct_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isspace_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isupper_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isxdigit_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isblank_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn __tolower_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn tolower_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn __toupper_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn toupper_l(__c: c_int, __l: locale_t) -> c_int;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = c_ulong;
pub type ushort = c_ushort;
pub type uint = c_uint;
pub type u_int8_t = c_uchar;
pub type u_int16_t = c_ushort;
pub type u_int32_t = c_uint;
pub type u_int64_t = c_ulong;
pub type register_t = c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16usize],
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: c_uint,
    pub __writers: c_uint,
    pub __wrphase_futex: c_uint,
    pub __writers_futex: c_uint,
    pub __pad3: c_uint,
    pub __pad4: c_uint,
    pub __cur_writer: c_int,
    pub __shared: c_int,
    pub __rwelision: c_schar,
    pub __pad1: [c_uchar; 7usize],
    pub __pad2: c_ulong,
    pub __flags: c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: c_int,
    pub __count: c_uint,
    pub __owner: c_int,
    pub __nusers: c_uint,
    pub __kind: c_int,
    pub __spins: c_short,
    pub __elision: c_short,
    pub __list: __pthread_list_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [c_uint; 2usize],
    pub __g_size: [c_uint; 2usize],
    pub __g1_orig_size: c_uint,
    pub __wrefs: c_uint,
    pub __g_signals: [c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: c_uint,
    pub __high: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: c_uint,
    pub __high: c_uint,
}
pub type pthread_t = c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
pub type pthread_key_t = c_uint;
pub type pthread_once_t = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [c_char; 56usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [c_char; 48usize],
    pub __align: c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [c_char; 56usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [c_char; 8usize],
    pub __align: c_long,
    _bindgen_union_align: u64,
}
pub type pthread_spinlock_t = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [c_char; 32usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
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
pub const TRE_CONFIG_APPROX: _bindgen_ty_2 = 0;
pub const TRE_CONFIG_WCHAR: _bindgen_ty_2 = 1;
pub const TRE_CONFIG_MULTIBYTE: _bindgen_ty_2 = 2;
pub const TRE_CONFIG_SYSTEM_ABI: _bindgen_ty_2 = 3;
pub const TRE_CONFIG_VERSION: _bindgen_ty_2 = 4;
pub type _bindgen_ty_2 = u32;
extern "C" {
    pub fn tre_have_backrefs(preg: *const regex_t) -> c_int;
}
extern "C" {
    pub fn tre_have_approx(preg: *const regex_t) -> c_int;
}
pub type tre_cint_t = c_short;
pub type tre_ctype_t = Option<unsafe extern "C" fn(arg1: tre_cint_t) -> c_int>;
extern "C" {
    pub fn tre_ctype(name: *const c_char) -> tre_ctype_t;
}
pub const tre_str_type_t_STR_WIDE: tre_str_type_t = 0;
pub const tre_str_type_t_STR_BYTE: tre_str_type_t = 1;
pub const tre_str_type_t_STR_MBS: tre_str_type_t = 2;
pub const tre_str_type_t_STR_USER: tre_str_type_t = 3;
pub type tre_str_type_t = u32;
pub type tre_tnfa_transition_t = tnfa_transition;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tnfa_transition {
    pub code_min: tre_cint_t,
    pub code_max: tre_cint_t,
    pub state: *mut tre_tnfa_transition_t,
    pub state_id: c_int,
    pub tags: *mut c_int,
    pub params: *mut c_int,
    pub assertions: c_int,
    pub u: tnfa_transition__bindgen_ty_1,
    pub neg_classes: *mut tre_ctype_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tnfa_transition__bindgen_ty_1 {
    pub class: tre_ctype_t,
    pub backref: c_int,
    _bindgen_union_align: u64,
}
pub const tre_tag_direction_t_TRE_TAG_MINIMIZE: tre_tag_direction_t = 0;
pub const tre_tag_direction_t_TRE_TAG_MAXIMIZE: tre_tag_direction_t = 1;
pub type tre_tag_direction_t = u32;
pub const tre_param_t_TRE_PARAM_COST_INS: tre_param_t = 0;
pub const tre_param_t_TRE_PARAM_COST_DEL: tre_param_t = 1;
pub const tre_param_t_TRE_PARAM_COST_SUBST: tre_param_t = 2;
pub const tre_param_t_TRE_PARAM_COST_MAX: tre_param_t = 3;
pub const tre_param_t_TRE_PARAM_MAX_INS: tre_param_t = 4;
pub const tre_param_t_TRE_PARAM_MAX_DEL: tre_param_t = 5;
pub const tre_param_t_TRE_PARAM_MAX_SUBST: tre_param_t = 6;
pub const tre_param_t_TRE_PARAM_MAX_ERR: tre_param_t = 7;
pub const tre_param_t_TRE_PARAM_DEPTH: tre_param_t = 8;
pub const tre_param_t_TRE_PARAM_LAST: tre_param_t = 9;
pub type tre_param_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_submatch_data {
    pub so_tag: c_int,
    pub eo_tag: c_int,
    pub parents: *mut c_int,
}
pub type tre_submatch_data_t = tre_submatch_data;
pub type tre_tnfa_t = tnfa;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tnfa {
    pub transitions: *mut tre_tnfa_transition_t,
    pub num_transitions: c_uint,
    pub initial: *mut tre_tnfa_transition_t,
    pub final_: *mut tre_tnfa_transition_t,
    pub submatch_data: *mut tre_submatch_data_t,
    pub firstpos_chars: *mut c_char,
    pub first_char: c_int,
    pub num_submatches: c_uint,
    pub tag_directions: *mut tre_tag_direction_t,
    pub minimal_tags: *mut c_int,
    pub num_tags: c_int,
    pub num_minimals: c_int,
    pub end_tag: c_int,
    pub num_states: c_int,
    pub cflags: c_int,
    pub have_backrefs: c_int,
    pub have_approx: c_int,
    pub params_depth: c_int,
}
extern "C" {
    pub fn tre_compile(
        preg: *mut regex_t,
        regex: *const tre_char_t,
        n: usize,
        cflags: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn tre_free(preg: *mut regex_t);
}
extern "C" {
    pub fn tre_fill_pmatch(
        nmatch: usize,
        pmatch: *mut regmatch_t,
        cflags: c_int,
        tnfa: *const tre_tnfa_t,
        tags: *mut c_int,
        match_eo: c_int,
    );
}
extern "C" {
    pub fn tre_tnfa_run_parallel(
        tnfa: *const tre_tnfa_t,
        string: *const c_void,
        len: c_int,
        type_: tre_str_type_t,
        match_tags: *mut c_int,
        eflags: c_int,
        match_end_ofs: *mut c_int,
    ) -> reg_errcode_t;
}
extern "C" {
    pub fn tre_tnfa_run_backtrack(
        tnfa: *const tre_tnfa_t,
        string: *const c_void,
        len: c_int,
        type_: tre_str_type_t,
        match_tags: *mut c_int,
        eflags: c_int,
        match_end_ofs: *mut c_int,
    ) -> reg_errcode_t;
}
extern "C" {
    pub fn tre_tnfa_run_approx(
        tnfa: *const tre_tnfa_t,
        string: *const c_void,
        len: c_int,
        type_: tre_str_type_t,
        match_tags: *mut c_int,
        match_: *mut regamatch_t,
        params: regaparams_t,
        eflags: c_int,
        match_end_ofs: *mut c_int,
    ) -> reg_errcode_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
