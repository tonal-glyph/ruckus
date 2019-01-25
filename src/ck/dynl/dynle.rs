//! ChucK Dynamic Linking module. Externals (ckx files) can be loaded dynamically
//! and invoked from the language. This allows for algorithms implemented in other
//! languages (C/C++ etc.) to be used from ChucK. This includes a interface-querying
//! mechanism for determining types.
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
use crate::libc::*;
use crate::libc::c_char;
use crate::dts::*;
use crate::chuck_absyn_h_edited::*;
use crate::chuck_instr_h_edited::*;
use crate::chuck_lang_h_edited::*;
use crate::chuck_oo_h_edited::*;
use crate::chuck_ugen_h_edited::*;
use crate::chuck_vm_h_edited::*;
pub const CHUCK_ARRAY4_DATAKIND: u32 = 1;
pub const CHUCK_ARRAY8_DATAKIND: u32 = 2;
pub const CHUCK_ARRAY16_DATAKIND: u32 = 3;
pub const CHUCK_ARRAY24_DATAKIND: u32 = 4;
pub const CHUCK_ARRAY32_DATAKIND: u32 = 5;
pub const CK_DLL_VERSION_MAJOR: u32 = 0x0007;
pub const CK_DLL_VERSION_MINOR: u32 = 0x0000;
pub const CK_QUERY_FUNC: &'static [u8; 9usize] = b"ck_query\0";
pub const CK_DECLVERSION_FUNC: &'static [u8; 11usize] = b"ck_version\0";
pub const CK_INVALID_OFFSET: u32 = 0xffffffff;
pub const _DLFCN_H: u32 = 1;
pub const RTLD_LAZY: u32 = 1;
pub const RTLD_NOW: u32 = 2;
pub const RTLD_BINDING_MASK: u32 = 3;
pub const RTLD_NOLOAD: u32 = 4;
pub const RTLD_DEEPBIND: u32 = 8;
pub const RTLD_GLOBAL: u32 = 256;
pub const RTLD_LOCAL: u32 = 0;
pub const RTLD_NODELETE: u32 = 4096;
pub const LM_ID_BASE: u32 = 0;
pub const LM_ID_NEWLM: i32 = -1;
pub type c_str = *mut c_char;
pub type c_constr = *const c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ChucK {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Compiler {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Env {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_socket_ {
    _unused: [u8; 0],
}
pub type ck_socket = *mut ck_socket_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WvOut {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_Carrier {
    pub chuck: *mut ChucK,
    pub compiler: *mut Chuck_Compiler,
    pub env: *mut Chuck_Env,
    pub vm: *mut Chuck_VM,
    pub chout: *mut Chuck_IO_Chout,
    pub cherr: *mut Chuck_IO_Cherr,
    pub otf_socket: ck_socket,
    pub otf_port: c_long,
    pub otf_thread: pthread_t,
    pub stk_writeThread: *mut XWriteThread,
    pub stk_wvOutMap: std::collections::HashMap,
}
extern "C" {
    #[link_name = "\u{1}hintIsRealtimeAudio"]
    pub fn Chuck_Carrier_hintIsRealtimeAudio(this: *mut Chuck_Carrier) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Carrier"]
    pub fn Chuck_Carrier_Chuck_Carrier(this: *mut Chuck_Carrier);
}
impl Chuck_Carrier {
    #[inline]
    pub unsafe fn hintIsRealtimeAudio(&mut self) -> c_ulong {
        Chuck_Carrier_hintIsRealtimeAudio(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Carrier_Chuck_Carrier(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Type {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Value {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Func {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Namespace {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_Code {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_Shred {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CBufferSimple {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_VM_Object__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_VM_Object {
    pub vtable_: *const Chuck_VM_Object__bindgen_vtable,
    pub m_ref_count: c_ulong,
    pub m_pooled: c_ulong,
    pub m_locked: c_ulong,
    pub m_v_ref: *mut Vec::new(Fn),
}
extern "C" {
    #[link_name = "\u{1}our_locks_in_effect"]
    pub static mut Chuck_VM_Object_our_locks_in_effect: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}lock_all"]
    pub fn Chuck_VM_Object_lock_all();
}
extern "C" {
    #[link_name = "\u{1}unlock_all"]
    pub fn Chuck_VM_Object_unlock_all();
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Object"]
    pub fn Chuck_VM_Object_Chuck_VM_Object(this: *mut Chuck_VM_Object);
}
impl Chuck_VM_Object {
    #[inline]
    pub unsafe fn lock_all() {
        Chuck_VM_Object_lock_all()
    }
    #[inline]
    pub unsafe fn unlock_all() {
        Chuck_VM_Object_unlock_all()
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Object_Chuck_VM_Object(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Object_destructor"]
    pub fn Chuck_VM_Object_Chuck_VM_Object_destructor(this: *mut Chuck_VM_Object);
}
extern "C" {
    #[link_name = "\u{1}add_ref"]
    pub fn Chuck_VM_Object_add_ref(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}release"]
    pub fn Chuck_VM_Object_release(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}lock"]
    pub fn Chuck_VM_Object_lock(this: *mut c_void);
}
#[repr(C)]
pub struct Chuck_VTable {
    pub funcs: Vec::new(Fn),
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Object {
    pub _base: Chuck_VM_Object,
    pub vtable: *mut Chuck_VTable,
    pub type_ref: *mut Chuck_Type,
    pub size: c_ulong,
    pub data: *mut c_uchar,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Object"]
    pub fn Chuck_Object_Chuck_Object(this: *mut Chuck_Object);
}
impl Chuck_Object {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Object_Chuck_Object(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Object_destructor"]
    pub fn Chuck_Object_Chuck_Object_destructor(this: *mut Chuck_Object);
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Array {
    pub _base: Chuck_Object,
    pub m_array_type: *mut Chuck_Type,
}
#[repr(C)]
pub struct Chuck_Array4 {
    pub _base: Chuck_Array,
    pub m_vector: Vec::new(),
    pub m_map: std::collections::HashMap,
    pub m_is_obj: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr(this: *mut Chuck_Array4, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr1(
        this: *mut Chuck_Array4,
        key: *const crate::dts::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get(this: *mut Chuck_Array4, i: c_long, val: *mut c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get1(
        this: *mut Chuck_Array4,
        key: *const crate::dts::string,
        val: *mut c_ulong,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set(this: *mut Chuck_Array4, i: c_long, val: c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set1(
        this: *mut Chuck_Array4,
        key: *const crate::dts::string,
        val: c_ulong,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array4_push_back(this: *mut Chuck_Array4, val: c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array4_pop_back(this: *mut Chuck_Array4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array4_back(this: *const Chuck_Array4, val: *mut c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array4_zero(this: *mut Chuck_Array4, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4"]
    pub fn Chuck_Array4_Chuck_Array4(this: *mut Chuck_Array4, is_obj: c_ulong, capacity: c_long);
}
impl Chuck_Array4 {
    #[inline]
    pub unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array4_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> c_ulong {
        Chuck_Array4_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: c_long, val: *mut c_ulong) -> c_long {
        Chuck_Array4_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut c_ulong,
    ) -> c_long {
        Chuck_Array4_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: c_long, val: c_ulong) -> c_long {
        Chuck_Array4_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: c_ulong,
    ) -> c_long {
        Chuck_Array4_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: c_ulong) -> c_long {
        Chuck_Array4_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array4_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut c_ulong) -> c_long {
        Chuck_Array4_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array4_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(is_obj: c_ulong, capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array4_Chuck_Array4(&mut __bindgen_tmp, is_obj, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4_destructor"]
    pub fn Chuck_Array4_Chuck_Array4_destructor(this: *mut Chuck_Array4);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array4_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array4_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array4_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array4_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array4_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array4_find(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array4_erase(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array4_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array4_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
pub struct Chuck_Array8 {
    pub _base: Chuck_Array,
    pub m_vector: Vec::new(),
    pub m_map: std::collections::HashMap,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array8_addr(this: *mut Chuck_Array8, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array8_addr1(
        this: *mut Chuck_Array8,
        key: *const crate::dts::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array8_get(this: *mut Chuck_Array8, i: c_long, val: *mut f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array8_get1(
        this: *mut Chuck_Array8,
        key: *const crate::dts::string,
        val: *mut f64,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array8_set(this: *mut Chuck_Array8, i: c_long, val: f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array8_set1(
        this: *mut Chuck_Array8,
        key: *const crate::dts::string,
        val: f64,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array8_push_back(this: *mut Chuck_Array8, val: f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array8_pop_back(this: *mut Chuck_Array8) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array8_back(this: *const Chuck_Array8, val: *mut f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array8_zero(this: *mut Chuck_Array8, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array8"]
    pub fn Chuck_Array8_Chuck_Array8(this: *mut Chuck_Array8, capacity: c_long);
}
impl Chuck_Array8 {
    #[inline]
    pub unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array8_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> c_ulong {
        Chuck_Array8_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: c_long, val: *mut f64) -> c_long {
        Chuck_Array8_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(&mut self, key: *const crate::dts::string, val: *mut f64) -> c_long {
        Chuck_Array8_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: c_long, val: f64) -> c_long {
        Chuck_Array8_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(&mut self, key: *const crate::dts::string, val: f64) -> c_long {
        Chuck_Array8_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: f64) -> c_long {
        Chuck_Array8_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array8_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut f64) -> c_long {
        Chuck_Array8_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array8_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array8_Chuck_Array8(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array8_destructor"]
    pub fn Chuck_Array8_Chuck_Array8_destructor(this: *mut Chuck_Array8);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array8_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array8_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array8_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array8_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array8_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array8_find(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array8_erase(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array8_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array8_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
pub struct Chuck_Array16 {
    pub _base: Chuck_Array,
    pub m_vector: Vec::new(),
    pub m_map: std::collections::HashMap,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array16_addr(this: *mut Chuck_Array16, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array16_addr1(
        this: *mut Chuck_Array16,
        key: *const crate::dts::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array16_get(
        this: *mut Chuck_Array16,
        i: c_long,
        val: *mut crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array16_get1(
        this: *mut Chuck_Array16,
        key: *const crate::dts::string,
        val: *mut crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array16_set(
        this: *mut Chuck_Array16,
        i: c_long,
        val: *const crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array16_set1(
        this: *mut Chuck_Array16,
        key: *const crate::dts::string,
        val: *const crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array16_push_back(
        this: *mut Chuck_Array16,
        val: *const crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array16_pop_back(this: *mut Chuck_Array16) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array16_back(
        this: *const Chuck_Array16,
        val: *mut crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array16_zero(this: *mut Chuck_Array16, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array16"]
    pub fn Chuck_Array16_Chuck_Array16(this: *mut Chuck_Array16, capacity: c_long);
}
impl Chuck_Array16 {
    #[inline]
    pub unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array16_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> c_ulong {
        Chuck_Array16_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        i: c_long,
        val: *mut crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long {
        Chuck_Array16_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long {
        Chuck_Array16_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(
        &mut self,
        i: c_long,
        val: *const crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long {
        Chuck_Array16_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: *const crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long {
        Chuck_Array16_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(
        &mut self,
        val: *const crate::chuck_def_h_edited::t_CKCOMPLEX,
    ) -> c_long {
        Chuck_Array16_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array16_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut crate::chuck_def_h_edited::t_CKCOMPLEX) -> c_long {
        Chuck_Array16_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array16_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array16_Chuck_Array16(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array16_destructor"]
    pub fn Chuck_Array16_Chuck_Array16_destructor(this: *mut Chuck_Array16);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array16_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array16_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array16_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array16_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array16_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array16_find(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array16_erase(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array16_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array16_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
pub struct Chuck_Array24 {
    pub _base: Chuck_Array,
    pub m_vector: Vec::new(),
    pub m_map: std::collections::HashMap,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array24_addr(this: *mut Chuck_Array24, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array24_addr1(
        this: *mut Chuck_Array24,
        key: *const crate::dts::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array24_get(
        this: *mut Chuck_Array24,
        i: c_long,
        val: *mut crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array24_get1(
        this: *mut Chuck_Array24,
        key: *const crate::dts::string,
        val: *mut crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array24_set(
        this: *mut Chuck_Array24,
        i: c_long,
        val: *const crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array24_set1(
        this: *mut Chuck_Array24,
        key: *const crate::dts::string,
        val: *const crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array24_push_back(
        this: *mut Chuck_Array24,
        val: *const crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array24_pop_back(this: *mut Chuck_Array24) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array24_back(
        this: *const Chuck_Array24,
        val: *mut crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array24_zero(this: *mut Chuck_Array24, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array24"]
    pub fn Chuck_Array24_Chuck_Array24(this: *mut Chuck_Array24, capacity: c_long);
}
impl Chuck_Array24 {
    #[inline]
    pub unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array24_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> c_ulong {
        Chuck_Array24_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        i: c_long,
        val: *mut crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long {
        Chuck_Array24_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long {
        Chuck_Array24_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(
        &mut self,
        i: c_long,
        val: *const crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long {
        Chuck_Array24_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: *const crate::chuck_def_h_edited::t_CKVEC3,
    ) -> c_long {
        Chuck_Array24_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const crate::chuck_def_h_edited::t_CKVEC3) -> c_long {
        Chuck_Array24_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array24_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut crate::chuck_def_h_edited::t_CKVEC3) -> c_long {
        Chuck_Array24_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array24_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array24_Chuck_Array24(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array24_destructor"]
    pub fn Chuck_Array24_Chuck_Array24_destructor(this: *mut Chuck_Array24);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array24_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array24_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array24_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array24_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array24_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array24_find(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array24_erase(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array24_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array24_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
pub struct Chuck_Array32 {
    pub _base: Chuck_Array,
    pub m_vector: Vec::new(),
    pub m_map: std::collections::HashMap,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array32_addr(this: *mut Chuck_Array32, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array32_addr1(
        this: *mut Chuck_Array32,
        key: *const crate::dts::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array32_get(
        this: *mut Chuck_Array32,
        i: c_long,
        val: *mut crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array32_get1(
        this: *mut Chuck_Array32,
        key: *const crate::dts::string,
        val: *mut crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array32_set(
        this: *mut Chuck_Array32,
        i: c_long,
        val: *const crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array32_set1(
        this: *mut Chuck_Array32,
        key: *const crate::dts::string,
        val: *const crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array32_push_back(
        this: *mut Chuck_Array32,
        val: *const crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array32_pop_back(this: *mut Chuck_Array32) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array32_back(
        this: *const Chuck_Array32,
        val: *mut crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array32_zero(this: *mut Chuck_Array32, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array32"]
    pub fn Chuck_Array32_Chuck_Array32(this: *mut Chuck_Array32, capacity: c_long);
}
impl Chuck_Array32 {
    #[inline]
    pub unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array32_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> c_ulong {
        Chuck_Array32_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        i: c_long,
        val: *mut crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long {
        Chuck_Array32_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long {
        Chuck_Array32_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(
        &mut self,
        i: c_long,
        val: *const crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long {
        Chuck_Array32_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: *const crate::chuck_def_h_edited::t_CKVEC4,
    ) -> c_long {
        Chuck_Array32_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const crate::chuck_def_h_edited::t_CKVEC4) -> c_long {
        Chuck_Array32_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array32_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut crate::chuck_def_h_edited::t_CKVEC4) -> c_long {
        Chuck_Array32_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array32_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array32_Chuck_Array32(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array32_destructor"]
    pub fn Chuck_Array32_Chuck_Array32_destructor(this: *mut Chuck_Array32);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array32_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array32_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array32_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array32_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array32_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array32_find(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array32_erase(
        this: *mut c_void,
        key: *const crate::dts::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array32_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array32_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
pub struct Chuck_Event {
    pub _base: Chuck_Object,
    pub m_queue: crate::dts::deque<std::collections::VecDeque>,
    pub m_queue_lock: XMutex,
}
extern "C" {
    #[link_name = "\u{1}our_can_wait"]
    pub static mut Chuck_Event_our_can_wait: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signal"]
    pub fn Chuck_Event_signal(this: *mut Chuck_Event);
}
extern "C" {
    #[link_name = "\u{1}broadcast"]
    pub fn Chuck_Event_broadcast(this: *mut Chuck_Event);
}
extern "C" {
    #[link_name = "\u{1}wait"]
    pub fn Chuck_Event_wait(this: *mut Chuck_Event, shred: *mut Chuck_VM_Shred, vm: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_Event_remove(this: *mut Chuck_Event, shred: *mut Chuck_VM_Shred) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_broadcast"]
    pub fn Chuck_Event_queue_broadcast(this: *mut Chuck_Event, event_buffer: *mut CBufferSimple);
}
impl Chuck_Event {
    #[inline]
    pub unsafe fn signal(&mut self) {
        Chuck_Event_signal(self)
    }
    #[inline]
    pub unsafe fn broadcast(&mut self) {
        Chuck_Event_broadcast(self)
    }
    #[inline]
    pub unsafe fn wait(&mut self, shred: *mut Chuck_VM_Shred, vm: *mut Chuck_VM) {
        Chuck_Event_wait(self, shred, vm)
    }
    #[inline]
    pub unsafe fn remove(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_Event_remove(self, shred)
    }
    #[inline]
    pub unsafe fn queue_broadcast(&mut self, event_buffer: *mut CBufferSimple) {
        Chuck_Event_queue_broadcast(self, event_buffer)
    }
}
#[repr(C)]
pub struct Chuck_String {
    pub _base: Chuck_Object,
    pub m_charptr: *const c_char,
    pub m_str: crate::dts::string,
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_String_set(this: *mut Chuck_String, s: *const crate::dts::string);
}
extern "C" {
    #[link_name = "\u{1}str"]
    pub fn Chuck_String_str(this: *mut Chuck_String) -> *const crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}c_str"]
    pub fn Chuck_String_c_str(this: *mut Chuck_String) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_String"]
    pub fn Chuck_String_Chuck_String(
        this: *mut Chuck_String,
        s: *const crate::dts::string,
    );
}
impl Chuck_String {
    #[inline]
    pub unsafe fn set(&mut self, s: *const crate::dts::string) {
        Chuck_String_set(self, s)
    }
    #[inline]
    pub unsafe fn str(&mut self) -> *const crate::dts::string {
        Chuck_String_str(self)
    }
    #[inline]
    pub unsafe fn c_str(&mut self) -> *const c_char {
        Chuck_String_c_str(self)
    }
    #[inline]
    pub unsafe fn new(s: *const crate::dts::string) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_String_Chuck_String(&mut __bindgen_tmp, s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_String_destructor"]
    pub fn Chuck_String_Chuck_String_destructor(this: *mut Chuck_String);
}
#[repr(C)]
pub struct Chuck_IO {
    pub _base: Chuck_Event,
    pub m_asyncEvent: *mut Chuck_Event,
    pub m_thread: *mut XThread,
}
#[repr(C)]
pub struct Chuck_IO_async_args {
    pub fileio_obj: *mut Chuck_IO_File,
    pub RETURN: *mut c_void,
    pub intArg: c_long,
    pub floatArg: f64,
    pub stringArg: crate::dts::string,
}
extern "C" {
    #[link_name = "\u{1}INT32"]
    pub static Chuck_IO_INT32: c_long;
}
extern "C" {
    #[link_name = "\u{1}INT16"]
    pub static Chuck_IO_INT16: c_long;
}
extern "C" {
    #[link_name = "\u{1}INT8"]
    pub static Chuck_IO_INT8: c_long;
}
extern "C" {
    #[link_name = "\u{1}MODE_SYNC"]
    pub static Chuck_IO_MODE_SYNC: c_long;
}
extern "C" {
    #[link_name = "\u{1}MODE_ASYNC"]
    pub static Chuck_IO_MODE_ASYNC: c_long;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO"]
    pub fn Chuck_IO_Chuck_IO(this: *mut Chuck_IO);
}
impl Chuck_IO {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_IO_Chuck_IO(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_destructor"]
    pub fn Chuck_IO_Chuck_IO_destructor(this: *mut Chuck_IO);
}
#[repr(C)]
pub struct Chuck_IO_File {
    pub _base: Chuck_IO,
    pub m_flags: c_long,
    pub m_iomode: c_long,
    pub m_io: crate::dts::fstream(std::fs::Path::new(&str)),
    pub m_dir: *mut DIR,
    pub m_dir_start: c_long,
    pub m_path: crate::dts::string,
    pub m_vmRef: *mut Chuck_VM,
}
extern "C" {
    #[link_name = "\u{1}FLAG_READ_WRITE"]
    pub static Chuck_IO_File_FLAG_READ_WRITE: c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_READONLY"]
    pub static Chuck_IO_File_FLAG_READONLY: c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_WRITEONLY"]
    pub static Chuck_IO_File_FLAG_WRITEONLY: c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_APPEND"]
    pub static Chuck_IO_File_FLAG_APPEND: c_long;
}
extern "C" {
    #[link_name = "\u{1}TYPE_ASCII"]
    pub static Chuck_IO_File_TYPE_ASCII: c_long;
}
extern "C" {
    #[link_name = "\u{1}TYPE_BINARY"]
    pub static Chuck_IO_File_TYPE_BINARY: c_long;
}
extern "C" {
    #[link_name = "\u{1}writeStr_thread"]
    pub fn Chuck_IO_File_writeStr_thread(data: *mut c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeInt_thread"]
    pub fn Chuck_IO_File_writeInt_thread(data: *mut c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeFloat_thread"]
    pub fn Chuck_IO_File_writeFloat_thread(data: *mut c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File"]
    pub fn Chuck_IO_File_Chuck_IO_File(this: *mut Chuck_IO_File, vm: *mut Chuck_VM);
}
impl Chuck_IO_File {
    #[inline]
    pub unsafe fn writeStr_thread(data: *mut c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeStr_thread(data)
    }
    #[inline]
    pub unsafe fn writeInt_thread(data: *mut c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeInt_thread(data)
    }
    #[inline]
    pub unsafe fn writeFloat_thread(data: *mut c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeFloat_thread(data)
    }
    #[inline]
    pub unsafe fn new(vm: *mut Chuck_VM) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_IO_File_Chuck_IO_File(&mut __bindgen_tmp, vm);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File_destructor"]
    pub fn Chuck_IO_File_Chuck_IO_File_destructor(this: *mut Chuck_IO_File);
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn Chuck_IO_File_open(
        this: *mut c_void,
        path: *const crate::dts::string,
        flags: c_long,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_IO_File_good(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_File_close(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_File_flush(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_File_mode(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_File_mode1(this: *mut c_void, flag: c_long);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_IO_File_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}seek"]
    pub fn Chuck_IO_File_seek(this: *mut c_void, pos: c_long);
}
extern "C" {
    #[link_name = "\u{1}tell"]
    pub fn Chuck_IO_File_tell(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}isDir"]
    pub fn Chuck_IO_File_isDir(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}dirList"]
    pub fn Chuck_IO_File_dirList(this: *mut c_void) -> *mut Chuck_Array4;
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_File_readLine(this: *mut c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_File_readInt(this: *mut c_void, flags: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_File_readFloat(this: *mut c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_File_readString(
        this: *mut c_void,
        str: *mut crate::dts::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_File_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write(this: *mut c_void, val: *const crate::dts::string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write1(this: *mut c_void, val: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write2(this: *mut c_void, val: c_long, flags: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write3(this: *mut c_void, val: f64);
}
#[repr(C)]
pub struct Chuck_IO_Chout {
    pub _base: Chuck_IO,
    pub m_callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_buffer: crate::dts::basic_istringstream<_CharT>,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Chout_set_output_callback(
        this: *mut Chuck_IO_Chout,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout"]
    pub fn Chuck_IO_Chout_Chuck_IO_Chout(this: *mut Chuck_IO_Chout, carrier: *mut Chuck_Carrier);
}
impl Chuck_IO_Chout {
    #[inline]
    pub unsafe fn set_output_callback(
        &mut self,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) {
        Chuck_IO_Chout_set_output_callback(self, fp)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_IO_Chout_Chuck_IO_Chout(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout_destructor"]
    pub fn Chuck_IO_Chout_Chuck_IO_Chout_destructor(this: *mut Chuck_IO_Chout);
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_IO_Chout_good(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_Chout_close(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_Chout_flush(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Chout_mode(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Chout_mode1(this: *mut c_void, flag: c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_Chout_readLine(this: *mut c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_Chout_readInt(this: *mut c_void, flags: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_Chout_readFloat(this: *mut c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_Chout_readString(
        this: *mut c_void,
        str: *mut crate::dts::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Chout_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write(this: *mut c_void, val: *const crate::dts::string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write1(this: *mut c_void, val: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write2(this: *mut c_void, val: c_long, flags: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write3(this: *mut c_void, val: f64);
}
#[repr(C)]
pub struct Chuck_IO_Cherr {
    pub _base: Chuck_IO,
    pub m_callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_buffer: crate::dts::basic_istringstream<_CharT>,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Cherr_set_output_callback(
        this: *mut Chuck_IO_Cherr,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr"]
    pub fn Chuck_IO_Cherr_Chuck_IO_Cherr(this: *mut Chuck_IO_Cherr, carrier: *mut Chuck_Carrier);
}
impl Chuck_IO_Cherr {
    #[inline]
    pub unsafe fn set_output_callback(
        &mut self,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) {
        Chuck_IO_Cherr_set_output_callback(self, fp)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_IO_Cherr_Chuck_IO_Cherr(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr_destructor"]
    pub fn Chuck_IO_Cherr_Chuck_IO_Cherr_destructor(this: *mut Chuck_IO_Cherr);
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_IO_Cherr_good(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_Cherr_close(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_Cherr_flush(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Cherr_mode(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Cherr_mode1(this: *mut c_void, flag: c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_Cherr_readLine(this: *mut c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_Cherr_readInt(this: *mut c_void, flags: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_Cherr_readFloat(this: *mut c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_Cherr_readString(
        this: *mut c_void,
        str: *mut crate::dts::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Cherr_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write(this: *mut c_void, val: *const crate::dts::string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write1(this: *mut c_void, val: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write2(this: *mut c_void, val: c_long, flags: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write3(this: *mut c_void, val: f64);
}
extern "C" {
    pub static mut g_default_chugin_path: [c_char; 0usize];
}
extern "C" {
    pub static mut g_chugin_path_envvar: [c_char; 0usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_UGen {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_UAna {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_UAnaBlobProxy {
    _unused: [u8; 0],
}
pub mod Chuck_DL_Api {
    #[allow(unused_imports)]
    pub type Object = *mut c_void;
    pub type Type = *mut c_void;
    pub type String = *mut c_void;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Api {
        pub vm: *mut crate::chuck_dl_h_edited::Chuck_DL_Api::Api_VMApi,
        pub object: *mut crate::chuck_dl_h_edited::Chuck_DL_Api::Api_ObjectApi,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Api_VMApi {
        pub get_srate: ::std::option::Option<
            unsafe extern "C" fn(arg1: CK_DL_API, arg2: *mut Chuck_VM_Shred) -> c_ulong,
        >,
    }
    extern "C" {
        #[link_name = "\u{1}VMApi"]
        pub fn Api_VMApi_VMApi(this: *mut crate::chuck_dl_h_edited::Chuck_DL_Api::Api_VMApi);
    }
    impl Api_VMApi {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
            Api_VMApi_VMApi(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Api_ObjectApi {
        pub get_type: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                arg2: *mut Chuck_VM_Shred,
                name: *mut crate::dts::string,
            ) -> crate::chuck_dl_h_edited::Chuck_DL_Api::Type,
        >,
        pub create: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                arg2: *mut Chuck_VM_Shred,
                type_: crate::chuck_dl_h_edited::Chuck_DL_Api::Type,
            ) -> crate::chuck_dl_h_edited::Chuck_DL_Api::Object,
        >,
        pub create_string: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                arg2: *mut Chuck_VM_Shred,
                value: *mut crate::dts::string,
            ) -> crate::chuck_dl_h_edited::Chuck_DL_Api::String,
        >,
        pub get_mvar_int: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: crate::chuck_dl_h_edited::Chuck_DL_Api::Object,
                name: *mut crate::dts::string,
                value: *mut c_long,
            ) -> c_ulong,
        >,
        pub get_mvar_float: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: crate::chuck_dl_h_edited::Chuck_DL_Api::Object,
                name: *mut crate::dts::string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_dur: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: crate::chuck_dl_h_edited::Chuck_DL_Api::Object,
                name: *mut crate::dts::string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_time: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: crate::chuck_dl_h_edited::Chuck_DL_Api::Object,
                name: *mut crate::dts::string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_string: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: crate::chuck_dl_h_edited::Chuck_DL_Api::Object,
                name: *mut crate::dts::string,
                value: *mut crate::chuck_dl_h_edited::Chuck_DL_Api::String,
            ) -> c_ulong,
        >,
        pub get_mvar_object: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: crate::chuck_dl_h_edited::Chuck_DL_Api::Object,
                name: *mut crate::dts::string,
                value: *mut crate::chuck_dl_h_edited::Chuck_DL_Api::Object,
            ) -> c_ulong,
        >,
        pub set_string: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                string: crate::chuck_dl_h_edited::Chuck_DL_Api::String,
                value: *mut crate::dts::string,
            ) -> c_ulong,
        >,
    }
    extern "C" {
        #[link_name = "\u{1}ObjectApi"]
        pub fn Api_ObjectApi_ObjectApi(this: *mut crate::chuck_dl_h_edited::Chuck_DL_Api::Api_ObjectApi);
    }
    impl Api_ObjectApi {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
            Api_ObjectApi_ObjectApi(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}g_api"]
        pub static mut Api_g_api: crate::chuck_dl_h_edited::Chuck_DL_Api::Api;
    }
    extern "C" {
        #[link_name = "\u{1}instance"]
        pub fn Api_instance() -> *const crate::chuck_dl_h_edited::Chuck_DL_Api::Api;
    }
    extern "C" {
        #[link_name = "\u{1}Api"]
        pub fn Api_Api(this: *mut crate::chuck_dl_h_edited::Chuck_DL_Api::Api);
    }
    impl Api {
        #[inline]
        pub unsafe fn instance() -> *const crate::chuck_dl_h_edited::Chuck_DL_Api::Api {
            Api_instance()
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
            Api_Api(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
}
pub type CK_DL_API = *const crate::chuck_dl_h_edited::Chuck_DL_Api::Api;
pub type f_ck_declversion = ::std::option::Option<unsafe extern "C" fn() -> c_ulong>;
pub type f_ck_query =
    ::std::option::Option<unsafe extern "C" fn(QUERY: *mut Chuck_DL_Query) -> c_ulong>;
pub type f_alloc = ::std::option::Option<
    unsafe extern "C" fn(
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ) -> *mut Chuck_Object,
>;
pub type f_ctor = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_dtor = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_mfun = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_sfun = ::std::option::Option<
    unsafe extern "C" fn(
        ARGS: *mut c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_tick = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        in_: f32,
        out: *mut f32,
        API: CK_DL_API,
    ) -> c_ulong,
>;
pub type f_tickf = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        in_: *mut f32,
        out: *mut f32,
        nframes: c_ulong,
        API: CK_DL_API,
    ) -> c_ulong,
>;
pub type f_ctrl = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_cget = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_pmsg = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        MSG: *const c_char,
        ARGS: *mut c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ) -> c_ulong,
>;
pub type f_tock = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        UANA: *mut Chuck_UAna,
        BLOB: *mut Chuck_UAnaBlobProxy,
        API: CK_DL_API,
    ) -> c_ulong,
>;
pub type f_mainthreadhook =
    ::std::option::Option<unsafe extern "C" fn(bindle: *mut c_void) -> c_ulong>;
pub type f_mainthreadquit =
    ::std::option::Option<unsafe extern "C" fn(bindle: *mut c_void) -> c_ulong>;
pub type f_setname =
    ::std::option::Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, name: *const c_char)>;
pub type f_begin_class = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, name: *const c_char, parent: *const c_char),
>;
pub type f_add_ctor =
    ::std::option::Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, ctor: f_ctor)>;
pub type f_add_dtor =
    ::std::option::Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, dtor: f_dtor)>;
pub type f_add_mfun = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        mfun: f_mfun,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_add_sfun = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        sfun: f_sfun,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_add_mvar = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
    ) -> c_ulong,
>;
pub type f_add_svar = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
        static_addr: *mut c_void,
    ),
>;
pub type f_add_arg = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, type_: *const c_char, name: *const c_char),
>;
pub type f_add_ugen_func = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        tick: f_tick,
        pmsg: f_pmsg,
        num_in: c_ulong,
        num_out: c_ulong,
    ),
>;
pub type f_add_ugen_funcf = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        tickf: f_tickf,
        pmsg: f_pmsg,
        num_in: c_ulong,
        num_out: c_ulong,
    ),
>;
pub type f_add_ugen_funcf_auto_num_channels = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, tickf: f_tickf, psmg: f_pmsg),
>;
pub type f_add_ugen_ctrl = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        ctrl: f_ctrl,
        cget: f_cget,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_end_class =
    ::std::option::Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query) -> c_ulong>;
pub type f_doc_class = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, doc: *const c_char) -> c_ulong,
>;
pub type f_add_example = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, ex: *const c_char) -> c_ulong,
>;
pub type f_doc_func = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, doc: *const c_char) -> c_ulong,
>;
pub type f_doc_var = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, doc: *const c_char) -> c_ulong,
>;
#[repr(C)]
pub struct Chuck_DL_Query {
    pub m_carrier: *mut Chuck_Carrier,
    pub setname: f_setname,
    pub begin_class: f_begin_class,
    pub add_ctor: f_add_ctor,
    pub add_dtor: f_add_dtor,
    pub add_mfun: f_add_mfun,
    pub add_sfun: f_add_sfun,
    pub add_mvar: f_add_mvar,
    pub add_svar: f_add_svar,
    pub add_arg: f_add_arg,
    pub add_ugen_func: f_add_ugen_func,
    pub add_ugen_funcf: f_add_ugen_funcf,
    pub add_ugen_funcf_auto_num_channels: f_add_ugen_funcf_auto_num_channels,
    pub add_ugen_ctrl: f_add_ugen_ctrl,
    pub end_class: f_end_class,
    pub last_var: *mut Chuck_DL_Value,
    pub doc_class: f_doc_class,
    pub doc_func: f_doc_func,
    pub doc_var: f_doc_var,
    pub add_ex: f_add_example,
    pub dll_ref: *mut Chuck_DLL,
    pub reserved: *mut c_void,
    pub srate: c_ulong,
    pub linepos: c_int,
    pub dll_name: crate::dts::string,
    pub curr_class: *mut Chuck_DL_Class,
    pub curr_func: *mut Chuck_DL_Func,
    pub name: crate::dts::string,
    pub classes: Vec::new(),
    pub stack: Vec::new(),
}
extern "C" {
    #[link_name = "\u{1}compiler"]
    pub fn Chuck_DL_Query_compiler(this: *const Chuck_DL_Query) -> *mut Chuck_Compiler;
}
extern "C" {
    #[link_name = "\u{1}vm"]
    pub fn Chuck_DL_Query_vm(this: *const Chuck_DL_Query) -> *mut Chuck_VM;
}
extern "C" {
    #[link_name = "\u{1}env"]
    pub fn Chuck_DL_Query_env(this: *const Chuck_DL_Query) -> *mut Chuck_Env;
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_DL_Query_clear(this: *mut Chuck_DL_Query);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Query"]
    pub fn Chuck_DL_Query_Chuck_DL_Query(this: *mut Chuck_DL_Query, carrier: *mut Chuck_Carrier);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Query_destructor"]
    pub fn Chuck_DL_Query_Chuck_DL_Query_destructor(this: *mut Chuck_DL_Query);
}
impl Chuck_DL_Query {
    #[inline]
    pub unsafe fn compiler(&self) -> *mut Chuck_Compiler {
        Chuck_DL_Query_compiler(self)
    }
    #[inline]
    pub unsafe fn vm(&self) -> *mut Chuck_VM {
        Chuck_DL_Query_vm(self)
    }
    #[inline]
    pub unsafe fn env(&self) -> *mut Chuck_Env {
        Chuck_DL_Query_env(self)
    }
    #[inline]
    pub unsafe fn clear(&mut self) {
        Chuck_DL_Query_clear(self)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Query_Chuck_DL_Query(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_DL_Query_Chuck_DL_Query_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_DL_Class {
    pub name: crate::dts::string,
    pub parent: crate::dts::string,
    pub ctors: Vec::new(),
    pub dtor: *mut Chuck_DL_Func,
    pub mfuns: Vec::new(),
    pub sfuns: Vec::new(),
    pub mvars: Vec::new(),
    pub svars: Vec::new(),
    pub ugen_tick: f_tick,
    pub ugen_tickf: f_tickf,
    pub ugen_pmsg: f_pmsg,
    pub ugen_ctrl: Vec::new(),
    pub uana_tock: f_tock,
    pub classes: Vec::new(),
    pub current_mvar_offset: c_ulong,
    pub ugen_num_in: c_ulong,
    pub ugen_num_out: c_ulong,
    pub doc: crate::dts::string,
    pub examples: Vec::new(),
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Class"]
    pub fn Chuck_DL_Class_Chuck_DL_Class(this: *mut Chuck_DL_Class);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Class_destructor"]
    pub fn Chuck_DL_Class_Chuck_DL_Class_destructor(this: *mut Chuck_DL_Class);
}
impl Chuck_DL_Class {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Class_Chuck_DL_Class(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_DL_Class_Chuck_DL_Class_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_DL_Value {
    pub name: crate::dts::string,
    pub type_: crate::dts::string,
    pub is_const: c_ulong,
    pub static_addr: *mut c_void,
    pub doc: crate::dts::string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Value"]
    pub fn Chuck_DL_Value_Chuck_DL_Value(this: *mut Chuck_DL_Value);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Value"]
    pub fn Chuck_DL_Value_Chuck_DL_Value1(
        this: *mut Chuck_DL_Value,
        t: *const c_char,
        n: *const c_char,
        c: c_ulong,
        a: *mut c_void,
    );
}
impl Chuck_DL_Value {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Value_Chuck_DL_Value(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(t: *const c_char, n: *const c_char, c: c_ulong, a: *mut c_void) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Value_Chuck_DL_Value1(&mut __bindgen_tmp, t, n, c, a);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DL_Func {
    pub name: crate::dts::string,
    pub type_: crate::dts::string,
    pub __bindgen_anon_1: Chuck_DL_Func__bindgen_ty_1,
    pub args: Vec::new(),
    pub doc: crate::dts::string,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union Chuck_DL_Func__bindgen_ty_1 {
    pub ctor: f_ctor,
    pub dtor: f_dtor,
    pub mfun: f_mfun,
    pub sfun: f_sfun,
    pub addr: c_ulong,
    _bindgen_union_align: u64,
}
extern "C" {
    #[link_name = "\u{1}add_arg"]
    pub fn Chuck_DL_Func_add_arg(this: *mut Chuck_DL_Func, t: *const c_char, n: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func"]
    pub fn Chuck_DL_Func_Chuck_DL_Func(this: *mut Chuck_DL_Func);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func"]
    pub fn Chuck_DL_Func_Chuck_DL_Func1(
        this: *mut Chuck_DL_Func,
        t: *const c_char,
        n: *const c_char,
        a: c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func_destructor"]
    pub fn Chuck_DL_Func_Chuck_DL_Func_destructor(this: *mut Chuck_DL_Func);
}
impl Chuck_DL_Func {
    #[inline]
    pub unsafe fn add_arg(&mut self, t: *const c_char, n: *const c_char) {
        Chuck_DL_Func_add_arg(self, t, n)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Func_Chuck_DL_Func(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(t: *const c_char, n: *const c_char, a: c_ulong) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Func_Chuck_DL_Func1(&mut __bindgen_tmp, t, n, a);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_DL_Func_Chuck_DL_Func_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_DL_Ctrl {
    pub name: crate::dts::string,
    pub type_: crate::dts::string,
    pub types: Vec::new(),
    pub ctrl: f_ctrl,
    pub cget: f_cget,
}
extern "C" {
    pub fn make_new_mfun(t: *const c_char, n: *const c_char, mfun: f_mfun) -> *mut Chuck_DL_Func;
}
extern "C" {
    pub fn make_new_sfun(t: *const c_char, n: *const c_char, sfun: f_sfun) -> *mut Chuck_DL_Func;
}
extern "C" {
    pub fn make_new_arg(t: *const c_char, n: *const c_char) -> *mut Chuck_DL_Value;
}
extern "C" {
    pub fn make_new_mvar(t: *const c_char, n: *const c_char, c: c_ulong) -> *mut Chuck_DL_Value;
}
extern "C" {
    pub fn make_new_svar(
        t: *const c_char,
        n: *const c_char,
        c: c_ulong,
        a: *mut c_void,
    ) -> *mut Chuck_DL_Value;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union Chuck_DL_Return {
    pub v_int: c_long,
    pub v_uint: c_ulong,
    pub v_float: f64,
    pub v_dur: f64,
    pub v_time: f64,
    pub v_complex: crate::chuck_def_h_edited::t_CKCOMPLEX,
    pub v_polar: crate::chuck_def_h_edited::t_CKPOLAR,
    pub v_vec3: crate::chuck_def_h_edited::t_CKVEC3,
    pub v_vec4: crate::chuck_def_h_edited::t_CKVEC4,
    pub v_object: *mut Chuck_Object,
    pub v_string: *mut Chuck_String,
    _bindgen_union_align: [u64; 4usize],
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Return"]
    pub fn Chuck_DL_Return_Chuck_DL_Return(this: *mut Chuck_DL_Return);
}
impl Chuck_DL_Return {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Return_Chuck_DL_Return(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DLL {
    pub _base: Chuck_VM_Object,
    pub m_handle: *mut c_void,
    pub m_last_error: crate::dts::string,
    pub m_filename: crate::dts::string,
    pub m_id: crate::dts::string,
    pub m_func: crate::dts::string,
    pub m_done_query: c_ulong,
    pub m_version_func: f_ck_declversion,
    pub m_query_func: f_ck_query,
    pub m_query: Chuck_DL_Query,
}
extern "C" {
    #[link_name = "\u{1}load"]
    pub fn Chuck_DLL_load(
        this: *mut Chuck_DLL,
        filename: *const c_char,
        func: *const c_char,
        lazy: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}load"]
    pub fn Chuck_DLL_load1(this: *mut Chuck_DLL, query_func: f_ck_query, lazy: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_addr"]
    pub fn Chuck_DLL_get_addr(this: *mut Chuck_DLL, symbol: *const c_char) -> *mut c_void;
}
extern "C" {
    #[link_name = "\u{1}last_error"]
    pub fn Chuck_DLL_last_error(this: *const Chuck_DLL) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}unload"]
    pub fn Chuck_DLL_unload(this: *mut Chuck_DLL) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}query"]
    pub fn Chuck_DLL_query(this: *mut Chuck_DLL) -> *const Chuck_DL_Query;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_DLL_good(this: *const Chuck_DLL) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}name"]
    pub fn Chuck_DLL_name(this: *const Chuck_DLL) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_DLL"]
    pub fn Chuck_DLL_Chuck_DLL(
        this: *mut Chuck_DLL,
        carrier: *mut Chuck_Carrier,
        xid: *const c_char,
    );
}
impl Chuck_DLL {
    #[inline]
    pub unsafe fn load(
        &mut self,
        filename: *const c_char,
        func: *const c_char,
        lazy: c_ulong,
    ) -> c_ulong {
        Chuck_DLL_load(self, filename, func, lazy)
    }
    #[inline]
    pub unsafe fn load1(&mut self, query_func: f_ck_query, lazy: c_ulong) -> c_ulong {
        Chuck_DLL_load1(self, query_func, lazy)
    }
    #[inline]
    pub unsafe fn get_addr(&mut self, symbol: *const c_char) -> *mut c_void {
        Chuck_DLL_get_addr(self, symbol)
    }
    #[inline]
    pub unsafe fn last_error(&self) -> *const c_char {
        Chuck_DLL_last_error(self)
    }
    #[inline]
    pub unsafe fn unload(&mut self) -> c_ulong {
        Chuck_DLL_unload(self)
    }
    #[inline]
    pub unsafe fn query(&mut self) -> *const Chuck_DL_Query {
        Chuck_DLL_query(self)
    }
    #[inline]
    pub unsafe fn good(&self) -> c_ulong {
        Chuck_DLL_good(self)
    }
    #[inline]
    pub unsafe fn name(&self) -> *const c_char {
        Chuck_DLL_name(self)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier, xid: *const c_char) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DLL_Chuck_DLL(&mut __bindgen_tmp, carrier, xid);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_DLL_destructor"]
    pub fn Chuck_DLL_Chuck_DLL_destructor(this: *mut Chuck_DLL);
}
extern "C" {
    pub fn _dl_mcount_wrapper_check(__selfpc: *mut c_void);
}
pub type Lmid_t = c_long;
/// dlfcn.h functions
extern "C" {
    pub fn dlopen(__file: *const c_char, __mode: c_int) -> *mut c_void;
}
extern "C" {
    pub fn dlclose(__handle: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dlsym(__handle: *mut c_void, __name: *const c_char) -> *mut c_void;
}
extern "C" {
    pub fn dlmopen(__nsid: Lmid_t, __file: *const c_char, __mode: c_int) -> *mut c_void;
}
extern "C" {
    pub fn dlvsym(
        __handle: *mut c_void,
        __name: *const c_char,
        __version: *const c_char,
    ) -> *mut c_void;
}
extern "C" {
    pub fn dlerror() -> *mut c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dl_info {
    pub dli_fname: *const c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const c_char,
    pub dli_saddr: *mut c_void,
}
extern "C" {
    pub fn dladdr(__address: *const c_void, __info: *mut Dl_info) -> c_int;
}
extern "C" {
    pub fn dladdr1(
        __address: *const c_void,
        __info: *mut Dl_info,
        __extra_info: *mut c_void,
        __flags: c_int,
    ) -> c_int;
}
pub const RTLD_DL_SYMENT: _bindgen_ty_47 = 1;
pub const RTLD_DL_LINKMAP: _bindgen_ty_47 = 2;
pub type _bindgen_ty_47 = u32;
extern "C" {
    pub fn dlinfo(__handle: *mut c_void, __request: c_int, __arg: *mut c_void) -> c_int;
}
pub const RTLD_DI_LMID: _bindgen_ty_48 = 1;
pub const RTLD_DI_LINKMAP: _bindgen_ty_48 = 2;
pub const RTLD_DI_CONFIGADDR: _bindgen_ty_48 = 3;
pub const RTLD_DI_SERINFO: _bindgen_ty_48 = 4;
pub const RTLD_DI_SERINFOSIZE: _bindgen_ty_48 = 5;
pub const RTLD_DI_ORIGIN: _bindgen_ty_48 = 6;
pub const RTLD_DI_PROFILENAME: _bindgen_ty_48 = 7;
pub const RTLD_DI_PROFILEOUT: _bindgen_ty_48 = 8;
pub const RTLD_DI_TLS_MODID: _bindgen_ty_48 = 9;
pub const RTLD_DI_TLS_DATA: _bindgen_ty_48 = 10;
pub const RTLD_DI_MAX: _bindgen_ty_48 = 10;
pub type _bindgen_ty_48 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dl_serpath {
    pub dls_name: *mut c_char,
    pub dls_flags: c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dl_serinfo {
    pub dls_size: usize,
    pub dls_cnt: c_uint,
    pub dls_serpath: [Dl_serpath; 1usize],
}
