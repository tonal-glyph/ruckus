/* automatically generated by rust-bindgen */

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
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
    pub type __u_char = ::std::os::raw::c_uchar;
    pub type __u_short = ::std::os::raw::c_ushort;
    pub type __u_int = ::std::os::raw::c_uint;
    pub type __u_long = ::std::os::raw::c_ulong;
    pub type __int8_t = ::std::os::raw::c_schar;
    pub type __uint8_t = ::std::os::raw::c_uchar;
    pub type __int16_t = ::std::os::raw::c_short;
    pub type __uint16_t = ::std::os::raw::c_ushort;
    pub type __int32_t = ::std::os::raw::c_int;
    pub type __uint32_t = ::std::os::raw::c_uint;
    pub type __int64_t = ::std::os::raw::c_long;
    pub type __uint64_t = ::std::os::raw::c_ulong;
    pub type __int_least8_t = root::__int8_t;
    pub type __uint_least8_t = root::__uint8_t;
    pub type __int_least16_t = root::__int16_t;
    pub type __uint_least16_t = root::__uint16_t;
    pub type __int_least32_t = root::__int32_t;
    pub type __uint_least32_t = root::__uint32_t;
    pub type __int_least64_t = root::__int64_t;
    pub type __uint_least64_t = root::__uint64_t;
    pub type __quad_t = ::std::os::raw::c_long;
    pub type __u_quad_t = ::std::os::raw::c_ulong;
    pub type __intmax_t = ::std::os::raw::c_long;
    pub type __uintmax_t = ::std::os::raw::c_ulong;
    pub type __dev_t = ::std::os::raw::c_ulong;
    pub type __uid_t = ::std::os::raw::c_uint;
    pub type __gid_t = ::std::os::raw::c_uint;
    pub type __ino_t = ::std::os::raw::c_ulong;
    pub type __ino64_t = ::std::os::raw::c_ulong;
    pub type __mode_t = ::std::os::raw::c_uint;
    pub type __nlink_t = ::std::os::raw::c_ulong;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    pub type __pid_t = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __fsid_t {
        pub __val: [::std::os::raw::c_int; 2usize],
    }
    pub type __clock_t = ::std::os::raw::c_long;
    pub type __rlim_t = ::std::os::raw::c_ulong;
    pub type __rlim64_t = ::std::os::raw::c_ulong;
    pub type __id_t = ::std::os::raw::c_uint;
    pub type __time_t = ::std::os::raw::c_long;
    pub type __useconds_t = ::std::os::raw::c_uint;
    pub type __suseconds_t = ::std::os::raw::c_long;
    pub type __daddr_t = ::std::os::raw::c_int;
    pub type __key_t = ::std::os::raw::c_int;
    pub type __clockid_t = ::std::os::raw::c_int;
    pub type __timer_t = *mut ::std::os::raw::c_void;
    pub type __blksize_t = ::std::os::raw::c_long;
    pub type __blkcnt_t = ::std::os::raw::c_long;
    pub type __blkcnt64_t = ::std::os::raw::c_long;
    pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
    pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
    pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
    pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
    pub type __fsword_t = ::std::os::raw::c_long;
    pub type __ssize_t = ::std::os::raw::c_long;
    pub type __syscall_slong_t = ::std::os::raw::c_long;
    pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
    pub type __loff_t = root::__off64_t;
    pub type __caddr_t = *mut ::std::os::raw::c_char;
    pub type __intptr_t = ::std::os::raw::c_long;
    pub type __socklen_t = ::std::os::raw::c_uint;
    pub type __sig_atomic_t = ::std::os::raw::c_int;
    pub type ino_t = root::__ino_t;
    pub type dev_t = root::__dev_t;
    pub type gid_t = root::__gid_t;
    pub type mode_t = root::__mode_t;
    pub type nlink_t = root::__nlink_t;
    pub type uid_t = root::__uid_t;
    pub type off_t = root::__off_t;
    pub type pid_t = root::__pid_t;
    pub type clockid_t = root::__clockid_t;
    pub type time_t = root::__time_t;
    pub type timer_t = root::__timer_t;
    pub type u_int8_t = ::std::os::raw::c_uchar;
    pub type u_int16_t = ::std::os::raw::c_ushort;
    pub type u_int32_t = ::std::os::raw::c_uint;
    pub type u_int64_t = ::std::os::raw::c_ulong;
    pub type register_t = ::std::os::raw::c_long;
    pub type blkcnt_t = root::__blkcnt_t;
    pub type fsblkcnt_t = root::__fsblkcnt_t;
    pub type fsfilcnt_t = root::__fsfilcnt_t;
    pub type regoff_t = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct regex_t {
        pub re_nsub: usize,
        pub value: *mut ::std::os::raw::c_void,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct regmatch_t {
        pub rm_so: root::regoff_t,
        pub rm_eo: root::regoff_t,
    }
    pub const reg_errcode_t_REG_OK: root::reg_errcode_t = 0;
    pub const reg_errcode_t_REG_NOMATCH: root::reg_errcode_t = 1;
    pub const reg_errcode_t_REG_BADPAT: root::reg_errcode_t = 2;
    pub const reg_errcode_t_REG_ECOLLATE: root::reg_errcode_t = 3;
    pub const reg_errcode_t_REG_ECTYPE: root::reg_errcode_t = 4;
    pub const reg_errcode_t_REG_EESCAPE: root::reg_errcode_t = 5;
    pub const reg_errcode_t_REG_ESUBREG: root::reg_errcode_t = 6;
    pub const reg_errcode_t_REG_EBRACK: root::reg_errcode_t = 7;
    pub const reg_errcode_t_REG_EPAREN: root::reg_errcode_t = 8;
    pub const reg_errcode_t_REG_EBRACE: root::reg_errcode_t = 9;
    pub const reg_errcode_t_REG_BADBR: root::reg_errcode_t = 10;
    pub const reg_errcode_t_REG_ERANGE: root::reg_errcode_t = 11;
    pub const reg_errcode_t_REG_ESPACE: root::reg_errcode_t = 12;
    pub const reg_errcode_t_REG_BADRPT: root::reg_errcode_t = 13;
    pub type reg_errcode_t = u32;
    extern "C" {
        pub fn tre_regcomp(
            preg: *mut root::regex_t,
            regex: *const ::std::os::raw::c_char,
            cflags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tre_regexec(
            preg: *const root::regex_t,
            string: *const ::std::os::raw::c_char,
            nmatch: usize,
            pmatch: *mut root::regmatch_t,
            eflags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tre_regerror(
            errcode: ::std::os::raw::c_int,
            preg: *const root::regex_t,
            errbuf: *mut ::std::os::raw::c_char,
            errbuf_size: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn tre_regfree(preg: *mut root::regex_t);
    }
    extern "C" {
        pub fn tre_regncomp(
            preg: *mut root::regex_t,
            regex: *const ::std::os::raw::c_char,
            len: usize,
            cflags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tre_regnexec(
            preg: *const root::regex_t,
            string: *const ::std::os::raw::c_char,
            len: usize,
            nmatch: usize,
            pmatch: *mut root::regmatch_t,
            eflags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct regaparams_t {
        pub cost_ins: ::std::os::raw::c_int,
        pub cost_del: ::std::os::raw::c_int,
        pub cost_subst: ::std::os::raw::c_int,
        pub max_cost: ::std::os::raw::c_int,
        pub max_ins: ::std::os::raw::c_int,
        pub max_del: ::std::os::raw::c_int,
        pub max_subst: ::std::os::raw::c_int,
        pub max_err: ::std::os::raw::c_int,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct regamatch_t {
        pub nmatch: usize,
        pub pmatch: *mut root::regmatch_t,
        pub cost: ::std::os::raw::c_int,
        pub num_ins: ::std::os::raw::c_int,
        pub num_del: ::std::os::raw::c_int,
        pub num_subst: ::std::os::raw::c_int,
    }
    extern "C" {
        pub fn tre_regaexec(
            preg: *const root::regex_t,
            string: *const ::std::os::raw::c_char,
            match_: *mut root::regamatch_t,
            params: root::regaparams_t,
            eflags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tre_reganexec(
            preg: *const root::regex_t,
            string: *const ::std::os::raw::c_char,
            len: usize,
            match_: *mut root::regamatch_t,
            params: root::regaparams_t,
            eflags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tre_regaparams_default(params: *mut root::regaparams_t);
    }
    pub type tre_char_t = ::std::os::raw::c_uchar;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct tre_str_source {
        pub get_next_char: ::std::option::Option<
            unsafe extern "C" fn(
                c: *mut root::tre_char_t,
                pos_add: *mut ::std::os::raw::c_uint,
                context: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        pub rewind: ::std::option::Option<
            unsafe extern "C" fn(pos: usize, context: *mut ::std::os::raw::c_void),
        >,
        pub compare: ::std::option::Option<
            unsafe extern "C" fn(
                pos1: usize,
                pos2: usize,
                len: usize,
                context: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        pub context: *mut ::std::os::raw::c_void,
    }
    extern "C" {
        pub fn tre_reguexec(
            preg: *const root::regex_t,
            string: *const root::tre_str_source,
            nmatch: usize,
            pmatch: *mut root::regmatch_t,
            eflags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tre_version() -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn tre_config(
            query: ::std::os::raw::c_int,
            result: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int;
    }
    pub const TRE_CONFIG_APPROX: root::_bindgen_ty_1 = 0;
    pub const TRE_CONFIG_WCHAR: root::_bindgen_ty_1 = 1;
    pub const TRE_CONFIG_MULTIBYTE: root::_bindgen_ty_1 = 2;
    pub const TRE_CONFIG_SYSTEM_ABI: root::_bindgen_ty_1 = 3;
    pub const TRE_CONFIG_VERSION: root::_bindgen_ty_1 = 4;
    pub type _bindgen_ty_1 = u32;
    extern "C" {
        pub fn tre_have_backrefs(preg: *const root::regex_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn tre_have_approx(preg: *const root::regex_t) -> ::std::os::raw::c_int;
    }
}
