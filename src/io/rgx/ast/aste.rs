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
use crate::regex::tre_compile_h_edited::*;
use crate::regex::tre_internal_h_edited::*;
use crate::regex::tre_mem_h_edited::*;
use libc::*;
//* tre abstract syntax tree
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const TRE_AST_H: u32 = 1;
pub const TRE_MEM_H: u32 = 1;
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
pub const TRE_MEM_BLOCK_SIZE: u32 = 1024;
pub const TRE_INTERNAL_H: u32 = 1;
pub const _CTYPE_H: u32 = 1;
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
pub const TRE_H: u32 = 1;
pub const HAVE_ALLOCA: u32 = 1;
pub const HAVE_ALLOCA_H: u32 = 1;
pub const HAVE_SYS_TYPES_H: u32 = 1;
pub const HAVE_WCHAR_H: u32 = 1;
pub const TRE_APPROX: u32 = 1;
pub const TRE_USE_ALLOCA: u32 = 1;
pub const TRE_VERSION: &'static [u8; 14usize] = b"@TRE_VERSION@\0";
pub const _SYS_TYPES_H: u32 = 1;
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
pub const TRE_COMPILE_H: u32 = 1;
/* Special subtypes of TRE_LITERAL. */
pub const EMPTY: i32 = -1; /* Empty leaf (denotes empty string). */
pub const ASSERTION: i32 = -2; /* Assertion leaf. */
pub const TAG: i32 = -3; /* Tag leaf. */
pub const BACKREF: i32 = -4; /* Back reference leaf. */
pub const PARAMETER: i32 = -5; /* Parameter. */
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
    pub fn strtoull(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_ulonglong;
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_list {
    pub data: *mut c_void,
    pub next: *mut tre_list,
}
pub type tre_list_t = tre_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_mem_struct {
    pub blocks: *mut tre_list_t,
    pub current: *mut tre_list_t,
    pub ptr: *mut c_char,
    pub n: usize,
    pub failed: c_int,
    pub provided: *mut c_void,
}
pub type tre_mem_t = *mut tre_mem_struct;
extern "C" {
    pub fn tre_mem_new_impl(provided: c_int, provided_block: *mut c_void) -> tre_mem_t;
}
extern "C" {
    pub fn tre_mem_alloc_impl(
        mem: tre_mem_t,
        provided: c_int,
        provided_block: *mut c_void,
        zero: c_int,
        size: usize,
    ) -> *mut c_void;
}
extern "C" {
    pub fn tre_mem_destroy(mem: tre_mem_t);
}
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
pub struct tre_pos_and_tags_t {
    pub position: c_int,
    pub code_min: c_int,
    pub code_max: c_int,
    pub tags: *mut c_int,
    pub assertions: c_int,
    pub class: tre_ctype_t,
    pub neg_classes: *mut tre_ctype_t,
    pub backref: c_int,
    pub params: *mut c_int,
}
/* The different AST node types. */
pub const tre_ast_type_t_LITERAL: tre_ast_type_t = 0;
pub const tre_ast_type_t_CATENATION: tre_ast_type_t = 1;
pub const tre_ast_type_t_ITERATION: tre_ast_type_t = 2;
pub const tre_ast_type_t_UNION: tre_ast_type_t = 3;
pub type tre_ast_type_t = u32;
/* A generic AST node.  All AST nodes consist of this node on the top
level with `obj' pointing to the actual content. */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_ast_node_t {
    pub type_: tre_ast_type_t, /* Type of the node. */
    pub obj: *mut c_void,      /* Pointer to actual node. */
    pub nullable: c_int,
    pub submatch_id: c_int,
    pub num_submatches: c_int,
    pub num_tags: c_int,
    pub firstpos: *mut tre_pos_and_tags_t,
    pub lastpos: *mut tre_pos_and_tags_t,
}
/* A "literal" node.  These are created for assertions, back references,
tags, matching parameter settings, and all expressions that match one
character. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tre_literal_t {
    pub code_min: c_long,
    pub code_max: c_long,
    pub position: c_int,
    pub u: tre_literal_t__bindgen_ty_1,
    pub neg_classes: *mut tre_ctype_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tre_literal_t__bindgen_ty_1 {
    pub class: tre_ctype_t,
    pub params: *mut c_int,
    _bindgen_union_align: u64,
}
/* A "catenation" node.	 These are created when two regexps are concatenated.
If there are more than one subexpressions in sequence, the `left' part
holds all but the last, and `right' part holds the last subexpression
(catenation is left associative). */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_catenation_t {
    pub left: *mut tre_ast_node_t,
    pub right: *mut tre_ast_node_t,
}
/* An "iteration" node.	 These are created for the "*", "+", "?", and "{m,n}"
operators. */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_iteration_t {
    pub arg: *mut tre_ast_node_t, /* Subexpression to match. */
    pub min: c_int,               /* Minimum number of consecutive matches. */
    pub max: c_int,               /* Maximum number of consecutive matches. */
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub params: *mut c_int, /* Approximate matching parameters (or NULL). */
}
impl tre_iteration_t {
    /* If 0, match as many characters as possible, if 1 match as few as
    possible.	Note that this does not always mean the same thing as
    matching as many/few repetitions as possible. */
    #[inline]
    pub fn minimal(&self) -> c_uint {
        unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_minimal(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(minimal: c_uint) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let minimal: u32 = unsafe { transmute(minimal) };
            minimal as u64
        });
        __bindgen_bitfield_unit
    }
}
/* An "union" node.  These are created for the "|" operator. */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_union_t {
    pub left: *mut tre_ast_node_t,
    pub right: *mut tre_ast_node_t,
}
extern "C" {
    pub fn tre_ast_new_node(
        mem: tre_mem_t,
        type_: tre_ast_type_t,
        size: usize,
    ) -> *mut tre_ast_node_t;
}
extern "C" {
    pub fn tre_ast_new_literal(
        mem: tre_mem_t,
        code_min: c_int,
        code_max: c_int,
        position: c_int,
    ) -> *mut tre_ast_node_t;
}
extern "C" {
    pub fn tre_ast_new_iter(
        mem: tre_mem_t,
        arg: *mut tre_ast_node_t,
        min: c_int,
        max: c_int,
        minimal: c_int,
    ) -> *mut tre_ast_node_t;
}
extern "C" {
    pub fn tre_ast_new_union(
        mem: tre_mem_t,
        left: *mut tre_ast_node_t,
        right: *mut tre_ast_node_t,
    ) -> *mut tre_ast_node_t;
}
extern "C" {
    pub fn tre_ast_new_catenation(
        mem: tre_mem_t,
        left: *mut tre_ast_node_t,
        right: *mut tre_ast_node_t,
    ) -> *mut tre_ast_node_t;
}
