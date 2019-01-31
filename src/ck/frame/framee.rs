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
use crate::ck::def::defe::*;
use crate::ck::util::string::stringe::*;
use crate::dts::*;
use crate::sys::*;
pub type _Bit_type = c_ulong;
pub const std__S_word_bit: _bindgen_ty_1 = 64;
pub type _bindgen_ty_1 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Bit_reference {
    pub _M_p: *mut _Bit_type,
    pub _M_mask: _Bit_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Bit_iterator_base {
    pub _M_p: *mut _Bit_type,
    pub _M_offset: c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Bit_iterator {
    pub _base: _Bit_iterator_base,
}
pub type _Bit_iterator_reference = _Bit_reference;
pub type _Bit_iterator_pointer = *mut _Bit_reference;
pub type _Bit_iterator_iterator = _Bit_iterator;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Bit_const_iterator {
    pub _base: _Bit_iterator_base,
}
pub type _Bit_const_iterator_reference = bool;
pub type _Bit_const_iterator_const_reference = bool;
pub type _Bit_const_iterator_pointer = *const bool;
pub type _Bit_const_iterator_const_iterator = _Bit_const_iterator;
#[repr(C)]
pub struct _Bvector_base {
    pub _M_impl: _Bvector_base__Bvector_impl,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __alloc_traits {
    pub _address: u8,
}
pub type _Bvector_base__Bit_alloc_type = [u8; 0usize];
pub type _Bvector_base__Bit_alloc_traits = __alloc_traits;
pub type _Bvector_base__Bit_pointer = [u8; 0usize];
#[repr(C)]
pub struct _Bvector_base__Bvector_impl_data {
    pub _M_start: _Bit_iterator,
    pub _M_finish: _Bit_iterator,
    pub _M_end_of_storage: _Bvector_base__Bit_pointer,
}
#[repr(C)]
pub struct _Bvector_base__Bvector_impl {
    pub _base_1: _Bvector_base__Bvector_impl_data,
}
pub type _Bvector_base_allocator_type<_Alloc> = _Alloc;
pub type error_t = c_int;
#[repr(C)]
pub struct Chuck_Local {
    pub name: string,
    pub size: c_ulong,
    pub is_ref: c_ulong,
    pub is_obj: c_ulong,
    pub is_global: c_ulong,
    pub offset: c_ulong,
}
#[repr(C)]
pub struct Chuck_Frame {
    pub name: string,
    pub curr_offset: c_ulong,
    pub num_access: c_ulong,
    pub stack: Vec<f64>,
}
extern "C" {
    #[link_name = "\u{1}_ZN11Chuck_Frame10push_scopeEv"]
    pub fn Chuck_Frame_push_scope(this: *mut Chuck_Frame);
}
extern "C" {
    #[link_name = "\u{1}_ZN11Chuck_Frame11alloc_localEmRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEmmm"]
    pub fn Chuck_Frame_alloc_local(
        this: *mut Chuck_Frame,
        size: c_ulong,
        name: *const string,
        is_ref: c_ulong,
        is_obj: c_ulong,
        is_global: c_ulong,
    ) -> *mut Chuck_Local;
}
extern "C" {
    #[link_name = "\u{1}_ZNK11Chuck_Frame9get_scopeERSt6vectorIP11Chuck_LocalSaIS2_EE"]
    pub fn Chuck_Frame_get_scope(this: *const Chuck_Frame, out: *mut Vec<f64>);
}
extern "C" {
    #[link_name = "\u{1}_ZN11Chuck_Frame9pop_scopeERSt6vectorIP11Chuck_LocalSaIS2_EE"]
    pub fn Chuck_Frame_pop_scope(this: *mut Chuck_Frame, out: *mut Vec<f64>);
}
extern "C" {
    #[link_name = "\u{1}_ZN11Chuck_FrameC1Ev"]
    pub fn Chuck_Frame_Chuck_Frame(this: *mut Chuck_Frame);
}
impl Chuck_Frame {
    #[inline]
    pub unsafe fn push_scope(&mut self) {
        Chuck_Frame_push_scope(self)
    }
    #[inline]
    pub unsafe fn alloc_local(
        &mut self,
        size: c_ulong,
        name: *const string,
        is_ref: c_ulong,
        is_obj: c_ulong,
        is_global: c_ulong,
    ) -> *mut Chuck_Local {
        Chuck_Frame_alloc_local(self, size, name, is_ref, is_obj, is_global)
    }
    #[inline]
    pub unsafe fn get_scope(&self, out: *mut Vec<f64>) {
        Chuck_Frame_get_scope(self, out)
    }
    #[inline]
    pub unsafe fn pop_scope(&mut self, out: *mut Vec<f64>) {
        Chuck_Frame_pop_scope(self, out)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Frame_Chuck_Frame(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_2 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_3 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_4 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_5 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_6 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_7 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_8 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_9 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_10 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_11 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_12 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_13 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_14 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_15 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_16 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_17 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_18 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_19 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_20 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_21 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_22 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_23 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_24 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_25 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_26 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_27 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_28 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_29 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_30 {
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_42 {
    pub _address: u8,
}
pub type size_type = usize;
pub type iterator = _Bit_iterator;
