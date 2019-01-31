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
use crate::chuck_def_h_edited::*;
use crate::chuck_type_h_edited::*;
use crate::sys::*;
use crate::util_buffers_h_edited::*;
///* thread utility functions
use libc::*;
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type blkcnt64_t = __blkcnt64_t;
pub type fsblkcnt64_t = __fsblkcnt64_t;
pub type fsfilcnt64_t = __fsfilcnt64_t;
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
#[repr(align(8))]
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
#[repr(align(8))]
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
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
pub type pthread_key_t = c_uint;
pub type pthread_once_t = c_int;
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [c_char; 56usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [c_char; 48usize],
    pub __align: c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [c_char; 56usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [c_char; 8usize],
    pub __align: c_long,
    _bindgen_union_align: u64,
}
pub type pthread_spinlock_t = c_int;
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [c_char; 32usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
pub const PTHREAD_CREATE_JOINABLE: _bindgen_ty_1 = 0;
pub const PTHREAD_CREATE_DETACHED: _bindgen_ty_1 = 1;
pub type _bindgen_ty_1 = u32;
pub const PTHREAD_MUTEX_TIMED_NP: _bindgen_ty_2 = 0;
pub const PTHREAD_MUTEX_RECURSIVE_NP: _bindgen_ty_2 = 1;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: _bindgen_ty_2 = 2;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: _bindgen_ty_2 = 3;
pub const PTHREAD_MUTEX_NORMAL: _bindgen_ty_2 = 0;
pub const PTHREAD_MUTEX_RECURSIVE: _bindgen_ty_2 = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: _bindgen_ty_2 = 2;
pub const PTHREAD_MUTEX_DEFAULT: _bindgen_ty_2 = 0;
pub const PTHREAD_MUTEX_FAST_NP: _bindgen_ty_2 = 0;
pub type _bindgen_ty_2 = u32;
pub const PTHREAD_MUTEX_STALLED: _bindgen_ty_3 = 0;
pub const PTHREAD_MUTEX_STALLED_NP: _bindgen_ty_3 = 0;
pub const PTHREAD_MUTEX_ROBUST: _bindgen_ty_3 = 1;
pub const PTHREAD_MUTEX_ROBUST_NP: _bindgen_ty_3 = 1;
pub type _bindgen_ty_3 = u32;
pub const PTHREAD_PRIO_NONE: _bindgen_ty_4 = 0;
pub const PTHREAD_PRIO_INHERIT: _bindgen_ty_4 = 1;
pub const PTHREAD_PRIO_PROTECT: _bindgen_ty_4 = 2;
pub type _bindgen_ty_4 = u32;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: _bindgen_ty_5 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: _bindgen_ty_5 = 1;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: _bindgen_ty_5 = 2;
pub const PTHREAD_RWLOCK_DEFAULT_NP: _bindgen_ty_5 = 0;
pub type _bindgen_ty_5 = u32;
pub const PTHREAD_INHERIT_SCHED: _bindgen_ty_6 = 0;
pub const PTHREAD_EXPLICIT_SCHED: _bindgen_ty_6 = 1;
pub type _bindgen_ty_6 = u32;
pub const PTHREAD_SCOPE_SYSTEM: _bindgen_ty_7 = 0;
pub const PTHREAD_SCOPE_PROCESS: _bindgen_ty_7 = 1;
pub type _bindgen_ty_7 = u32;
pub const PTHREAD_PROCESS_PRIVATE: _bindgen_ty_8 = 0;
pub const PTHREAD_PROCESS_SHARED: _bindgen_ty_8 = 1;
pub type _bindgen_ty_8 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _pthread_cleanup_buffer {
    pub __routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __arg: *mut c_void,
    pub __canceltype: c_int,
    pub __prev: *mut _pthread_cleanup_buffer,
}
pub const PTHREAD_CANCEL_ENABLE: _bindgen_ty_9 = 0;
pub const PTHREAD_CANCEL_DISABLE: _bindgen_ty_9 = 1;
pub type _bindgen_ty_9 = u32;
pub const PTHREAD_CANCEL_DEFERRED: _bindgen_ty_10 = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: _bindgen_ty_10 = 1;
pub type _bindgen_ty_10 = u32;
extern "C" {
    pub fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>,
        __arg: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_exit(__retval: *mut c_void);
}
extern "C" {
    pub fn pthread_join(__th: pthread_t, __thread_return: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pthread_tryjoin_np(__th: pthread_t, __thread_return: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pthread_timedjoin_np(
        __th: pthread_t,
        __thread_return: *mut c_void,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_detach(__th: pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_self() -> pthread_t;
}
extern "C" {
    pub fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_init(__attr: *mut pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getdetachstate(
        __attr: *const pthread_attr_t,
        __detachstate: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setdetachstate(__attr: *mut pthread_attr_t, __detachstate: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getguardsize(
        __attr: *const pthread_attr_t,
        __guardsize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setguardsize(__attr: *mut pthread_attr_t, __guardsize: usize) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getschedparam(
        __attr: *const pthread_attr_t,
        __param: *mut sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setschedparam(
        __attr: *mut pthread_attr_t,
        __param: *const sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getschedpolicy(
        __attr: *const pthread_attr_t,
        __policy: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setschedpolicy(__attr: *mut pthread_attr_t, __policy: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getinheritsched(
        __attr: *const pthread_attr_t,
        __inherit: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setinheritsched(__attr: *mut pthread_attr_t, __inherit: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getscope(__attr: *const pthread_attr_t, __scope: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setscope(__attr: *mut pthread_attr_t, __scope: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstackaddr(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstackaddr(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstacksize(__attr: *mut pthread_attr_t, __stacksize: usize) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstack(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut c_void,
        __stacksize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstack(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut c_void,
        __stacksize: usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setaffinity_np(
        __attr: *mut pthread_attr_t,
        __cpusetsize: usize,
        __cpuset: *const cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getaffinity_np(
        __attr: *const pthread_attr_t,
        __cpusetsize: usize,
        __cpuset: *mut cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getattr_default_np(__attr: *mut pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_setattr_default_np(__attr: *const pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_getattr_np(__th: pthread_t, __attr: *mut pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: c_int,
        __param: *const sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut c_int,
        __param: *mut sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_setschedprio(__target_thread: pthread_t, __prio: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_getname_np(
        __target_thread: pthread_t,
        __buf: *mut c_char,
        __buflen: usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_setname_np(__target_thread: pthread_t, __name: *const c_char) -> c_int;
}
extern "C" {
    pub fn pthread_getconcurrency() -> c_int;
}
extern "C" {
    pub fn pthread_setconcurrency(__level: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_yield() -> c_int;
}
extern "C" {
    pub fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: usize,
        __cpuset: *const cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getaffinity_np(
        __th: pthread_t,
        __cpusetsize: usize,
        __cpuset: *mut cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn()>,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_setcancelstate(__state: c_int, __oldstate: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pthread_setcanceltype(__type: c_int, __oldtype: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pthread_cancel(__th: pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_testcancel();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__pthread_unwind_buf_t__bindgen_ty_1; 1usize],
    pub __pad: [*mut c_void; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_unwind_buf_t__bindgen_ty_1 {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __cancel_arg: *mut c_void,
    pub __do_it: c_int,
    pub __cancel_type: c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct __pthread_cleanup_class {
    pub __cancel_routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __cancel_arg: *mut c_void,
    pub __do_it: c_int,
    pub __cancel_type: c_int,
}
extern "C" {
    #[link_name = "\u{1}__setdoit"]
    pub fn __pthread_cleanup_class___setdoit(this: *mut __pthread_cleanup_class, __newval: c_int);
}
extern "C" {
    #[link_name = "\u{1}__defer"]
    pub fn __pthread_cleanup_class___defer(this: *mut __pthread_cleanup_class);
}
extern "C" {
    #[link_name = "\u{1}__restore"]
    pub fn __pthread_cleanup_class___restore(this: *const __pthread_cleanup_class);
}
extern "C" {
    #[link_name = "\u{1}__pthread_cleanup_class"]
    pub fn __pthread_cleanup_class___pthread_cleanup_class(
        this: *mut __pthread_cleanup_class,
        __fct: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        __arg: *mut c_void,
    );
}
extern "C" {
    #[link_name = "\u{1}__pthread_cleanup_class_destructor"]
    pub fn __pthread_cleanup_class___pthread_cleanup_class_destructor(
        this: *mut __pthread_cleanup_class,
    );
}
impl __pthread_cleanup_class {
    #[inline]
    pub unsafe fn __setdoit(&mut self, __newval: c_int) {
        __pthread_cleanup_class___setdoit(self, __newval)
    }
    #[inline]
    pub unsafe fn __defer(&mut self) {
        __pthread_cleanup_class___defer(self)
    }
    #[inline]
    pub unsafe fn __restore(&self) {
        __pthread_cleanup_class___restore(self)
    }
    #[inline]
    pub unsafe fn new(
        __fct: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        __arg: *mut c_void,
    ) -> Self {
        let mut __bindgen_tmp = uninitialized();
        __pthread_cleanup_class___pthread_cleanup_class(&mut __bindgen_tmp, __fct, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        __pthread_cleanup_class___pthread_cleanup_class_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
    _unused: [u8; 0],
}
extern "C" {
    pub fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_getprioceiling(
        __mutex: *const pthread_mutex_t,
        __prioceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_setprioceiling(
        __mutex: *mut pthread_mutex_t,
        __prioceiling: c_int,
        __old_ceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent_np(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getpshared(
        __attr: *const pthread_mutexattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_settype(__attr: *mut pthread_mutexattr_t, __kind: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprotocol(
        __attr: *const pthread_mutexattr_t,
        __protocol: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprotocol(
        __attr: *mut pthread_mutexattr_t,
        __protocol: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprioceiling(
        __attr: *const pthread_mutexattr_t,
        __prioceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprioceiling(
        __attr: *mut pthread_mutexattr_t,
        __prioceiling: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust_np(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust_np(
        __attr: *mut pthread_mutexattr_t,
        __robustness: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_tryrdlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
extern "C" {
    pub fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_trywrlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getpshared(
        __attr: *const pthread_rwlockattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setpshared(
        __attr: *mut pthread_rwlockattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getkind_np(
        __attr: *const pthread_rwlockattr_t,
        __pref: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setkind_np(__attr: *mut pthread_rwlockattr_t, __pref: c_int)
        -> c_int;
}
extern "C" {
    pub fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_getpshared(
        __attr: *const pthread_condattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_setpshared(__attr: *mut pthread_condattr_t, __pshared: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut __clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_spin_init(__lock: *mut pthread_spinlock_t, __pshared: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_spin_destroy(__lock: *mut pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_init(
        __barrier: *mut pthread_barrier_t,
        __attr: *const pthread_barrierattr_t,
        __count: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_destroy(__barrier: *mut pthread_barrier_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_wait(__barrier: *mut pthread_barrier_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_init(__attr: *mut pthread_barrierattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_destroy(__attr: *mut pthread_barrierattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_getpshared(
        __attr: *const pthread_barrierattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_setpshared(
        __attr: *mut pthread_barrierattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_key_delete(__key: pthread_key_t) -> c_int;
}
extern "C" {
    pub fn pthread_getspecific(__key: pthread_key_t) -> *mut c_void;
}
extern "C" {
    pub fn pthread_setspecific(__key: pthread_key_t, __pointer: *const c_void) -> c_int;
}
extern "C" {
    pub fn pthread_getcpuclockid(__thread_id: pthread_t, __clock_id: *mut __clockid_t) -> c_int;
}
extern "C" {
    pub fn pthread_atfork(
        __prepare: Option<unsafe extern "C" fn()>,
        __parent: Option<unsafe extern "C" fn()>,
        __child: Option<unsafe extern "C" fn()>,
    ) -> c_int;
}
pub type THREAD_HANDLE = pthread_t;
pub type THREAD_RETURN = *mut c_void;
pub type THREAD_FUNCTION = Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>;
pub type MUTEX = pthread_mutex_t;
#[repr(C)]
#[derive(Debug)]
pub struct XThread {
    pub thread: THREAD_HANDLE,
}
///* begin execution of the thread routine
///* the thread routine can be passed an argument via ptr
extern "C" {
    #[link_name = "\u{1}start"]
    pub fn XThread_start(this: *mut XThread, routine: THREAD_FUNCTION, ptr: *mut c_void) -> bool;
}
extern "C" {
    #[link_name = "\u{1}wait"]
    pub fn XThread_wait(this: *mut XThread, milliseconds: c_long, cancel: bool) -> bool;
}
extern "C" {
    #[link_name = "\u{1}test"]
    pub fn XThread_test();
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn XThread_clear(this: *mut XThread);
}
extern "C" {
    #[link_name = "\u{1}XThread"]
    pub fn XThread_XThread(this: *mut XThread);
}
extern "C" {
    #[link_name = "\u{1}XThread_destructor"]
    pub fn XThread_XThread_destructor(this: *mut XThread);
}
impl XThread {
    #[inline]
    pub unsafe fn start(&mut self, routine: THREAD_FUNCTION, ptr: *mut c_void) -> bool {
        XThread_start(self, routine, ptr)
    }
    ///* wait the specified number of milliseconds for the thread to terminate
    #[inline]
    pub unsafe fn wait(&mut self, milliseconds: c_long, cancel: bool) -> bool {
        XThread_wait(self, milliseconds, cancel)
    }
    ///* test for a thread cancellation request.
    #[inline]
    pub unsafe fn test() {
        XThread_test()
    }
    ///* clear
    #[inline]
    pub unsafe fn clear(&mut self) {
        XThread_clear(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        XThread_XThread(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        XThread_XThread_destructor(self)
    }
}
///*-----------------------------------------------------------------------------
///* name: struct XThreadUtil
///* desc: general thread utility
///*-----------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XThreadUtil {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}our_priority"]
    pub static mut XThreadUtil_our_priority: c_long;
}
///* set priority on thread
extern "C" {
    #[link_name = "\u{1}set_priority"]
    pub fn XThreadUtil_set_priority(tid: pthread_t, priority: c_long) -> c_ulong;
}
///* set priority on current thread
extern "C" {
    #[link_name = "\u{1}set_priority"]
    pub fn XThreadUtil_set_priority1(priority: c_long) -> c_ulong;
}
impl XThreadUtil {
    #[inline]
    pub unsafe fn set_priority(tid: pthread_t, priority: c_long) -> c_ulong {
        XThreadUtil_set_priority(tid, priority)
    }
    #[inline]
    pub unsafe fn set_priority1(priority: c_long) -> c_ulong {
        XThreadUtil_set_priority1(priority)
    }
}
#[repr(C)]
pub struct XMutex {
    pub mutex: MUTEX,
}
extern "C" {
    #[link_name = "\u{1}acquire"]
    pub fn XMutex_acquire(this: *mut XMutex);
}
extern "C" {
    #[link_name = "\u{1}release"]
    pub fn XMutex_release(this: *mut XMutex);
}
extern "C" {
    #[link_name = "\u{1}XMutex"]
    pub fn XMutex_XMutex(this: *mut XMutex);
}
extern "C" {
    #[link_name = "\u{1}XMutex_destructor"]
    pub fn XMutex_XMutex_destructor(this: *mut XMutex);
}
impl XMutex {
    #[inline]
    pub unsafe fn acquire(&mut self) {
        XMutex_acquire(self)
    }
    #[inline]
    pub unsafe fn release(&mut self) {
        XMutex_release(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        XMutex_XMutex(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        XMutex_XMutex_destructor(self)
    }
}
///*-----------------------------------------------------------------------------
///* name: XWriteThread()
///* desc: utility class for scheduling writes to be executed on a separate
///*       thread.
///*-----------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug)]
pub struct XWriteThread {
    pub m_thread_exit: c_ulong,
    pub m_thread: XThread,
    pub m_data_buffer: *mut FastCircularBuffer,
    pub m_bytes_in_buffer: usize,
    pub m_thread_buffer: *mut c_uchar,
    pub m_stream: *mut FILE,
    pub m_msg_buffer: *mut CircularBuffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XWriteThread_Message {
    pub operation: XWriteThread_Message__bindgen_ty_1,
    pub file: *mut FILE,
    pub __bindgen_anon_1: XWriteThread_Message__bindgen_ty_2,
}
pub const XWriteThread_Message_WRITE: XWriteThread_Message__bindgen_ty_1 = 0;
pub const XWriteThread_Message_SEEK: XWriteThread_Message__bindgen_ty_1 = 1;
pub const XWriteThread_Message_FLUSH: XWriteThread_Message__bindgen_ty_1 = 2;
pub const XWriteThread_Message_CLOSE: XWriteThread_Message__bindgen_ty_1 = 3;
pub const XWriteThread_Message_SHUTDOWN: XWriteThread_Message__bindgen_ty_1 = 4;
pub type XWriteThread_Message__bindgen_ty_1 = u32;
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union XWriteThread_Message__bindgen_ty_2 {
    pub write: XWriteThread_Message__bindgen_ty_2__bindgen_ty_1,
    pub seek: XWriteThread_Message__bindgen_ty_2__bindgen_ty_2,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XWriteThread_Message__bindgen_ty_2__bindgen_ty_1 {
    pub data_size: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XWriteThread_Message__bindgen_ty_2__bindgen_ty_2 {
    pub offset: c_long,
    pub whence: c_int,
}
///* static buffer size
extern "C" {
    #[link_name = "\u{1}PRODUCER_BUFFER_SIZE"]
    pub static XWriteThread_PRODUCER_BUFFER_SIZE: usize;
}
extern "C" {
    #[link_name = "\u{1}o_defaultWriteThread"]
    pub static mut XWriteThread_o_defaultWriteThread: *mut XWriteThread;
}
///* get the shared instance
extern "C" {
    #[link_name = "\u{1}shared"]
    pub fn XWriteThread_shared() -> *mut XWriteThread;
}
extern "C" {
    #[link_name = "\u{1}fwrite"]
    pub fn XWriteThread_fwrite(
        this: *mut XWriteThread,
        ptr: *const c_void,
        size: usize,
        nitems: usize,
        stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    #[link_name = "\u{1}fseek"]
    pub fn XWriteThread_fseek(
        this: *mut XWriteThread,
        stream: *mut FILE,
        offset: c_long,
        whence: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}fflush"]
    pub fn XWriteThread_fflush(this: *mut XWriteThread, stream: *mut FILE) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}fclose"]
    pub fn XWriteThread_fclose(this: *mut XWriteThread, stream: *mut FILE) -> c_int;
}
///! DO NOT DELETE INSTANCES OF XWriteThread
///! instead call shutdown and they will be cleaned up in the background
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn XWriteThread_shutdown(this: *mut XWriteThread);
}
extern "C" {
    #[link_name = "\u{1}XWriteThread"]
    pub fn XWriteThread_XWriteThread(
        this: *mut XWriteThread,
        data_buffer_size: usize,
        msg_buffer_size: usize,
    );
}
impl XWriteThread {
    #[inline]
    pub unsafe fn shared() -> *mut XWriteThread {
        XWriteThread_shared()
    }
    #[inline]
    pub unsafe fn fwrite(
        &mut self,
        ptr: *const c_void,
        size: usize,
        nitems: usize,
        stream: *mut FILE,
    ) -> usize {
        XWriteThread_fwrite(self, ptr, size, nitems, stream)
    }
    #[inline]
    pub unsafe fn fseek(&mut self, stream: *mut FILE, offset: c_long, whence: c_int) -> c_int {
        XWriteThread_fseek(self, stream, offset, whence)
    }
    #[inline]
    pub unsafe fn fflush(&mut self, stream: *mut FILE) -> c_int {
        XWriteThread_fflush(self, stream)
    }
    #[inline]
    pub unsafe fn fclose(&mut self, stream: *mut FILE) -> c_int {
        XWriteThread_fclose(self, stream)
    }
    ///! DO NOT DELETE INSTANCES OF XWriteThread
    ///! instead call shutdown and they will be cleaned up in the background
    #[inline]
    pub unsafe fn shutdown(&mut self) {
        XWriteThread_shutdown(self)
    }
    #[inline]
    pub unsafe fn new(data_buffer_size: usize, msg_buffer_size: usize) -> Self {
        let mut __bindgen_tmp = uninitialized();
        XWriteThread_XWriteThread(&mut __bindgen_tmp, data_buffer_size, msg_buffer_size);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}
