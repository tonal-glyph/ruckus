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
//* On-the-fly programming utilities
#![feature(libc)]
use libc::*;
use crate::ck::def::defe::*;
use crate::ck::util::net::nete::*;
use crate::dts::*;
use crate::sys::*;
use std::marker::PhantomData;
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> Copy for __IncompleteArrayField<T> {}

pub const NET_HEADER: u32 = 2358036680;
pub const NET_BUFFER_SIZE: u32 = 512;
pub const NET_ERROR: u32 = 4294967295;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Compiler {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Net_Msg {
    pub header: c_ulong,
    pub type_: c_ulong,
    pub param: c_ulong,
    pub param2: c_ulong,
    pub param3: c_ulong,
    pub length: c_ulong,
    pub buffer: [c_char; 512usize],
}
extern "C" {
    #[link_name = "\u{1}_Z8otf_htonP7Net_Msg"]
    pub fn otf_hton(msg: *mut Net_Msg);
}
extern "C" {
    #[link_name = "\u{1}_Z8otf_ntohP7Net_Msg"]
    pub fn otf_ntoh(msg: *mut Net_Msg);
}
extern "C" {
    #[link_name = "\u{1}_Z15otf_process_msgP8Chuck_VMP14Chuck_CompilerP7Net_MsgmPv"]
    pub fn otf_process_msg(
        vm: *mut Chuck_VM,
        compiler: *mut Chuck_Compiler,
        msg: *mut Net_Msg,
        immediate: c_ulong,
        data: *mut c_void,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_Z12otf_send_cmdiPPKcRlS0_iPi"]
    pub fn otf_send_cmd(
        argc: c_int,
        argv: *mut *const c_char,
        i: *mut c_long,
        host: *const c_char,
        port: c_int,
        is_otf: *mut c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13otf_send_filePKcR7Net_MsgS0_P10ck_socket_"]
    pub fn otf_send_file(
        filename: *const c_char,
        msg: *mut Net_Msg,
        op: *const c_char,
        sock: ck_socket,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z16otf_send_connectPKci"]
    pub fn otf_send_connect(host: *const c_char, port: c_int) -> ck_socket;
}
extern "C" {
    #[link_name = "\u{1}_Z6otf_cbPv"]
    pub fn otf_cb(p: *mut c_void) -> *mut c_void;
}
extern "C" {
    #[link_name = "\u{1}_Z2uhv"]
    pub fn uh();
}
extern "C" {
    #[link_name = "\u{1}poop"]
    pub static mut poop: [*const c_char; 0usize];
}
extern "C" {
    #[link_name = "\u{1}poop_size"]
    pub static mut poop_size: c_long;
}
extern "C" {
    #[link_name = "\u{1}g_otf_log"]
    pub static mut g_otf_log: c_ulong;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
