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
use crate::chuck_oo_h_edited::*;
use crate::util_buffers_h_edited::*;
use crate::util_thread_h_edited::*;
///* standard class library
use libc::*;
pub fn DLLQUERY() {
    libstd_query(QUERY: *mut Chuck_DL_Query);
}
pub fn main() {
    DLLQUERY();
    // exports
    CK_DLL_SFUN(abs_impl);
    CK_DLL_SFUN(fabs_impl);
    CK_DLL_SFUN(rand_impl);
    CK_DLL_SFUN(randf_impl);
    CK_DLL_SFUN(rand2f_impl);
    CK_DLL_SFUN(rand2_impl);
    CK_DLL_SFUN(srand_impl);
    CK_DLL_SFUN(sgn_impl);
    CK_DLL_SFUN(system_impl);
    CK_DLL_SFUN(atoi_impl);
    CK_DLL_SFUN(atof_impl);
    CK_DLL_SFUN(itoa_impl);
    CK_DLL_SFUN(ftoa_impl);
    CK_DLL_SFUN(ftoi_impl);
    CK_DLL_SFUN(getenv_impl);
    CK_DLL_SFUN(setenv_impl);
    CK_DLL_SFUN(mtof_impl);
    CK_DLL_SFUN(ftom_impl);
    CK_DLL_SFUN(powtodb_impl);
    CK_DLL_SFUN(rmstodb_impl);
    CK_DLL_SFUN(dbtopow_impl);
    CK_DLL_SFUN(dbtorms_impl);
    CK_DLL_SFUN(dbtolin_impl);
    CK_DLL_SFUN(lintodb_impl);
    CK_DLL_SFUN(clamp_impl);
    CK_DLL_SFUN(clampf_impl);
    CK_DLL_SFUN(scalef_impl);
}
#[repr(C)]
pub struct KBHit {
    pub _base: Chuck_Event,
    pub m_buffer: *mut CBufferAdvance,
    pub m_read_index: c_ulong,
    pub m_valid: c_ulong,
    pub m_onoff: c_ulong,
    pub m_echo: c_ulong,
    pub SELF: *mut Chuck_Object,
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn KBHit_open(this: *mut KBHit) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn KBHit_close(this: *mut KBHit) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}on"]
    pub fn KBHit_on(this: *mut KBHit);
}
extern "C" {
    #[link_name = "\u{1}off"]
    pub fn KBHit_off(this: *mut KBHit);
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn KBHit_good(this: *mut KBHit) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}state"]
    pub fn KBHit_state(this: *mut KBHit) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_echo"]
    pub fn KBHit_set_echo(this: *mut KBHit, echo: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}get_echo"]
    pub fn KBHit_get_echo(this: *mut KBHit) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}empty"]
    pub fn KBHit_empty(this: *mut KBHit) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}getch"]
    pub fn KBHit_getch(this: *mut KBHit) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}KBHit"]
    pub fn KBHit_KBHit(this: *mut KBHit);
}
impl KBHit {
    #[inline]
    pub unsafe fn open(&mut self) -> c_ulong {
        KBHit_open(self)
    }
    #[inline]
    pub unsafe fn close(&mut self) -> c_ulong {
        KBHit_close(self)
    }
    #[inline]
    pub unsafe fn on(&mut self) {
        KBHit_on(self)
    }
    #[inline]
    pub unsafe fn off(&mut self) {
        KBHit_off(self)
    }
    #[inline]
    pub unsafe fn good(&mut self) -> c_ulong {
        KBHit_good(self)
    }
    #[inline]
    pub unsafe fn state(&mut self) -> c_ulong {
        KBHit_state(self)
    }
    #[inline]
    pub unsafe fn set_echo(&mut self, echo: c_ulong) {
        KBHit_set_echo(self, echo)
    }
    #[inline]
    pub unsafe fn get_echo(&mut self) -> c_ulong {
        KBHit_get_echo(self)
    }
    #[inline]
    pub unsafe fn empty(&mut self) -> c_ulong {
        KBHit_empty(self)
    }
    #[inline]
    pub unsafe fn getch(&mut self) -> c_long {
        KBHit_getch(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        KBHit_KBHit(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}KBHit_destructor"]
    pub fn KBHit_KBHit_destructor(this: *mut KBHit);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KBHitManager {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}the_buf"]
    pub static mut KBHitManager_the_buf: *mut CBufferAdvance;
}
extern "C" {
    #[link_name = "\u{1}the_onoff"]
    pub static mut KBHitManager_the_onoff: c_long;
}
extern "C" {
    #[link_name = "\u{1}the_init"]
    pub static mut KBHitManager_the_init: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}the_thread"]
    pub static mut KBHitManager_the_thread: *mut XThread;
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn KBHitManager_init() -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn KBHitManager_shutdown();
}
extern "C" {
    #[link_name = "\u{1}on"]
    pub fn KBHitManager_on();
}
extern "C" {
    #[link_name = "\u{1}off"]
    pub fn KBHitManager_off();
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn KBHitManager_open(kb: *mut KBHit) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn KBHitManager_close(kb: *mut KBHit) -> c_ulong;
}
impl KBHitManager {
    #[inline]
    pub unsafe fn init() -> c_ulong {
        KBHitManager_init()
    }
    #[inline]
    pub unsafe fn shutdown() {
        KBHitManager_shutdown()
    }
    #[inline]
    pub unsafe fn on() {
        KBHitManager_on()
    }
    #[inline]
    pub unsafe fn off() {
        KBHitManager_off()
    }
    #[inline]
    pub unsafe fn open(kb: *mut KBHit) -> c_ulong {
        KBHitManager_open(kb)
    }
    #[inline]
    pub unsafe fn close(kb: *mut KBHit) -> c_ulong {
        KBHitManager_close(kb)
    }
}
