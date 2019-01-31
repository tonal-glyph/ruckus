//* This module replaces some C++ types in the C++ `std` namespace with their
//* Rust equivalents. (std::string, vector, queue, deque, map,
//* list)
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
#[feature(libc)]
use libc::*;
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::cell::UnsafeCell;
//* Rustified fstream
pub mod fstream {
    use std::fs;
    pub fn main(s: String) {
        let data = fs::read(s).expect("Unable to read file");
        println!("ChucK read {} bytes", data.len());
    }
}
//* A double-ended queue implemented with a growable ring buffer
pub type deque = VecDeque::new;
pub type hash = Vec::new;
pub type list = Vec::new;
//* A hash map implemented with linear probing and Robin Hood bucket stealing
pub type map = HashMap::new;
//* An actual void pointer
pub type nullptr_t = *const c_void;
//* A contiguous growable array type
pub type queue = Vec::new;
// pub fn fstream() -> Result<()> {
//     let mut f = File::open<p: AsRef<Path>;
//     Ok(())
// }
//* A contiguous growable array type
pub type vector = Vec::new;
//* Closest thing Rust has to a `void` value
pub type void = ();
pub enum Void {}
//* Rust char/string nonsense
pub type c_str = *mut c_char;
pub type c_constr = *const c_char;
pub type _CharT = *mut c_char;
pub type _Traits = PhantomData<UnsafeCell<_CharT>>;
pub type _Alloc = PhantomData<UnsafeCell<_CharT>>;
pub type _Iftrue = PhantomData<UnsafeCell<_CharT>>;
pub type _Tp2 = PhantomData<UnsafeCell<_CharT>>;
pub type _Value = PhantomData<UnsafeCell<_CharT>>;
pub type _Iterator = PhantomData<UnsafeCell<_CharT>>;
pub type _Iterator1 = PhantomData<UnsafeCell<_CharT>>;
pub type _Compare = PhantomData<UnsafeCell<_CharT>>;
pub type _Predicate = PhantomData<UnsafeCell<_CharT>>;
pub const __error_t_defined: u32 = 1;
pub const _GLIBCXX_CERRNO: u32 = 1;
pub const _FUNCTIONAL_HASH_H: u32 = 1;
pub const __cpp_lib_string_udls: u32 = 201304;
pub const _BASIC_STRING_TCC: u32 = 1;
pub const _GLIBCXX_VECTOR: u32 = 1;
pub const _STL_CONSTRUCT_H: u32 = 1;
pub const _STL_UNINITIALIZED_H: u32 = 1;
pub const _STL_VECTOR_H: u32 = 1;
pub const _STL_BVECTOR_H: u32 = 1;
pub const _VECTOR_TCC: u32 = 1;
pub const _GLIBCXX_IOSTREAM: u32 = 1;
pub const _GLIBCXX_OSTREAM: u32 = 1;
pub const _GLIBCXX_IOS: u32 = 1;
pub const _IOS_BASE_H: u32 = 1;
pub const _LOCALE_CLASSES_H: u32 = 1;
pub const _LOCALE_CLASSES_TCC: u32 = 1;
pub const _GLIBCXX_SYSTEM_ERROR: u32 = 1;
pub const _GLIBCXX_ERROR_CONSTANTS: u32 = 1;
pub const _GLIBCXX_STDEXCEPT: u32 = 1;
pub const _GLIBXX_STREAMBUF: u32 = 1;
pub const _STREAMBUF_TCC: u32 = 1;
pub const _BASIC_IOS_H: u32 = 1;
pub const _LOCALE_FACETS_H: u32 = 1;
pub const _WCTYPE_H: u32 = 1;
pub const _BITS_WCTYPE_WCHAR_H: u32 = 1;
pub const _GLIBCXX_CWCTYPE: u32 = 1;
pub const _STREAMBUF_ITERATOR_H: u32 = 1;
pub const _GLIBCXX_NUM_FACETS: u32 = 28;
pub const _GLIBCXX_NUM_CXX11_FACETS: u32 = 16;
pub const _GLIBCXX_NUM_UNICODE_FACETS: u32 = 2;
pub const _LOCALE_FACETS_TCC: u32 = 1;
pub const _BASIC_IOS_TCC: u32 = 1;
pub const _OSTREAM_TCC: u32 = 1;
pub const _GLIBCXX_ISTREAM: u32 = 1;
pub const _ISTREAM_TCC: u32 = 1;
pub const _GLIBCXX_SSTREAM: u32 = 1;
pub const _SSTREAM_TCC: u32 = 1;
pub const NUM_CHANNELS_DEFAULT: u32 = 2;
pub const NUM_BUFFERS_DEFAULT: u32 = 8;
pub const DEVICE_NUM_OUT_DEFAULT: u32 = 0;
pub const DEVICE_NUM_IN_DEFAULT: u32 = 0;
pub const SAMPLE_RATE_DEFAULT: u32 = 48000;
pub const BUFFER_SIZE_DEFAULT: u32 = 256;
pub type string = basic_string<c_char>;
pub type wstring = basic_string<u32>;
pub type u16string = basic_string<u16>;
pub type u32string = basic_string<u32>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct time_get {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct time_get_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct money_get {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct money_put {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct messages {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct messages_byname {
    pub _address: u8,
}
#[repr(C)]
pub struct basic_string<_CharT> {
    pub _M_dataplus: basic_string__Alloc_hider,
    pub _M_string_length: basic_string_size_type,
    pub __bindgen_anon_1: basic_string__bindgen_ty_2<_CharT>,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_string__Char_alloc_type = [u8; 0usize];
pub type basic_string__Alloc_traits = __alloc_traits;
pub type basic_string_traits_type<_Traits> = _Traits;
pub type basic_string_value_type = [u8; 0usize];
pub type basic_string_allocator_type =
    basic_string__Char_alloc_type;
pub type basic_string_size_type = [u8; 0usize];
pub type basic_string_difference_type = [u8; 0usize];
pub type basic_string_reference = [u8; 0usize];
pub type basic_string_const_reference = [u8; 0usize];
pub type basic_string_pointer = [u8; 0usize];
pub type basic_string_const_pointer = [u8; 0usize];
pub type basic_string_iterator =
    __normal_iterator<basic_string_pointer>;
pub type basic_string_const_iterator =
    __normal_iterator<basic_string_const_pointer>;
pub type basic_string_const_reverse_iterator =
    reverse_iterator<basic_string_const_iterator>;
pub type basic_string_reverse_iterator =
    reverse_iterator<basic_string_iterator>;
pub type basic_string___const_iterator =
    basic_string_const_iterator;
#[repr(C)]
pub struct basic_string__Alloc_hider {
    pub _M_p: basic_string_pointer,
}
pub const basic_string__S_local_capacity:
    basic_string__bindgen_ty_1 = 0;
pub type basic_string__bindgen_ty_1 = i32;
#[repr(C)]
pub struct basic_string__bindgen_ty_2<_CharT> {
    pub _M_local_buf: __BindgenUnionField<*mut _CharT>,
    pub _M_allocated_capacity:
        __BindgenUnionField<basic_string_size_type>,
    pub bindgen_union_field: u64,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
#[repr(C)]
#[derive(Debug)]
pub struct collate {
    pub _base: locale_facet,
    pub _M_c_locale_collate: __c_locale,
}
pub type collate_char_type<_CharT> = _CharT;
pub type collate_string_type = basic_string<_CharT>;
#[repr(C)]
#[derive(Debug)]
pub struct collate_byname {
    pub _base: collate,
}
pub type collate_byname_char_type<_CharT> = _CharT;
pub type collate_byname_string_type = basic_string<_CharT>;
#[repr(C)]
#[derive(Debug)]
pub struct numpunct<_CharT> {
    pub _base: locale_facet,
    pub _M_data: *mut numpunct___cache_type<_CharT>,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type numpunct_char_type<_CharT> = _CharT;
pub type numpunct_string_type = basic_string<_CharT>;
pub type numpunct___cache_type<_CharT> = __numpunct_cache<_CharT>;
extern "C" {
    pub static mut id: locale_id;
}
#[repr(C)]
#[derive(Debug)]
pub struct numpunct_byname<_CharT> {
    pub _base: numpunct<_CharT>,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type numpunct_byname_char_type<_CharT> = _CharT;
pub type numpunct_byname_string_type = basic_string<_CharT>;
#[repr(C)]
pub struct basic_stringbuf<_CharT> {
    pub _base: basic_streambuf<_CharT>,
    pub _M_mode: ios_base_openmode,
    pub _M_string: basic_stringbuf___string_type<_CharT>,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_stringbuf_char_type<_CharT> = _CharT;
pub type basic_stringbuf_traits_type<_Traits> = _Traits;
pub type basic_stringbuf_allocator_type<_Alloc> = _Alloc;
pub type basic_stringbuf_int_type = [u8; 0usize];
pub type basic_stringbuf_pos_type = [u8; 0usize];
pub type basic_stringbuf_off_type = [u8; 0usize];
pub type basic_stringbuf___streambuf_type<_CharT> =
    basic_streambuf<basic_stringbuf_char_type<_CharT>>;
pub type basic_stringbuf___string_type<_CharT> = basic_string<
    basic_stringbuf_char_type<_CharT>,
>;
pub type basic_stringbuf___size_type<_CharT> =
    basic_stringbuf___string_type<_CharT>;
#[repr(C)]
pub struct basic_stringbuf___xfer_bufptrs<_CharT> {
    pub _M_to: *mut basic_stringbuf<_CharT>,
    pub _M_goff: [basic_stringbuf_off_type; 3usize],
    pub _M_poff: [basic_stringbuf_off_type; 3usize],
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
#[repr(C)]
pub struct basic_istringstream<_CharT> {
    pub _base: basic_istream<_CharT>,
    pub _M_stringbuf: basic_istringstream___stringbuf_type<_CharT>,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_istringstream_char_type<_CharT> = _CharT;
pub type basic_istringstream_traits_type<_Traits> = _Traits;
pub type basic_istringstream_allocator_type<_Alloc> = _Alloc;
pub type basic_istringstream_int_type = [u8; 0usize];
pub type basic_istringstream_pos_type = [u8; 0usize];
pub type basic_istringstream_off_type = [u8; 0usize];
pub type basic_istringstream___string_type<_CharT> =
    basic_string<_CharT>;
pub type basic_istringstream___stringbuf_type<_CharT> =
    basic_stringbuf<_CharT>;
pub type basic_istringstream___istream_type<_CharT> =
    basic_istream<basic_istringstream_char_type<_CharT>>;
#[repr(C)]
pub struct basic_ostringstream<_CharT> {
    pub _base: basic_ostream<_CharT>,
    pub _M_stringbuf: basic_ostringstream___stringbuf_type<_CharT>,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_ostringstream_char_type<_CharT> = _CharT;
pub type basic_ostringstream_traits_type<_Traits> = _Traits;
pub type basic_ostringstream_allocator_type<_Alloc> = _Alloc;
pub type basic_ostringstream_int_type = [u8; 0usize];
pub type basic_ostringstream_pos_type = [u8; 0usize];
pub type basic_ostringstream_off_type = [u8; 0usize];
pub type basic_ostringstream___string_type<_CharT> =
    basic_string<_CharT>;
pub type basic_ostringstream___stringbuf_type<_CharT> =
    basic_stringbuf<_CharT>;
pub type basic_ostringstream___ostream_type<_CharT> =
    basic_ostream<basic_ostringstream_char_type<_CharT>>;
#[repr(C)]
pub struct basic_stringstream<_CharT> {
    pub _base: basic_iostream<_CharT>,
    pub _M_stringbuf: basic_stringstream___stringbuf_type<_CharT>,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_stringstream_char_type<_CharT> = _CharT;
pub type basic_stringstream_traits_type<_Traits> = _Traits;
pub type basic_stringstream_allocator_type<_Alloc> = _Alloc;
pub type basic_stringstream_int_type = [u8; 0usize];
pub type basic_stringstream_pos_type = [u8; 0usize];
pub type basic_stringstream_off_type = [u8; 0usize];
pub type basic_stringstream___string_type<_CharT> =
    basic_string<_CharT>;
pub type basic_stringstream___stringbuf_type<_CharT> =
    basic_stringbuf<_CharT>;
pub type basic_stringstream___iostream_type<_CharT> =
    basic_iostream<basic_stringstream_char_type<_CharT>>;
pub type __conditional_type___type<_Iftrue> = _Iftrue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __add_unsigned {
    pub _address: u8,
}
pub type __add_unsigned___if_type = u8;
pub type __add_unsigned___type = __add_unsigned___if_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __remove_unsigned {
    pub _address: u8,
}
pub type __remove_unsigned___if_type = u8;
pub type __remove_unsigned___type = __remove_unsigned___if_type;
pub type __promote___type = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __promote_2 {
    pub _address: u8,
}
pub type __promote_2___type<_Tp2> = _Tp2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __promote_3 {
    pub _address: u8,
}
pub type __promote_3___type<_Tp2> = _Tp2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __promote_4 {
    pub _address: u8,
}
pub type __promote_4___type<_Tp2> = _Tp2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __numeric_traits_integer {
    pub _address: u8,
}
extern "C" {
    pub static __min: _Value;
}
extern "C" {
    pub static __max: _Value;
}
extern "C" {
    pub static __is_signed: bool;
}
extern "C" {
    pub static __digits: c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __numeric_traits_floating {
    pub _address: u8,
}
extern "C" {
    pub static __max_digits10: c_int;
}
extern "C" {
    pub static __digits10: c_int;
}
extern "C" {
    pub static __max_exponent10: c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __numeric_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __normal_iterator<_Iterator> {
    pub _M_current: _Iterator,
    pub _phantom_0: PhantomData<UnsafeCell<_Iterator>>,
}
pub type __normal_iterator___traits_type = iterator_traits;
pub type __normal_iterator_iterator_type<_Iterator> = _Iterator;
pub type __normal_iterator_iterator_category =
    __normal_iterator___traits_type;
pub type __normal_iterator_value_type = __normal_iterator___traits_type;
pub type __normal_iterator_difference_type =
    __normal_iterator___traits_type;
pub type __normal_iterator_reference = __normal_iterator___traits_type;
pub type __normal_iterator_pointer = __normal_iterator___traits_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Iter_less_iter {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Iter_less_val {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Val_less_iter {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct _Iter_equal_to_iter {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct _Iter_equal_to_val {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct _Iter_comp_iter<_Compare> {
    pub _M_comp: _Compare,
    pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct _Iter_comp_val<_Compare> {
    pub _M_comp: _Compare,
    pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct _Val_comp_iter<_Compare> {
    pub _M_comp: _Compare,
    pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Iter_equals_val<_Value> {
    pub _M_value: *mut _Value,
    pub _phantom_0: PhantomData<UnsafeCell<_Value>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct _Iter_equals_iter<_Iterator1> {
    pub _M_it1: _Iterator1,
    pub _phantom_0: PhantomData<UnsafeCell<_Iterator1>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Iter_pred<_Predicate> {
    pub _M_pred: _Predicate,
    pub _phantom_0: PhantomData<UnsafeCell<_Predicate>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct _Iter_comp_to_val<_Compare, _Value> {
    pub _M_comp: _Compare,
    pub _M_value: *mut _Value,
    pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Value>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct _Iter_comp_to_iter<_Compare, _Iterator1> {
    pub _M_comp: _Compare,
    pub _M_it1: _Iterator1,
    pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Iterator1>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Iter_negate<_Predicate> {
    pub _M_pred: _Predicate,
    pub _phantom_0: PhantomData<UnsafeCell<_Predicate>>,
}