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
use crate::sys::*;
///* Gary Scavone's RtMidi header. Need a way to configure RtApi at runtime,
///* maybe a config.toml?
use libc::*;
use crate::*;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        transmute(self)
    }
}
impl<T> Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> Copy for __BindgenUnionField<T> {}
impl<T> Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> Hash for __BindgenUnionField<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}
impl<T> PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> Eq for __BindgenUnionField<T> {}
#[allow(unused_imports)]
pub type nullptr_t = *const c_void;
pub type string = basic_string<c_char>;
pub type wstring = basic_string<u32>;
pub type u16string = basic_string<u16>;
pub type u32string = basic_string<u32>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_stringbuf {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_istringstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_ostringstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_stringstream {
    pub _address: u8,
}
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
pub type basic_string_allocator_type = basic_string__Char_alloc_type;
pub type basic_string_size_type = [u8; 0usize];
pub type basic_string_difference_type = [u8; 0usize];
pub type basic_string_reference = [u8; 0usize];
pub type basic_string_const_reference = [u8; 0usize];
pub type basic_string_pointer = [u8; 0usize];
pub type basic_string_const_pointer = [u8; 0usize];
pub type basic_string_iterator = __normal_iterator<basic_string_pointer>;
pub type basic_string_const_iterator = __normal_iterator<basic_string_const_pointer>;
pub type basic_string_const_reverse_iterator = reverse_iterator<basic_string_const_iterator>;
pub type basic_string_reverse_iterator = reverse_iterator<basic_string_iterator>;
pub type basic_string___const_iterator = basic_string_const_iterator;
#[repr(C)]
pub struct basic_string__Alloc_hider {
    pub _M_p: basic_string_pointer,
}
pub const basic_string__S_local_capacity: basic_string__bindgen_ty_1 = 0;
pub type basic_string__bindgen_ty_1 = i32;
#[repr(C)]
pub struct basic_string__bindgen_ty_2<_CharT> {
    pub _M_local_buf: __BindgenUnionField<*mut _CharT>,
    pub _M_allocated_capacity: __BindgenUnionField<basic_string_size_type>,
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
pub struct exception__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct exception {
    pub vtable_: *const exception__bindgen_vtable,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt9exceptionD1Ev"]
    pub fn exception_exception_destructor(this: *mut exception);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9exception4whatEv"]
    pub fn exception_what(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct bad_exception {
    pub _base: exception,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13bad_exceptionD1Ev"]
    pub fn bad_exception_bad_exception_destructor(this: *mut bad_exception);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt13bad_exception4whatEv"]
    pub fn bad_exception_what(this: *mut c_void) -> *const c_char;
}
pub type terminate_handler = Option<unsafe extern "C" fn()>;
pub type unexpected_handler = Option<unsafe extern "C" fn()>;
extern "C" {
    #[link_name = "\u{1}_ZSt13set_terminatePFvvE"]
    pub fn set_terminate(arg1: terminate_handler) -> terminate_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt13get_terminatev"]
    pub fn get_terminate() -> terminate_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt9terminatev"]
    pub fn terminate();
}
extern "C" {
    #[link_name = "\u{1}_ZSt14set_unexpectedPFvvE"]
    pub fn set_unexpected(arg1: unexpected_handler) -> unexpected_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt14get_unexpectedv"]
    pub fn get_unexpected() -> unexpected_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt10unexpectedv"]
    pub fn unexpected();
}
extern "C" {
    #[link_name = "\u{1}_ZSt18uncaught_exceptionv"]
    pub fn uncaught_exception() -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZSt11_Hash_bytesPKvmm"]
    pub fn _Hash_bytes(__ptr: *const c_void, __len: usize, __seed: usize) -> usize;
}
extern "C" {
    #[link_name = "\u{1}_ZSt15_Fnv_hash_bytesPKvmm"]
    pub fn _Fnv_hash_bytes(__ptr: *const c_void, __len: usize, __seed: usize) -> usize;
}
#[repr(C)]
pub struct type_info__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct type_info {
    pub vtable_: *const type_info__bindgen_vtable,
    pub __name: *const c_char,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt9type_infoD1Ev"]
    pub fn type_info_type_info_destructor(this: *mut type_info);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9type_info14__is_pointer_pEv"]
    pub fn type_info___is_pointer_p(this: *mut c_void) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9type_info15__is_function_pEv"]
    pub fn type_info___is_function_p(this: *mut c_void) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9type_info10__do_catchEPKS_PPvj"]
    pub fn type_info___do_catch(
        this: *mut c_void,
        __thr_type: *const type_info,
        __thr_obj: *mut c_void,
        __outer: c_uint,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9type_info11__do_upcastEPKN10__cxxabiv117__class_type_infoEPPv"]
    pub fn type_info___do_upcast(
        this: *mut c_void,
        __target: *const __class_type_info,
        __obj_ptr: *mut c_void,
    ) -> bool;
}
#[repr(C)]
#[derive(Debug)]
pub struct bad_cast {
    pub _base: exception,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8bad_castD1Ev"]
    pub fn bad_cast_bad_cast_destructor(this: *mut bad_cast);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt8bad_cast4whatEv"]
    pub fn bad_cast_what(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct bad_typeid {
    pub _base: exception,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt10bad_typeidD1Ev"]
    pub fn bad_typeid_bad_typeid_destructor(this: *mut bad_typeid);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt10bad_typeid4whatEv"]
    pub fn bad_typeid_what(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct bad_alloc {
    pub _base: exception,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt9bad_allocD1Ev"]
    pub fn bad_alloc_bad_alloc_destructor(this: *mut bad_alloc);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt9bad_alloc4whatEv"]
    pub fn bad_alloc_what(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct bad_array_new_length {
    pub _base: bad_alloc,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt20bad_array_new_lengthD1Ev"]
    pub fn bad_array_new_length_bad_array_new_length_destructor(this: *mut bad_array_new_length);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt20bad_array_new_length4whatEv"]
    pub fn bad_array_new_length_what(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nothrow_t {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZSt7nothrow"]
    pub static nothrow: nothrow_t;
}
pub type new_handler = Option<unsafe extern "C" fn()>;
extern "C" {
    #[link_name = "\u{1}_ZSt15set_new_handlerPFvvE"]
    pub fn set_new_handler(arg1: new_handler) -> new_handler;
}
extern "C" {
    #[link_name = "\u{1}_ZSt15get_new_handlerv"]
    pub fn get_new_handler() -> new_handler;
}
#[allow(unused_imports)]
#[repr(C)]
#[derive(Debug)]
pub struct exception_ptr {
    pub _M_exception_object: *mut c_void,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptr4swapERS0_"]
    pub fn exception_ptr_swap(this: *mut exception_ptr, arg1: *mut exception_ptr);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt15__exception_ptr13exception_ptr20__cxa_exception_typeEv"]
    pub fn exception_ptr___cxa_exception_type(this: *const exception_ptr) -> *const type_info;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrC1Ev"]
    pub fn exception_ptr_exception_ptr(this: *mut exception_ptr);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrC1ERKS0_"]
    pub fn exception_ptr_exception_ptr1(this: *mut exception_ptr, arg1: *const exception_ptr);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrD1Ev"]
    pub fn exception_ptr_exception_ptr_destructor(this: *mut exception_ptr);
}
impl exception_ptr {
    #[inline]
    pub unsafe fn swap(&mut self, arg1: *mut exception_ptr) {
        exception_ptr_swap(self, arg1)
    }
    #[inline]
    pub unsafe fn __cxa_exception_type(&self) -> *const type_info {
        exception_ptr___cxa_exception_type(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        exception_ptr_exception_ptr(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const exception_ptr) -> Self {
        let mut __bindgen_tmp = uninitialized();
        exception_ptr_exception_ptr1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        exception_ptr_exception_ptr_destructor(self)
    }
}
extern "C" {
    #[link_name = "\u{1}_ZSt17current_exceptionv"]
    pub fn current_exception() -> exception_ptr;
}
extern "C" {
    #[link_name = "\u{1}_ZSt17rethrow_exceptionNSt15__exception_ptr13exception_ptrE"]
    pub fn rethrow_exception(arg1: exception_ptr);
}
pub type integral_constant_value_type<_Tp> = _Tp;
pub type integral_constant_type = u8;
extern "C" {
    pub static value: _Tp;
}
pub type true_type = u8;
pub type false_type = u8;
pub type __bool_constant = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __or_ {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __and_ {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __not_ {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __success_type {
    pub _address: u8,
}
pub type __success_type_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __failure_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_void_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_void {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_integral_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_integral {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_floating_point_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_floating_point {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_array {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_pointer_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_lvalue_reference {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_rvalue_reference {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_member_object_pointer_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_member_object_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_member_function_pointer_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_member_function_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_enum {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_union {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_class {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_function {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_null_pointer_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_null_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nullptr_t {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_reference {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_arithmetic {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_fundamental {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_object {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_scalar {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_compound {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_member_pointer_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_member_pointer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_referenceable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_const {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_volatile {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivial {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_copyable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_standard_layout {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_pod {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_literal_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_empty {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_polymorphic {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_final {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_abstract {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_signed {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_unsigned {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_array_known_bounds {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_array_unknown_bounds {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __do_is_destructible_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_destructible_impl {
    pub _address: u8,
}
pub type __is_destructible_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_destructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __do_is_nt_destructible_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nt_destructible_impl {
    pub _address: u8,
}
pub type __is_nt_destructible_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_nothrow_destructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __do_is_default_constructible_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_default_constructible_impl {
    pub _address: u8,
}
pub type __is_default_constructible_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_default_constructible_atom {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_default_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_copy_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_move_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nt_default_constructible_atom {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_nothrow_default_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nt_constructible_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_nothrow_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_nothrow_copy_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_nothrow_move_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_copy_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_move_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nt_assignable_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_nothrow_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_nothrow_copy_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_nothrow_move_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_default_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __do_is_implicitly_default_constructible_impl {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt45__do_is_implicitly_default_constructible_impl6__testEz"]
    pub fn __do_is_implicitly_default_constructible_impl___test() -> false_type;
}
impl __do_is_implicitly_default_constructible_impl {
    #[inline]
    pub unsafe fn __test() -> false_type {
        __do_is_implicitly_default_constructible_impl___test()
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_implicitly_default_constructible_impl {
    pub _address: u8,
}
pub type __is_implicitly_default_constructible_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_implicitly_default_constructible_safe {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_implicitly_default_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_copy_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_move_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_copy_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_move_assignable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_trivially_destructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct has_virtual_destructor {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct alignment_of {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rank {
    pub _base: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_same {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_base_of {
    pub _address: u8,
}
pub type __is_convertible_helper_type = is_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_convertible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct remove_const {
    pub _address: u8,
}
pub type remove_const_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct remove_volatile {
    pub _address: u8,
}
pub type remove_volatile_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct remove_cv {
    pub _address: u8,
}
pub type remove_cv_type = remove_const;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct add_const {
    pub _address: u8,
}
pub type add_const_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct add_volatile {
    pub _address: u8,
}
pub type add_volatile_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct add_cv {
    pub _address: u8,
}
pub type add_cv_type = add_const;
pub type remove_const_t = remove_const;
pub type remove_volatile_t = remove_volatile;
pub type remove_cv_t = remove_cv;
pub type add_const_t = add_const;
pub type add_volatile_t = add_volatile;
pub type add_cv_t = add_cv;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct remove_reference {
    pub _address: u8,
}
pub type remove_reference_type<_Tp> = _Tp;
pub type __add_lvalue_reference_helper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct add_lvalue_reference {
    pub _address: u8,
}
pub type __add_rvalue_reference_helper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct add_rvalue_reference {
    pub _address: u8,
}
pub type remove_reference_t = remove_reference;
pub type add_lvalue_reference_t = add_lvalue_reference;
pub type add_rvalue_reference_t = add_rvalue_reference;
pub type __match_cv_qualifiers___match = u8;
pub type __match_cv_qualifiers___type = __match_cv_qualifiers___match;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __make_unsigned {
    pub _address: u8,
}
pub type __make_unsigned___type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct make_unsigned {
    pub _address: u8,
}
pub type make_unsigned_type = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __make_signed {
    pub _address: u8,
}
pub type __make_signed___type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct make_signed {
    pub _address: u8,
}
pub type make_signed_type = u8;
pub type make_signed_t = make_signed;
pub type make_unsigned_t = make_unsigned;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct remove_extent {
    pub _address: u8,
}
pub type remove_extent_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct remove_all_extents {
    pub _address: u8,
}
pub type remove_all_extents_type<_Tp> = _Tp;
pub type remove_extent_t = remove_extent;
pub type remove_all_extents_t = remove_all_extents;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __remove_pointer_helper {
    pub _address: u8,
}
pub type __remove_pointer_helper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct remove_pointer {
    pub _address: u8,
}
pub type __add_pointer_helper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct add_pointer {
    pub _address: u8,
}
pub type remove_pointer_t = remove_pointer;
pub type add_pointer_t = add_pointer;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __aligned_storage_msa___type {
    pub __data: *mut c_uchar,
    pub __align: __aligned_storage_msa___type__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __aligned_storage_msa___type__bindgen_ty_1 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union aligned_storage_type {
    pub __data: *mut c_uchar,
    pub __align: aligned_storage_type__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aligned_storage_type__bindgen_ty_1 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __strictest_alignment {
    pub _address: u8,
}
pub type aligned_union___strictest = __strictest_alignment;
pub type aligned_union_type = u8;
extern "C" {
    pub static alignment_value: usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct decay {
    pub _address: u8,
}
pub type decay___remove_type = remove_reference;
pub type decay_type = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct reference_wrapper {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __strip_reference_wrapper {
    pub _address: u8,
}
pub type __strip_reference_wrapper___type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __decay_and_strip {
    pub _address: u8,
}
pub type __decay_and_strip___type = __strip_reference_wrapper;
pub type _Require = u8;
pub type conditional_type<_Iftrue> = _Iftrue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct common_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __do_common_type_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __common_type_impl {
    pub _address: u8,
}
pub type __common_type_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __do_member_type_wrapper {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __member_type_wrapper {
    pub _address: u8,
}
pub type __member_type_wrapper_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __expanded_common_type_wrapper {
    pub _address: u8,
}
pub type __expanded_common_type_wrapper_type = common_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct underlying_type {
    pub _address: u8,
}
pub type underlying_type_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __declval_protector {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct result_of {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __invoke_memfun_ref {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __invoke_memfun_deref {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __invoke_memobj_ref {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __invoke_memobj_deref {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __invoke_other {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_success {
    pub _address: u8,
}
pub type __result_of_success___invoke_type<_Tag> = _Tag;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memfun_ref_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memfun_ref {
    pub _address: u8,
}
pub type __result_of_memfun_ref_type<_MemPtr> = _MemPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memfun_deref_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memfun_deref {
    pub _address: u8,
}
pub type __result_of_memfun_deref_type<_MemPtr> = _MemPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memobj_ref_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memobj_ref {
    pub _address: u8,
}
pub type __result_of_memobj_ref_type<_MemPtr> = _MemPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memobj_deref_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memobj_deref {
    pub _address: u8,
}
pub type __result_of_memobj_deref_type<_MemPtr> = _MemPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memobj {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_memfun {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __inv_unwrap {
    pub _address: u8,
}
pub type __inv_unwrap_type<_Tp> = _Tp;
pub type __result_of_impl_type = __failure_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __result_of_other_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __invoke_result {
    pub _address: u8,
}
pub type aligned_storage_t = u8;
pub type aligned_union_t = u8;
pub type decay_t = decay;
pub type enable_if_t = u8;
pub type conditional_t = u8;
pub type common_type_t = common_type;
pub type underlying_type_t = underlying_type;
pub type result_of_t = result_of;
pub type __enable_if_t = u8;
pub type __void_t = c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __detector {
    pub _address: u8,
}
pub type __detector_value_t = false_type;
pub type __detector_type<_Default> = _Default;
pub type __detected_or = __detector;
pub type __detected_or_t = __detected_or;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tuple {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_tuple_like_impl {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_tuple_like {
    pub _address: u8,
}
#[allow(unused_imports)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __do_is_swappable_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __do_is_nothrow_swappable_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_swappable_impl {
    pub _address: u8,
}
pub type __is_swappable_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nothrow_swappable_impl {
    pub _address: u8,
}
pub type __is_nothrow_swappable_impl_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_swappable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nothrow_swappable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_invocable_impl {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_invocable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __call_is_nothrow {
    pub _address: u8,
}
pub type __call_is_nothrow_ = __call_is_nothrow;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nothrow_invocable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct __nonesuch {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __move_if_noexcept_cond {
    pub _address: u8,
}
#[repr(C)]
pub struct nested_exception__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct nested_exception {
    pub vtable_: *const nested_exception__bindgen_vtable,
    pub _M_ptr: exception_ptr,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt16nested_exceptionD1Ev"]
    pub fn nested_exception_nested_exception_destructor(this: *mut nested_exception);
}
#[repr(C)]
#[derive(Debug)]
pub struct _Nested_exception<_Except> {
    pub _base: _Except,
    pub _base_1: nested_exception,
    pub _phantom_0: PhantomData<UnsafeCell<_Except>>,
}
pub type __rethrow_if_nested_cond = u8;
pub type streamoff = c_long;
pub type streamsize = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fpos<_StateT> {
    pub _M_off: streamoff,
    pub _M_state: _StateT,
    pub _phantom_0: PhantomData<UnsafeCell<_StateT>>,
}
pub type streampos = fpos<mbstate_t>;
pub type wstreampos = fpos<mbstate_t>;
pub type u16streampos = fpos<mbstate_t>;
pub type u32streampos = fpos<mbstate_t>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_filebuf {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_ifstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_ofstream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_fstream {
    pub _address: u8,
}
pub type ios = basic_ios<c_char>;
pub type streambuf = basic_streambuf<c_char>;
pub type istream = basic_istream<c_char>;
pub type ostream = basic_ostream<c_char>;
pub type iostream = basic_iostream<c_char>;
pub type stringbuf = basic_stringbuf;
pub type istringstream = basic_istringstream;
pub type ostringstream = basic_ostringstream;
pub type stringstream = basic_stringstream;
pub type filebuf = basic_filebuf;
pub type ifstream = basic_ifstream;
pub type ofstream = basic_ofstream;
pub type fstream = basic_fstream;
pub type wios = basic_ios<u32>;
pub type wstreambuf = basic_streambuf<u32>;
pub type wistream = basic_istream<u32>;
pub type wostream = basic_ostream<u32>;
pub type wiostream = basic_iostream<u32>;
pub type wstringbuf = basic_stringbuf;
pub type wistringstream = basic_istringstream;
pub type wostringstream = basic_ostringstream;
pub type wstringstream = basic_stringstream;
pub type wfilebuf = basic_filebuf;
pub type wifstream = basic_ifstream;
pub type wofstream = basic_ofstream;
pub type wfstream = basic_fstream;
extern "C" {
    #[link_name = "\u{1}_ZSt21__throw_bad_exceptionv"]
    pub fn __throw_bad_exception();
}
extern "C" {
    #[link_name = "\u{1}_ZSt17__throw_bad_allocv"]
    pub fn __throw_bad_alloc();
}
extern "C" {
    #[link_name = "\u{1}_ZSt16__throw_bad_castv"]
    pub fn __throw_bad_cast();
}
extern "C" {
    #[link_name = "\u{1}_ZSt18__throw_bad_typeidv"]
    pub fn __throw_bad_typeid();
}
extern "C" {
    #[link_name = "\u{1}_ZSt19__throw_logic_errorPKc"]
    pub fn __throw_logic_error(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_domain_errorPKc"]
    pub fn __throw_domain_error(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt24__throw_invalid_argumentPKc"]
    pub fn __throw_invalid_argument(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_length_errorPKc"]
    pub fn __throw_length_error(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_out_of_rangePKc"]
    pub fn __throw_out_of_range(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt24__throw_out_of_range_fmtPKcz"]
    pub fn __throw_out_of_range_fmt(arg1: *const c_char, ...);
}
extern "C" {
    #[link_name = "\u{1}_ZSt21__throw_runtime_errorPKc"]
    pub fn __throw_runtime_error(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt19__throw_range_errorPKc"]
    pub fn __throw_range_error(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt22__throw_overflow_errorPKc"]
    pub fn __throw_overflow_error(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt23__throw_underflow_errorPKc"]
    pub fn __throw_underflow_error(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt19__throw_ios_failurePKc"]
    pub fn __throw_ios_failure(arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_system_errori"]
    pub fn __throw_system_error(arg1: c_int);
}
extern "C" {
    #[link_name = "\u{1}_ZSt20__throw_future_errori"]
    pub fn __throw_future_error(arg1: c_int);
}
extern "C" {
    #[link_name = "\u{1}_ZSt25__throw_bad_function_callv"]
    pub fn __throw_bad_function_call();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __true_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __false_type {
    pub _address: u8,
}
pub type __truth_type___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __traitor {
    pub _address: u8,
}
pub const __traitor___value: __traitor__bindgen_ty_1 = 0;
pub type __traitor__bindgen_ty_1 = i32;
pub type __traitor___type = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __are_same {
    pub _address: u8,
}
pub const __are_same___value: __are_same__bindgen_ty_1 = 0;
pub type __are_same__bindgen_ty_1 = i32;
pub type __are_same___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_void {
    pub _address: u8,
}
pub const __is_void___value: __is_void__bindgen_ty_1 = 0;
pub type __is_void__bindgen_ty_1 = i32;
pub type __is_void___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_integer {
    pub _address: u8,
}
pub const __is_integer___value: __is_integer__bindgen_ty_1 = 0;
pub type __is_integer__bindgen_ty_1 = i32;
pub type __is_integer___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_floating {
    pub _address: u8,
}
pub const __is_floating___value: __is_floating__bindgen_ty_1 = 0;
pub type __is_floating__bindgen_ty_1 = i32;
pub type __is_floating___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_pointer {
    pub _address: u8,
}
pub const __is_pointer___value: __is_pointer__bindgen_ty_1 = 0;
pub type __is_pointer__bindgen_ty_1 = i32;
pub type __is_pointer___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_arithmetic {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_scalar {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_char {
    pub _address: u8,
}
pub const __is_char___value: __is_char__bindgen_ty_1 = 0;
pub type __is_char__bindgen_ty_1 = i32;
pub type __is_char___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_byte {
    pub _address: u8,
}
pub const __is_byte___value: __is_byte__bindgen_ty_1 = 0;
pub type __is_byte__bindgen_ty_1 = i32;
pub type __is_byte___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_move_iterator {
    pub _address: u8,
}
pub const __is_move_iterator___value: __is_move_iterator__bindgen_ty_1 = 0;
pub type __is_move_iterator__bindgen_ty_1 = i32;
pub type __is_move_iterator___type = __false_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct piecewise_construct_t {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZStL19piecewise_construct"]
    pub static piecewise_construct: piecewise_construct_t;
}
#[repr(C)]
#[derive(Debug)]
pub struct __nonesuch_no_braces {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct __pair_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct pair<_T1, _T2> {
    pub first: _T1,
    pub second: _T2,
    pub _phantom_0: PhantomData<UnsafeCell<_T1>>,
    pub _phantom_1: PhantomData<UnsafeCell<_T2>>,
}
pub type pair_first_type<_T1> = _T1;
pub type pair_second_type<_T2> = _T2;
pub type pair__PCCP = u8;
pub type pair__PCCFP = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct input_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct output_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct forward_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bidirectional_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_access_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iterator {
    pub _address: u8,
}
pub type iterator_iterator_category<_Category> = _Category;
pub type iterator_value_type<_Tp> = _Tp;
pub type iterator_difference_type<_Distance> = _Distance;
pub type iterator_pointer<_Pointer> = _Pointer;
pub type iterator_reference<_Reference> = _Reference;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __iterator_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iterator_traits {
    pub _address: u8,
}
pub type _RequireInputIter = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _List_iterator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _List_const_iterator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __undefined {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __get_first_arg {
    pub _address: u8,
}
pub type __get_first_arg_type = __undefined;
pub type __get_first_arg_t = __get_first_arg;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __replace_first_arg {
    pub _address: u8,
}
pub type __replace_first_arg_t = __replace_first_arg;
pub type __make_not_void = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pointer_traits {
    pub _address: u8,
}
pub type pointer_traits___element_type = [u8; 0usize];
pub type pointer_traits___difference_type = [u8; 0usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pointer_traits___rebind {
    pub _address: u8,
}
pub type pointer_traits_pointer<_Ptr> = _Ptr;
pub type pointer_traits_element_type = __detected_or_t;
pub type pointer_traits_difference_type = __detected_or_t;
pub type pointer_traits_rebind = pointer_traits___rebind;
pub type __ptr_rebind = pointer_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct reverse_iterator<_Iterator> {
    pub current: _Iterator,
    pub _phantom_0: PhantomData<UnsafeCell<_Iterator>>,
}
pub type reverse_iterator___traits_type = iterator_traits;
pub type reverse_iterator_iterator_type<_Iterator> = _Iterator;
pub type reverse_iterator_difference_type = reverse_iterator___traits_type;
pub type reverse_iterator_pointer = reverse_iterator___traits_type;
pub type reverse_iterator_reference = reverse_iterator___traits_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct back_insert_iterator<_Container> {
    pub container: *mut _Container,
    pub _phantom_0: PhantomData<UnsafeCell<_Container>>,
}
pub type back_insert_iterator_container_type<_Container> = _Container;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct front_insert_iterator<_Container> {
    pub container: *mut _Container,
    pub _phantom_0: PhantomData<UnsafeCell<_Container>>,
}
pub type front_insert_iterator_container_type<_Container> = _Container;
#[repr(C)]
pub struct insert_iterator<_Container> {
    pub container: *mut _Container,
    pub iter: [u8; 0usize],
    pub _phantom_0: PhantomData<UnsafeCell<_Container>>,
}
pub type insert_iterator_container_type<_Container> = _Container;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct move_iterator<_Iterator> {
    pub _M_current: _Iterator,
    pub _phantom_0: PhantomData<UnsafeCell<_Iterator>>,
}
pub type move_iterator___traits_type = iterator_traits;
pub type move_iterator___base_ref = move_iterator___traits_type;
pub type move_iterator_iterator_type<_Iterator> = _Iterator;
pub type move_iterator_iterator_category = move_iterator___traits_type;
pub type move_iterator_value_type = move_iterator___traits_type;
pub type move_iterator_difference_type = move_iterator___traits_type;
pub type move_iterator_pointer<_Iterator> = _Iterator;
pub type move_iterator_reference = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __lc_rai {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct char_traits {
    pub _address: u8,
}
pub type __c_locale = __locale_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct codecvt_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct codecvt {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct codecvt_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct time_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct time_put {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct time_put_byname {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct money_base {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct messages_base {
    _unused: [u8; 0],
}
pub type __allocator_base = new_allocator;
#[repr(C)]
#[derive(Debug)]
pub struct allocator {
    pub _address: u8,
}
pub type allocator_size_type = usize;
pub type allocator_difference_type = isize;
pub type allocator_pointer<_Tp> = *mut _Tp;
pub type allocator_const_pointer<_Tp> = *const _Tp;
pub type allocator_reference<_Tp> = *mut _Tp;
pub type allocator_const_reference<_Tp> = *const _Tp;
pub type allocator_value_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct allocator_rebind {
    pub _address: u8,
}
pub type allocator_rebind_other = allocator;
pub type allocator_propagate_on_container_move_assignment = true_type;
pub type allocator_is_always_equal = true_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unary_function {
    pub _address: u8,
}
pub type unary_function_argument_type<_Arg> = _Arg;
pub type unary_function_result_type<_Result> = _Result;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct binary_function {
    pub _address: u8,
}
pub type binary_function_first_argument_type<_Arg1> = _Arg1;
pub type binary_function_second_argument_type<_Arg2> = _Arg2;
pub type binary_function_result_type<_Result> = _Result;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_transparent {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct plus {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct minus {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct multiplies {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct divides {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct modulus {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct negate {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct equal_to {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct not_equal_to {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct greater {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct less {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct greater_equal {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct less_equal {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct logical_and {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct logical_or {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct logical_not {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bit_and {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bit_or {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bit_xor {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bit_not {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unary_negate<_Predicate> {
    pub _M_pred: _Predicate,
    pub _phantom_0: PhantomData<UnsafeCell<_Predicate>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct binary_negate<_Predicate> {
    pub _M_pred: _Predicate,
    pub _phantom_0: PhantomData<UnsafeCell<_Predicate>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pointer_to_unary_function<_Arg, _Result> {
    pub _M_ptr: Option<unsafe extern "C" fn(arg1: _Arg) -> _Result>,
    pub _phantom_0: PhantomData<UnsafeCell<_Arg>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Result>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pointer_to_binary_function<_Arg1, _Arg2, _Result> {
    pub _M_ptr: Option<unsafe extern "C" fn(arg1: _Arg1, arg2: _Arg2) -> _Result>,
    pub _phantom_0: PhantomData<UnsafeCell<_Arg1>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Arg2>>,
    pub _phantom_2: PhantomData<UnsafeCell<_Result>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Identity {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Select1st {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Select2nd {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mem_fun_t<_Ret> {
    pub _M_f: Option<unsafe extern "C" fn() -> _Ret>,
    pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct const_mem_fun_t<_Ret> {
    pub _M_f: Option<unsafe extern "C" fn() -> _Ret>,
    pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mem_fun_ref_t<_Ret> {
    pub _M_f: Option<unsafe extern "C" fn() -> _Ret>,
    pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct const_mem_fun_ref_t<_Ret> {
    pub _M_f: Option<unsafe extern "C" fn() -> _Ret>,
    pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mem_fun1_t<_Ret, _Arg> {
    pub _M_f: Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
    pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Arg>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct const_mem_fun1_t<_Ret, _Arg> {
    pub _M_f: Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
    pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Arg>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mem_fun1_ref_t<_Ret, _Arg> {
    pub _M_f: Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
    pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Arg>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct const_mem_fun1_ref_t<_Ret, _Arg> {
    pub _M_f: Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
    pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Arg>>,
}
#[repr(C)]
pub struct binder1st<_Operation> {
    pub op: _Operation,
    pub value: [u8; 0usize],
    pub _phantom_0: PhantomData<UnsafeCell<_Operation>>,
}
#[repr(C)]
pub struct binder2nd<_Operation> {
    pub op: _Operation,
    pub value: [u8; 0usize],
    pub _phantom_0: PhantomData<UnsafeCell<_Operation>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct initializer_list<_E> {
    pub _M_array: initializer_list_iterator<_E>,
    pub _M_len: initializer_list_size_type,
    pub _phantom_0: PhantomData<UnsafeCell<_E>>,
}
pub type initializer_list_value_type<_E> = _E;
pub type initializer_list_reference<_E> = *const _E;
pub type initializer_list_const_reference<_E> = *const _E;
pub type initializer_list_size_type = usize;
pub type initializer_list_iterator<_E> = *const _E;
pub type initializer_list_const_iterator<_E> = *const _E;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct valarray {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __allocator_traits_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __allocator_traits_base___rebind {
    pub _address: u8,
}
pub type __allocator_traits_base___pointer = [u8; 0usize];
pub type __allocator_traits_base___c_pointer = [u8; 0usize];
pub type __allocator_traits_base___v_pointer = [u8; 0usize];
pub type __allocator_traits_base___cv_pointer = [u8; 0usize];
pub type __allocator_traits_base___pocca = [u8; 0usize];
pub type __allocator_traits_base___pocma = [u8; 0usize];
pub type __allocator_traits_base___pocs = [u8; 0usize];
pub type __allocator_traits_base___equal = [u8; 0usize];
pub type __alloc_rebind = __allocator_traits_base;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct allocator_traits {
    pub _address: u8,
}
pub type allocator_traits_allocator_type<_Alloc> = _Alloc;
pub type allocator_traits_value_type = [u8; 0usize];
pub type allocator_traits_pointer = __detected_or_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct allocator_traits__Ptr {
    pub _address: u8,
}
pub type allocator_traits__Ptr_type = [u8; 0usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct allocator_traits__Diff {
    pub _address: u8,
}
pub type allocator_traits__Diff_type = pointer_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct allocator_traits__Size {
    pub _address: u8,
}
pub type allocator_traits_const_pointer = [u8; 0usize];
pub type allocator_traits_void_pointer = allocator_traits__Ptr;
pub type allocator_traits_const_void_pointer = allocator_traits__Ptr;
pub type allocator_traits_difference_type = [u8; 0usize];
pub type allocator_traits_size_type = [u8; 0usize];
pub type allocator_traits_propagate_on_container_copy_assignment = __detected_or_t;
pub type allocator_traits_propagate_on_container_move_assignment = __detected_or_t;
pub type allocator_traits_propagate_on_container_swap = __detected_or_t;
pub type allocator_traits_is_always_equal = __detected_or_t;
pub type allocator_traits_rebind_alloc = __alloc_rebind;
pub type allocator_traits_rebind_traits = allocator_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct allocator_traits___construct_helper {
    pub _address: u8,
}
pub type allocator_traits___construct_helper_type<_Alloc> = _Alloc;
pub type allocator_traits___has_construct = allocator_traits___construct_helper;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_copy_insertable_impl {
    pub _address: u8,
}
pub type __is_copy_insertable_impl__Traits = allocator_traits;
pub type __is_copy_insertable_impl_type<_Alloc> = _Alloc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_copy_insertable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_allocator {
    pub _base: false_type,
}
pub type _RequireAllocator = u8;
extern "C" {
    #[link_name = "\u{1}_ZSt7getlineIcSt11char_traitsIcESaIcEERSt13basic_istreamIT_T0_ES7_RNSt7__cxx1112basic_stringIS4_S5_T1_EES4_"]
    pub fn getline(
        __in: *mut basic_istream<c_char>,
        __str: *mut basic_string<c_char>,
        __delim: c_char,
    ) -> *mut basic_istream<c_char>;
}
extern "C" {
    #[link_name = "\u{1}_ZSt7getlineIwSt11char_traitsIwESaIwEERSt13basic_istreamIT_T0_ES7_RNSt7__cxx1112basic_stringIS4_S5_T1_EES4_"]
    pub fn getline1(
        __in: *mut basic_istream<u32>,
        __str: *mut basic_string<u32>,
        __delim: u32,
    ) -> *mut basic_istream<u32>;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __hash_base {
    pub _address: u8,
}
pub type __hash_base_result_type<_Result> = _Result;
pub type __hash_base_argument_type<_Arg> = _Arg;
#[repr(C)]
#[derive(Debug)]
pub struct __poison_hash {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct hash {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Hash_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Fnv_hash_impl {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_fast_hash {
    pub _base: true_type,
}
extern "C" {
    pub static npos: basic_string_size_type;
}
#[repr(C)]
#[derive(Debug)]
pub struct locale {
    pub _M_impl: *mut locale__Impl,
}
pub type locale_category = c_int;
pub const locale__S_categories_size: locale__bindgen_ty_1 = 12;
pub type locale__bindgen_ty_1 = u32;
pub const locale_none: locale_category = 0;
pub const locale_ctype: locale_category = 1;
pub const locale_numeric: locale_category = 2;
pub const locale_collate: locale_category = 4;
pub const locale_time: locale_category = 8;
pub const locale_monetary: locale_category = 16;
pub const locale_messages: locale_category = 32;
pub const locale_all: locale_category = 63;
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale10_S_classicE"]
    pub static mut locale__S_classic: *mut locale__Impl;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale9_S_globalE"]
    pub static mut locale__S_global: *mut locale__Impl;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale13_S_categoriesE"]
    pub static locale__S_categories: *const *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale7_S_onceE"]
    pub static mut locale__S_once: __gthread_once_t;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale17_S_twinned_facetsE"]
    pub static mut locale__S_twinned_facets: [*const locale_id; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt6locale4nameB5cxx11Ev"]
    pub fn locale_name(this: *const locale) -> string;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale6globalERKS_"]
    pub fn locale_global(__loc: *const locale) -> locale;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale7classicEv"]
    pub fn locale_classic() -> *const locale;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6localeC1Ev"]
    pub fn locale_locale(this: *mut locale);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6localeC1ERKS_"]
    pub fn locale_locale1(this: *mut locale, __other: *const locale);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6localeC1EPKc"]
    pub fn locale_locale2(this: *mut locale, __s: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6localeC1ERKS_PKci"]
    pub fn locale_locale3(
        this: *mut locale,
        __base: *const locale,
        __s: *const c_char,
        __cat: locale_category,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6localeC1ERKS_S1_i"]
    pub fn locale_locale4(
        this: *mut locale,
        __base: *const locale,
        __add: *const locale,
        __cat: locale_category,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6localeD1Ev"]
    pub fn locale_locale_destructor(this: *mut locale);
}
impl locale {
    #[inline]
    pub unsafe fn name(&self) -> string {
        locale_name(self)
    }
    #[inline]
    pub unsafe fn global(__loc: *const locale) -> locale {
        locale_global(__loc)
    }
    #[inline]
    pub unsafe fn classic() -> *const locale {
        locale_classic()
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        locale_locale(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(__other: *const locale) -> Self {
        let mut __bindgen_tmp = uninitialized();
        locale_locale1(&mut __bindgen_tmp, __other);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new2(__s: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        locale_locale2(&mut __bindgen_tmp, __s);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new3(__base: *const locale, __s: *const c_char, __cat: locale_category) -> Self {
        let mut __bindgen_tmp = uninitialized();
        locale_locale3(&mut __bindgen_tmp, __base, __s, __cat);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new4(
        __base: *const locale,
        __add: *const locale,
        __cat: locale_category,
    ) -> Self {
        let mut __bindgen_tmp = uninitialized();
        locale_locale4(&mut __bindgen_tmp, __base, __add, __cat);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        locale_locale_destructor(self)
    }
}
#[repr(C)]
pub struct locale_facet__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct locale_facet {
    pub vtable_: *const locale_facet__bindgen_vtable,
    pub _M_refcount: _Atomic_word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct locale_facet___shim {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet11_S_c_localeE"]
    pub static mut locale_facet__S_c_locale: __c_locale;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet9_S_c_nameE"]
    pub static mut locale_facet__S_c_name: [c_char; 2usize];
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet7_S_onceE"]
    pub static mut locale_facet__S_once: __gthread_once_t;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet18_S_create_c_localeERP15__locale_structPKcS2_"]
    pub fn locale_facet__S_create_c_locale(
        __cloc: *mut __c_locale,
        __s: *const c_char,
        __old: __c_locale,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet17_S_clone_c_localeERP15__locale_struct"]
    pub fn locale_facet__S_clone_c_locale(__cloc: *mut __c_locale) -> __c_locale;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet19_S_destroy_c_localeERP15__locale_struct"]
    pub fn locale_facet__S_destroy_c_locale(__cloc: *mut __c_locale);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet20_S_lc_ctype_c_localeEP15__locale_structPKc"]
    pub fn locale_facet__S_lc_ctype_c_locale(__cloc: __c_locale, __s: *const c_char) -> __c_locale;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet15_S_get_c_localeEv"]
    pub fn locale_facet__S_get_c_locale() -> __c_locale;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facet13_S_get_c_nameEv"]
    pub fn locale_facet__S_get_c_name() -> *const c_char;
}
impl locale_facet {
    #[inline]
    pub unsafe fn _S_create_c_locale(
        __cloc: *mut __c_locale,
        __s: *const c_char,
        __old: __c_locale,
    ) {
        locale_facet__S_create_c_locale(__cloc, __s, __old)
    }
    #[inline]
    pub unsafe fn _S_clone_c_locale(__cloc: *mut __c_locale) -> __c_locale {
        locale_facet__S_clone_c_locale(__cloc)
    }
    #[inline]
    pub unsafe fn _S_destroy_c_locale(__cloc: *mut __c_locale) {
        locale_facet__S_destroy_c_locale(__cloc)
    }
    #[inline]
    pub unsafe fn _S_lc_ctype_c_locale(__cloc: __c_locale, __s: *const c_char) -> __c_locale {
        locale_facet__S_lc_ctype_c_locale(__cloc, __s)
    }
    #[inline]
    pub unsafe fn _S_get_c_locale() -> __c_locale {
        locale_facet__S_get_c_locale()
    }
    #[inline]
    pub unsafe fn _S_get_c_name() -> *const c_char {
        locale_facet__S_get_c_name()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5facetD1Ev"]
    pub fn locale_facet_facet_destructor(this: *mut locale_facet);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct locale_id {
    pub _M_index: usize,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale2id11_S_refcountE"]
    pub static mut locale_id__S_refcount: _Atomic_word;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt6locale2id5_M_idEv"]
    pub fn locale_id__M_id(this: *const locale_id) -> usize;
}
impl locale_id {
    #[inline]
    pub unsafe fn _M_id(&self) -> usize {
        locale_id__M_id(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct locale__Impl {
    pub _M_refcount: _Atomic_word,
    pub _M_facets: *mut *const locale_facet,
    pub _M_facets_size: usize,
    pub _M_caches: *mut *const locale_facet,
    pub _M_names: *mut c_char,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5_Impl11_S_id_ctypeE"]
    pub static mut locale__Impl__S_id_ctype: [*const locale_id; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5_Impl13_S_id_numericE"]
    pub static mut locale__Impl__S_id_numeric: [*const locale_id; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5_Impl13_S_id_collateE"]
    pub static mut locale__Impl__S_id_collate: [*const locale_id; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5_Impl10_S_id_timeE"]
    pub static mut locale__Impl__S_id_time: [*const locale_id; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5_Impl14_S_id_monetaryE"]
    pub static mut locale__Impl__S_id_monetary: [*const locale_id; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5_Impl14_S_id_messagesE"]
    pub static mut locale__Impl__S_id_messages: [*const locale_id; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_ZNSt6locale5_Impl19_S_facet_categoriesE"]
    pub static mut locale__Impl__S_facet_categories: [*const *const locale_id; 0usize];
}
pub const errc_address_family_not_supported: errc = 97;
pub const errc_address_in_use: errc = 98;
pub const errc_address_not_available: errc = 99;
pub const errc_already_connected: errc = 106;
pub const errc_argument_list_too_long: errc = 7;
pub const errc_argument_out_of_domain: errc = 33;
pub const errc_bad_address: errc = 14;
pub const errc_bad_file_descriptor: errc = 9;
pub const errc_bad_message: errc = 74;
pub const errc_broken_pipe: errc = 32;
pub const errc_connection_aborted: errc = 103;
pub const errc_connection_already_in_progress: errc = 114;
pub const errc_connection_refused: errc = 111;
pub const errc_connection_reset: errc = 104;
pub const errc_cross_device_link: errc = 18;
pub const errc_destination_address_required: errc = 89;
pub const errc_device_or_resource_busy: errc = 16;
pub const errc_directory_not_empty: errc = 39;
pub const errc_executable_format_error: errc = 8;
pub const errc_file_exists: errc = 17;
pub const errc_file_too_large: errc = 27;
pub const errc_filename_too_long: errc = 36;
pub const errc_function_not_supported: errc = 38;
pub const errc_host_unreachable: errc = 113;
pub const errc_identifier_removed: errc = 43;
pub const errc_illegal_byte_sequence: errc = 84;
pub const errc_inappropriate_io_control_operation: errc = 25;
pub const errc_interrupted: errc = 4;
pub const errc_invalid_argument: errc = 22;
pub const errc_invalid_seek: errc = 29;
pub const errc_io_error: errc = 5;
pub const errc_is_a_directory: errc = 21;
pub const errc_message_size: errc = 90;
pub const errc_network_down: errc = 100;
pub const errc_network_reset: errc = 102;
pub const errc_network_unreachable: errc = 101;
pub const errc_no_buffer_space: errc = 105;
pub const errc_no_child_process: errc = 10;
pub const errc_no_link: errc = 67;
pub const errc_no_lock_available: errc = 37;
pub const errc_no_message_available: errc = 61;
pub const errc_no_message: errc = 42;
pub const errc_no_protocol_option: errc = 92;
pub const errc_no_space_on_device: errc = 28;
pub const errc_no_stream_resources: errc = 63;
pub const errc_no_such_device_or_address: errc = 6;
pub const errc_no_such_device: errc = 19;
pub const errc_no_such_file_or_directory: errc = 2;
pub const errc_no_such_process: errc = 3;
pub const errc_not_a_directory: errc = 20;
pub const errc_not_a_socket: errc = 88;
pub const errc_not_a_stream: errc = 60;
pub const errc_not_connected: errc = 107;
pub const errc_not_enough_memory: errc = 12;
pub const errc_not_supported: errc = 95;
pub const errc_operation_canceled: errc = 125;
pub const errc_operation_in_progress: errc = 115;
pub const errc_operation_not_permitted: errc = 1;
pub const errc_operation_not_supported: errc = 95;
pub const errc_operation_would_block: errc = 11;
pub const errc_owner_dead: errc = 130;
pub const errc_permission_denied: errc = 13;
pub const errc_protocol_error: errc = 71;
pub const errc_protocol_not_supported: errc = 93;
pub const errc_read_only_file_system: errc = 30;
pub const errc_resource_deadlock_would_occur: errc = 35;
pub const errc_resource_unavailable_try_again: errc = 11;
pub const errc_result_out_of_range: errc = 34;
pub const errc_state_not_recoverable: errc = 131;
pub const errc_stream_timeout: errc = 62;
pub const errc_text_file_busy: errc = 26;
pub const errc_timed_out: errc = 110;
pub const errc_too_many_files_open_in_system: errc = 23;
pub const errc_too_many_files_open: errc = 24;
pub const errc_too_many_links: errc = 31;
pub const errc_too_many_symbolic_link_levels: errc = 40;
pub const errc_value_too_large: errc = 75;
pub const errc_wrong_protocol_type: errc = 91;
pub type errc = i32;
#[repr(C)]
pub struct __cow_string {
    pub __bindgen_anon_1: __cow_string__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __cow_string__bindgen_ty_1 {
    pub _M_p: *const c_char,
    pub _M_bytes: [c_char; 8usize],
    _bindgen_union_align: u64,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1Ev"]
    pub fn __cow_string___cow_string(this: *mut __cow_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn __cow_string___cow_string1(this: *mut __cow_string, arg1: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1EPKcm"]
    pub fn __cow_string___cow_string2(this: *mut __cow_string, arg1: *const c_char, arg2: usize);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1ERKS_"]
    pub fn __cow_string___cow_string3(this: *mut __cow_string, arg1: *const __cow_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringC1EOS_"]
    pub fn __cow_string___cow_string4(this: *mut __cow_string, arg1: *mut __cow_string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__cow_stringD1Ev"]
    pub fn __cow_string___cow_string_destructor(this: *mut __cow_string);
}
impl __cow_string {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        __cow_string___cow_string(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        __cow_string___cow_string1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new2(arg1: *const c_char, arg2: usize) -> Self {
        let mut __bindgen_tmp = uninitialized();
        __cow_string___cow_string2(&mut __bindgen_tmp, arg1, arg2);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new3(arg1: *const __cow_string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        __cow_string___cow_string3(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new4(arg1: *mut __cow_string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        __cow_string___cow_string4(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        __cow_string___cow_string_destructor(self)
    }
}
pub type __sso_string = basic_string<c_char>;
#[repr(C)]
pub struct logic_error {
    pub _base: exception,
    pub _M_msg: __cow_string,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn logic_error_logic_error(this: *mut logic_error, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorC1EPKc"]
    pub fn logic_error_logic_error1(this: *mut logic_error, arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorC1ERKS_"]
    pub fn logic_error_logic_error2(this: *mut logic_error, arg1: *const logic_error);
}
impl logic_error {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        logic_error_logic_error(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        logic_error_logic_error1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new2(arg1: *const logic_error) -> Self {
        let mut __bindgen_tmp = uninitialized();
        logic_error_logic_error2(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11logic_errorD1Ev"]
    pub fn logic_error_logic_error_destructor(this: *mut logic_error);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt11logic_error4whatEv"]
    pub fn logic_error_what(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
pub struct domain_error {
    pub _base: logic_error,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12domain_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn domain_error_domain_error(this: *mut domain_error, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12domain_errorC1EPKc"]
    pub fn domain_error_domain_error1(this: *mut domain_error, arg1: *const c_char);
}
impl domain_error {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        domain_error_domain_error(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        domain_error_domain_error1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12domain_errorD1Ev"]
    pub fn domain_error_domain_error_destructor(this: *mut domain_error);
}
#[repr(C)]
pub struct invalid_argument {
    pub _base: logic_error,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt16invalid_argumentC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn invalid_argument_invalid_argument(this: *mut invalid_argument, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt16invalid_argumentC1EPKc"]
    pub fn invalid_argument_invalid_argument1(this: *mut invalid_argument, arg1: *const c_char);
}
impl invalid_argument {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        invalid_argument_invalid_argument(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        invalid_argument_invalid_argument1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt16invalid_argumentD1Ev"]
    pub fn invalid_argument_invalid_argument_destructor(this: *mut invalid_argument);
}
#[repr(C)]
pub struct length_error {
    pub _base: logic_error,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12length_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn length_error_length_error(this: *mut length_error, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12length_errorC1EPKc"]
    pub fn length_error_length_error1(this: *mut length_error, arg1: *const c_char);
}
impl length_error {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        length_error_length_error(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        length_error_length_error1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12length_errorD1Ev"]
    pub fn length_error_length_error_destructor(this: *mut length_error);
}
#[repr(C)]
pub struct out_of_range {
    pub _base: logic_error,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12out_of_rangeC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn out_of_range_out_of_range(this: *mut out_of_range, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12out_of_rangeC1EPKc"]
    pub fn out_of_range_out_of_range1(this: *mut out_of_range, arg1: *const c_char);
}
impl out_of_range {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        out_of_range_out_of_range(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        out_of_range_out_of_range1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12out_of_rangeD1Ev"]
    pub fn out_of_range_out_of_range_destructor(this: *mut out_of_range);
}
#[repr(C)]
pub struct runtime_error {
    pub _base: exception,
    pub _M_msg: __cow_string,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn runtime_error_runtime_error(this: *mut runtime_error, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorC1EPKc"]
    pub fn runtime_error_runtime_error1(this: *mut runtime_error, arg1: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorC1ERKS_"]
    pub fn runtime_error_runtime_error2(this: *mut runtime_error, arg1: *const runtime_error);
}
impl runtime_error {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        runtime_error_runtime_error(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        runtime_error_runtime_error1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new2(arg1: *const runtime_error) -> Self {
        let mut __bindgen_tmp = uninitialized();
        runtime_error_runtime_error2(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt13runtime_errorD1Ev"]
    pub fn runtime_error_runtime_error_destructor(this: *mut runtime_error);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt13runtime_error4whatEv"]
    pub fn runtime_error_what(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
pub struct range_error {
    pub _base: runtime_error,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11range_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn range_error_range_error(this: *mut range_error, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11range_errorC1EPKc"]
    pub fn range_error_range_error1(this: *mut range_error, arg1: *const c_char);
}
impl range_error {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        range_error_range_error(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        range_error_range_error1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt11range_errorD1Ev"]
    pub fn range_error_range_error_destructor(this: *mut range_error);
}
#[repr(C)]
pub struct overflow_error {
    pub _base: runtime_error,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt14overflow_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn overflow_error_overflow_error(this: *mut overflow_error, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt14overflow_errorC1EPKc"]
    pub fn overflow_error_overflow_error1(this: *mut overflow_error, arg1: *const c_char);
}
impl overflow_error {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        overflow_error_overflow_error(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        overflow_error_overflow_error1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt14overflow_errorD1Ev"]
    pub fn overflow_error_overflow_error_destructor(this: *mut overflow_error);
}
#[repr(C)]
pub struct underflow_error {
    pub _base: runtime_error,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15underflow_errorC1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn underflow_error_underflow_error(this: *mut underflow_error, __arg: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15underflow_errorC1EPKc"]
    pub fn underflow_error_underflow_error1(this: *mut underflow_error, arg1: *const c_char);
}
impl underflow_error {
    #[inline]
    pub unsafe fn new(__arg: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        underflow_error_underflow_error(&mut __bindgen_tmp, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        underflow_error_underflow_error1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt15underflow_errorD1Ev"]
    pub fn underflow_error_underflow_error_destructor(this: *mut underflow_error);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_error_code_enum {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct is_error_condition_enum {
    pub _base: false_type,
}
#[repr(C)]
pub struct error_category__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct error_category {
    pub vtable_: *const error_category__bindgen_vtable,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt3_V214error_categoryD1Ev"]
    pub fn error_category_error_category_destructor(this: *mut error_category);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt3_V214error_category23default_error_conditionEi"]
    pub fn error_category_default_error_condition(this: *mut c_void, __i: c_int)
        -> error_condition;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt3_V214error_category10equivalentEiRKSt15error_condition"]
    pub fn error_category_equivalent(
        this: *mut c_void,
        __i: c_int,
        __cond: *const error_condition,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt3_V214error_category10equivalentERKSt10error_codei"]
    pub fn error_category_equivalent1(
        this: *mut c_void,
        __code: *const error_code,
        __i: c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt3_V215system_categoryEv"]
    pub fn system_category() -> *const error_category;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt3_V216generic_categoryEv"]
    pub fn generic_category() -> *const error_category;
}
extern "C" {
    #[link_name = "\u{1}_ZSt15make_error_codeSt4errc"]
    pub fn make_error_code(arg1: errc) -> error_code;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct error_code {
    pub _M_value: c_int,
    pub _M_cat: *const error_category,
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt10error_code23default_error_conditionEv"]
    pub fn error_code_default_error_condition(this: *const error_code) -> error_condition;
}
impl error_code {
    #[inline]
    pub unsafe fn default_error_condition(&self) -> error_condition {
        error_code_default_error_condition(self)
    }
}
extern "C" {
    #[link_name = "\u{1}_ZSt20make_error_conditionSt4errc"]
    pub fn make_error_condition(arg1: errc) -> error_condition;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct error_condition {
    pub _M_value: c_int,
    pub _M_cat: *const error_category,
}
#[repr(C)]
pub struct system_error {
    pub _base: runtime_error,
    pub _M_code: error_code,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12system_errorD1Ev"]
    pub fn system_error_system_error_destructor(this: *mut system_error);
}
pub const _Ios_Fmtflags__S_boolalpha: _Ios_Fmtflags = 1;
pub const _Ios_Fmtflags__S_dec: _Ios_Fmtflags = 2;
pub const _Ios_Fmtflags__S_fixed: _Ios_Fmtflags = 4;
pub const _Ios_Fmtflags__S_hex: _Ios_Fmtflags = 8;
pub const _Ios_Fmtflags__S_internal: _Ios_Fmtflags = 16;
pub const _Ios_Fmtflags__S_left: _Ios_Fmtflags = 32;
pub const _Ios_Fmtflags__S_oct: _Ios_Fmtflags = 64;
pub const _Ios_Fmtflags__S_right: _Ios_Fmtflags = 128;
pub const _Ios_Fmtflags__S_scientific: _Ios_Fmtflags = 256;
pub const _Ios_Fmtflags__S_showbase: _Ios_Fmtflags = 512;
pub const _Ios_Fmtflags__S_showpoint: _Ios_Fmtflags = 1024;
pub const _Ios_Fmtflags__S_showpos: _Ios_Fmtflags = 2048;
pub const _Ios_Fmtflags__S_skipws: _Ios_Fmtflags = 4096;
pub const _Ios_Fmtflags__S_unitbuf: _Ios_Fmtflags = 8192;
pub const _Ios_Fmtflags__S_uppercase: _Ios_Fmtflags = 16384;
pub const _Ios_Fmtflags__S_adjustfield: _Ios_Fmtflags = 176;
pub const _Ios_Fmtflags__S_basefield: _Ios_Fmtflags = 74;
pub const _Ios_Fmtflags__S_floatfield: _Ios_Fmtflags = 260;
pub const _Ios_Fmtflags__S_ios_fmtflags_end: _Ios_Fmtflags = 65536;
pub const _Ios_Fmtflags__S_ios_fmtflags_max: _Ios_Fmtflags = 2147483647;
pub const _Ios_Fmtflags__S_ios_fmtflags_min: _Ios_Fmtflags = -2147483648;
pub const _Ios_Openmode__S_app: _Ios_Openmode = 1;
pub const _Ios_Openmode__S_ate: _Ios_Openmode = 2;
pub const _Ios_Openmode__S_bin: _Ios_Openmode = 4;
pub const _Ios_Openmode__S_in: _Ios_Openmode = 8;
pub const _Ios_Openmode__S_out: _Ios_Openmode = 16;
pub const _Ios_Openmode__S_trunc: _Ios_Openmode = 32;
pub const _Ios_Openmode__S_ios_openmode_end: _Ios_Openmode = 65536;
pub const _Ios_Openmode__S_ios_openmode_max: _Ios_Openmode = 2147483647;
pub const _Ios_Openmode__S_ios_openmode_min: _Ios_Openmode = -2147483648;
pub const _Ios_Iostate__S_goodbit: _Ios_Iostate = 0;
pub const _Ios_Iostate__S_badbit: _Ios_Iostate = 1;
pub const _Ios_Iostate__S_eofbit: _Ios_Iostate = 2;
pub const _Ios_Iostate__S_failbit: _Ios_Iostate = 4;
pub const _Ios_Iostate__S_ios_iostate_end: _Ios_Iostate = 65536;
pub const _Ios_Iostate__S_ios_iostate_max: _Ios_Iostate = 2147483647;
pub const _Ios_Iostate__S_ios_iostate_min: _Ios_Iostate = -2147483648;
pub const _Ios_Seekdir__S_beg: _Ios_Seekdir = 0;
pub const _Ios_Seekdir__S_cur: _Ios_Seekdir = 1;
pub const _Ios_Seekdir__S_end: _Ios_Seekdir = 2;
pub const _Ios_Seekdir__S_ios_seekdir_end: _Ios_Seekdir = 65536;
pub const io_errc_stream: io_errc = 1;
pub type io_errc = i32;
extern "C" {
    #[link_name = "\u{1}_ZSt17iostream_categoryv"]
    pub fn iostream_category() -> *const error_category;
}
#[repr(C)]
pub struct ios_base__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct ios_base {
    pub vtable_: *const ios_base__bindgen_vtable,
    pub _M_precision: streamsize,
    pub _M_width: streamsize,
    pub _M_flags: ios_base_fmtflags,
    pub _M_exception: ios_base_iostate,
    pub _M_streambuf_state: ios_base_iostate,
    pub _M_callbacks: *mut ios_base__Callback_list,
    pub _M_word_zero: ios_base__Words,
    pub _M_local_word: [ios_base__Words; 8usize],
    pub _M_word_size: c_int,
    pub _M_word: *mut ios_base__Words,
    pub _M_ios_locale: locale,
}
#[repr(C)]
pub struct ios_base_failure {
    pub _base: system_error,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7failureB5cxx11C1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn ios_base_failure_failure(this: *mut ios_base_failure, __str: *const string);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7failureB5cxx11C1ERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEERKSt10error_code"]
    pub fn ios_base_failure_failure1(
        this: *mut ios_base_failure,
        arg1: *const string,
        arg2: *const error_code,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7failureB5cxx11C1EPKcRKSt10error_code"]
    pub fn ios_base_failure_failure2(
        this: *mut ios_base_failure,
        arg1: *const c_char,
        arg2: *const error_code,
    );
}
impl ios_base_failure {
    #[inline]
    pub unsafe fn new(__str: *const string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        ios_base_failure_failure(&mut __bindgen_tmp, __str);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const string, arg2: *const error_code) -> Self {
        let mut __bindgen_tmp = uninitialized();
        ios_base_failure_failure1(&mut __bindgen_tmp, arg1, arg2);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new2(arg1: *const c_char, arg2: *const error_code) -> Self {
        let mut __bindgen_tmp = uninitialized();
        ios_base_failure_failure2(&mut __bindgen_tmp, arg1, arg2);
        __bindgen_tmp
    }
}
pub type _Ios_Fmtflags = i32;
pub type _Ios_Iostate = i32;
pub type _Ios_Openmode = i32;
pub type _Ios_Seekdir = i32;
pub use self::_Ios_Fmtflags as ios_base_fmtflags;
pub use self::_Ios_Iostate as ios_base_iostate;
pub use self::_Ios_Openmode as ios_base_openmode;
pub use self::_Ios_Seekdir as ios_base_seekdir;
pub type ios_base_io_state = c_int;
pub type ios_base_open_mode = c_int;
pub type ios_base_seek_dir = c_int;
pub type ios_base_streampos = streampos;
pub type ios_base_streamoff = streamoff;
pub const ios_base_event_erase_event: ios_base_event = 0;
pub const ios_base_event_imbue_event: ios_base_event = 1;
pub const ios_base_event_copyfmt_event: ios_base_event = 2;
pub type ios_base_event = u32;
pub type ios_base_event_callback =
    Option<unsafe extern "C" fn(__e: ios_base_event, __b: *mut ios_base, __i: c_int)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ios_base__Callback_list {
    pub _M_next: *mut ios_base__Callback_list,
    pub _M_fn: ios_base_event_callback,
    pub _M_index: c_int,
    pub _M_refcount: _Atomic_word,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ios_base__Words {
    pub _M_pword: *mut c_void,
    pub _M_iword: c_long,
}
pub const ios_base__S_local_word_size: ios_base__bindgen_ty_1 = 8;
pub type ios_base__bindgen_ty_1 = u32;
#[repr(C)]
#[derive(Debug)]
pub struct ios_base_Init {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base4Init11_S_refcountE"]
    pub static mut ios_base_Init__S_refcount: _Atomic_word;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base4Init20_S_synced_with_stdioE"]
    pub static mut ios_base_Init__S_synced_with_stdio: bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base4InitC1Ev"]
    pub fn ios_base_Init_Init(this: *mut ios_base_Init);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base4InitD1Ev"]
    pub fn ios_base_Init_Init_destructor(this: *mut ios_base_Init);
}
impl ios_base_Init {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        ios_base_Init_Init(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        ios_base_Init_Init_destructor(self)
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base9boolalphaE"]
    pub static ios_base_boolalpha: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3decE"]
    pub static ios_base_dec: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base5fixedE"]
    pub static ios_base_fixed: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3hexE"]
    pub static ios_base_hex: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base8internalE"]
    pub static ios_base_internal: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base4leftE"]
    pub static ios_base_left: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3octE"]
    pub static ios_base_oct: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base5rightE"]
    pub static ios_base_right: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base10scientificE"]
    pub static ios_base_scientific: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base8showbaseE"]
    pub static ios_base_showbase: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base9showpointE"]
    pub static ios_base_showpoint: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7showposE"]
    pub static ios_base_showpos: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base6skipwsE"]
    pub static ios_base_skipws: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7unitbufE"]
    pub static ios_base_unitbuf: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base9uppercaseE"]
    pub static ios_base_uppercase: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base11adjustfieldE"]
    pub static ios_base_adjustfield: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base9basefieldE"]
    pub static ios_base_basefield: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base10floatfieldE"]
    pub static ios_base_floatfield: ios_base_fmtflags;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base6badbitE"]
    pub static ios_base_badbit: ios_base_iostate;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base6eofbitE"]
    pub static ios_base_eofbit: ios_base_iostate;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7failbitE"]
    pub static ios_base_failbit: ios_base_iostate;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7goodbitE"]
    pub static ios_base_goodbit: ios_base_iostate;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3appE"]
    pub static ios_base_app: ios_base_openmode;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3ateE"]
    pub static ios_base_ate: ios_base_openmode;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base6binaryE"]
    pub static ios_base_binary: ios_base_openmode;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base2inE"]
    pub static ios_base_in: ios_base_openmode;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3outE"]
    pub static ios_base_out: ios_base_openmode;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base5truncE"]
    pub static ios_base_trunc: ios_base_openmode;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3begE"]
    pub static ios_base_beg: ios_base_seekdir;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3curE"]
    pub static ios_base_cur: ios_base_seekdir;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base3endE"]
    pub static ios_base_end: ios_base_seekdir;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base17register_callbackEPFvNS_5eventERS_iEi"]
    pub fn ios_base_register_callback(
        this: *mut ios_base,
        __fn: ios_base_event_callback,
        __index: c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base17_M_call_callbacksENS_5eventE"]
    pub fn ios_base__M_call_callbacks(this: *mut ios_base, __ev: ios_base_event);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base20_M_dispose_callbacksEv"]
    pub fn ios_base__M_dispose_callbacks(this: *mut ios_base);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base13_M_grow_wordsEib"]
    pub fn ios_base__M_grow_words(
        this: *mut ios_base,
        __index: c_int,
        __iword: bool,
    ) -> *mut ios_base__Words;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7_M_initEv"]
    pub fn ios_base__M_init(this: *mut ios_base);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base15sync_with_stdioEb"]
    pub fn ios_base_sync_with_stdio(__sync: bool) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base5imbueERKSt6locale"]
    pub fn ios_base_imbue(this: *mut ios_base, __loc: *const locale) -> locale;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base6xallocEv"]
    pub fn ios_base_xalloc() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7_M_moveERS_"]
    pub fn ios_base__M_move(this: *mut ios_base, arg1: *mut ios_base);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7_M_swapERS_"]
    pub fn ios_base__M_swap(this: *mut ios_base, __rhs: *mut ios_base);
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_baseC1Ev"]
    pub fn ios_base_ios_base(this: *mut ios_base);
}
impl ios_base {
    #[inline]
    pub unsafe fn register_callback(&mut self, __fn: ios_base_event_callback, __index: c_int) {
        ios_base_register_callback(self, __fn, __index)
    }
    #[inline]
    pub unsafe fn _M_call_callbacks(&mut self, __ev: ios_base_event) {
        ios_base__M_call_callbacks(self, __ev)
    }
    #[inline]
    pub unsafe fn _M_dispose_callbacks(&mut self) {
        ios_base__M_dispose_callbacks(self)
    }
    #[inline]
    pub unsafe fn _M_grow_words(&mut self, __index: c_int, __iword: bool) -> *mut ios_base__Words {
        ios_base__M_grow_words(self, __index, __iword)
    }
    #[inline]
    pub unsafe fn _M_init(&mut self) {
        ios_base__M_init(self)
    }
    #[inline]
    pub unsafe fn sync_with_stdio(__sync: bool) -> bool {
        ios_base_sync_with_stdio(__sync)
    }
    #[inline]
    pub unsafe fn imbue(&mut self, __loc: *const locale) -> locale {
        ios_base_imbue(self, __loc)
    }
    #[inline]
    pub unsafe fn xalloc() -> c_int {
        ios_base_xalloc()
    }
    #[inline]
    pub unsafe fn _M_move(&mut self, arg1: *mut ios_base) {
        ios_base__M_move(self, arg1)
    }
    #[inline]
    pub unsafe fn _M_swap(&mut self, __rhs: *mut ios_base) {
        ios_base__M_swap(self, __rhs)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        ios_base_ios_base(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_base7failureB5cxx11D1Ev"]
    pub fn ios_base_failure_failure_destructor(this: *mut ios_base_failure);
}
extern "C" {
    #[link_name = "\u{1}_ZNKSt8ios_base7failureB5cxx114whatEv"]
    pub fn ios_base_failure_what(this: *mut c_void) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt8ios_baseD1Ev"]
    pub fn ios_base_ios_base_destructor(this: *mut ios_base);
}
#[repr(C)]
pub struct basic_streambuf__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct basic_streambuf<_CharT> {
    pub vtable_: *const basic_streambuf__bindgen_vtable,
    pub _M_in_beg: *mut basic_streambuf_char_type<_CharT>,
    pub _M_in_cur: *mut basic_streambuf_char_type<_CharT>,
    pub _M_in_end: *mut basic_streambuf_char_type<_CharT>,
    pub _M_out_beg: *mut basic_streambuf_char_type<_CharT>,
    pub _M_out_cur: *mut basic_streambuf_char_type<_CharT>,
    pub _M_out_end: *mut basic_streambuf_char_type<_CharT>,
    pub _M_buf_locale: locale,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_streambuf_char_type<_CharT> = _CharT;
pub type basic_streambuf_traits_type<_Traits> = _Traits;
pub type basic_streambuf_int_type = [u8; 0usize];
pub type basic_streambuf_pos_type = [u8; 0usize];
pub type basic_streambuf_off_type = [u8; 0usize];
pub type basic_streambuf___streambuf_type<_CharT> =
    basic_streambuf<basic_streambuf_char_type<_CharT>>;
extern "C" {
    #[link_name = "\u{1}_ZSt21__copy_streambufs_eofIcSt11char_traitsIcEElPSt15basic_streambufIT_T0_ES6_Rb"]
    pub fn __copy_streambufs_eof(
        __sbin: *mut basic_streambuf<c_char>,
        __sbout: *mut basic_streambuf<c_char>,
        __ineof: *mut bool,
    ) -> streamsize;
}
extern "C" {
    #[link_name = "\u{1}_ZSt21__copy_streambufs_eofIwSt11char_traitsIwEElPSt15basic_streambufIT_T0_ES6_Rb"]
    pub fn __copy_streambufs_eof1(
        __sbin: *mut basic_streambuf<u32>,
        __sbout: *mut basic_streambuf<u32>,
        __ineof: *mut bool,
    ) -> streamsize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ctype_base {
    pub _address: u8,
}
pub type ctype_base___to_type = *const c_int;
pub type ctype_base_mask = c_ushort;
pub const ctype_base_upper: ctype_base_mask = 256;
pub const ctype_base_lower: ctype_base_mask = 512;
pub const ctype_base_alpha: ctype_base_mask = 1024;
pub const ctype_base_digit: ctype_base_mask = 2048;
pub const ctype_base_xdigit: ctype_base_mask = 4096;
pub const ctype_base_space: ctype_base_mask = 8192;
pub const ctype_base_print: ctype_base_mask = 16384;
pub const ctype_base_graph: ctype_base_mask = 3076;
pub const ctype_base_cntrl: ctype_base_mask = 2;
pub const ctype_base_punct: ctype_base_mask = 4;
pub const ctype_base_alnum: ctype_base_mask = 3072;
pub const ctype_base_blank: ctype_base_mask = 1;
#[repr(C)]
pub struct istreambuf_iterator<_CharT> {
    pub _M_sbuf: *mut istreambuf_iterator_streambuf_type<_CharT>,
    pub _M_c: istreambuf_iterator_int_type,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type istreambuf_iterator_char_type<_CharT> = _CharT;
pub type istreambuf_iterator_traits_type<_Traits> = _Traits;
pub type istreambuf_iterator_int_type = [u8; 0usize];
pub type istreambuf_iterator_streambuf_type<_CharT> = basic_streambuf<_CharT>;
pub type istreambuf_iterator_istream_type<_CharT> = basic_istream<_CharT>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ostreambuf_iterator<_CharT> {
    pub _M_sbuf: *mut ostreambuf_iterator_streambuf_type<_CharT>,
    pub _M_failed: bool,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type ostreambuf_iterator_char_type<_CharT> = _CharT;
pub type ostreambuf_iterator_traits_type<_Traits> = _Traits;
pub type ostreambuf_iterator_streambuf_type<_CharT> = basic_streambuf<_CharT>;
pub type ostreambuf_iterator_ostream_type<_CharT> = basic_ostream<_CharT>;
extern "C" {
    #[link_name = "\u{1}_ZSt14__convert_to_vIfEvPKcRT_RSt12_Ios_IostateRKP15__locale_struct"]
    pub fn __convert_to_v(
        arg1: *const c_char,
        arg2: *mut f32,
        arg3: *mut ios_base_iostate,
        arg4: *const __c_locale,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZSt14__convert_to_vIdEvPKcRT_RSt12_Ios_IostateRKP15__locale_struct"]
    pub fn __convert_to_v1(
        arg1: *const c_char,
        arg2: *mut f64,
        arg3: *mut ios_base_iostate,
        arg4: *const __c_locale,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZSt14__convert_to_vIeEvPKcRT_RSt12_Ios_IostateRKP15__locale_struct"]
    pub fn __convert_to_v2(
        arg1: *const c_char,
        arg2: *mut f64,
        arg3: *mut ios_base_iostate,
        arg4: *const __c_locale,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pad {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct __ctype_abstract_base {
    pub _base: locale_facet,
}
pub type __ctype_abstract_base_char_type<_CharT> = _CharT;
#[repr(C)]
#[derive(Debug)]
pub struct ctype {
    pub _base: __ctype_abstract_base,
}
pub type ctype_char_type<_CharT> = _CharT;
pub type ctype_mask = __ctype_abstract_base;
#[repr(C)]
#[derive(Debug)]
pub struct ctype_byname {
    pub _base: ctype,
}
pub type ctype_byname_mask = ctype;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __num_base {
    pub _address: u8,
}
pub const __num_base__S_ominus: __num_base__bindgen_ty_1 = 0;
pub const __num_base__S_oplus: __num_base__bindgen_ty_1 = 1;
pub const __num_base__S_ox: __num_base__bindgen_ty_1 = 2;
pub const __num_base__S_oX: __num_base__bindgen_ty_1 = 3;
pub const __num_base__S_odigits: __num_base__bindgen_ty_1 = 4;
pub const __num_base__S_odigits_end: __num_base__bindgen_ty_1 = 20;
pub const __num_base__S_oudigits: __num_base__bindgen_ty_1 = 20;
pub const __num_base__S_oudigits_end: __num_base__bindgen_ty_1 = 36;
pub const __num_base__S_oe: __num_base__bindgen_ty_1 = 18;
pub const __num_base__S_oE: __num_base__bindgen_ty_1 = 34;
pub const __num_base__S_oend: __num_base__bindgen_ty_1 = 36;
pub type __num_base__bindgen_ty_1 = u32;
pub const __num_base__S_iminus: __num_base__bindgen_ty_2 = 0;
pub const __num_base__S_iplus: __num_base__bindgen_ty_2 = 1;
pub const __num_base__S_ix: __num_base__bindgen_ty_2 = 2;
pub const __num_base__S_iX: __num_base__bindgen_ty_2 = 3;
pub const __num_base__S_izero: __num_base__bindgen_ty_2 = 4;
pub const __num_base__S_ie: __num_base__bindgen_ty_2 = 18;
pub const __num_base__S_iE: __num_base__bindgen_ty_2 = 24;
pub const __num_base__S_iend: __num_base__bindgen_ty_2 = 26;
pub type __num_base__bindgen_ty_2 = u32;
extern "C" {
    #[link_name = "\u{1}_ZNSt10__num_base12_S_atoms_outE"]
    pub static mut __num_base__S_atoms_out: *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt10__num_base11_S_atoms_inE"]
    pub static mut __num_base__S_atoms_in: *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_ZNSt10__num_base15_S_format_floatERKSt8ios_basePcc"]
    pub fn __num_base__S_format_float(__io: *const ios_base, __fptr: *mut c_char, __mod: c_char);
}
impl __num_base {
    #[inline]
    pub unsafe fn _S_format_float(__io: *const ios_base, __fptr: *mut c_char, __mod: c_char) {
        __num_base__S_format_float(__io, __fptr, __mod)
    }
}
#[repr(C)]
pub struct __numpunct_cache<_CharT> {
    pub _base: locale_facet,
    pub _M_grouping: *const c_char,
    pub _M_grouping_size: usize,
    pub _M_use_grouping: bool,
    pub _M_truename: *const _CharT,
    pub _M_truename_size: usize,
    pub _M_falsename: *const _CharT,
    pub _M_falsename_size: usize,
    pub _M_decimal_point: _CharT,
    pub _M_thousands_sep: _CharT,
    pub _M_atoms_out: [_CharT; 36usize],
    pub _M_atoms_in: [_CharT; 26usize],
    pub _M_allocated: bool,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
#[repr(C)]
#[derive(Debug)]
pub struct num_get {
    pub _base: locale_facet,
}
pub type num_get_char_type<_CharT> = _CharT;
pub type num_get_iter_type<_InIter> = _InIter;
#[repr(C)]
#[derive(Debug)]
pub struct num_put {
    pub _base: locale_facet,
}
pub type num_put_char_type<_CharT> = _CharT;
pub type num_put_iter_type<_OutIter> = _OutIter;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __use_cache {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZSt17__verify_groupingPKcmRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn __verify_grouping(
        __grouping: *const c_char,
        __grouping_size: usize,
        __grouping_tmp: *const string,
    ) -> bool;
}
#[repr(C)]
#[derive(Debug)]
pub struct basic_ios<_CharT> {
    pub _base: ios_base,
    pub _M_tie: *mut basic_ostream<_CharT>,
    pub _M_fill: basic_ios_char_type<_CharT>,
    pub _M_fill_init: bool,
    pub _M_streambuf: *mut basic_streambuf<_CharT>,
    pub _M_ctype: *const basic_ios___ctype_type,
    pub _M_num_put: *const basic_ios___num_put_type,
    pub _M_num_get: *const basic_ios___num_get_type,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_ios_char_type<_CharT> = _CharT;
pub type basic_ios_int_type = [u8; 0usize];
pub type basic_ios_pos_type = [u8; 0usize];
pub type basic_ios_off_type = [u8; 0usize];
pub type basic_ios_traits_type<_Traits> = _Traits;
pub type basic_ios___ctype_type = ctype;
pub type basic_ios___num_put_type = num_put;
pub type basic_ios___num_get_type = num_get;
#[repr(C)]
#[derive(Debug)]
pub struct basic_ostream<_CharT> {
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_ostream_char_type<_CharT> = _CharT;
pub type basic_ostream_int_type = [u8; 0usize];
pub type basic_ostream_pos_type = [u8; 0usize];
pub type basic_ostream_off_type = [u8; 0usize];
pub type basic_ostream_traits_type<_Traits> = _Traits;
pub type basic_ostream___streambuf_type<_CharT> = basic_streambuf<_CharT>;
pub type basic_ostream___ios_type<_CharT> = basic_ios<_CharT>;
pub type basic_ostream___ostream_type<_CharT> = basic_ostream<_CharT>;
pub type basic_ostream___num_put_type = num_put;
pub type basic_ostream___ctype_type = ctype;
#[repr(C)]
#[derive(Debug)]
pub struct basic_ostream_sentry {
    pub _M_ok: bool,
    pub _M_os: *mut basic_ostream<_CharT>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_convertible_to_basic_ostream_impl {
    pub _address: u8,
}
pub type __is_convertible_to_basic_ostream_impl___ostream_type = c_void;
pub type __do_is_convertible_to_basic_ostream_impl = remove_reference;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_convertible_to_basic_ostream {
    pub _address: u8,
}
pub type __is_convertible_to_basic_ostream_type = __not_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_insertable {
    pub _base: false_type,
}
pub type __rvalue_ostream_type = __is_convertible_to_basic_ostream;
#[repr(C)]
#[derive(Debug)]
pub struct basic_istream<_CharT> {
    pub _M_gcount: streamsize,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_istream_char_type<_CharT> = _CharT;
pub type basic_istream_int_type = [u8; 0usize];
pub type basic_istream_pos_type = [u8; 0usize];
pub type basic_istream_off_type = [u8; 0usize];
pub type basic_istream_traits_type<_Traits> = _Traits;
pub type basic_istream___streambuf_type<_CharT> = basic_streambuf<_CharT>;
pub type basic_istream___ios_type<_CharT> = basic_ios<_CharT>;
pub type basic_istream___istream_type<_CharT> = basic_istream<_CharT>;
pub type basic_istream___num_get_type = num_get;
pub type basic_istream___ctype_type = ctype;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_istream_sentry {
    pub _M_ok: bool,
}
pub type basic_istream_sentry_traits_type = _Traits;
pub type basic_istream_sentry___streambuf_type = basic_streambuf<_CharT>;
pub type basic_istream_sentry___istream_type = basic_istream<_CharT>;
pub type basic_istream_sentry___ctype_type = basic_istream___ctype_type;
pub type basic_istream_sentry___int_type = [u8; 0usize];
#[repr(C)]
#[derive(Debug)]
pub struct basic_iostream<_CharT> {
    pub _base: basic_istream<_CharT>,
    pub _base_1: basic_ostream<_CharT>,
    pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
}
pub type basic_iostream_char_type<_CharT> = _CharT;
pub type basic_iostream_int_type = [u8; 0usize];
pub type basic_iostream_pos_type = [u8; 0usize];
pub type basic_iostream_off_type = [u8; 0usize];
pub type basic_iostream_traits_type<_Traits> = _Traits;
pub type basic_iostream___istream_type<_CharT> = basic_istream<_CharT>;
pub type basic_iostream___ostream_type<_CharT> = basic_ostream<_CharT>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_convertible_to_basic_istream_impl {
    pub _address: u8,
}
pub type __is_convertible_to_basic_istream_impl___istream_type = c_void;
pub type __do_is_convertible_to_basic_istream_impl = remove_reference;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_convertible_to_basic_istream {
    pub _address: u8,
}
pub type __is_convertible_to_basic_istream_type = __not_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_extractable {
    pub _base: false_type,
}
pub type __rvalue_istream_type = __is_convertible_to_basic_istream;
extern "C" {
    #[link_name = "\u{1}_ZSt3cin"]
    pub static mut cin: istream;
}
extern "C" {
    #[link_name = "\u{1}_ZSt4cout"]
    pub static mut cout: ostream;
}
extern "C" {
    #[link_name = "\u{1}_ZSt4cerr"]
    pub static mut cerr: ostream;
}
extern "C" {
    #[link_name = "\u{1}_ZSt4clog"]
    pub static mut clog: ostream;
}
extern "C" {
    #[link_name = "\u{1}_ZSt4wcin"]
    pub static mut wcin: wistream;
}
extern "C" {
    #[link_name = "\u{1}_ZSt5wcout"]
    pub static mut wcout: wostream;
}
extern "C" {
    #[link_name = "\u{1}_ZSt5wcerr"]
    pub static mut wcerr: wostream;
}
extern "C" {
    #[link_name = "\u{1}_ZSt5wclog"]
    pub static mut wclog: wostream;
}
extern "C" {
    #[link_name = "\u{1}_ZStL8__ioinit"]
    pub static mut __ioinit: ios_base_Init;
}
#[repr(C)]
pub struct _Vector_base {
    pub _M_impl: _Vector_base__Vector_impl,
}
pub type _Vector_base__Tp_alloc_type = [u8; 0usize];
pub type _Vector_base_pointer = [u8; 0usize];
#[repr(C)]
pub struct _Vector_base__Vector_impl {
    pub _M_start: _Vector_base_pointer,
    pub _M_finish: _Vector_base_pointer,
    pub _M_end_of_storage: _Vector_base_pointer,
}
pub type _Vector_base_allocator_type<_Alloc> = _Alloc;
#[repr(C)]
pub struct vector {
    pub _base: _Vector_base,
}
pub type vector__Base = _Vector_base;
pub type vector__Tp_alloc_type = vector__Base;
pub type vector__Alloc_traits = __alloc_traits;
pub type vector_value_type<_Tp> = _Tp;
pub type vector_pointer = vector__Base;
pub type vector_const_pointer = vector__Alloc_traits;
pub type vector_reference = vector__Alloc_traits;
pub type vector_const_reference = vector__Alloc_traits;
pub type vector_iterator = __normal_iterator<vector_pointer>;
pub type vector_const_iterator = __normal_iterator<vector_const_pointer>;
pub type vector_const_reverse_iterator = reverse_iterator<vector_const_iterator>;
pub type vector_reverse_iterator = reverse_iterator<vector_iterator>;
pub type vector_size_type = usize;
pub type vector_difference_type = isize;
pub type vector_allocator_type<_Alloc> = _Alloc;
#[repr(C)]
#[derive(Debug)]
pub struct vector__Temporary_value {
    pub _M_this: *mut vector,
    pub __buf: u8,
}
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Deque_iterator {
    pub _M_cur: _Deque_iterator__Elt_pointer,
    pub _M_first: _Deque_iterator__Elt_pointer,
    pub _M_last: _Deque_iterator__Elt_pointer,
    pub _M_node: _Deque_iterator__Map_pointer,
}
pub type _Deque_iterator___ptr_to = pointer_traits;
pub type _Deque_iterator___iter = _Deque_iterator;
pub type _Deque_iterator_iterator = _Deque_iterator___iter;
pub type _Deque_iterator_const_iterator = _Deque_iterator___iter;
pub type _Deque_iterator__Elt_pointer = _Deque_iterator___ptr_to;
pub type _Deque_iterator__Map_pointer = _Deque_iterator___ptr_to;
pub type _Deque_iterator_iterator_category = random_access_iterator_tag;
pub type _Deque_iterator_value_type<_Tp> = _Tp;
pub type _Deque_iterator_pointer<_Ptr> = _Ptr;
pub type _Deque_iterator_reference<_Ref> = _Ref;
pub type _Deque_iterator_size_type = usize;
pub type _Deque_iterator_difference_type = isize;
pub type _Deque_iterator__Self = _Deque_iterator;
#[repr(C)]
pub struct _Deque_base {
    pub _M_impl: _Deque_base__Deque_impl,
}
pub type _Deque_base__Tp_alloc_type = [u8; 0usize];
pub type _Deque_base__Alloc_traits = __alloc_traits;
pub type _Deque_base__Ptr = [u8; 0usize];
pub type _Deque_base__Ptr_const = [u8; 0usize];
pub type _Deque_base__Map_alloc_type = [u8; 0usize];
pub type _Deque_base__Map_alloc_traits = __alloc_traits;
pub type _Deque_base_allocator_type<_Alloc> = _Alloc;
pub type _Deque_base_size_type = [u8; 0usize];
pub type _Deque_base_iterator = _Deque_iterator;
pub type _Deque_base_const_iterator = _Deque_iterator;
pub type _Deque_base__Map_pointer = [u8; 0usize];
#[repr(C)]
pub struct _Deque_base__Deque_impl {
    pub _M_map: _Deque_base__Map_pointer,
    pub _M_map_size: usize,
    pub _M_start: _Deque_base_iterator,
    pub _M_finish: _Deque_base_iterator,
}
pub const _Deque_base__S_initial_map_size: _Deque_base__bindgen_ty_1 = 0;
pub type _Deque_base__bindgen_ty_1 = i32;
#[repr(C)]
pub struct deque {
    pub _base: _Deque_base,
}
pub type deque__Base = _Deque_base;
pub type deque__Tp_alloc_type = deque__Base;
pub type deque__Alloc_traits = deque__Base;
pub type deque__Map_pointer = deque__Base;
pub type deque_value_type<_Tp> = _Tp;
pub type deque_pointer = deque__Alloc_traits;
pub type deque_const_pointer = deque__Alloc_traits;
pub type deque_reference = deque__Alloc_traits;
pub type deque_const_reference = deque__Alloc_traits;
pub type deque_iterator = deque__Base;
pub type deque_const_iterator = deque__Base;
pub type deque_const_reverse_iterator = reverse_iterator<deque_const_iterator>;
pub type deque_reverse_iterator = reverse_iterator<deque_iterator>;
pub type deque_size_type = usize;
pub type deque_difference_type = isize;
pub type deque_allocator_type<_Alloc> = _Alloc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __erased_type {
    pub _address: u8,
}
pub type __is_erased_or_convertible = __or_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct allocator_arg_t {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZStL13allocator_arg"]
    pub static allocator_arg: allocator_arg_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __uses_allocator_helper {
    pub _base: false_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uses_allocator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __uses_alloc_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __uses_alloc0 {
    pub _M_a: __uses_alloc0__Sink,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __uses_alloc0__Sink {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __uses_alloc1<_Alloc> {
    pub _M_a: *const _Alloc,
    pub _phantom_0: PhantomData<UnsafeCell<_Alloc>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __uses_alloc2<_Alloc> {
    pub _M_a: *const _Alloc,
    pub _phantom_0: PhantomData<UnsafeCell<_Alloc>>,
}
pub type __uses_alloc_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_uses_allocator_predicate {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_uses_allocator_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __is_nothrow_uses_allocator_constructible {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct queue<_Sequence> {
    pub c: _Sequence,
    pub _phantom_0: PhantomData<UnsafeCell<_Sequence>>,
}
pub type queue__Uses = u8;
pub type queue_value_type = [u8; 0usize];
pub type queue_reference = [u8; 0usize];
pub type queue_const_reference = [u8; 0usize];
pub type queue_size_type = [u8; 0usize];
pub type queue_container_type<_Sequence> = _Sequence;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct priority_queue<_Sequence, _Compare> {
    pub c: _Sequence,
    pub comp: _Compare,
    pub _phantom_0: PhantomData<UnsafeCell<_Sequence>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Compare>>,
}
pub type priority_queue__Uses = u8;
pub type priority_queue_value_type = [u8; 0usize];
pub type priority_queue_reference = [u8; 0usize];
pub type priority_queue_const_reference = [u8; 0usize];
pub type priority_queue_size_type = [u8; 0usize];
pub type priority_queue_container_type<_Sequence> = _Sequence;
pub type priority_queue_value_compare<_Compare> = _Compare;
#[allow(unused_imports)]
extern "C" {
    #[link_name = "\u{1}_ZN9__gnu_cxx27__verbose_terminate_handlerEv"]
    pub fn __verbose_terminate_handler();
}
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
pub type __normal_iterator_iterator_category = __normal_iterator___traits_type;
pub type __normal_iterator_value_type = __normal_iterator___traits_type;
pub type __normal_iterator_difference_type = __normal_iterator___traits_type;
pub type __normal_iterator_reference = __normal_iterator___traits_type;
pub type __normal_iterator_pointer = __normal_iterator___traits_type;
#[allow(unused_imports)]
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
#[derive(Debug, Copy, Clone)]
pub struct _Iter_equal_to_iter {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Iter_equal_to_val {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Iter_comp_iter<_Compare> {
    pub _M_comp: _Compare,
    pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Iter_comp_val<_Compare> {
    pub _M_comp: _Compare,
    pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct _Iter_comp_to_val<_Compare, _Value> {
    pub _M_comp: _Compare,
    pub _M_value: *mut _Value,
    pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
    pub _phantom_1: PhantomData<UnsafeCell<_Value>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Char_types {
    pub _address: u8,
}
pub type _Char_types_int_type = c_ulong;
pub type _Char_types_pos_type = streampos;
pub type _Char_types_off_type = streamoff;
pub type _Char_types_state_type = mbstate_t;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct char_traits {
//     pub _address: u8,
// }
pub type char_traits_char_type<_CharT> = _CharT;
pub type char_traits_int_type = _Char_types;
pub type char_traits_pos_type = _Char_types;
pub type char_traits_off_type = _Char_types;
pub type char_traits_state_type = _Char_types;
extern "C" {
    pub fn __uselocale(arg1: locale_t) -> locale_t;
}
#[repr(C)]
#[derive(Debug)]
pub struct new_allocator {
    pub _address: u8,
}
pub type new_allocator_size_type = usize;
pub type new_allocator_difference_type = isize;
pub type new_allocator_pointer<_Tp> = *mut _Tp;
pub type new_allocator_const_pointer<_Tp> = *const _Tp;
pub type new_allocator_reference<_Tp> = *mut _Tp;
pub type new_allocator_const_reference<_Tp> = *const _Tp;
pub type new_allocator_value_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct new_allocator_rebind {
    pub _address: u8,
}
pub type new_allocator_rebind_other = new_allocator;
pub type new_allocator_propagate_on_container_move_assignment = true_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __alloc_traits {
    pub _address: u8,
}
pub type __alloc_traits_allocator_type<_Alloc> = _Alloc;
pub type __alloc_traits__Base_type = allocator_traits;
pub type __alloc_traits_value_type = __alloc_traits__Base_type;
pub type __alloc_traits_pointer = __alloc_traits__Base_type;
pub type __alloc_traits_const_pointer = __alloc_traits__Base_type;
pub type __alloc_traits_size_type = __alloc_traits__Base_type;
pub type __alloc_traits_difference_type = __alloc_traits__Base_type;
pub type __alloc_traits_reference = *mut __alloc_traits_value_type;
pub type __alloc_traits_const_reference = *const __alloc_traits_value_type;
pub type __alloc_traits___is_custom_pointer = __and_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __alloc_traits_rebind {
    pub _address: u8,
}
pub type __alloc_traits_rebind_other = __alloc_traits__Base_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __max_align_ll: c_longlong,
    pub __bindgen_padding_0: u64,
    pub __max_align_ld: f64,
}
#[allow(unused_imports)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __cxa_refcounted_exception {
    _unused: [u8; 0],
}
extern "C" {
    pub fn __cxa_allocate_exception(arg1: usize) -> *mut c_void;
}
extern "C" {
    pub fn __cxa_free_exception(arg1: *mut c_void);
}
extern "C" {
    pub fn __cxa_init_primary_exception(
        object: *mut c_void,
        tinfo: *mut type_info,
        dest: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> *mut __cxa_refcounted_exception;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __class_type_info {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct __forced_unwind__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct __forced_unwind {
    pub vtable_: *const __forced_unwind__bindgen_vtable,
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
pub type __gnuc_va_list = __builtin_va_list;
pub type wint_t = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4usize],
    _bindgen_union_align: u32,
}
pub type mbstate_t = __mbstate_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const c_ushort,
    pub __ctype_tolower: *const c_int,
    pub __ctype_toupper: *const c_int,
    pub __names: [*const c_char; 13usize],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
#[repr(C)]
pub struct RtMidiError {
    pub _base: exception,
    pub message_: string,
    pub type_: RtMidiError_Type,
}
#[doc = "< A non-critical error."]
pub const RtMidiError_Type_WARNING: RtMidiError_Type = 0;
#[doc = "< A non-critical error which might be useful for debugging."]
pub const RtMidiError_Type_DEBUG_WARNING: RtMidiError_Type = 1;
#[doc = "< The default, unspecified error type."]
pub const RtMidiError_Type_UNSPECIFIED: RtMidiError_Type = 2;
#[doc = "< No devices found on system."]
pub const RtMidiError_Type_NO_DEVICES_FOUND: RtMidiError_Type = 3;
#[doc = "< An invalid device ID was specified."]
pub const RtMidiError_Type_INVALID_DEVICE: RtMidiError_Type = 4;
#[doc = "< An error occured during memory allocation."]
pub const RtMidiError_Type_MEMORY_ERROR: RtMidiError_Type = 5;
#[doc = "< An invalid parameter was specified to a function."]
pub const RtMidiError_Type_INVALID_PARAMETER: RtMidiError_Type = 6;
#[doc = "< The function was called incorrectly."]
pub const RtMidiError_Type_INVALID_USE: RtMidiError_Type = 7;
#[doc = "< A system driver error occured."]
pub const RtMidiError_Type_DRIVER_ERROR: RtMidiError_Type = 8;
#[doc = "< A system error occured."]
pub const RtMidiError_Type_SYSTEM_ERROR: RtMidiError_Type = 9;
#[doc = "< A thread error occured."]
pub const RtMidiError_Type_THREAD_ERROR: RtMidiError_Type = 10;
#[doc = "! Defined RtMidiError types."]
pub type RtMidiError_Type = u32;
#[repr(C)]
pub struct RtMidi__bindgen_vtable(c_void);
#[repr(C)]
pub struct RtMidi {
    pub vtable_: *const RtMidi__bindgen_vtable,
    pub apiData_: *mut c_void,
    pub connected_: bool,
    pub errorString_: string,
}
extern "C" {
    #[link_name = "\u{1}_ZN6RtMidi5errorEN11RtMidiError4TypeE"]
    pub fn RtMidi_error(this: *mut RtMidi, type_: RtMidiError_Type);
}
extern "C" {
    #[link_name = "\u{1}_ZN6RtMidiC2Ev"]
    pub fn RtMidi_RtMidi(this: *mut RtMidi);
}
impl RtMidi {
    #[inline]
    pub unsafe fn error(&mut self, type_: RtMidiError_Type) {
        RtMidi_error(self, type_)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        RtMidi_RtMidi(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct RtMidiIn {
    pub _base: RtMidi,
    pub inputData_: RtMidiIn_RtMidiInData,
}
#[doc = "! User callback function type definition."]
pub type RtMidiIn_RtMidiCallback =
    Option<unsafe extern "C" fn(timeStamp: f64, message: *mut vector, userData: *mut c_void)>;
#[repr(C)]
pub struct RtMidiIn_MidiMessage {
    pub bytes: vector,
    pub timeStamp: f64,
}
#[repr(C)]
pub struct RtMidiIn_RtMidiInData {
    pub queue: deque,
    pub queueLimit: c_uint,
    pub ignoreFlags: c_uchar,
    pub doInput: bool,
    pub firstMessage: bool,
    pub apiData: *mut c_void,
    pub usingCallback: bool,
    pub userCallback: *mut c_void,
    pub userData: *mut c_void,
}
extern "C" {
    #[doc = "! Set a callback function to be invoked for incoming MIDI messages."]
    #[doc = "*!"]
    #[doc = "The callback function will be called whenever an incoming MIDI"]
    #[doc = "message is received.  While not absolutely necessary, it is best"]
    #[doc = "to set the callback function before opening a MIDI port to avoid"]
    #[doc = "leaving some messages in the queue."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiIn11setCallbackEPFvdPSt6vectorIhSaIhEEPvES4_"]
    pub fn RtMidiIn_setCallback(
        this: *mut RtMidiIn,
        callback: RtMidiIn_RtMidiCallback,
        userData: *mut c_void,
    );
}
extern "C" {
    #[doc = "! Cancel use of the current callback function (if one exists)."]
    #[doc = "*!"]
    #[doc = "Subsequent incoming MIDI messages will be written to the queue"]
    #[doc = "and can be retrieved with the \\e getMessage function."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiIn14cancelCallbackEv"]
    pub fn RtMidiIn_cancelCallback(this: *mut RtMidiIn);
}
extern "C" {
    #[doc = "! Set the maximum number of MIDI messages to be saved in the queue."]
    #[doc = "*!"]
    #[doc = "If the queue size limit is reached, incoming messages will be"]
    #[doc = "ignored.  The default limit is 1024."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiIn17setQueueSizeLimitEj"]
    pub fn RtMidiIn_setQueueSizeLimit(this: *mut RtMidiIn, queueSize: c_uint);
}
extern "C" {
    #[doc = "! Specify whether certain MIDI message types should be queued or ignored during input."]
    #[doc = "*!"]
    #[doc = "By default, active sensing messages are ignored"]
    #[doc = "during message input because of their relative high data rates."]
    #[doc = "MIDI sysex messages are ignored by default as well.  Variable"]
    #[doc = "values of \"true\" imply that the respective message type will be"]
    #[doc = "ignored."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiIn11ignoreTypesEbbb"]
    pub fn RtMidiIn_ignoreTypes(
        this: *mut RtMidiIn,
        midiSysex: bool,
        midiTime: bool,
        midiSense: bool,
    );
}
extern "C" {
    #[doc = "! Fill the user-provided vector with the data bytes for the next available MIDI message in the input queue and return the event delta-time in seconds."]
    #[doc = "*!"]
    #[doc = "This function returns immediately whether a new message is"]
    #[doc = "available or not.  A valid message is indicated by a non-zero"]
    #[doc = "vector size.  An exception is thrown if an error occurs during"]
    #[doc = "message retrieval or an input connection was not previously"]
    #[doc = "established."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiIn10getMessageEPSt6vectorIhSaIhEE"]
    pub fn RtMidiIn_getMessage(this: *mut RtMidiIn, message: *mut vector) -> f64;
}
extern "C" {
    #[doc = "! Default constructor."]
    #[doc = "*!"]
    #[doc = "An exception will be thrown if a MIDI system initialization error occurs."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiInC1Ev"]
    pub fn RtMidiIn_RtMidiIn(this: *mut RtMidiIn);
}
impl RtMidiIn {
    #[inline]
    pub unsafe fn setCallback(&mut self, callback: RtMidiIn_RtMidiCallback, userData: *mut c_void) {
        RtMidiIn_setCallback(self, callback, userData)
    }
    #[inline]
    pub unsafe fn cancelCallback(&mut self) {
        RtMidiIn_cancelCallback(self)
    }
    #[inline]
    pub unsafe fn setQueueSizeLimit(&mut self, queueSize: c_uint) {
        RtMidiIn_setQueueSizeLimit(self, queueSize)
    }
    #[inline]
    pub unsafe fn ignoreTypes(&mut self, midiSysex: bool, midiTime: bool, midiSense: bool) {
        RtMidiIn_ignoreTypes(self, midiSysex, midiTime, midiSense)
    }
    #[inline]
    pub unsafe fn getMessage(&mut self, message: *mut vector) -> f64 {
        RtMidiIn_getMessage(self, message)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        RtMidiIn_RtMidiIn(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[doc = "! If a MIDI connection is still open, it will be closed by the destructor."]
    #[link_name = "\u{1}_ZN8RtMidiInD1Ev"]
    pub fn RtMidiIn_RtMidiIn_destructor(this: *mut RtMidiIn);
}
extern "C" {
    #[doc = "! Open a MIDI input connection."]
    #[doc = "*!"]
    #[doc = "An optional port number greater than 0 can be specified."]
    #[doc = "Otherwise, the default or first port found is opened."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiIn8openPortEj"]
    pub fn RtMidiIn_openPort(this: *mut c_void, portNumber: c_uint);
}
extern "C" {
    #[doc = "! Create a virtual input port to allow software connections (OS X and ALSA only)."]
    #[doc = "*!"]
    #[doc = "This function creates a virtual MIDI input port to which other"]
    #[doc = "software applications can connect.  This type of functionality"]
    #[doc = "is currently only supported by the Macintosh OS-X and Linux ALSA"]
    #[doc = "APIs (the function does nothing for the other APIs)."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiIn15openVirtualPortEv"]
    pub fn RtMidiIn_openVirtualPort(this: *mut c_void);
}
extern "C" {
    #[doc = "! Close an open MIDI connection (if one exists)."]
    #[link_name = "\u{1}_ZN8RtMidiIn9closePortEv"]
    pub fn RtMidiIn_closePort(this: *mut c_void);
}
extern "C" {
    #[doc = "! Return the number of available MIDI input ports."]
    #[link_name = "\u{1}_ZN8RtMidiIn12getPortCountEv"]
    pub fn RtMidiIn_getPortCount(this: *mut c_void) -> c_uint;
}
extern "C" {
    #[doc = "! Return a string identifier for the specified MIDI input port number."]
    #[doc = "*!"]
    #[doc = "An exception is thrown if an invalid port specifier is provided."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN8RtMidiIn11getPortNameB5cxx11Ej"]
    pub fn RtMidiIn_getPortName(this: *mut c_void, portNumber: c_uint) -> string;
}
#[doc = "\\brief A realtime MIDI output class."]
#[doc = ""]
#[doc = "This class provides a common, platform-independent API for MIDI"]
#[doc = "output.  It allows one to probe available MIDI output ports, to"]
#[doc = "connect to one such port, and to send MIDI bytes immediately over"]
#[doc = "the connection.  Create multiple instances of this class to"]
#[doc = "connect to more than one MIDI device at the same time."]
#[doc = ""]
#[doc = "by Gary P. Scavone, 2003-2004."]
#[repr(C)]
pub struct RtMidiOut {
    pub _base: RtMidi,
}
extern "C" {
    #[doc = "! Immediately send a single message out an open MIDI output port."]
    #[doc = "*!"]
    #[doc = "An exception is thrown if an error occurs during output or an"]
    #[doc = "output connection was not previously established."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN9RtMidiOut11sendMessageEPSt6vectorIhSaIhEE"]
    pub fn RtMidiOut_sendMessage(this: *mut RtMidiOut, message: *mut vector);
}
extern "C" {
    #[doc = "! Default constructor."]
    #[doc = "*!"]
    #[doc = "An exception will be thrown if a MIDI system initialization error occurs."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN9RtMidiOutC1Ev"]
    pub fn RtMidiOut_RtMidiOut(this: *mut RtMidiOut);
}
impl RtMidiOut {
    #[inline]
    pub unsafe fn sendMessage(&mut self, message: *mut vector) {
        RtMidiOut_sendMessage(self, message)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        RtMidiOut_RtMidiOut(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[doc = "! The destructor closes any open MIDI connections."]
    #[link_name = "\u{1}_ZN9RtMidiOutD1Ev"]
    pub fn RtMidiOut_RtMidiOut_destructor(this: *mut RtMidiOut);
}
extern "C" {
    #[doc = "! Open a MIDI output connection."]
    #[doc = "*!"]
    #[doc = "An optional port number greater than 0 can be specified."]
    #[doc = "Otherwise, the default or first port found is opened.  An"]
    #[doc = "exception is thrown if an error occurs while attempting to make"]
    #[doc = "the port connection."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN9RtMidiOut8openPortEj"]
    pub fn RtMidiOut_openPort(this: *mut c_void, portNumber: c_uint);
}
extern "C" {
    #[doc = "! Close an open MIDI connection (if one exists)."]
    #[link_name = "\u{1}_ZN9RtMidiOut9closePortEv"]
    pub fn RtMidiOut_closePort(this: *mut c_void);
}
extern "C" {
    #[doc = "! Create a virtual output port to allow software connections (OS X and ALSA only)."]
    #[doc = "*!"]
    #[doc = "This function creates a virtual MIDI output port to which other"]
    #[doc = "software applications can connect.  This type of functionality"]
    #[doc = "is currently only supported by the Macintosh OS-X and Linux ALSA"]
    #[doc = "APIs (the function does nothing with the other APIs).  An"]
    #[doc = "exception is thrown if an error occurs while attempting to create"]
    #[doc = "the virtual port."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN9RtMidiOut15openVirtualPortEv"]
    pub fn RtMidiOut_openVirtualPort(this: *mut c_void);
}
extern "C" {
    #[doc = "! Return the number of available MIDI output ports."]
    #[link_name = "\u{1}_ZN9RtMidiOut12getPortCountEv"]
    pub fn RtMidiOut_getPortCount(this: *mut c_void) -> c_uint;
}
extern "C" {
    #[doc = "! Return a string identifier for the specified MIDI port type and number."]
    #[doc = "*!"]
    #[doc = "An exception is thrown if an invalid port specifier is provided."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN9RtMidiOut11getPortNameB5cxx11Ej"]
    pub fn RtMidiOut_getPortName(this: *mut c_void, portNumber: c_uint) -> string;
}
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct _bindgen_ty_1 {
//     pub _address: u8,
// }
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
pub struct __locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_30 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_42 {
    pub _address: u8,
}
pub type char_type = c_char;
pub type int_type = c_int;
// pub type iterator = _Bit_iterator;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_44 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_45 {
    pub _address: u8,
}
