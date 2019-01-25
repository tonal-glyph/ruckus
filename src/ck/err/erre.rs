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
extern crate libc;
use crate::chuck_def_h_edited::*;
use crate::dts::*;
/// log levels
pub const CK_LOG_CRAZY: u64 = 10; // set this to log everything
pub const CK_LOG_FINEST: u64 = 9;
pub const CK_LOG_FINER: u64 = 8;
pub const CK_LOG_FINE: u64 = 7;
pub const CK_LOG_CONFIG: u64 = 6;
pub const CK_LOG_INFO: u64 = 5;
pub const CK_LOG_WARNING: u64 = 4;
pub const CK_LOG_SEVERE: u64 = 3;
pub const CK_LOG_SYSTEM: u64 = 2;
pub const CK_LOG_CORE: u64 = 1;
pub const CK_LOG_NONE: u64 = 0; // set this to log nothing
/// Rewrite stdio functions
// #define CK_FPRINTF_STDOUT(...) ck_fprintf_stdout(__VA_ARGS__)
// #define CK_FPRINTF_STDERR(...) ck_fprintf_stderr(__VA_ARGS__)
// #define CK_FFLUSH_STDOUT() ck_fflush_stdout()
// #define CK_FFLUSH_STDERR() ck_fflush_stderr()
// #define CK_VFPRINTF_STDOUT(message, ap) ck_vfprintf_stdout(message, ap)
// #define CK_VFPRINTF_STDERR(message, ap) ck_vfprintf_stderr(message, ap)
extern "C" {
    #[link_name = "\u{1}EM_anyErrors"]
    pub static mut EM_anyErrors: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}EM_tokPos"]
    pub static mut EM_tokPos: libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}EM_lineNum"]
    pub static mut EM_lineNum: libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}EM_extLineNum"]
    pub static mut EM_extLineNum: libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = libc::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: usize,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20usize],
}
pub type cookie_read_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut libc::c_void,
        __buf: *mut libc::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_write_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut libc::c_void,
        __buf: *const libc::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_seek_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut libc::c_void,
        __pos: *mut __off64_t,
        __w: libc::c_int,
    ) -> libc::c_int,
>;
pub type cookie_close_function_t =
    ::std::option::Option<unsafe extern "C" fn(__cookie: *mut libc::c_void) -> libc::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_cookie_io_functions_t {
    pub read: cookie_read_function_t,
    pub write: cookie_write_function_t,
    pub seek: cookie_seek_function_t,
    pub close: cookie_close_function_t,
}
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
pub type va_list = __gnuc_va_list;
pub type fpos_t = __fpos_t;
pub type fpos64_t = __fpos64_t;
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __u_long = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = libc::c_long;
pub type __u_quad_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
extern "C" {
    #[link_name = "\u{1}stdin"]
    pub static mut stdin: *mut FILE;
}
extern "C" {
    #[link_name = "\u{1}stdout"]
    pub static mut stdout: *mut FILE;
}
extern "C" {
    #[link_name = "\u{1}stderr"]
    pub static mut stderr: *mut FILE;
}
extern "C" {
    pub fn remove(__filename: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn renameat2(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
        __flags: libc::c_uint,
    ) -> libc::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpfile64() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn tempnam(__dir: *const libc::c_char, __pfx: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn fcloseall() -> libc::c_int;
}
extern "C" {
    pub fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fopen64(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
}
extern "C" {
    pub fn freopen64(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
}
extern "C" {
    pub fn fopencookie(
        __magic_cookie: *mut libc::c_void,
        __modes: *const libc::c_char,
        __io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut libc::c_void,
        __len: usize,
        __modes: *const libc::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(__bufloc: *mut libc::c_char, __sizeloc: *mut usize) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut libc::c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(__stream: *mut FILE, __format: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn printf(__format: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn sprintf(__s: *mut libc::c_char, __format: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const libc::c_char,
        __arg: *mut __va_list_tag,
    ) -> libc::c_int;
}
extern "C" {
    pub fn vprintf(__format: *const libc::c_char, __arg: *mut __va_list_tag) -> libc::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut libc::c_char,
        __format: *const libc::c_char,
        __arg: *mut __va_list_tag,
    ) -> libc::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut libc::c_char,
        __maxlen: usize,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut libc::c_char,
        __maxlen: usize,
        __format: *const libc::c_char,
        __arg: *mut __va_list_tag,
    ) -> libc::c_int;
}
extern "C" {
    pub fn vasprintf(
        __ptr: *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: *mut __va_list_tag,
    ) -> libc::c_int;
}
extern "C" {
    pub fn __asprintf(__ptr: *mut libc::c_char, __fmt: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn asprintf(__ptr: *mut libc::c_char, __fmt: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: libc::c_int,
        __fmt: *const libc::c_char,
        __arg: *mut __va_list_tag,
    ) -> libc::c_int;
}
extern "C" {
    pub fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn fscanf(__stream: *mut FILE, __format: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn scanf(__format: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn sscanf(__s: *const libc::c_char, __format: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const libc::c_char,
        __arg: *mut __va_list_tag,
    ) -> libc::c_int;
}
extern "C" {
    pub fn vscanf(__format: *const libc::c_char, __arg: *mut __va_list_tag) -> libc::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const libc::c_char,
        __format: *const libc::c_char,
        __arg: *mut __va_list_tag,
    ) -> libc::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn getchar() -> libc::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> libc::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn putchar(__c: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn putw(__w: libc::c_int, __stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn fgets_unlocked(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut libc::c_char,
        __n: *mut usize,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut libc::c_char,
        __n: *mut usize,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(__lineptr: *mut libc::c_char, __n: *mut usize, __stream: *mut FILE)
        -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn puts(__s: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn fread(__ptr: *mut libc::c_void, __size: usize, __n: usize, __stream: *mut FILE)
        -> usize;
}
extern "C" {
    pub fn fwrite(__ptr: *const libc::c_void, __size: usize, __n: usize, __s: *mut FILE) -> usize;
}
extern "C" {
    pub fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> libc::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> libc::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> libc::c_int;
}
extern "C" {
    pub fn fseeko64(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn ftello64(__stream: *mut FILE) -> __off64_t;
}
extern "C" {
    pub fn fgetpos64(__stream: *mut FILE, __pos: *mut fpos64_t) -> libc::c_int;
}
extern "C" {
    pub fn fsetpos64(__stream: *mut FILE, __pos: *const fpos64_t) -> libc::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn perror(__s: *const libc::c_char);
}
extern "C" {
    #[link_name = "\u{1}sys_nerr"]
    pub static mut sys_nerr: libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}sys_errlist"]
    pub static mut sys_errlist: [*const libc::c_char; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_sys_nerr"]
    pub static mut _sys_nerr: libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}_sys_errlist"]
    pub static mut _sys_errlist: [*const libc::c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn cuserid(__s: *mut libc::c_char) -> *mut libc::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct obstack {
    _unused: [u8; 0],
}
extern "C" {
    pub fn obstack_printf(
        __obstack: *mut obstack,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C" {
    pub fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const libc::c_char,
        __args: *mut __va_list_tag,
    ) -> libc::c_int;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn EM_newline();
}
extern "C" {
    pub fn ck_fprintf_stdout(format: *const libc::c_char, ...);
}
extern "C" {
    pub fn ck_fprintf_stderr(format: *const libc::c_char, ...);
}
extern "C" {
    pub fn ck_fflush_stdout();
}
extern "C" {
    pub fn ck_fflush_stderr();
}
extern "C" {
    pub fn ck_vfprintf_stdout(format: *const libc::c_char, args: *mut __va_list_tag);
}
extern "C" {
    pub fn ck_vfprintf_stderr(format: *const libc::c_char, args: *mut __va_list_tag);
}
extern "C" {
    pub fn ck_set_stdout_callback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    );
}
extern "C" {
    pub fn ck_set_stderr_callback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    );
}
#[repr(C)]
pub struct ChuckOutStream {
    pub m_stream: crate::dts::basic_istringstream<_CharT>,
    pub m_callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    pub m_isErr: bool,
}
extern "C" {
    #[link_name = "\u{1}_ZN14ChuckOutStream12set_callbackEPFvPKcE"]
    pub fn ChuckOutStream_set_callback(
        this: *mut ChuckOutStream,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN14ChuckOutStreamC1Eb"]
    pub fn ChuckOutStream_ChuckOutStream(this: *mut ChuckOutStream, isErr: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN14ChuckOutStreamD1Ev"]
    pub fn ChuckOutStream_ChuckOutStream_destructor(this: *mut ChuckOutStream);
}
impl ChuckOutStream {
    #[inline]
    pub unsafe fn set_callback(
        &mut self,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) {
        ChuckOutStream_set_callback(self, callback)
    }
    #[inline]
    pub unsafe fn new(isErr: bool) -> Self {
        let mut __bindgen_tmp = uninitialized();
        ChuckOutStream_ChuckOutStream(&mut __bindgen_tmp, isErr);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        ChuckOutStream_ChuckOutStream_destructor(self)
    }
}
extern "C" {
    #[link_name = "\u{1}g_ck_stdoutstream"]
    pub static mut g_ck_stdoutstream: ChuckOutStream;
}
extern "C" {
    #[link_name = "\u{1}g_ck_stderrstream"]
    pub static mut g_ck_stderrstream: ChuckOutStream;
}
extern "C" {
    pub fn EM_log(arg1: libc::c_long, arg2: c_constr, ...);
}
extern "C" {
    pub fn EM_setlog(arg1: libc::c_long);
}
extern "C" {
    pub fn EM_pushlog();
}
extern "C" {
    pub fn EM_poplog();
}
extern "C" {
    #[link_name = "\u{1}g_loglevel"]
    pub static mut g_loglevel: libc::c_long;
}
extern "C" {
    pub fn EM_error(arg1: libc::c_int, arg2: c_constr, ...);
}
extern "C" {
    pub fn EM_error2(arg1: libc::c_int, arg2: c_constr, ...);
}
extern "C" {
    pub fn EM_error2b(arg1: libc::c_int, arg2: c_constr, ...);
}
extern "C" {
    pub fn EM_error3(arg1: c_constr, ...);
}
extern "C" {
    pub fn EM_impossible(arg1: c_constr, ...);
}
extern "C" {
    pub fn EM_reset(filename: c_constr, fd: *mut FILE) -> libc::c_ulong;
}
extern "C" {
    pub fn EM_change_file(filename: c_constr);
}
extern "C" {
    pub fn EM_lasterror() -> *const libc::c_char;
}
extern "C" {
    pub fn EM_reset_msg();
}
extern "C" {
    pub fn mini(str: *const libc::c_char) -> *const libc::c_char;
}
extern "C" {
    pub fn mini_type(str: *const libc::c_char) -> *const libc::c_char;
}
