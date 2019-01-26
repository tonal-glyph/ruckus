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
use libc::*;
use crate::chuck_def_h_edited::*;
use crate::chuck_carrier_h_edited::*;
use crate::chuck_oo_h_edited::*;
use crate::util_thread_h_edited::*;
use crate::dts::*;
use crate::rtmidi_h_edited::string;
///* ChucK shell implementation
use std::mem::*;
#[repr(C)]
struct Chuck_VM_Object__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_VM_Object {
    vtable_: *const Chuck_VM_Object__bindgen_vtable,
    m_ref_count: c_ulong,
    m_pooled: c_ulong,
    m_locked: c_ulong,
    m_v_ref: *mut crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}our_locks_in_effect"]
    static mut Chuck_VM_Object_our_locks_in_effect: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}lock_all"]
    fn Chuck_VM_Object_lock_all();
}
extern "C" {
    #[link_name = "\u{1}unlock_all"]
    fn Chuck_VM_Object_unlock_all();
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Object"]
    fn Chuck_VM_Object_Chuck_VM_Object(this: *mut Chuck_VM_Object);
}
impl Chuck_VM_Object {
    #[inline]
    unsafe fn lock_all() {
        Chuck_VM_Object_lock_all()
    }
    #[inline]
    unsafe fn unlock_all() {
        Chuck_VM_Object_unlock_all()
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Object_Chuck_VM_Object(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Object_destructor"]
    fn Chuck_VM_Object_Chuck_VM_Object_destructor(this: *mut Chuck_VM_Object);
}
extern "C" {
    #[link_name = "\u{1}add_ref"]
    fn Chuck_VM_Object_add_ref(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}release"]
    fn Chuck_VM_Object_release(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}lock"]
    fn Chuck_VM_Object_lock(this: *mut c_void);
}
#[repr(C)]
pub struct Chuck_VTable {
    pub funcs: crate::dts::vector,
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
    fn Chuck_Object_Chuck_Object(this: *mut Chuck_Object);
}
impl Chuck_Object {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Object_Chuck_Object(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Object_destructor"]
    fn Chuck_Object_Chuck_Object_destructor(this: *mut Chuck_Object);
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Array {
    pub _base: Chuck_Object,
    pub m_array_type: *mut Chuck_Type,
}
#[repr(C)]
struct Chuck_Array4 {
    _base: Chuck_Array,
    m_vector: crate::dts::vector,
    m_map: std::collections::HashMap::new(),
    m_is_obj: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array4_addr(this: *mut Chuck_Array4, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array4_addr1(this: *mut Chuck_Array4, key: *const crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array4_get(this: *mut Chuck_Array4, i: c_long, val: *mut c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array4_get1(this: *mut Chuck_Array4, key: *const crate::rtmidi_h_edited::string, val: *mut c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array4_set(this: *mut Chuck_Array4, i: c_long, val: c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array4_set1(this: *mut Chuck_Array4, key: *const crate::rtmidi_h_edited::string, val: c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    fn Chuck_Array4_push_back(this: *mut Chuck_Array4, val: c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    fn Chuck_Array4_pop_back(this: *mut Chuck_Array4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    fn Chuck_Array4_back(this: *const Chuck_Array4, val: *mut c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    fn Chuck_Array4_zero(this: *mut Chuck_Array4, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4"]
    fn Chuck_Array4_Chuck_Array4(this: *mut Chuck_Array4, is_obj: c_ulong, capacity: c_long);
}
impl Chuck_Array4 {
    #[inline]
    unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array4_addr(self, i)
    }
    #[inline]
    unsafe fn addr1(&mut self, key: *const crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_Array4_addr1(self, key)
    }
    #[inline]
    unsafe fn get(&mut self, i: c_long, val: *mut c_ulong) -> c_long {
        Chuck_Array4_get(self, i, val)
    }
    #[inline]
    unsafe fn get1(&mut self, key: *const crate::rtmidi_h_edited::string, val: *mut c_ulong) -> c_long {
        Chuck_Array4_get1(self, key, val)
    }
    #[inline]
    unsafe fn set(&mut self, i: c_long, val: c_ulong) -> c_long {
        Chuck_Array4_set(self, i, val)
    }
    #[inline]
    unsafe fn set1(&mut self, key: *const crate::rtmidi_h_edited::string, val: c_ulong) -> c_long {
        Chuck_Array4_set1(self, key, val)
    }
    #[inline]
    unsafe fn push_back(&mut self, val: c_ulong) -> c_long {
        Chuck_Array4_push_back(self, val)
    }
    #[inline]
    unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array4_pop_back(self)
    }
    #[inline]
    unsafe fn back(&self, val: *mut c_ulong) -> c_long {
        Chuck_Array4_back(self, val)
    }
    #[inline]
    unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array4_zero(self, start, end)
    }
    #[inline]
    unsafe fn new(is_obj: c_ulong, capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array4_Chuck_Array4(&mut __bindgen_tmp, is_obj, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4_destructor"]
    fn Chuck_Array4_Chuck_Array4_destructor(this: *mut Chuck_Array4);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    fn Chuck_Array4_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    fn Chuck_Array4_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    fn Chuck_Array4_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    fn Chuck_Array4_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    fn Chuck_Array4_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    fn Chuck_Array4_find(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    fn Chuck_Array4_erase(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    fn Chuck_Array4_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    fn Chuck_Array4_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
struct Chuck_Array8 {
    _base: Chuck_Array,
    m_vector: crate::dts::vector,
    m_map: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array8_addr(this: *mut Chuck_Array8, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array8_addr1(this: *mut Chuck_Array8, key: *const crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array8_get(this: *mut Chuck_Array8, i: c_long, val: *mut f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array8_get1(this: *mut Chuck_Array8, key: *const crate::rtmidi_h_edited::string, val: *mut f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array8_set(this: *mut Chuck_Array8, i: c_long, val: f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array8_set1(this: *mut Chuck_Array8, key: *const crate::rtmidi_h_edited::string, val: f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    fn Chuck_Array8_push_back(this: *mut Chuck_Array8, val: f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    fn Chuck_Array8_pop_back(this: *mut Chuck_Array8) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    fn Chuck_Array8_back(this: *const Chuck_Array8, val: *mut f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    fn Chuck_Array8_zero(this: *mut Chuck_Array8, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array8"]
    fn Chuck_Array8_Chuck_Array8(this: *mut Chuck_Array8, capacity: c_long);
}
impl Chuck_Array8 {
    #[inline]
    unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array8_addr(self, i)
    }
    #[inline]
    unsafe fn addr1(&mut self, key: *const crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_Array8_addr1(self, key)
    }
    #[inline]
    unsafe fn get(&mut self, i: c_long, val: *mut f64) -> c_long {
        Chuck_Array8_get(self, i, val)
    }
    #[inline]
    unsafe fn get1(&mut self, key: *const crate::rtmidi_h_edited::string, val: *mut f64) -> c_long {
        Chuck_Array8_get1(self, key, val)
    }
    #[inline]
    unsafe fn set(&mut self, i: c_long, val: f64) -> c_long {
        Chuck_Array8_set(self, i, val)
    }
    #[inline]
    unsafe fn set1(&mut self, key: *const crate::rtmidi_h_edited::string, val: f64) -> c_long {
        Chuck_Array8_set1(self, key, val)
    }
    #[inline]
    unsafe fn push_back(&mut self, val: f64) -> c_long {
        Chuck_Array8_push_back(self, val)
    }
    #[inline]
    unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array8_pop_back(self)
    }
    #[inline]
    unsafe fn back(&self, val: *mut f64) -> c_long {
        Chuck_Array8_back(self, val)
    }
    #[inline]
    unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array8_zero(self, start, end)
    }
    #[inline]
    unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array8_Chuck_Array8(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array8_destructor"]
    fn Chuck_Array8_Chuck_Array8_destructor(this: *mut Chuck_Array8);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    fn Chuck_Array8_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    fn Chuck_Array8_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    fn Chuck_Array8_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    fn Chuck_Array8_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    fn Chuck_Array8_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    fn Chuck_Array8_find(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    fn Chuck_Array8_erase(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    fn Chuck_Array8_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    fn Chuck_Array8_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
struct Chuck_Array16 {
    _base: Chuck_Array,
    m_vector: crate::dts::vector,
    m_map: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array16_addr(this: *mut Chuck_Array16, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array16_addr1(this: *mut Chuck_Array16, key: *const crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array16_get(this: *mut Chuck_Array16, i: c_long, val: *mut t_CKCOMPLEX) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array16_get1(
        this: *mut Chuck_Array16,
        key: *const crate::rtmidi_h_edited::string,
        val: *mut t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array16_set(this: *mut Chuck_Array16, i: c_long, val: *const t_CKCOMPLEX) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array16_set1(
        this: *mut Chuck_Array16,
        key: *const crate::rtmidi_h_edited::string,
        val: *const t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    fn Chuck_Array16_push_back(this: *mut Chuck_Array16, val: *const t_CKCOMPLEX) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    fn Chuck_Array16_pop_back(this: *mut Chuck_Array16) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    fn Chuck_Array16_back(this: *const Chuck_Array16, val: *mut t_CKCOMPLEX) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    fn Chuck_Array16_zero(this: *mut Chuck_Array16, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array16"]
    fn Chuck_Array16_Chuck_Array16(this: *mut Chuck_Array16, capacity: c_long);
}
impl Chuck_Array16 {
    #[inline]
    unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array16_addr(self, i)
    }
    #[inline]
    unsafe fn addr1(&mut self, key: *const crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_Array16_addr1(self, key)
    }
    #[inline]
    unsafe fn get(&mut self, i: c_long, val: *mut t_CKCOMPLEX) -> c_long {
        Chuck_Array16_get(self, i, val)
    }
    #[inline]
    unsafe fn get1(&mut self, key: *const crate::rtmidi_h_edited::string, val: *mut t_CKCOMPLEX) -> c_long {
        Chuck_Array16_get1(self, key, val)
    }
    #[inline]
    unsafe fn set(&mut self, i: c_long, val: *const t_CKCOMPLEX) -> c_long {
        Chuck_Array16_set(self, i, val)
    }
    #[inline]
    unsafe fn set1(&mut self, key: *const crate::rtmidi_h_edited::string, val: *const t_CKCOMPLEX) -> c_long {
        Chuck_Array16_set1(self, key, val)
    }
    #[inline]
    unsafe fn push_back(&mut self, val: *const t_CKCOMPLEX) -> c_long {
        Chuck_Array16_push_back(self, val)
    }
    #[inline]
    unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array16_pop_back(self)
    }
    #[inline]
    unsafe fn back(&self, val: *mut t_CKCOMPLEX) -> c_long {
        Chuck_Array16_back(self, val)
    }
    #[inline]
    unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array16_zero(self, start, end)
    }
    #[inline]
    unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array16_Chuck_Array16(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array16_destructor"]
    fn Chuck_Array16_Chuck_Array16_destructor(this: *mut Chuck_Array16);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    fn Chuck_Array16_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    fn Chuck_Array16_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    fn Chuck_Array16_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    fn Chuck_Array16_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    fn Chuck_Array16_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    fn Chuck_Array16_find(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    fn Chuck_Array16_erase(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    fn Chuck_Array16_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    fn Chuck_Array16_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
struct Chuck_Array24 {
    _base: Chuck_Array,
    m_vector: crate::dts::vector,
    m_map: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array24_addr(this: *mut Chuck_Array24, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array24_addr1(this: *mut Chuck_Array24, key: *const crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array24_get(this: *mut Chuck_Array24, i: c_long, val: *mut t_CKVEC3) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array24_get1(
        this: *mut Chuck_Array24,
        key: *const crate::rtmidi_h_edited::string,
        val: *mut t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array24_set(this: *mut Chuck_Array24, i: c_long, val: *const t_CKVEC3) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array24_set1(
        this: *mut Chuck_Array24,
        key: *const crate::rtmidi_h_edited::string,
        val: *const t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    fn Chuck_Array24_push_back(this: *mut Chuck_Array24, val: *const t_CKVEC3) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    fn Chuck_Array24_pop_back(this: *mut Chuck_Array24) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    fn Chuck_Array24_back(this: *const Chuck_Array24, val: *mut t_CKVEC3) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    fn Chuck_Array24_zero(this: *mut Chuck_Array24, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array24"]
    fn Chuck_Array24_Chuck_Array24(this: *mut Chuck_Array24, capacity: c_long);
}
impl Chuck_Array24 {
    #[inline]
    unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array24_addr(self, i)
    }
    #[inline]
    unsafe fn addr1(&mut self, key: *const crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_Array24_addr1(self, key)
    }
    #[inline]
    unsafe fn get(&mut self, i: c_long, val: *mut t_CKVEC3) -> c_long {
        Chuck_Array24_get(self, i, val)
    }
    #[inline]
    unsafe fn get1(&mut self, key: *const crate::rtmidi_h_edited::string, val: *mut t_CKVEC3) -> c_long {
        Chuck_Array24_get1(self, key, val)
    }
    #[inline]
    unsafe fn set(&mut self, i: c_long, val: *const t_CKVEC3) -> c_long {
        Chuck_Array24_set(self, i, val)
    }
    #[inline]
    unsafe fn set1(&mut self, key: *const crate::rtmidi_h_edited::string, val: *const t_CKVEC3) -> c_long {
        Chuck_Array24_set1(self, key, val)
    }
    #[inline]
    unsafe fn push_back(&mut self, val: *const t_CKVEC3) -> c_long {
        Chuck_Array24_push_back(self, val)
    }
    #[inline]
    unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array24_pop_back(self)
    }
    #[inline]
    unsafe fn back(&self, val: *mut t_CKVEC3) -> c_long {
        Chuck_Array24_back(self, val)
    }
    #[inline]
    unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array24_zero(self, start, end)
    }
    #[inline]
    unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array24_Chuck_Array24(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array24_destructor"]
    fn Chuck_Array24_Chuck_Array24_destructor(this: *mut Chuck_Array24);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    fn Chuck_Array24_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    fn Chuck_Array24_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    fn Chuck_Array24_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    fn Chuck_Array24_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    fn Chuck_Array24_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    fn Chuck_Array24_find(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    fn Chuck_Array24_erase(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    fn Chuck_Array24_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    fn Chuck_Array24_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
struct Chuck_Array32 {
    _base: Chuck_Array,
    m_vector: crate::dts::vector,
    m_map: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array32_addr(this: *mut Chuck_Array32, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    fn Chuck_Array32_addr1(this: *mut Chuck_Array32, key: *const crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array32_get(this: *mut Chuck_Array32, i: c_long, val: *mut t_CKVEC4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_Array32_get1(
        this: *mut Chuck_Array32,
        key: *const crate::rtmidi_h_edited::string,
        val: *mut t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array32_set(this: *mut Chuck_Array32, i: c_long, val: *const t_CKVEC4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Array32_set1(
        this: *mut Chuck_Array32,
        key: *const crate::rtmidi_h_edited::string,
        val: *const t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    fn Chuck_Array32_push_back(this: *mut Chuck_Array32, val: *const t_CKVEC4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    fn Chuck_Array32_pop_back(this: *mut Chuck_Array32) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    fn Chuck_Array32_back(this: *const Chuck_Array32, val: *mut t_CKVEC4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    fn Chuck_Array32_zero(this: *mut Chuck_Array32, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array32"]
    fn Chuck_Array32_Chuck_Array32(this: *mut Chuck_Array32, capacity: c_long);
}
impl Chuck_Array32 {
    #[inline]
    unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array32_addr(self, i)
    }
    #[inline]
    unsafe fn addr1(&mut self, key: *const crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_Array32_addr1(self, key)
    }
    #[inline]
    unsafe fn get(&mut self, i: c_long, val: *mut t_CKVEC4) -> c_long {
        Chuck_Array32_get(self, i, val)
    }
    #[inline]
    unsafe fn get1(&mut self, key: *const crate::rtmidi_h_edited::string, val: *mut t_CKVEC4) -> c_long {
        Chuck_Array32_get1(self, key, val)
    }
    #[inline]
    unsafe fn set(&mut self, i: c_long, val: *const t_CKVEC4) -> c_long {
        Chuck_Array32_set(self, i, val)
    }
    #[inline]
    unsafe fn set1(&mut self, key: *const crate::rtmidi_h_edited::string, val: *const t_CKVEC4) -> c_long {
        Chuck_Array32_set1(self, key, val)
    }
    #[inline]
    unsafe fn push_back(&mut self, val: *const t_CKVEC4) -> c_long {
        Chuck_Array32_push_back(self, val)
    }
    #[inline]
    unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array32_pop_back(self)
    }
    #[inline]
    unsafe fn back(&self, val: *mut t_CKVEC4) -> c_long {
        Chuck_Array32_back(self, val)
    }
    #[inline]
    unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array32_zero(self, start, end)
    }
    #[inline]
    unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Array32_Chuck_Array32(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array32_destructor"]
    fn Chuck_Array32_Chuck_Array32_destructor(this: *mut Chuck_Array32);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    fn Chuck_Array32_clear(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    fn Chuck_Array32_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    fn Chuck_Array32_capacity(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    fn Chuck_Array32_set_size(this: *mut c_void, size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    fn Chuck_Array32_set_capacity(this: *mut c_void, capacity: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    fn Chuck_Array32_find(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    fn Chuck_Array32_erase(this: *mut c_void, key: *const crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    fn Chuck_Array32_data_type_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    fn Chuck_Array32_data_type_kind(this: *mut c_void) -> c_long;
}
#[repr(C)]
pub struct Chuck_Event {
    pub _base: Chuck_Object,
    pub m_queue: crate::dts::deque,
    pub m_queue_lock: XMutex,
}
extern "C" {
    #[link_name = "\u{1}our_can_wait"]
    static mut Chuck_Event_our_can_wait: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signal"]
    fn Chuck_Event_signal(this: *mut Chuck_Event);
}
extern "C" {
    #[link_name = "\u{1}broadcast"]
    fn Chuck_Event_broadcast(this: *mut Chuck_Event);
}
extern "C" {
    #[link_name = "\u{1}wait"]
    fn Chuck_Event_wait(this: *mut Chuck_Event, shred: *mut Chuck_VM_Shred, vm: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}remove"]
    fn Chuck_Event_remove(this: *mut Chuck_Event, shred: *mut Chuck_VM_Shred) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_broadcast"]
    fn Chuck_Event_queue_broadcast(this: *mut Chuck_Event, event_buffer: *mut CBufferSimple);
}
impl Chuck_Event {
    #[inline]
    unsafe fn signal(&mut self) {
        Chuck_Event_signal(self)
    }
    #[inline]
    unsafe fn broadcast(&mut self) {
        Chuck_Event_broadcast(self)
    }
    #[inline]
    unsafe fn wait(&mut self, shred: *mut Chuck_VM_Shred, vm: *mut Chuck_VM) {
        Chuck_Event_wait(self, shred, vm)
    }
    #[inline]
    unsafe fn remove(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_Event_remove(self, shred)
    }
    #[inline]
    unsafe fn queue_broadcast(&mut self, event_buffer: *mut CBufferSimple) {
        Chuck_Event_queue_broadcast(self, event_buffer)
    }
}
#[repr(C)]
pub struct Chuck_String {
    pub _base: Chuck_Object,
    pub m_charptr: *const c_char,
    pub m_str: crate::rtmidi_h_edited::string,
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_String_set(this: *mut Chuck_String, s: *const crate::rtmidi_h_edited::string);
}
extern "C" {
    #[link_name = "\u{1}str"]
    fn Chuck_String_str(this: *mut Chuck_String) -> *const crate::rtmidi_h_edited::string;
}
extern "C" {
    #[link_name = "\u{1}c_str"]
    fn Chuck_String_c_str(this: *mut Chuck_String) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_String"]
    fn Chuck_String_Chuck_String(this: *mut Chuck_String, s: *const crate::rtmidi_h_edited::string);
}
impl Chuck_String {
    #[inline]
    unsafe fn set(&mut self, s: *const crate::rtmidi_h_edited::string) {
        Chuck_String_set(self, s)
    }
    #[inline]
    unsafe fn str(&mut self) -> *const crate::rtmidi_h_edited::string {
        Chuck_String_str(self)
    }
    #[inline]
    unsafe fn c_str(&mut self) -> *const c_char {
        Chuck_String_c_str(self)
    }
    #[inline]
    unsafe fn new(s: *const crate::rtmidi_h_edited::string) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_String_Chuck_String(&mut __bindgen_tmp, s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_String_destructor"]
    fn Chuck_String_Chuck_String_destructor(this: *mut Chuck_String);
}
#[repr(C)]
pub struct Chuck_IO {
    pub _base: Chuck_Event,
    pub m_asyncEvent: *mut Chuck_Event,
    pub m_thread: *mut XThread,
}
#[repr(C)]
struct Chuck_IO_async_args {
    fileio_obj: *mut Chuck_IO_File,
    RETURN: *mut c_void,
    intArg: c_long,
    floatArg: f64,
    stringArg: crate::rtmidi_h_edited::string,
}
extern "C" {
    #[link_name = "\u{1}INT32"]
    static Chuck_IO_INT32: c_long;
}
extern "C" {
    #[link_name = "\u{1}INT16"]
    static Chuck_IO_INT16: c_long;
}
extern "C" {
    #[link_name = "\u{1}INT8"]
    static Chuck_IO_INT8: c_long;
}
extern "C" {
    #[link_name = "\u{1}MODE_SYNC"]
    static Chuck_IO_MODE_SYNC: c_long;
}
extern "C" {
    #[link_name = "\u{1}MODE_ASYNC"]
    static Chuck_IO_MODE_ASYNC: c_long;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO"]
    fn Chuck_IO_Chuck_IO(this: *mut Chuck_IO);
}
impl Chuck_IO {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_IO_Chuck_IO(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_destructor"]
    fn Chuck_IO_Chuck_IO_destructor(this: *mut Chuck_IO);
}
fn file() -> std::io::Result<()> {let mut f = std::fs::File::create("temp.ck")?;Ok(())}
#[repr(C)]
pub struct Chuck_IO_File {
    pub _base: Chuck_IO,
    pub m_flags: c_long,
    pub m_iomode: c_long,
    pub m_io: file,
    pub m_dir: *mut std::fs::DirEntry,
    pub m_dir_start: c_long,
    pub m_path: crate::rtmidi_h_edited::string,
    pub m_vmRef: *mut Chuck_VM,
}
extern "C" {
    #[link_name = "\u{1}FLAG_READ_WRITE"]
    static Chuck_IO_File_FLAG_READ_WRITE: c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_READONLY"]
    static Chuck_IO_File_FLAG_READONLY: c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_WRITEONLY"]
    static Chuck_IO_File_FLAG_WRITEONLY: c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_APPEND"]
    static Chuck_IO_File_FLAG_APPEND: c_long;
}
extern "C" {
    #[link_name = "\u{1}TYPE_ASCII"]
    static Chuck_IO_File_TYPE_ASCII: c_long;
}
extern "C" {
    #[link_name = "\u{1}TYPE_BINARY"]
    static Chuck_IO_File_TYPE_BINARY: c_long;
}
extern "C" {
    #[link_name = "\u{1}writeStr_thread"]
    fn Chuck_IO_File_writeStr_thread(data: *mut c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeInt_thread"]
    fn Chuck_IO_File_writeInt_thread(data: *mut c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeFloat_thread"]
    fn Chuck_IO_File_writeFloat_thread(data: *mut c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File"]
    fn Chuck_IO_File_Chuck_IO_File(this: *mut Chuck_IO_File, vm: *mut Chuck_VM);
}
impl Chuck_IO_File {
    #[inline]
    unsafe fn writeStr_thread(data: *mut c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeStr_thread(data)
    }
    #[inline]
    unsafe fn writeInt_thread(data: *mut c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeInt_thread(data)
    }
    #[inline]
    unsafe fn writeFloat_thread(data: *mut c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeFloat_thread(data)
    }
    #[inline]
    unsafe fn new(vm: *mut Chuck_VM) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_IO_File_Chuck_IO_File(&mut __bindgen_tmp, vm);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File_destructor"]
    fn Chuck_IO_File_Chuck_IO_File_destructor(this: *mut Chuck_IO_File);
}
extern "C" {
    #[link_name = "\u{1}open"]
    fn Chuck_IO_File_open(this: *mut c_void, path: *const crate::rtmidi_h_edited::string, flags: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    fn Chuck_IO_File_good(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    fn Chuck_IO_File_close(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    fn Chuck_IO_File_flush(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    fn Chuck_IO_File_mode(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    fn Chuck_IO_File_mode1(this: *mut c_void, flag: c_long);
}
extern "C" {
    #[link_name = "\u{1}size"]
    fn Chuck_IO_File_size(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}seek"]
    fn Chuck_IO_File_seek(this: *mut c_void, pos: c_long);
}
extern "C" {
    #[link_name = "\u{1}tell"]
    fn Chuck_IO_File_tell(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}isDir"]
    fn Chuck_IO_File_isDir(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}dirList"]
    fn Chuck_IO_File_dirList(this: *mut c_void) -> *mut Chuck_Array4;
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    fn Chuck_IO_File_readLine(this: *mut c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    fn Chuck_IO_File_readInt(this: *mut c_void, flags: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    fn Chuck_IO_File_readFloat(this: *mut c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    fn Chuck_IO_File_readString(this: *mut c_void, str: *mut crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    fn Chuck_IO_File_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_File_write(this: *mut c_void, val: *const crate::rtmidi_h_edited::string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_File_write1(this: *mut c_void, val: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_File_write2(this: *mut c_void, val: c_long, flags: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_File_write3(this: *mut c_void, val: f64);
}
#[repr(C)]
pub struct Chuck_IO_Chout {
    pub _base: Chuck_IO,
    pub m_callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_buffer: crate::dts::basic_istringstream<_CharT>,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    fn Chuck_IO_Chout_set_output_callback(
        this: *mut Chuck_IO_Chout,
        fp: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout"]
    fn Chuck_IO_Chout_Chuck_IO_Chout(this: *mut Chuck_IO_Chout, carrier: *mut Chuck_Carrier);
}
impl Chuck_IO_Chout {
    #[inline]
    unsafe fn set_output_callback(
        &mut self,
        fp: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) {
        Chuck_IO_Chout_set_output_callback(self, fp)
    }
    #[inline]
    unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_IO_Chout_Chuck_IO_Chout(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout_destructor"]
    fn Chuck_IO_Chout_Chuck_IO_Chout_destructor(this: *mut Chuck_IO_Chout);
}
extern "C" {
    #[link_name = "\u{1}good"]
    fn Chuck_IO_Chout_good(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    fn Chuck_IO_Chout_close(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    fn Chuck_IO_Chout_flush(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    fn Chuck_IO_Chout_mode(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    fn Chuck_IO_Chout_mode1(this: *mut c_void, flag: c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    fn Chuck_IO_Chout_readLine(this: *mut c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    fn Chuck_IO_Chout_readInt(this: *mut c_void, flags: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    fn Chuck_IO_Chout_readFloat(this: *mut c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    fn Chuck_IO_Chout_readString(this: *mut c_void, str: *mut crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    fn Chuck_IO_Chout_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_Chout_write(this: *mut c_void, val: *const crate::rtmidi_h_edited::string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_Chout_write1(this: *mut c_void, val: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_Chout_write2(this: *mut c_void, val: c_long, flags: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_Chout_write3(this: *mut c_void, val: f64);
}
#[repr(C)]
pub struct Chuck_IO_Cherr {
    pub _base: Chuck_IO,
    pub m_callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_buffer: crate::dts::basic_istringstream<_CharT>,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    fn Chuck_IO_Cherr_set_output_callback(
        this: *mut Chuck_IO_Cherr,
        fp: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr"]
    fn Chuck_IO_Cherr_Chuck_IO_Cherr(this: *mut Chuck_IO_Cherr, carrier: *mut Chuck_Carrier);
}
impl Chuck_IO_Cherr {
    #[inline]
    unsafe fn set_output_callback(
        &mut self,
        fp: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) {
        Chuck_IO_Cherr_set_output_callback(self, fp)
    }
    #[inline]
    unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_IO_Cherr_Chuck_IO_Cherr(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr_destructor"]
    fn Chuck_IO_Cherr_Chuck_IO_Cherr_destructor(this: *mut Chuck_IO_Cherr);
}
extern "C" {
    #[link_name = "\u{1}good"]
    fn Chuck_IO_Cherr_good(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    fn Chuck_IO_Cherr_close(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    fn Chuck_IO_Cherr_flush(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    fn Chuck_IO_Cherr_mode(this: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    fn Chuck_IO_Cherr_mode1(this: *mut c_void, flag: c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    fn Chuck_IO_Cherr_readLine(this: *mut c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    fn Chuck_IO_Cherr_readInt(this: *mut c_void, flags: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    fn Chuck_IO_Cherr_readFloat(this: *mut c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    fn Chuck_IO_Cherr_readString(this: *mut c_void, str: *mut crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    fn Chuck_IO_Cherr_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_Cherr_write(this: *mut c_void, val: *const crate::rtmidi_h_edited::string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_Cherr_write1(this: *mut c_void, val: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_Cherr_write2(this: *mut c_void, val: c_long, flags: c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    fn Chuck_IO_Cherr_write3(this: *mut c_void, val: f64);
}
extern "C" {
    pub static mut g_default_chugin_path: [c_char; 0usize];
}
extern "C" {
    pub static mut g_chugin_path_envvar: [c_char; 0usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_UAnaBlobProxy {
    _unused: [u8; 0],
}
// pub mod Chuck_DL_Api {
//     #[allow(unused_imports)]
//     pub type Object = *mut c_void;
//     pub type Type = *mut c_void;
//     pub type String = *mut c_void;
//     #[repr(C)]
//     #[derive(Debug, Copy, Clone)]
//     pub struct Api {
//         pub vm: *mut Chuck_DL_Api::Api_VMApi,
//         pub object: *mut Chuck_DL_Api::Api_ObjectApi,
//     }
//     #[repr(C)]
//     #[derive(Debug, Copy, Clone)]
//     pub struct Api_VMApi {
//         pub get_srate:
//             Option<unsafe extern "C" fn(arg1: CK_DL_API, arg2: *mut Chuck_VM_Shred) -> c_ulong>,
//     }
//     extern "C" {
//         #[link_name = "\u{1}VMApi"]
//         fn Api_VMApi_VMApi(this: *mut Chuck_DL_Api::Api_VMApi);
//     }
//     impl Api_VMApi {
//         #[inline]
//         unsafe fn new() -> Self {
//             let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
//             Api_VMApi_VMApi(&mut __bindgen_tmp);
//             __bindgen_tmp
//         }
//     }
//     #[repr(C)]
//     #[derive(Debug, Copy, Clone)]
//     pub struct Api_ObjectApi {
//         pub get_type: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 arg2: *mut Chuck_VM_Shred,
//                 name: *mut crate::rtmidi_h_edited::string,
//             ) -> Chuck_DL_Api::Type,
//         >,
//         pub create: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 arg2: *mut Chuck_VM_Shred,
//                 type_: Chuck_DL_Api::Type,
//             ) -> Chuck_DL_Api::Object,
//         >,
//         pub create_string: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 arg2: *mut Chuck_VM_Shred,
//                 value: *mut crate::rtmidi_h_edited::string,
//             ) -> Chuck_DL_Api::String,
//         >,
//         pub get_mvar_int: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 object: Chuck_DL_Api::Object,
//                 name: *mut crate::rtmidi_h_edited::string,
//                 value: *mut c_long,
//             ) -> c_ulong,
//         >,
//         pub get_mvar_float: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 object: Chuck_DL_Api::Object,
//                 name: *mut crate::rtmidi_h_edited::string,
//                 value: *mut f64,
//             ) -> c_ulong,
//         >,
//         pub get_mvar_dur: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 object: Chuck_DL_Api::Object,
//                 name: *mut crate::rtmidi_h_edited::string,
//                 value: *mut f64,
//             ) -> c_ulong,
//         >,
//         pub get_mvar_time: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 object: Chuck_DL_Api::Object,
//                 name: *mut crate::rtmidi_h_edited::string,
//                 value: *mut f64,
//             ) -> c_ulong,
//         >,
//         pub get_mvar_string: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 object: Chuck_DL_Api::Object,
//                 name: *mut crate::rtmidi_h_edited::string,
//                 value: *mut Chuck_DL_Api::String,
//             ) -> c_ulong,
//         >,
//         pub get_mvar_object: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 object: Chuck_DL_Api::Object,
//                 name: *mut crate::rtmidi_h_edited::string,
//                 value: *mut Chuck_DL_Api::Object,
//             ) -> c_ulong,
//         >,
//         pub set_string: Option<
//             unsafe extern "C" fn(
//                 arg1: CK_DL_API,
//                 string: Chuck_DL_Api::String,
//                 value: *mut crate::rtmidi_h_edited::string,
//             ) -> c_ulong,
//         >,
//     }
//     extern "C" {
//         #[link_name = "\u{1}ObjectApi"]
//         fn Api_ObjectApi_ObjectApi(this: *mut Chuck_DL_Api::Api_ObjectApi);
//     }
//     impl Api_ObjectApi {
//         #[inline]
//         unsafe fn new() -> Self {
//             let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
//             Api_ObjectApi_ObjectApi(&mut __bindgen_tmp);
//             __bindgen_tmp
//         }
//     }
//     extern "C" {
//         #[link_name = "\u{1}g_api"]
//         static mut Api_g_api: Chuck_DL_Api::Api;
//     }
//     extern "C" {
//         #[link_name = "\u{1}instance"]
//         pub fn Api_instance() -> *const Chuck_DL_Api::Api;
//     }
//     extern "C" {
//         #[link_name = "\u{1}Api"]
//         pub fn Api_Api(this: *mut Chuck_DL_Api::Api);
//     }
//     impl Api {
//         #[inline]
//         unsafe fn instance() -> *const Chuck_DL_Api::Api {
//             Api_instance()
//         }
//         #[inline]
//         unsafe fn new() -> Self {
//             let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
//             Api_Api(&mut __bindgen_tmp);
//             __bindgen_tmp
//         }
//     }
// }
// pub type CK_DL_API = *const Chuck_DL_Api::Api;
pub type f_ck_declversion = Option<unsafe extern "C" fn() -> c_ulong>;
pub type f_ck_query = Option<unsafe extern "C" fn(QUERY: *mut Chuck_DL_Query) -> c_ulong>;
pub type f_alloc = Option<
    unsafe extern "C" fn(
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ) -> *mut Chuck_Object,
>;
pub type f_ctor = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_dtor = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_mfun = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_sfun = Option<
    unsafe extern "C" fn(
        ARGS: *mut c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_tick = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        in_: f32,
        out: *mut f32,
        API: CK_DL_API,
    ) -> c_ulong,
>;
pub type f_tickf = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        in_: *mut f32,
        out: *mut f32,
        nframes: c_ulong,
        API: CK_DL_API,
    ) -> c_ulong,
>;
pub type f_ctrl = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_cget = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_pmsg = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        MSG: *const c_char,
        ARGS: *mut c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ) -> c_ulong,
>;
pub type f_tock = Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        UANA: *mut Chuck_UAna,
        BLOB: *mut Chuck_UAnaBlobProxy,
        API: CK_DL_API,
    ) -> c_ulong,
>;
pub type f_mainthreadhook = Option<unsafe extern "C" fn(bindle: *mut c_void) -> c_ulong>;
pub type f_mainthreadquit = Option<unsafe extern "C" fn(bindle: *mut c_void) -> c_ulong>;
pub type f_setname = Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, name: *const c_char)>;
pub type f_begin_class = Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, name: *const c_char, parent: *const c_char),
>;
pub type f_add_ctor = Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, ctor: f_ctor)>;
pub type f_add_dtor = Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, dtor: f_dtor)>;
pub type f_add_mfun = Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        mfun: f_mfun,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_add_sfun = Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        sfun: f_sfun,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_add_mvar = Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
    ) -> c_ulong,
>;
pub type f_add_svar = Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
        static_addr: *mut c_void,
    ),
>;
pub type f_add_arg = Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, type_: *const c_char, name: *const c_char),
>;
pub type f_add_ugen_func = Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        tick: f_tick,
        pmsg: f_pmsg,
        num_in: c_ulong,
        num_out: c_ulong,
    ),
>;
pub type f_add_ugen_funcf = Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        tickf: f_tickf,
        pmsg: f_pmsg,
        num_in: c_ulong,
        num_out: c_ulong,
    ),
>;
pub type f_add_ugen_funcf_auto_num_channels =
    Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, tickf: f_tickf, psmg: f_pmsg)>;
pub type f_add_ugen_ctrl = Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        ctrl: f_ctrl,
        cget: f_cget,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_end_class = Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query) -> c_ulong>;
pub type f_doc_class =
    Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, doc: *const c_char) -> c_ulong>;
pub type f_add_example =
    Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, ex: *const c_char) -> c_ulong>;
pub type f_doc_func =
    Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, doc: *const c_char) -> c_ulong>;
pub type f_doc_var =
    Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, doc: *const c_char) -> c_ulong>;
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
    pub dll_name: crate::rtmidi_h_edited::string,
    pub curr_class: *mut Chuck_DL_Class,
    pub curr_func: *mut Chuck_DL_Func,
    pub name: crate::rtmidi_h_edited::string,
    pub classes: crate::dts::vector,
    pub stack: crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}compiler"]
    fn Chuck_DL_Query_compiler(this: *const Chuck_DL_Query) -> *mut Chuck_Compiler;
}
extern "C" {
    #[link_name = "\u{1}vm"]
    fn Chuck_DL_Query_vm(this: *const Chuck_DL_Query) -> *mut Chuck_VM;
}
extern "C" {
    #[link_name = "\u{1}env"]
    fn Chuck_DL_Query_env(this: *const Chuck_DL_Query) -> *mut Chuck_Env;
}
extern "C" {
    #[link_name = "\u{1}clear"]
    fn Chuck_DL_Query_clear(this: *mut Chuck_DL_Query);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Query"]
    fn Chuck_DL_Query_Chuck_DL_Query(this: *mut Chuck_DL_Query, carrier: *mut Chuck_Carrier);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Query_destructor"]
    fn Chuck_DL_Query_Chuck_DL_Query_destructor(this: *mut Chuck_DL_Query);
}
impl Chuck_DL_Query {
    #[inline]
    unsafe fn compiler(&self) -> *mut Chuck_Compiler {
        Chuck_DL_Query_compiler(self)
    }
    #[inline]
    unsafe fn vm(&self) -> *mut Chuck_VM {
        Chuck_DL_Query_vm(self)
    }
    #[inline]
    unsafe fn env(&self) -> *mut Chuck_Env {
        Chuck_DL_Query_env(self)
    }
    #[inline]
    unsafe fn clear(&mut self) {
        Chuck_DL_Query_clear(self)
    }
    #[inline]
    unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Query_Chuck_DL_Query(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        Chuck_DL_Query_Chuck_DL_Query_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_DL_Class {
    pub name: crate::rtmidi_h_edited::string,
    pub parent: crate::rtmidi_h_edited::string,
    pub ctors: crate::dts::vector,
    pub dtor: *mut Chuck_DL_Func,
    pub mfuns: crate::dts::vector,
    pub sfuns: crate::dts::vector,
    pub mvars: crate::dts::vector,
    pub svars: crate::dts::vector,
    pub ugen_tick: f_tick,
    pub ugen_tickf: f_tickf,
    pub ugen_pmsg: f_pmsg,
    pub ugen_ctrl: crate::dts::vector,
    pub uana_tock: f_tock,
    pub classes: crate::dts::vector,
    pub current_mvar_offset: c_ulong,
    pub ugen_num_in: c_ulong,
    pub ugen_num_out: c_ulong,
    pub doc: crate::rtmidi_h_edited::string,
    pub examples: crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Class"]
    fn Chuck_DL_Class_Chuck_DL_Class(this: *mut Chuck_DL_Class);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Class_destructor"]
    fn Chuck_DL_Class_Chuck_DL_Class_destructor(this: *mut Chuck_DL_Class);
}
impl Chuck_DL_Class {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Class_Chuck_DL_Class(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        Chuck_DL_Class_Chuck_DL_Class_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_DL_Value {
    pub name: crate::rtmidi_h_edited::string,
    pub type_: crate::rtmidi_h_edited::string,
    pub is_const: c_ulong,
    pub static_addr: *mut c_void,
    pub doc: crate::rtmidi_h_edited::string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Value"]
    fn Chuck_DL_Value_Chuck_DL_Value(this: *mut Chuck_DL_Value);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Value"]
    fn Chuck_DL_Value_Chuck_DL_Value1(
        this: *mut Chuck_DL_Value,
        t: *const c_char,
        n: *const c_char,
        c: c_ulong,
        a: *mut c_void,
    );
}
impl Chuck_DL_Value {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Value_Chuck_DL_Value(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn new1(t: *const c_char, n: *const c_char, c: c_ulong, a: *mut c_void) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Value_Chuck_DL_Value1(&mut __bindgen_tmp, t, n, c, a);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DL_Func {
    pub name: crate::rtmidi_h_edited::string,
    pub type_: crate::rtmidi_h_edited::string,
    pub __bindgen_anon_1: Chuck_DL_Func__bindgen_ty_1,
    pub args: crate::dts::vector,
    pub doc: crate::rtmidi_h_edited::string,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
union Chuck_DL_Func__bindgen_ty_1 {
    ctor: f_ctor,
    dtor: f_dtor,
    mfun: f_mfun,
    sfun: f_sfun,
    addr: c_ulong,
    _bindgen_union_align: u64,
}
extern "C" {
    #[link_name = "\u{1}add_arg"]
    fn Chuck_DL_Func_add_arg(this: *mut Chuck_DL_Func, t: *const c_char, n: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func"]
    fn Chuck_DL_Func_Chuck_DL_Func(this: *mut Chuck_DL_Func);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func"]
    fn Chuck_DL_Func_Chuck_DL_Func1(
        this: *mut Chuck_DL_Func,
        t: *const c_char,
        n: *const c_char,
        a: c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func_destructor"]
    fn Chuck_DL_Func_Chuck_DL_Func_destructor(this: *mut Chuck_DL_Func);
}
impl Chuck_DL_Func {
    #[inline]
    unsafe fn add_arg(&mut self, t: *const c_char, n: *const c_char) {
        Chuck_DL_Func_add_arg(self, t, n)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Func_Chuck_DL_Func(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn new1(t: *const c_char, n: *const c_char, a: c_ulong) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Func_Chuck_DL_Func1(&mut __bindgen_tmp, t, n, a);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        Chuck_DL_Func_Chuck_DL_Func_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_DL_Ctrl {
    pub name: crate::rtmidi_h_edited::string,
    pub type_: crate::rtmidi_h_edited::string,
    pub types: crate::dts::vector,
    pub ctrl: f_ctrl,
    pub cget: f_cget,
}
extern "C" {
    fn make_new_mfun(t: *const c_char, n: *const c_char, mfun: f_mfun) -> *mut Chuck_DL_Func;
}
extern "C" {
    fn make_new_sfun(t: *const c_char, n: *const c_char, sfun: f_sfun) -> *mut Chuck_DL_Func;
}
extern "C" {
    fn make_new_arg(t: *const c_char, n: *const c_char) -> *mut Chuck_DL_Value;
}
extern "C" {
    fn make_new_mvar(t: *const c_char, n: *const c_char, c: c_ulong) -> *mut Chuck_DL_Value;
}
extern "C" {
    fn make_new_svar(
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
    pub v_complex: t_CKCOMPLEX,
    pub v_polar: t_CKPOLAR,
    pub v_vec3: t_CKVEC3,
    pub v_vec4: t_CKVEC4,
    pub v_object: *mut Chuck_Object,
    pub v_string: *mut Chuck_String,
    pub _bindgen_union_align: [u64; 4usize],
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Return"]
    fn Chuck_DL_Return_Chuck_DL_Return(this: *mut Chuck_DL_Return);
}
impl Chuck_DL_Return {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DL_Return_Chuck_DL_Return(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DLL {
    pub _base: Chuck_VM_Object,
    pub m_handle: *mut c_void,
    pub m_last_error: crate::rtmidi_h_edited::string,
    pub m_filename: crate::rtmidi_h_edited::string,
    pub m_id: crate::rtmidi_h_edited::string,
    pub m_func: crate::rtmidi_h_edited::string,
    pub m_done_query: c_ulong,
    pub m_version_func: f_ck_declversion,
    pub m_query_func: f_ck_query,
    pub m_query: Chuck_DL_Query,
}
extern "C" {
    #[link_name = "\u{1}load"]
    fn Chuck_DLL_load(
        this: *mut Chuck_DLL,
        filename: *const c_char,
        func: *const c_char,
        lazy: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}load"]
    fn Chuck_DLL_load1(this: *mut Chuck_DLL, query_func: f_ck_query, lazy: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_addr"]
    fn Chuck_DLL_get_addr(this: *mut Chuck_DLL, symbol: *const c_char) -> *mut c_void;
}
extern "C" {
    #[link_name = "\u{1}last_error"]
    fn Chuck_DLL_last_error(this: *const Chuck_DLL) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}unload"]
    fn Chuck_DLL_unload(this: *mut Chuck_DLL) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}query"]
    fn Chuck_DLL_query(this: *mut Chuck_DLL) -> *const Chuck_DL_Query;
}
extern "C" {
    #[link_name = "\u{1}good"]
    fn Chuck_DLL_good(this: *const Chuck_DLL) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}name"]
    fn Chuck_DLL_name(this: *const Chuck_DLL) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_DLL"]
    fn Chuck_DLL_Chuck_DLL(this: *mut Chuck_DLL, carrier: *mut Chuck_Carrier, xid: *const c_char);
}
impl Chuck_DLL {
    #[inline]
    unsafe fn load(
        &mut self,
        filename: *const c_char,
        func: *const c_char,
        lazy: c_ulong,
    ) -> c_ulong {
        Chuck_DLL_load(self, filename, func, lazy)
    }
    #[inline]
    unsafe fn load1(&mut self, query_func: f_ck_query, lazy: c_ulong) -> c_ulong {
        Chuck_DLL_load1(self, query_func, lazy)
    }
    #[inline]
    unsafe fn get_addr(&mut self, symbol: *const c_char) -> *mut c_void {
        Chuck_DLL_get_addr(self, symbol)
    }
    #[inline]
    unsafe fn last_error(&self) -> *const c_char {
        Chuck_DLL_last_error(self)
    }
    #[inline]
    unsafe fn unload(&mut self) -> c_ulong {
        Chuck_DLL_unload(self)
    }
    #[inline]
    unsafe fn query(&mut self) -> *const Chuck_DL_Query {
        Chuck_DLL_query(self)
    }
    #[inline]
    unsafe fn good(&self) -> c_ulong {
        Chuck_DLL_good(self)
    }
    #[inline]
    unsafe fn name(&self) -> *const c_char {
        Chuck_DLL_name(self)
    }
    #[inline]
    unsafe fn new(carrier: *mut Chuck_Carrier, xid: *const c_char) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_DLL_Chuck_DLL(&mut __bindgen_tmp, carrier, xid);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_DLL_destructor"]
    fn Chuck_DLL_Chuck_DLL_destructor(this: *mut Chuck_DLL);
}
const RTLD_DI_LMID: _bindgen_ty_48 = 1;
const RTLD_DI_LINKMAP: _bindgen_ty_48 = 2;
const RTLD_DI_CONFIGADDR: _bindgen_ty_48 = 3;
const RTLD_DI_SERINFO: _bindgen_ty_48 = 4;
const RTLD_DI_SERINFOSIZE: _bindgen_ty_48 = 5;
const RTLD_DI_ORIGIN: _bindgen_ty_48 = 6;
const RTLD_DI_PROFILENAME: _bindgen_ty_48 = 7;
const RTLD_DI_PROFILEOUT: _bindgen_ty_48 = 8;
const RTLD_DI_TLS_MODID: _bindgen_ty_48 = 9;
const RTLD_DI_TLS_DATA: _bindgen_ty_48 = 10;
const RTLD_DI_MAX: _bindgen_ty_48 = 10;
type _bindgen_ty_48 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Dl_serpath {
    dls_name: *mut c_char,
    dls_flags: c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Dl_serinfo {
    dls_size: usize,
    dls_cnt: c_uint,
    dls_serpath: [Dl_serpath; 1usize],
}
extern "C" {
    static mut EM_anyErrors: c_ulong;
}
extern "C" {
    static mut EM_tokPos: c_int;
}
extern "C" {
    static mut EM_lineNum: c_int;
}
extern "C" {
    static mut EM_extLineNum: c_int;
}
extern "C" {
    fn EM_newline();
}
extern "C" {
    fn ck_fprintf_stdout(format: *const c_char, ...);
}
extern "C" {
    fn ck_fprintf_stderr(format: *const c_char, ...);
}
extern "C" {
    fn ck_fflush_stdout();
}
extern "C" {
    fn ck_fflush_stderr();
}
extern "C" {
    fn ck_vfprintf_stdout(format: *const c_char, args: *mut __va_list_tag);
}
extern "C" {
    fn ck_vfprintf_stderr(format: *const c_char, args: *mut __va_list_tag);
}
extern "C" {
    fn ck_set_stdout_callback(callback: Option<unsafe extern "C" fn(arg1: *const c_char)>);
}
extern "C" {
    fn ck_set_stderr_callback(callback: Option<unsafe extern "C" fn(arg1: *const c_char)>);
}
#[repr(C)]
pub struct ChuckOutStream {
    pub m_stream: crate::dts::basic_istringstream<_CharT>,
    pub m_callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_isErr: bool,
}
extern "C" {
    #[link_name = "\u{1}set_callback"]
    fn ChuckOutStream_set_callback(
        this: *mut ChuckOutStream,
        callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}ChuckOutStream"]
    fn ChuckOutStream_ChuckOutStream(this: *mut ChuckOutStream, isErr: bool);
}
extern "C" {
    #[link_name = "\u{1}ChuckOutStream_destructor"]
    fn ChuckOutStream_ChuckOutStream_destructor(this: *mut ChuckOutStream);
}
impl ChuckOutStream {
    #[inline]
    unsafe fn set_callback(&mut self, callback: Option<unsafe extern "C" fn(arg1: *const c_char)>) {
        ChuckOutStream_set_callback(self, callback)
    }
    #[inline]
    unsafe fn new(isErr: bool) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        ChuckOutStream_ChuckOutStream(&mut __bindgen_tmp, isErr);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        ChuckOutStream_ChuckOutStream_destructor(self)
    }
}
extern "C" {
    pub static mut g_ck_stdoutstream: ChuckOutStream;
}
extern "C" {
    pub static mut g_ck_stderrstream: ChuckOutStream;
}
#[repr(C)]
pub struct Chuck_Scope {
    pub scope: crate::dts::vector,
    pub commit_map: std::collections::HashMap::new(),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Multi {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_Namespace {
    pub _base: Chuck_VM_Object,
    pub type_: Chuck_Scope,
    pub value: Chuck_Scope,
    pub func: Chuck_Scope,
    pub obj_v_table: Chuck_VTable,
    pub class_data: *mut c_uchar,
    pub class_data_size: c_ulong,
    pub name: crate::rtmidi_h_edited::string,
    pub pre_ctor: *mut Chuck_VM_Code,
    pub dtor: *mut Chuck_VM_Code,
    pub parent: *mut Chuck_Namespace,
    pub offset: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}lookup_type"]
    fn Chuck_Namespace_lookup_type(
        this: *mut Chuck_Namespace,
        name: *const crate::rtmidi_h_edited::string,
        climb: c_long,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}lookup_type"]
    fn Chuck_Namespace_lookup_type1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: c_long,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}lookup_value"]
    fn Chuck_Namespace_lookup_value(
        this: *mut Chuck_Namespace,
        name: *const crate::rtmidi_h_edited::string,
        climb: c_long,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}lookup_value"]
    fn Chuck_Namespace_lookup_value1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: c_long,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}lookup_func"]
    fn Chuck_Namespace_lookup_func(
        this: *mut Chuck_Namespace,
        name: *const crate::rtmidi_h_edited::string,
        climb: c_long,
    ) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}lookup_func"]
    fn Chuck_Namespace_lookup_func1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: c_long,
    ) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}commit"]
    fn Chuck_Namespace_commit(this: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}rollback"]
    fn Chuck_Namespace_rollback(this: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}get_types"]
    fn Chuck_Namespace_get_types(this: *mut Chuck_Namespace, out: *mut crate::dts::vector);
}
extern "C" {
    #[link_name = "\u{1}get_values"]
    fn Chuck_Namespace_get_values(this: *mut Chuck_Namespace, out: *mut crate::dts::vector);
}
extern "C" {
    #[link_name = "\u{1}get_funcs"]
    fn Chuck_Namespace_get_funcs(this: *mut Chuck_Namespace, out: *mut crate::dts::vector);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Namespace"]
    fn Chuck_Namespace_Chuck_Namespace(this: *mut Chuck_Namespace);
}
impl Chuck_Namespace {
    #[inline]
    unsafe fn lookup_type(&mut self, name: *const crate::rtmidi_h_edited::string, climb: c_long) -> *mut Chuck_Type {
        Chuck_Namespace_lookup_type(self, name, climb)
    }
    #[inline]
    unsafe fn lookup_type1(&mut self, name: S_Symbol, climb: c_long) -> *mut Chuck_Type {
        Chuck_Namespace_lookup_type1(self, name, climb)
    }
    #[inline]
    unsafe fn lookup_value(&mut self, name: *const crate::rtmidi_h_edited::string, climb: c_long) -> *mut Chuck_Value {
        Chuck_Namespace_lookup_value(self, name, climb)
    }
    #[inline]
    unsafe fn lookup_value1(&mut self, name: S_Symbol, climb: c_long) -> *mut Chuck_Value {
        Chuck_Namespace_lookup_value1(self, name, climb)
    }
    #[inline]
    unsafe fn lookup_func(&mut self, name: *const crate::rtmidi_h_edited::string, climb: c_long) -> *mut Chuck_Func {
        Chuck_Namespace_lookup_func(self, name, climb)
    }
    #[inline]
    unsafe fn lookup_func1(&mut self, name: S_Symbol, climb: c_long) -> *mut Chuck_Func {
        Chuck_Namespace_lookup_func1(self, name, climb)
    }
    #[inline]
    unsafe fn commit(&mut self) {
        Chuck_Namespace_commit(self)
    }
    #[inline]
    unsafe fn rollback(&mut self) {
        Chuck_Namespace_rollback(self)
    }
    #[inline]
    unsafe fn get_types(&mut self, out: *mut crate::dts::vector) {
        Chuck_Namespace_get_types(self, out)
    }
    #[inline]
    unsafe fn get_values(&mut self, out: *mut crate::dts::vector) {
        Chuck_Namespace_get_values(self, out)
    }
    #[inline]
    unsafe fn get_funcs(&mut self, out: *mut crate::dts::vector) {
        Chuck_Namespace_get_funcs(self, out)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Namespace_Chuck_Namespace(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Namespace_destructor"]
    fn Chuck_Namespace_Chuck_Namespace_destructor(this: *mut Chuck_Namespace);
}
#[repr(C)]
pub struct Chuck_Context {
    pub _base: Chuck_VM_Object,
    pub filename: crate::rtmidi_h_edited::string,
    pub full_path: crate::rtmidi_h_edited::string,
    pub parse_tree: crate::chuck_absyn_h_edited::a_Program,
    pub nspc: *mut Chuck_Namespace,
    pub public_class_def: crate::chuck_absyn_h_edited::a_Class_Def,
    pub has_error: c_ulong,
    pub progress: c_ulong,
    pub new_types: crate::dts::vector,
    pub new_values: crate::dts::vector,
    pub new_funcs: crate::dts::vector,
    pub new_nspc: crate::dts::vector,
    pub commit_map: std::collections::HashMap::new(),
}
const Chuck_Context_P_NONE: Chuck_Context__bindgen_ty_1 = 0;
const Chuck_Context_P_CLASSES_ONLY: Chuck_Context__bindgen_ty_1 = 1;
const Chuck_Context_P_ALL: Chuck_Context__bindgen_ty_1 = 2;
type Chuck_Context__bindgen_ty_1 = u32;
extern "C" {
    #[link_name = "\u{1}add_commit_candidate"]
    fn Chuck_Context_add_commit_candidate(this: *mut Chuck_Context, nspc: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}commit"]
    fn Chuck_Context_commit(this: *mut Chuck_Context);
}
extern "C" {
    #[link_name = "\u{1}rollback"]
    fn Chuck_Context_rollback(this: *mut Chuck_Context);
}
extern "C" {
    #[link_name = "\u{1}code"]
    fn Chuck_Context_code(this: *mut Chuck_Context) -> *mut Chuck_VM_Code;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Type"]
    fn Chuck_Context_new_Chuck_Type(
        this: *mut Chuck_Context,
        env: *mut Chuck_Env,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Value"]
    fn Chuck_Context_new_Chuck_Value(
        this: *mut Chuck_Context,
        t: *mut Chuck_Type,
        name: *const crate::rtmidi_h_edited::string,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Func"]
    fn Chuck_Context_new_Chuck_Func(this: *mut Chuck_Context) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Namespace"]
    fn Chuck_Context_new_Chuck_Namespace(this: *mut Chuck_Context) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Context"]
    fn Chuck_Context_Chuck_Context(this: *mut Chuck_Context);
}
impl Chuck_Context {
    #[inline]
    unsafe fn add_commit_candidate(&mut self, nspc: *mut Chuck_Namespace) {
        Chuck_Context_add_commit_candidate(self, nspc)
    }
    #[inline]
    unsafe fn commit(&mut self) {
        Chuck_Context_commit(self)
    }
    #[inline]
    unsafe fn rollback(&mut self) {
        Chuck_Context_rollback(self)
    }
    #[inline]
    unsafe fn code(&mut self) -> *mut Chuck_VM_Code {
        Chuck_Context_code(self)
    }
    #[inline]
    unsafe fn new_Chuck_Type(&mut self, env: *mut Chuck_Env) -> *mut Chuck_Type {
        Chuck_Context_new_Chuck_Type(self, env)
    }
    #[inline]
    unsafe fn new_Chuck_Value(
        &mut self,
        t: *mut Chuck_Type,
        name: *const crate::rtmidi_h_edited::string,
    ) -> *mut Chuck_Value {
        Chuck_Context_new_Chuck_Value(self, t, name)
    }
    #[inline]
    unsafe fn new_Chuck_Func(&mut self) -> *mut Chuck_Func {
        Chuck_Context_new_Chuck_Func(self)
    }
    #[inline]
    unsafe fn new_Chuck_Namespace(&mut self) -> *mut Chuck_Namespace {
        Chuck_Context_new_Chuck_Namespace(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Context_Chuck_Context(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Context_destructor"]
    fn Chuck_Context_Chuck_Context_destructor(this: *mut Chuck_Context);
}
#[repr(C)]
pub struct Chuck_Env {
    pub _base: Chuck_VM_Object,
    pub m_carrier: *mut Chuck_Carrier,
    pub global_nspc: *mut Chuck_Namespace,
    pub global_context: Chuck_Context,
    pub user_nspc: *mut Chuck_Namespace,
    pub nspc_stack: crate::dts::vector,
    pub curr: *mut Chuck_Namespace,
    pub class_stack: crate::dts::vector,
    pub class_def: *mut Chuck_Type,
    pub func: *mut Chuck_Func,
    pub class_scope: c_ulong,
    pub contexts: crate::dts::vector,
    pub context: *mut Chuck_Context,
    pub breaks: crate::dts::vector,
    pub key_words: std::collections::HashMap::new(),
    pub key_types: std::collections::HashMap::new(),
    pub key_values: std::collections::HashMap::new(),
    pub deprecated: std::collections::HashMap::new(),
    pub deprecate_level: c_long,
    pub t_void: *mut Chuck_Type,
    pub t_int: *mut Chuck_Type,
    pub t_float: *mut Chuck_Type,
    pub t_time: *mut Chuck_Type,
    pub t_dur: *mut Chuck_Type,
    pub t_complex: *mut Chuck_Type,
    pub t_polar: *mut Chuck_Type,
    pub t_vec3: *mut Chuck_Type,
    pub t_vec4: *mut Chuck_Type,
    pub t_null: *mut Chuck_Type,
    pub t_function: *mut Chuck_Type,
    pub t_object: *mut Chuck_Type,
    pub t_array: *mut Chuck_Type,
    pub t_string: *mut Chuck_Type,
    pub t_event: *mut Chuck_Type,
    pub t_ugen: *mut Chuck_Type,
    pub t_uana: *mut Chuck_Type,
    pub t_uanablob: *mut Chuck_Type,
    pub t_shred: *mut Chuck_Type,
    pub t_io: *mut Chuck_Type,
    pub t_fileio: *mut Chuck_Type,
    pub t_chout: *mut Chuck_Type,
    pub t_cherr: *mut Chuck_Type,
    pub t_thread: *mut Chuck_Type,
    pub t_class: *mut Chuck_Type,
    pub t_dac: *mut Chuck_Type,
    pub t_adc: *mut Chuck_Type,
}
extern "C" {
    #[link_name = "\u{1}set_carrier"]
    fn Chuck_Env_set_carrier(this: *mut Chuck_Env, carrier: *mut Chuck_Carrier);
}
extern "C" {
    #[link_name = "\u{1}vm"]
    fn Chuck_Env_vm(this: *mut Chuck_Env) -> *mut Chuck_VM;
}
extern "C" {
    #[link_name = "\u{1}global"]
    fn Chuck_Env_global(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}user"]
    fn Chuck_Env_user(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}reset"]
    fn Chuck_Env_reset(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}load_user_namespace"]
    fn Chuck_Env_load_user_namespace(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}clear_user_namespace"]
    fn Chuck_Env_clear_user_namespace(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}nspc_top"]
    fn Chuck_Env_nspc_top(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}class_top"]
    fn Chuck_Env_class_top(this: *mut Chuck_Env) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}is_global"]
    fn Chuck_Env_is_global(this: *mut Chuck_Env) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Env"]
    fn Chuck_Env_Chuck_Env(this: *mut Chuck_Env);
}
impl Chuck_Env {
    #[inline]
    unsafe fn set_carrier(&mut self, carrier: *mut Chuck_Carrier) {
        Chuck_Env_set_carrier(self, carrier)
    }
    #[inline]
    unsafe fn vm(&mut self) -> *mut Chuck_VM {
        Chuck_Env_vm(self)
    }
    #[inline]
    unsafe fn global(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_global(self)
    }
    #[inline]
    unsafe fn user(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_user(self)
    }
    #[inline]
    unsafe fn reset(&mut self) {
        Chuck_Env_reset(self)
    }
    #[inline]
    unsafe fn load_user_namespace(&mut self) {
        Chuck_Env_load_user_namespace(self)
    }
    #[inline]
    unsafe fn clear_user_namespace(&mut self) {
        Chuck_Env_clear_user_namespace(self)
    }
    #[inline]
    unsafe fn nspc_top(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_nspc_top(self)
    }
    #[inline]
    unsafe fn class_top(&mut self) -> *mut Chuck_Type {
        Chuck_Env_class_top(self)
    }
    #[inline]
    unsafe fn is_global(&mut self) -> c_ulong {
        Chuck_Env_is_global(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Env_Chuck_Env(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Env_destructor"]
    fn Chuck_Env_Chuck_Env_destructor(this: *mut Chuck_Env);
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_UGen_Info {
    pub _base: Chuck_VM_Object,
    pub tick: f_tick,
    pub tickf: f_tickf,
    pub pmsg: f_pmsg,
    pub num_ins: c_ulong,
    pub num_outs: c_ulong,
    pub tock: f_tock,
    pub num_ins_ana: c_ulong,
    pub num_outs_ana: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen_Info"]
    fn Chuck_UGen_Info_Chuck_UGen_Info(this: *mut Chuck_UGen_Info);
}
impl Chuck_UGen_Info {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_UGen_Info_Chuck_UGen_Info(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_Type {
    pub _base: Chuck_VM_Object,
    pub xid: te_Type,
    pub name: crate::rtmidi_h_edited::string,
    pub parent: *mut Chuck_Type,
    pub size: c_ulong,
    pub owner: *mut Chuck_Namespace,
    pub __bindgen_anon_1: Chuck_Type__bindgen_ty_1,
    pub array_depth: c_ulong,
    pub obj_size: c_ulong,
    pub info: *mut Chuck_Namespace,
    pub func: *mut Chuck_Func,
    pub def: crate::chuck_absyn_h_edited::a_Class_Def,
    pub ugen_info: *mut Chuck_UGen_Info,
    pub is_copy: c_ulong,
    pub is_complete: c_ulong,
    pub has_constructor: c_ulong,
    pub has_destructor: c_ulong,
    pub allocator: f_alloc,
    pub doc: crate::rtmidi_h_edited::string,
    pub examples: crate::dts::vector,
    pub ret: crate::rtmidi_h_edited::string,
    pub m_env: *mut Chuck_Env,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
union Chuck_Type__bindgen_ty_1 {
    array_type: *mut Chuck_Type,
    actual_type: *mut Chuck_Type,
    _bindgen_union_align: u64,
}
extern "C" {
    #[link_name = "\u{1}reset"]
    fn Chuck_Type_reset(this: *mut Chuck_Type);
}
extern "C" {
    #[link_name = "\u{1}copy"]
    fn Chuck_Type_copy(this: *const Chuck_Type, env: *mut Chuck_Env) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}str"]
    fn Chuck_Type_str(this: *mut Chuck_Type) -> *const crate::rtmidi_h_edited::string;
}
extern "C" {
    #[link_name = "\u{1}c_name"]
    fn Chuck_Type_c_name(this: *mut Chuck_Type) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Type"]
    fn Chuck_Type_Chuck_Type(
        this: *mut Chuck_Type,
        env: *mut Chuck_Env,
        _id: te_Type,
        _n: *const crate::rtmidi_h_edited::string,
        _p: *mut Chuck_Type,
        _s: c_ulong,
    );
}
impl Chuck_Type {
    #[inline]
    unsafe fn reset(&mut self) {
        Chuck_Type_reset(self)
    }
    #[inline]
    unsafe fn copy(&self, env: *mut Chuck_Env) -> *mut Chuck_Type {
        Chuck_Type_copy(self, env)
    }
    #[inline]
    unsafe fn str(&mut self) -> *const crate::rtmidi_h_edited::string {
        Chuck_Type_str(self)
    }
    #[inline]
    unsafe fn c_name(&mut self) -> *const c_char {
        Chuck_Type_c_name(self)
    }
    #[inline]
    unsafe fn new(
        env: *mut Chuck_Env,
        _id: te_Type,
        _n: *const crate::rtmidi_h_edited::string,
        _p: *mut Chuck_Type,
        _s: c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Type_Chuck_Type(&mut __bindgen_tmp, env, _id, _n, _p, _s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Type_destructor"]
    fn Chuck_Type_Chuck_Type_destructor(this: *mut Chuck_Type);
}
#[repr(C)]
pub struct Chuck_Value {
    pub _base: Chuck_VM_Object,
    pub type_: *mut Chuck_Type,
    pub name: crate::rtmidi_h_edited::string,
    pub offset: c_ulong,
    pub addr: *mut c_void,
    pub is_const: c_ulong,
    pub is_member: c_ulong,
    pub is_static: c_ulong,
    pub is_context_global: c_ulong,
    pub is_decl_checked: c_ulong,
    pub is_global: c_ulong,
    pub access: c_ulong,
    pub owner: *mut Chuck_Namespace,
    pub owner_class: *mut Chuck_Type,
    pub func_ref: *mut Chuck_Func,
    pub func_num_overloads: c_long,
    pub doc: crate::rtmidi_h_edited::string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Value"]
    fn Chuck_Value_Chuck_Value(
        this: *mut Chuck_Value,
        t: *mut Chuck_Type,
        n: *const crate::rtmidi_h_edited::string,
        a: *mut c_void,
        c: c_ulong,
        acc: c_ulong,
        o: *mut Chuck_Namespace,
        oc: *mut Chuck_Type,
        s: c_ulong,
    );
}
impl Chuck_Value {
    #[inline]
    unsafe fn new(
        t: *mut Chuck_Type,
        n: *const crate::rtmidi_h_edited::string,
        a: *mut c_void,
        c: c_ulong,
        acc: c_ulong,
        o: *mut Chuck_Namespace,
        oc: *mut Chuck_Type,
        s: c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Value_Chuck_Value(&mut __bindgen_tmp, t, n, a, c, acc, o, oc, s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Value_destructor"]
    fn Chuck_Value_Chuck_Value_destructor(this: *mut Chuck_Value);
}
#[repr(C)]
pub struct Chuck_Func {
    pub _base: Chuck_VM_Object,
    pub name: crate::rtmidi_h_edited::string,
    pub def: crate::chuck_absyn_h_edited::a_Func_Def,
    pub code: *mut Chuck_VM_Code,
    pub is_member: c_ulong,
    pub vt_index: c_ulong,
    pub value_ref: *mut Chuck_Value,
    pub next: *mut Chuck_Func,
    pub up: *mut Chuck_Value,
    pub doc: crate::rtmidi_h_edited::string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Func"]
    fn Chuck_Func_Chuck_Func(this: *mut Chuck_Func);
}
impl Chuck_Func {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Func_Chuck_Func(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Func_destructor"]
    fn Chuck_Func_Chuck_Func_destructor(this: *mut Chuck_Func);
}
extern "C" {
    fn type_engine_init(carrier: *mut Chuck_Carrier) -> *mut Chuck_Env;
}
extern "C" {
    fn type_engine_shutdown(env: *mut Chuck_Env);
}
extern "C" {
    fn type_engine_load_context(env: *mut Chuck_Env, context: *mut Chuck_Context) -> c_ulong;
}
extern "C" {
    fn type_engine_unload_context(env: *mut Chuck_Env) -> c_ulong;
}
extern "C" {
    fn type_engine_check_prog(
        env: *mut Chuck_Env,
        prog: crate::chuck_absyn_h_edited::a_Program,
        filename: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_make_context(prog: crate::chuck_absyn_h_edited::a_Program, filename: *const crate::rtmidi_h_edited::string) -> *mut Chuck_Context;
}
extern "C" {
    fn type_engine_check_context(
        env: *mut Chuck_Env,
        context: *mut Chuck_Context,
        how_much: te_HowMuch,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_check_stmt(env: *mut Chuck_Env, stmt: crate::chuck_absyn_h_edited::a_Stmt) -> c_ulong;
}
extern "C" {
    fn type_engine_check_exp(env: *mut Chuck_Env, exp: crate::chuck_absyn_h_edited::a_Exp) -> t_CKTYPE;
}
extern "C" {
    fn type_engine_add_dll(
        env: *mut Chuck_Env,
        dll: *mut Chuck_DLL,
        nspc: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_add_dll2(
        env: *mut Chuck_Env,
        dll: *mut Chuck_DLL,
        dest: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_add_class_from_dl(env: *mut Chuck_Env, c: *mut Chuck_DL_Class) -> c_ulong;
}
extern "C" {
    fn equals(lhs: *mut Chuck_Type, rhs: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn isa(lhs: *mut Chuck_Type, rhs: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn isprim(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn isobj(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn isfunc(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn iskindofint(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn getkindof(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn type_engine_import_class_begin(
        env: *mut Chuck_Env,
        type_: *mut Chuck_Type,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        doc: *const c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    fn type_engine_import_ugen_begin(
        env: *mut Chuck_Env,
        name: *const c_char,
        parent: *const c_char,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        tick: f_tick,
        tickf: f_tickf,
        pmsg: f_pmsg,
        num_ins: c_ulong,
        num_outs: c_ulong,
        doc: *const c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    fn type_engine_import_uana_begin(
        env: *mut Chuck_Env,
        name: *const c_char,
        parent: *const c_char,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        tick: f_tick,
        tock: f_tock,
        pmsg: f_pmsg,
        num_ins: c_ulong,
        num_outs: c_ulong,
        num_ins_ana: c_ulong,
        num_outs_ana: c_ulong,
        doc: *const c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    fn type_engine_import_mfun(env: *mut Chuck_Env, mfun: *mut Chuck_DL_Func) -> c_ulong;
}
extern "C" {
    fn type_engine_import_sfun(env: *mut Chuck_Env, sfun: *mut Chuck_DL_Func) -> c_ulong;
}
extern "C" {
    fn type_engine_import_mvar(
        env: *mut Chuck_Env,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
        doc: *const c_char,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_import_svar(
        env: *mut Chuck_Env,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
        addr: c_ulong,
        doc: *const c_char,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_import_ugen_ctrl(
        env: *mut Chuck_Env,
        type_: *const c_char,
        name: *const c_char,
        ctrl: f_ctrl,
        write: c_ulong,
        read: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_import_add_ex(env: *mut Chuck_Env, ex: *const c_char) -> c_ulong;
}
extern "C" {
    fn type_engine_import_class_end(env: *mut Chuck_Env) -> c_ulong;
}
extern "C" {
    fn type_engine_register_deprecate(
        env: *mut Chuck_Env,
        former: *const crate::rtmidi_h_edited::string,
        latter: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_check_reserved(env: *mut Chuck_Env, xid: *const crate::rtmidi_h_edited::string, pos: c_int) -> c_ulong;
}
extern "C" {
    fn type_engine_check_primitive(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn type_engine_compat_func(
        lhs: crate::chuck_absyn_h_edited::a_Func_Def,
        rhs: crate::chuck_absyn_h_edited::a_Func_Def,
        pos: c_int,
        err: *mut crate::rtmidi_h_edited::string,
        print: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_get_deprecate(
        env: *mut Chuck_Env,
        from: *const crate::rtmidi_h_edited::string,
        to: *mut crate::rtmidi_h_edited::string,
    ) -> c_ulong;
}
extern "C" {
    fn type_engine_find_common_anc(lhs: *mut Chuck_Type, rhs: *mut Chuck_Type) -> *mut Chuck_Type;
}
extern "C" {
    fn type_engine_find_type(env: *mut Chuck_Env, path: crate::chuck_absyn_h_edited::a_Id_List) -> *mut Chuck_Type;
}
extern "C" {
    fn type_engine_find_value(type_: *mut Chuck_Type, xid: *const crate::rtmidi_h_edited::string) -> *mut Chuck_Value;
}
extern "C" {
    fn type_engine_find_nspc(env: *mut Chuck_Env, path: crate::chuck_absyn_h_edited::a_Id_List) -> *mut Chuck_Namespace;
}
extern "C" {
    /// spencer: added this into function to provide the same logic path
    /// for type_engine_check_exp_decl() and ck_add_mvar() when they determine
    /// offsets for mvars -- added 1.3.0.0
    fn type_engine_next_offset(current_offset: c_ulong, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    fn verify_array(array: crate::chuck_absyn_h_edited::a_Array_Sub) -> c_ulong;
}
extern "C" {
    fn new_array_type(
        env: *mut Chuck_Env,
        array_parent: *mut Chuck_Type,
        depth: c_ulong,
        base_type: *mut Chuck_Type,
        owner_nspc: *mut Chuck_Namespace,
    ) -> *mut Chuck_Type;
}
extern "C" {
    fn type_path(path: crate::chuck_absyn_h_edited::a_Id_List) -> *const c_char;
}
extern "C" {
    fn str2list(path: *const crate::rtmidi_h_edited::string) -> crate::chuck_absyn_h_edited::a_Id_List;
}
extern "C" {
    fn howmuch2str(how_much: te_HowMuch) -> *const c_char;
}
extern "C" {
    fn escape_str(str_lit: *mut c_char, linepos: c_int) -> c_ulong;
}
extern "C" {
    fn str2char(char_lit: *const c_char, linepos: c_int) -> c_long;
}
extern "C" {
    fn type_engine_scan0_prog(env: *mut Chuck_Env, prog: crate::chuck_absyn_h_edited::a_Program, val: te_HowMuch) -> c_ulong;
}
extern "C" {
    fn type_engine_scan0_class_def(env: *mut Chuck_Env, def: crate::chuck_absyn_h_edited::a_Class_Def) -> c_ulong;
}
extern "C" {
    fn type_engine_scan1_prog(env: *mut Chuck_Env, prog: crate::chuck_absyn_h_edited::a_Program, val: te_HowMuch) -> c_ulong;
}
extern "C" {
    fn type_engine_scan1_class_def(env: *mut Chuck_Env, def: crate::chuck_absyn_h_edited::a_Class_Def) -> c_ulong;
}
extern "C" {
    fn type_engine_scan1_func_def(env: *mut Chuck_Env, def: crate::chuck_absyn_h_edited::a_Func_Def) -> c_ulong;
}
extern "C" {
    fn type_engine_scan1_exp_decl(env: *mut Chuck_Env, decl: crate::chuck_absyn_h_edited::a_Exp_Decl) -> c_ulong;
}
extern "C" {
    fn type_engine_scan2_prog(env: *mut Chuck_Env, prog: crate::chuck_absyn_h_edited::a_Program, val: te_HowMuch) -> c_ulong;
}
extern "C" {
    fn type_engine_scan2_class_def(env: *mut Chuck_Env, def: crate::chuck_absyn_h_edited::a_Class_Def) -> c_ulong;
}
extern "C" {
    fn type_engine_scan2_func_def(env: *mut Chuck_Env, def: crate::chuck_absyn_h_edited::a_Func_Def) -> c_ulong;
}
extern "C" {
    fn type_engine_scan2_exp_decl(env: *mut Chuck_Env, decl: crate::chuck_absyn_h_edited::a_Exp_Decl) -> c_ulong;
}
#[repr(C)]
pub struct Chuck_Local {
    pub name: crate::rtmidi_h_edited::string,
    pub size: c_ulong,
    pub is_ref: c_ulong,
    pub is_obj: c_ulong,
    pub is_global: c_ulong,
    pub offset: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Local"]
    fn Chuck_Local_Chuck_Local(this: *mut Chuck_Local);
}
impl Chuck_Local {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Local_Chuck_Local(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_Frame {
    pub name: crate::rtmidi_h_edited::string,
    pub curr_offset: c_ulong,
    pub num_access: c_ulong,
    pub stack: crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}push_scope"]
    fn Chuck_Frame_push_scope(this: *mut Chuck_Frame);
}
extern "C" {
    #[link_name = "\u{1}alloc_local"]
    fn Chuck_Frame_alloc_local(
        this: *mut Chuck_Frame,
        size: c_ulong,
        name: *const crate::rtmidi_h_edited::string,
        is_ref: c_ulong,
        is_obj: c_ulong,
        is_global: c_ulong,
    ) -> *mut Chuck_Local;
}
extern "C" {
    #[link_name = "\u{1}get_scope"]
    fn Chuck_Frame_get_scope(this: *const Chuck_Frame, out: *mut crate::dts::vector);
}
extern "C" {
    #[link_name = "\u{1}pop_scope"]
    fn Chuck_Frame_pop_scope(this: *mut Chuck_Frame, out: *mut crate::dts::vector);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Frame"]
    fn Chuck_Frame_Chuck_Frame(this: *mut Chuck_Frame);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Frame_destructor"]
    fn Chuck_Frame_Chuck_Frame_destructor(this: *mut Chuck_Frame);
}
impl Chuck_Frame {
    #[inline]
    unsafe fn push_scope(&mut self) {
        Chuck_Frame_push_scope(self)
    }
    #[inline]
    unsafe fn alloc_local(
        &mut self,
        size: c_ulong,
        name: *const crate::rtmidi_h_edited::string,
        is_ref: c_ulong,
        is_obj: c_ulong,
        is_global: c_ulong,
    ) -> *mut Chuck_Local {
        Chuck_Frame_alloc_local(self, size, name, is_ref, is_obj, is_global)
    }
    #[inline]
    unsafe fn get_scope(&self, out: *mut crate::dts::vector) {
        Chuck_Frame_get_scope(self, out)
    }
    #[inline]
    unsafe fn pop_scope(&mut self, out: *mut crate::dts::vector) {
        Chuck_Frame_pop_scope(self, out)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Frame_Chuck_Frame(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        Chuck_Frame_Chuck_Frame_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Instr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Instr_Goto {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_Code {
    pub name: crate::rtmidi_h_edited::string,
    pub stack_depth: c_ulong,
    pub need_this: c_ulong,
    pub frame: *mut Chuck_Frame,
    pub code: crate::dts::vector,
    pub stack_cont: crate::dts::vector,
    pub stack_break: crate::dts::vector,
    pub stack_return: crate::dts::vector,
    pub filename: crate::rtmidi_h_edited::string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Code"]
    fn Chuck_Code_Chuck_Code(this: *mut Chuck_Code);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Code_destructor"]
    fn Chuck_Code_Chuck_Code_destructor(this: *mut Chuck_Code);
}
impl Chuck_Code {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Code_Chuck_Code(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        Chuck_Code_Chuck_Code_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_Emitter {
    pub _base: Chuck_VM_Object,
    pub env: *mut Chuck_Env,
    pub code: *mut Chuck_Code,
    pub context: *mut Chuck_Context,
    pub nspc: *mut Chuck_Namespace,
    pub func: *mut Chuck_Func,
    pub stack: crate::dts::vector,
    pub locals: crate::dts::vector,
    pub dump: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}append"]
    fn Chuck_Emitter_append(this: *mut Chuck_Emitter, instr: *mut Chuck_Instr);
}
extern "C" {
    #[link_name = "\u{1}next_index"]
    fn Chuck_Emitter_next_index(this: *mut Chuck_Emitter) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}push_scope"]
    fn Chuck_Emitter_push_scope(this: *mut Chuck_Emitter);
}
extern "C" {
    #[link_name = "\u{1}alloc_local"]
    fn Chuck_Emitter_alloc_local(
        this: *mut Chuck_Emitter,
        size: c_ulong,
        name: *const crate::rtmidi_h_edited::string,
        is_ref: c_ulong,
        is_obj: c_ulong,
        is_global: c_ulong,
    ) -> *mut Chuck_Local;
}
extern "C" {
    #[link_name = "\u{1}addref_on_scope"]
    fn Chuck_Emitter_addref_on_scope(this: *mut Chuck_Emitter);
}
extern "C" {
    #[link_name = "\u{1}pop_scope"]
    fn Chuck_Emitter_pop_scope(this: *mut Chuck_Emitter);
}
extern "C" {
    #[link_name = "\u{1}find_dur"]
    fn Chuck_Emitter_find_dur(
        this: *mut Chuck_Emitter,
        name: *const crate::rtmidi_h_edited::string,
        out: *mut f64,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Emitter"]
    fn Chuck_Emitter_Chuck_Emitter(this: *mut Chuck_Emitter);
}
impl Chuck_Emitter {
    #[inline]
    unsafe fn append(&mut self, instr: *mut Chuck_Instr) {
        Chuck_Emitter_append(self, instr)
    }
    #[inline]
    unsafe fn next_index(&mut self) -> c_ulong {
        Chuck_Emitter_next_index(self)
    }
    #[inline]
    unsafe fn push_scope(&mut self) {
        Chuck_Emitter_push_scope(self)
    }
    #[inline]
    unsafe fn alloc_local(
        &mut self,
        size: c_ulong,
        name: *const crate::rtmidi_h_edited::string,
        is_ref: c_ulong,
        is_obj: c_ulong,
        is_global: c_ulong,
    ) -> *mut Chuck_Local {
        Chuck_Emitter_alloc_local(self, size, name, is_ref, is_obj, is_global)
    }
    #[inline]
    unsafe fn addref_on_scope(&mut self) {
        Chuck_Emitter_addref_on_scope(self)
    }
    #[inline]
    unsafe fn pop_scope(&mut self) {
        Chuck_Emitter_pop_scope(self)
    }
    #[inline]
    unsafe fn find_dur(&mut self, name: *const crate::rtmidi_h_edited::string, out: *mut f64) -> c_ulong {
        Chuck_Emitter_find_dur(self, name, out)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Emitter_Chuck_Emitter(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Emitter_destructor"]
    fn Chuck_Emitter_Chuck_Emitter_destructor(this: *mut Chuck_Emitter);
}
extern "C" {
    fn emit_engine_init(env: *mut Chuck_Env) -> *mut Chuck_Emitter;
}
extern "C" {
    fn emit_engine_shutdown(emit: *mut Chuck_Emitter) -> c_ulong;
}
extern "C" {
    fn emit_engine_emit_prog(
        emit: *mut Chuck_Emitter,
        prog: crate::chuck_absyn_h_edited::a_Program,
        how_much: te_HowMuch,
    ) -> *mut Chuck_VM_Code;
}
extern "C" {
    fn emit_to_code(
        in_: *mut Chuck_Code,
        out: *mut Chuck_VM_Code,
        dump: c_ulong,
    ) -> *mut Chuck_VM_Code;
}
extern "C" {
    fn emit_engine_addr_map(emit: *mut Chuck_Emitter, shred: *mut Chuck_VM_Shred) -> c_ulong;
}
extern "C" {
    fn emit_engine_resolve() -> c_ulong;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_UGen {
    pub _base: Chuck_Object,
    pub tick: f_tick,
    pub tickf: f_tickf,
    pub pmsg: f_pmsg,
    pub m_multi_chan: *mut Chuck_UGen,
    pub m_multi_chan_size: c_ulong,
    pub m_num_ins: c_ulong,
    pub m_num_outs: c_ulong,
    pub m_src_list: *mut Chuck_UGen,
    pub m_src_cap: c_ulong,
    pub m_num_src: c_ulong,
    pub m_dest_list: *mut Chuck_UGen,
    pub m_dest_cap: c_ulong,
    pub m_num_dest: c_ulong,
    pub m_src_uana_list: *mut Chuck_UGen,
    pub m_src_uana_cap: c_ulong,
    pub m_num_uana_src: c_ulong,
    pub m_dest_uana_list: *mut Chuck_UGen,
    pub m_dest_uana_cap: c_ulong,
    pub m_num_uana_dest: c_ulong,
    pub m_max_src: c_ulong,
    pub m_time: f64,
    pub m_valid: c_ulong,
    pub m_use_next: c_ulong,
    pub m_sum: f32,
    pub m_current: f32,
    pub m_next: f32,
    pub m_last: f32,
    pub m_gain: f32,
    pub m_pan: f32,
    pub m_op: c_long,
    pub m_max_block_size: c_long,
    pub m_multi_in_v: *mut f32,
    pub m_multi_out_v: *mut f32,
    pub m_is_subgraph: c_ulong,
    pub m_inlet: *mut Chuck_UGen,
    pub m_outlet: *mut Chuck_UGen,
    pub m_sum_v: *mut f32,
    pub m_current_v: *mut f32,
    pub shred: *mut Chuck_VM_Shred,
    pub vm: *mut Chuck_VM,
    pub owner: *mut Chuck_UGen,
    pub m_is_uana: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}add"]
    fn Chuck_UGen_add(this: *mut Chuck_UGen, src: *mut Chuck_UGen, isUpChuck: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove"]
    fn Chuck_UGen_remove(this: *mut Chuck_UGen, src: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_all"]
    fn Chuck_UGen_remove_all(this: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}set_max_src"]
    fn Chuck_UGen_set_max_src(this: *mut Chuck_UGen, num: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_num_src"]
    fn Chuck_UGen_get_num_src(this: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}is_connected_from"]
    fn Chuck_UGen_is_connected_from(this: *mut Chuck_UGen, src: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}disconnect"]
    fn Chuck_UGen_disconnect(this: *mut Chuck_UGen, recursive: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}system_tick"]
    fn Chuck_UGen_system_tick(this: *mut Chuck_UGen, now: f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}system_tick_v"]
    fn Chuck_UGen_system_tick_v(this: *mut Chuck_UGen, now: f64, numFrames: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}alloc_v"]
    fn Chuck_UGen_alloc_v(this: *mut Chuck_UGen, size: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}src_chan"]
    fn Chuck_UGen_src_chan(this: *mut Chuck_UGen, chan: c_ulong) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}dst_for_src_chan"]
    fn Chuck_UGen_dst_for_src_chan(this: *mut Chuck_UGen, chan: c_ulong) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}add_by"]
    fn Chuck_UGen_add_by(this: *mut Chuck_UGen, dest: *mut Chuck_UGen, isUpChuck: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}remove_by"]
    fn Chuck_UGen_remove_by(this: *mut Chuck_UGen, dest: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}alloc_multi_chan"]
    fn Chuck_UGen_alloc_multi_chan(this: *mut Chuck_UGen, num_ins: c_ulong, num_outs: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}init_subgraph"]
    fn Chuck_UGen_init_subgraph(this: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}inlet"]
    fn Chuck_UGen_inlet(this: *mut Chuck_UGen) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}outlet"]
    fn Chuck_UGen_outlet(this: *mut Chuck_UGen) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen"]
    fn Chuck_UGen_Chuck_UGen(this: *mut Chuck_UGen);
}
impl Chuck_UGen {
    #[inline]
    unsafe fn add(&mut self, src: *mut Chuck_UGen, isUpChuck: c_ulong) -> c_ulong {
        Chuck_UGen_add(self, src, isUpChuck)
    }
    #[inline]
    unsafe fn remove(&mut self, src: *mut Chuck_UGen) -> c_ulong {
        Chuck_UGen_remove(self, src)
    }
    #[inline]
    unsafe fn remove_all(&mut self) {
        Chuck_UGen_remove_all(self)
    }
    #[inline]
    unsafe fn set_max_src(&mut self, num: c_ulong) -> c_ulong {
        Chuck_UGen_set_max_src(self, num)
    }
    #[inline]
    unsafe fn get_num_src(&mut self) -> c_ulong {
        Chuck_UGen_get_num_src(self)
    }
    #[inline]
    unsafe fn is_connected_from(&mut self, src: *mut Chuck_UGen) -> c_ulong {
        Chuck_UGen_is_connected_from(self, src)
    }
    #[inline]
    unsafe fn disconnect(&mut self, recursive: c_ulong) -> c_ulong {
        Chuck_UGen_disconnect(self, recursive)
    }
    #[inline]
    unsafe fn system_tick(&mut self, now: f64) -> c_ulong {
        Chuck_UGen_system_tick(self, now)
    }
    #[inline]
    unsafe fn system_tick_v(&mut self, now: f64, numFrames: c_ulong) -> c_ulong {
        Chuck_UGen_system_tick_v(self, now, numFrames)
    }
    #[inline]
    unsafe fn alloc_v(&mut self, size: c_ulong) -> c_ulong {
        Chuck_UGen_alloc_v(self, size)
    }
    #[inline]
    unsafe fn src_chan(&mut self, chan: c_ulong) -> *mut Chuck_UGen {
        Chuck_UGen_src_chan(self, chan)
    }
    #[inline]
    unsafe fn dst_for_src_chan(&mut self, chan: c_ulong) -> *mut Chuck_UGen {
        Chuck_UGen_dst_for_src_chan(self, chan)
    }
    #[inline]
    unsafe fn add_by(&mut self, dest: *mut Chuck_UGen, isUpChuck: c_ulong) {
        Chuck_UGen_add_by(self, dest, isUpChuck)
    }
    #[inline]
    unsafe fn remove_by(&mut self, dest: *mut Chuck_UGen) {
        Chuck_UGen_remove_by(self, dest)
    }
    #[inline]
    unsafe fn alloc_multi_chan(&mut self, num_ins: c_ulong, num_outs: c_ulong) {
        Chuck_UGen_alloc_multi_chan(self, num_ins, num_outs)
    }
    #[inline]
    unsafe fn init_subgraph(&mut self) {
        Chuck_UGen_init_subgraph(self)
    }
    #[inline]
    unsafe fn inlet(&mut self) -> *mut Chuck_UGen {
        Chuck_UGen_inlet(self)
    }
    #[inline]
    unsafe fn outlet(&mut self) -> *mut Chuck_UGen {
        Chuck_UGen_outlet(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_UGen_Chuck_UGen(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen_destructor"]
    fn Chuck_UGen_Chuck_UGen_destructor(this: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}init"]
    fn Chuck_UGen_init(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}done"]
    fn Chuck_UGen_done(this: *mut c_void);
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_UAna {
    pub _base: Chuck_UGen,
    pub tock: f_tock,
    pub m_uana_time: f64,
}
extern "C" {
    #[link_name = "\u{1}system_tock"]
    fn Chuck_UAna_system_tock(this: *mut Chuck_UAna, now: f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}is_up_connected_from"]
    fn Chuck_UAna_is_up_connected_from(this: *mut Chuck_UAna, src: *mut Chuck_UAna) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}numIncomingUAnae"]
    fn Chuck_UAna_numIncomingUAnae(this: *const Chuck_UAna) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}getIncomingUAna"]
    fn Chuck_UAna_getIncomingUAna(this: *const Chuck_UAna, index: c_ulong) -> *mut Chuck_UAna;
}
extern "C" {
    #[link_name = "\u{1}getIncomingBlob"]
    fn Chuck_UAna_getIncomingBlob(
        this: *const Chuck_UAna,
        index: c_ulong,
    ) -> *mut Chuck_UAnaBlobProxy;
}
extern "C" {
    #[link_name = "\u{1}blobProxy"]
    fn Chuck_UAna_blobProxy(this: *const Chuck_UAna) -> *mut Chuck_UAnaBlobProxy;
}
extern "C" {
    #[link_name = "\u{1}Chuck_UAna"]
    fn Chuck_UAna_Chuck_UAna(this: *mut Chuck_UAna);
}
impl Chuck_UAna {
    #[inline]
    unsafe fn system_tock(&mut self, now: f64) -> c_ulong {
        Chuck_UAna_system_tock(self, now)
    }
    #[inline]
    unsafe fn is_up_connected_from(&mut self, src: *mut Chuck_UAna) -> c_ulong {
        Chuck_UAna_is_up_connected_from(self, src)
    }
    #[inline]
    unsafe fn numIncomingUAnae(&self) -> c_long {
        Chuck_UAna_numIncomingUAnae(self)
    }
    #[inline]
    unsafe fn getIncomingUAna(&self, index: c_ulong) -> *mut Chuck_UAna {
        Chuck_UAna_getIncomingUAna(self, index)
    }
    #[inline]
    unsafe fn getIncomingBlob(&self, index: c_ulong) -> *mut Chuck_UAnaBlobProxy {
        Chuck_UAna_getIncomingBlob(self, index)
    }
    #[inline]
    unsafe fn blobProxy(&self) -> *mut Chuck_UAnaBlobProxy {
        Chuck_UAna_blobProxy(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_UAna_Chuck_UAna(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_UAna_destructor"]
    fn Chuck_UAna_Chuck_UAna_destructor(this: *mut Chuck_UAna);
}
extern "C" {
    fn ugen_generic_num_in(obj: *mut Chuck_Object, isArray: c_ulong) -> c_long;
}
extern "C" {
    fn ugen_generic_get_src(
        obj: *mut Chuck_Object,
        chan: c_long,
        isArray: c_ulong,
    ) -> *mut Chuck_UGen;
}
extern "C" {
    fn ugen_generic_get_dst(
        obj: *mut Chuck_Object,
        chan: c_long,
        isArray: c_ulong,
    ) -> *mut Chuck_UGen;
}
#[repr(C)]
pub struct CBufferAdvance {
    pub m_data: *mut c_uchar,
    pub m_data_width: c_ulong,
    pub m_read_offsets: crate::dts::vector,
    pub m_free: crate::dts::deque,
    pub m_write_offset: c_long,
    pub m_max_elem: c_long,
    pub m_mutex: XMutex,
    pub m_event_buffer: *mut CBufferSimple,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct CBufferAdvance_ReadOffset {
    read_offset: c_long,
    event: *mut Chuck_Event,
}
extern "C" {
    #[link_name = "\u{1}ReadOffset"]
    fn CBufferAdvance_ReadOffset_ReadOffset(
        this: *mut CBufferAdvance_ReadOffset,
        ro: c_long,
        e: *mut Chuck_Event,
    );
}
impl CBufferAdvance_ReadOffset {
    #[inline]
    unsafe fn new(ro: c_long, e: *mut Chuck_Event) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        CBufferAdvance_ReadOffset_ReadOffset(&mut __bindgen_tmp, ro, e);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    fn CBufferAdvance_initialize(
        this: *mut CBufferAdvance,
        num_elem: c_ulong,
        width: c_ulong,
        event_buffer: *mut CBufferSimple,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    fn CBufferAdvance_cleanup(this: *mut CBufferAdvance);
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn CBufferAdvance_get(
        this: *mut CBufferAdvance,
        data: *mut c_void,
        num_elem: c_ulong,
        read_offset_index: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}put"]
    fn CBufferAdvance_put(this: *mut CBufferAdvance, data: *mut c_void, num_elem: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}empty"]
    fn CBufferAdvance_empty(this: *mut CBufferAdvance, read_offset_index: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}join"]
    fn CBufferAdvance_join(this: *mut CBufferAdvance, event: *mut Chuck_Event) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}resign"]
    fn CBufferAdvance_resign(this: *mut CBufferAdvance, read_offset_index: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}CBufferAdvance"]
    fn CBufferAdvance_CBufferAdvance(this: *mut CBufferAdvance);
}
extern "C" {
    #[link_name = "\u{1}CBufferAdvance_destructor"]
    fn CBufferAdvance_CBufferAdvance_destructor(this: *mut CBufferAdvance);
}
impl CBufferAdvance {
    #[inline]
    unsafe fn initialize(
        &mut self,
        num_elem: c_ulong,
        width: c_ulong,
        event_buffer: *mut CBufferSimple,
    ) -> c_ulong {
        CBufferAdvance_initialize(self, num_elem, width, event_buffer)
    }
    #[inline]
    unsafe fn cleanup(&mut self) {
        CBufferAdvance_cleanup(self)
    }
    #[inline]
    unsafe fn get(
        &mut self,
        data: *mut c_void,
        num_elem: c_ulong,
        read_offset_index: c_ulong,
    ) -> c_ulong {
        CBufferAdvance_get(self, data, num_elem, read_offset_index)
    }
    #[inline]
    unsafe fn put(&mut self, data: *mut c_void, num_elem: c_ulong) {
        CBufferAdvance_put(self, data, num_elem)
    }
    #[inline]
    unsafe fn empty(&mut self, read_offset_index: c_ulong) -> c_ulong {
        CBufferAdvance_empty(self, read_offset_index)
    }
    #[inline]
    unsafe fn join(&mut self, event: *mut Chuck_Event) -> c_ulong {
        CBufferAdvance_join(self, event)
    }
    #[inline]
    unsafe fn resign(&mut self, read_offset_index: c_ulong) {
        CBufferAdvance_resign(self, read_offset_index)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        CBufferAdvance_CBufferAdvance(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        CBufferAdvance_CBufferAdvance_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct CBufferSimple {
    pub m_data: *mut c_uchar,
    pub m_data_width: c_ulong,
    pub m_read_offset: c_ulong,
    pub m_write_offset: c_ulong,
    pub m_max_elem: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    fn CBufferSimple_initialize(
        this: *mut CBufferSimple,
        num_elem: c_ulong,
        width: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    fn CBufferSimple_cleanup(this: *mut CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn CBufferSimple_get(this: *mut CBufferSimple, data: *mut c_void, num_elem: c_ulong)
        -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}put"]
    fn CBufferSimple_put(this: *mut CBufferSimple, data: *mut c_void, num_elem: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}CBufferSimple"]
    fn CBufferSimple_CBufferSimple(this: *mut CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}CBufferSimple_destructor"]
    fn CBufferSimple_CBufferSimple_destructor(this: *mut CBufferSimple);
}
impl CBufferSimple {
    #[inline]
    unsafe fn initialize(&mut self, num_elem: c_ulong, width: c_ulong) -> c_ulong {
        CBufferSimple_initialize(self, num_elem, width)
    }
    #[inline]
    unsafe fn cleanup(&mut self) {
        CBufferSimple_cleanup(self)
    }
    #[inline]
    unsafe fn get(&mut self, data: *mut c_void, num_elem: c_ulong) -> c_ulong {
        CBufferSimple_get(self, data, num_elem)
    }
    #[inline]
    unsafe fn put(&mut self, data: *mut c_void, num_elem: c_ulong) {
        CBufferSimple_put(self, data, num_elem)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        CBufferSimple_CBufferSimple(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        CBufferSimple_CBufferSimple_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct AccumBuffer {
    pub m_data: *mut f32,
    pub m_write_offset: c_ulong,
    pub m_max_elem: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}resize"]
    fn AccumBuffer_resize(this: *mut AccumBuffer, new_size: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    fn AccumBuffer_cleanup(this: *mut AccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}put"]
    fn AccumBuffer_put(this: *mut AccumBuffer, next: f32);
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn AccumBuffer_get(this: *mut AccumBuffer, buffer: *mut f32, num_elem: c_long);
}
extern "C" {
    #[link_name = "\u{1}AccumBuffer"]
    fn AccumBuffer_AccumBuffer(this: *mut AccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}AccumBuffer_destructor"]
    fn AccumBuffer_AccumBuffer_destructor(this: *mut AccumBuffer);
}
impl AccumBuffer {
    #[inline]
    unsafe fn resize(&mut self, new_size: c_long) -> c_long {
        AccumBuffer_resize(self, new_size)
    }
    #[inline]
    unsafe fn cleanup(&mut self) {
        AccumBuffer_cleanup(self)
    }
    #[inline]
    unsafe fn put(&mut self, next: f32) {
        AccumBuffer_put(self, next)
    }
    #[inline]
    unsafe fn get(&mut self, buffer: *mut f32, num_elem: c_long) {
        AccumBuffer_get(self, buffer, num_elem)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        AccumBuffer_AccumBuffer(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        AccumBuffer_AccumBuffer_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_Func {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_FTable {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_IO_Serial {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_VM_Stack {
    stack: *mut c_uchar,
    sp: *mut c_uchar,
    sp_max: *mut c_uchar,
    prev: *mut Chuck_VM_Stack,
    next: *mut Chuck_VM_Stack,
    m_is_init: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    fn Chuck_VM_Stack_initialize(this: *mut Chuck_VM_Stack, size: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    fn Chuck_VM_Stack_shutdown(this: *mut Chuck_VM_Stack) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Stack"]
    fn Chuck_VM_Stack_Chuck_VM_Stack(this: *mut Chuck_VM_Stack);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Stack_destructor"]
    fn Chuck_VM_Stack_Chuck_VM_Stack_destructor(this: *mut Chuck_VM_Stack);
}
impl Chuck_VM_Stack {
    #[inline]
    unsafe fn initialize(&mut self, size: c_ulong) -> c_ulong {
        Chuck_VM_Stack_initialize(self, size)
    }
    #[inline]
    unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_Stack_shutdown(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Stack_Chuck_VM_Stack(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        Chuck_VM_Stack_Chuck_VM_Stack_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_VM_Code {
    pub _base: Chuck_Object,
    pub instr: *mut Chuck_Instr,
    pub num_instr: c_ulong,
    pub name: crate::rtmidi_h_edited::string,
    pub stack_depth: c_ulong,
    pub need_this: c_ulong,
    pub native_func: c_ulong,
    pub native_func_type: c_ulong,
    pub filename: crate::rtmidi_h_edited::string,
}
const Chuck_VM_Code_NATIVE_UNKNOWN: Chuck_VM_Code__bindgen_ty_1 = 0;
const Chuck_VM_Code_NATIVE_CTOR: Chuck_VM_Code__bindgen_ty_1 = 1;
const Chuck_VM_Code_NATIVE_DTOR: Chuck_VM_Code__bindgen_ty_1 = 2;
const Chuck_VM_Code_NATIVE_MFUN: Chuck_VM_Code__bindgen_ty_1 = 3;
const Chuck_VM_Code_NATIVE_SFUN: Chuck_VM_Code__bindgen_ty_1 = 4;
type Chuck_VM_Code__bindgen_ty_1 = u32;
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Code"]
    fn Chuck_VM_Code_Chuck_VM_Code(this: *mut Chuck_VM_Code);
}
impl Chuck_VM_Code {
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Code_Chuck_VM_Code(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Code_destructor"]
    fn Chuck_VM_Code_Chuck_VM_Code_destructor(this: *mut Chuck_VM_Code);
}
#[repr(C)]
pub struct Chuck_VM_Shred {
    pub _base: Chuck_Object,
    pub mem: *mut Chuck_VM_Stack,
    pub reg: *mut Chuck_VM_Stack,
    pub base_ref: *mut Chuck_VM_Stack,
    pub code: *mut Chuck_VM_Code,
    pub code_orig: *mut Chuck_VM_Code,
    pub instr: *mut Chuck_Instr,
    pub parent: *mut Chuck_VM_Shred,
    pub children: std::collections::HashMap::new(),
    pub pc: c_ulong,
    pub vm_ref: *mut Chuck_VM,
    pub now: f64,
    pub start: f64,
    pub wake_time: f64,
    pub next_pc: c_ulong,
    pub is_done: c_ulong,
    pub is_running: c_ulong,
    pub is_abort: c_ulong,
    pub is_dumped: c_ulong,
    pub event: *mut Chuck_Event,
    pub m_ugen_map: std::collections::HashMap::new(),
    pub m_parent_objects: crate::dts::vector,
    pub xid: c_ulong,
    pub name: crate::rtmidi_h_edited::string,
    pub args: crate::dts::vector,
    pub prev: *mut Chuck_VM_Shred,
    pub next: *mut Chuck_VM_Shred,
    pub m_loopCounters: crate::dts::vector,
    pub m_serials: *mut crate::dts::list,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    fn Chuck_VM_Shred_initialize(
        this: *mut Chuck_VM_Shred,
        c: *mut Chuck_VM_Code,
        mem_st_size: c_ulong,
        reg_st_size: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    fn Chuck_VM_Shred_shutdown(this: *mut Chuck_VM_Shred) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    fn Chuck_VM_Shred_run(this: *mut Chuck_VM_Shred, vm: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add"]
    fn Chuck_VM_Shred_add(this: *mut Chuck_VM_Shred, ugen: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove"]
    fn Chuck_VM_Shred_remove(this: *mut Chuck_VM_Shred, ugen: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add_parent_ref"]
    fn Chuck_VM_Shred_add_parent_ref(this: *mut Chuck_VM_Shred, obj: *mut Chuck_Object);
}
extern "C" {
    #[link_name = "\u{1}add_serialio"]
    fn Chuck_VM_Shred_add_serialio(this: *mut Chuck_VM_Shred, serial: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}remove_serialio"]
    fn Chuck_VM_Shred_remove_serialio(this: *mut Chuck_VM_Shred, serial: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}pushLoopCounter"]
    fn Chuck_VM_Shred_pushLoopCounter(this: *mut Chuck_VM_Shred) -> *mut c_ulong;
}
extern "C" {
    #[link_name = "\u{1}currentLoopCounter"]
    fn Chuck_VM_Shred_currentLoopCounter(this: *mut Chuck_VM_Shred) -> *mut c_ulong;
}
extern "C" {
    #[link_name = "\u{1}popLoopCounter"]
    fn Chuck_VM_Shred_popLoopCounter(this: *mut Chuck_VM_Shred) -> bool;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred"]
    fn Chuck_VM_Shred_Chuck_VM_Shred(this: *mut Chuck_VM_Shred);
}
impl Chuck_VM_Shred {
    #[inline]
    unsafe fn initialize(
        &mut self,
        c: *mut Chuck_VM_Code,
        mem_st_size: c_ulong,
        reg_st_size: c_ulong,
    ) -> c_ulong {
        Chuck_VM_Shred_initialize(self, c, mem_st_size, reg_st_size)
    }
    #[inline]
    unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_Shred_shutdown(self)
    }
    #[inline]
    unsafe fn run(&mut self, vm: *mut Chuck_VM) -> c_ulong {
        Chuck_VM_Shred_run(self, vm)
    }
    #[inline]
    unsafe fn add(&mut self, ugen: *mut Chuck_UGen) -> c_ulong {
        Chuck_VM_Shred_add(self, ugen)
    }
    #[inline]
    unsafe fn remove(&mut self, ugen: *mut Chuck_UGen) -> c_ulong {
        Chuck_VM_Shred_remove(self, ugen)
    }
    #[inline]
    unsafe fn add_parent_ref(&mut self, obj: *mut Chuck_Object) {
        Chuck_VM_Shred_add_parent_ref(self, obj)
    }
    #[inline]
    unsafe fn add_serialio(&mut self, serial: *mut Chuck_IO_Serial) {
        Chuck_VM_Shred_add_serialio(self, serial)
    }
    #[inline]
    unsafe fn remove_serialio(&mut self, serial: *mut Chuck_IO_Serial) {
        Chuck_VM_Shred_remove_serialio(self, serial)
    }
    #[inline]
    unsafe fn pushLoopCounter(&mut self) -> *mut c_ulong {
        Chuck_VM_Shred_pushLoopCounter(self)
    }
    #[inline]
    unsafe fn currentLoopCounter(&mut self) -> *mut c_ulong {
        Chuck_VM_Shred_currentLoopCounter(self)
    }
    #[inline]
    unsafe fn popLoopCounter(&mut self) -> bool {
        Chuck_VM_Shred_popLoopCounter(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Shred_Chuck_VM_Shred(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred_destructor"]
    fn Chuck_VM_Shred_Chuck_VM_Shred_destructor(this: *mut Chuck_VM_Shred);
}
#[repr(C)]
pub struct Chuck_VM_Shred_Status {
    pub _base: Chuck_Object,
    pub xid: c_ulong,
    pub name: crate::rtmidi_h_edited::string,
    pub start: f64,
    pub has_event: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred_Status"]
    fn Chuck_VM_Shred_Status_Chuck_VM_Shred_Status(
        this: *mut Chuck_VM_Shred_Status,
        _id: c_ulong,
        n: *const crate::rtmidi_h_edited::string,
        _start: f64,
        e: c_ulong,
    );
}
impl Chuck_VM_Shred_Status {
    #[inline]
    unsafe fn new(_id: c_ulong, n: *const crate::rtmidi_h_edited::string, _start: f64, e: c_ulong) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Shred_Status_Chuck_VM_Shred_Status(&mut __bindgen_tmp, _id, n, _start, e);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_VM_Status {
    pub _base: Chuck_Object,
    pub srate: c_ulong,
    pub now_system: f64,
    pub t_second: c_ulong,
    pub t_minute: c_ulong,
    pub t_hour: c_ulong,
    pub list: crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}clear"]
    fn Chuck_VM_Status_clear(this: *mut Chuck_VM_Status);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Status"]
    fn Chuck_VM_Status_Chuck_VM_Status(this: *mut Chuck_VM_Status);
}
impl Chuck_VM_Status {
    #[inline]
    unsafe fn clear(&mut self) {
        Chuck_VM_Status_clear(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Status_Chuck_VM_Status(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Status_destructor"]
    fn Chuck_VM_Status_Chuck_VM_Status_destructor(this: *mut Chuck_VM_Status);
}
#[repr(C)]
pub struct Chuck_VM_Shreduler {
    pub _base: Chuck_Object,
    pub now_system: f64,
    pub rt_audio: c_ulong,
    pub vm_ref: *mut Chuck_VM,
    pub shred_list: *mut Chuck_VM_Shred,
    pub blocked: std::collections::HashMap::new(),
    pub m_current_shred: *mut Chuck_VM_Shred,
    pub m_dac: *mut Chuck_UGen,
    pub m_adc: *mut Chuck_UGen,
    pub m_bunghole: *mut Chuck_UGen,
    pub m_num_dac_channels: c_ulong,
    pub m_num_adc_channels: c_ulong,
    pub m_status: Chuck_VM_Status,
    pub m_max_block_size: c_ulong,
    pub m_adaptive: c_ulong,
    pub m_samps_until_next: f64,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    fn Chuck_VM_Shreduler_initialize(this: *mut Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    fn Chuck_VM_Shreduler_shutdown(this: *mut Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shredule"]
    fn Chuck_VM_Shreduler_shredule(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shredule"]
    fn Chuck_VM_Shreduler_shredule1(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
        wake_time: f64,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    fn Chuck_VM_Shreduler_get(this: *mut Chuck_VM_Shreduler) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}advance"]
    fn Chuck_VM_Shreduler_advance(this: *mut Chuck_VM_Shreduler, N: c_long);
}
extern "C" {
    #[link_name = "\u{1}advance_v"]
    fn Chuck_VM_Shreduler_advance_v(
        this: *mut Chuck_VM_Shreduler,
        num_left: *mut c_long,
        offset: *mut c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}set_adaptive"]
    fn Chuck_VM_Shreduler_set_adaptive(this: *mut Chuck_VM_Shreduler, max_block_size: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}remove"]
    fn Chuck_VM_Shreduler_remove(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}replace"]
    fn Chuck_VM_Shreduler_replace(
        this: *mut Chuck_VM_Shreduler,
        out: *mut Chuck_VM_Shred,
        in_: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}lookup"]
    fn Chuck_VM_Shreduler_lookup(
        this: *mut Chuck_VM_Shreduler,
        xid: c_ulong,
    ) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}status"]
    fn Chuck_VM_Shreduler_status(this: *mut Chuck_VM_Shreduler);
}
extern "C" {
    #[link_name = "\u{1}status"]
    fn Chuck_VM_Shreduler_status1(this: *mut Chuck_VM_Shreduler, status: *mut Chuck_VM_Status);
}
extern "C" {
    #[link_name = "\u{1}highest"]
    fn Chuck_VM_Shreduler_highest(this: *mut Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add_blocked"]
    fn Chuck_VM_Shreduler_add_blocked(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_blocked"]
    fn Chuck_VM_Shreduler_remove_blocked(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shreduler"]
    fn Chuck_VM_Shreduler_Chuck_VM_Shreduler(this: *mut Chuck_VM_Shreduler);
}
impl Chuck_VM_Shreduler {
    #[inline]
    unsafe fn initialize(&mut self) -> c_ulong {
        Chuck_VM_Shreduler_initialize(self)
    }
    #[inline]
    unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_Shreduler_shutdown(self)
    }
    #[inline]
    unsafe fn shredule(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_shredule(self, shred)
    }
    #[inline]
    unsafe fn shredule1(&mut self, shred: *mut Chuck_VM_Shred, wake_time: f64) -> c_ulong {
        Chuck_VM_Shreduler_shredule1(self, shred, wake_time)
    }
    #[inline]
    unsafe fn get(&mut self) -> *mut Chuck_VM_Shred {
        Chuck_VM_Shreduler_get(self)
    }
    #[inline]
    unsafe fn advance(&mut self, N: c_long) {
        Chuck_VM_Shreduler_advance(self, N)
    }
    #[inline]
    unsafe fn advance_v(&mut self, num_left: *mut c_long, offset: *mut c_long) {
        Chuck_VM_Shreduler_advance_v(self, num_left, offset)
    }
    #[inline]
    unsafe fn set_adaptive(&mut self, max_block_size: c_ulong) {
        Chuck_VM_Shreduler_set_adaptive(self, max_block_size)
    }
    #[inline]
    unsafe fn remove(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_remove(self, shred)
    }
    #[inline]
    unsafe fn replace(&mut self, out: *mut Chuck_VM_Shred, in_: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_replace(self, out, in_)
    }
    #[inline]
    unsafe fn lookup(&mut self, xid: c_ulong) -> *mut Chuck_VM_Shred {
        Chuck_VM_Shreduler_lookup(self, xid)
    }
    #[inline]
    unsafe fn status(&mut self) {
        Chuck_VM_Shreduler_status(self)
    }
    #[inline]
    unsafe fn status1(&mut self, status: *mut Chuck_VM_Status) {
        Chuck_VM_Shreduler_status1(self, status)
    }
    #[inline]
    unsafe fn highest(&mut self) -> c_ulong {
        Chuck_VM_Shreduler_highest(self)
    }
    #[inline]
    unsafe fn add_blocked(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_add_blocked(self, shred)
    }
    #[inline]
    unsafe fn remove_blocked(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_remove_blocked(self, shred)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Shreduler_Chuck_VM_Shreduler(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shreduler_destructor"]
    fn Chuck_VM_Shreduler_Chuck_VM_Shreduler_destructor(this: *mut Chuck_VM_Shreduler);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Set_Global_Int_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Get_Global_Int_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Set_Global_Float_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Get_Global_Float_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Signal_Global_Event_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Global_Int_Container {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Global_Float_Container {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chuck_Global_Event_Container {
    _unused: [u8; 0],
}
const Chuck_Global_Request_Type_set_global_int_request: Chuck_Global_Request_Type = 0;
const Chuck_Global_Request_Type_get_global_int_request: Chuck_Global_Request_Type = 1;
const Chuck_Global_Request_Type_set_global_float_request: Chuck_Global_Request_Type = 2;
const Chuck_Global_Request_Type_get_global_float_request: Chuck_Global_Request_Type = 3;
const Chuck_Global_Request_Type_signal_global_event_request: Chuck_Global_Request_Type = 4;
const Chuck_Global_Request_Type_spork_shred_request: Chuck_Global_Request_Type = 5;
type Chuck_Global_Request_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
struct Chuck_Global_Request {
    type_: Chuck_Global_Request_Type,
    __bindgen_anon_1: Chuck_Global_Request__bindgen_ty_1,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
union Chuck_Global_Request__bindgen_ty_1 {
    setIntRequest: *mut Chuck_Set_Global_Int_Request,
    getIntRequest: *mut Chuck_Get_Global_Int_Request,
    setFloatRequest: *mut Chuck_Set_Global_Float_Request,
    getFloatRequest: *mut Chuck_Get_Global_Float_Request,
    signalEventRequest: *mut Chuck_Signal_Global_Event_Request,
    shred: *mut Chuck_VM_Shred,
    _bindgen_union_align: u64,
}
#[repr(C)]
pub struct Chuck_VM {
    pub _base: Chuck_Object,
    pub m_carrier: *mut Chuck_Carrier,
    pub m_adc: *mut Chuck_UGen,
    pub m_dac: *mut Chuck_UGen,
    pub m_bunghole: *mut Chuck_UGen,
    pub m_srate: c_ulong,
    pub m_num_adc_channels: c_ulong,
    pub m_num_dac_channels: c_ulong,
    pub m_halt: c_ulong,
    pub m_is_running: c_ulong,
    pub m_input_ref: *const f32,
    pub m_output_ref: *mut f32,
    pub m_init: c_ulong,
    pub m_last_error: crate::rtmidi_h_edited::string,
    pub m_shreds: *mut Chuck_VM_Shred,
    pub m_num_shreds: c_ulong,
    pub m_shred_id: c_ulong,
    pub m_shreduler: *mut Chuck_VM_Shreduler,
    pub m_shred_dump: crate::dts::vector,
    pub m_num_dumped_shreds: c_ulong,
    pub m_msg_buffer: *mut CBufferSimple,
    pub m_reply_buffer: *mut CBufferSimple,
    pub m_event_buffer: *mut CBufferSimple,
    pub m_event_buffers: crate::dts::list,
    pub m_global_ints: std::collections::HashMap::new(),
    pub m_global_floats: std::collections::HashMap::new(),
    pub m_global_events: std::collections::HashMap::new(),
    pub m_global_request_queue: XCircleBuffer<Chuck_Global_Request>,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_initialize(
        this: *mut Chuck_VM,
        srate: c_ulong,
        dac_chan: c_ulong,
        adc_chan: c_ulong,
        adaptive: c_ulong,
        halt: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}initialize_synthesis"]
    fn Chuck_VM_initialize_synthesis(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setCarrier"]
    fn Chuck_VM_setCarrier(this: *mut Chuck_VM, c: *mut Chuck_Carrier) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    fn Chuck_VM_shutdown(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}has_init"]
    fn Chuck_VM_has_init(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}start"]
    fn Chuck_VM_start(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}running"]
    fn Chuck_VM_running(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}stop"]
    fn Chuck_VM_stop(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}runningState"]
    fn Chuck_VM_runningState(this: *mut Chuck_VM) -> *mut c_ulong;
}
extern "C" {
    #[link_name = "\u{1}spork"]
    fn Chuck_VM_spork(
        this: *mut Chuck_VM,
        code: *mut Chuck_VM_Code,
        parent: *mut Chuck_VM_Shred,
        immediate: c_ulong,
    ) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}shreduler"]
    fn Chuck_VM_shreduler(this: *const Chuck_VM) -> *mut Chuck_VM_Shreduler;
}
extern "C" {
    #[link_name = "\u{1}next_id"]
    fn Chuck_VM_next_id(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}srate"]
    fn Chuck_VM_srate(this: *const Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    fn Chuck_VM_run(
        this: *mut Chuck_VM,
        numFrames: c_long,
        input: *const f32,
        output: *mut f32,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}compute"]
    fn Chuck_VM_compute(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}abort_current_shred"]
    fn Chuck_VM_abort_current_shred(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}invoke_static"]
    fn Chuck_VM_invoke_static(this: *mut Chuck_VM, shred: *mut Chuck_VM_Shred) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}gc"]
    fn Chuck_VM_gc(this: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}gc"]
    fn Chuck_VM_gc1(this: *mut Chuck_VM, amount: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}queue_msg"]
    fn Chuck_VM_queue_msg(this: *mut Chuck_VM, msg: *mut Chuck_Msg, num_msg: c_int) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_event"]
    fn Chuck_VM_queue_event(
        this: *mut Chuck_VM,
        event: *mut Chuck_Event,
        num_msg: c_int,
        buffer: *mut CBufferSimple,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}process_msg"]
    fn Chuck_VM_process_msg(this: *mut Chuck_VM, msg: *mut Chuck_Msg) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_reply"]
    fn Chuck_VM_get_reply(this: *mut Chuck_VM) -> *mut Chuck_Msg;
}
extern "C" {
    #[link_name = "\u{1}create_event_buffer"]
    fn Chuck_VM_create_event_buffer(this: *mut Chuck_VM) -> *mut CBufferSimple;
}
extern "C" {
    #[link_name = "\u{1}destroy_event_buffer"]
    fn Chuck_VM_destroy_event_buffer(this: *mut Chuck_VM, buffer: *mut CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}last_error"]
    fn Chuck_VM_last_error(this: *const Chuck_VM) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}get_global_int"]
    fn Chuck_VM_get_global_int(
        this: *mut Chuck_VM,
        name: crate::rtmidi_h_edited::string,
        callback: Option<unsafe extern "C" fn(arg1: c_long)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_global_int"]
    fn Chuck_VM_set_global_int(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string, val: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_float"]
    fn Chuck_VM_get_global_float(
        this: *mut Chuck_VM,
        name: crate::rtmidi_h_edited::string,
        callback: Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_global_float"]
    fn Chuck_VM_set_global_float(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string, val: f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signal_global_event"]
    fn Chuck_VM_signal_global_event(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}broadcast_global_event"]
    fn Chuck_VM_broadcast_global_event(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}init_global_int"]
    fn Chuck_VM_init_global_int(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_int_value"]
    fn Chuck_VM_get_global_int_value(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_int"]
    fn Chuck_VM_get_ptr_to_global_int(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> *mut c_long;
}
extern "C" {
    #[link_name = "\u{1}init_global_float"]
    fn Chuck_VM_init_global_float(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_float_value"]
    fn Chuck_VM_get_global_float_value(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> f64;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_float"]
    fn Chuck_VM_get_ptr_to_global_float(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> *mut f64;
}
extern "C" {
    #[link_name = "\u{1}init_global_event"]
    fn Chuck_VM_init_global_event(
        this: *mut Chuck_VM,
        name: crate::rtmidi_h_edited::string,
        type_: *mut Chuck_Type,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_event"]
    fn Chuck_VM_get_global_event(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string) -> *mut Chuck_Event;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_event"]
    fn Chuck_VM_get_ptr_to_global_event(this: *mut Chuck_VM, name: crate::rtmidi_h_edited::string)
        -> *mut Chuck_Event;
}
extern "C" {
    #[link_name = "\u{1}handle_global_queue_messages"]
    fn Chuck_VM_handle_global_queue_messages(this: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}carrier"]
    fn Chuck_VM_carrier(this: *const Chuck_VM) -> *mut Chuck_Carrier;
}
extern "C" {
    #[link_name = "\u{1}env"]
    fn Chuck_VM_env(this: *const Chuck_VM) -> *mut Chuck_Env;
}
extern "C" {
    #[link_name = "\u{1}chout"]
    fn Chuck_VM_chout(this: *const Chuck_VM) -> *mut Chuck_IO_Chout;
}
extern "C" {
    #[link_name = "\u{1}cherr"]
    fn Chuck_VM_cherr(this: *const Chuck_VM) -> *mut Chuck_IO_Cherr;
}
extern "C" {
    #[link_name = "\u{1}input_ref"]
    fn Chuck_VM_input_ref(this: *mut Chuck_VM) -> *const f32;
}
extern "C" {
    #[link_name = "\u{1}output_ref"]
    fn Chuck_VM_output_ref(this: *mut Chuck_VM) -> *mut f32;
}
extern "C" {
    #[link_name = "\u{1}spork"]
    fn Chuck_VM_spork1(this: *mut Chuck_VM, shred: *mut Chuck_VM_Shred) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}free"]
    fn Chuck_VM_free(
        this: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
        cascade: c_ulong,
        dec: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}dump"]
    fn Chuck_VM_dump(this: *mut Chuck_VM, shred: *mut Chuck_VM_Shred);
}
extern "C" {
    #[link_name = "\u{1}release_dump"]
    fn Chuck_VM_release_dump(this: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM"]
    fn Chuck_VM_Chuck_VM(this: *mut Chuck_VM);
}
impl Chuck_VM {
    #[inline]
    unsafe fn initialize(
        &mut self,
        srate: c_ulong,
        dac_chan: c_ulong,
        adc_chan: c_ulong,
        adaptive: c_ulong,
        halt: c_ulong,
    ) -> c_ulong {
        Chuck_VM_initialize(self, srate, dac_chan, adc_chan, adaptive, halt)
    }
    #[inline]
    unsafe fn initialize_synthesis(&mut self) -> c_ulong {
        Chuck_VM_initialize_synthesis(self)
    }
    #[inline]
    unsafe fn setCarrier(&mut self, c: *mut Chuck_Carrier) -> c_ulong {
        Chuck_VM_setCarrier(self, c)
    }
    #[inline]
    unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_shutdown(self)
    }
    #[inline]
    unsafe fn has_init(&mut self) -> c_ulong {
        Chuck_VM_has_init(self)
    }
    #[inline]
    unsafe fn start(&mut self) -> c_ulong {
        Chuck_VM_start(self)
    }
    #[inline]
    unsafe fn running(&mut self) -> c_ulong {
        Chuck_VM_running(self)
    }
    #[inline]
    unsafe fn stop(&mut self) -> c_ulong {
        Chuck_VM_stop(self)
    }
    #[inline]
    unsafe fn runningState(&mut self) -> *mut c_ulong {
        Chuck_VM_runningState(self)
    }
    #[inline]
    unsafe fn spork(
        &mut self,
        code: *mut Chuck_VM_Code,
        parent: *mut Chuck_VM_Shred,
        immediate: c_ulong,
    ) -> *mut Chuck_VM_Shred {
        Chuck_VM_spork(self, code, parent, immediate)
    }
    #[inline]
    unsafe fn shreduler(&self) -> *mut Chuck_VM_Shreduler {
        Chuck_VM_shreduler(self)
    }
    #[inline]
    unsafe fn next_id(&mut self) -> c_ulong {
        Chuck_VM_next_id(self)
    }
    #[inline]
    unsafe fn srate(&self) -> c_ulong {
        Chuck_VM_srate(self)
    }
    #[inline]
    unsafe fn run(&mut self, numFrames: c_long, input: *const f32, output: *mut f32) -> c_ulong {
        Chuck_VM_run(self, numFrames, input, output)
    }
    #[inline]
    unsafe fn compute(&mut self) -> c_ulong {
        Chuck_VM_compute(self)
    }
    #[inline]
    unsafe fn abort_current_shred(&mut self) -> c_ulong {
        Chuck_VM_abort_current_shred(self)
    }
    #[inline]
    unsafe fn invoke_static(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_invoke_static(self, shred)
    }
    #[inline]
    unsafe fn gc(&mut self) {
        Chuck_VM_gc(self)
    }
    #[inline]
    unsafe fn gc1(&mut self, amount: c_ulong) {
        Chuck_VM_gc1(self, amount)
    }
    #[inline]
    unsafe fn queue_msg(&mut self, msg: *mut Chuck_Msg, num_msg: c_int) -> c_ulong {
        Chuck_VM_queue_msg(self, msg, num_msg)
    }
    #[inline]
    unsafe fn queue_event(
        &mut self,
        event: *mut Chuck_Event,
        num_msg: c_int,
        buffer: *mut CBufferSimple,
    ) -> c_ulong {
        Chuck_VM_queue_event(self, event, num_msg, buffer)
    }
    #[inline]
    unsafe fn process_msg(&mut self, msg: *mut Chuck_Msg) -> c_ulong {
        Chuck_VM_process_msg(self, msg)
    }
    #[inline]
    unsafe fn get_reply(&mut self) -> *mut Chuck_Msg {
        Chuck_VM_get_reply(self)
    }
    #[inline]
    unsafe fn create_event_buffer(&mut self) -> *mut CBufferSimple {
        Chuck_VM_create_event_buffer(self)
    }
    #[inline]
    unsafe fn destroy_event_buffer(&mut self, buffer: *mut CBufferSimple) {
        Chuck_VM_destroy_event_buffer(self, buffer)
    }
    #[inline]
    unsafe fn last_error(&self) -> *const c_char {
        Chuck_VM_last_error(self)
    }
    #[inline]
    unsafe fn get_global_int(
        &mut self,
        name: crate::rtmidi_h_edited::string,
        callback: Option<unsafe extern "C" fn(arg1: c_long)>,
    ) -> c_ulong {
        Chuck_VM_get_global_int(self, name, callback)
    }
    #[inline]
    unsafe fn set_global_int(&mut self, name: crate::rtmidi_h_edited::string, val: c_long) -> c_ulong {
        Chuck_VM_set_global_int(self, name, val)
    }
    #[inline]
    unsafe fn get_global_float(
        &mut self,
        name: crate::rtmidi_h_edited::string,
        callback: Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> c_ulong {
        Chuck_VM_get_global_float(self, name, callback)
    }
    #[inline]
    unsafe fn set_global_float(&mut self, name: crate::rtmidi_h_edited::string, val: f64) -> c_ulong {
        Chuck_VM_set_global_float(self, name, val)
    }
    #[inline]
    unsafe fn signal_global_event(&mut self, name: crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_VM_signal_global_event(self, name)
    }
    #[inline]
    unsafe fn broadcast_global_event(&mut self, name: crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_VM_broadcast_global_event(self, name)
    }
    #[inline]
    unsafe fn init_global_int(&mut self, name: crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_VM_init_global_int(self, name)
    }
    #[inline]
    unsafe fn get_global_int_value(&mut self, name: crate::rtmidi_h_edited::string) -> c_long {
        Chuck_VM_get_global_int_value(self, name)
    }
    #[inline]
    unsafe fn get_ptr_to_global_int(&mut self, name: crate::rtmidi_h_edited::string) -> *mut c_long {
        Chuck_VM_get_ptr_to_global_int(self, name)
    }
    #[inline]
    unsafe fn init_global_float(&mut self, name: crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_VM_init_global_float(self, name)
    }
    #[inline]
    unsafe fn get_global_float_value(&mut self, name: crate::rtmidi_h_edited::string) -> f64 {
        Chuck_VM_get_global_float_value(self, name)
    }
    #[inline]
    unsafe fn get_ptr_to_global_float(&mut self, name: crate::rtmidi_h_edited::string) -> *mut f64 {
        Chuck_VM_get_ptr_to_global_float(self, name)
    }
    #[inline]
    unsafe fn init_global_event(&mut self, name: crate::rtmidi_h_edited::string, type_: *mut Chuck_Type) -> c_ulong {
        Chuck_VM_init_global_event(self, name, type_)
    }
    #[inline]
    unsafe fn get_global_event(&mut self, name: crate::rtmidi_h_edited::string) -> *mut Chuck_Event {
        Chuck_VM_get_global_event(self, name)
    }
    #[inline]
    unsafe fn get_ptr_to_global_event(&mut self, name: crate::rtmidi_h_edited::string) -> *mut Chuck_Event {
        Chuck_VM_get_ptr_to_global_event(self, name)
    }
    #[inline]
    unsafe fn handle_global_queue_messages(&mut self) {
        Chuck_VM_handle_global_queue_messages(self)
    }
    #[inline]
    unsafe fn carrier(&self) -> *mut Chuck_Carrier {
        Chuck_VM_carrier(self)
    }
    #[inline]
    unsafe fn env(&self) -> *mut Chuck_Env {
        Chuck_VM_env(self)
    }
    #[inline]
    unsafe fn chout(&self) -> *mut Chuck_IO_Chout {
        Chuck_VM_chout(self)
    }
    #[inline]
    unsafe fn cherr(&self) -> *mut Chuck_IO_Cherr {
        Chuck_VM_cherr(self)
    }
    #[inline]
    unsafe fn input_ref(&mut self) -> *const f32 {
        Chuck_VM_input_ref(self)
    }
    #[inline]
    unsafe fn output_ref(&mut self) -> *mut f32 {
        Chuck_VM_output_ref(self)
    }
    #[inline]
    unsafe fn spork1(&mut self, shred: *mut Chuck_VM_Shred) -> *mut Chuck_VM_Shred {
        Chuck_VM_spork1(self, shred)
    }
    #[inline]
    unsafe fn free(
        &mut self,
        shred: *mut Chuck_VM_Shred,
        cascade: c_ulong,
        dec: c_ulong,
    ) -> c_ulong {
        Chuck_VM_free(self, shred, cascade, dec)
    }
    #[inline]
    unsafe fn dump(&mut self, shred: *mut Chuck_VM_Shred) {
        Chuck_VM_dump(self, shred)
    }
    #[inline]
    unsafe fn release_dump(&mut self) {
        Chuck_VM_release_dump(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_VM_Chuck_VM(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_destructor"]
    fn Chuck_VM_Chuck_VM_destructor(this: *mut Chuck_VM);
}
const Chuck_Msg_Type_MSG_ADD: Chuck_Msg_Type = 1;
const Chuck_Msg_Type_MSG_REMOVE: Chuck_Msg_Type = 2;
const Chuck_Msg_Type_MSG_REMOVEALL: Chuck_Msg_Type = 3;
const Chuck_Msg_Type_MSG_REPLACE: Chuck_Msg_Type = 4;
const Chuck_Msg_Type_MSG_STATUS: Chuck_Msg_Type = 5;
const Chuck_Msg_Type_MSG_PAUSE: Chuck_Msg_Type = 6;
const Chuck_Msg_Type_MSG_KILL: Chuck_Msg_Type = 7;
const Chuck_Msg_Type_MSG_TIME: Chuck_Msg_Type = 8;
const Chuck_Msg_Type_MSG_RESET_ID: Chuck_Msg_Type = 9;
const Chuck_Msg_Type_MSG_DONE: Chuck_Msg_Type = 10;
const Chuck_Msg_Type_MSG_ABORT: Chuck_Msg_Type = 11;
const Chuck_Msg_Type_MSG_ERROR: Chuck_Msg_Type = 12;
const Chuck_Msg_Type_MSG_CLEARVM: Chuck_Msg_Type = 13;
pub type Chuck_Msg_Type = u32;
pub type ck_msg_func = Option<unsafe extern "C" fn(msg: *const Chuck_Msg)>;
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Msg {
    pub type_: c_ulong,
    pub param: c_ulong,
    pub code: *mut Chuck_VM_Code,
    pub shred: *mut Chuck_VM_Shred,
    pub when: f64,
    pub user: *mut c_void,
    pub reply: ck_msg_func,
    pub replyA: c_ulong,
    pub replyB: c_ulong,
    pub replyC: *mut c_void,
    pub args: *mut crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}clear"]
    fn Chuck_Msg_clear(this: *mut Chuck_Msg);
}
extern "C" {
    #[link_name = "\u{1}set"]
    fn Chuck_Msg_set(this: *mut Chuck_Msg, vargs: *const crate::dts::vector);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Msg"]
    fn Chuck_Msg_Chuck_Msg(this: *mut Chuck_Msg);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Msg_destructor"]
    fn Chuck_Msg_Chuck_Msg_destructor(this: *mut Chuck_Msg);
}
impl Chuck_Msg {
    #[inline]
    unsafe fn clear(&mut self) {
        Chuck_Msg_clear(self)
    }
    #[inline]
    unsafe fn set(&mut self, vargs: *const crate::dts::vector) {
        Chuck_Msg_set(self, vargs)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Msg_Chuck_Msg(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    unsafe fn destruct(&mut self) {
        Chuck_Msg_Chuck_Msg_destructor(self)
    }
}
#[repr(C)]
struct Chuck_Compiler__bindgen_vtable(c_void);
#[repr(C)]
pub struct Chuck_Compiler {
    pub vtable_: *const Chuck_Compiler__bindgen_vtable,
    pub m_carrier: *mut Chuck_Carrier,
    pub emitter: *mut Chuck_Emitter,
    pub code: *mut Chuck_VM_Code,
    pub m_auto_depend: c_ulong,
    pub m_recent: std::collections::HashMap::new(),
    pub m_dlls: crate::dts::list,
    pub m_cklibs_to_preload: crate::dts::list,
}
extern "C" {
    #[link_name = "\u{1}env"]
    fn Chuck_Compiler_env(this: *const Chuck_Compiler) -> *mut Chuck_Env;
}
extern "C" {
    #[link_name = "\u{1}vm"]
    fn Chuck_Compiler_vm(this: *const Chuck_Compiler) -> *mut Chuck_VM;
}
extern "C" {
    #[link_name = "\u{1}carrier"]
    fn Chuck_Compiler_carrier(this: *const Chuck_Compiler) -> *mut Chuck_Carrier;
}
extern "C" {
    #[link_name = "\u{1}setCarrier"]
    fn Chuck_Compiler_setCarrier(this: *mut Chuck_Compiler, c: *mut Chuck_Carrier) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    fn Chuck_Compiler_initialize(
        this: *mut Chuck_Compiler,
        chugin_search_paths: *mut crate::dts::list,
        named_dls: *mut crate::dts::list,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    fn Chuck_Compiler_shutdown(this: *mut Chuck_Compiler);
}
extern "C" {
    #[link_name = "\u{1}bind"]
    fn Chuck_Compiler_bind(
        this: *mut Chuck_Compiler,
        query_func: f_ck_query,
        name: *const crate::rtmidi_h_edited::string,
        nspc: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_auto_depend"]
    fn Chuck_Compiler_set_auto_depend(this: *mut Chuck_Compiler, v: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}go"]
    fn Chuck_Compiler_go(
        this: *mut Chuck_Compiler,
        filename: *const crate::rtmidi_h_edited::string,
        fd: *mut FILE,
        str_src: *const c_char,
        full_path: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}resolve"]
    fn Chuck_Compiler_resolve(this: *mut Chuck_Compiler, type_: *const crate::rtmidi_h_edited::string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}output"]
    fn Chuck_Compiler_output(this: *mut Chuck_Compiler) -> *mut Chuck_VM_Code;
}
extern "C" {
    #[link_name = "\u{1}do_entire_file"]
    fn Chuck_Compiler_do_entire_file(
        this: *mut Chuck_Compiler,
        context: *mut Chuck_Context,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_only_classes"]
    fn Chuck_Compiler_do_only_classes(
        this: *mut Chuck_Compiler,
        context: *mut Chuck_Context,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_all_except_classes"]
    fn Chuck_Compiler_do_all_except_classes(
        this: *mut Chuck_Compiler,
        context: *mut Chuck_Context,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_normal"]
    fn Chuck_Compiler_do_normal(
        this: *mut Chuck_Compiler,
        path: *const crate::rtmidi_h_edited::string,
        fd: *mut FILE,
        str_src: *const c_char,
        full_path: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}find_recent_path"]
    fn Chuck_Compiler_find_recent_path(
        this: *mut Chuck_Compiler,
        path: *const crate::rtmidi_h_edited::string,
    ) -> *mut Chuck_Context;
}
extern "C" {
    #[link_name = "\u{1}find_recent_type"]
    fn Chuck_Compiler_find_recent_type(
        this: *mut Chuck_Compiler,
        type_: *const crate::rtmidi_h_edited::string,
    ) -> *mut Chuck_Context;
}
extern "C" {
    #[link_name = "\u{1}add_recent_path"]
    fn Chuck_Compiler_add_recent_path(
        this: *mut Chuck_Compiler,
        path: *const crate::rtmidi_h_edited::string,
        context: *mut Chuck_Context,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Compiler"]
    fn Chuck_Compiler_Chuck_Compiler(this: *mut Chuck_Compiler);
}
impl Chuck_Compiler {
    #[inline]
    unsafe fn env(&self) -> *mut Chuck_Env {
        Chuck_Compiler_env(self)
    }
    #[inline]
    unsafe fn vm(&self) -> *mut Chuck_VM {
        Chuck_Compiler_vm(self)
    }
    #[inline]
    unsafe fn carrier(&self) -> *mut Chuck_Carrier {
        Chuck_Compiler_carrier(self)
    }
    #[inline]
    unsafe fn setCarrier(&mut self, c: *mut Chuck_Carrier) -> c_ulong {
        Chuck_Compiler_setCarrier(self, c)
    }
    #[inline]
    unsafe fn initialize(
        &mut self,
        chugin_search_paths: *mut crate::dts::list,
        named_dls: *mut crate::dts::list,
    ) -> c_ulong {
        Chuck_Compiler_initialize(self, chugin_search_paths, named_dls)
    }
    #[inline]
    unsafe fn shutdown(&mut self) {
        Chuck_Compiler_shutdown(self)
    }
    #[inline]
    unsafe fn bind(
        &mut self,
        query_func: f_ck_query,
        name: *const crate::rtmidi_h_edited::string,
        nspc: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong {
        Chuck_Compiler_bind(self, query_func, name, nspc)
    }
    #[inline]
    unsafe fn set_auto_depend(&mut self, v: c_ulong) {
        Chuck_Compiler_set_auto_depend(self, v)
    }
    #[inline]
    unsafe fn go(
        &mut self,
        filename: *const crate::rtmidi_h_edited::string,
        fd: *mut FILE,
        str_src: *const c_char,
        full_path: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong {
        Chuck_Compiler_go(self, filename, fd, str_src, full_path)
    }
    #[inline]
    unsafe fn resolve(&mut self, type_: *const crate::rtmidi_h_edited::string) -> c_ulong {
        Chuck_Compiler_resolve(self, type_)
    }
    #[inline]
    unsafe fn output(&mut self) -> *mut Chuck_VM_Code {
        Chuck_Compiler_output(self)
    }
    #[inline]
    unsafe fn do_entire_file(&mut self, context: *mut Chuck_Context) -> c_ulong {
        Chuck_Compiler_do_entire_file(self, context)
    }
    #[inline]
    unsafe fn do_only_classes(&mut self, context: *mut Chuck_Context) -> c_ulong {
        Chuck_Compiler_do_only_classes(self, context)
    }
    #[inline]
    unsafe fn do_all_except_classes(&mut self, context: *mut Chuck_Context) -> c_ulong {
        Chuck_Compiler_do_all_except_classes(self, context)
    }
    #[inline]
    unsafe fn do_normal(
        &mut self,
        path: *const crate::rtmidi_h_edited::string,
        fd: *mut FILE,
        str_src: *const c_char,
        full_path: *const crate::rtmidi_h_edited::string,
    ) -> c_ulong {
        Chuck_Compiler_do_normal(self, path, fd, str_src, full_path)
    }
    #[inline]
    unsafe fn find_recent_path(&mut self, path: *const crate::rtmidi_h_edited::string) -> *mut Chuck_Context {
        Chuck_Compiler_find_recent_path(self, path)
    }
    #[inline]
    unsafe fn find_recent_type(&mut self, type_: *const crate::rtmidi_h_edited::string) -> *mut Chuck_Context {
        Chuck_Compiler_find_recent_type(self, type_)
    }
    #[inline]
    unsafe fn add_recent_path(
        &mut self,
        path: *const crate::rtmidi_h_edited::string,
        context: *mut Chuck_Context,
    ) -> c_ulong {
        Chuck_Compiler_add_recent_path(self, path, context)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Compiler_Chuck_Compiler(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Compiler_destructor"]
    fn Chuck_Compiler_Chuck_Compiler_destructor(this: *mut Chuck_Compiler);
}
