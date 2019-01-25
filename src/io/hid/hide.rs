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
use crate::util_buffers_h_edited::*;
use crate::util_hid_h_edited::*;
use crate::util_thread_h_edited::*;
///* HID io over SDL header
use libc::*;
pub const CK_MAX_HID_DEVICES: u32 = 1024;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhyHidDevIn {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhyHidDevOut {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct HidOut {
    pub phout: *mut PhyHidDevOut,
    pub m_msg: vector,
    pub m_device_num: c_ulong,
    pub m_valid: c_ulong,
    pub m_suppress_output: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn HidOut_open(this: *mut HidOut, device_num: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn HidOut_close(this: *mut HidOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn HidOut_good(this: *mut HidOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}num"]
    pub fn HidOut_num(this: *mut HidOut) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_suppress"]
    pub fn HidOut_set_suppress(this: *mut HidOut, print_or_not: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}get_suppress"]
    pub fn HidOut_get_suppress(this: *mut HidOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}send"]
    pub fn HidOut_send(this: *mut HidOut, msg: *const HidMsg) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}HidOut"]
    pub fn HidOut_HidOut(this: *mut HidOut);
}
extern "C" {
    #[link_name = "\u{1}HidOut_destructor"]
    pub fn HidOut_HidOut_destructor(this: *mut HidOut);
}
impl HidOut {
    #[inline]
    pub unsafe fn open(&mut self, device_num: c_ulong) -> c_ulong {
        HidOut_open(self, device_num)
    }
    #[inline]
    pub unsafe fn close(&mut self) -> c_ulong {
        HidOut_close(self)
    }
    #[inline]
    pub unsafe fn good(&mut self) -> c_ulong {
        HidOut_good(self)
    }
    #[inline]
    pub unsafe fn num(&mut self) -> c_long {
        HidOut_num(self)
    }
    #[inline]
    pub unsafe fn set_suppress(&mut self, print_or_not: c_ulong) {
        HidOut_set_suppress(self, print_or_not)
    }
    #[inline]
    pub unsafe fn get_suppress(&mut self) -> c_ulong {
        HidOut_get_suppress(self)
    }
    #[inline]
    pub unsafe fn send(&mut self, msg: *const HidMsg) -> c_ulong {
        HidOut_send(self, msg)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        HidOut_HidOut(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        HidOut_HidOut_destructor(self)
    }
}
#[repr(C)]
pub struct HidIn {
    pub _base: Chuck_Event,
    pub phin: *mut PhyHidDevIn,
    pub m_buffer: *mut CBufferAdvance,
    pub m_read_index: c_ulong,
    pub m_valid: c_ulong,
    pub m_device_num: c_long,
    pub SELF: *mut Chuck_Object,
    pub m_suppress_output: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn HidIn_open(
        this: *mut HidIn,
        vm: *mut Chuck_VM,
        device_type: c_long,
        device_num: c_long,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn HidIn_open1(
        this: *mut HidIn,
        vm: *mut Chuck_VM,
        name: *mut string,
        device_type: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn HidIn_close(this: *mut HidIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}read"]
    pub fn HidIn_read(this: *mut HidIn, type_: c_long, num: c_long, msg: *mut HidMsg) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}send"]
    pub fn HidIn_send(this: *mut HidIn, msg: *const HidMsg) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn HidIn_good(this: *mut HidIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}num"]
    pub fn HidIn_num(this: *mut HidIn) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_suppress"]
    pub fn HidIn_set_suppress(this: *mut HidIn, print_or_not: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}get_suppress"]
    pub fn HidIn_get_suppress(this: *mut HidIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}empty"]
    pub fn HidIn_empty(this: *mut HidIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}recv"]
    pub fn HidIn_recv(this: *mut HidIn, msg: *mut HidMsg) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}name"]
    pub fn HidIn_name(this: *mut HidIn) -> string;
}
extern "C" {
    #[link_name = "\u{1}HidIn"]
    pub fn HidIn_HidIn(this: *mut HidIn);
}
impl HidIn {
    #[inline]
    pub unsafe fn open(
        &mut self,
        vm: *mut Chuck_VM,
        device_type: c_long,
        device_num: c_long,
    ) -> c_ulong {
        HidIn_open(self, vm, device_type, device_num)
    }
    #[inline]
    pub unsafe fn open1(
        &mut self,
        vm: *mut Chuck_VM,
        name: *mut string,
        device_type: c_ulong,
    ) -> c_ulong {
        HidIn_open1(self, vm, name, device_type)
    }
    #[inline]
    pub unsafe fn close(&mut self) -> c_ulong {
        HidIn_close(self)
    }
    #[inline]
    pub unsafe fn read(&mut self, type_: c_long, num: c_long, msg: *mut HidMsg) -> c_ulong {
        HidIn_read(self, type_, num, msg)
    }
    #[inline]
    pub unsafe fn send(&mut self, msg: *const HidMsg) -> c_ulong {
        HidIn_send(self, msg)
    }
    #[inline]
    pub unsafe fn good(&mut self) -> c_ulong {
        HidIn_good(self)
    }
    #[inline]
    pub unsafe fn num(&mut self) -> c_long {
        HidIn_num(self)
    }
    #[inline]
    pub unsafe fn set_suppress(&mut self, print_or_not: c_ulong) {
        HidIn_set_suppress(self, print_or_not)
    }
    #[inline]
    pub unsafe fn get_suppress(&mut self) -> c_ulong {
        HidIn_get_suppress(self)
    }
    #[inline]
    pub unsafe fn empty(&mut self) -> c_ulong {
        HidIn_empty(self)
    }
    #[inline]
    pub unsafe fn recv(&mut self, msg: *mut HidMsg) -> c_ulong {
        HidIn_recv(self, msg)
    }
    #[inline]
    pub unsafe fn name(&mut self) -> string {
        HidIn_name(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        HidIn_HidIn(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}HidIn_destructor"]
    pub fn HidIn_HidIn_destructor(this: *mut HidIn);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HidInManager {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}m_event_buffers"]
    pub static mut HidInManager_m_event_buffers: map;
}
extern "C" {
    #[link_name = "\u{1}the_matrix"]
    pub static mut HidInManager_the_matrix: vector;
}
extern "C" {
    #[link_name = "\u{1}the_thread"]
    pub static mut HidInManager_the_thread: *mut XThread;
}
extern "C" {
    #[link_name = "\u{1}msg_buffer"]
    pub static mut HidInManager_msg_buffer: *mut CBufferSimple;
}
extern "C" {
    #[link_name = "\u{1}thread_going"]
    pub static mut HidInManager_thread_going: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}has_init"]
    pub static mut HidInManager_has_init: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn HidInManager_init();
}
extern "C" {
    #[link_name = "\u{1}init_default_drivers"]
    pub fn HidInManager_init_default_drivers();
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn HidInManager_cleanup();
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn HidInManager_open(
        hin: *mut HidIn,
        vm: *mut Chuck_VM,
        device_type: c_long,
        device_num: c_long,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn HidInManager_open1(
        hin: *mut HidIn,
        vm: *mut Chuck_VM,
        device_type: c_long,
        device_name: *mut string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn HidInManager_close(hin: *mut HidIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup_buffer"]
    pub fn HidInManager_cleanup_buffer(vm: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}probeHidIn"]
    pub fn HidInManager_probeHidIn();
}
extern "C" {
    #[link_name = "\u{1}probeHidOut"]
    pub fn HidInManager_probeHidOut();
}
extern "C" {
    #[link_name = "\u{1}cb_hid_input"]
    pub fn HidInManager_cb_hid_input(arg1: *mut c_void) -> *mut c_void;
}
extern "C" {
    #[link_name = "\u{1}push_message"]
    pub fn HidInManager_push_message(msg: *mut HidMsg);
}
impl HidInManager {
    #[inline]
    pub unsafe fn init() {
        HidInManager_init()
    }
    #[inline]
    pub unsafe fn init_default_drivers() {
        HidInManager_init_default_drivers()
    }
    #[inline]
    pub unsafe fn cleanup() {
        HidInManager_cleanup()
    }
    #[inline]
    pub unsafe fn open(
        hin: *mut HidIn,
        vm: *mut Chuck_VM,
        device_type: c_long,
        device_num: c_long,
    ) -> c_ulong {
        HidInManager_open(hin, vm, device_type, device_num)
    }
    #[inline]
    pub unsafe fn open1(
        hin: *mut HidIn,
        vm: *mut Chuck_VM,
        device_type: c_long,
        device_name: *mut string,
    ) -> c_ulong {
        HidInManager_open1(hin, vm, device_type, device_name)
    }
    #[inline]
    pub unsafe fn close(hin: *mut HidIn) -> c_ulong {
        HidInManager_close(hin)
    }
    #[inline]
    pub unsafe fn cleanup_buffer(vm: *mut Chuck_VM) {
        HidInManager_cleanup_buffer(vm)
    }
    #[inline]
    pub unsafe fn probeHidIn() {
        HidInManager_probeHidIn()
    }
    #[inline]
    pub unsafe fn probeHidOut() {
        HidInManager_probeHidOut()
    }
    #[inline]
    pub unsafe fn cb_hid_input(arg1: *mut c_void) -> *mut c_void {
        HidInManager_cb_hid_input(arg1)
    }
    #[inline]
    pub unsafe fn push_message(msg: *mut HidMsg) {
        HidInManager_push_message(msg)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct HidOutManager {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}the_phouts"]
    pub static mut HidOutManager_the_phouts: vector;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn HidOutManager_open(hout: *mut HidOut, device_num: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn HidOutManager_close(hout: *mut HidOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}HidOutManager"]
    pub fn HidOutManager_HidOutManager(this: *mut HidOutManager);
}
extern "C" {
    #[link_name = "\u{1}HidOutManager_destructor"]
    pub fn HidOutManager_HidOutManager_destructor(this: *mut HidOutManager);
}
impl HidOutManager {
    #[inline]
    pub unsafe fn open(hout: *mut HidOut, device_num: c_long) -> c_ulong {
        HidOutManager_open(hout, device_num)
    }
    #[inline]
    pub unsafe fn close(hout: *mut HidOut) -> c_ulong {
        HidOutManager_close(hout)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        HidOutManager_HidOutManager(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        HidOutManager_HidOutManager_destructor(self)
    }
}
extern "C" {
    pub static mut default_drivers: *mut Chuck_Hid_Driver;
}
