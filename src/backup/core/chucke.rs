#![allow(
    dead_code,
    improper_ctypes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    parenthesized_params_in_types_and_modules,
    unused_imports,
    unused_mut
)]
#![feature(libc)]
extern crate libc;
use crate::ck::carry::carrye::*;
use crate::ck::compile::compilee::*;
use crate::ck::dynl::dynle::*;
use crate::ck::hid::hide::*;
use crate::ck::midi::rtme::*;
use crate::ck::shell::shelle::*;
use crate::ck::ulib::mach::mache::*;
use crate::ck::util::math::utilmathe::*;
use crate::ck::util::string::stringe::*;
use crate::ck::vm::vme::*;
use crate::dts::*;
use libc::*;
///! master chuck module for easy embedding, thanks to 1.4.0.0 `numchucks` release!
fn main() {}
// #[repr(C)]
// pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
// impl<T> __BindgenUnionField<T> {
//     #[inline]
//     pub fn new() -> Self {
//         __BindgenUnionField(::std::marker::PhantomData)
//     }
//     #[inline]
//     pub unsafe fn as_ref(&self) -> &T {
//         transmute(self)
//     }
//     #[inline]
//     pub unsafe fn as_mut(&mut self) -> &mut T {
//         transmute(self)
//     }
// }
// impl<T> Default for __BindgenUnionField<T> {
//     #[inline]
//     fn default() -> Self {
//         Self::new()
//     }
// }
// impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
//     #[inline]
//     fn clone(&self) -> Self {
//         Self::new()
//     }
// }
// impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
// impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
//     fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//         fmt.write_str("__BindgenUnionField")
//     }
// }
// impl<T> Hash for __BindgenUnionField<T> {
//     fn hash<H: Hasher>(&self, _state: &mut H) {}
// }
// impl<T> PartialEq for __BindgenUnionField<T> {
//     fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
//         true
//     }
// }
// impl<T> Eq for __BindgenUnionField<T> {}

pub mod std {
    pub mod __cxx11 {
        pub type string = self::root::std::__cxx11::basic_string<c_char>;
        #[repr(C)]
        pub struct basic_string<_CharT> {
            pub _M_dataplus: self::root::std::__cxx11::basic_string__Alloc_hider,
            pub _M_string_length: self::root::std::__cxx11::basic_string_size_type,
            pub __bindgen_anon_1: self::root::std::__cxx11::basic_string__bindgen_ty_2<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type basic_string__Char_alloc_type = [u8; 0usize];
        pub type basic_string__Alloc_traits = self::root::__gnu_cxx::__alloc_traits;
        pub type basic_string_traits_type<_Traits> = _Traits;
        pub type basic_string_value_type = [u8; 0usize];
        pub type basic_string_allocator_type =
            self::root::std::__cxx11::basic_string__Char_alloc_type;
        pub type basic_string_size_type = [u8; 0usize];
        pub type basic_string_difference_type = [u8; 0usize];
        pub type basic_string_reference = [u8; 0usize];
        pub type basic_string_const_reference = [u8; 0usize];
        pub type basic_string_pointer = [u8; 0usize];
        pub type basic_string_const_pointer = [u8; 0usize];
        pub type basic_string_iterator = self::root::__gnu_cxx::__normal_iterator<
            self::root::std::__cxx11::basic_string_pointer,
        >;
        pub type basic_string_const_iterator = self::root::__gnu_cxx::__normal_iterator<
            self::root::std::__cxx11::basic_string_const_pointer,
        >;
        pub type basic_string_const_reverse_iterator = self::root::std::reverse_iterator<
            self::root::std::__cxx11::basic_string_const_iterator,
        >;
        pub type basic_string_reverse_iterator =
            self::root::std::reverse_iterator<self::root::std::__cxx11::basic_string_iterator>;
        pub type basic_string___const_iterator =
            self::root::std::__cxx11::basic_string_const_iterator;
        pub type basic_string___sv_type<_CharT> = self::root::std::basic_string_view<_CharT>;
        pub type basic_string__If_sv = self::root::std::enable_if_t;
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct basic_string___sv_wrapper<_CharT> {
            pub _M_sv: self::root::std::__cxx11::basic_string___sv_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        impl<_CharT> Default for basic_string___sv_wrapper<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        pub struct basic_string__Alloc_hider {
            pub _M_p: self::root::std::__cxx11::basic_string_pointer,
        }
        impl Default for basic_string__Alloc_hider {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl ::std::fmt::Debug for basic_string__Alloc_hider {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "basic_string__Alloc_hider {{  }}")
            }
        }
        impl PartialEq for basic_string__Alloc_hider {
            fn eq(&self, other: &basic_string__Alloc_hider) -> bool {
                self._M_p == other._M_p
            }
        }
        pub const basic_string__S_local_capacity:
            self::root::std::__cxx11::basic_string__bindgen_ty_1 = 0;
        pub type basic_string__bindgen_ty_1 = i32;
        #[repr(C)]
        pub struct basic_string__bindgen_ty_2<_CharT> {
            pub _M_local_buf: self::root::__BindgenUnionField<*mut _CharT>,
            pub _M_allocated_capacity:
                self::root::__BindgenUnionField<self::root::std::__cxx11::basic_string_size_type>,
            pub bindgen_union_field: u64,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        impl<_CharT> Default for basic_string__bindgen_ty_2<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_string__bindgen_ty_2<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "basic_string__bindgen_ty_2 {{ union }}")
            }
        }
        impl<_CharT> Default for basic_string<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_string<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "basic_string {{ _M_dataplus: {:?}, __bindgen_anon_1: {:?} }}",
                    self._M_dataplus, self.__bindgen_anon_1
                )
            }
        }
        #[repr(C)]
        pub struct basic_stringbuf<_CharT> {
            pub _base: self::root::std::basic_streambuf<_CharT>,
            pub _M_mode: self::root::std::ios_base_openmode,
            pub _M_string: self::root::std::__cxx11::basic_stringbuf___string_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type basic_stringbuf_char_type<_CharT> = _CharT;
        pub type basic_stringbuf_traits_type<_Traits> = _Traits;
        pub type basic_stringbuf_allocator_type<_Alloc> = _Alloc;
        pub type basic_stringbuf_int_type = [u8; 0usize];
        pub type basic_stringbuf_pos_type = [u8; 0usize];
        pub type basic_stringbuf_off_type = [u8; 0usize];
        pub type basic_stringbuf___streambuf_type<_CharT> = self::root::std::basic_streambuf<
            self::root::std::__cxx11::basic_stringbuf_char_type<_CharT>,
        >;
        pub type basic_stringbuf___string_type<_CharT> = self::root::std::__cxx11::basic_string<
            self::root::std::__cxx11::basic_stringbuf_char_type<_CharT>,
        >;
        pub type basic_stringbuf___size_type<_CharT> =
            self::root::std::__cxx11::basic_stringbuf___string_type<_CharT>;
        #[repr(C)]
        pub struct basic_stringbuf___xfer_bufptrs<_CharT> {
            pub _M_to: *mut self::root::std::__cxx11::basic_stringbuf<_CharT>,
            pub _M_goff: [self::root::std::__cxx11::basic_stringbuf_off_type; 3usize],
            pub _M_poff: [self::root::std::__cxx11::basic_stringbuf_off_type; 3usize],
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        impl<_CharT> Default for basic_stringbuf___xfer_bufptrs<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_stringbuf___xfer_bufptrs<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write ! ( f , "basic_stringbuf___xfer_bufptrs {{ _M_to: {:?}, _M_goff: {:?}, _M_poff: {:?} }}" , self . _M_to , self . _M_goff , self . _M_poff )
            }
        }
        impl<_CharT> Default for basic_stringbuf<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_stringbuf<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "basic_stringbuf {{ _M_mode: {:?}, _M_string: {:?} }}",
                    self._M_mode, self._M_string
                )
            }
        }
        #[repr(C)]
        pub struct basic_stringstream<_CharT> {
            pub _base: self::root::std::basic_iostream<_CharT>,
            pub _M_stringbuf: self::root::std::__cxx11::basic_stringstream___stringbuf_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type basic_stringstream_char_type<_CharT> = _CharT;
        pub type basic_stringstream_traits_type<_Traits> = _Traits;
        pub type basic_stringstream_allocator_type<_Alloc> = _Alloc;
        pub type basic_stringstream_int_type = [u8; 0usize];
        pub type basic_stringstream_pos_type = [u8; 0usize];
        pub type basic_stringstream_off_type = [u8; 0usize];
        pub type basic_stringstream___string_type<_CharT> =
            self::root::std::__cxx11::basic_string<_CharT>;
        pub type basic_stringstream___stringbuf_type<_CharT> =
            self::root::std::__cxx11::basic_stringbuf<_CharT>;
        pub type basic_stringstream___iostream_type<_CharT> = self::root::std::basic_iostream<
            self::root::std::__cxx11::basic_stringstream_char_type<_CharT>,
        >;
        impl<_CharT> Default for basic_stringstream<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_stringstream<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "basic_stringstream {{ _M_stringbuf: {:?} }}",
                    self._M_stringbuf
                )
            }
        }
        #[repr(C)]
        pub struct _List_base {
            pub _M_impl: self::root::std::__cxx11::_List_base__List_impl,
        }
        pub type _List_base__Tp_alloc_type = [u8; 0usize];
        pub type _List_base__Tp_alloc_traits = self::root::__gnu_cxx::__alloc_traits;
        pub type _List_base__Node_alloc_type = [u8; 0usize];
        pub type _List_base__Node_alloc_traits = self::root::__gnu_cxx::__alloc_traits;
        #[repr(C)]
        pub struct _List_base__List_impl {
            pub _M_node: self::root::std::__detail::_List_node_header,
        }
        impl Default for _List_base__List_impl {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl ::std::fmt::Debug for _List_base__List_impl {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "_List_base__List_impl {{ _M_node: {:?} }}", self._M_node)
            }
        }
        impl PartialEq for _List_base__List_impl {
            fn eq(&self, other: &_List_base__List_impl) -> bool {
                self._M_node == other._M_node
            }
        }
        pub type _List_base_allocator_type<_Alloc> = _Alloc;
        impl Default for _List_base {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl ::std::fmt::Debug for _List_base {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "_List_base {{ _M_impl: {:?} }}", self._M_impl)
            }
        }
        impl PartialEq for _List_base {
            fn eq(&self, other: &_List_base) -> bool {
                self._M_impl == other._M_impl
            }
        }
        #[repr(C)]
        pub struct list {
            pub _base: self::root::std::__cxx11::_List_base,
        }
        pub type list__Base = self::root::std::__cxx11::_List_base;
        pub type list__Tp_alloc_type = self::root::std::__cxx11::list__Base;
        pub type list__Tp_alloc_traits = self::root::std::__cxx11::list__Base;
        pub type list__Node_alloc_type = self::root::std::__cxx11::list__Base;
        pub type list__Node_alloc_traits = self::root::std::__cxx11::list__Base;
        pub type list_value_type<_Tp> = _Tp;
        pub type list_pointer = self::root::std::__cxx11::list__Tp_alloc_traits;
        pub type list_const_pointer = self::root::std::__cxx11::list__Tp_alloc_traits;
        pub type list_reference = self::root::std::__cxx11::list__Tp_alloc_traits;
        pub type list_const_reference = self::root::std::__cxx11::list__Tp_alloc_traits;
        pub type list_iterator = self::root::std::_List_iterator;
        pub type list_const_iterator = self::root::std::_List_const_iterator;
        pub type list_const_reverse_iterator =
            self::root::std::reverse_iterator<self::root::std::__cxx11::list_const_iterator>;
        pub type list_reverse_iterator =
            self::root::std::reverse_iterator<self::root::std::__cxx11::list_iterator>;
        pub type list_size_type = usize;
        pub type list_difference_type = isize;
        pub type list_allocator_type<_Alloc> = _Alloc;
        pub type list__Node = self::root::std::_List_node;
        impl Default for list {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl ::std::fmt::Debug for list {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "list {{  }}")
            }
        }
        impl PartialEq for list {
            fn eq(&self, other: &list) -> bool {
                self._base == other._base
            }
        }
    }
    pub type integral_constant_value_type<_Tp> = _Tp;
    pub type integral_constant_type = u8;
    pub type true_type = u8;
    pub type false_type = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __and_ {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_empty {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct remove_reference {
        pub _address: u8,
    }
    pub type remove_reference_type<_Tp> = _Tp;
    pub type remove_reference_t = self::root::std::remove_reference;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct make_unsigned {
        pub _address: u8,
    }
    pub type make_unsigned_type = u8;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union aligned_storage_type {
        pub __data: *mut c_uchar,
        pub __align: self::root::std::aligned_storage_type__bindgen_ty_1,
        _bindgen_union_align: u64,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct aligned_storage_type__bindgen_ty_1 {
        pub _address: u8,
    }
    // #[test]
    // fn bindgen_test_layout_aligned_storage_type() {
    //     assert_eq!(
    //         size_of::<aligned_storage_type>(),
    //         8usize,
    //         concat!("Size of: ", stringify!(aligned_storage_type))
    //     );
    //     assert_eq!(
    //         align_of::<aligned_storage_type>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(aligned_storage_type))
    //     );
    // }
    impl Default for aligned_storage_type {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for aligned_storage_type {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "aligned_storage_type {{ union }}")
        }
    }
    pub type enable_if_t = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __detector {
        pub _address: u8,
    }
    pub type __detector_value_t = self::root::std::false_type;
    pub type __detector_type<_Default> = _Default;
    pub type __detected_or = self::root::std::__detector;
    pub type __detected_or_t = self::root::std::__detected_or;
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __pair_base {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct pair<_T1, _T2> {
        pub first: _T1,
        pub second: _T2,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_T1>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_T2>>,
    }
    pub type pair_first_type<_T1> = _T1;
    pub type pair_second_type<_T2> = _T2;
    pub type pair__PCCP = u8;
    pub type pair__PCCFP = u8;
    impl<_T1, _T2> Default for pair<_T1, _T2> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct input_iterator_tag {
        pub _address: u8,
    }
    // #[test]
    // fn bindgen_test_layout_input_iterator_tag() {
    //     assert_eq!(
    //         size_of::<input_iterator_tag>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(input_iterator_tag))
    //     );
    //     assert_eq!(
    //         align_of::<input_iterator_tag>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(input_iterator_tag))
    //     );
    // }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct output_iterator_tag {
        pub _address: u8,
    }
    // #[test]
    // fn bindgen_test_layout_output_iterator_tag() {
    //     assert_eq!(
    //         size_of::<output_iterator_tag>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(output_iterator_tag))
    //     );
    //     assert_eq!(
    //         align_of::<output_iterator_tag>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(output_iterator_tag))
    //     );
    // }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct forward_iterator_tag {
        pub _address: u8,
    }
    // #[test]
    // fn bindgen_test_layout_forward_iterator_tag() {
    //     assert_eq!(
    //         size_of::<forward_iterator_tag>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(forward_iterator_tag))
    //     );
    //     assert_eq!(
    //         align_of::<forward_iterator_tag>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(forward_iterator_tag))
    //     );
    // }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bidirectional_iterator_tag {
        pub _address: u8,
    }
    // #[test]
    // fn bindgen_test_layout_bidirectional_iterator_tag() {
    //     assert_eq!(
    //         size_of::<bidirectional_iterator_tag>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(bidirectional_iterator_tag))
    //     );
    //     assert_eq!(
    //         align_of::<bidirectional_iterator_tag>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(bidirectional_iterator_tag))
    //     );
    // }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct random_access_iterator_tag {
        pub _address: u8,
    }
    // #[test]
    // fn bindgen_test_layout_random_access_iterator_tag() {
    //     assert_eq!(
    //         size_of::<random_access_iterator_tag>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(random_access_iterator_tag))
    //     );
    //     assert_eq!(
    //         align_of::<random_access_iterator_tag>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(random_access_iterator_tag))
    //     );
    // }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct iterator {
        pub _address: u8,
    }
    pub type iterator_iterator_category<_Category> = _Category;
    pub type iterator_value_type<_Tp> = _Tp;
    pub type iterator_difference_type<_Distance> = _Distance;
    pub type iterator_pointer<_Pointer> = _Pointer;
    pub type iterator_reference<_Reference> = _Reference;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __iterator_traits {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct iterator_traits {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __undefined {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __get_first_arg {
        pub _address: u8,
    }
    pub type __get_first_arg_type = self::root::std::__undefined;
    pub type __get_first_arg_t = self::root::std::__get_first_arg;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __replace_first_arg {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct pointer_traits {
        pub _address: u8,
    }
    pub type pointer_traits___element_type = [u8; 0usize];
    pub type pointer_traits___difference_type = [u8; 0usize];
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct pointer_traits___rebind {
        pub _address: u8,
    }
    impl Default for pointer_traits___rebind {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type pointer_traits_pointer<_Ptr> = _Ptr;
    pub type pointer_traits_element_type = self::root::std::__detected_or_t;
    pub type pointer_traits_difference_type = self::root::std::__detected_or_t;
    pub type pointer_traits_rebind = self::root::std::pointer_traits___rebind;
    pub type __ptr_rebind = self::root::std::pointer_traits;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct reverse_iterator<_Iterator> {
        pub current: _Iterator,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
    }
    pub type reverse_iterator___traits_type = self::root::std::iterator_traits;
    pub type reverse_iterator_iterator_type<_Iterator> = _Iterator;
    pub type reverse_iterator_difference_type = self::root::std::reverse_iterator___traits_type;
    pub type reverse_iterator_pointer = self::root::std::reverse_iterator___traits_type;
    pub type reverse_iterator_reference = self::root::std::reverse_iterator___traits_type;
    impl<_Iterator> Default for reverse_iterator<_Iterator> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type streamoff = c_long;
    pub type streamsize = isize;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct fpos<_StateT> {
        pub _M_off: self::root::std::streamoff,
        pub _M_state: _StateT,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_StateT>>,
    }
    impl<_StateT> Default for fpos<_StateT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type streampos = self::root::std::fpos<self::root::mbstate_t>;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct char_traits {
        pub _address: u8,
    }
    impl Default for char_traits {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    pub struct exception__bindgen_vtable(c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct exception {
        pub vtable_: *const exception__bindgen_vtable,
    }
    // #[test]
    // fn bindgen_test_layout_exception() {
    //     assert_eq!(
    //         size_of::<exception>(),
    //         8usize,
    //         concat!("Size of: ", stringify!(exception))
    //     );
    //     assert_eq!(
    //         align_of::<exception>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(exception))
    //     );
    // }
    impl Default for exception {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}exception_destructor"]
        pub fn exception_exception_destructor(this: *mut self::root::std::exception);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn exception_what(this: *mut c_void) -> *const c_char;
    }
    pub type __allocator_base = self::root::__gnu_cxx::new_allocator;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_rebind {
        pub _address: u8,
    }
    pub type allocator_rebind_other = self::root::std::allocator;
    pub type allocator_propagate_on_container_move_assignment = self::root::std::true_type;
    pub type allocator_is_always_equal = self::root::std::true_type;
    impl Default for allocator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type __c_locale = self::root::__locale_t;
    pub type stringstream = self::root::std::__cxx11::basic_stringstream<c_char>;
    pub type fstream = self::root::std::basic_fstream<c_char>;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct unary_function {
        pub _address: u8,
    }
    pub type unary_function_argument_type<_Arg> = _Arg;
    pub type unary_function_result_type<_Result> = _Result;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct binary_function {
        pub _address: u8,
    }
    pub type binary_function_first_argument_type<_Arg1> = _Arg1;
    pub type binary_function_second_argument_type<_Arg2> = _Arg2;
    pub type binary_function_result_type<_Result> = _Result;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct less {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Select1st {
        pub _address: u8,
    }
    impl Default for _Select1st {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __allocator_traits_base {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __allocator_traits_base___rebind {
        pub _address: u8,
    }
    impl Default for __allocator_traits_base___rebind {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type __allocator_traits_base___pointer = [u8; 0usize];
    pub type __allocator_traits_base___c_pointer = [u8; 0usize];
    pub type __allocator_traits_base___v_pointer = [u8; 0usize];
    pub type __allocator_traits_base___cv_pointer = [u8; 0usize];
    pub type __allocator_traits_base___pocca = [u8; 0usize];
    pub type __allocator_traits_base___pocma = [u8; 0usize];
    pub type __allocator_traits_base___pocs = [u8; 0usize];
    pub type __allocator_traits_base___equal = [u8; 0usize];
    // #[test]
    // fn bindgen_test_layout___allocator_traits_base() {
    //     assert_eq!(
    //         size_of::<__allocator_traits_base>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(__allocator_traits_base))
    //     );
    //     assert_eq!(
    //         align_of::<__allocator_traits_base>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(__allocator_traits_base))
    //     );
    // }
    pub type __alloc_rebind = self::root::std::__allocator_traits_base;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits {
        pub _address: u8,
    }
    pub type allocator_traits_allocator_type<_Alloc> = _Alloc;
    pub type allocator_traits_value_type = [u8; 0usize];
    pub type allocator_traits_pointer = self::root::std::__detected_or_t;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits__Ptr {
        pub _address: u8,
    }
    pub type allocator_traits__Ptr_type = [u8; 0usize];
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits__Diff {
        pub _address: u8,
    }
    pub type allocator_traits__Diff_type = self::root::std::pointer_traits;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits__Size {
        pub _address: u8,
    }
    impl Default for allocator_traits__Size {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type allocator_traits_const_pointer = [u8; 0usize];
    pub type allocator_traits_void_pointer = self::root::std::allocator_traits__Ptr;
    pub type allocator_traits_const_void_pointer = self::root::std::allocator_traits__Ptr;
    pub type allocator_traits_difference_type = [u8; 0usize];
    pub type allocator_traits_size_type = [u8; 0usize];
    pub type allocator_traits_propagate_on_container_copy_assignment =
        self::root::std::__detected_or_t;
    pub type allocator_traits_propagate_on_container_move_assignment =
        self::root::std::__detected_or_t;
    pub type allocator_traits_propagate_on_container_swap = self::root::std::__detected_or_t;
    pub type allocator_traits_is_always_equal = self::root::std::__detected_or_t;
    pub type allocator_traits_rebind_alloc = self::root::std::__alloc_rebind;
    pub type allocator_traits_rebind_traits = self::root::std::allocator_traits;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits___construct_helper {
        pub _address: u8,
    }
    pub type allocator_traits___construct_helper_type<_Alloc> = _Alloc;
    pub type allocator_traits___has_construct =
        self::root::std::allocator_traits___construct_helper;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_string_view<_CharT> {
        pub _M_len: usize,
        pub _M_str: *const _CharT,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_string_view_traits_type<_Traits> = _Traits;
    pub type basic_string_view_value_type<_CharT> = _CharT;
    pub type basic_string_view_pointer<_CharT> = *const _CharT;
    pub type basic_string_view_const_pointer<_CharT> = *const _CharT;
    pub type basic_string_view_reference<_CharT> = *const _CharT;
    pub type basic_string_view_const_reference<_CharT> = *const _CharT;
    pub type basic_string_view_const_iterator<_CharT> = *const _CharT;
    pub type basic_string_view_iterator<_CharT> =
        self::root::std::basic_string_view_const_iterator<_CharT>;
    pub type basic_string_view_const_reverse_iterator<_CharT> = self::root::std::reverse_iterator<
        self::root::std::basic_string_view_const_iterator<_CharT>,
    >;
    pub type basic_string_view_reverse_iterator<_CharT> =
        self::root::std::basic_string_view_const_reverse_iterator<_CharT>;
    pub type basic_string_view_size_type = usize;
    pub type basic_string_view_difference_type = isize;
    impl<_CharT> Default for basic_string_view<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub mod __detail {

        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _List_node_base {
            pub _M_next: *mut self::root::std::__detail::_List_node_base,
            pub _M_prev: *mut self::root::std::__detail::_List_node_base,
        }
        // #[test]
        // fn bindgen_test_layout__List_node_base() {
        //     assert_eq!(
        //         size_of::<_List_node_base>(),
        //         16usize,
        //         concat!("Size of: ", stringify!(_List_node_base))
        //     );
        //     assert_eq!(
        //         align_of::<_List_node_base>(),
        //         8usize,
        //         concat!("Alignment of ", stringify!(_List_node_base))
        //     );
        //     assert_eq!(
        //         unsafe {
        //             &(*(::std::ptr::null::<_List_node_base>()))._M_next as *const _ as usize
        //         },
        //         0usize,
        //         concat!(
        //             "Offset of field: ",
        //             stringify!(_List_node_base),
        //             "::",
        //             stringify!(_M_next)
        //         )
        //     );
        //     assert_eq!(
        //         unsafe {
        //             &(*(::std::ptr::null::<_List_node_base>()))._M_prev as *const _ as usize
        //         },
        //         8usize,
        //         concat!(
        //             "Offset of field: ",
        //             stringify!(_List_node_base),
        //             "::",
        //             stringify!(_M_prev)
        //         )
        //     );
        // }
        extern "C" {
            #[link_name = "\u{1}swap"]
            pub fn _List_node_base_swap(
                __x: *mut self::root::std::__detail::_List_node_base,
                __y: *mut self::root::std::__detail::_List_node_base,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_M_transfer"]
            pub fn _List_node_base__M_transfer(
                this: *mut self::root::std::__detail::_List_node_base,
                __first: *mut self::root::std::__detail::_List_node_base,
                __last: *mut self::root::std::__detail::_List_node_base,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_M_reverse"]
            pub fn _List_node_base__M_reverse(
                this: *mut self::root::std::__detail::_List_node_base,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_M_hook"]
            pub fn _List_node_base__M_hook(
                this: *mut self::root::std::__detail::_List_node_base,
                __position: *mut self::root::std::__detail::_List_node_base,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_M_unhook"]
            pub fn _List_node_base__M_unhook(this: *mut self::root::std::__detail::_List_node_base);
        }
        impl Default for _List_node_base {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl _List_node_base {
            #[inline]
            pub unsafe fn swap(
                __x: *mut self::root::std::__detail::_List_node_base,
                __y: *mut self::root::std::__detail::_List_node_base,
            ) {
                _List_node_base_swap(__x, __y)
            }
            #[inline]
            pub unsafe fn _M_transfer(
                &mut self,
                __first: *mut self::root::std::__detail::_List_node_base,
                __last: *mut self::root::std::__detail::_List_node_base,
            ) {
                _List_node_base__M_transfer(self, __first, __last)
            }
            #[inline]
            pub unsafe fn _M_reverse(&mut self) {
                _List_node_base__M_reverse(self)
            }
            #[inline]
            pub unsafe fn _M_hook(
                &mut self,
                __position: *mut self::root::std::__detail::_List_node_base,
            ) {
                _List_node_base__M_hook(self, __position)
            }
            #[inline]
            pub unsafe fn _M_unhook(&mut self) {
                _List_node_base__M_unhook(self)
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _List_node_header {
            pub _base: self::root::std::__detail::_List_node_base,
            pub _M_size: usize,
        }
        // #[test]
        // fn bindgen_test_layout__List_node_header() {
        //     assert_eq!(
        //         size_of::<_List_node_header>(),
        //         24usize,
        //         concat!("Size of: ", stringify!(_List_node_header))
        //     );
        //     assert_eq!(
        //         align_of::<_List_node_header>(),
        //         8usize,
        //         concat!("Alignment of ", stringify!(_List_node_header))
        //     );
        //     assert_eq!(
        //         unsafe {
        //             &(*(::std::ptr::null::<_List_node_header>()))._M_size as *const _ as usize
        //         },
        //         16usize,
        //         concat!(
        //             "Offset of field: ",
        //             stringify!(_List_node_header),
        //             "::",
        //             stringify!(_M_size)
        //         )
        //     );
        // }
        impl Default for _List_node_header {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
    }
    pub mod literals {}
    #[repr(C)]
    pub struct _Vector_base {
        pub _M_impl: self::root::std::_Vector_base__Vector_impl,
    }
    pub type _Vector_base__Tp_alloc_type = [u8; 0usize];
    pub type _Vector_base_pointer = [u8; 0usize];
    #[repr(C)]
    pub struct _Vector_base__Vector_impl {
        pub _M_start: self::root::std::_Vector_base_pointer,
        pub _M_finish: self::root::std::_Vector_base_pointer,
        pub _M_end_of_storage: self::root::std::_Vector_base_pointer,
    }
    impl Default for _Vector_base__Vector_impl {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Vector_base__Vector_impl {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Vector_base__Vector_impl {{  }}")
        }
    }
    impl PartialEq for _Vector_base__Vector_impl {
        fn eq(&self, other: &_Vector_base__Vector_impl) -> bool {
            self._M_start == other._M_start
                && self._M_finish == other._M_finish
                && self._M_end_of_storage == other._M_end_of_storage
        }
    }
    pub type _Vector_base_allocator_type<_Alloc> = _Alloc;
    impl Default for _Vector_base {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Vector_base {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Vector_base {{ _M_impl: {:?} }}", self._M_impl)
        }
    }
    impl PartialEq for _Vector_base {
        fn eq(&self, other: &_Vector_base) -> bool {
            self._M_impl == other._M_impl
        }
    }
    #[repr(C)]
    pub struct vector {
        pub _base: self::root::std::_Vector_base,
    }
    pub type vector__Base = self::root::std::_Vector_base;
    pub type vector__Tp_alloc_type = self::root::std::vector__Base;
    pub type vector__Alloc_traits = self::root::__gnu_cxx::__alloc_traits;
    pub type vector_value_type<_Tp> = _Tp;
    pub type vector_pointer = self::root::std::vector__Base;
    pub type vector_const_pointer = self::root::std::vector__Alloc_traits;
    pub type vector_reference = self::root::std::vector__Alloc_traits;
    pub type vector_const_reference = self::root::std::vector__Alloc_traits;
    pub type vector_iterator =
        self::root::__gnu_cxx::__normal_iterator<self::root::std::vector_pointer>;
    pub type vector_const_iterator =
        self::root::__gnu_cxx::__normal_iterator<self::root::std::vector_const_pointer>;
    pub type vector_const_reverse_iterator =
        self::root::std::reverse_iterator<self::root::std::vector_const_iterator>;
    pub type vector_reverse_iterator =
        self::root::std::reverse_iterator<self::root::std::vector_iterator>;
    pub type vector_size_type = usize;
    pub type vector_difference_type = isize;
    pub type vector_allocator_type<_Alloc> = _Alloc;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct vector__Temporary_value {
        pub _M_this: *mut self::root::std::vector,
        pub __buf: u8,
    }
    impl Default for vector__Temporary_value {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Default for vector {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for vector {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "vector {{  }}")
        }
    }
    impl PartialEq for vector {
        fn eq(&self, other: &vector) -> bool {
            self._base == other._base
        }
    }
    #[repr(C)]
    pub struct __cow_string {
        pub __bindgen_anon_1: self::root::std::__cow_string__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union __cow_string__bindgen_ty_1 {
        pub _M_p: *const c_char,
        pub _M_bytes: [c_char; 8usize],
        _bindgen_union_align: u64,
    }
    // #[test]
    // fn bindgen_test_layout___cow_string__bindgen_ty_1() {
    //     assert_eq!(
    //         size_of::<__cow_string__bindgen_ty_1>(),
    //         8usize,
    //         concat!("Size of: ", stringify!(__cow_string__bindgen_ty_1))
    //     );
    //     assert_eq!(
    //         align_of::<__cow_string__bindgen_ty_1>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(__cow_string__bindgen_ty_1))
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<__cow_string__bindgen_ty_1>()))._M_p as *const _ as usize
    //         },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(__cow_string__bindgen_ty_1),
    //             "::",
    //             stringify!(_M_p)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<__cow_string__bindgen_ty_1>()))._M_bytes as *const _
    //                 as usize
    //         },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(__cow_string__bindgen_ty_1),
    //             "::",
    //             stringify!(_M_bytes)
    //         )
    //     );
    // }
    impl Default for __cow_string__bindgen_ty_1 {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for __cow_string__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "__cow_string__bindgen_ty_1 {{ union }}")
        }
    }
    // #[test]
    // fn bindgen_test_layout___cow_string() {
    //     assert_eq!(
    //         size_of::<__cow_string>(),
    //         8usize,
    //         concat!("Size of: ", stringify!(__cow_string))
    //     );
    //     assert_eq!(
    //         align_of::<__cow_string>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(__cow_string))
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string(this: *mut self::root::std::__cow_string);
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string1(
            this: *mut self::root::std::__cow_string,
            arg1: *const self::root::std::__cxx11::string,
        );
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string2(
            this: *mut self::root::std::__cow_string,
            arg1: *const c_char,
            arg2: usize,
        );
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string3(
            this: *mut self::root::std::__cow_string,
            arg1: *const self::root::std::__cow_string,
        );
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string4(
            this: *mut self::root::std::__cow_string,
            arg1: *mut self::root::std::__cow_string,
        );
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string_destructor"]
        pub fn __cow_string___cow_string_destructor(this: *mut self::root::std::__cow_string);
    }
    impl Default for __cow_string {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for __cow_string {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "__cow_string {{ __bindgen_anon_1: {:?} }}",
                self.__bindgen_anon_1
            )
        }
    }
    impl __cow_string {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = uninitialized();
            __cow_string___cow_string(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const self::root::std::__cxx11::string) -> Self {
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
        pub unsafe fn new3(arg1: *const self::root::std::__cow_string) -> Self {
            let mut __bindgen_tmp = uninitialized();
            __cow_string___cow_string3(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new4(arg1: *mut self::root::std::__cow_string) -> Self {
            let mut __bindgen_tmp = uninitialized();
            __cow_string___cow_string4(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn destruct(&mut self) {
            __cow_string___cow_string_destructor(self)
        }
    }
    #[repr(C)]
    pub struct runtime_error {
        pub _base: self::root::std::exception,
        pub _M_msg: self::root::std::__cow_string,
    }
    // #[test]
    // fn bindgen_test_layout_runtime_error() {
    //     assert_eq!(
    //         size_of::<runtime_error>(),
    //         16usize,
    //         concat!("Size of: ", stringify!(runtime_error))
    //     );
    //     assert_eq!(
    //         align_of::<runtime_error>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(runtime_error))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<runtime_error>()))._M_msg as *const _ as usize },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(runtime_error),
    //             "::",
    //             stringify!(_M_msg)
    //         )
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}runtime_error"]
        pub fn runtime_error_runtime_error(
            this: *mut self::root::std::runtime_error,
            __arg: *const self::root::std::__cxx11::string,
        );
    }
    extern "C" {
        #[link_name = "\u{1}runtime_error"]
        pub fn runtime_error_runtime_error1(
            this: *mut self::root::std::runtime_error,
            arg1: *const c_char,
        );
    }
    extern "C" {
        #[link_name = "\u{1}runtime_error"]
        pub fn runtime_error_runtime_error2(
            this: *mut self::root::std::runtime_error,
            arg1: *const self::root::std::runtime_error,
        );
    }
    impl Default for runtime_error {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for runtime_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "runtime_error {{ _M_msg: {:?} }}", self._M_msg)
        }
    }
    impl runtime_error {
        #[inline]
        pub unsafe fn new(__arg: *const self::root::std::__cxx11::string) -> Self {
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
        pub unsafe fn new2(arg1: *const self::root::std::runtime_error) -> Self {
            let mut __bindgen_tmp = uninitialized();
            runtime_error_runtime_error2(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}runtime_error_destructor"]
        pub fn runtime_error_runtime_error_destructor(this: *mut self::root::std::runtime_error);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn runtime_error_what(this: *mut c_void) -> *const c_char;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct optional {
        pub _address: u8,
    }
    pub type optional__Base = u8;
    pub type optional_value_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Node_handle_common {
        pub _M_ptr: self::root::std::_Node_handle_common__AllocTraits,
        pub _M_alloc: self::root::std::optional,
    }
    pub type _Node_handle_common__AllocTraits = self::root::std::allocator_traits;
    pub type _Node_handle_common_allocator_type = self::root::std::__alloc_rebind;
    impl Default for _Node_handle_common {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Node_handle {
        pub _base: self::root::std::_Node_handle_common,
        pub _M_pkey: self::root::std::_Node_handle___pointer,
        pub _M_pmapped: self::root::std::_Node_handle___pointer,
    }
    pub type _Node_handle_key_type<_Key> = _Key;
    pub type _Node_handle_mapped_type = [u8; 0usize];
    pub type _Node_handle__AllocTraits = self::root::std::allocator_traits;
    pub type _Node_handle___pointer = self::root::std::__ptr_rebind;
    impl Default for _Node_handle {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Node_insert_return<_Iterator, _NodeHandle> {
        pub position: _Iterator,
        pub inserted: bool,
        pub node: _NodeHandle,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_NodeHandle>>,
    }
    impl<_Iterator, _NodeHandle> Default for _Node_insert_return<_Iterator, _NodeHandle> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub const _Rb_tree_color__S_red: self::root::std::_Rb_tree_color = 0;
    pub const _Rb_tree_color__S_black: self::root::std::_Rb_tree_color = 1;
    pub type _Rb_tree_color = u32;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_node_base {
        pub _M_color: self::root::std::_Rb_tree_color,
        pub _M_parent: self::root::std::_Rb_tree_node_base__Base_ptr,
        pub _M_left: self::root::std::_Rb_tree_node_base__Base_ptr,
        pub _M_right: self::root::std::_Rb_tree_node_base__Base_ptr,
    }
    pub type _Rb_tree_node_base__Base_ptr = *mut self::root::std::_Rb_tree_node_base;
    pub type _Rb_tree_node_base__Const_Base_ptr = *const self::root::std::_Rb_tree_node_base;
    // #[test]
    // fn bindgen_test_layout__Rb_tree_node_base() {
    //     assert_eq!(
    //         size_of::<_Rb_tree_node_base>(),
    //         32usize,
    //         concat!("Size of: ", stringify!(_Rb_tree_node_base))
    //     );
    //     assert_eq!(
    //         align_of::<_Rb_tree_node_base>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(_Rb_tree_node_base))
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<_Rb_tree_node_base>()))._M_color as *const _ as usize
    //         },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(_Rb_tree_node_base),
    //             "::",
    //             stringify!(_M_color)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<_Rb_tree_node_base>()))._M_parent as *const _ as usize
    //         },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(_Rb_tree_node_base),
    //             "::",
    //             stringify!(_M_parent)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<_Rb_tree_node_base>()))._M_left as *const _ as usize
    //         },
    //         16usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(_Rb_tree_node_base),
    //             "::",
    //             stringify!(_M_left)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<_Rb_tree_node_base>()))._M_right as *const _ as usize
    //         },
    //         24usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(_Rb_tree_node_base),
    //             "::",
    //             stringify!(_M_right)
    //         )
    //     );
    // }
    impl Default for _Rb_tree_node_base {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_key_compare<_Key_compare> {
        pub _M_key_compare: _Key_compare,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Key_compare>>,
    }
    impl<_Key_compare> Default for _Rb_tree_key_compare<_Key_compare> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_node {
        pub _base: self::root::std::_Rb_tree_node_base,
        pub _M_storage: self::root::__gnu_cxx::__aligned_membuf,
    }
    pub type _Rb_tree_node__Link_type = *mut self::root::std::_Rb_tree_node;
    impl Default for _Rb_tree_node {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_iterator {
        pub _M_node: self::root::std::_Rb_tree_iterator__Base_ptr,
    }
    pub type _Rb_tree_iterator_value_type<_Tp> = _Tp;
    pub type _Rb_tree_iterator_reference<_Tp> = *mut _Tp;
    pub type _Rb_tree_iterator_pointer<_Tp> = *mut _Tp;
    pub type _Rb_tree_iterator_iterator_category = self::root::std::bidirectional_iterator_tag;
    pub type _Rb_tree_iterator_difference_type = isize;
    pub type _Rb_tree_iterator__Self = self::root::std::_Rb_tree_iterator;
    pub type _Rb_tree_iterator__Base_ptr = self::root::std::_Rb_tree_node_base__Base_ptr;
    pub type _Rb_tree_iterator__Link_type = *mut self::root::std::_Rb_tree_node;
    impl Default for _Rb_tree_iterator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_const_iterator {
        pub _M_node: self::root::std::_Rb_tree_const_iterator__Base_ptr,
    }
    pub type _Rb_tree_const_iterator_value_type<_Tp> = _Tp;
    pub type _Rb_tree_const_iterator_reference<_Tp> = *const _Tp;
    pub type _Rb_tree_const_iterator_pointer<_Tp> = *const _Tp;
    pub type _Rb_tree_const_iterator_iterator = self::root::std::_Rb_tree_iterator;
    pub type _Rb_tree_const_iterator_iterator_category =
        self::root::std::bidirectional_iterator_tag;
    pub type _Rb_tree_const_iterator_difference_type = isize;
    pub type _Rb_tree_const_iterator__Self = self::root::std::_Rb_tree_const_iterator;
    pub type _Rb_tree_const_iterator__Base_ptr =
        self::root::std::_Rb_tree_node_base__Const_Base_ptr;
    pub type _Rb_tree_const_iterator__Link_type = *const self::root::std::_Rb_tree_node;
    impl Default for _Rb_tree_const_iterator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree {
        pub _M_impl: u8,
    }
    pub type _Rb_tree__Node_allocator = [u8; 0usize];
    pub type _Rb_tree__Alloc_traits = self::root::__gnu_cxx::__alloc_traits;
    pub type _Rb_tree__Base_ptr = *mut self::root::std::_Rb_tree_node_base;
    pub type _Rb_tree__Const_Base_ptr = *const self::root::std::_Rb_tree_node_base;
    pub type _Rb_tree__Link_type = *mut self::root::std::_Rb_tree_node;
    pub type _Rb_tree__Const_Link_type = *const self::root::std::_Rb_tree_node;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree__Reuse_or_alloc_node {
        pub _M_root: self::root::std::_Rb_tree__Base_ptr,
        pub _M_nodes: self::root::std::_Rb_tree__Base_ptr,
        pub _M_t: *mut self::root::std::_Rb_tree,
    }
    impl Default for _Rb_tree__Reuse_or_alloc_node {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree__Alloc_node {
        pub _M_t: *mut self::root::std::_Rb_tree,
    }
    impl Default for _Rb_tree__Alloc_node {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type _Rb_tree_key_type<_Key> = _Key;
    pub type _Rb_tree_value_type<_Val> = _Val;
    pub type _Rb_tree_pointer<_Val> = *mut self::root::std::_Rb_tree_value_type<_Val>;
    pub type _Rb_tree_const_pointer<_Val> = *const self::root::std::_Rb_tree_value_type<_Val>;
    pub type _Rb_tree_reference<_Val> = *mut self::root::std::_Rb_tree_value_type<_Val>;
    pub type _Rb_tree_const_reference<_Val> = *const self::root::std::_Rb_tree_value_type<_Val>;
    pub type _Rb_tree_size_type = usize;
    pub type _Rb_tree_difference_type = isize;
    pub type _Rb_tree_allocator_type<_Alloc> = _Alloc;
    pub type _Rb_tree_reverse_iterator =
        self::root::std::reverse_iterator<self::root::std::_Rb_tree_iterator>;
    pub type _Rb_tree_const_reverse_iterator =
        self::root::std::reverse_iterator<self::root::std::_Rb_tree_const_iterator>;
    pub type _Rb_tree_node_type = self::root::std::_Node_handle;
    pub type _Rb_tree_insert_return_type =
        self::root::std::_Node_insert_return<_Iterator, _NodeHandle>;
    pub type _Rb_tree__Compatible_tree = self::root::std::_Rb_tree;
    pub type _Rb_tree__Rb_tree_impl__Base_key_compare<_Key_compare> =
        self::root::std::_Rb_tree_key_compare<_Key_compare>;
    #[repr(C)]
    pub struct map {
        pub _M_t: self::root::std::map__Rep_type,
    }
    pub type map_key_type<_Key> = _Key;
    pub type map_mapped_type<_Tp> = _Tp;
    pub type map_value_type<_Key, _Tp> = self::root::std::pair<_Key, _Tp>;
    pub type map_key_compare<_Compare> = _Compare;
    pub type map_allocator_type<_Alloc> = _Alloc;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct map_value_compare<_Compare> {
        pub comp: _Compare,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Compare>>,
    }
    impl<_Compare> Default for map_value_compare<_Compare> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type map__Pair_alloc_type = [u8; 0usize];
    pub type map__Rep_type = self::root::std::_Rb_tree;
    pub type map__Alloc_traits = self::root::__gnu_cxx::__alloc_traits;
    pub type map_pointer = [u8; 0usize];
    pub type map_const_pointer = [u8; 0usize];
    pub type map_reference = [u8; 0usize];
    pub type map_const_reference = [u8; 0usize];
    pub type map_iterator = [u8; 0usize];
    pub type map_const_iterator = [u8; 0usize];
    pub type map_size_type = [u8; 0usize];
    pub type map_difference_type = [u8; 0usize];
    pub type map_reverse_iterator = [u8; 0usize];
    pub type map_const_reverse_iterator = [u8; 0usize];
    pub type map_node_type = [u8; 0usize];
    pub type map_insert_return_type = [u8; 0usize];
    impl Default for map {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for map {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "map {{ _M_t: {:?} }}", self._M_t)
        }
    }
    impl PartialEq for map {
        fn eq(&self, other: &map) -> bool {
            self._M_t == other._M_t
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Deque_iterator {
        pub _M_cur: self::root::std::_Deque_iterator__Elt_pointer,
        pub _M_first: self::root::std::_Deque_iterator__Elt_pointer,
        pub _M_last: self::root::std::_Deque_iterator__Elt_pointer,
        pub _M_node: self::root::std::_Deque_iterator__Map_pointer,
    }
    pub type _Deque_iterator___ptr_to = self::root::std::pointer_traits;
    pub type _Deque_iterator___iter = self::root::std::_Deque_iterator;
    pub type _Deque_iterator_iterator = self::root::std::_Deque_iterator___iter;
    pub type _Deque_iterator_const_iterator = self::root::std::_Deque_iterator___iter;
    pub type _Deque_iterator__Elt_pointer = self::root::std::_Deque_iterator___ptr_to;
    pub type _Deque_iterator__Map_pointer = self::root::std::_Deque_iterator___ptr_to;
    pub type _Deque_iterator_iterator_category = self::root::std::random_access_iterator_tag;
    pub type _Deque_iterator_value_type<_Tp> = _Tp;
    pub type _Deque_iterator_pointer<_Ptr> = _Ptr;
    pub type _Deque_iterator_reference<_Ref> = _Ref;
    pub type _Deque_iterator_size_type = usize;
    pub type _Deque_iterator_difference_type = isize;
    pub type _Deque_iterator__Self = self::root::std::_Deque_iterator;
    impl Default for _Deque_iterator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    pub struct _Deque_base {
        pub _M_impl: self::root::std::_Deque_base__Deque_impl,
    }
    pub type _Deque_base__Tp_alloc_type = [u8; 0usize];
    pub type _Deque_base__Alloc_traits = self::root::__gnu_cxx::__alloc_traits;
    pub type _Deque_base__Ptr = [u8; 0usize];
    pub type _Deque_base__Ptr_const = [u8; 0usize];
    pub type _Deque_base__Map_alloc_type = [u8; 0usize];
    pub type _Deque_base__Map_alloc_traits = self::root::__gnu_cxx::__alloc_traits;
    pub type _Deque_base_allocator_type<_Alloc> = _Alloc;
    pub type _Deque_base_size_type = [u8; 0usize];
    pub type _Deque_base_iterator = self::root::std::_Deque_iterator;
    pub type _Deque_base_const_iterator = self::root::std::_Deque_iterator;
    pub type _Deque_base__Map_pointer = [u8; 0usize];
    #[repr(C)]
    pub struct _Deque_base__Deque_impl {
        pub _M_map: self::root::std::_Deque_base__Map_pointer,
        pub _M_map_size: usize,
        pub _M_start: self::root::std::_Deque_base_iterator,
        pub _M_finish: self::root::std::_Deque_base_iterator,
    }
    impl Default for _Deque_base__Deque_impl {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Deque_base__Deque_impl {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "_Deque_base__Deque_impl {{ _M_map_size: {:?}, _M_start: {:?}, _M_finish: {:?} }}",
                self._M_map_size, self._M_start, self._M_finish
            )
        }
    }
    impl PartialEq for _Deque_base__Deque_impl {
        fn eq(&self, other: &_Deque_base__Deque_impl) -> bool {
            self._M_map == other._M_map
                && self._M_map_size == other._M_map_size
                && self._M_start == other._M_start
                && self._M_finish == other._M_finish
        }
    }
    pub const _Deque_base__S_initial_map_size: self::root::std::_Deque_base__bindgen_ty_1 = 0;
    pub type _Deque_base__bindgen_ty_1 = i32;
    impl Default for _Deque_base {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Deque_base {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Deque_base {{ _M_impl: {:?} }}", self._M_impl)
        }
    }
    impl PartialEq for _Deque_base {
        fn eq(&self, other: &_Deque_base) -> bool {
            self._M_impl == other._M_impl
        }
    }
    #[repr(C)]
    pub struct deque {
        pub _base: self::root::std::_Deque_base,
    }
    pub type deque__Base = self::root::std::_Deque_base;
    pub type deque__Tp_alloc_type = self::root::std::deque__Base;
    pub type deque__Alloc_traits = self::root::std::deque__Base;
    pub type deque__Map_pointer = self::root::std::deque__Base;
    pub type deque_value_type<_Tp> = _Tp;
    pub type deque_pointer = self::root::std::deque__Alloc_traits;
    pub type deque_const_pointer = self::root::std::deque__Alloc_traits;
    pub type deque_reference = self::root::std::deque__Alloc_traits;
    pub type deque_const_reference = self::root::std::deque__Alloc_traits;
    pub type deque_iterator = self::root::std::deque__Base;
    pub type deque_const_iterator = self::root::std::deque__Base;
    pub type deque_const_reverse_iterator =
        self::root::std::reverse_iterator<self::root::std::deque_const_iterator>;
    pub type deque_reverse_iterator =
        self::root::std::reverse_iterator<self::root::std::deque_iterator>;
    pub type deque_size_type = usize;
    pub type deque_difference_type = isize;
    pub type deque_allocator_type<_Alloc> = _Alloc;
    impl Default for deque {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for deque {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "deque {{  }}")
        }
    }
    impl PartialEq for deque {
        fn eq(&self, other: &deque) -> bool {
            self._base == other._base
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct queue<_Sequence> {
        pub c: _Sequence,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Sequence>>,
    }
    pub type queue__Uses = u8;
    pub type queue_value_type = [u8; 0usize];
    pub type queue_reference = [u8; 0usize];
    pub type queue_const_reference = [u8; 0usize];
    pub type queue_size_type = [u8; 0usize];
    pub type queue_container_type<_Sequence> = _Sequence;
    impl<_Sequence> Default for queue<_Sequence> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct locale {
        pub _M_impl: *mut self::root::std::locale__Impl,
    }
    pub type locale_category = c_int;
    pub const locale__S_categories_size: self::root::std::locale__bindgen_ty_1 = 12;
    pub type locale__bindgen_ty_1 = u32;
    pub const locale_none: self::root::std::locale_category = 0;
    pub const locale_ctype: self::root::std::locale_category = 1;
    pub const locale_numeric: self::root::std::locale_category = 2;
    pub const locale_collate: self::root::std::locale_category = 4;
    pub const locale_time: self::root::std::locale_category = 8;
    pub const locale_monetary: self::root::std::locale_category = 16;
    pub const locale_messages: self::root::std::locale_category = 32;
    pub const locale_all: self::root::std::locale_category = 63;
    extern "C" {
        #[link_name = "\u{1}_S_classic"]
        pub static mut locale__S_classic: *mut self::root::std::locale__Impl;
    }
    extern "C" {
        #[link_name = "\u{1}_S_global"]
        pub static mut locale__S_global: *mut self::root::std::locale__Impl;
    }
    extern "C" {
        #[link_name = "\u{1}_S_categories"]
        pub static locale__S_categories: *const *const c_char;
    }
    extern "C" {
        #[link_name = "\u{1}_S_once"]
        pub static mut locale__S_once: self::root::__gthread_once_t;
    }
    extern "C" {
        #[link_name = "\u{1}_S_twinned_facets"]
        pub static mut locale__S_twinned_facets: [*const self::root::std::locale_id; 0usize];
    }
    // #[test]
    // fn bindgen_test_layout_locale() {
    //     assert_eq!(
    //         size_of::<locale>(),
    //         8usize,
    //         concat!("Size of: ", stringify!(locale))
    //     );
    //     assert_eq!(
    //         align_of::<locale>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(locale))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<locale>()))._M_impl as *const _ as usize },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(locale),
    //             "::",
    //             stringify!(_M_impl)
    //         )
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}name"]
        pub fn locale_name(
            this: *const self::root::std::locale,
        ) -> self::root::std::__cxx11::string;
    }
    extern "C" {
        #[link_name = "\u{1}global"]
        pub fn locale_global(__loc: *const self::root::std::locale) -> self::root::std::locale;
    }
    extern "C" {
        #[link_name = "\u{1}classic"]
        pub fn locale_classic() -> *const self::root::std::locale;
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale(this: *mut self::root::std::locale);
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale1(
            this: *mut self::root::std::locale,
            __other: *const self::root::std::locale,
        );
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale2(this: *mut self::root::std::locale, __s: *const c_char);
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale3(
            this: *mut self::root::std::locale,
            __base: *const self::root::std::locale,
            __s: *const c_char,
            __cat: self::root::std::locale_category,
        );
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale4(
            this: *mut self::root::std::locale,
            __base: *const self::root::std::locale,
            __add: *const self::root::std::locale,
            __cat: self::root::std::locale_category,
        );
    }
    extern "C" {
        #[link_name = "\u{1}locale_destructor"]
        pub fn locale_locale_destructor(this: *mut self::root::std::locale);
    }
    impl Default for locale {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl locale {
        #[inline]
        pub unsafe fn name(&self) -> self::root::std::__cxx11::string {
            locale_name(self)
        }
        #[inline]
        pub unsafe fn global(__loc: *const self::root::std::locale) -> self::root::std::locale {
            locale_global(__loc)
        }
        #[inline]
        pub unsafe fn classic() -> *const self::root::std::locale {
            locale_classic()
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = uninitialized();
            locale_locale(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(__other: *const self::root::std::locale) -> Self {
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
        pub unsafe fn new3(
            __base: *const self::root::std::locale,
            __s: *const c_char,
            __cat: self::root::std::locale_category,
        ) -> Self {
            let mut __bindgen_tmp = uninitialized();
            locale_locale3(&mut __bindgen_tmp, __base, __s, __cat);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new4(
            __base: *const self::root::std::locale,
            __add: *const self::root::std::locale,
            __cat: self::root::std::locale_category,
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
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct locale_facet {
        pub vtable_: *const locale_facet__bindgen_vtable,
        pub _M_refcount: self::root::_Atomic_word,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct locale_facet___shim {
        _unused: [u8; 0],
    }
    extern "C" {
        #[link_name = "\u{1}_S_c_locale"]
        pub static mut locale_facet__S_c_locale: self::root::std::__c_locale;
    }
    extern "C" {
        #[link_name = "\u{1}_S_c_name"]
        pub static mut locale_facet__S_c_name: [c_char; 2usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_once"]
        pub static mut locale_facet__S_once: self::root::__gthread_once_t;
    }
    // #[test]
    // fn bindgen_test_layout_locale_facet() {
    //     assert_eq!(
    //         size_of::<locale_facet>(),
    //         16usize,
    //         concat!("Size of: ", stringify!(locale_facet))
    //     );
    //     assert_eq!(
    //         align_of::<locale_facet>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(locale_facet))
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<locale_facet>()))._M_refcount as *const _ as usize
    //         },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(locale_facet),
    //             "::",
    //             stringify!(_M_refcount)
    //         )
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}_S_create_c_locale"]
        pub fn locale_facet__S_create_c_locale(
            __cloc: *mut self::root::std::__c_locale,
            __s: *const c_char,
            __old: self::root::std::__c_locale,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_S_clone_c_locale"]
        pub fn locale_facet__S_clone_c_locale(
            __cloc: *mut self::root::std::__c_locale,
        ) -> self::root::std::__c_locale;
    }
    extern "C" {
        #[link_name = "\u{1}_S_destroy_c_locale"]
        pub fn locale_facet__S_destroy_c_locale(__cloc: *mut self::root::std::__c_locale);
    }
    extern "C" {
        #[link_name = "\u{1}_S_lc_ctype_c_locale"]
        pub fn locale_facet__S_lc_ctype_c_locale(
            __cloc: self::root::std::__c_locale,
            __s: *const c_char,
        ) -> self::root::std::__c_locale;
    }
    extern "C" {
        #[link_name = "\u{1}_S_get_c_locale"]
        pub fn locale_facet__S_get_c_locale() -> self::root::std::__c_locale;
    }
    extern "C" {
        #[link_name = "\u{1}_S_get_c_name"]
        pub fn locale_facet__S_get_c_name() -> *const c_char;
    }
    impl Default for locale_facet {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl locale_facet {
        #[inline]
        pub unsafe fn _S_create_c_locale(
            __cloc: *mut self::root::std::__c_locale,
            __s: *const c_char,
            __old: self::root::std::__c_locale,
        ) {
            locale_facet__S_create_c_locale(__cloc, __s, __old)
        }
        #[inline]
        pub unsafe fn _S_clone_c_locale(
            __cloc: *mut self::root::std::__c_locale,
        ) -> self::root::std::__c_locale {
            locale_facet__S_clone_c_locale(__cloc)
        }
        #[inline]
        pub unsafe fn _S_destroy_c_locale(__cloc: *mut self::root::std::__c_locale) {
            locale_facet__S_destroy_c_locale(__cloc)
        }
        #[inline]
        pub unsafe fn _S_lc_ctype_c_locale(
            __cloc: self::root::std::__c_locale,
            __s: *const c_char,
        ) -> self::root::std::__c_locale {
            locale_facet__S_lc_ctype_c_locale(__cloc, __s)
        }
        #[inline]
        pub unsafe fn _S_get_c_locale() -> self::root::std::__c_locale {
            locale_facet__S_get_c_locale()
        }
        #[inline]
        pub unsafe fn _S_get_c_name() -> *const c_char {
            locale_facet__S_get_c_name()
        }
    }
    extern "C" {
        #[link_name = "\u{1}facet_destructor"]
        pub fn locale_facet_facet_destructor(this: *mut self::root::std::locale_facet);
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct locale_id {
        pub _M_index: usize,
    }
    extern "C" {
        #[link_name = "\u{1}_S_refcount"]
        pub static mut locale_id__S_refcount: self::root::_Atomic_word;
    }
    // #[test]
    // fn bindgen_test_layout_locale_id() {
    //     assert_eq!(
    //         size_of::<locale_id>(),
    //         8usize,
    //         concat!("Size of: ", stringify!(locale_id))
    //     );
    //     assert_eq!(
    //         align_of::<locale_id>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(locale_id))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<locale_id>()))._M_index as *const _ as usize },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(locale_id),
    //             "::",
    //             stringify!(_M_index)
    //         )
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}_M_id"]
        pub fn locale_id__M_id(this: *const self::root::std::locale_id) -> usize;
    }
    impl locale_id {
        #[inline]
        pub unsafe fn _M_id(&self) -> usize {
            locale_id__M_id(self)
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct locale__Impl {
        pub _M_refcount: self::root::_Atomic_word,
        pub _M_facets: *mut *const self::root::std::locale_facet,
        pub _M_facets_size: usize,
        pub _M_caches: *mut *const self::root::std::locale_facet,
        pub _M_names: *mut *mut c_char,
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_ctype"]
        pub static mut locale__Impl__S_id_ctype: [*const self::root::std::locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_numeric"]
        pub static mut locale__Impl__S_id_numeric: [*const self::root::std::locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_collate"]
        pub static mut locale__Impl__S_id_collate: [*const self::root::std::locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_time"]
        pub static mut locale__Impl__S_id_time: [*const self::root::std::locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_monetary"]
        pub static mut locale__Impl__S_id_monetary: [*const self::root::std::locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_messages"]
        pub static mut locale__Impl__S_id_messages: [*const self::root::std::locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_facet_categories"]
        pub static mut locale__Impl__S_facet_categories:
            [*const *const self::root::std::locale_id; 0usize];
    }
    // #[test]
    // fn bindgen_test_layout_locale__Impl() {
    //     assert_eq!(
    //         size_of::<locale__Impl>(),
    //         40usize,
    //         concat!("Size of: ", stringify!(locale__Impl))
    //     );
    //     assert_eq!(
    //         align_of::<locale__Impl>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(locale__Impl))
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<locale__Impl>()))._M_refcount as *const _ as usize
    //         },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(locale__Impl),
    //             "::",
    //             stringify!(_M_refcount)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<locale__Impl>()))._M_facets as *const _ as usize },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(locale__Impl),
    //             "::",
    //             stringify!(_M_facets)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<locale__Impl>()))._M_facets_size as *const _ as usize
    //         },
    //         16usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(locale__Impl),
    //             "::",
    //             stringify!(_M_facets_size)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<locale__Impl>()))._M_caches as *const _ as usize },
    //         24usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(locale__Impl),
    //             "::",
    //             stringify!(_M_caches)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<locale__Impl>()))._M_names as *const _ as usize },
    //         32usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(locale__Impl),
    //             "::",
    //             stringify!(_M_names)
    //         )
    //     );
    // }
    impl Default for locale__Impl {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub mod _V2 {

        #[repr(C)]
        pub struct error_category__bindgen_vtable(c_void);
        #[repr(C)]
        #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct error_category {
            pub vtable_: *const error_category__bindgen_vtable,
        }
        // #[test]
        // fn bindgen_test_layout_error_category() {
        //     assert_eq!(
        //         size_of::<error_category>(),
        //         8usize,
        //         concat!("Size of: ", stringify!(error_category))
        //     );
        //     assert_eq!(
        //         align_of::<error_category>(),
        //         8usize,
        //         concat!("Alignment of ", stringify!(error_category))
        //     );
        // }
        impl Default for error_category {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        extern "C" {
            #[link_name = "\u{1}error_category_destructor"]
            pub fn error_category_error_category_destructor(
                this: *mut self::root::std::_V2::error_category,
            );
        }
        extern "C" {
            #[link_name = "\u{1}default_error_condition"]
            pub fn error_category_default_error_condition(
                this: *mut c_void,
                __i: c_int,
            ) -> self::root::std::error_condition;
        }
        extern "C" {
            #[link_name = "\u{1}equivalent"]
            pub fn error_category_equivalent(
                this: *mut c_void,
                __i: c_int,
                __cond: *const self::root::std::error_condition,
            ) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}equivalent"]
            pub fn error_category_equivalent1(
                this: *mut c_void,
                __code: *const self::root::std::error_code,
                __i: c_int,
            ) -> bool;
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct error_code {
        pub _M_value: c_int,
        pub _M_cat: *const self::root::std::_V2::error_category,
    }
    // #[test]
    // fn bindgen_test_layout_error_code() {
    //     assert_eq!(
    //         size_of::<error_code>(),
    //         16usize,
    //         concat!("Size of: ", stringify!(error_code))
    //     );
    //     assert_eq!(
    //         align_of::<error_code>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(error_code))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<error_code>()))._M_value as *const _ as usize },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(error_code),
    //             "::",
    //             stringify!(_M_value)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<error_code>()))._M_cat as *const _ as usize },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(error_code),
    //             "::",
    //             stringify!(_M_cat)
    //         )
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}default_error_condition"]
        pub fn error_code_default_error_condition(
            this: *const self::root::std::error_code,
        ) -> self::root::std::error_condition;
    }
    impl Default for error_code {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl error_code {
        #[inline]
        pub unsafe fn default_error_condition(&self) -> self::root::std::error_condition {
            error_code_default_error_condition(self)
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct error_condition {
        pub _M_value: c_int,
        pub _M_cat: *const self::root::std::_V2::error_category,
    }
    // #[test]
    // fn bindgen_test_layout_error_condition() {
    //     assert_eq!(
    //         size_of::<error_condition>(),
    //         16usize,
    //         concat!("Size of: ", stringify!(error_condition))
    //     );
    //     assert_eq!(
    //         align_of::<error_condition>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(error_condition))
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<error_condition>()))._M_value as *const _ as usize
    //         },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(error_condition),
    //             "::",
    //             stringify!(_M_value)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<error_condition>()))._M_cat as *const _ as usize },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(error_condition),
    //             "::",
    //             stringify!(_M_cat)
    //         )
    //     );
    // }
    impl Default for error_condition {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    pub struct system_error {
        pub _base: self::root::std::runtime_error,
        pub _M_code: self::root::std::error_code,
    }
    // #[test]
    // fn bindgen_test_layout_system_error() {
    //     assert_eq!(
    //         size_of::<system_error>(),
    //         32usize,
    //         concat!("Size of: ", stringify!(system_error))
    //     );
    //     assert_eq!(
    //         align_of::<system_error>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(system_error))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<system_error>()))._M_code as *const _ as usize },
    //         16usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(system_error),
    //             "::",
    //             stringify!(_M_code)
    //         )
    //     );
    // }
    impl Default for system_error {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for system_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "system_error {{ _M_code: {:?} }}", self._M_code)
        }
    }
    extern "C" {
        #[link_name = "\u{1}system_error_destructor"]
        pub fn system_error_system_error_destructor(this: *mut self::root::std::system_error);
    }
    pub const _Ios_Fmtflags__S_boolalpha: self::root::std::_Ios_Fmtflags = 1;
    pub const _Ios_Fmtflags__S_dec: self::root::std::_Ios_Fmtflags = 2;
    pub const _Ios_Fmtflags__S_fixed: self::root::std::_Ios_Fmtflags = 4;
    pub const _Ios_Fmtflags__S_hex: self::root::std::_Ios_Fmtflags = 8;
    pub const _Ios_Fmtflags__S_internal: self::root::std::_Ios_Fmtflags = 16;
    pub const _Ios_Fmtflags__S_left: self::root::std::_Ios_Fmtflags = 32;
    pub const _Ios_Fmtflags__S_oct: self::root::std::_Ios_Fmtflags = 64;
    pub const _Ios_Fmtflags__S_right: self::root::std::_Ios_Fmtflags = 128;
    pub const _Ios_Fmtflags__S_scientific: self::root::std::_Ios_Fmtflags = 256;
    pub const _Ios_Fmtflags__S_showbase: self::root::std::_Ios_Fmtflags = 512;
    pub const _Ios_Fmtflags__S_showpoint: self::root::std::_Ios_Fmtflags = 1024;
    pub const _Ios_Fmtflags__S_showpos: self::root::std::_Ios_Fmtflags = 2048;
    pub const _Ios_Fmtflags__S_skipws: self::root::std::_Ios_Fmtflags = 4096;
    pub const _Ios_Fmtflags__S_unitbuf: self::root::std::_Ios_Fmtflags = 8192;
    pub const _Ios_Fmtflags__S_uppercase: self::root::std::_Ios_Fmtflags = 16384;
    pub const _Ios_Fmtflags__S_adjustfield: self::root::std::_Ios_Fmtflags = 176;
    pub const _Ios_Fmtflags__S_basefield: self::root::std::_Ios_Fmtflags = 74;
    pub const _Ios_Fmtflags__S_floatfield: self::root::std::_Ios_Fmtflags = 260;
    pub const _Ios_Fmtflags__S_ios_fmtflags_end: self::root::std::_Ios_Fmtflags = 65536;
    pub const _Ios_Fmtflags__S_ios_fmtflags_max: self::root::std::_Ios_Fmtflags = 2147483647;
    pub const _Ios_Fmtflags__S_ios_fmtflags_min: self::root::std::_Ios_Fmtflags = -2147483648;
    pub type _Ios_Fmtflags = i32;
    pub const _Ios_Openmode__S_app: self::root::std::_Ios_Openmode = 1;
    pub const _Ios_Openmode__S_ate: self::root::std::_Ios_Openmode = 2;
    pub const _Ios_Openmode__S_bin: self::root::std::_Ios_Openmode = 4;
    pub const _Ios_Openmode__S_in: self::root::std::_Ios_Openmode = 8;
    pub const _Ios_Openmode__S_out: self::root::std::_Ios_Openmode = 16;
    pub const _Ios_Openmode__S_trunc: self::root::std::_Ios_Openmode = 32;
    pub const _Ios_Openmode__S_ios_openmode_end: self::root::std::_Ios_Openmode = 65536;
    pub const _Ios_Openmode__S_ios_openmode_max: self::root::std::_Ios_Openmode = 2147483647;
    pub const _Ios_Openmode__S_ios_openmode_min: self::root::std::_Ios_Openmode = -2147483648;
    pub type _Ios_Openmode = i32;
    pub const _Ios_Iostate__S_goodbit: self::root::std::_Ios_Iostate = 0;
    pub const _Ios_Iostate__S_badbit: self::root::std::_Ios_Iostate = 1;
    pub const _Ios_Iostate__S_eofbit: self::root::std::_Ios_Iostate = 2;
    pub const _Ios_Iostate__S_failbit: self::root::std::_Ios_Iostate = 4;
    pub const _Ios_Iostate__S_ios_iostate_end: self::root::std::_Ios_Iostate = 65536;
    pub const _Ios_Iostate__S_ios_iostate_max: self::root::std::_Ios_Iostate = 2147483647;
    pub const _Ios_Iostate__S_ios_iostate_min: self::root::std::_Ios_Iostate = -2147483648;
    pub type _Ios_Iostate = i32;
    pub const _Ios_Seekdir__S_beg: self::root::std::_Ios_Seekdir = 0;
    pub const _Ios_Seekdir__S_cur: self::root::std::_Ios_Seekdir = 1;
    pub const _Ios_Seekdir__S_end: self::root::std::_Ios_Seekdir = 2;
    pub const _Ios_Seekdir__S_ios_seekdir_end: self::root::std::_Ios_Seekdir = 65536;
    pub type _Ios_Seekdir = u32;
    #[repr(C)]
    pub struct ios_base__bindgen_vtable(c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ios_base {
        pub vtable_: *const ios_base__bindgen_vtable,
        pub _M_precision: self::root::std::streamsize,
        pub _M_width: self::root::std::streamsize,
        pub _M_flags: self::root::std::ios_base_fmtflags,
        pub _M_exception: self::root::std::ios_base_iostate,
        pub _M_streambuf_state: self::root::std::ios_base_iostate,
        pub _M_callbacks: *mut self::root::std::ios_base__Callback_list,
        pub _M_word_zero: self::root::std::ios_base__Words,
        pub _M_local_word: [self::root::std::ios_base__Words; 8usize],
        pub _M_word_size: c_int,
        pub _M_word: *mut self::root::std::ios_base__Words,
        pub _M_ios_locale: self::root::std::locale,
    }
    #[repr(C)]
    pub struct ios_base_failure {
        pub _base: self::root::std::system_error,
    }
    // #[test]
    // fn bindgen_test_layout_ios_base_failure() {
    //     assert_eq!(
    //         size_of::<ios_base_failure>(),
    //         32usize,
    //         concat!("Size of: ", stringify!(ios_base_failure))
    //     );
    //     assert_eq!(
    //         align_of::<ios_base_failure>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(ios_base_failure))
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}failure"]
        pub fn ios_base_failure_failure(
            this: *mut self::root::std::ios_base_failure,
            __str: *const self::root::std::__cxx11::string,
        );
    }
    extern "C" {
        #[link_name = "\u{1}failure"]
        pub fn ios_base_failure_failure1(
            this: *mut self::root::std::ios_base_failure,
            arg1: *const self::root::std::__cxx11::string,
            arg2: *const self::root::std::error_code,
        );
    }
    extern "C" {
        #[link_name = "\u{1}failure"]
        pub fn ios_base_failure_failure2(
            this: *mut self::root::std::ios_base_failure,
            arg1: *const c_char,
            arg2: *const self::root::std::error_code,
        );
    }
    impl Default for ios_base_failure {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ::std::fmt::Debug for ios_base_failure {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "ios_base_failure {{  }}")
        }
    }
    impl ios_base_failure {
        #[inline]
        pub unsafe fn new(__str: *const self::root::std::__cxx11::string) -> Self {
            let mut __bindgen_tmp = uninitialized();
            ios_base_failure_failure(&mut __bindgen_tmp, __str);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(
            arg1: *const self::root::std::__cxx11::string,
            arg2: *const self::root::std::error_code,
        ) -> Self {
            let mut __bindgen_tmp = uninitialized();
            ios_base_failure_failure1(&mut __bindgen_tmp, arg1, arg2);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new2(arg1: *const c_char, arg2: *const self::root::std::error_code) -> Self {
            let mut __bindgen_tmp = uninitialized();
            ios_base_failure_failure2(&mut __bindgen_tmp, arg1, arg2);
            __bindgen_tmp
        }
    }
    pub use std::_Ios_Fmtflags as ios_base_fmtflags;
    pub use std::_Ios_Iostate as ios_base_iostate;
    pub use std::_Ios_Openmode as ios_base_openmode;
    pub use std::_Ios_Seekdir as ios_base_seekdir;
    pub const ios_base_event_erase_event: self::root::std::ios_base_event = 0;
    pub const ios_base_event_imbue_event: self::root::std::ios_base_event = 1;
    pub const ios_base_event_copyfmt_event: self::root::std::ios_base_event = 2;
    pub type ios_base_event = u32;
    pub type ios_base_event_callback = ::std::option::Option<
        unsafe extern "C" fn(
            __e: self::root::std::ios_base_event,
            __b: *mut self::root::std::ios_base,
            __i: c_int,
        ),
    >;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ios_base__Callback_list {
        pub _M_next: *mut self::root::std::ios_base__Callback_list,
        pub _M_fn: self::root::std::ios_base_event_callback,
        pub _M_index: c_int,
        pub _M_refcount: self::root::_Atomic_word,
    }
    // #[test]
    // fn bindgen_test_layout_ios_base__Callback_list() {
    //     assert_eq!(
    //         size_of::<ios_base__Callback_list>(),
    //         24usize,
    //         concat!("Size of: ", stringify!(ios_base__Callback_list))
    //     );
    //     assert_eq!(
    //         align_of::<ios_base__Callback_list>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(ios_base__Callback_list))
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<ios_base__Callback_list>()))._M_next as *const _ as usize
    //         },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base__Callback_list),
    //             "::",
    //             stringify!(_M_next)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<ios_base__Callback_list>()))._M_fn as *const _ as usize
    //         },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base__Callback_list),
    //             "::",
    //             stringify!(_M_fn)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<ios_base__Callback_list>()))._M_index as *const _
    //                 as usize
    //         },
    //         16usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base__Callback_list),
    //             "::",
    //             stringify!(_M_index)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<ios_base__Callback_list>()))._M_refcount as *const _
    //                 as usize
    //         },
    //         20usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base__Callback_list),
    //             "::",
    //             stringify!(_M_refcount)
    //         )
    //     );
    // }
    impl Default for ios_base__Callback_list {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ios_base__Words {
        pub _M_pword: *mut c_void,
        pub _M_iword: c_long,
    }
    // #[test]
    // fn bindgen_test_layout_ios_base__Words() {
    //     assert_eq!(
    //         size_of::<ios_base__Words>(),
    //         16usize,
    //         concat!("Size of: ", stringify!(ios_base__Words))
    //     );
    //     assert_eq!(
    //         align_of::<ios_base__Words>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(ios_base__Words))
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<ios_base__Words>()))._M_pword as *const _ as usize
    //         },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base__Words),
    //             "::",
    //             stringify!(_M_pword)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<ios_base__Words>()))._M_iword as *const _ as usize
    //         },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base__Words),
    //             "::",
    //             stringify!(_M_iword)
    //         )
    //     );
    // }
    impl Default for ios_base__Words {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub const ios_base__S_local_word_size: self::root::std::ios_base__bindgen_ty_1 = 8;
    pub type ios_base__bindgen_ty_1 = u32;
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ios_base_Init {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}_S_refcount"]
        pub static mut ios_base_Init__S_refcount: self::root::_Atomic_word;
    }
    extern "C" {
        #[link_name = "\u{1}_S_synced_with_stdio"]
        pub static mut ios_base_Init__S_synced_with_stdio: bool;
    }
    // #[test]
    // fn bindgen_test_layout_ios_base_Init() {
    //     assert_eq!(
    //         size_of::<ios_base_Init>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(ios_base_Init))
    //     );
    //     assert_eq!(
    //         align_of::<ios_base_Init>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(ios_base_Init))
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}Init"]
        pub fn ios_base_Init_Init(this: *mut self::root::std::ios_base_Init);
    }
    extern "C" {
        #[link_name = "\u{1}Init_destructor"]
        pub fn ios_base_Init_Init_destructor(this: *mut self::root::std::ios_base_Init);
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
        #[link_name = "\u{1}boolalpha"]
        pub static ios_base_boolalpha: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}dec"]
        pub static ios_base_dec: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}fixed"]
        pub static ios_base_fixed: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}hex"]
        pub static ios_base_hex: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}internal"]
        pub static ios_base_internal: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}left"]
        pub static ios_base_left: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}oct"]
        pub static ios_base_oct: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}right"]
        pub static ios_base_right: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}scientific"]
        pub static ios_base_scientific: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}showbase"]
        pub static ios_base_showbase: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}showpoint"]
        pub static ios_base_showpoint: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}showpos"]
        pub static ios_base_showpos: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}skipws"]
        pub static ios_base_skipws: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}unitbuf"]
        pub static ios_base_unitbuf: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}uppercase"]
        pub static ios_base_uppercase: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}adjustfield"]
        pub static ios_base_adjustfield: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}basefield"]
        pub static ios_base_basefield: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}floatfield"]
        pub static ios_base_floatfield: self::root::std::ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}badbit"]
        pub static ios_base_badbit: self::root::std::ios_base_iostate;
    }
    extern "C" {
        #[link_name = "\u{1}eofbit"]
        pub static ios_base_eofbit: self::root::std::ios_base_iostate;
    }
    extern "C" {
        #[link_name = "\u{1}failbit"]
        pub static ios_base_failbit: self::root::std::ios_base_iostate;
    }
    extern "C" {
        #[link_name = "\u{1}goodbit"]
        pub static ios_base_goodbit: self::root::std::ios_base_iostate;
    }
    extern "C" {
        #[link_name = "\u{1}app"]
        pub static ios_base_app: self::root::std::ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}ate"]
        pub static ios_base_ate: self::root::std::ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}binary"]
        pub static ios_base_binary: self::root::std::ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}in"]
        pub static ios_base_in: self::root::std::ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}out"]
        pub static ios_base_out: self::root::std::ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}trunc"]
        pub static ios_base_trunc: self::root::std::ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}beg"]
        pub static ios_base_beg: self::root::std::ios_base_seekdir;
    }
    extern "C" {
        #[link_name = "\u{1}cur"]
        pub static ios_base_cur: self::root::std::ios_base_seekdir;
    }
    extern "C" {
        #[link_name = "\u{1}end"]
        pub static ios_base_end: self::root::std::ios_base_seekdir;
    }
    // #[test]
    // fn bindgen_test_layout_ios_base() {
    //     assert_eq!(
    //         size_of::<ios_base>(),
    //         216usize,
    //         concat!("Size of: ", stringify!(ios_base))
    //     );
    //     assert_eq!(
    //         align_of::<ios_base>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(ios_base))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_precision as *const _ as usize },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_precision)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_width as *const _ as usize },
    //         16usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_width)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_flags as *const _ as usize },
    //         24usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_flags)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_exception as *const _ as usize },
    //         28usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_exception)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<ios_base>()))._M_streambuf_state as *const _ as usize
    //         },
    //         32usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_streambuf_state)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_callbacks as *const _ as usize },
    //         40usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_callbacks)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_word_zero as *const _ as usize },
    //         48usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_word_zero)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_local_word as *const _ as usize },
    //         64usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_local_word)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_word_size as *const _ as usize },
    //         192usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_word_size)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_word as *const _ as usize },
    //         200usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_word)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<ios_base>()))._M_ios_locale as *const _ as usize },
    //         208usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(ios_base),
    //             "::",
    //             stringify!(_M_ios_locale)
    //         )
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}register_callback"]
        pub fn ios_base_register_callback(
            this: *mut self::root::std::ios_base,
            __fn: self::root::std::ios_base_event_callback,
            __index: c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_M_call_callbacks"]
        pub fn ios_base__M_call_callbacks(
            this: *mut self::root::std::ios_base,
            __ev: self::root::std::ios_base_event,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_M_dispose_callbacks"]
        pub fn ios_base__M_dispose_callbacks(this: *mut self::root::std::ios_base);
    }
    extern "C" {
        #[link_name = "\u{1}_M_grow_words"]
        pub fn ios_base__M_grow_words(
            this: *mut self::root::std::ios_base,
            __index: c_int,
            __iword: bool,
        ) -> *mut self::root::std::ios_base__Words;
    }
    extern "C" {
        #[link_name = "\u{1}_M_init"]
        pub fn ios_base__M_init(this: *mut self::root::std::ios_base);
    }
    extern "C" {
        #[link_name = "\u{1}sync_with_stdio"]
        pub fn ios_base_sync_with_stdio(__sync: bool) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}imbue"]
        pub fn ios_base_imbue(
            this: *mut self::root::std::ios_base,
            __loc: *const self::root::std::locale,
        ) -> self::root::std::locale;
    }
    extern "C" {
        #[link_name = "\u{1}xalloc"]
        pub fn ios_base_xalloc() -> c_int;
    }
    extern "C" {
        #[link_name = "\u{1}_M_move"]
        pub fn ios_base__M_move(
            this: *mut self::root::std::ios_base,
            arg1: *mut self::root::std::ios_base,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_M_swap"]
        pub fn ios_base__M_swap(
            this: *mut self::root::std::ios_base,
            __rhs: *mut self::root::std::ios_base,
        );
    }
    extern "C" {
        #[link_name = "\u{1}ios_base"]
        pub fn ios_base_ios_base(this: *mut self::root::std::ios_base);
    }
    impl Default for ios_base {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl ios_base {
        #[inline]
        pub unsafe fn register_callback(
            &mut self,
            __fn: self::root::std::ios_base_event_callback,
            __index: c_int,
        ) {
            ios_base_register_callback(self, __fn, __index)
        }
        #[inline]
        pub unsafe fn _M_call_callbacks(&mut self, __ev: self::root::std::ios_base_event) {
            ios_base__M_call_callbacks(self, __ev)
        }
        #[inline]
        pub unsafe fn _M_dispose_callbacks(&mut self) {
            ios_base__M_dispose_callbacks(self)
        }
        #[inline]
        pub unsafe fn _M_grow_words(
            &mut self,
            __index: c_int,
            __iword: bool,
        ) -> *mut self::root::std::ios_base__Words {
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
        pub unsafe fn imbue(
            &mut self,
            __loc: *const self::root::std::locale,
        ) -> self::root::std::locale {
            ios_base_imbue(self, __loc)
        }
        #[inline]
        pub unsafe fn xalloc() -> c_int {
            ios_base_xalloc()
        }
        #[inline]
        pub unsafe fn _M_move(&mut self, arg1: *mut self::root::std::ios_base) {
            ios_base__M_move(self, arg1)
        }
        #[inline]
        pub unsafe fn _M_swap(&mut self, __rhs: *mut self::root::std::ios_base) {
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
        #[link_name = "\u{1}failure_destructor"]
        pub fn ios_base_failure_failure_destructor(this: *mut self::root::std::ios_base_failure);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn ios_base_failure_what(this: *mut c_void) -> *const c_char;
    }
    extern "C" {
        #[link_name = "\u{1}ios_base_destructor"]
        pub fn ios_base_ios_base_destructor(this: *mut self::root::std::ios_base);
    }
    #[repr(C)]
    pub struct basic_streambuf__bindgen_vtable(c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_streambuf<_CharT> {
        pub vtable_: *const basic_streambuf__bindgen_vtable,
        pub _M_in_beg: *mut self::root::std::basic_streambuf_char_type<_CharT>,
        pub _M_in_cur: *mut self::root::std::basic_streambuf_char_type<_CharT>,
        pub _M_in_end: *mut self::root::std::basic_streambuf_char_type<_CharT>,
        pub _M_out_beg: *mut self::root::std::basic_streambuf_char_type<_CharT>,
        pub _M_out_cur: *mut self::root::std::basic_streambuf_char_type<_CharT>,
        pub _M_out_end: *mut self::root::std::basic_streambuf_char_type<_CharT>,
        pub _M_buf_locale: self::root::std::locale,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_streambuf_char_type<_CharT> = _CharT;
    pub type basic_streambuf_traits_type<_Traits> = _Traits;
    pub type basic_streambuf_int_type = [u8; 0usize];
    pub type basic_streambuf_pos_type = [u8; 0usize];
    pub type basic_streambuf_off_type = [u8; 0usize];
    pub type basic_streambuf___streambuf_type<_CharT> =
        self::root::std::basic_streambuf<self::root::std::basic_streambuf_char_type<_CharT>>;
    impl<_CharT> Default for basic_streambuf<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ctype_base {
        pub _address: u8,
    }
    pub type ctype_base___to_type = *const c_int;
    pub type ctype_base_mask = c_ushort;
    pub const ctype_base_upper: self::root::std::ctype_base_mask = 256;
    pub const ctype_base_lower: self::root::std::ctype_base_mask = 512;
    pub const ctype_base_alpha: self::root::std::ctype_base_mask = 1024;
    pub const ctype_base_digit: self::root::std::ctype_base_mask = 2048;
    pub const ctype_base_xdigit: self::root::std::ctype_base_mask = 4096;
    pub const ctype_base_space: self::root::std::ctype_base_mask = 8192;
    pub const ctype_base_print: self::root::std::ctype_base_mask = 16384;
    pub const ctype_base_graph: self::root::std::ctype_base_mask = 3076;
    pub const ctype_base_cntrl: self::root::std::ctype_base_mask = 2;
    pub const ctype_base_punct: self::root::std::ctype_base_mask = 4;
    pub const ctype_base_alnum: self::root::std::ctype_base_mask = 3072;
    pub const ctype_base_blank: self::root::std::ctype_base_mask = 1;
    // #[test]
    // fn bindgen_test_layout_ctype_base() {
    //     assert_eq!(
    //         size_of::<ctype_base>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(ctype_base))
    //     );
    //     assert_eq!(
    //         align_of::<ctype_base>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(ctype_base))
    //     );
    // }
    #[repr(C)]
    pub struct istreambuf_iterator<_CharT> {
        pub _M_sbuf: *mut self::root::std::istreambuf_iterator_streambuf_type<_CharT>,
        pub _M_c: self::root::std::istreambuf_iterator_int_type,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type istreambuf_iterator_char_type<_CharT> = _CharT;
    pub type istreambuf_iterator_traits_type<_Traits> = _Traits;
    pub type istreambuf_iterator_int_type = [u8; 0usize];
    pub type istreambuf_iterator_streambuf_type<_CharT> = self::root::std::basic_streambuf<_CharT>;
    pub type istreambuf_iterator_istream_type<_CharT> = self::root::std::basic_istream<_CharT>;
    impl<_CharT> Default for istreambuf_iterator<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for istreambuf_iterator<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "istreambuf_iterator {{ _M_sbuf: {:?} }}", self._M_sbuf)
        }
    }
    impl<_CharT> PartialEq for istreambuf_iterator<_CharT>
    where
        _CharT: PartialEq,
    {
        fn eq(&self, other: &istreambuf_iterator<_CharT>) -> bool {
            self._M_sbuf == other._M_sbuf && self._M_c == other._M_c
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ostreambuf_iterator<_CharT> {
        pub _M_sbuf: *mut self::root::std::ostreambuf_iterator_streambuf_type<_CharT>,
        pub _M_failed: bool,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type ostreambuf_iterator_char_type<_CharT> = _CharT;
    pub type ostreambuf_iterator_traits_type<_Traits> = _Traits;
    pub type ostreambuf_iterator_streambuf_type<_CharT> = self::root::std::basic_streambuf<_CharT>;
    pub type ostreambuf_iterator_ostream_type<_CharT> = self::root::std::basic_ostream<_CharT>;
    impl<_CharT> Default for ostreambuf_iterator<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __ctype_abstract_base {
        pub _base: self::root::std::locale_facet,
    }
    pub type __ctype_abstract_base_char_type<_CharT> = _CharT;
    impl Default for __ctype_abstract_base {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ctype {
        pub _base: self::root::std::__ctype_abstract_base,
    }
    pub type ctype_char_type<_CharT> = _CharT;
    pub type ctype_mask = self::root::std::__ctype_abstract_base;
    impl Default for ctype {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct num_get {
        pub _base: self::root::std::locale_facet,
    }
    pub type num_get_char_type<_CharT> = _CharT;
    pub type num_get_iter_type<_InIter> = _InIter;
    impl Default for num_get {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct num_put {
        pub _base: self::root::std::locale_facet,
    }
    pub type num_put_char_type<_CharT> = _CharT;
    pub type num_put_iter_type<_OutIter> = _OutIter;
    impl Default for num_put {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ios<_CharT> {
        pub _base: self::root::std::ios_base,
        pub _M_tie: *mut self::root::std::basic_ostream<_CharT>,
        pub _M_fill: self::root::std::basic_ios_char_type<_CharT>,
        pub _M_fill_init: bool,
        pub _M_streambuf: *mut self::root::std::basic_streambuf<_CharT>,
        pub _M_ctype: *const self::root::std::basic_ios___ctype_type,
        pub _M_num_put: *const self::root::std::basic_ios___num_put_type,
        pub _M_num_get: *const self::root::std::basic_ios___num_get_type,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_ios_char_type<_CharT> = _CharT;
    pub type basic_ios_int_type = [u8; 0usize];
    pub type basic_ios_pos_type = [u8; 0usize];
    pub type basic_ios_off_type = [u8; 0usize];
    pub type basic_ios_traits_type<_Traits> = _Traits;
    pub type basic_ios___ctype_type = self::root::std::ctype;
    pub type basic_ios___num_put_type = self::root::std::num_put;
    pub type basic_ios___num_get_type = self::root::std::num_get;
    impl<_CharT> Default for basic_ios<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ostream<_CharT> {
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_ostream_char_type<_CharT> = _CharT;
    pub type basic_ostream_int_type = [u8; 0usize];
    pub type basic_ostream_pos_type = [u8; 0usize];
    pub type basic_ostream_off_type = [u8; 0usize];
    pub type basic_ostream_traits_type<_Traits> = _Traits;
    pub type basic_ostream___streambuf_type<_CharT> = self::root::std::basic_streambuf<_CharT>;
    pub type basic_ostream___ios_type<_CharT> = self::root::std::basic_ios<_CharT>;
    pub type basic_ostream___ostream_type<_CharT> = self::root::std::basic_ostream<_CharT>;
    pub type basic_ostream___num_put_type = self::root::std::num_put;
    pub type basic_ostream___ctype_type = self::root::std::ctype;
    impl<_CharT> Default for basic_ostream<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ostream_sentry {
        pub _M_ok: bool,
        pub _M_os: *mut self::root::std::basic_ostream<_CharT>,
    }
    impl Default for basic_ostream_sentry {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_istream<_CharT> {
        pub _M_gcount: self::root::std::streamsize,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_istream_char_type<_CharT> = _CharT;
    pub type basic_istream_int_type = [u8; 0usize];
    pub type basic_istream_pos_type = [u8; 0usize];
    pub type basic_istream_off_type = [u8; 0usize];
    pub type basic_istream_traits_type<_Traits> = _Traits;
    pub type basic_istream___streambuf_type<_CharT> = self::root::std::basic_streambuf<_CharT>;
    pub type basic_istream___ios_type<_CharT> = self::root::std::basic_ios<_CharT>;
    pub type basic_istream___istream_type<_CharT> = self::root::std::basic_istream<_CharT>;
    pub type basic_istream___num_get_type = self::root::std::num_get;
    pub type basic_istream___ctype_type = self::root::std::ctype;
    impl<_CharT> Default for basic_istream<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_istream_sentry {
        pub _M_ok: bool,
    }
    pub type basic_istream_sentry_traits_type = _Traits;
    pub type basic_istream_sentry___streambuf_type = self::root::std::basic_streambuf<_CharT>;
    pub type basic_istream_sentry___istream_type = self::root::std::basic_istream<_CharT>;
    pub type basic_istream_sentry___ctype_type = self::root::std::basic_istream___ctype_type;
    pub type basic_istream_sentry___int_type = [u8; 0usize];
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_iostream<_CharT> {
        pub _base: self::root::std::basic_istream<_CharT>,
        pub _base_1: self::root::std::basic_ostream<_CharT>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_iostream_char_type<_CharT> = _CharT;
    pub type basic_iostream_int_type = [u8; 0usize];
    pub type basic_iostream_pos_type = [u8; 0usize];
    pub type basic_iostream_off_type = [u8; 0usize];
    pub type basic_iostream_traits_type<_Traits> = _Traits;
    pub type basic_iostream___istream_type<_CharT> = self::root::std::basic_istream<_CharT>;
    pub type basic_iostream___ostream_type<_CharT> = self::root::std::basic_ostream<_CharT>;
    impl<_CharT> Default for basic_iostream<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct codecvt_base {
        pub _address: u8,
    }
    pub const codecvt_base_result_ok: self::root::std::codecvt_base_result = 0;
    pub const codecvt_base_result_partial: self::root::std::codecvt_base_result = 1;
    pub const codecvt_base_result_error: self::root::std::codecvt_base_result = 2;
    pub const codecvt_base_result_noconv: self::root::std::codecvt_base_result = 3;
    pub type codecvt_base_result = u32;
    // #[test]
    // fn bindgen_test_layout_codecvt_base() {
    //     assert_eq!(
    //         size_of::<codecvt_base>(),
    //         1usize,
    //         concat!("Size of: ", stringify!(codecvt_base))
    //     );
    //     assert_eq!(
    //         align_of::<codecvt_base>(),
    //         1usize,
    //         concat!("Alignment of ", stringify!(codecvt_base))
    //     );
    // }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __codecvt_abstract_base {
        pub _base: self::root::std::locale_facet,
    }
    pub use std::codecvt_base_result as __codecvt_abstract_base_result;
    pub type __codecvt_abstract_base_intern_type<_InternT> = _InternT;
    pub type __codecvt_abstract_base_extern_type<_ExternT> = _ExternT;
    pub type __codecvt_abstract_base_state_type<_StateT> = _StateT;
    impl Default for __codecvt_abstract_base {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct codecvt {
        pub _base: self::root::std::__codecvt_abstract_base,
        pub _M_c_locale_codecvt: self::root::std::__c_locale,
    }
    pub use std::codecvt_base_result as codecvt_result;
    pub type codecvt_intern_type<_InternT> = _InternT;
    pub type codecvt_extern_type<_ExternT> = _ExternT;
    pub type codecvt_state_type<_StateT> = _StateT;
    impl Default for codecvt {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type __c_lock = self::root::__gthread_mutex_t;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __basic_file {
        pub _address: u8,
    }
    #[repr(C)]
    pub struct basic_filebuf<_CharT> {
        pub _base: self::root::std::basic_streambuf<_CharT>,
        pub _M_lock: self::root::std::__c_lock,
        pub _M_file: self::root::std::basic_filebuf___file_type,
        pub _M_mode: self::root::std::ios_base_openmode,
        pub _M_state_beg: self::root::std::basic_filebuf___state_type,
        pub _M_state_cur: self::root::std::basic_filebuf___state_type,
        pub _M_state_last: self::root::std::basic_filebuf___state_type,
        pub _M_buf: *mut self::root::std::basic_filebuf_char_type<_CharT>,
        pub _M_buf_size: usize,
        pub _M_buf_allocated: bool,
        pub _M_reading: bool,
        pub _M_writing: bool,
        pub _M_pback: self::root::std::basic_filebuf_char_type<_CharT>,
        pub _M_pback_cur_save: *mut self::root::std::basic_filebuf_char_type<_CharT>,
        pub _M_pback_end_save: *mut self::root::std::basic_filebuf_char_type<_CharT>,
        pub _M_pback_init: bool,
        pub _M_codecvt: *const self::root::std::basic_filebuf___codecvt_type,
        pub _M_ext_buf: *mut c_char,
        pub _M_ext_buf_size: self::root::std::streamsize,
        pub _M_ext_next: *const c_char,
        pub _M_ext_end: *mut c_char,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_filebuf___chk_state = self::root::std::__and_;
    pub type basic_filebuf_char_type<_CharT> = _CharT;
    pub type basic_filebuf_traits_type<_Traits> = _Traits;
    pub type basic_filebuf_int_type = [u8; 0usize];
    pub type basic_filebuf_pos_type = [u8; 0usize];
    pub type basic_filebuf_off_type = [u8; 0usize];
    pub type basic_filebuf___streambuf_type<_CharT> =
        self::root::std::basic_streambuf<self::root::std::basic_filebuf_char_type<_CharT>>;
    pub type basic_filebuf___filebuf_type<_CharT> =
        self::root::std::basic_filebuf<self::root::std::basic_filebuf_char_type<_CharT>>;
    pub type basic_filebuf___file_type = self::root::std::__basic_file;
    pub type basic_filebuf___state_type = [u8; 0usize];
    pub type basic_filebuf___codecvt_type = self::root::std::codecvt;
    impl<_CharT> Default for basic_filebuf<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for basic_filebuf<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! ( f , "basic_filebuf {{ _M_lock: {:?}, _M_file: {:?}, _M_mode: {:?}, _M_buf: {:?}, _M_buf_size: {:?}, _M_buf_allocated: {:?}, _M_reading: {:?}, _M_writing: {:?}, _M_pback: Non-debuggable generic, _M_pback_cur_save: {:?}, _M_pback_end_save: {:?}, _M_pback_init: {:?}, _M_codecvt: {:?}, _M_ext_buf: {:?}, _M_ext_buf_size: {:?}, _M_ext_next: {:?}, _M_ext_end: {:?} }}" , self . _M_lock , self . _M_file , self . _M_mode , self . _M_buf , self . _M_buf_size , self . _M_buf_allocated , self . _M_reading , self . _M_writing , self . _M_pback_cur_save , self . _M_pback_end_save , self . _M_pback_init , self . _M_codecvt , self . _M_ext_buf , self . _M_ext_buf_size , self . _M_ext_next , self . _M_ext_end )
        }
    }
    #[repr(C)]
    pub struct basic_fstream<_CharT> {
        pub _base: self::root::std::basic_iostream<_CharT>,
        pub _M_filebuf: self::root::std::basic_fstream___filebuf_type<_CharT>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_fstream_char_type<_CharT> = _CharT;
    pub type basic_fstream_traits_type<_Traits> = _Traits;
    pub type basic_fstream_int_type = [u8; 0usize];
    pub type basic_fstream_pos_type = [u8; 0usize];
    pub type basic_fstream_off_type = [u8; 0usize];
    pub type basic_fstream___filebuf_type<_CharT> =
        self::root::std::basic_filebuf<self::root::std::basic_fstream_char_type<_CharT>>;
    pub type basic_fstream___ios_type<_CharT> =
        self::root::std::basic_ios<self::root::std::basic_fstream_char_type<_CharT>>;
    pub type basic_fstream___iostream_type<_CharT> =
        self::root::std::basic_iostream<self::root::std::basic_fstream_char_type<_CharT>>;
    impl<_CharT> Default for basic_fstream<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for basic_fstream<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "basic_fstream {{ _M_filebuf: {:?} }}", self._M_filebuf)
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _List_node {
        pub _base: self::root::std::__detail::_List_node_base,
        pub _M_storage: self::root::__gnu_cxx::__aligned_membuf,
    }
    impl Default for _List_node {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _List_iterator {
        pub _M_node: *mut self::root::std::__detail::_List_node_base,
    }
    pub type _List_iterator__Self = self::root::std::_List_iterator;
    pub type _List_iterator__Node = self::root::std::_List_node;
    pub type _List_iterator_difference_type = isize;
    pub type _List_iterator_iterator_category = self::root::std::bidirectional_iterator_tag;
    pub type _List_iterator_value_type<_Tp> = _Tp;
    pub type _List_iterator_pointer<_Tp> = *mut _Tp;
    pub type _List_iterator_reference<_Tp> = *mut _Tp;
    impl Default for _List_iterator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _List_const_iterator {
        pub _M_node: *const self::root::std::__detail::_List_node_base,
    }
    pub type _List_const_iterator__Self = self::root::std::_List_const_iterator;
    pub type _List_const_iterator__Node = self::root::std::_List_node;
    pub type _List_const_iterator_iterator = self::root::std::_List_iterator;
    pub type _List_const_iterator_difference_type = isize;
    pub type _List_const_iterator_iterator_category = self::root::std::bidirectional_iterator_tag;
    pub type _List_const_iterator_value_type<_Tp> = _Tp;
    pub type _List_const_iterator_pointer<_Tp> = *const _Tp;
    pub type _List_const_iterator_reference<_Tp> = *const _Tp;
    impl Default for _List_const_iterator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
}
pub mod __gnu_cxx {

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __normal_iterator<_Iterator> {
        pub _M_current: _Iterator,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
    }
    pub type __normal_iterator___traits_type = self::root::std::iterator_traits;
    pub type __normal_iterator_iterator_type<_Iterator> = _Iterator;
    pub type __normal_iterator_iterator_category =
        self::root::__gnu_cxx::__normal_iterator___traits_type;
    pub type __normal_iterator_value_type = self::root::__gnu_cxx::__normal_iterator___traits_type;
    pub type __normal_iterator_difference_type =
        self::root::__gnu_cxx::__normal_iterator___traits_type;
    pub type __normal_iterator_reference = self::root::__gnu_cxx::__normal_iterator___traits_type;
    pub type __normal_iterator_pointer = self::root::__gnu_cxx::__normal_iterator___traits_type;
    impl<_Iterator> Default for __normal_iterator<_Iterator> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Char_types {
        pub _address: u8,
    }
    pub type _Char_types_int_type = c_ulong;
    pub type _Char_types_pos_type = self::root::std::streampos;
    pub type _Char_types_off_type = self::root::std::streamoff;
    pub type _Char_types_state_type = self::root::mbstate_t;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct char_traits {
        pub _address: u8,
    }
    pub type char_traits_char_type<_CharT> = _CharT;
    pub type char_traits_int_type = self::root::__gnu_cxx::_Char_types;
    pub type char_traits_pos_type = self::root::__gnu_cxx::_Char_types;
    pub type char_traits_off_type = self::root::__gnu_cxx::_Char_types;
    pub type char_traits_state_type = self::root::__gnu_cxx::_Char_types;
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct new_allocator_rebind {
        pub _address: u8,
    }
    pub type new_allocator_rebind_other = self::root::__gnu_cxx::new_allocator;
    pub type new_allocator_propagate_on_container_move_assignment = self::root::std::true_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __alloc_traits {
        pub _address: u8,
    }
    pub type __alloc_traits_allocator_type<_Alloc> = _Alloc;
    pub type __alloc_traits__Base_type = self::root::std::allocator_traits;
    pub type __alloc_traits_value_type = self::root::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_pointer = self::root::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_const_pointer = self::root::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_size_type = self::root::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_difference_type = self::root::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_reference = *mut self::root::__gnu_cxx::__alloc_traits_value_type;
    pub type __alloc_traits_const_reference =
        *const self::root::__gnu_cxx::__alloc_traits_value_type;
    pub type __alloc_traits___is_custom_pointer = self::root::std::__and_;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __alloc_traits_rebind {
        pub _address: u8,
    }
    pub type __alloc_traits_rebind_other = self::root::__gnu_cxx::__alloc_traits__Base_type;
    impl Default for __alloc_traits {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __aligned_membuf {
        pub _M_storage: *mut c_uchar,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __aligned_membuf__Tp2<_Tp> {
        pub _M_t: _Tp,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Tp>>,
    }
    impl<_Tp> Default for __aligned_membuf__Tp2<_Tp> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Default for __aligned_membuf {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __locale_struct {
    pub __locales: [*mut self::root::__locale_data; 13usize],
    pub __ctype_b: *const c_ushort,
    pub __ctype_tolower: *const c_int,
    pub __ctype_toupper: *const c_int,
    pub __names: [*const c_char; 13usize],
}
// #[test]
// fn bindgen_test_layout___locale_struct() {
//     assert_eq!(
//         size_of::<__locale_struct>(),
//         232usize,
//         concat!("Size of: ", stringify!(__locale_struct))
//     );
//     assert_eq!(
//         align_of::<__locale_struct>(),
//         8usize,
//         concat!("Alignment of ", stringify!(__locale_struct))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__locale_struct>())).__locales as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__locale_struct),
//             "::",
//             stringify!(__locales)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__locale_struct>())).__ctype_b as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__locale_struct),
//             "::",
//             stringify!(__ctype_b)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<__locale_struct>())).__ctype_tolower as *const _ as usize
//         },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__locale_struct),
//             "::",
//             stringify!(__ctype_tolower)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<__locale_struct>())).__ctype_toupper as *const _ as usize
//         },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__locale_struct),
//             "::",
//             stringify!(__ctype_toupper)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__locale_struct>())).__names as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__locale_struct),
//             "::",
//             stringify!(__names)
//         )
//     );
// }
// impl Default for __locale_struct {
//     fn default() -> Self {
//         unsafe { zeroed() }
//     }
// }
// pub type __locale_t = *mut self::root::__locale_struct;
// pub type __off_t = c_long;
// pub type __off64_t = c_long;
// #[repr(C)]
// #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
// pub struct __pthread_internal_list {
//     pub __prev: *mut self::root::__pthread_internal_list,
//     pub __next: *mut self::root::__pthread_internal_list,
// }
// #[test]
// fn bindgen_test_layout___pthread_internal_list() {
//     assert_eq!(
//         size_of::<__pthread_internal_list>(),
//         16usize,
//         concat!("Size of: ", stringify!(__pthread_internal_list))
//     );
//     assert_eq!(
//         align_of::<__pthread_internal_list>(),
//         8usize,
//         concat!("Alignment of ", stringify!(__pthread_internal_list))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<__pthread_internal_list>())).__prev as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_internal_list),
//             "::",
//             stringify!(__prev)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<__pthread_internal_list>())).__next as *const _ as usize
//         },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_internal_list),
//             "::",
//             stringify!(__next)
//         )
//     );
// }
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type __pthread_list_t = self::root::__pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_mutex_s {
    pub __lock: c_int,
    pub __count: c_uint,
    pub __owner: c_int,
    pub __nusers: c_uint,
    pub __kind: c_int,
    pub __spins: c_short,
    pub __elision: c_short,
    pub __list: self::root::__pthread_list_t,
}
// #[test]
// fn bindgen_test_layout___pthread_mutex_s() {
//     assert_eq!(
//         size_of::<__pthread_mutex_s>(),
//         40usize,
//         concat!("Size of: ", stringify!(__pthread_mutex_s))
//     );
//     assert_eq!(
//         align_of::<__pthread_mutex_s>(),
//         8usize,
//         concat!("Alignment of ", stringify!(__pthread_mutex_s))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__lock as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_mutex_s),
//             "::",
//             stringify!(__lock)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__count as *const _ as usize },
//         4usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_mutex_s),
//             "::",
//             stringify!(__count)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__owner as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_mutex_s),
//             "::",
//             stringify!(__owner)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__nusers as *const _ as usize },
//         12usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_mutex_s),
//             "::",
//             stringify!(__nusers)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__kind as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_mutex_s),
//             "::",
//             stringify!(__kind)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__spins as *const _ as usize },
//         20usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_mutex_s),
//             "::",
//             stringify!(__spins)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__elision as *const _ as usize },
//         22usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_mutex_s),
//             "::",
//             stringify!(__elision)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__list as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__pthread_mutex_s),
//             "::",
//             stringify!(__list)
//         )
//     );
// }
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type pthread_t = c_ulong;
pub type pthread_once_t = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: self::root::__pthread_mutex_s,
    pub __size: [c_char; 40usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 5usize],
}
// #[test]
// fn bindgen_test_layout_pthread_mutex_t() {
//     assert_eq!(
//         size_of::<pthread_mutex_t>(),
//         40usize,
//         concat!("Size of: ", stringify!(pthread_mutex_t))
//     );
//     assert_eq!(
//         align_of::<pthread_mutex_t>(),
//         8usize,
//         concat!("Alignment of ", stringify!(pthread_mutex_t))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__data as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(pthread_mutex_t),
//             "::",
//             stringify!(__data)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__size as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(pthread_mutex_t),
//             "::",
//             stringify!(__size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__align as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(pthread_mutex_t),
//             "::",
//             stringify!(__align)
//         )
//     );
// }
// impl Default for pthread_mutex_t {
//     fn default() -> Self {
//         unsafe { zeroed() }
//     }
// }
// impl ::std::fmt::Debug for pthread_mutex_t {
//     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//         write!(f, "pthread_mutex_t {{ union }}")
//     }
// }
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX {
    pub re: f64,
    pub im: f64,
}
// #[test]
// fn bindgen_test_layout_t_CKCOMPLEX() {
//     assert_eq!(
//         size_of::<t_CKCOMPLEX>(),
//         16usize,
//         concat!("Size of: ", stringify!(t_CKCOMPLEX))
//     );
//     assert_eq!(
//         align_of::<t_CKCOMPLEX>(),
//         8usize,
//         concat!("Alignment of ", stringify!(t_CKCOMPLEX))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX>())).re as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKCOMPLEX),
//             "::",
//             stringify!(re)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX>())).im as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKCOMPLEX),
//             "::",
//             stringify!(im)
//         )
//     );
// }
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKPOLAR {
    pub modulus: f64,
    pub phase: f64,
}
// #[test]
// fn bindgen_test_layout_t_CKPOLAR() {
//     assert_eq!(
//         size_of::<t_CKPOLAR>(),
//         16usize,
//         concat!("Size of: ", stringify!(t_CKPOLAR))
//     );
//     assert_eq!(
//         align_of::<t_CKPOLAR>(),
//         8usize,
//         concat!("Alignment of ", stringify!(t_CKPOLAR))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKPOLAR>())).modulus as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKPOLAR),
//             "::",
//             stringify!(modulus)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKPOLAR>())).phase as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKPOLAR),
//             "::",
//             stringify!(phase)
//         )
//     );
// }
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
// #[test]
// fn bindgen_test_layout_t_CKVEC3() {
//     assert_eq!(
//         size_of::<t_CKVEC3>(),
//         24usize,
//         concat!("Size of: ", stringify!(t_CKVEC3))
//     );
//     assert_eq!(
//         align_of::<t_CKVEC3>(),
//         8usize,
//         concat!("Alignment of ", stringify!(t_CKVEC3))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).x as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKVEC3),
//             "::",
//             stringify!(x)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).y as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKVEC3),
//             "::",
//             stringify!(y)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).z as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKVEC3),
//             "::",
//             stringify!(z)
//         )
//     );
// }
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
// #[test]
// fn bindgen_test_layout_t_CKVEC4() {
//     assert_eq!(
//         size_of::<t_CKVEC4>(),
//         32usize,
//         concat!("Size of: ", stringify!(t_CKVEC4))
//     );
//     assert_eq!(
//         align_of::<t_CKVEC4>(),
//         8usize,
//         concat!("Alignment of ", stringify!(t_CKVEC4))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).x as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKVEC4),
//             "::",
//             stringify!(x)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).y as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKVEC4),
//             "::",
//             stringify!(y)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).z as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKVEC4),
//             "::",
//             stringify!(z)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).w as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(t_CKVEC4),
//             "::",
//             stringify!(w)
//         )
//     );
// }
pub type c_str = *mut c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S_Symbol_ {
    _unused: [u8; 0],
}
pub type S_Symbol = *mut self::root::S_Symbol_;
pub const ae_Operator_ae_op_plus: self::root::ae_Operator = 0;
pub const ae_Operator_ae_op_minus: self::root::ae_Operator = 1;
pub const ae_Operator_ae_op_times: self::root::ae_Operator = 2;
pub const ae_Operator_ae_op_divide: self::root::ae_Operator = 3;
pub const ae_Operator_ae_op_eq: self::root::ae_Operator = 4;
pub const ae_Operator_ae_op_neq: self::root::ae_Operator = 5;
pub const ae_Operator_ae_op_lt: self::root::ae_Operator = 6;
pub const ae_Operator_ae_op_le: self::root::ae_Operator = 7;
pub const ae_Operator_ae_op_gt: self::root::ae_Operator = 8;
pub const ae_Operator_ae_op_ge: self::root::ae_Operator = 9;
pub const ae_Operator_ae_op_and: self::root::ae_Operator = 10;
pub const ae_Operator_ae_op_or: self::root::ae_Operator = 11;
pub const ae_Operator_ae_op_s_or: self::root::ae_Operator = 12;
pub const ae_Operator_ae_op_s_and: self::root::ae_Operator = 13;
pub const ae_Operator_ae_op_shift_left: self::root::ae_Operator = 14;
pub const ae_Operator_ae_op_shift_right: self::root::ae_Operator = 15;
pub const ae_Operator_ae_op_percent: self::root::ae_Operator = 16;
pub const ae_Operator_ae_op_s_xor: self::root::ae_Operator = 17;
pub const ae_Operator_ae_op_chuck: self::root::ae_Operator = 18;
pub const ae_Operator_ae_op_plus_chuck: self::root::ae_Operator = 19;
pub const ae_Operator_ae_op_minus_chuck: self::root::ae_Operator = 20;
pub const ae_Operator_ae_op_times_chuck: self::root::ae_Operator = 21;
pub const ae_Operator_ae_op_divide_chuck: self::root::ae_Operator = 22;
pub const ae_Operator_ae_op_s_and_chuck: self::root::ae_Operator = 23;
pub const ae_Operator_ae_op_s_or_chuck: self::root::ae_Operator = 24;
pub const ae_Operator_ae_op_s_xor_chuck: self::root::ae_Operator = 25;
pub const ae_Operator_ae_op_shift_right_chuck: self::root::ae_Operator = 26;
pub const ae_Operator_ae_op_shift_left_chuck: self::root::ae_Operator = 27;
pub const ae_Operator_ae_op_percent_chuck: self::root::ae_Operator = 28;
pub const ae_Operator_ae_op_s_chuck: self::root::ae_Operator = 29;
pub const ae_Operator_ae_op_plusplus: self::root::ae_Operator = 30;
pub const ae_Operator_ae_op_minusminus: self::root::ae_Operator = 31;
pub const ae_Operator_ae_op_tilda: self::root::ae_Operator = 32;
pub const ae_Operator_ae_op_exclamation: self::root::ae_Operator = 33;
pub const ae_Operator_ae_op_at_chuck: self::root::ae_Operator = 34;
pub const ae_Operator_ae_op_unchuck: self::root::ae_Operator = 35;
pub const ae_Operator_ae_op_upchuck: self::root::ae_Operator = 36;
pub const ae_Operator_ae_op_spork: self::root::ae_Operator = 37;
pub const ae_Operator_ae_op_typeof: self::root::ae_Operator = 38;
pub const ae_Operator_ae_op_sizeof: self::root::ae_Operator = 39;
pub const ae_Operator_ae_op_new: self::root::ae_Operator = 40;
pub const ae_Operator_ae_op_arrow_left: self::root::ae_Operator = 41;
pub const ae_Operator_ae_op_arrow_right: self::root::ae_Operator = 42;
pub type ae_Operator = u32;
pub const ae_Keyword_ae_key_this: self::root::ae_Keyword = 0;
pub const ae_Keyword_ae_key_me: self::root::ae_Keyword = 1;
pub const ae_Keyword_ae_key_func: self::root::ae_Keyword = 2;
pub const ae_Keyword_ae_key_public: self::root::ae_Keyword = 3;
pub const ae_Keyword_ae_key_protected: self::root::ae_Keyword = 4;
pub const ae_Keyword_ae_key_private: self::root::ae_Keyword = 5;
pub const ae_Keyword_ae_key_static: self::root::ae_Keyword = 6;
pub const ae_Keyword_ae_key_instance: self::root::ae_Keyword = 7;
pub const ae_Keyword_ae_key_abstract: self::root::ae_Keyword = 8;
pub type ae_Keyword = u32;
pub type a_Program = *mut self::root::a_Program_;
pub type a_Section = *mut self::root::a_Section_;
pub type a_Stmt_List = *mut self::root::a_Stmt_List_;
pub type a_Class_Def = *mut self::root::a_Class_Def_;
pub type a_Func_Def = *mut self::root::a_Func_Def_;
pub type a_Stmt = *mut self::root::a_Stmt_;
pub type a_Exp = *mut self::root::a_Exp_;
pub type a_Var_Decl = *mut self::root::a_Var_Decl_;
pub type a_Var_Decl_List = *mut self::root::a_Var_Decl_List_;
pub type a_Type_Decl = *mut self::root::a_Type_Decl_;
pub type a_Arg_List = *mut self::root::a_Arg_List_;
pub type a_Id_List = *mut self::root::a_Id_List_;
pub type a_Class_Ext = *mut self::root::a_Class_Ext_;
pub type a_Class_Body = *mut self::root::a_Class_Body_;
pub type a_Array_Sub = *mut self::root::a_Array_Sub_;
pub type a_Complex = *mut self::root::a_Complex_;
pub type a_Polar = *mut self::root::a_Polar_;
pub type a_Vec = *mut self::root::a_Vec_;
pub type t_CKTYPE = *mut self::root::Chuck_Type;
pub type t_CKVALUE = *mut self::root::Chuck_Value;
pub type t_CKFUNC = *mut self::root::Chuck_Func;
pub type t_CKNSPC = *mut self::root::Chuck_Namespace;
pub type t_CKVMCODE = *mut self::root::Chuck_VM_Code;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Binary_ {
    pub lhs: self::root::a_Exp,
    pub op: self::root::ae_Operator,
    pub rhs: self::root::a_Exp,
    pub ck_func: self::root::t_CKFUNC,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Binary_() {
//     assert_eq!(
//         size_of::<a_Exp_Binary_>(),
//         48usize,
//         concat!("Size of: ", stringify!(a_Exp_Binary_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Binary_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Binary_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Binary_>())).lhs as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Binary_),
//             "::",
//             stringify!(lhs)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Binary_>())).op as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Binary_),
//             "::",
//             stringify!(op)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Binary_>())).rhs as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Binary_),
//             "::",
//             stringify!(rhs)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Binary_>())).ck_func as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Binary_),
//             "::",
//             stringify!(ck_func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Binary_>())).linepos as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Binary_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Binary_>())).self_ as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Binary_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Binary_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Cast_ {
    pub type_: self::root::a_Type_Decl,
    pub exp: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Cast_() {
//     assert_eq!(
//         size_of::<a_Exp_Cast_>(),
//         32usize,
//         concat!("Size of: ", stringify!(a_Exp_Cast_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Cast_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Cast_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Cast_>())).type_ as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Cast_),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Cast_>())).exp as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Cast_),
//             "::",
//             stringify!(exp)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Cast_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Cast_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Cast_>())).self_ as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Cast_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Cast_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Unary_ {
    pub op: self::root::ae_Operator,
    pub exp: self::root::a_Exp,
    pub type_: self::root::a_Type_Decl,
    pub array: self::root::a_Array_Sub,
    pub code: self::root::a_Stmt,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Unary_() {
//     assert_eq!(
//         size_of::<a_Exp_Unary_>(),
//         56usize,
//         concat!("Size of: ", stringify!(a_Exp_Unary_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Unary_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Unary_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Unary_>())).op as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Unary_),
//             "::",
//             stringify!(op)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Unary_>())).exp as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Unary_),
//             "::",
//             stringify!(exp)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Unary_>())).type_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Unary_),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Unary_>())).array as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Unary_),
//             "::",
//             stringify!(array)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Unary_>())).code as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Unary_),
//             "::",
//             stringify!(code)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Unary_>())).linepos as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Unary_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Unary_>())).self_ as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Unary_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Unary_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Postfix_ {
    pub exp: self::root::a_Exp,
    pub op: self::root::ae_Operator,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// 3
impl Default for a_Exp_Postfix_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Dur_ {
    pub base: self::root::a_Exp,
    pub unit: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Dur_() {
//     assert_eq!(
//         size_of::<a_Exp_Dur_>(),
//         32usize,
//         concat!("Size of: ", stringify!(a_Exp_Dur_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Dur_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Dur_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dur_>())).base as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dur_),
//             "::",
//             stringify!(base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dur_>())).unit as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dur_),
//             "::",
//             stringify!(unit)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dur_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dur_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dur_>())).self_ as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dur_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Dur_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Array_ {
    pub base: self::root::a_Exp,
    pub indices: self::root::a_Array_Sub,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Array_() {
//     assert_eq!(
//         size_of::<a_Exp_Array_>(),
//         32usize,
//         concat!("Size of: ", stringify!(a_Exp_Array_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Array_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Array_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Array_>())).base as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Array_),
//             "::",
//             stringify!(base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Array_>())).indices as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Array_),
//             "::",
//             stringify!(indices)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Array_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Array_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Array_>())).self_ as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Array_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Array_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Func_Call_ {
    pub func: self::root::a_Exp,
    pub args: self::root::a_Exp,
    pub ret_type: self::root::t_CKTYPE,
    pub ck_func: self::root::t_CKFUNC,
    pub ck_vm_code: self::root::t_CKVMCODE,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Func_Call_() {
//     assert_eq!(
//         size_of::<a_Exp_Func_Call_>(),
//         56usize,
//         concat!("Size of: ", stringify!(a_Exp_Func_Call_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Func_Call_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Func_Call_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Func_Call_>())).func as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Func_Call_),
//             "::",
//             stringify!(func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Func_Call_>())).args as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Func_Call_),
//             "::",
//             stringify!(args)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Func_Call_>())).ret_type as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Func_Call_),
//             "::",
//             stringify!(ret_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Func_Call_>())).ck_func as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Func_Call_),
//             "::",
//             stringify!(ck_func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Func_Call_>())).ck_vm_code as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Func_Call_),
//             "::",
//             stringify!(ck_vm_code)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Func_Call_>())).linepos as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Func_Call_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Func_Call_>())).self_ as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Func_Call_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Func_Call_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Dot_Member_ {
    pub base: self::root::a_Exp,
    pub t_base: self::root::t_CKTYPE,
    pub xid: self::root::S_Symbol,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Dot_Member_() {
//     assert_eq!(
//         size_of::<a_Exp_Dot_Member_>(),
//         40usize,
//         concat!("Size of: ", stringify!(a_Exp_Dot_Member_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Dot_Member_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Dot_Member_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dot_Member_>())).base as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dot_Member_),
//             "::",
//             stringify!(base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dot_Member_>())).t_base as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dot_Member_),
//             "::",
//             stringify!(t_base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dot_Member_>())).xid as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dot_Member_),
//             "::",
//             stringify!(xid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dot_Member_>())).linepos as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dot_Member_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Dot_Member_>())).self_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Dot_Member_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Dot_Member_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_If_ {
    pub cond: self::root::a_Exp,
    pub if_exp: self::root::a_Exp,
    pub else_exp: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_If_() {
//     assert_eq!(
//         size_of::<a_Exp_If_>(),
//         40usize,
//         concat!("Size of: ", stringify!(a_Exp_If_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_If_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_If_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_If_>())).cond as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_If_),
//             "::",
//             stringify!(cond)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_If_>())).if_exp as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_If_),
//             "::",
//             stringify!(if_exp)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_If_>())).else_exp as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_If_),
//             "::",
//             stringify!(else_exp)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_If_>())).linepos as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_If_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_If_>())).self_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_If_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_If_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Decl_ {
    pub type_: self::root::a_Type_Decl,
    pub var_decl_list: self::root::a_Var_Decl_List,
    pub num_var_decls: c_int,
    pub is_static: c_int,
    pub is_global: c_int,
    pub ck_type: self::root::t_CKTYPE,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Decl_() {
//     assert_eq!(
//         size_of::<a_Exp_Decl_>(),
//         56usize,
//         concat!("Size of: ", stringify!(a_Exp_Decl_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Decl_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Decl_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Decl_>())).type_ as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Decl_),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Decl_>())).var_decl_list as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Decl_),
//             "::",
//             stringify!(var_decl_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Decl_>())).num_var_decls as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Decl_),
//             "::",
//             stringify!(num_var_decls)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Decl_>())).is_static as *const _ as usize },
//         20usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Decl_),
//             "::",
//             stringify!(is_static)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Decl_>())).is_global as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Decl_),
//             "::",
//             stringify!(is_global)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Decl_>())).ck_type as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Decl_),
//             "::",
//             stringify!(ck_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Decl_>())).linepos as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Decl_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Decl_>())).self_ as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Decl_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Decl_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Var_Decl_List_ {
    pub var_decl: self::root::a_Var_Decl,
    pub next: self::root::a_Var_Decl_List,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Var_Decl_List_() {
//     assert_eq!(
//         size_of::<a_Var_Decl_List_>(),
//         32usize,
//         concat!("Size of: ", stringify!(a_Var_Decl_List_))
//     );
//     assert_eq!(
//         align_of::<a_Var_Decl_List_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Var_Decl_List_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_List_>())).var_decl as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_List_),
//             "::",
//             stringify!(var_decl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_List_>())).next as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_List_),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_List_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_List_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_List_>())).self_ as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_List_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Var_Decl_List_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Var_Decl_ {
    pub xid: self::root::S_Symbol,
    pub var_decl: self::root::a_Var_Decl,
    pub array: self::root::a_Array_Sub,
    pub value: self::root::t_CKVALUE,
    pub addr: *mut c_void,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Var_Decl_() {
//     assert_eq!(
//         size_of::<a_Var_Decl_>(),
//         56usize,
//         concat!("Size of: ", stringify!(a_Var_Decl_))
//     );
//     assert_eq!(
//         align_of::<a_Var_Decl_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Var_Decl_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_>())).xid as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_),
//             "::",
//             stringify!(xid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_>())).var_decl as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_),
//             "::",
//             stringify!(var_decl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_>())).array as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_),
//             "::",
//             stringify!(array)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_>())).value as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_),
//             "::",
//             stringify!(value)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_>())).addr as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_),
//             "::",
//             stringify!(addr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_>())).linepos as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Var_Decl_>())).self_ as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Var_Decl_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Var_Decl_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Type_Decl_ {
    pub xid: self::root::a_Id_List,
    pub array: self::root::a_Array_Sub,
    pub ref_: c_int,
    pub linepos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Type_Decl_() {
//     assert_eq!(
//         size_of::<a_Type_Decl_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Type_Decl_))
//     );
//     assert_eq!(
//         align_of::<a_Type_Decl_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Type_Decl_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Type_Decl_>())).xid as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Type_Decl_),
//             "::",
//             stringify!(xid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Type_Decl_>())).array as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Type_Decl_),
//             "::",
//             stringify!(array)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Type_Decl_>())).ref_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Type_Decl_),
//             "::",
//             stringify!(ref_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Type_Decl_>())).linepos as *const _ as usize },
//         20usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Type_Decl_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Type_Decl_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Array_Sub_ {
    pub depth: c_ulong,
    pub exp_list: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
    pub err_num: c_int,
    pub err_pos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Array_Sub_() {
//     assert_eq!(
//         size_of::<a_Array_Sub_>(),
//         40usize,
//         concat!("Size of: ", stringify!(a_Array_Sub_))
//     );
//     assert_eq!(
//         align_of::<a_Array_Sub_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Array_Sub_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Array_Sub_>())).depth as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Array_Sub_),
//             "::",
//             stringify!(depth)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Array_Sub_>())).exp_list as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Array_Sub_),
//             "::",
//             stringify!(exp_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Array_Sub_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Array_Sub_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Array_Sub_>())).self_ as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Array_Sub_),
//             "::",
//             stringify!(self_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Array_Sub_>())).err_num as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Array_Sub_),
//             "::",
//             stringify!(err_num)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Array_Sub_>())).err_pos as *const _ as usize },
//         36usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Array_Sub_),
//             "::",
//             stringify!(err_pos)
//         )
//     );
// }
impl Default for a_Array_Sub_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Arg_List_ {
    pub type_decl: self::root::a_Type_Decl,
    pub var_decl: self::root::a_Var_Decl,
    pub type_: self::root::t_CKTYPE,
    pub next: self::root::a_Arg_List,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Arg_List_() {
//     assert_eq!(
//         size_of::<a_Arg_List_>(),
//         48usize,
//         concat!("Size of: ", stringify!(a_Arg_List_))
//     );
//     assert_eq!(
//         align_of::<a_Arg_List_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Arg_List_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Arg_List_>())).type_decl as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Arg_List_),
//             "::",
//             stringify!(type_decl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Arg_List_>())).var_decl as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Arg_List_),
//             "::",
//             stringify!(var_decl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Arg_List_>())).type_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Arg_List_),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Arg_List_>())).next as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Arg_List_),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Arg_List_>())).linepos as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Arg_List_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Arg_List_>())).self_ as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Arg_List_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Arg_List_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Complex_ {
    pub re: self::root::a_Exp,
    pub im: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Complex_() {
//     assert_eq!(
//         size_of::<a_Complex_>(),
//         32usize,
//         concat!("Size of: ", stringify!(a_Complex_))
//     );
//     assert_eq!(
//         align_of::<a_Complex_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Complex_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Complex_>())).re as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Complex_),
//             "::",
//             stringify!(re)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Complex_>())).im as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Complex_),
//             "::",
//             stringify!(im)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Complex_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Complex_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Complex_>())).self_ as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Complex_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Complex_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Polar_ {
    pub mod_: self::root::a_Exp,
    pub phase: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Polar_() {
//     assert_eq!(
//         size_of::<a_Polar_>(),
//         32usize,
//         concat!("Size of: ", stringify!(a_Polar_))
//     );
//     assert_eq!(
//         align_of::<a_Polar_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Polar_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Polar_>())).mod_ as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Polar_),
//             "::",
//             stringify!(mod_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Polar_>())).phase as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Polar_),
//             "::",
//             stringify!(phase)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Polar_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Polar_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Polar_>())).self_ as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Polar_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Polar_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Vec_ {
    pub args: self::root::a_Exp,
    pub numdims: c_int,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
// #[test]
// fn bindgen_test_layout_a_Vec_() {
//     assert_eq!(
//         size_of::<a_Vec_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Vec_))
//     );
//     assert_eq!(
//         align_of::<a_Vec_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Vec_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Vec_>())).args as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Vec_),
//             "::",
//             stringify!(args)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Vec_>())).numdims as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Vec_),
//             "::",
//             stringify!(numdims)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Vec_>())).linepos as *const _ as usize },
//         12usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Vec_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Vec_>())).self_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Vec_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Vec_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const ae_Exp_Primary_Type_ae_primary_var: self::root::ae_Exp_Primary_Type = 0;
pub const ae_Exp_Primary_Type_ae_primary_num: self::root::ae_Exp_Primary_Type = 1;
pub const ae_Exp_Primary_Type_ae_primary_float: self::root::ae_Exp_Primary_Type = 2;
pub const ae_Exp_Primary_Type_ae_primary_str: self::root::ae_Exp_Primary_Type = 3;
pub const ae_Exp_Primary_Type_ae_primary_array: self::root::ae_Exp_Primary_Type = 4;
pub const ae_Exp_Primary_Type_ae_primary_exp: self::root::ae_Exp_Primary_Type = 5;
pub const ae_Exp_Primary_Type_ae_primary_hack: self::root::ae_Exp_Primary_Type = 6;
pub const ae_Exp_Primary_Type_ae_primary_complex: self::root::ae_Exp_Primary_Type = 7;
pub const ae_Exp_Primary_Type_ae_primary_polar: self::root::ae_Exp_Primary_Type = 8;
pub const ae_Exp_Primary_Type_ae_primary_vec: self::root::ae_Exp_Primary_Type = 9;
pub const ae_Exp_Primary_Type_ae_primary_char: self::root::ae_Exp_Primary_Type = 10;
pub const ae_Exp_Primary_Type_ae_primary_nil: self::root::ae_Exp_Primary_Type = 11;
pub type ae_Exp_Primary_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Exp_Primary_ {
    pub s_type: self::root::ae_Exp_Primary_Type,
    pub value: self::root::t_CKVALUE,
    pub __bindgen_anon_1: self::root::a_Exp_Primary___bindgen_ty_1,
    pub linepos: c_int,
    pub self_: self::root::a_Exp,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Exp_Primary___bindgen_ty_1 {
    pub var: self::root::S_Symbol,
    pub num: c_long,
    pub fnum: f64,
    pub str: self::root::c_str,
    pub chr: self::root::c_str,
    pub array: self::root::a_Array_Sub,
    pub exp: self::root::a_Exp,
    pub complex: self::root::a_Complex,
    pub polar: self::root::a_Polar,
    pub vec: self::root::a_Vec,
    _bindgen_union_align: u64,
}
// #[test]
// fn bindgen_test_layout_a_Exp_Primary___bindgen_ty_1() {
//     assert_eq!(
//         size_of::<a_Exp_Primary___bindgen_ty_1>(),
//         8usize,
//         concat!("Size of: ", stringify!(a_Exp_Primary___bindgen_ty_1))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Primary___bindgen_ty_1>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Primary___bindgen_ty_1))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).var as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(var)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).num as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(num)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).fnum as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(fnum)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).str as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(str)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).chr as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(chr)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).array as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(array)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).exp as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(exp)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).complex as *const _
//                 as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(complex)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).polar as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(polar)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp_Primary___bindgen_ty_1>())).vec as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary___bindgen_ty_1),
//             "::",
//             stringify!(vec)
//         )
//     );
// }
impl Default for a_Exp_Primary___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for a_Exp_Primary___bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "a_Exp_Primary___bindgen_ty_1 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout_a_Exp_Primary_() {
//     assert_eq!(
//         size_of::<a_Exp_Primary_>(),
//         40usize,
//         concat!("Size of: ", stringify!(a_Exp_Primary_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_Primary_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_Primary_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Primary_>())).s_type as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary_),
//             "::",
//             stringify!(s_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Primary_>())).value as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary_),
//             "::",
//             stringify!(value)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Primary_>())).linepos as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_Primary_>())).self_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_Primary_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Exp_Primary_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for a_Exp_Primary_ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "a_Exp_Primary_ {{ s_type: {:?}, value: {:?}, __bindgen_anon_1: {:?}, linepos: {:?}, self: {:?} }}" , self . s_type , self . value , self . __bindgen_anon_1 , self . linepos , self . self_ )
    }
}
pub const ae_Exp_Type_ae_exp_binary: self::root::ae_Exp_Type = 0;
pub const ae_Exp_Type_ae_exp_unary: self::root::ae_Exp_Type = 1;
pub const ae_Exp_Type_ae_exp_cast: self::root::ae_Exp_Type = 2;
pub const ae_Exp_Type_ae_exp_postfix: self::root::ae_Exp_Type = 3;
pub const ae_Exp_Type_ae_exp_dur: self::root::ae_Exp_Type = 4;
pub const ae_Exp_Type_ae_exp_primary: self::root::ae_Exp_Type = 5;
pub const ae_Exp_Type_ae_exp_array: self::root::ae_Exp_Type = 6;
pub const ae_Exp_Type_ae_exp_func_call: self::root::ae_Exp_Type = 7;
pub const ae_Exp_Type_ae_exp_dot_member: self::root::ae_Exp_Type = 8;
pub const ae_Exp_Type_ae_exp_if: self::root::ae_Exp_Type = 9;
pub const ae_Exp_Type_ae_exp_decl: self::root::ae_Exp_Type = 10;
pub type ae_Exp_Type = u32;
pub const ae_Exp_Meta_ae_meta_value: self::root::ae_Exp_Meta = 0;
pub const ae_Exp_Meta_ae_meta_var: self::root::ae_Exp_Meta = 1;
pub type ae_Exp_Meta = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Exp_ {
    pub s_type: self::root::ae_Exp_Type,
    pub s_meta: self::root::ae_Exp_Meta,
    pub type_: self::root::t_CKTYPE,
    pub owner: self::root::t_CKNSPC,
    pub next: self::root::a_Exp,
    pub group_size: c_ulong,
    pub cast_to: self::root::t_CKTYPE,
    pub emit_var: c_ulong,
    pub __bindgen_anon_1: self::root::a_Exp___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Exp___bindgen_ty_1 {
    pub binary: self::root::a_Exp_Binary_,
    pub unary: self::root::a_Exp_Unary_,
    pub cast: self::root::a_Exp_Cast_,
    pub postfix: self::root::a_Exp_Postfix_,
    pub dur: self::root::a_Exp_Dur_,
    pub primary: self::root::a_Exp_Primary_,
    pub array: self::root::a_Exp_Array_,
    pub func_call: self::root::a_Exp_Func_Call_,
    pub dot_member: self::root::a_Exp_Dot_Member_,
    pub exp_if: self::root::a_Exp_If_,
    pub decl: self::root::a_Exp_Decl_,
    _bindgen_union_align: [u64; 7usize],
}
// #[test]
// fn bindgen_test_layout_a_Exp___bindgen_ty_1() {
//     assert_eq!(
//         size_of::<a_Exp___bindgen_ty_1>(),
//         56usize,
//         concat!("Size of: ", stringify!(a_Exp___bindgen_ty_1))
//     );
//     assert_eq!(
//         align_of::<a_Exp___bindgen_ty_1>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp___bindgen_ty_1))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).binary as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(binary)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).unary as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(unary)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).cast as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(cast)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).postfix as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(postfix)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).dur as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(dur)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).primary as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(primary)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).array as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(array)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).func_call as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(func_call)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).dot_member as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(dot_member)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).exp_if as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(exp_if)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp___bindgen_ty_1>())).decl as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp___bindgen_ty_1),
//             "::",
//             stringify!(decl)
//         )
//     );
// }
impl Default for a_Exp___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for a_Exp___bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "a_Exp___bindgen_ty_1 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout_a_Exp_() {
//     assert_eq!(
//         size_of::<a_Exp_>(),
//         120usize,
//         concat!("Size of: ", stringify!(a_Exp_))
//     );
//     assert_eq!(
//         align_of::<a_Exp_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Exp_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).s_type as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(s_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).s_meta as *const _ as usize },
//         4usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(s_meta)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).type_ as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).owner as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(owner)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).next as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).group_size as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(group_size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).cast_to as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(cast_to)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).emit_var as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(emit_var)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Exp_>())).linepos as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Exp_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Exp_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for a_Exp_ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "a_Exp_ {{ s_type: {:?}, s_meta: {:?}, type: {:?}, owner: {:?}, next: {:?}, group_size: {:?}, cast_to: {:?}, emit_var: {:?}, __bindgen_anon_1: {:?}, linepos: {:?} }}" , self . s_type , self . s_meta , self . type_ , self . owner , self . next , self . group_size , self . cast_to , self . emit_var , self . __bindgen_anon_1 , self . linepos )
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_While_ {
    pub is_do: c_int,
    pub cond: self::root::a_Exp,
    pub body: self::root::a_Stmt,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_While_() {
//     assert_eq!(
//         size_of::<a_Stmt_While_>(),
//         40usize,
//         concat!("Size of: ", stringify!(a_Stmt_While_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_While_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_While_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_While_>())).is_do as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_While_),
//             "::",
//             stringify!(is_do)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_While_>())).cond as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_While_),
//             "::",
//             stringify!(cond)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_While_>())).body as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_While_),
//             "::",
//             stringify!(body)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_While_>())).linepos as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_While_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_While_>())).self_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_While_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_While_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Until_ {
    pub is_do: c_int,
    pub cond: self::root::a_Exp,
    pub body: self::root::a_Stmt,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_Until_() {
//     assert_eq!(
//         size_of::<a_Stmt_Until_>(),
//         40usize,
//         concat!("Size of: ", stringify!(a_Stmt_Until_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_Until_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_Until_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Until_>())).is_do as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Until_),
//             "::",
//             stringify!(is_do)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Until_>())).cond as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Until_),
//             "::",
//             stringify!(cond)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Until_>())).body as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Until_),
//             "::",
//             stringify!(body)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Until_>())).linepos as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Until_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Until_>())).self_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Until_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_Until_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_For_ {
    pub c1: self::root::a_Stmt,
    pub c2: self::root::a_Stmt,
    pub c3: self::root::a_Exp,
    pub body: self::root::a_Stmt,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_For_() {
//     assert_eq!(
//         size_of::<a_Stmt_For_>(),
//         48usize,
//         concat!("Size of: ", stringify!(a_Stmt_For_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_For_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_For_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_For_>())).c1 as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_For_),
//             "::",
//             stringify!(c1)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_For_>())).c2 as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_For_),
//             "::",
//             stringify!(c2)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_For_>())).c3 as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_For_),
//             "::",
//             stringify!(c3)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_For_>())).body as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_For_),
//             "::",
//             stringify!(body)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_For_>())).linepos as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_For_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_For_>())).self_ as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_For_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_For_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Loop_ {
    pub cond: self::root::a_Exp,
    pub body: self::root::a_Stmt,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_Loop_() {
//     assert_eq!(
//         size_of::<a_Stmt_Loop_>(),
//         32usize,
//         concat!("Size of: ", stringify!(a_Stmt_Loop_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_Loop_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_Loop_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Loop_>())).cond as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Loop_),
//             "::",
//             stringify!(cond)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Loop_>())).body as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Loop_),
//             "::",
//             stringify!(body)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Loop_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Loop_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Loop_>())).self_ as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Loop_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_Loop_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Code_ {
    pub stmt_list: self::root::a_Stmt_List,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_Code_() {
//     assert_eq!(
//         size_of::<a_Stmt_Code_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Stmt_Code_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_Code_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_Code_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Code_>())).stmt_list as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Code_),
//             "::",
//             stringify!(stmt_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Code_>())).linepos as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Code_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Code_>())).self_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Code_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_Code_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_If_ {
    pub cond: self::root::a_Exp,
    pub if_body: self::root::a_Stmt,
    pub else_body: self::root::a_Stmt,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_If_() {
//     assert_eq!(
//         size_of::<a_Stmt_If_>(),
//         40usize,
//         concat!("Size of: ", stringify!(a_Stmt_If_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_If_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_If_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_If_>())).cond as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_If_),
//             "::",
//             stringify!(cond)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_If_>())).if_body as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_If_),
//             "::",
//             stringify!(if_body)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_If_>())).else_body as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_If_),
//             "::",
//             stringify!(else_body)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_If_>())).linepos as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_If_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_If_>())).self_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_If_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_If_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Switch_ {
    pub val: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_Switch_() {
//     assert_eq!(
//         size_of::<a_Stmt_Switch_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Stmt_Switch_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_Switch_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_Switch_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Switch_>())).val as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Switch_),
//             "::",
//             stringify!(val)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Switch_>())).linepos as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Switch_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Switch_>())).self_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Switch_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_Switch_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Break_ {
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_Break_() {
//     assert_eq!(
//         size_of::<a_Stmt_Break_>(),
//         16usize,
//         concat!("Size of: ", stringify!(a_Stmt_Break_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_Break_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_Break_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Break_>())).linepos as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Break_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Break_>())).self_ as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Break_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_Break_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Continue_ {
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_Continue_() {
//     assert_eq!(
//         size_of::<a_Stmt_Continue_>(),
//         16usize,
//         concat!("Size of: ", stringify!(a_Stmt_Continue_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_Continue_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_Continue_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Continue_>())).linepos as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Continue_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Continue_>())).self_ as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Continue_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_Continue_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Return_ {
    pub val: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_Return_() {
//     assert_eq!(
//         size_of::<a_Stmt_Return_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Stmt_Return_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_Return_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_Return_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Return_>())).val as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Return_),
//             "::",
//             stringify!(val)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Return_>())).linepos as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Return_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Return_>())).self_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Return_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_Return_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Case_ {
    pub exp: self::root::a_Exp,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_Case_() {
//     assert_eq!(
//         size_of::<a_Stmt_Case_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Stmt_Case_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_Case_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_Case_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Case_>())).exp as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Case_),
//             "::",
//             stringify!(exp)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Case_>())).linepos as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Case_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_Case_>())).self_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_Case_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_Case_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_GotoLabel_ {
    pub name: self::root::S_Symbol,
    pub linepos: c_int,
    pub self_: self::root::a_Stmt,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_GotoLabel_() {
//     assert_eq!(
//         size_of::<a_Stmt_GotoLabel_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Stmt_GotoLabel_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_GotoLabel_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_GotoLabel_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_GotoLabel_>())).name as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_GotoLabel_),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_GotoLabel_>())).linepos as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_GotoLabel_),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_GotoLabel_>())).self_ as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_GotoLabel_),
//             "::",
//             stringify!(self_)
//         )
//     );
// }
impl Default for a_Stmt_GotoLabel_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const ae_Stmt_Type_ae_stmt_exp: self::root::ae_Stmt_Type = 0;
pub const ae_Stmt_Type_ae_stmt_while: self::root::ae_Stmt_Type = 1;
pub const ae_Stmt_Type_ae_stmt_until: self::root::ae_Stmt_Type = 2;
pub const ae_Stmt_Type_ae_stmt_for: self::root::ae_Stmt_Type = 3;
pub const ae_Stmt_Type_ae_stmt_loop: self::root::ae_Stmt_Type = 4;
pub const ae_Stmt_Type_ae_stmt_if: self::root::ae_Stmt_Type = 5;
pub const ae_Stmt_Type_ae_stmt_code: self::root::ae_Stmt_Type = 6;
pub const ae_Stmt_Type_ae_stmt_switch: self::root::ae_Stmt_Type = 7;
pub const ae_Stmt_Type_ae_stmt_break: self::root::ae_Stmt_Type = 8;
pub const ae_Stmt_Type_ae_stmt_continue: self::root::ae_Stmt_Type = 9;
pub const ae_Stmt_Type_ae_stmt_return: self::root::ae_Stmt_Type = 10;
pub const ae_Stmt_Type_ae_stmt_case: self::root::ae_Stmt_Type = 11;
pub const ae_Stmt_Type_ae_stmt_gotolabel: self::root::ae_Stmt_Type = 12;
pub type ae_Stmt_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Stmt_ {
    pub s_type: self::root::ae_Stmt_Type,
    pub skip: c_int,
    pub __bindgen_anon_1: self::root::a_Stmt___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Stmt___bindgen_ty_1 {
    pub stmt_exp: self::root::a_Exp,
    pub stmt_code: self::root::a_Stmt_Code_,
    pub stmt_while: self::root::a_Stmt_While_,
    pub stmt_until: self::root::a_Stmt_Until_,
    pub stmt_loop: self::root::a_Stmt_Loop_,
    pub stmt_for: self::root::a_Stmt_For_,
    pub stmt_if: self::root::a_Stmt_If_,
    pub stmt_switch: self::root::a_Stmt_Switch_,
    pub stmt_break: self::root::a_Stmt_Break_,
    pub stmt_continue: self::root::a_Stmt_Continue_,
    pub stmt_return: self::root::a_Stmt_Return_,
    pub stmt_case: self::root::a_Stmt_Case_,
    pub stmt_gotolabel: self::root::a_Stmt_GotoLabel_,
    _bindgen_union_align: [u64; 6usize],
}
// #[test]
// fn bindgen_test_layout_a_Stmt___bindgen_ty_1() {
//     assert_eq!(
//         size_of::<a_Stmt___bindgen_ty_1>(),
//         48usize,
//         concat!("Size of: ", stringify!(a_Stmt___bindgen_ty_1))
//     );
//     assert_eq!(
//         align_of::<a_Stmt___bindgen_ty_1>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt___bindgen_ty_1))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_exp as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_exp)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_code as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_code)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_while as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_while)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_until as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_until)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_loop as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_loop)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_for as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_for)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_if as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_if)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_switch as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_switch)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_break as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_break)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_continue as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_continue)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_return as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_return)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_case as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_case)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Stmt___bindgen_ty_1>())).stmt_gotolabel as *const _
//                 as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt___bindgen_ty_1),
//             "::",
//             stringify!(stmt_gotolabel)
//         )
//     );
// }
impl Default for a_Stmt___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for a_Stmt___bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "a_Stmt___bindgen_ty_1 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout_a_Stmt_() {
//     assert_eq!(
//         size_of::<a_Stmt_>(),
//         64usize,
//         concat!("Size of: ", stringify!(a_Stmt_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_>())).s_type as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_),
//             "::",
//             stringify!(s_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_>())).skip as *const _ as usize },
//         4usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_),
//             "::",
//             stringify!(skip)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_>())).linepos as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Stmt_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for a_Stmt_ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "a_Stmt_ {{ s_type: {:?}, skip: {:?}, __bindgen_anon_1: {:?}, linepos: {:?} }}",
            self.s_type, self.skip, self.__bindgen_anon_1, self.linepos
        )
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_List_ {
    pub stmt: self::root::a_Stmt,
    pub next: self::root::a_Stmt_List,
    pub linepos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Stmt_List_() {
//     assert_eq!(
//         size_of::<a_Stmt_List_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Stmt_List_))
//     );
//     assert_eq!(
//         align_of::<a_Stmt_List_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Stmt_List_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_List_>())).stmt as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_List_),
//             "::",
//             stringify!(stmt)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_List_>())).next as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_List_),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Stmt_List_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Stmt_List_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Stmt_List_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Def_ {
    pub decl: self::root::ae_Keyword,
    pub name: self::root::a_Id_List,
    pub ext: self::root::a_Class_Ext,
    pub body: self::root::a_Class_Body,
    pub type_: self::root::t_CKTYPE,
    pub iface: c_int,
    pub home: self::root::t_CKNSPC,
    pub linepos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Class_Def_() {
//     assert_eq!(
//         size_of::<a_Class_Def_>(),
//         64usize,
//         concat!("Size of: ", stringify!(a_Class_Def_))
//     );
//     assert_eq!(
//         align_of::<a_Class_Def_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Class_Def_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Def_>())).decl as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Def_),
//             "::",
//             stringify!(decl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Def_>())).name as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Def_),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Def_>())).ext as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Def_),
//             "::",
//             stringify!(ext)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Def_>())).body as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Def_),
//             "::",
//             stringify!(body)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Def_>())).type_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Def_),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Def_>())).iface as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Def_),
//             "::",
//             stringify!(iface)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Def_>())).home as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Def_),
//             "::",
//             stringify!(home)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Def_>())).linepos as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Def_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Class_Def_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Ext_ {
    pub extend_id: self::root::a_Id_List,
    pub impl_list: self::root::a_Id_List,
    pub linepos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Class_Ext_() {
//     assert_eq!(
//         size_of::<a_Class_Ext_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Class_Ext_))
//     );
//     assert_eq!(
//         align_of::<a_Class_Ext_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Class_Ext_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Ext_>())).extend_id as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Ext_),
//             "::",
//             stringify!(extend_id)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Ext_>())).impl_list as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Ext_),
//             "::",
//             stringify!(impl_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Ext_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Ext_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Class_Ext_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Body_ {
    pub section: self::root::a_Section,
    pub next: self::root::a_Class_Body,
    pub linepos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Class_Body_() {
//     assert_eq!(
//         size_of::<a_Class_Body_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Class_Body_))
//     );
//     assert_eq!(
//         align_of::<a_Class_Body_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Class_Body_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Body_>())).section as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Body_),
//             "::",
//             stringify!(section)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Body_>())).next as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Body_),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Class_Body_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Class_Body_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Class_Body_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Id_List_ {
    pub xid: self::root::S_Symbol,
    pub next: self::root::a_Id_List,
    pub linepos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Id_List_() {
//     assert_eq!(
//         size_of::<a_Id_List_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Id_List_))
//     );
//     assert_eq!(
//         align_of::<a_Id_List_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Id_List_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Id_List_>())).xid as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Id_List_),
//             "::",
//             stringify!(xid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Id_List_>())).next as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Id_List_),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Id_List_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Id_List_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Id_List_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Func_Def_ {
    pub func_decl: self::root::ae_Keyword,
    pub static_decl: self::root::ae_Keyword,
    pub type_decl: self::root::a_Type_Decl,
    pub ret_type: self::root::t_CKTYPE,
    pub name: self::root::S_Symbol,
    pub arg_list: self::root::a_Arg_List,
    pub code: self::root::a_Stmt,
    pub ck_func: self::root::t_CKFUNC,
    pub global: c_uint,
    pub s_type: c_uint,
    pub stack_depth: c_uint,
    pub dl_func_ptr: *mut c_void,
    pub linepos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Func_Def_() {
//     assert_eq!(
//         size_of::<a_Func_Def_>(),
//         88usize,
//         concat!("Size of: ", stringify!(a_Func_Def_))
//     );
//     assert_eq!(
//         align_of::<a_Func_Def_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Func_Def_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).func_decl as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(func_decl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).static_decl as *const _ as usize },
//         4usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(static_decl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).type_decl as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(type_decl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).ret_type as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(ret_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).name as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).arg_list as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(arg_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).code as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(code)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).ck_func as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(ck_func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).global as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(global)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).s_type as *const _ as usize },
//         60usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(s_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).stack_depth as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(stack_depth)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).dl_func_ptr as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(dl_func_ptr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Func_Def_>())).linepos as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Func_Def_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Func_Def_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const ae_Section_Type_ae_section_stmt: self::root::ae_Section_Type = 0;
pub const ae_Section_Type_ae_section_func: self::root::ae_Section_Type = 1;
pub const ae_Section_Type_ae_section_class: self::root::ae_Section_Type = 2;
pub type ae_Section_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Section_ {
    pub s_type: self::root::ae_Section_Type,
    pub __bindgen_anon_1: self::root::a_Section___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Section___bindgen_ty_1 {
    pub stmt_list: self::root::a_Stmt_List,
    pub class_def: self::root::a_Class_Def,
    pub func_def: self::root::a_Func_Def,
    _bindgen_union_align: u64,
}
// #[test]
// fn bindgen_test_layout_a_Section___bindgen_ty_1() {
//     assert_eq!(
//         size_of::<a_Section___bindgen_ty_1>(),
//         8usize,
//         concat!("Size of: ", stringify!(a_Section___bindgen_ty_1))
//     );
//     assert_eq!(
//         align_of::<a_Section___bindgen_ty_1>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Section___bindgen_ty_1))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Section___bindgen_ty_1>())).stmt_list as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Section___bindgen_ty_1),
//             "::",
//             stringify!(stmt_list)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Section___bindgen_ty_1>())).class_def as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Section___bindgen_ty_1),
//             "::",
//             stringify!(class_def)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<a_Section___bindgen_ty_1>())).func_def as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Section___bindgen_ty_1),
//             "::",
//             stringify!(func_def)
//         )
//     );
// }
impl Default for a_Section___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for a_Section___bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "a_Section___bindgen_ty_1 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout_a_Section_() {
//     assert_eq!(
//         size_of::<a_Section_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Section_))
//     );
//     assert_eq!(
//         align_of::<a_Section_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Section_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Section_>())).s_type as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Section_),
//             "::",
//             stringify!(s_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Section_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Section_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Section_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for a_Section_ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "a_Section_ {{ s_type: {:?}, __bindgen_anon_1: {:?}, linepos: {:?} }}",
            self.s_type, self.__bindgen_anon_1, self.linepos
        )
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Program_ {
    pub section: self::root::a_Section,
    pub next: self::root::a_Program,
    pub linepos: c_int,
}
// #[test]
// fn bindgen_test_layout_a_Program_() {
//     assert_eq!(
//         size_of::<a_Program_>(),
//         24usize,
//         concat!("Size of: ", stringify!(a_Program_))
//     );
//     assert_eq!(
//         align_of::<a_Program_>(),
//         8usize,
//         concat!("Alignment of ", stringify!(a_Program_))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Program_>())).section as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Program_),
//             "::",
//             stringify!(section)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Program_>())).next as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Program_),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<a_Program_>())).linepos as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(a_Program_),
//             "::",
//             stringify!(linepos)
//         )
//     );
// }
impl Default for a_Program_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: self::root::__mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4usize],
    _bindgen_union_align: u32,
}
// #[test]
// fn bindgen_test_layout___mbstate_t__bindgen_ty_1() {
//     assert_eq!(
//         size_of::<__mbstate_t__bindgen_ty_1>(),
//         4usize,
//         concat!("Size of: ", stringify!(__mbstate_t__bindgen_ty_1))
//     );
//     assert_eq!(
//         align_of::<__mbstate_t__bindgen_ty_1>(),
//         4usize,
//         concat!("Alignment of ", stringify!(__mbstate_t__bindgen_ty_1))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<__mbstate_t__bindgen_ty_1>())).__wch as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__mbstate_t__bindgen_ty_1),
//             "::",
//             stringify!(__wch)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<__mbstate_t__bindgen_ty_1>())).__wchb as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__mbstate_t__bindgen_ty_1),
//             "::",
//             stringify!(__wchb)
//         )
//     );
// }
impl Default for __mbstate_t__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for __mbstate_t__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "__mbstate_t__bindgen_ty_1 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout___mbstate_t() {
//     assert_eq!(
//         size_of::<__mbstate_t>(),
//         8usize,
//         concat!("Size of: ", stringify!(__mbstate_t))
//     );
//     assert_eq!(
//         align_of::<__mbstate_t>(),
//         4usize,
//         concat!("Alignment of ", stringify!(__mbstate_t))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__mbstate_t>())).__count as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__mbstate_t),
//             "::",
//             stringify!(__count)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<__mbstate_t>())).__value as *const _ as usize },
//         4usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(__mbstate_t),
//             "::",
//             stringify!(__value)
//         )
//     );
// }
impl Default for __mbstate_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for __mbstate_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "__mbstate_t {{ __count: {:?}, __value: {:?} }}",
            self.__count, self.__value
        )
    }
}
pub type FILE = self::root::_IO_FILE;
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
pub type _IO_lock_t = c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut self::root::_IO_marker,
    pub _chain: *mut self::root::_IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: self::root::__off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1usize],
    pub _lock: *mut self::root::_IO_lock_t,
    pub _offset: self::root::__off64_t,
    pub _codecvt: *mut self::root::_IO_codecvt,
    pub _wide_data: *mut self::root::_IO_wide_data,
    pub _freeres_list: *mut self::root::_IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub __pad5: usize,
    pub _mode: c_int,
    pub _unused2: [c_char; 20usize],
}
// #[test]
// fn bindgen_test_layout__IO_FILE() {
//     assert_eq!(
//         size_of::<_IO_FILE>(),
//         216usize,
//         concat!("Size of: ", stringify!(_IO_FILE))
//     );
//     assert_eq!(
//         align_of::<_IO_FILE>(),
//         8usize,
//         concat!("Alignment of ", stringify!(_IO_FILE))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_flags)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_ptr as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_read_ptr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_end as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_read_end)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_base as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_read_base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_base as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_write_base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_ptr as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_write_ptr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_end as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_write_end)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_base as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_buf_base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_end as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_buf_end)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_base as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_save_base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_backup_base as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_backup_base)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_end as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_IO_save_end)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._markers as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_markers)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._chain as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_chain)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._fileno as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_fileno)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags2 as *const _ as usize },
//         116usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_flags2)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._old_offset as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_old_offset)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._cur_column as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_cur_column)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._vtable_offset as *const _ as usize },
//         130usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_vtable_offset)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._shortbuf as *const _ as usize },
//         131usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_shortbuf)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._lock as *const _ as usize },
//         136usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_lock)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._offset as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_offset)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._codecvt as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_codecvt)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._wide_data as *const _ as usize },
//         160usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_wide_data)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._freeres_list as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_freeres_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._freeres_buf as *const _ as usize },
//         176usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_freeres_buf)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad5 as *const _ as usize },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(__pad5)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._mode as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_mode)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._unused2 as *const _ as usize },
//         196usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(_IO_FILE),
//             "::",
//             stringify!(_unused2)
//         )
//     );
// }
impl Default for _IO_FILE {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type mbstate_t = self::root::__mbstate_t;
pub type __gthread_once_t = self::root::pthread_once_t;
pub type __gthread_mutex_t = self::root::pthread_mutex_t;
pub type _Atomic_word = c_int;
pub type THREAD_HANDLE = self::root::pthread_t;
pub type THREAD_RETURN = *mut c_void;
pub type THREAD_FUNCTION =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>;
pub type MUTEX = self::root::pthread_mutex_t;
#[repr(C)]
#[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XThread {
    pub thread: self::root::THREAD_HANDLE,
}
// #[test]
// fn bindgen_test_layout_XThread() {
//     assert_eq!(
//         size_of::<XThread>(),
//         8usize,
//         concat!("Size of: ", stringify!(XThread))
//     );
//     assert_eq!(
//         align_of::<XThread>(),
//         8usize,
//         concat!("Alignment of ", stringify!(XThread))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<XThread>())).thread as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XThread),
//             "::",
//             stringify!(thread)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}start"]
    pub fn XThread_start(
        this: *mut self::root::XThread,
        routine: self::root::THREAD_FUNCTION,
        ptr: *mut c_void,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}wait"]
    pub fn XThread_wait(this: *mut self::root::XThread, milliseconds: c_long, cancel: bool)
        -> bool;
}
extern "C" {
    #[link_name = "\u{1}test"]
    pub fn XThread_test();
}
extern "C" {
    #[link_name = "\u{1}XThread"]
    pub fn XThread_XThread(this: *mut self::root::XThread);
}
extern "C" {
    #[link_name = "\u{1}XThread_destructor"]
    pub fn XThread_XThread_destructor(this: *mut self::root::XThread);
}
impl XThread {
    #[inline]
    pub unsafe fn start(&mut self, routine: self::root::THREAD_FUNCTION, ptr: *mut c_void) -> bool {
        XThread_start(self, routine, ptr)
    }
    #[inline]
    pub unsafe fn wait(&mut self, milliseconds: c_long, cancel: bool) -> bool {
        XThread_wait(self, milliseconds, cancel)
    }
    #[inline]
    pub unsafe fn test() {
        XThread_test()
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
#[repr(C)]
pub struct XMutex {
    pub mutex: self::root::MUTEX,
}
// #[test]
// fn bindgen_test_layout_XMutex() {
//     assert_eq!(
//         size_of::<XMutex>(),
//         40usize,
//         concat!("Size of: ", stringify!(XMutex))
//     );
//     assert_eq!(
//         align_of::<XMutex>(),
//         8usize,
//         concat!("Alignment of ", stringify!(XMutex))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<XMutex>())).mutex as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XMutex),
//             "::",
//             stringify!(mutex)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}acquire"]
    pub fn XMutex_acquire(this: *mut self::root::XMutex);
}
extern "C" {
    #[link_name = "\u{1}release"]
    pub fn XMutex_release(this: *mut self::root::XMutex);
}
extern "C" {
    #[link_name = "\u{1}XMutex"]
    pub fn XMutex_XMutex(this: *mut self::root::XMutex);
}
extern "C" {
    #[link_name = "\u{1}XMutex_destructor"]
    pub fn XMutex_XMutex_destructor(this: *mut self::root::XMutex);
}
impl Default for XMutex {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for XMutex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "XMutex {{ mutex: {:?} }}", self.mutex)
    }
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
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XWriteThread {
    pub m_thread_exit: c_ulong,
    pub m_thread: self::root::XThread,
    pub m_data_buffer: *mut self::root::FastCircularBuffer,
    pub m_bytes_in_buffer: usize,
    pub m_thread_buffer: *mut c_uchar,
    pub m_stream: *mut self::root::FILE,
    pub m_msg_buffer: *mut self::root::CircularBuffer<self::root::XWriteThread_Message>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XWriteThread_Message {
    pub operation: self::root::XWriteThread_Message__bindgen_ty_1,
    pub file: *mut self::root::FILE,
    pub __bindgen_anon_1: self::root::XWriteThread_Message__bindgen_ty_2,
}
pub const XWriteThread_Message_WRITE: self::root::XWriteThread_Message__bindgen_ty_1 = 0;
pub const XWriteThread_Message_SEEK: self::root::XWriteThread_Message__bindgen_ty_1 = 1;
pub const XWriteThread_Message_FLUSH: self::root::XWriteThread_Message__bindgen_ty_1 = 2;
pub const XWriteThread_Message_CLOSE: self::root::XWriteThread_Message__bindgen_ty_1 = 3;
pub const XWriteThread_Message_SHUTDOWN: self::root::XWriteThread_Message__bindgen_ty_1 = 4;
pub type XWriteThread_Message__bindgen_ty_1 = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub union XWriteThread_Message__bindgen_ty_2 {
    pub write: self::root::XWriteThread_Message__bindgen_ty_2__bindgen_ty_1,
    pub seek: self::root::XWriteThread_Message__bindgen_ty_2__bindgen_ty_2,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XWriteThread_Message__bindgen_ty_2__bindgen_ty_1 {
    pub data_size: usize,
}
// #[test]
// fn bindgen_test_layout_XWriteThread_Message__bindgen_ty_2__bindgen_ty_1() {
//     assert_eq!(
//         size_of::<XWriteThread_Message__bindgen_ty_2__bindgen_ty_1>(),
//         8usize,
//         concat!(
//             "Size of: ",
//             stringify!(XWriteThread_Message__bindgen_ty_2__bindgen_ty_1)
//         )
//     );
//     assert_eq!(
//         align_of::<XWriteThread_Message__bindgen_ty_2__bindgen_ty_1>(),
//         8usize,
//         concat!(
//             "Alignment of ",
//             stringify!(XWriteThread_Message__bindgen_ty_2__bindgen_ty_1)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<XWriteThread_Message__bindgen_ty_2__bindgen_ty_1>()))
//                 .data_size as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread_Message__bindgen_ty_2__bindgen_ty_1),
//             "::",
//             stringify!(data_size)
//         )
//     );
// }
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XWriteThread_Message__bindgen_ty_2__bindgen_ty_2 {
    pub offset: c_long,
    pub whence: c_int,
}
// #[test]
// fn bindgen_test_layout_XWriteThread_Message__bindgen_ty_2__bindgen_ty_2() {
//     assert_eq!(
//         size_of::<XWriteThread_Message__bindgen_ty_2__bindgen_ty_2>(),
//         16usize,
//         concat!(
//             "Size of: ",
//             stringify!(XWriteThread_Message__bindgen_ty_2__bindgen_ty_2)
//         )
//     );
//     assert_eq!(
//         align_of::<XWriteThread_Message__bindgen_ty_2__bindgen_ty_2>(),
//         8usize,
//         concat!(
//             "Alignment of ",
//             stringify!(XWriteThread_Message__bindgen_ty_2__bindgen_ty_2)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<XWriteThread_Message__bindgen_ty_2__bindgen_ty_2>())).offset
//                 as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread_Message__bindgen_ty_2__bindgen_ty_2),
//             "::",
//             stringify!(offset)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<XWriteThread_Message__bindgen_ty_2__bindgen_ty_2>())).whence
//                 as *const _ as usize
//         },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread_Message__bindgen_ty_2__bindgen_ty_2),
//             "::",
//             stringify!(whence)
//         )
//     );
// }
// #[test]
// fn bindgen_test_layout_XWriteThread_Message__bindgen_ty_2() {
//     assert_eq!(
//         size_of::<XWriteThread_Message__bindgen_ty_2>(),
//         16usize,
//         concat!("Size of: ", stringify!(XWriteThread_Message__bindgen_ty_2))
//     );
//     assert_eq!(
//         align_of::<XWriteThread_Message__bindgen_ty_2>(),
//         8usize,
//         concat!(
//             "Alignment of ",
//             stringify!(XWriteThread_Message__bindgen_ty_2)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<XWriteThread_Message__bindgen_ty_2>())).write as *const _
//                 as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread_Message__bindgen_ty_2),
//             "::",
//             stringify!(write)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<XWriteThread_Message__bindgen_ty_2>())).seek as *const _
//                 as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread_Message__bindgen_ty_2),
//             "::",
//             stringify!(seek)
//         )
//     );
// }
impl Default for XWriteThread_Message__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for XWriteThread_Message__bindgen_ty_2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "XWriteThread_Message__bindgen_ty_2 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout_XWriteThread_Message() {
//     assert_eq!(
//         size_of::<XWriteThread_Message>(),
//         32usize,
//         concat!("Size of: ", stringify!(XWriteThread_Message))
//     );
//     assert_eq!(
//         align_of::<XWriteThread_Message>(),
//         8usize,
//         concat!("Alignment of ", stringify!(XWriteThread_Message))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<XWriteThread_Message>())).operation as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread_Message),
//             "::",
//             stringify!(operation)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<XWriteThread_Message>())).file as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread_Message),
//             "::",
//             stringify!(file)
//         )
//     );
// }
impl Default for XWriteThread_Message {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for XWriteThread_Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "XWriteThread_Message {{ operation: {:?}, file: {:?}, __bindgen_anon_1: {:?} }}",
            self.operation, self.file, self.__bindgen_anon_1
        )
    }
}
extern "C" {
    #[link_name = "\u{1}PRODUCER_BUFFER_SIZE"]
    pub static XWriteThread_PRODUCER_BUFFER_SIZE: usize;
}
extern "C" {
    #[link_name = "\u{1}o_defaultWriteThread"]
    pub static mut XWriteThread_o_defaultWriteThread: *mut self::root::XWriteThread;
}
// #[test]
// fn bindgen_test_layout_XWriteThread() {
//     assert_eq!(
//         size_of::<XWriteThread>(),
//         56usize,
//         concat!("Size of: ", stringify!(XWriteThread))
//     );
//     assert_eq!(
//         align_of::<XWriteThread>(),
//         8usize,
//         concat!("Alignment of ", stringify!(XWriteThread))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<XWriteThread>())).m_thread_exit as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread),
//             "::",
//             stringify!(m_thread_exit)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<XWriteThread>())).m_thread as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread),
//             "::",
//             stringify!(m_thread)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<XWriteThread>())).m_data_buffer as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread),
//             "::",
//             stringify!(m_data_buffer)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<XWriteThread>())).m_bytes_in_buffer as *const _ as usize
//         },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread),
//             "::",
//             stringify!(m_bytes_in_buffer)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<XWriteThread>())).m_thread_buffer as *const _ as usize
//         },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread),
//             "::",
//             stringify!(m_thread_buffer)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<XWriteThread>())).m_stream as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread),
//             "::",
//             stringify!(m_stream)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<XWriteThread>())).m_msg_buffer as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(XWriteThread),
//             "::",
//             stringify!(m_msg_buffer)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}shared"]
    pub fn XWriteThread_shared() -> *mut self::root::XWriteThread;
}
extern "C" {
    #[link_name = "\u{1}fwrite"]
    pub fn XWriteThread_fwrite(
        this: *mut self::root::XWriteThread,
        ptr: *const c_void,
        size: usize,
        nitems: usize,
        stream: *mut self::root::FILE,
    ) -> usize;
}
extern "C" {
    #[link_name = "\u{1}fseek"]
    pub fn XWriteThread_fseek(
        this: *mut self::root::XWriteThread,
        stream: *mut self::root::FILE,
        offset: c_long,
        whence: c_int,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}fflush"]
    pub fn XWriteThread_fflush(
        this: *mut self::root::XWriteThread,
        stream: *mut self::root::FILE,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}fclose"]
    pub fn XWriteThread_fclose(
        this: *mut self::root::XWriteThread,
        stream: *mut self::root::FILE,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn XWriteThread_shutdown(this: *mut self::root::XWriteThread);
}
extern "C" {
    #[link_name = "\u{1}XWriteThread"]
    pub fn XWriteThread_XWriteThread(
        this: *mut self::root::XWriteThread,
        data_buffer_size: usize,
        msg_buffer_size: usize,
    );
}
impl Default for XWriteThread {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl XWriteThread {
    #[inline]
    pub unsafe fn shared() -> *mut self::root::XWriteThread {
        XWriteThread_shared()
    }
    #[inline]
    pub unsafe fn fwrite(
        &mut self,
        ptr: *const c_void,
        size: usize,
        nitems: usize,
        stream: *mut self::root::FILE,
    ) -> usize {
        XWriteThread_fwrite(self, ptr, size, nitems, stream)
    }
    #[inline]
    pub unsafe fn fseek(
        &mut self,
        stream: *mut self::root::FILE,
        offset: c_long,
        whence: c_int,
    ) -> c_int {
        XWriteThread_fseek(self, stream, offset, whence)
    }
    #[inline]
    pub unsafe fn fflush(&mut self, stream: *mut self::root::FILE) -> c_int {
        XWriteThread_fflush(self, stream)
    }
    #[inline]
    pub unsafe fn fclose(&mut self, stream: *mut self::root::FILE) -> c_int {
        XWriteThread_fclose(self, stream)
    }
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
pub struct ck_socket_ {
    _unused: [u8; 0],
}
pub type ck_socket = *mut self::root::ck_socket_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WvOut {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_Carrier {
    pub chuck: *mut self::root::ChucK,
    pub compiler: *mut self::root::Chuck_Compiler,
    pub env: *mut self::root::Chuck_Env,
    pub vm: *mut self::root::Chuck_VM,
    pub chout: *mut self::root::Chuck_IO_Chout,
    pub cherr: *mut self::root::Chuck_IO_Cherr,
    pub otf_socket: self::root::ck_socket,
    pub otf_port: c_long,
    pub otf_thread: self::root::pthread_t,
    pub stk_writeThread: *mut self::root::XWriteThread,
    pub stk_wvOutMap: self::root::std::map,
}
// #[test]
// fn bindgen_test_layout_Chuck_Carrier() {
//     assert_eq!(
//         size_of::<Chuck_Carrier>(),
//         128usize,
//         concat!("Size of: ", stringify!(Chuck_Carrier))
//     );
//     assert_eq!(
//         align_of::<Chuck_Carrier>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Carrier))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).chuck as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(chuck)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).compiler as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(compiler)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).env as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(env)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).vm as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(vm)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).chout as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(chout)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).cherr as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(cherr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).otf_socket as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(otf_socket)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).otf_port as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(otf_port)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).otf_thread as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(otf_thread)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Carrier>())).stk_writeThread as *const _ as usize
//         },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(stk_writeThread)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Carrier>())).stk_wvOutMap as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Carrier),
//             "::",
//             stringify!(stk_wvOutMap)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}hintIsRealtimeAudio"]
    pub fn Chuck_Carrier_hintIsRealtimeAudio(this: *mut self::root::Chuck_Carrier) -> c_ulong;
}
impl Default for Chuck_Carrier {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Carrier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Carrier {{ chuck: {:?}, compiler: {:?}, env: {:?}, vm: {:?}, chout: {:?}, cherr: {:?}, otf_socket: {:?}, otf_port: {:?}, otf_thread: {:?}, stk_writeThread: {:?}, stk_wvOutMap: {:?} }}" , self . chuck , self . compiler , self . env , self . vm , self . chout , self . cherr , self . otf_socket , self . otf_port , self . otf_thread , self . stk_writeThread , self . stk_wvOutMap )
    }
}
impl PartialEq for Chuck_Carrier {
    fn eq(&self, other: &Chuck_Carrier) -> bool {
        self.chuck == other.chuck
            && self.compiler == other.compiler
            && self.env == other.env
            && self.vm == other.vm
            && self.chout == other.chout
            && self.cherr == other.cherr
            && self.otf_socket == other.otf_socket
            && self.otf_port == other.otf_port
            && self.otf_thread == other.otf_thread
            && self.stk_writeThread == other.stk_writeThread
            && self.stk_wvOutMap == other.stk_wvOutMap
    }
}
impl Chuck_Carrier {
    #[inline]
    pub unsafe fn hintIsRealtimeAudio(&mut self) -> c_ulong {
        Chuck_Carrier_hintIsRealtimeAudio(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dirstream {
    _unused: [u8; 0],
}
pub type DIR = self::root::__dirstream;
#[repr(C)]
pub struct Chuck_VM_Object__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_VM_Object {
    pub vtable_: *const Chuck_VM_Object__bindgen_vtable,
    pub m_ref_count: c_ulong,
    pub m_pooled: c_ulong,
    pub m_locked: c_ulong,
    pub m_v_ref: *mut self::root::std::vector,
}
extern "C" {
    #[link_name = "\u{1}our_locks_in_effect"]
    pub static mut Chuck_VM_Object_our_locks_in_effect: c_ulong;
}
// #[test]
// fn bindgen_test_layout_Chuck_VM_Object() {
//     assert_eq!(
//         size_of::<Chuck_VM_Object>(),
//         40usize,
//         concat!("Size of: ", stringify!(Chuck_VM_Object))
//     );
//     assert_eq!(
//         align_of::<Chuck_VM_Object>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VM_Object))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Object>())).m_ref_count as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Object),
//             "::",
//             stringify!(m_ref_count)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Object>())).m_pooled as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Object),
//             "::",
//             stringify!(m_pooled)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Object>())).m_locked as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Object),
//             "::",
//             stringify!(m_locked)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Object>())).m_v_ref as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Object),
//             "::",
//             stringify!(m_v_ref)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}lock_all"]
    pub fn Chuck_VM_Object_lock_all();
}
extern "C" {
    #[link_name = "\u{1}unlock_all"]
    pub fn Chuck_VM_Object_unlock_all();
}
impl Default for Chuck_VM_Object {
    fn default() -> Self {
        unsafe { zeroed() }
    }
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
    pub funcs: self::root::std::vector,
}
// #[test]
// fn bindgen_test_layout_Chuck_VTable() {
//     assert_eq!(
//         size_of::<Chuck_VTable>(),
//         24usize,
//         concat!("Size of: ", stringify!(Chuck_VTable))
//     );
//     assert_eq!(
//         align_of::<Chuck_VTable>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VTable))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VTable>())).funcs as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VTable),
//             "::",
//             stringify!(funcs)
//         )
//     );
// }
impl Default for Chuck_VTable {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_VTable {{ funcs: {:?} }}", self.funcs)
    }
}
impl PartialEq for Chuck_VTable {
    fn eq(&self, other: &Chuck_VTable) -> bool {
        self.funcs == other.funcs
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_Object {
    pub _base: self::root::Chuck_VM_Object,
    pub vtable: *mut self::root::Chuck_VTable,
    pub type_ref: *mut self::root::Chuck_Type,
    pub size: c_ulong,
    pub data: *mut c_uchar,
}
// #[test]
// fn bindgen_test_layout_Chuck_Object() {
//     assert_eq!(
//         size_of::<Chuck_Object>(),
//         72usize,
//         concat!("Size of: ", stringify!(Chuck_Object))
//     );
//     assert_eq!(
//         align_of::<Chuck_Object>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Object))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Object>())).vtable as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Object),
//             "::",
//             stringify!(vtable)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Object>())).type_ref as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Object),
//             "::",
//             stringify!(type_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Object>())).size as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Object),
//             "::",
//             stringify!(size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Object>())).data as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Object),
//             "::",
//             stringify!(data)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}Chuck_Object"]
    pub fn Chuck_Object_Chuck_Object(this: *mut self::root::Chuck_Object);
}
impl Default for Chuck_Object {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Chuck_Object {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Object_Chuck_Object(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Object_destructor"]
    pub fn Chuck_Object_Chuck_Object_destructor(this: *mut self::root::Chuck_Object);
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_Array {
    pub _base: self::root::Chuck_Object,
    pub m_array_type: *mut self::root::Chuck_Type,
}
// #[test]
// fn bindgen_test_layout_Chuck_Array() {
//     assert_eq!(
//         size_of::<Chuck_Array>(),
//         80usize,
//         concat!("Size of: ", stringify!(Chuck_Array))
//     );
//     assert_eq!(
//         align_of::<Chuck_Array>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Array))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Array>())).m_array_type as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Array),
//             "::",
//             stringify!(m_array_type)
//         )
//     );
// }
impl Default for Chuck_Array {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
pub struct Chuck_Array4 {
    pub _base: self::root::Chuck_Array,
    pub m_vector: self::root::std::vector,
    pub m_map: self::root::std::map,
    pub m_is_obj: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Chuck_Array4() {
//     assert_eq!(
//         size_of::<Chuck_Array4>(),
//         160usize,
//         concat!("Size of: ", stringify!(Chuck_Array4))
//     );
//     assert_eq!(
//         align_of::<Chuck_Array4>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Array4))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Array4>())).m_vector as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Array4),
//             "::",
//             stringify!(m_vector)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Array4>())).m_map as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Array4),
//             "::",
//             stringify!(m_map)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Array4>())).m_is_obj as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Array4),
//             "::",
//             stringify!(m_is_obj)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr(this: *mut self::root::Chuck_Array4, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr1(
        this: *mut self::root::Chuck_Array4,
        key: *const self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get(
        this: *mut self::root::Chuck_Array4,
        i: c_long,
        val: *mut c_ulong,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get1(
        this: *mut self::root::Chuck_Array4,
        key: *const self::root::std::__cxx11::string,
        val: *mut c_ulong,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set(this: *mut self::root::Chuck_Array4, i: c_long, val: c_ulong)
        -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set1(
        this: *mut self::root::Chuck_Array4,
        key: *const self::root::std::__cxx11::string,
        val: c_ulong,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array4_push_back(this: *mut self::root::Chuck_Array4, val: c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array4_pop_back(this: *mut self::root::Chuck_Array4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array4_back(this: *const self::root::Chuck_Array4, val: *mut c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array4_zero(this: *mut self::root::Chuck_Array4, start: c_ulong, end: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4"]
    pub fn Chuck_Array4_Chuck_Array4(
        this: *mut self::root::Chuck_Array4,
        is_obj: c_ulong,
        capacity: c_long,
    );
}
impl Default for Chuck_Array4 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Array4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Array4 {{ m_vector: {:?}, m_map: {:?}, m_is_obj: {:?} }}",
            self.m_vector, self.m_map, self.m_is_obj
        )
    }
}
impl Chuck_Array4 {
    #[inline]
    pub unsafe fn addr(&mut self, i: c_long) -> c_ulong {
        Chuck_Array4_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const self::root::std::__cxx11::string) -> c_ulong {
        Chuck_Array4_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: c_long, val: *mut c_ulong) -> c_long {
        Chuck_Array4_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const self::root::std::__cxx11::string,
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
        key: *const self::root::std::__cxx11::string,
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
        let mut __bindgen_tmp = uninitialized();
        Chuck_Array4_Chuck_Array4(&mut __bindgen_tmp, is_obj, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4_destructor"]
    pub fn Chuck_Array4_Chuck_Array4_destructor(this: *mut self::root::Chuck_Array4);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array4_clear(this: *mut c_void);
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
        key: *const self::root::std::__cxx11::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array4_erase(
        this: *mut c_void,
        key: *const self::root::std::__cxx11::string,
    ) -> c_long;
}
#[repr(C)]
pub struct Chuck_Event {
    pub _base: self::root::Chuck_Object,
    pub m_queue: self::root::std::queue<self::root::std::deque>,
    pub m_queue_lock: self::root::XMutex,
}
extern "C" {
    #[link_name = "\u{1}our_can_wait"]
    pub static mut Chuck_Event_our_can_wait: c_ulong;
}
// #[test]
// fn bindgen_test_layout_Chuck_Event() {
//     assert_eq!(
//         size_of::<Chuck_Event>(),
//         192usize,
//         concat!("Size of: ", stringify!(Chuck_Event))
//     );
//     assert_eq!(
//         align_of::<Chuck_Event>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Event))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Event>())).m_queue as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Event),
//             "::",
//             stringify!(m_queue)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Event>())).m_queue_lock as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Event),
//             "::",
//             stringify!(m_queue_lock)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}signal"]
    pub fn Chuck_Event_signal(this: *mut self::root::Chuck_Event);
}
extern "C" {
    #[link_name = "\u{1}broadcast"]
    pub fn Chuck_Event_broadcast(this: *mut self::root::Chuck_Event);
}
extern "C" {
    #[link_name = "\u{1}wait"]
    pub fn Chuck_Event_wait(
        this: *mut self::root::Chuck_Event,
        shred: *mut self::root::Chuck_VM_Shred,
        vm: *mut self::root::Chuck_VM,
    );
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_Event_remove(
        this: *mut self::root::Chuck_Event,
        shred: *mut self::root::Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_broadcast"]
    pub fn Chuck_Event_queue_broadcast(
        this: *mut self::root::Chuck_Event,
        event_buffer: *mut self::root::CBufferSimple,
    );
}
impl Default for Chuck_Event {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Event {{ m_queue: {:?}, m_queue_lock: {:?} }}",
            self.m_queue, self.m_queue_lock
        )
    }
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
    pub unsafe fn wait(
        &mut self,
        shred: *mut self::root::Chuck_VM_Shred,
        vm: *mut self::root::Chuck_VM,
    ) {
        Chuck_Event_wait(self, shred, vm)
    }
    #[inline]
    pub unsafe fn remove(&mut self, shred: *mut self::root::Chuck_VM_Shred) -> c_ulong {
        Chuck_Event_remove(self, shred)
    }
    #[inline]
    pub unsafe fn queue_broadcast(&mut self, event_buffer: *mut self::root::CBufferSimple) {
        Chuck_Event_queue_broadcast(self, event_buffer)
    }
}
#[repr(C)]
pub struct Chuck_String {
    pub _base: self::root::Chuck_Object,
    pub m_charptr: *const c_char,
    pub m_str: self::root::std::__cxx11::string,
}
// #[test]
// fn bindgen_test_layout_Chuck_String() {
//     assert_eq!(
//         size_of::<Chuck_String>(),
//         112usize,
//         concat!("Size of: ", stringify!(Chuck_String))
//     );
//     assert_eq!(
//         align_of::<Chuck_String>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_String))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_String>())).m_charptr as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_String),
//             "::",
//             stringify!(m_charptr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_String>())).m_str as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_String),
//             "::",
//             stringify!(m_str)
//         )
//     );
// }
impl Default for Chuck_String {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_String {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_String {{ m_charptr: {:?}, m_str: {:?} }}",
            self.m_charptr, self.m_str
        )
    }
}
#[repr(C)]
pub struct Chuck_IO {
    pub _base: self::root::Chuck_Event,
    pub m_asyncEvent: *mut self::root::Chuck_Event,
    pub m_thread: *mut self::root::XThread,
}
#[repr(C)]
pub struct Chuck_IO_async_args {
    pub fileio_obj: *mut self::root::Chuck_IO_File,
    pub RETURN: *mut c_void,
    pub intArg: c_long,
    pub floatArg: f64,
    pub stringArg: self::root::std::__cxx11::string,
}
// #[test]
// fn bindgen_test_layout_Chuck_IO_async_args() {
//     assert_eq!(
//         size_of::<Chuck_IO_async_args>(),
//         64usize,
//         concat!("Size of: ", stringify!(Chuck_IO_async_args))
//     );
//     assert_eq!(
//         align_of::<Chuck_IO_async_args>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_IO_async_args))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_IO_async_args>())).fileio_obj as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_async_args),
//             "::",
//             stringify!(fileio_obj)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_async_args>())).RETURN as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_async_args),
//             "::",
//             stringify!(RETURN)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_async_args>())).intArg as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_async_args),
//             "::",
//             stringify!(intArg)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_IO_async_args>())).floatArg as *const _ as usize
//         },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_async_args),
//             "::",
//             stringify!(floatArg)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_IO_async_args>())).stringArg as *const _ as usize
//         },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_async_args),
//             "::",
//             stringify!(stringArg)
//         )
//     );
// }
impl Default for Chuck_IO_async_args {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_IO_async_args {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_IO_async_args {{ fileio_obj: {:?}, RETURN: {:?}, intArg: {:?}, floatArg: {:?}, stringArg: {:?} }}" , self . fileio_obj , self . RETURN , self . intArg , self . floatArg , self . stringArg )
    }
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
// #[test]
// fn bindgen_test_layout_Chuck_IO() {
//     assert_eq!(
//         size_of::<Chuck_IO>(),
//         208usize,
//         concat!("Size of: ", stringify!(Chuck_IO))
//     );
//     assert_eq!(
//         align_of::<Chuck_IO>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_IO))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO>())).m_asyncEvent as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO),
//             "::",
//             stringify!(m_asyncEvent)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO>())).m_thread as *const _ as usize },
//         200usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO),
//             "::",
//             stringify!(m_thread)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}Chuck_IO"]
    pub fn Chuck_IO_Chuck_IO(this: *mut self::root::Chuck_IO);
}
impl Default for Chuck_IO {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_IO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_IO {{ m_asyncEvent: {:?}, m_thread: {:?} }}",
            self.m_asyncEvent, self.m_thread
        )
    }
}
impl Chuck_IO {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_Chuck_IO(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_destructor"]
    pub fn Chuck_IO_Chuck_IO_destructor(this: *mut self::root::Chuck_IO);
}
#[repr(C)]
pub struct Chuck_IO_File {
    pub _base: self::root::Chuck_IO,
    pub m_flags: c_long,
    pub m_iomode: c_long,
    pub m_io: self::root::std::fstream,
    pub m_dir: *mut self::root::DIR,
    pub m_dir_start: c_long,
    pub m_path: self::root::std::__cxx11::string,
    pub m_vmRef: *mut self::root::Chuck_VM,
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
// #[test]
// fn bindgen_test_layout_Chuck_IO_File() {
//     assert_eq!(
//         size_of::<Chuck_IO_File>(),
//         808usize,
//         concat!("Size of: ", stringify!(Chuck_IO_File))
//     );
//     assert_eq!(
//         align_of::<Chuck_IO_File>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_IO_File))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_File>())).m_flags as *const _ as usize },
//         208usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_File),
//             "::",
//             stringify!(m_flags)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_File>())).m_iomode as *const _ as usize },
//         216usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_File),
//             "::",
//             stringify!(m_iomode)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_File>())).m_io as *const _ as usize },
//         224usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_File),
//             "::",
//             stringify!(m_io)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_File>())).m_dir as *const _ as usize },
//         752usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_File),
//             "::",
//             stringify!(m_dir)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_File>())).m_dir_start as *const _ as usize },
//         760usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_File),
//             "::",
//             stringify!(m_dir_start)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_File>())).m_path as *const _ as usize },
//         768usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_File),
//             "::",
//             stringify!(m_path)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_File>())).m_vmRef as *const _ as usize },
//         800usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_File),
//             "::",
//             stringify!(m_vmRef)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}writeStr_thread"]
    pub fn Chuck_IO_File_writeStr_thread(data: *mut c_void) -> self::root::THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeInt_thread"]
    pub fn Chuck_IO_File_writeInt_thread(data: *mut c_void) -> self::root::THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeFloat_thread"]
    pub fn Chuck_IO_File_writeFloat_thread(data: *mut c_void) -> self::root::THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File"]
    pub fn Chuck_IO_File_Chuck_IO_File(
        this: *mut self::root::Chuck_IO_File,
        vm: *mut self::root::Chuck_VM,
    );
}
impl Default for Chuck_IO_File {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_IO_File {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_IO_File {{ m_flags: {:?}, m_iomode: {:?}, m_io: {:?}, m_dir: {:?}, m_dir_start: {:?}, m_path: {:?}, m_vmRef: {:?} }}" , self . m_flags , self . m_iomode , self . m_io , self . m_dir , self . m_dir_start , self . m_path , self . m_vmRef )
    }
}
impl Chuck_IO_File {
    #[inline]
    pub unsafe fn writeStr_thread(data: *mut c_void) -> self::root::THREAD_RETURN {
        Chuck_IO_File_writeStr_thread(data)
    }
    #[inline]
    pub unsafe fn writeInt_thread(data: *mut c_void) -> self::root::THREAD_RETURN {
        Chuck_IO_File_writeInt_thread(data)
    }
    #[inline]
    pub unsafe fn writeFloat_thread(data: *mut c_void) -> self::root::THREAD_RETURN {
        Chuck_IO_File_writeFloat_thread(data)
    }
    #[inline]
    pub unsafe fn new(vm: *mut self::root::Chuck_VM) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_File_Chuck_IO_File(&mut __bindgen_tmp, vm);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File_destructor"]
    pub fn Chuck_IO_File_Chuck_IO_File_destructor(this: *mut self::root::Chuck_IO_File);
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn Chuck_IO_File_open(
        this: *mut c_void,
        path: *const self::root::std::__cxx11::string,
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
    pub fn Chuck_IO_File_dirList(this: *mut c_void) -> *mut self::root::Chuck_Array4;
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_File_readLine(this: *mut c_void) -> *mut self::root::Chuck_String;
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
        str: *mut self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_File_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write(this: *mut c_void, val: *const self::root::std::__cxx11::string);
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
    pub _base: self::root::Chuck_IO,
    pub m_callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_buffer: self::root::std::stringstream,
}
// #[test]
// fn bindgen_test_layout_Chuck_IO_Chout() {
//     assert_eq!(
//         size_of::<Chuck_IO_Chout>(),
//         608usize,
//         concat!("Size of: ", stringify!(Chuck_IO_Chout))
//     );
//     assert_eq!(
//         align_of::<Chuck_IO_Chout>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_IO_Chout))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_Chout>())).m_callback as *const _ as usize },
//         208usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_Chout),
//             "::",
//             stringify!(m_callback)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_Chout>())).m_buffer as *const _ as usize },
//         216usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_Chout),
//             "::",
//             stringify!(m_buffer)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Chout_set_output_callback(
        this: *mut self::root::Chuck_IO_Chout,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout"]
    pub fn Chuck_IO_Chout_Chuck_IO_Chout(
        this: *mut self::root::Chuck_IO_Chout,
        carrier: *mut self::root::Chuck_Carrier,
    );
}
impl Default for Chuck_IO_Chout {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_IO_Chout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_IO_Chout {{ m_callback: {:?}, m_buffer: {:?} }}",
            self.m_callback, self.m_buffer
        )
    }
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
    pub unsafe fn new(carrier: *mut self::root::Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_Chout_Chuck_IO_Chout(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout_destructor"]
    pub fn Chuck_IO_Chout_Chuck_IO_Chout_destructor(this: *mut self::root::Chuck_IO_Chout);
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
    pub fn Chuck_IO_Chout_readLine(this: *mut c_void) -> *mut self::root::Chuck_String;
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
        str: *mut self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Chout_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write(this: *mut c_void, val: *const self::root::std::__cxx11::string);
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
    pub _base: self::root::Chuck_IO,
    pub m_callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_buffer: self::root::std::stringstream,
}
// #[test]
// fn bindgen_test_layout_Chuck_IO_Cherr() {
//     assert_eq!(
//         size_of::<Chuck_IO_Cherr>(),
//         608usize,
//         concat!("Size of: ", stringify!(Chuck_IO_Cherr))
//     );
//     assert_eq!(
//         align_of::<Chuck_IO_Cherr>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_IO_Cherr))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_Cherr>())).m_callback as *const _ as usize },
//         208usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_Cherr),
//             "::",
//             stringify!(m_callback)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_IO_Cherr>())).m_buffer as *const _ as usize },
//         216usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_IO_Cherr),
//             "::",
//             stringify!(m_buffer)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Cherr_set_output_callback(
        this: *mut self::root::Chuck_IO_Cherr,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr"]
    pub fn Chuck_IO_Cherr_Chuck_IO_Cherr(
        this: *mut self::root::Chuck_IO_Cherr,
        carrier: *mut self::root::Chuck_Carrier,
    );
}
impl Default for Chuck_IO_Cherr {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_IO_Cherr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_IO_Cherr {{ m_callback: {:?}, m_buffer: {:?} }}",
            self.m_callback, self.m_buffer
        )
    }
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
    pub unsafe fn new(carrier: *mut self::root::Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_Cherr_Chuck_IO_Cherr(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr_destructor"]
    pub fn Chuck_IO_Cherr_Chuck_IO_Cherr_destructor(this: *mut self::root::Chuck_IO_Cherr);
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
    pub fn Chuck_IO_Cherr_readLine(this: *mut c_void) -> *mut self::root::Chuck_String;
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
        str: *mut self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Cherr_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write(this: *mut c_void, val: *const self::root::std::__cxx11::string);
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_UAnaBlobProxy {
    _unused: [u8; 0],
}
pub mod Chuck_DL_Api {

    pub type Object = *mut c_void;
    pub type Type = *mut c_void;
    pub type String = *mut c_void;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Api {
        pub vm: *mut self::root::Chuck_DL_Api::Api_VMApi,
        pub object: *mut self::root::Chuck_DL_Api::Api_ObjectApi,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Api_VMApi {
        pub get_srate: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                arg2: *mut self::root::Chuck_VM_Shred,
            ) -> c_ulong,
        >,
    }
    // #[test]
    // fn bindgen_test_layout_Api_VMApi() {
    //     assert_eq!(
    //         size_of::<Api_VMApi>(),
    //         8usize,
    //         concat!("Size of: ", stringify!(Api_VMApi))
    //     );
    //     assert_eq!(
    //         align_of::<Api_VMApi>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(Api_VMApi))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<Api_VMApi>())).get_srate as *const _ as usize },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_VMApi),
    //             "::",
    //             stringify!(get_srate)
    //         )
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}VMApi"]
        pub fn Api_VMApi_VMApi(this: *mut self::root::Chuck_DL_Api::Api_VMApi);
    }
    impl Default for Api_VMApi {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Api_VMApi {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = uninitialized();
            Api_VMApi_VMApi(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Api_ObjectApi {
        pub get_type: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                arg2: *mut self::root::Chuck_VM_Shred,
                name: *mut self::root::std::__cxx11::string,
            ) -> self::root::Chuck_DL_Api::Type,
        >,
        pub create: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                arg2: *mut self::root::Chuck_VM_Shred,
                type_: self::root::Chuck_DL_Api::Type,
            ) -> self::root::Chuck_DL_Api::Object,
        >,
        pub create_string: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                arg2: *mut self::root::Chuck_VM_Shred,
                value: *mut self::root::std::__cxx11::string,
            ) -> self::root::Chuck_DL_Api::String,
        >,
        pub get_mvar_int: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                object: self::root::Chuck_DL_Api::Object,
                name: *mut self::root::std::__cxx11::string,
                value: *mut c_long,
            ) -> c_ulong,
        >,
        pub get_mvar_float: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                object: self::root::Chuck_DL_Api::Object,
                name: *mut self::root::std::__cxx11::string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_dur: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                object: self::root::Chuck_DL_Api::Object,
                name: *mut self::root::std::__cxx11::string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_time: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                object: self::root::Chuck_DL_Api::Object,
                name: *mut self::root::std::__cxx11::string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_string: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                object: self::root::Chuck_DL_Api::Object,
                name: *mut self::root::std::__cxx11::string,
                value: *mut self::root::Chuck_DL_Api::String,
            ) -> c_ulong,
        >,
        pub get_mvar_object: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                object: self::root::Chuck_DL_Api::Object,
                name: *mut self::root::std::__cxx11::string,
                value: *mut self::root::Chuck_DL_Api::Object,
            ) -> c_ulong,
        >,
        pub set_string: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: self::root::CK_DL_API,
                string: self::root::Chuck_DL_Api::String,
                value: *mut self::root::std::__cxx11::string,
            ) -> c_ulong,
        >,
    }
    // #[test]
    // fn bindgen_test_layout_Api_ObjectApi() {
    //     assert_eq!(
    //         size_of::<Api_ObjectApi>(),
    //         80usize,
    //         concat!("Size of: ", stringify!(Api_ObjectApi))
    //     );
    //     assert_eq!(
    //         align_of::<Api_ObjectApi>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(Api_ObjectApi))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<Api_ObjectApi>())).get_type as *const _ as usize },
    //         0usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(get_type)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<Api_ObjectApi>())).create as *const _ as usize },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(create)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<Api_ObjectApi>())).create_string as *const _ as usize
    //         },
    //         16usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(create_string)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<Api_ObjectApi>())).get_mvar_int as *const _ as usize
    //         },
    //         24usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(get_mvar_int)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<Api_ObjectApi>())).get_mvar_float as *const _ as usize
    //         },
    //         32usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(get_mvar_float)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<Api_ObjectApi>())).get_mvar_dur as *const _ as usize
    //         },
    //         40usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(get_mvar_dur)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<Api_ObjectApi>())).get_mvar_time as *const _ as usize
    //         },
    //         48usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(get_mvar_time)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<Api_ObjectApi>())).get_mvar_string as *const _ as usize
    //         },
    //         56usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(get_mvar_string)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<Api_ObjectApi>())).get_mvar_object as *const _ as usize
    //         },
    //         64usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(get_mvar_object)
    //         )
    //     );
    //     assert_eq!(
    //         unsafe {
    //             &(*(::std::ptr::null::<Api_ObjectApi>())).set_string as *const _ as usize
    //         },
    //         72usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api_ObjectApi),
    //             "::",
    //             stringify!(set_string)
    //         )
    //     );
    // }
    extern "C" {
        #[link_name = "\u{1}ObjectApi"]
        pub fn Api_ObjectApi_ObjectApi(this: *mut self::root::Chuck_DL_Api::Api_ObjectApi);
    }
    impl Default for Api_ObjectApi {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Api_ObjectApi {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = uninitialized();
            Api_ObjectApi_ObjectApi(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}g_api"]
        pub static mut Api_g_api: self::root::Chuck_DL_Api::Api;
    }
    // #[test]
    // fn bindgen_test_layout_Api() {
    //     assert_eq!(
    //         size_of::<Api>(),
    //         16usize,
    //         concat!("Size of: ", stringify!(Api))
    //     );
    //     assert_eq!(
    //         align_of::<Api>(),
    //         8usize,
    //         concat!("Alignment of ", stringify!(Api))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<Api>())).vm as *const _ as usize },
    //         0usize,
    //         concat!("Offset of field: ", stringify!(Api), "::", stringify!(vm))
    //     );
    //     assert_eq!(
    //         unsafe { &(*(::std::ptr::null::<Api>())).object as *const _ as usize },
    //         8usize,
    //         concat!(
    //             "Offset of field: ",
    //             stringify!(Api),
    //             "::",
    //             stringify!(object)
    //         )
    //     );
    // }
    impl Default for Api {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
}
pub type CK_DL_API = *const self::root::Chuck_DL_Api::Api;
pub type f_ck_declversion = ::std::option::Option<unsafe extern "C" fn() -> c_ulong>;
pub type f_ck_query =
    ::std::option::Option<unsafe extern "C" fn(QUERY: *mut self::root::Chuck_DL_Query) -> c_ulong>;
pub type f_alloc = ::std::option::Option<
    unsafe extern "C" fn(
        VM: *mut self::root::Chuck_VM,
        SHRED: *mut self::root::Chuck_VM_Shred,
        API: self::root::CK_DL_API,
    ) -> *mut self::root::Chuck_Object,
>;
pub type f_ctor = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        ARGS: *mut c_void,
        VM: *mut self::root::Chuck_VM,
        SHRED: *mut self::root::Chuck_VM_Shred,
        API: self::root::CK_DL_API,
    ),
>;
pub type f_dtor = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        VM: *mut self::root::Chuck_VM,
        SHRED: *mut self::root::Chuck_VM_Shred,
        API: self::root::CK_DL_API,
    ),
>;
pub type f_mfun = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut self::root::Chuck_DL_Return,
        VM: *mut self::root::Chuck_VM,
        SHRED: *mut self::root::Chuck_VM_Shred,
        API: self::root::CK_DL_API,
    ),
>;
pub type f_sfun = ::std::option::Option<
    unsafe extern "C" fn(
        ARGS: *mut c_void,
        RETURN: *mut self::root::Chuck_DL_Return,
        VM: *mut self::root::Chuck_VM,
        SHRED: *mut self::root::Chuck_VM_Shred,
        API: self::root::CK_DL_API,
    ),
>;
pub type f_tick = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        in_: f64,
        out: *mut f64,
        API: self::root::CK_DL_API,
    ) -> c_ulong,
>;
pub type f_tickf = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        in_: *mut f64,
        out: *mut f64,
        nframes: c_ulong,
        API: self::root::CK_DL_API,
    ) -> c_ulong,
>;
pub type f_ctrl = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut self::root::Chuck_DL_Return,
        VM: *mut self::root::Chuck_VM,
        SHRED: *mut self::root::Chuck_VM_Shred,
        API: self::root::CK_DL_API,
    ),
>;
pub type f_cget = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        ARGS: *mut c_void,
        RETURN: *mut self::root::Chuck_DL_Return,
        VM: *mut self::root::Chuck_VM,
        SHRED: *mut self::root::Chuck_VM_Shred,
        API: self::root::CK_DL_API,
    ),
>;
pub type f_pmsg = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        MSG: *const c_char,
        ARGS: *mut c_void,
        VM: *mut self::root::Chuck_VM,
        SHRED: *mut self::root::Chuck_VM_Shred,
        API: self::root::CK_DL_API,
    ) -> c_ulong,
>;
pub type f_tock = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut self::root::Chuck_Object,
        UANA: *mut self::root::Chuck_UAna,
        BLOB: *mut self::root::Chuck_UAnaBlobProxy,
        API: self::root::CK_DL_API,
    ) -> c_ulong,
>;
pub type f_setname = ::std::option::Option<
    unsafe extern "C" fn(query: *mut self::root::Chuck_DL_Query, name: *const c_char),
>;
pub type f_begin_class = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        name: *const c_char,
        parent: *const c_char,
    ),
>;
pub type f_add_ctor = ::std::option::Option<
    unsafe extern "C" fn(query: *mut self::root::Chuck_DL_Query, ctor: self::root::f_ctor),
>;
pub type f_add_dtor = ::std::option::Option<
    unsafe extern "C" fn(query: *mut self::root::Chuck_DL_Query, dtor: self::root::f_dtor),
>;
pub type f_add_mfun = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        mfun: self::root::f_mfun,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_add_sfun = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        sfun: self::root::f_sfun,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_add_mvar = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
    ) -> c_ulong,
>;
pub type f_add_svar = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
        static_addr: *mut c_void,
    ),
>;
pub type f_add_arg = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_add_ugen_func = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        tick: self::root::f_tick,
        pmsg: self::root::f_pmsg,
        num_in: c_ulong,
        num_out: c_ulong,
    ),
>;
pub type f_add_ugen_funcf = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        tickf: self::root::f_tickf,
        pmsg: self::root::f_pmsg,
        num_in: c_ulong,
        num_out: c_ulong,
    ),
>;
pub type f_add_ugen_funcf_auto_num_channels = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        tickf: self::root::f_tickf,
        psmg: self::root::f_pmsg,
    ),
>;
pub type f_add_ugen_ctrl = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut self::root::Chuck_DL_Query,
        ctrl: self::root::f_ctrl,
        cget: self::root::f_cget,
        type_: *const c_char,
        name: *const c_char,
    ),
>;
pub type f_end_class =
    ::std::option::Option<unsafe extern "C" fn(query: *mut self::root::Chuck_DL_Query) -> c_ulong>;
pub type f_doc_class = ::std::option::Option<
    unsafe extern "C" fn(query: *mut self::root::Chuck_DL_Query, doc: *const c_char) -> c_ulong,
>;
pub type f_add_example = ::std::option::Option<
    unsafe extern "C" fn(query: *mut self::root::Chuck_DL_Query, ex: *const c_char) -> c_ulong,
>;
pub type f_doc_func = ::std::option::Option<
    unsafe extern "C" fn(query: *mut self::root::Chuck_DL_Query, doc: *const c_char) -> c_ulong,
>;
pub type f_doc_var = ::std::option::Option<
    unsafe extern "C" fn(query: *mut self::root::Chuck_DL_Query, doc: *const c_char) -> c_ulong,
>;
#[repr(C)]
pub struct Chuck_DL_Query {
    pub m_carrier: *mut self::root::Chuck_Carrier,
    pub setname: self::root::f_setname,
    pub begin_class: self::root::f_begin_class,
    pub add_ctor: self::root::f_add_ctor,
    pub add_dtor: self::root::f_add_dtor,
    pub add_mfun: self::root::f_add_mfun,
    pub add_sfun: self::root::f_add_sfun,
    pub add_mvar: self::root::f_add_mvar,
    pub add_svar: self::root::f_add_svar,
    pub add_arg: self::root::f_add_arg,
    pub add_ugen_func: self::root::f_add_ugen_func,
    pub add_ugen_funcf: self::root::f_add_ugen_funcf,
    pub add_ugen_funcf_auto_num_channels: self::root::f_add_ugen_funcf_auto_num_channels,
    pub add_ugen_ctrl: self::root::f_add_ugen_ctrl,
    pub end_class: self::root::f_end_class,
    pub last_var: *mut self::root::Chuck_DL_Value,
    pub doc_class: self::root::f_doc_class,
    pub doc_func: self::root::f_doc_func,
    pub doc_var: self::root::f_doc_var,
    pub add_ex: self::root::f_add_example,
    pub dll_ref: *mut self::root::Chuck_DLL,
    pub reserved: *mut c_void,
    pub srate: c_ulong,
    pub linepos: c_int,
    pub dll_name: self::root::std::__cxx11::string,
    pub curr_class: *mut self::root::Chuck_DL_Class,
    pub curr_func: *mut self::root::Chuck_DL_Func,
    pub name: self::root::std::__cxx11::string,
    pub classes: self::root::std::vector,
    pub stack: self::root::std::vector,
}
// #[test]
// fn bindgen_test_layout_Chuck_DL_Query() {
//     assert_eq!(
//         size_of::<Chuck_DL_Query>(),
//         320usize,
//         concat!("Size of: ", stringify!(Chuck_DL_Query))
//     );
//     assert_eq!(
//         align_of::<Chuck_DL_Query>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_DL_Query))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).m_carrier as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(m_carrier)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).setname as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(setname)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).begin_class as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(begin_class)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).add_ctor as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_ctor)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).add_dtor as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_dtor)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).add_mfun as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_mfun)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).add_sfun as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_sfun)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).add_mvar as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_mvar)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).add_svar as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_svar)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).add_arg as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_arg)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Query>())).add_ugen_func as *const _ as usize
//         },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_ugen_func)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Query>())).add_ugen_funcf as *const _ as usize
//         },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_ugen_funcf)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Query>())).add_ugen_funcf_auto_num_channels
//                 as *const _ as usize
//         },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_ugen_funcf_auto_num_channels)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Query>())).add_ugen_ctrl as *const _ as usize
//         },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_ugen_ctrl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).end_class as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(end_class)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).last_var as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(last_var)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).doc_class as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(doc_class)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).doc_func as *const _ as usize },
//         136usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(doc_func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).doc_var as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(doc_var)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).add_ex as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(add_ex)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).dll_ref as *const _ as usize },
//         160usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(dll_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).reserved as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(reserved)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).srate as *const _ as usize },
//         176usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(srate)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).linepos as *const _ as usize },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(linepos)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).dll_name as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(dll_name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).curr_class as *const _ as usize },
//         224usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(curr_class)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).curr_func as *const _ as usize },
//         232usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(curr_func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).name as *const _ as usize },
//         240usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).classes as *const _ as usize },
//         272usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(classes)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Query>())).stack as *const _ as usize },
//         296usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Query),
//             "::",
//             stringify!(stack)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_DL_Query_clear(this: *mut self::root::Chuck_DL_Query);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Query"]
    pub fn Chuck_DL_Query_Chuck_DL_Query(
        this: *mut self::root::Chuck_DL_Query,
        carrier: *mut self::root::Chuck_Carrier,
    );
}
impl Default for Chuck_DL_Query {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Query {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DL_Query {{ m_carrier: {:?}, setname: {:?}, begin_class: {:?}, add_ctor: {:?}, add_dtor: {:?}, add_mfun: {:?}, add_sfun: {:?}, add_mvar: {:?}, add_svar: {:?}, add_arg: {:?}, add_ugen_func: {:?}, add_ugen_funcf: {:?}, add_ugen_funcf_auto_num_channels: {:?}, add_ugen_ctrl: {:?}, end_class: {:?}, last_var: {:?}, doc_class: {:?}, doc_func: {:?}, doc_var: {:?}, add_ex: {:?}, dll_ref: {:?}, reserved: {:?}, srate: {:?}, linepos: {:?}, dll_name: {:?}, curr_class: {:?}, curr_func: {:?}, name: {:?}, classes: {:?}, stack: {:?} }}" , self . m_carrier , self . setname , self . begin_class , self . add_ctor , self . add_dtor , self . add_mfun , self . add_sfun , self . add_mvar , self . add_svar , self . add_arg , self . add_ugen_func , self . add_ugen_funcf , self . add_ugen_funcf_auto_num_channels , self . add_ugen_ctrl , self . end_class , self . last_var , self . doc_class , self . doc_func , self . doc_var , self . add_ex , self . dll_ref , self . reserved , self . srate , self . linepos , self . dll_name , self . curr_class , self . curr_func , self . name , self . classes , self . stack )
    }
}
impl Chuck_DL_Query {
    #[inline]
    pub unsafe fn clear(&mut self) {
        Chuck_DL_Query_clear(self)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut self::root::Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_DL_Query_Chuck_DL_Query(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DL_Class {
    pub name: self::root::std::__cxx11::string,
    pub parent: self::root::std::__cxx11::string,
    pub ctors: self::root::std::vector,
    pub dtor: *mut self::root::Chuck_DL_Func,
    pub mfuns: self::root::std::vector,
    pub sfuns: self::root::std::vector,
    pub mvars: self::root::std::vector,
    pub svars: self::root::std::vector,
    pub ugen_tick: self::root::f_tick,
    pub ugen_tickf: self::root::f_tickf,
    pub ugen_pmsg: self::root::f_pmsg,
    pub ugen_ctrl: self::root::std::vector,
    pub uana_tock: self::root::f_tock,
    pub classes: self::root::std::vector,
    pub current_mvar_offset: c_ulong,
    pub ugen_num_in: c_ulong,
    pub ugen_num_out: c_ulong,
    pub doc: self::root::std::__cxx11::string,
    pub examples: self::root::std::vector,
}
// #[test]
// fn bindgen_test_layout_Chuck_DL_Class() {
//     assert_eq!(
//         size_of::<Chuck_DL_Class>(),
//         352usize,
//         concat!("Size of: ", stringify!(Chuck_DL_Class))
//     );
//     assert_eq!(
//         align_of::<Chuck_DL_Class>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_DL_Class))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).name as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).parent as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(parent)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).ctors as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(ctors)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).dtor as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(dtor)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).mfuns as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(mfuns)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).sfuns as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(sfuns)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).mvars as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(mvars)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).svars as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(svars)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).ugen_tick as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(ugen_tick)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).ugen_tickf as *const _ as usize },
//         200usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(ugen_tickf)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).ugen_pmsg as *const _ as usize },
//         208usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(ugen_pmsg)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).ugen_ctrl as *const _ as usize },
//         216usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(ugen_ctrl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).uana_tock as *const _ as usize },
//         240usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(uana_tock)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).classes as *const _ as usize },
//         248usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(classes)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Class>())).current_mvar_offset as *const _ as usize
//         },
//         272usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(current_mvar_offset)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).ugen_num_in as *const _ as usize },
//         280usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(ugen_num_in)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).ugen_num_out as *const _ as usize },
//         288usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(ugen_num_out)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).doc as *const _ as usize },
//         296usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(doc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Class>())).examples as *const _ as usize },
//         328usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Class),
//             "::",
//             stringify!(examples)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Class_destructor"]
    pub fn Chuck_DL_Class_Chuck_DL_Class_destructor(this: *mut self::root::Chuck_DL_Class);
}
impl Default for Chuck_DL_Class {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Class {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DL_Class {{ name: {:?}, parent: {:?}, ctors: {:?}, dtor: {:?}, mfuns: {:?}, sfuns: {:?}, mvars: {:?}, svars: {:?}, ugen_tick: {:?}, ugen_tickf: {:?}, ugen_pmsg: {:?}, ugen_ctrl: {:?}, uana_tock: {:?}, classes: {:?}, current_mvar_offset: {:?}, ugen_num_in: {:?}, ugen_num_out: {:?}, doc: {:?}, examples: {:?} }}" , self . name , self . parent , self . ctors , self . dtor , self . mfuns , self . sfuns , self . mvars , self . svars , self . ugen_tick , self . ugen_tickf , self . ugen_pmsg , self . ugen_ctrl , self . uana_tock , self . classes , self . current_mvar_offset , self . ugen_num_in , self . ugen_num_out , self . doc , self . examples )
    }
}
impl Chuck_DL_Class {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_DL_Class_Chuck_DL_Class_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_DL_Value {
    pub name: self::root::std::__cxx11::string,
    pub type_: self::root::std::__cxx11::string,
    pub is_const: c_ulong,
    pub static_addr: *mut c_void,
    pub doc: self::root::std::__cxx11::string,
}
// #[test]
// fn bindgen_test_layout_Chuck_DL_Value() {
//     assert_eq!(
//         size_of::<Chuck_DL_Value>(),
//         112usize,
//         concat!("Size of: ", stringify!(Chuck_DL_Value))
//     );
//     assert_eq!(
//         align_of::<Chuck_DL_Value>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_DL_Value))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Value>())).name as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Value),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Value>())).type_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Value),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Value>())).is_const as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Value),
//             "::",
//             stringify!(is_const)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Value>())).static_addr as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Value),
//             "::",
//             stringify!(static_addr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Value>())).doc as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Value),
//             "::",
//             stringify!(doc)
//         )
//     );
// }
impl Default for Chuck_DL_Value {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DL_Value {{ name: {:?}, type: {:?}, is_const: {:?}, static_addr: {:?}, doc: {:?} }}" , self . name , self . type_ , self . is_const , self . static_addr , self . doc )
    }
}
#[repr(C)]
pub struct Chuck_DL_Func {
    pub name: self::root::std::__cxx11::string,
    pub type_: self::root::std::__cxx11::string,
    pub __bindgen_anon_1: self::root::Chuck_DL_Func__bindgen_ty_1,
    pub args: self::root::std::vector,
    pub doc: self::root::std::__cxx11::string,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Chuck_DL_Func__bindgen_ty_1 {
    pub ctor: self::root::f_ctor,
    pub dtor: self::root::f_dtor,
    pub mfun: self::root::f_mfun,
    pub sfun: self::root::f_sfun,
    pub addr: c_ulong,
    _bindgen_union_align: u64,
}
// #[test]
// fn bindgen_test_layout_Chuck_DL_Func__bindgen_ty_1() {
//     assert_eq!(
//         size_of::<Chuck_DL_Func__bindgen_ty_1>(),
//         8usize,
//         concat!("Size of: ", stringify!(Chuck_DL_Func__bindgen_ty_1))
//     );
//     assert_eq!(
//         align_of::<Chuck_DL_Func__bindgen_ty_1>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_DL_Func__bindgen_ty_1))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Func__bindgen_ty_1>())).ctor as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func__bindgen_ty_1),
//             "::",
//             stringify!(ctor)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Func__bindgen_ty_1>())).dtor as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func__bindgen_ty_1),
//             "::",
//             stringify!(dtor)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Func__bindgen_ty_1>())).mfun as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func__bindgen_ty_1),
//             "::",
//             stringify!(mfun)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Func__bindgen_ty_1>())).sfun as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func__bindgen_ty_1),
//             "::",
//             stringify!(sfun)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_DL_Func__bindgen_ty_1>())).addr as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func__bindgen_ty_1),
//             "::",
//             stringify!(addr)
//         )
//     );
// }
impl Default for Chuck_DL_Func__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Func__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_DL_Func__bindgen_ty_1 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout_Chuck_DL_Func() {
//     assert_eq!(
//         size_of::<Chuck_DL_Func>(),
//         128usize,
//         concat!("Size of: ", stringify!(Chuck_DL_Func))
//     );
//     assert_eq!(
//         align_of::<Chuck_DL_Func>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_DL_Func))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Func>())).name as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Func>())).type_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Func>())).args as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func),
//             "::",
//             stringify!(args)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Func>())).doc as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Func),
//             "::",
//             stringify!(doc)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func_destructor"]
    pub fn Chuck_DL_Func_Chuck_DL_Func_destructor(this: *mut self::root::Chuck_DL_Func);
}
impl Default for Chuck_DL_Func {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Func {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DL_Func {{ name: {:?}, type: {:?}, __bindgen_anon_1: {:?}, args: {:?}, doc: {:?} }}" , self . name , self . type_ , self . __bindgen_anon_1 , self . args , self . doc )
    }
}
impl Chuck_DL_Func {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_DL_Func_Chuck_DL_Func_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_DL_Ctrl {
    pub name: self::root::std::__cxx11::string,
    pub type_: self::root::std::__cxx11::string,
    pub types: self::root::std::vector,
    pub ctrl: self::root::f_ctrl,
    pub cget: self::root::f_cget,
}
// #[test]
// fn bindgen_test_layout_Chuck_DL_Ctrl() {
//     assert_eq!(
//         size_of::<Chuck_DL_Ctrl>(),
//         104usize,
//         concat!("Size of: ", stringify!(Chuck_DL_Ctrl))
//     );
//     assert_eq!(
//         align_of::<Chuck_DL_Ctrl>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_DL_Ctrl))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Ctrl>())).name as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Ctrl),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Ctrl>())).type_ as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Ctrl),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Ctrl>())).types as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Ctrl),
//             "::",
//             stringify!(types)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Ctrl>())).ctrl as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Ctrl),
//             "::",
//             stringify!(ctrl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Ctrl>())).cget as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Ctrl),
//             "::",
//             stringify!(cget)
//         )
//     );
// }
impl Default for Chuck_DL_Ctrl {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Ctrl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_DL_Ctrl {{ name: {:?}, type: {:?}, types: {:?}, ctrl: {:?}, cget: {:?} }}",
            self.name, self.type_, self.types, self.ctrl, self.cget
        )
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Chuck_DL_Return {
    pub v_int: c_long,
    pub v_uint: c_ulong,
    pub v_float: f64,
    pub v_dur: f64,
    pub v_time: f64,
    pub v_complex: self::root::t_CKCOMPLEX,
    pub v_polar: self::root::t_CKPOLAR,
    pub v_vec3: self::root::t_CKVEC3,
    pub v_vec4: self::root::t_CKVEC4,
    pub v_object: *mut self::root::Chuck_Object,
    pub v_string: *mut self::root::Chuck_String,
    _bindgen_union_align: [u64; 4usize],
}
// #[test]
// fn bindgen_test_layout_Chuck_DL_Return() {
//     assert_eq!(
//         size_of::<Chuck_DL_Return>(),
//         32usize,
//         concat!("Size of: ", stringify!(Chuck_DL_Return))
//     );
//     assert_eq!(
//         align_of::<Chuck_DL_Return>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_DL_Return))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_int as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_int)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_uint as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_uint)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_float as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_float)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_dur as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_dur)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_time as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_time)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_complex as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_complex)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_polar as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_polar)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_vec3 as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_vec3)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_vec4 as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_vec4)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_object as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_object)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DL_Return>())).v_string as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DL_Return),
//             "::",
//             stringify!(v_string)
//         )
//     );
// }
impl Default for Chuck_DL_Return {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Return {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_DL_Return {{ union }}")
    }
}
#[repr(C)]
pub struct Chuck_DLL {
    pub _base: self::root::Chuck_VM_Object,
    pub m_handle: *mut c_void,
    pub m_last_error: self::root::std::__cxx11::string,
    pub m_filename: self::root::std::__cxx11::string,
    pub m_id: self::root::std::__cxx11::string,
    pub m_func: self::root::std::__cxx11::string,
    pub m_done_query: c_ulong,
    pub m_version_func: self::root::f_ck_declversion,
    pub m_query_func: self::root::f_ck_query,
    pub m_query: self::root::Chuck_DL_Query,
}
// #[test]
// fn bindgen_test_layout_Chuck_DLL() {
//     assert_eq!(
//         size_of::<Chuck_DLL>(),
//         520usize,
//         concat!("Size of: ", stringify!(Chuck_DLL))
//     );
//     assert_eq!(
//         align_of::<Chuck_DLL>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_DLL))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_handle as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_handle)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_last_error as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_last_error)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_filename as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_filename)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_id as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_id)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_func as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_done_query as *const _ as usize },
//         176usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_done_query)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_version_func as *const _ as usize },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_version_func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_query_func as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_query_func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_DLL>())).m_query as *const _ as usize },
//         200usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_DLL),
//             "::",
//             stringify!(m_query)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}load"]
    pub fn Chuck_DLL_load(
        this: *mut self::root::Chuck_DLL,
        filename: *const c_char,
        func: *const c_char,
        lazy: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}load"]
    pub fn Chuck_DLL_load1(
        this: *mut self::root::Chuck_DLL,
        query_func: self::root::f_ck_query,
        lazy: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_addr"]
    pub fn Chuck_DLL_get_addr(
        this: *mut self::root::Chuck_DLL,
        symbol: *const c_char,
    ) -> *mut c_void;
}
extern "C" {
    #[link_name = "\u{1}last_error"]
    pub fn Chuck_DLL_last_error(this: *const self::root::Chuck_DLL) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}unload"]
    pub fn Chuck_DLL_unload(this: *mut self::root::Chuck_DLL) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}query"]
    pub fn Chuck_DLL_query(this: *mut self::root::Chuck_DLL) -> *const self::root::Chuck_DL_Query;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_DLL_good(this: *const self::root::Chuck_DLL) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}name"]
    pub fn Chuck_DLL_name(this: *const self::root::Chuck_DLL) -> *const c_char;
}
impl Default for Chuck_DLL {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DLL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DLL {{ m_handle: {:?}, m_last_error: {:?}, m_filename: {:?}, m_id: {:?}, m_func: {:?}, m_done_query: {:?}, m_version_func: {:?}, m_query_func: {:?}, m_query: {:?} }}" , self . m_handle , self . m_last_error , self . m_filename , self . m_id , self . m_func , self . m_done_query , self . m_version_func , self . m_query_func , self . m_query )
    }
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
    pub unsafe fn load1(&mut self, query_func: self::root::f_ck_query, lazy: c_ulong) -> c_ulong {
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
    pub unsafe fn query(&mut self) -> *const self::root::Chuck_DL_Query {
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
}
pub const te_Type_te_none: self::root::te_Type = 0;
pub const te_Type_te_int: self::root::te_Type = 1;
pub const te_Type_te_uint: self::root::te_Type = 2;
pub const te_Type_te_single: self::root::te_Type = 3;
pub const te_Type_te_float: self::root::te_Type = 4;
pub const te_Type_te_double: self::root::te_Type = 5;
pub const te_Type_te_time: self::root::te_Type = 6;
pub const te_Type_te_dur: self::root::te_Type = 7;
pub const te_Type_te_complex: self::root::te_Type = 8;
pub const te_Type_te_polar: self::root::te_Type = 9;
pub const te_Type_te_string: self::root::te_Type = 10;
pub const te_Type_te_thread: self::root::te_Type = 11;
pub const te_Type_te_shred: self::root::te_Type = 12;
pub const te_Type_te_class: self::root::te_Type = 13;
pub const te_Type_te_function: self::root::te_Type = 14;
pub const te_Type_te_object: self::root::te_Type = 15;
pub const te_Type_te_user: self::root::te_Type = 16;
pub const te_Type_te_array: self::root::te_Type = 17;
pub const te_Type_te_null: self::root::te_Type = 18;
pub const te_Type_te_ugen: self::root::te_Type = 19;
pub const te_Type_te_uana: self::root::te_Type = 20;
pub const te_Type_te_event: self::root::te_Type = 21;
pub const te_Type_te_void: self::root::te_Type = 22;
pub const te_Type_te_stdout: self::root::te_Type = 23;
pub const te_Type_te_stderr: self::root::te_Type = 24;
pub const te_Type_te_adc: self::root::te_Type = 25;
pub const te_Type_te_dac: self::root::te_Type = 26;
pub const te_Type_te_bunghole: self::root::te_Type = 27;
pub const te_Type_te_uanablob: self::root::te_Type = 28;
pub const te_Type_te_io: self::root::te_Type = 29;
pub const te_Type_te_fileio: self::root::te_Type = 30;
pub const te_Type_te_chout: self::root::te_Type = 31;
pub const te_Type_te_cherr: self::root::te_Type = 32;
pub const te_Type_te_multi: self::root::te_Type = 33;
pub const te_Type_te_vec3: self::root::te_Type = 34;
pub const te_Type_te_vec4: self::root::te_Type = 35;
pub const te_Type_te_vector: self::root::te_Type = 36;
pub type te_Type = u32;
#[repr(C)]
pub struct Chuck_Scope {
    pub scope: self::root::std::vector,
    pub commit_map: self::root::std::map,
}
impl Default for Chuck_Scope {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Scope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Scope {{ scope: {:?}, commit_map: {:?} }}",
            self.scope, self.commit_map
        )
    }
}
impl PartialEq for Chuck_Scope {
    fn eq(&self, other: &Chuck_Scope) -> bool {
        self.scope == other.scope && self.commit_map == other.commit_map
    }
}
#[repr(C)]
pub struct Chuck_Namespace {
    pub _base: self::root::Chuck_VM_Object,
    pub type_: self::root::Chuck_Scope,
    pub value: self::root::Chuck_Scope,
    pub func: self::root::Chuck_Scope,
    pub obj_v_table: self::root::Chuck_VTable,
    pub class_data: *mut c_uchar,
    pub class_data_size: c_ulong,
    pub name: self::root::std::__cxx11::string,
    pub pre_ctor: *mut self::root::Chuck_VM_Code,
    pub dtor: *mut self::root::Chuck_VM_Code,
    pub parent: *mut self::root::Chuck_Namespace,
    pub offset: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Chuck_Namespace() {
//     assert_eq!(
//         size_of::<Chuck_Namespace>(),
//         360usize,
//         concat!("Size of: ", stringify!(Chuck_Namespace))
//     );
//     assert_eq!(
//         align_of::<Chuck_Namespace>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Namespace))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).type_ as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).value as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(value)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).func as *const _ as usize },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).obj_v_table as *const _ as usize },
//         256usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(obj_v_table)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).class_data as *const _ as usize },
//         280usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(class_data)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Namespace>())).class_data_size as *const _ as usize
//         },
//         288usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(class_data_size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).name as *const _ as usize },
//         296usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).pre_ctor as *const _ as usize },
//         328usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(pre_ctor)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).dtor as *const _ as usize },
//         336usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(dtor)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).parent as *const _ as usize },
//         344usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(parent)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Namespace>())).offset as *const _ as usize },
//         352usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Namespace),
//             "::",
//             stringify!(offset)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}lookup_type"]
    pub fn Chuck_Namespace_lookup_type(
        this: *mut self::root::Chuck_Namespace,
        name: *const self::root::std::__cxx11::string,
        climb: c_long,
    ) -> *mut self::root::Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}lookup_type"]
    pub fn Chuck_Namespace_lookup_type1(
        this: *mut self::root::Chuck_Namespace,
        name: self::root::S_Symbol,
        climb: c_long,
    ) -> *mut self::root::Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}lookup_value"]
    pub fn Chuck_Namespace_lookup_value(
        this: *mut self::root::Chuck_Namespace,
        name: *const self::root::std::__cxx11::string,
        climb: c_long,
    ) -> *mut self::root::Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}lookup_value"]
    pub fn Chuck_Namespace_lookup_value1(
        this: *mut self::root::Chuck_Namespace,
        name: self::root::S_Symbol,
        climb: c_long,
    ) -> *mut self::root::Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}lookup_func"]
    pub fn Chuck_Namespace_lookup_func(
        this: *mut self::root::Chuck_Namespace,
        name: *const self::root::std::__cxx11::string,
        climb: c_long,
    ) -> *mut self::root::Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}lookup_func"]
    pub fn Chuck_Namespace_lookup_func1(
        this: *mut self::root::Chuck_Namespace,
        name: self::root::S_Symbol,
        climb: c_long,
    ) -> *mut self::root::Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}get_types"]
    pub fn Chuck_Namespace_get_types(
        this: *mut self::root::Chuck_Namespace,
        out: *mut self::root::std::vector,
    );
}
extern "C" {
    #[link_name = "\u{1}get_values"]
    pub fn Chuck_Namespace_get_values(
        this: *mut self::root::Chuck_Namespace,
        out: *mut self::root::std::vector,
    );
}
extern "C" {
    #[link_name = "\u{1}get_funcs"]
    pub fn Chuck_Namespace_get_funcs(
        this: *mut self::root::Chuck_Namespace,
        out: *mut self::root::std::vector,
    );
}
impl Default for Chuck_Namespace {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Namespace {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Namespace {{ type: {:?}, value: {:?}, func: {:?}, obj_v_table: {:?}, class_data: {:?}, class_data_size: {:?}, name: {:?}, pre_ctor: {:?}, dtor: {:?}, parent: {:?}, offset: {:?} }}" , self . type_ , self . value , self . func , self . obj_v_table , self . class_data , self . class_data_size , self . name , self . pre_ctor , self . dtor , self . parent , self . offset )
    }
}
impl Chuck_Namespace {
    #[inline]
    pub unsafe fn lookup_type(
        &mut self,
        name: *const self::root::std::__cxx11::string,
        climb: c_long,
    ) -> *mut self::root::Chuck_Type {
        Chuck_Namespace_lookup_type(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_type1(
        &mut self,
        name: self::root::S_Symbol,
        climb: c_long,
    ) -> *mut self::root::Chuck_Type {
        Chuck_Namespace_lookup_type1(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_value(
        &mut self,
        name: *const self::root::std::__cxx11::string,
        climb: c_long,
    ) -> *mut self::root::Chuck_Value {
        Chuck_Namespace_lookup_value(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_value1(
        &mut self,
        name: self::root::S_Symbol,
        climb: c_long,
    ) -> *mut self::root::Chuck_Value {
        Chuck_Namespace_lookup_value1(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_func(
        &mut self,
        name: *const self::root::std::__cxx11::string,
        climb: c_long,
    ) -> *mut self::root::Chuck_Func {
        Chuck_Namespace_lookup_func(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_func1(
        &mut self,
        name: self::root::S_Symbol,
        climb: c_long,
    ) -> *mut self::root::Chuck_Func {
        Chuck_Namespace_lookup_func1(self, name, climb)
    }
    #[inline]
    pub unsafe fn get_types(&mut self, out: *mut self::root::std::vector) {
        Chuck_Namespace_get_types(self, out)
    }
    #[inline]
    pub unsafe fn get_values(&mut self, out: *mut self::root::std::vector) {
        Chuck_Namespace_get_values(self, out)
    }
    #[inline]
    pub unsafe fn get_funcs(&mut self, out: *mut self::root::std::vector) {
        Chuck_Namespace_get_funcs(self, out)
    }
}
#[repr(C)]
pub struct Chuck_Context {
    pub _base: self::root::Chuck_VM_Object,
    pub filename: self::root::std::__cxx11::string,
    pub full_path: self::root::std::__cxx11::string,
    pub parse_tree: self::root::a_Program,
    pub nspc: *mut self::root::Chuck_Namespace,
    pub public_class_def: self::root::a_Class_Def,
    pub has_error: c_ulong,
    pub progress: c_ulong,
    pub new_types: self::root::std::vector,
    pub new_values: self::root::std::vector,
    pub new_funcs: self::root::std::vector,
    pub new_nspc: self::root::std::vector,
    pub commit_map: self::root::std::map,
}
pub const Chuck_Context_P_NONE: self::root::Chuck_Context__bindgen_ty_1 = 0;
pub const Chuck_Context_P_CLASSES_ONLY: self::root::Chuck_Context__bindgen_ty_1 = 1;
pub const Chuck_Context_P_ALL: self::root::Chuck_Context__bindgen_ty_1 = 2;
pub type Chuck_Context__bindgen_ty_1 = u32;
// #[test]
// fn bindgen_test_layout_Chuck_Context() {
//     assert_eq!(
//         size_of::<Chuck_Context>(),
//         288usize,
//         concat!("Size of: ", stringify!(Chuck_Context))
//     );
//     assert_eq!(
//         align_of::<Chuck_Context>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Context))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).filename as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(filename)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).full_path as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(full_path)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).parse_tree as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(parse_tree)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).nspc as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(nspc)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Context>())).public_class_def as *const _ as usize
//         },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(public_class_def)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).has_error as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(has_error)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).progress as *const _ as usize },
//         136usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(progress)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).new_types as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(new_types)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).new_values as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(new_values)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).new_funcs as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(new_funcs)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).new_nspc as *const _ as usize },
//         216usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(new_nspc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Context>())).commit_map as *const _ as usize },
//         240usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Context),
//             "::",
//             stringify!(commit_map)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}add_commit_candidate"]
    pub fn Chuck_Context_add_commit_candidate(
        this: *mut self::root::Chuck_Context,
        nspc: *mut self::root::Chuck_Namespace,
    );
}
extern "C" {
    #[link_name = "\u{1}commit"]
    pub fn Chuck_Context_commit(this: *mut self::root::Chuck_Context);
}
extern "C" {
    #[link_name = "\u{1}rollback"]
    pub fn Chuck_Context_rollback(this: *mut self::root::Chuck_Context);
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Type"]
    pub fn Chuck_Context_new_Chuck_Type(
        this: *mut self::root::Chuck_Context,
        env: *mut self::root::Chuck_Env,
    ) -> *mut self::root::Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Value"]
    pub fn Chuck_Context_new_Chuck_Value(
        this: *mut self::root::Chuck_Context,
        t: *mut self::root::Chuck_Type,
        name: *const self::root::std::__cxx11::string,
    ) -> *mut self::root::Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Func"]
    pub fn Chuck_Context_new_Chuck_Func(
        this: *mut self::root::Chuck_Context,
    ) -> *mut self::root::Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Namespace"]
    pub fn Chuck_Context_new_Chuck_Namespace(
        this: *mut self::root::Chuck_Context,
    ) -> *mut self::root::Chuck_Namespace;
}
impl Default for Chuck_Context {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Context {{ filename: {:?}, full_path: {:?}, parse_tree: {:?}, nspc: {:?}, public_class_def: {:?}, has_error: {:?}, progress: {:?}, new_types: {:?}, new_values: {:?}, new_funcs: {:?}, new_nspc: {:?}, commit_map: {:?} }}" , self . filename , self . full_path , self . parse_tree , self . nspc , self . public_class_def , self . has_error , self . progress , self . new_types , self . new_values , self . new_funcs , self . new_nspc , self . commit_map )
    }
}
impl Chuck_Context {
    #[inline]
    pub unsafe fn add_commit_candidate(&mut self, nspc: *mut self::root::Chuck_Namespace) {
        Chuck_Context_add_commit_candidate(self, nspc)
    }
    #[inline]
    pub unsafe fn commit(&mut self) {
        Chuck_Context_commit(self)
    }
    #[inline]
    pub unsafe fn rollback(&mut self) {
        Chuck_Context_rollback(self)
    }
    #[inline]
    pub unsafe fn new_Chuck_Type(
        &mut self,
        env: *mut self::root::Chuck_Env,
    ) -> *mut self::root::Chuck_Type {
        Chuck_Context_new_Chuck_Type(self, env)
    }
    #[inline]
    pub unsafe fn new_Chuck_Value(
        &mut self,
        t: *mut self::root::Chuck_Type,
        name: *const self::root::std::__cxx11::string,
    ) -> *mut self::root::Chuck_Value {
        Chuck_Context_new_Chuck_Value(self, t, name)
    }
    #[inline]
    pub unsafe fn new_Chuck_Func(&mut self) -> *mut self::root::Chuck_Func {
        Chuck_Context_new_Chuck_Func(self)
    }
    #[inline]
    pub unsafe fn new_Chuck_Namespace(&mut self) -> *mut self::root::Chuck_Namespace {
        Chuck_Context_new_Chuck_Namespace(self)
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Context_destructor"]
    pub fn Chuck_Context_Chuck_Context_destructor(this: *mut self::root::Chuck_Context);
}
#[repr(C)]
pub struct Chuck_Env {
    pub _base: self::root::Chuck_VM_Object,
    pub m_carrier: *mut self::root::Chuck_Carrier,
    pub global_nspc: *mut self::root::Chuck_Namespace,
    pub global_context: self::root::Chuck_Context,
    pub user_nspc: *mut self::root::Chuck_Namespace,
    pub nspc_stack: self::root::std::vector,
    pub curr: *mut self::root::Chuck_Namespace,
    pub class_stack: self::root::std::vector,
    pub class_def: *mut self::root::Chuck_Type,
    pub func: *mut self::root::Chuck_Func,
    pub class_scope: c_ulong,
    pub contexts: self::root::std::vector,
    pub context: *mut self::root::Chuck_Context,
    pub breaks: self::root::std::vector,
    pub key_words: self::root::std::map,
    pub key_types: self::root::std::map,
    pub key_values: self::root::std::map,
    pub deprecated: self::root::std::map,
    pub deprecate_level: c_long,
    pub t_void: *mut self::root::Chuck_Type,
    pub t_int: *mut self::root::Chuck_Type,
    pub t_float: *mut self::root::Chuck_Type,
    pub t_time: *mut self::root::Chuck_Type,
    pub t_dur: *mut self::root::Chuck_Type,
    pub t_complex: *mut self::root::Chuck_Type,
    pub t_polar: *mut self::root::Chuck_Type,
    pub t_vec3: *mut self::root::Chuck_Type,
    pub t_vec4: *mut self::root::Chuck_Type,
    pub t_null: *mut self::root::Chuck_Type,
    pub t_function: *mut self::root::Chuck_Type,
    pub t_object: *mut self::root::Chuck_Type,
    pub t_array: *mut self::root::Chuck_Type,
    pub t_string: *mut self::root::Chuck_Type,
    pub t_event: *mut self::root::Chuck_Type,
    pub t_ugen: *mut self::root::Chuck_Type,
    pub t_uana: *mut self::root::Chuck_Type,
    pub t_uanablob: *mut self::root::Chuck_Type,
    pub t_shred: *mut self::root::Chuck_Type,
    pub t_io: *mut self::root::Chuck_Type,
    pub t_fileio: *mut self::root::Chuck_Type,
    pub t_chout: *mut self::root::Chuck_Type,
    pub t_cherr: *mut self::root::Chuck_Type,
    pub t_thread: *mut self::root::Chuck_Type,
    pub t_class: *mut self::root::Chuck_Type,
    pub t_dac: *mut self::root::Chuck_Type,
    pub t_adc: *mut self::root::Chuck_Type,
}
// #[test]
// fn bindgen_test_layout_Chuck_Env() {
//     assert_eq!(
//         size_of::<Chuck_Env>(),
//         904usize,
//         concat!("Size of: ", stringify!(Chuck_Env))
//     );
//     assert_eq!(
//         align_of::<Chuck_Env>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Env))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).m_carrier as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(m_carrier)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).global_nspc as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(global_nspc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).global_context as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(global_context)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).user_nspc as *const _ as usize },
//         344usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(user_nspc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).nspc_stack as *const _ as usize },
//         352usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(nspc_stack)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).curr as *const _ as usize },
//         376usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(curr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).class_stack as *const _ as usize },
//         384usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(class_stack)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).class_def as *const _ as usize },
//         408usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(class_def)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).func as *const _ as usize },
//         416usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).class_scope as *const _ as usize },
//         424usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(class_scope)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).contexts as *const _ as usize },
//         432usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(contexts)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).context as *const _ as usize },
//         456usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(context)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).breaks as *const _ as usize },
//         464usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(breaks)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).key_words as *const _ as usize },
//         488usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(key_words)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).key_types as *const _ as usize },
//         536usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(key_types)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).key_values as *const _ as usize },
//         584usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(key_values)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).deprecated as *const _ as usize },
//         632usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(deprecated)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).deprecate_level as *const _ as usize },
//         680usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(deprecate_level)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_void as *const _ as usize },
//         688usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_void)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_int as *const _ as usize },
//         696usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_int)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_float as *const _ as usize },
//         704usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_float)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_time as *const _ as usize },
//         712usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_time)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_dur as *const _ as usize },
//         720usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_dur)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_complex as *const _ as usize },
//         728usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_complex)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_polar as *const _ as usize },
//         736usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_polar)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_vec3 as *const _ as usize },
//         744usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_vec3)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_vec4 as *const _ as usize },
//         752usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_vec4)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_null as *const _ as usize },
//         760usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_null)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_function as *const _ as usize },
//         768usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_function)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_object as *const _ as usize },
//         776usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_object)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_array as *const _ as usize },
//         784usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_array)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_string as *const _ as usize },
//         792usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_string)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_event as *const _ as usize },
//         800usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_event)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_ugen as *const _ as usize },
//         808usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_ugen)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_uana as *const _ as usize },
//         816usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_uana)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_uanablob as *const _ as usize },
//         824usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_uanablob)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_shred as *const _ as usize },
//         832usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_shred)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_io as *const _ as usize },
//         840usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_io)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_fileio as *const _ as usize },
//         848usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_fileio)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_chout as *const _ as usize },
//         856usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_chout)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_cherr as *const _ as usize },
//         864usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_cherr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_thread as *const _ as usize },
//         872usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_thread)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_class as *const _ as usize },
//         880usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_class)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_dac as *const _ as usize },
//         888usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_dac)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Env>())).t_adc as *const _ as usize },
//         896usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Env),
//             "::",
//             stringify!(t_adc)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}Chuck_Env"]
    pub fn Chuck_Env_Chuck_Env(this: *mut self::root::Chuck_Env);
}
impl Default for Chuck_Env {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Env {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Env {{ m_carrier: {:?}, global_nspc: {:?}, global_context: {:?}, user_nspc: {:?}, nspc_stack: {:?}, curr: {:?}, class_stack: {:?}, class_def: {:?}, func: {:?}, class_scope: {:?}, contexts: {:?}, context: {:?}, breaks: {:?}, key_words: {:?}, key_types: {:?}, key_values: {:?}, deprecated: {:?}, deprecate_level: {:?}, t_void: {:?}, t_int: {:?}, t_float: {:?}, t_time: {:?}, t_dur: {:?}, t_complex: {:?}, t_polar: {:?}, t_vec3: {:?}, t_vec4: {:?}, t_null: {:?}, t_function: {:?}, t_object: {:?}, t_array: {:?}, t_string: {:?}, t_event: {:?}, t_ugen: {:?}, t_uana: {:?}, t_uanablob: {:?}, t_shred: {:?}, t_io: {:?}, t_fileio: {:?}, t_chout: {:?}, t_cherr: {:?}, t_thread: {:?}, t_class: {:?}, t_dac: {:?}, t_adc: {:?} }}" , self . m_carrier , self . global_nspc , self . global_context , self . user_nspc , self . nspc_stack , self . curr , self . class_stack , self . class_def , self . func , self . class_scope , self . contexts , self . context , self . breaks , self . key_words , self . key_types , self . key_values , self . deprecated , self . deprecate_level , self . t_void , self . t_int , self . t_float , self . t_time , self . t_dur , self . t_complex , self . t_polar , self . t_vec3 , self . t_vec4 , self . t_null , self . t_function , self . t_object , self . t_array , self . t_string , self . t_event , self . t_ugen , self . t_uana , self . t_uanablob , self . t_shred , self . t_io , self . t_fileio , self . t_chout , self . t_cherr , self . t_thread , self . t_class , self . t_dac , self . t_adc )
    }
}
impl Chuck_Env {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Env_Chuck_Env(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Env_destructor"]
    pub fn Chuck_Env_Chuck_Env_destructor(this: *mut self::root::Chuck_Env);
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_UGen_Info {
    pub _base: self::root::Chuck_VM_Object,
    pub tick: self::root::f_tick,
    pub tickf: self::root::f_tickf,
    pub pmsg: self::root::f_pmsg,
    pub num_ins: c_ulong,
    pub num_outs: c_ulong,
    pub tock: self::root::f_tock,
    pub num_ins_ana: c_ulong,
    pub num_outs_ana: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Chuck_UGen_Info() {
//     assert_eq!(
//         size_of::<Chuck_UGen_Info>(),
//         104usize,
//         concat!("Size of: ", stringify!(Chuck_UGen_Info))
//     );
//     assert_eq!(
//         align_of::<Chuck_UGen_Info>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_UGen_Info))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen_Info>())).tick as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen_Info),
//             "::",
//             stringify!(tick)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen_Info>())).tickf as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen_Info),
//             "::",
//             stringify!(tickf)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen_Info>())).pmsg as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen_Info),
//             "::",
//             stringify!(pmsg)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen_Info>())).num_ins as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen_Info),
//             "::",
//             stringify!(num_ins)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen_Info>())).num_outs as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen_Info),
//             "::",
//             stringify!(num_outs)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen_Info>())).tock as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen_Info),
//             "::",
//             stringify!(tock)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen_Info>())).num_ins_ana as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen_Info),
//             "::",
//             stringify!(num_ins_ana)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_UGen_Info>())).num_outs_ana as *const _ as usize
//         },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen_Info),
//             "::",
//             stringify!(num_outs_ana)
//         )
//     );
// }
impl Default for Chuck_UGen_Info {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
pub struct Chuck_Type {
    pub _base: self::root::Chuck_VM_Object,
    pub xid: self::root::te_Type,
    pub name: self::root::std::__cxx11::string,
    pub parent: *mut self::root::Chuck_Type,
    pub size: c_ulong,
    pub owner: *mut self::root::Chuck_Namespace,
    pub __bindgen_anon_1: self::root::Chuck_Type__bindgen_ty_1,
    pub array_depth: c_ulong,
    pub obj_size: c_ulong,
    pub info: *mut self::root::Chuck_Namespace,
    pub func: *mut self::root::Chuck_Func,
    pub def: self::root::a_Class_Def,
    pub ugen_info: *mut self::root::Chuck_UGen_Info,
    pub is_copy: c_ulong,
    pub is_complete: c_ulong,
    pub has_constructor: c_ulong,
    pub has_destructor: c_ulong,
    pub allocator: self::root::f_alloc,
    pub doc: self::root::std::__cxx11::string,
    pub examples: self::root::std::vector,
    pub ret: self::root::std::__cxx11::string,
    pub m_env: *mut self::root::Chuck_Env,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Chuck_Type__bindgen_ty_1 {
    pub array_type: *mut self::root::Chuck_Type,
    pub actual_type: *mut self::root::Chuck_Type,
    _bindgen_union_align: u64,
}
// #[test]
// fn bindgen_test_layout_Chuck_Type__bindgen_ty_1() {
//     assert_eq!(
//         size_of::<Chuck_Type__bindgen_ty_1>(),
//         8usize,
//         concat!("Size of: ", stringify!(Chuck_Type__bindgen_ty_1))
//     );
//     assert_eq!(
//         align_of::<Chuck_Type__bindgen_ty_1>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Type__bindgen_ty_1))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Type__bindgen_ty_1>())).array_type as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type__bindgen_ty_1),
//             "::",
//             stringify!(array_type)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Type__bindgen_ty_1>())).actual_type as *const _
//                 as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type__bindgen_ty_1),
//             "::",
//             stringify!(actual_type)
//         )
//     );
// }
impl Default for Chuck_Type__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Type__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_Type__bindgen_ty_1 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout_Chuck_Type() {
//     assert_eq!(
//         size_of::<Chuck_Type>(),
//         296usize,
//         concat!("Size of: ", stringify!(Chuck_Type))
//     );
//     assert_eq!(
//         align_of::<Chuck_Type>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Type))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).xid as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(xid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).name as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).parent as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(parent)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).size as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).owner as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(owner)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).array_depth as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(array_depth)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).obj_size as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(obj_size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).info as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(info)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).func as *const _ as usize },
//         136usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).def as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(def)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).ugen_info as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(ugen_info)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).is_copy as *const _ as usize },
//         160usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(is_copy)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).is_complete as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(is_complete)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).has_constructor as *const _ as usize },
//         176usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(has_constructor)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).has_destructor as *const _ as usize },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(has_destructor)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).allocator as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(allocator)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).doc as *const _ as usize },
//         200usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(doc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).examples as *const _ as usize },
//         232usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(examples)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).ret as *const _ as usize },
//         256usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(ret)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Type>())).m_env as *const _ as usize },
//         288usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Type),
//             "::",
//             stringify!(m_env)
//         )
//     );
// }
impl Default for Chuck_Type {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Type {{ xid: {:?}, name: {:?}, parent: {:?}, size: {:?}, owner: {:?}, __bindgen_anon_1: {:?}, array_depth: {:?}, obj_size: {:?}, info: {:?}, func: {:?}, def: {:?}, ugen_info: {:?}, is_copy: {:?}, is_complete: {:?}, has_constructor: {:?}, has_destructor: {:?}, allocator: {:?}, doc: {:?}, examples: {:?}, ret: {:?}, m_env: {:?} }}" , self . xid , self . name , self . parent , self . size , self . owner , self . __bindgen_anon_1 , self . array_depth , self . obj_size , self . info , self . func , self . def , self . ugen_info , self . is_copy , self . is_complete , self . has_constructor , self . has_destructor , self . allocator , self . doc , self . examples , self . ret , self . m_env )
    }
}
#[repr(C)]
pub struct Chuck_Value {
    pub _base: self::root::Chuck_VM_Object,
    pub type_: *mut self::root::Chuck_Type,
    pub name: self::root::std::__cxx11::string,
    pub offset: c_ulong,
    pub addr: *mut c_void,
    pub is_const: c_ulong,
    pub is_member: c_ulong,
    pub is_static: c_ulong,
    pub is_context_global: c_ulong,
    pub is_decl_checked: c_ulong,
    pub is_global: c_ulong,
    pub access: c_ulong,
    pub owner: *mut self::root::Chuck_Namespace,
    pub owner_class: *mut self::root::Chuck_Type,
    pub func_ref: *mut self::root::Chuck_Func,
    pub func_num_overloads: c_long,
    pub doc: self::root::std::__cxx11::string,
}
// #[test]
// fn bindgen_test_layout_Chuck_Value() {
//     assert_eq!(
//         size_of::<Chuck_Value>(),
//         216usize,
//         concat!("Size of: ", stringify!(Chuck_Value))
//     );
//     assert_eq!(
//         align_of::<Chuck_Value>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Value))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).type_ as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).name as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).offset as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(offset)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).addr as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(addr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).is_const as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(is_const)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).is_member as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(is_member)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).is_static as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(is_static)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Value>())).is_context_global as *const _ as usize
//         },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(is_context_global)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).is_decl_checked as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(is_decl_checked)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).is_global as *const _ as usize },
//         136usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(is_global)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).access as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(access)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).owner as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(owner)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).owner_class as *const _ as usize },
//         160usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(owner_class)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).func_ref as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(func_ref)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Value>())).func_num_overloads as *const _ as usize
//         },
//         176usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(func_num_overloads)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Value>())).doc as *const _ as usize },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Value),
//             "::",
//             stringify!(doc)
//         )
//     );
// }
impl Default for Chuck_Value {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Value {{ type: {:?}, name: {:?}, offset: {:?}, addr: {:?}, is_const: {:?}, is_member: {:?}, is_static: {:?}, is_context_global: {:?}, is_decl_checked: {:?}, is_global: {:?}, access: {:?}, owner: {:?}, owner_class: {:?}, func_ref: {:?}, func_num_overloads: {:?}, doc: {:?} }}" , self . type_ , self . name , self . offset , self . addr , self . is_const , self . is_member , self . is_static , self . is_context_global , self . is_decl_checked , self . is_global , self . access , self . owner , self . owner_class , self . func_ref , self . func_num_overloads , self . doc )
    }
}
#[repr(C)]
pub struct Chuck_Func {
    pub _base: self::root::Chuck_VM_Object,
    pub name: self::root::std::__cxx11::string,
    pub def: self::root::a_Func_Def,
    pub code: *mut self::root::Chuck_VM_Code,
    pub is_member: c_ulong,
    pub vt_index: c_ulong,
    pub value_ref: *mut self::root::Chuck_Value,
    pub next: *mut self::root::Chuck_Func,
    pub up: *mut self::root::Chuck_Value,
    pub doc: self::root::std::__cxx11::string,
}
// #[test]
// fn bindgen_test_layout_Chuck_Func() {
//     assert_eq!(
//         size_of::<Chuck_Func>(),
//         160usize,
//         concat!("Size of: ", stringify!(Chuck_Func))
//     );
//     assert_eq!(
//         align_of::<Chuck_Func>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Func))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).name as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).def as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(def)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).code as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(code)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).is_member as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(is_member)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).vt_index as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(vt_index)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).value_ref as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(value_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).next as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).up as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(up)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Func>())).doc as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Func),
//             "::",
//             stringify!(doc)
//         )
//     );
// }
impl Default for Chuck_Func {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Func {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Func {{ name: {:?}, def: {:?}, code: {:?}, is_member: {:?}, vt_index: {:?}, value_ref: {:?}, next: {:?}, up: {:?}, doc: {:?} }}" , self . name , self . def , self . code , self . is_member , self . vt_index , self . value_ref , self . next , self . up , self . doc )
    }
}
#[repr(C)]
pub struct Chuck_Local {
    pub name: self::root::std::__cxx11::string,
    pub size: c_ulong,
    pub is_ref: c_ulong,
    pub is_obj: c_ulong,
    pub is_global: c_ulong,
    pub offset: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Chuck_Local() {
//     assert_eq!(
//         size_of::<Chuck_Local>(),
//         72usize,
//         concat!("Size of: ", stringify!(Chuck_Local))
//     );
//     assert_eq!(
//         align_of::<Chuck_Local>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Local))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Local>())).name as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Local),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Local>())).size as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Local),
//             "::",
//             stringify!(size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Local>())).is_ref as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Local),
//             "::",
//             stringify!(is_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Local>())).is_obj as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Local),
//             "::",
//             stringify!(is_obj)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Local>())).is_global as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Local),
//             "::",
//             stringify!(is_global)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Local>())).offset as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Local),
//             "::",
//             stringify!(offset)
//         )
//     );
// }
impl Default for Chuck_Local {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Local {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Local {{ name: {:?}, size: {:?}, is_ref: {:?}, is_obj: {:?}, is_global: {:?}, offset: {:?} }}" , self . name , self . size , self . is_ref , self . is_obj , self . is_global , self . offset )
    }
}
#[repr(C)]
pub struct Chuck_Frame {
    pub name: self::root::std::__cxx11::string,
    pub curr_offset: c_ulong,
    pub num_access: c_ulong,
    pub stack: self::root::std::vector,
}
// #[test]
// fn bindgen_test_layout_Chuck_Frame() {
//     assert_eq!(
//         size_of::<Chuck_Frame>(),
//         72usize,
//         concat!("Size of: ", stringify!(Chuck_Frame))
//     );
//     assert_eq!(
//         align_of::<Chuck_Frame>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Frame))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Frame>())).name as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Frame),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Frame>())).curr_offset as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Frame),
//             "::",
//             stringify!(curr_offset)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Frame>())).num_access as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Frame),
//             "::",
//             stringify!(num_access)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Frame>())).stack as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Frame),
//             "::",
//             stringify!(stack)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}push_scope"]
    pub fn Chuck_Frame_push_scope(this: *mut self::root::Chuck_Frame);
}
extern "C" {
    #[link_name = "\u{1}alloc_local"]
    pub fn Chuck_Frame_alloc_local(
        this: *mut self::root::Chuck_Frame,
        size: c_ulong,
        name: *const self::root::std::__cxx11::string,
        is_ref: c_ulong,
        is_obj: c_ulong,
        is_global: c_ulong,
    ) -> *mut self::root::Chuck_Local;
}
extern "C" {
    #[link_name = "\u{1}get_scope"]
    pub fn Chuck_Frame_get_scope(
        this: *const self::root::Chuck_Frame,
        out: *mut self::root::std::vector,
    );
}
extern "C" {
    #[link_name = "\u{1}pop_scope"]
    pub fn Chuck_Frame_pop_scope(
        this: *mut self::root::Chuck_Frame,
        out: *mut self::root::std::vector,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_Frame"]
    pub fn Chuck_Frame_Chuck_Frame(this: *mut self::root::Chuck_Frame);
}
impl Default for Chuck_Frame {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Frame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Frame {{ name: {:?}, curr_offset: {:?}, num_access: {:?}, stack: {:?} }}",
            self.name, self.curr_offset, self.num_access, self.stack
        )
    }
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
        name: *const self::root::std::__cxx11::string,
        is_ref: c_ulong,
        is_obj: c_ulong,
        is_global: c_ulong,
    ) -> *mut self::root::Chuck_Local {
        Chuck_Frame_alloc_local(self, size, name, is_ref, is_obj, is_global)
    }
    #[inline]
    pub unsafe fn get_scope(&self, out: *mut self::root::std::vector) {
        Chuck_Frame_get_scope(self, out)
    }
    #[inline]
    pub unsafe fn pop_scope(&mut self, out: *mut self::root::std::vector) {
        Chuck_Frame_pop_scope(self, out)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Frame_Chuck_Frame(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Instr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Instr_Goto {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_Code {
    pub name: self::root::std::__cxx11::string,
    pub stack_depth: c_ulong,
    pub need_this: c_ulong,
    pub frame: *mut self::root::Chuck_Frame,
    pub code: self::root::std::vector,
    pub stack_cont: self::root::std::vector,
    pub stack_break: self::root::std::vector,
    pub stack_return: self::root::std::vector,
    pub filename: self::root::std::__cxx11::string,
}
// #[test]
// fn bindgen_test_layout_Chuck_Code() {
//     assert_eq!(
//         size_of::<Chuck_Code>(),
//         184usize,
//         concat!("Size of: ", stringify!(Chuck_Code))
//     );
//     assert_eq!(
//         align_of::<Chuck_Code>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Code))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).name as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).stack_depth as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(stack_depth)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).need_this as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(need_this)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).frame as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(frame)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).code as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(code)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).stack_cont as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(stack_cont)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).stack_break as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(stack_break)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).stack_return as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(stack_return)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Code>())).filename as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Code),
//             "::",
//             stringify!(filename)
//         )
//     );
// }
impl Default for Chuck_Code {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Code {{ name: {:?}, stack_depth: {:?}, need_this: {:?}, frame: {:?}, code: {:?}, stack_cont: {:?}, stack_break: {:?}, stack_return: {:?}, filename: {:?} }}" , self . name , self . stack_depth , self . need_this , self . frame , self . code , self . stack_cont , self . stack_break , self . stack_return , self . filename )
    }
}
#[repr(C)]
pub struct Chuck_Emitter {
    pub _base: self::root::Chuck_VM_Object,
    pub env: *mut self::root::Chuck_Env,
    pub code: *mut self::root::Chuck_Code,
    pub context: *mut self::root::Chuck_Context,
    pub nspc: *mut self::root::Chuck_Namespace,
    pub func: *mut self::root::Chuck_Func,
    pub stack: self::root::std::vector,
    pub locals: self::root::std::vector,
    pub dump: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Chuck_Emitter() {
//     assert_eq!(
//         size_of::<Chuck_Emitter>(),
//         136usize,
//         concat!("Size of: ", stringify!(Chuck_Emitter))
//     );
//     assert_eq!(
//         align_of::<Chuck_Emitter>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Emitter))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Emitter>())).env as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Emitter),
//             "::",
//             stringify!(env)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Emitter>())).code as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Emitter),
//             "::",
//             stringify!(code)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Emitter>())).context as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Emitter),
//             "::",
//             stringify!(context)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Emitter>())).nspc as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Emitter),
//             "::",
//             stringify!(nspc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Emitter>())).func as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Emitter),
//             "::",
//             stringify!(func)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Emitter>())).stack as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Emitter),
//             "::",
//             stringify!(stack)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Emitter>())).locals as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Emitter),
//             "::",
//             stringify!(locals)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Emitter>())).dump as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Emitter),
//             "::",
//             stringify!(dump)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}addref_on_scope"]
    pub fn Chuck_Emitter_addref_on_scope(this: *mut self::root::Chuck_Emitter);
}
extern "C" {
    #[link_name = "\u{1}pop_scope"]
    pub fn Chuck_Emitter_pop_scope(this: *mut self::root::Chuck_Emitter);
}
extern "C" {
    #[link_name = "\u{1}find_dur"]
    pub fn Chuck_Emitter_find_dur(
        this: *mut self::root::Chuck_Emitter,
        name: *const self::root::std::__cxx11::string,
        out: *mut f64,
    ) -> c_ulong;
}
impl Default for Chuck_Emitter {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Emitter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Emitter {{ env: {:?}, code: {:?}, context: {:?}, nspc: {:?}, func: {:?}, stack: {:?}, locals: {:?}, dump: {:?} }}" , self . env , self . code , self . context , self . nspc , self . func , self . stack , self . locals , self . dump )
    }
}
impl PartialEq for Chuck_Emitter {
    fn eq(&self, other: &Chuck_Emitter) -> bool {
        self._base == other._base
            && self.env == other.env
            && self.code == other.code
            && self.context == other.context
            && self.nspc == other.nspc
            && self.func == other.func
            && self.stack == other.stack
            && self.locals == other.locals
            && self.dump == other.dump
    }
}
impl Chuck_Emitter {
    #[inline]
    pub unsafe fn addref_on_scope(&mut self) {
        Chuck_Emitter_addref_on_scope(self)
    }
    #[inline]
    pub unsafe fn pop_scope(&mut self) {
        Chuck_Emitter_pop_scope(self)
    }
    #[inline]
    pub unsafe fn find_dur(
        &mut self,
        name: *const self::root::std::__cxx11::string,
        out: *mut f64,
    ) -> c_ulong {
        Chuck_Emitter_find_dur(self, name, out)
    }
}
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Chuck_UGen {
    pub _base: self::root::Chuck_Object,
    pub tick: self::root::f_tick,
    pub tickf: self::root::f_tickf,
    pub pmsg: self::root::f_pmsg,
    pub m_multi_chan: *mut *mut self::root::Chuck_UGen,
    pub m_multi_chan_size: c_ulong,
    pub m_num_ins: c_ulong,
    pub m_num_outs: c_ulong,
    pub m_src_list: *mut *mut self::root::Chuck_UGen,
    pub m_src_cap: c_ulong,
    pub m_num_src: c_ulong,
    pub m_dest_list: *mut *mut self::root::Chuck_UGen,
    pub m_dest_cap: c_ulong,
    pub m_num_dest: c_ulong,
    pub m_src_uana_list: *mut *mut self::root::Chuck_UGen,
    pub m_src_uana_cap: c_ulong,
    pub m_num_uana_src: c_ulong,
    pub m_dest_uana_list: *mut *mut self::root::Chuck_UGen,
    pub m_dest_uana_cap: c_ulong,
    pub m_num_uana_dest: c_ulong,
    pub m_max_src: c_ulong,
    pub m_time: f64,
    pub m_valid: c_ulong,
    pub m_use_next: c_ulong,
    pub m_sum: f64,
    pub m_current: f64,
    pub m_next: f64,
    pub m_last: f64,
    pub m_gain: f64,
    pub m_pan: f64,
    pub m_op: c_long,
    pub m_max_block_size: c_long,
    pub m_multi_in_v: *mut f64,
    pub m_multi_out_v: *mut f64,
    pub m_is_subgraph: c_ulong,
    pub m_inlet: *mut self::root::Chuck_UGen,
    pub m_outlet: *mut self::root::Chuck_UGen,
    pub m_sum_v: *mut f64,
    pub m_current_v: *mut f64,
    pub shred: *mut self::root::Chuck_VM_Shred,
    pub vm: *mut self::root::Chuck_VM,
    pub owner: *mut self::root::Chuck_UGen,
    pub m_is_uana: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Chuck_UGen() {
//     assert_eq!(
//         size_of::<Chuck_UGen>(),
//         408usize,
//         concat!("Size of: ", stringify!(Chuck_UGen))
//     );
//     assert_eq!(
//         align_of::<Chuck_UGen>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_UGen))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).tick as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(tick)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).tickf as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(tickf)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).pmsg as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(pmsg)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_multi_chan as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_multi_chan)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_UGen>())).m_multi_chan_size as *const _ as usize
//         },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_multi_chan_size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_num_ins as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_num_ins)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_num_outs as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_num_outs)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_src_list as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_src_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_src_cap as *const _ as usize },
//         136usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_src_cap)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_num_src as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_num_src)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_dest_list as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_dest_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_dest_cap as *const _ as usize },
//         160usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_dest_cap)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_num_dest as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_num_dest)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_src_uana_list as *const _ as usize },
//         176usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_src_uana_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_src_uana_cap as *const _ as usize },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_src_uana_cap)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_num_uana_src as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_num_uana_src)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_dest_uana_list as *const _ as usize },
//         200usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_dest_uana_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_dest_uana_cap as *const _ as usize },
//         208usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_dest_uana_cap)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_num_uana_dest as *const _ as usize },
//         216usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_num_uana_dest)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_max_src as *const _ as usize },
//         224usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_max_src)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_time as *const _ as usize },
//         232usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_time)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_valid as *const _ as usize },
//         240usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_valid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_use_next as *const _ as usize },
//         248usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_use_next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_sum as *const _ as usize },
//         256usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_sum)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_current as *const _ as usize },
//         264usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_current)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_next as *const _ as usize },
//         272usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_last as *const _ as usize },
//         280usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_last)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_gain as *const _ as usize },
//         288usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_gain)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_pan as *const _ as usize },
//         296usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_pan)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_op as *const _ as usize },
//         304usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_op)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_max_block_size as *const _ as usize },
//         312usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_max_block_size)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_multi_in_v as *const _ as usize },
//         320usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_multi_in_v)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_multi_out_v as *const _ as usize },
//         328usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_multi_out_v)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_is_subgraph as *const _ as usize },
//         336usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_is_subgraph)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_inlet as *const _ as usize },
//         344usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_inlet)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_outlet as *const _ as usize },
//         352usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_outlet)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_sum_v as *const _ as usize },
//         360usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_sum_v)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_current_v as *const _ as usize },
//         368usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_current_v)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).shred as *const _ as usize },
//         376usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(shred)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).vm as *const _ as usize },
//         384usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(vm)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).owner as *const _ as usize },
//         392usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(owner)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UGen>())).m_is_uana as *const _ as usize },
//         400usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UGen),
//             "::",
//             stringify!(m_is_uana)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}add"]
    pub fn Chuck_UGen_add(
        this: *mut self::root::Chuck_UGen,
        src: *mut self::root::Chuck_UGen,
        isUpChuck: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_UGen_remove(
        this: *mut self::root::Chuck_UGen,
        src: *mut self::root::Chuck_UGen,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_all"]
    pub fn Chuck_UGen_remove_all(this: *mut self::root::Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}set_max_src"]
    pub fn Chuck_UGen_set_max_src(this: *mut self::root::Chuck_UGen, num: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_num_src"]
    pub fn Chuck_UGen_get_num_src(this: *mut self::root::Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}is_connected_from"]
    pub fn Chuck_UGen_is_connected_from(
        this: *mut self::root::Chuck_UGen,
        src: *mut self::root::Chuck_UGen,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}disconnect"]
    pub fn Chuck_UGen_disconnect(this: *mut self::root::Chuck_UGen, recursive: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}system_tick"]
    pub fn Chuck_UGen_system_tick(this: *mut self::root::Chuck_UGen, now: f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}system_tick_v"]
    pub fn Chuck_UGen_system_tick_v(
        this: *mut self::root::Chuck_UGen,
        now: f64,
        numFrames: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}alloc_v"]
    pub fn Chuck_UGen_alloc_v(this: *mut self::root::Chuck_UGen, size: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}src_chan"]
    pub fn Chuck_UGen_src_chan(
        this: *mut self::root::Chuck_UGen,
        chan: c_ulong,
    ) -> *mut self::root::Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}dst_for_src_chan"]
    pub fn Chuck_UGen_dst_for_src_chan(
        this: *mut self::root::Chuck_UGen,
        chan: c_ulong,
    ) -> *mut self::root::Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}add_by"]
    pub fn Chuck_UGen_add_by(
        this: *mut self::root::Chuck_UGen,
        dest: *mut self::root::Chuck_UGen,
        isUpChuck: c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}remove_by"]
    pub fn Chuck_UGen_remove_by(
        this: *mut self::root::Chuck_UGen,
        dest: *mut self::root::Chuck_UGen,
    );
}
extern "C" {
    #[link_name = "\u{1}alloc_multi_chan"]
    pub fn Chuck_UGen_alloc_multi_chan(
        this: *mut self::root::Chuck_UGen,
        num_ins: c_ulong,
        num_outs: c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}init_subgraph"]
    pub fn Chuck_UGen_init_subgraph(this: *mut self::root::Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}inlet"]
    pub fn Chuck_UGen_inlet(this: *mut self::root::Chuck_UGen) -> *mut self::root::Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}outlet"]
    pub fn Chuck_UGen_outlet(this: *mut self::root::Chuck_UGen) -> *mut self::root::Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen"]
    pub fn Chuck_UGen_Chuck_UGen(this: *mut self::root::Chuck_UGen);
}
impl Default for Chuck_UGen {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Chuck_UGen {
    #[inline]
    pub unsafe fn add(&mut self, src: *mut self::root::Chuck_UGen, isUpChuck: c_ulong) -> c_ulong {
        Chuck_UGen_add(self, src, isUpChuck)
    }
    #[inline]
    pub unsafe fn remove(&mut self, src: *mut self::root::Chuck_UGen) -> c_ulong {
        Chuck_UGen_remove(self, src)
    }
    #[inline]
    pub unsafe fn remove_all(&mut self) {
        Chuck_UGen_remove_all(self)
    }
    #[inline]
    pub unsafe fn set_max_src(&mut self, num: c_ulong) -> c_ulong {
        Chuck_UGen_set_max_src(self, num)
    }
    #[inline]
    pub unsafe fn get_num_src(&mut self) -> c_ulong {
        Chuck_UGen_get_num_src(self)
    }
    #[inline]
    pub unsafe fn is_connected_from(&mut self, src: *mut self::root::Chuck_UGen) -> c_ulong {
        Chuck_UGen_is_connected_from(self, src)
    }
    #[inline]
    pub unsafe fn disconnect(&mut self, recursive: c_ulong) -> c_ulong {
        Chuck_UGen_disconnect(self, recursive)
    }
    #[inline]
    pub unsafe fn system_tick(&mut self, now: f64) -> c_ulong {
        Chuck_UGen_system_tick(self, now)
    }
    #[inline]
    pub unsafe fn system_tick_v(&mut self, now: f64, numFrames: c_ulong) -> c_ulong {
        Chuck_UGen_system_tick_v(self, now, numFrames)
    }
    #[inline]
    pub unsafe fn alloc_v(&mut self, size: c_ulong) -> c_ulong {
        Chuck_UGen_alloc_v(self, size)
    }
    #[inline]
    pub unsafe fn src_chan(&mut self, chan: c_ulong) -> *mut self::root::Chuck_UGen {
        Chuck_UGen_src_chan(self, chan)
    }
    #[inline]
    pub unsafe fn dst_for_src_chan(&mut self, chan: c_ulong) -> *mut self::root::Chuck_UGen {
        Chuck_UGen_dst_for_src_chan(self, chan)
    }
    #[inline]
    pub unsafe fn add_by(&mut self, dest: *mut self::root::Chuck_UGen, isUpChuck: c_ulong) {
        Chuck_UGen_add_by(self, dest, isUpChuck)
    }
    #[inline]
    pub unsafe fn remove_by(&mut self, dest: *mut self::root::Chuck_UGen) {
        Chuck_UGen_remove_by(self, dest)
    }
    #[inline]
    pub unsafe fn alloc_multi_chan(&mut self, num_ins: c_ulong, num_outs: c_ulong) {
        Chuck_UGen_alloc_multi_chan(self, num_ins, num_outs)
    }
    #[inline]
    pub unsafe fn init_subgraph(&mut self) {
        Chuck_UGen_init_subgraph(self)
    }
    #[inline]
    pub unsafe fn inlet(&mut self) -> *mut self::root::Chuck_UGen {
        Chuck_UGen_inlet(self)
    }
    #[inline]
    pub unsafe fn outlet(&mut self) -> *mut self::root::Chuck_UGen {
        Chuck_UGen_outlet(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_UGen_Chuck_UGen(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen_destructor"]
    pub fn Chuck_UGen_Chuck_UGen_destructor(this: *mut self::root::Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn Chuck_UGen_init(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}done"]
    pub fn Chuck_UGen_done(this: *mut c_void);
}
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Chuck_UAna {
    pub _base: self::root::Chuck_UGen,
    pub tock: self::root::f_tock,
    pub m_uana_time: f64,
}
// #[test]
// fn bindgen_test_layout_Chuck_UAna() {
//     assert_eq!(
//         size_of::<Chuck_UAna>(),
//         424usize,
//         concat!("Size of: ", stringify!(Chuck_UAna))
//     );
//     assert_eq!(
//         align_of::<Chuck_UAna>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_UAna))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UAna>())).tock as *const _ as usize },
//         408usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UAna),
//             "::",
//             stringify!(tock)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_UAna>())).m_uana_time as *const _ as usize },
//         416usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_UAna),
//             "::",
//             stringify!(m_uana_time)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}system_tock"]
    pub fn Chuck_UAna_system_tock(this: *mut self::root::Chuck_UAna, now: f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}is_up_connected_from"]
    pub fn Chuck_UAna_is_up_connected_from(
        this: *mut self::root::Chuck_UAna,
        src: *mut self::root::Chuck_UAna,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}numIncomingUAnae"]
    pub fn Chuck_UAna_numIncomingUAnae(this: *const self::root::Chuck_UAna) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}getIncomingUAna"]
    pub fn Chuck_UAna_getIncomingUAna(
        this: *const self::root::Chuck_UAna,
        index: c_ulong,
    ) -> *mut self::root::Chuck_UAna;
}
extern "C" {
    #[link_name = "\u{1}getIncomingBlob"]
    pub fn Chuck_UAna_getIncomingBlob(
        this: *const self::root::Chuck_UAna,
        index: c_ulong,
    ) -> *mut self::root::Chuck_UAnaBlobProxy;
}
extern "C" {
    #[link_name = "\u{1}blobProxy"]
    pub fn Chuck_UAna_blobProxy(
        this: *const self::root::Chuck_UAna,
    ) -> *mut self::root::Chuck_UAnaBlobProxy;
}
extern "C" {
    #[link_name = "\u{1}Chuck_UAna"]
    pub fn Chuck_UAna_Chuck_UAna(this: *mut self::root::Chuck_UAna);
}
impl Default for Chuck_UAna {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Chuck_UAna {
    #[inline]
    pub unsafe fn system_tock(&mut self, now: f64) -> c_ulong {
        Chuck_UAna_system_tock(self, now)
    }
    #[inline]
    pub unsafe fn is_up_connected_from(&mut self, src: *mut self::root::Chuck_UAna) -> c_ulong {
        Chuck_UAna_is_up_connected_from(self, src)
    }
    #[inline]
    pub unsafe fn numIncomingUAnae(&self) -> c_long {
        Chuck_UAna_numIncomingUAnae(self)
    }
    #[inline]
    pub unsafe fn getIncomingUAna(&self, index: c_ulong) -> *mut self::root::Chuck_UAna {
        Chuck_UAna_getIncomingUAna(self, index)
    }
    #[inline]
    pub unsafe fn getIncomingBlob(&self, index: c_ulong) -> *mut self::root::Chuck_UAnaBlobProxy {
        Chuck_UAna_getIncomingBlob(self, index)
    }
    #[inline]
    pub unsafe fn blobProxy(&self) -> *mut self::root::Chuck_UAnaBlobProxy {
        Chuck_UAna_blobProxy(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_UAna_Chuck_UAna(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_UAna_destructor"]
    pub fn Chuck_UAna_Chuck_UAna_destructor(this: *mut self::root::Chuck_UAna);
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CBufferSimple {
    pub m_data: *mut c_uchar,
    pub m_data_width: c_ulong,
    pub m_read_offset: c_ulong,
    pub m_write_offset: c_ulong,
    pub m_max_elem: c_ulong,
}
// #[test]
// fn bindgen_test_layout_CBufferSimple() {
//     assert_eq!(
//         size_of::<CBufferSimple>(),
//         40usize,
//         concat!("Size of: ", stringify!(CBufferSimple))
//     );
//     assert_eq!(
//         align_of::<CBufferSimple>(),
//         8usize,
//         concat!("Alignment of ", stringify!(CBufferSimple))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<CBufferSimple>())).m_data as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(CBufferSimple),
//             "::",
//             stringify!(m_data)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<CBufferSimple>())).m_data_width as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(CBufferSimple),
//             "::",
//             stringify!(m_data_width)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<CBufferSimple>())).m_read_offset as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(CBufferSimple),
//             "::",
//             stringify!(m_read_offset)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<CBufferSimple>())).m_write_offset as *const _ as usize
//         },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(CBufferSimple),
//             "::",
//             stringify!(m_write_offset)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<CBufferSimple>())).m_max_elem as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(CBufferSimple),
//             "::",
//             stringify!(m_max_elem)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn CBufferSimple_initialize(
        this: *mut self::root::CBufferSimple,
        num_elem: c_ulong,
        width: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn CBufferSimple_cleanup(this: *mut self::root::CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn CBufferSimple_get(
        this: *mut self::root::CBufferSimple,
        data: *mut c_void,
        num_elem: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn CBufferSimple_put(
        this: *mut self::root::CBufferSimple,
        data: *mut c_void,
        num_elem: c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}CBufferSimple"]
    pub fn CBufferSimple_CBufferSimple(this: *mut self::root::CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}CBufferSimple_destructor"]
    pub fn CBufferSimple_CBufferSimple_destructor(this: *mut self::root::CBufferSimple);
}
impl Default for CBufferSimple {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl CBufferSimple {
    #[inline]
    pub unsafe fn initialize(&mut self, num_elem: c_ulong, width: c_ulong) -> c_ulong {
        CBufferSimple_initialize(self, num_elem, width)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        CBufferSimple_cleanup(self)
    }
    #[inline]
    pub unsafe fn get(&mut self, data: *mut c_void, num_elem: c_ulong) -> c_ulong {
        CBufferSimple_get(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn put(&mut self, data: *mut c_void, num_elem: c_ulong) {
        CBufferSimple_put(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        CBufferSimple_CBufferSimple(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        CBufferSimple_CBufferSimple_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CircularBuffer<T> {
    pub m_elements: *mut T,
    pub m_read: usize,
    pub m_write: usize,
    pub m_numElements: usize,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for CircularBuffer<T> {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct FastCircularBuffer {
    pub m_data: *mut c_uchar,
    pub m_data_width: c_ulong,
    pub m_read_offset: c_ulong,
    pub m_write_offset: c_ulong,
    pub m_max_elem: c_ulong,
}
// #[test]
// fn bindgen_test_layout_FastCircularBuffer() {
//     assert_eq!(
//         size_of::<FastCircularBuffer>(),
//         40usize,
//         concat!("Size of: ", stringify!(FastCircularBuffer))
//     );
//     assert_eq!(
//         align_of::<FastCircularBuffer>(),
//         8usize,
//         concat!("Alignment of ", stringify!(FastCircularBuffer))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<FastCircularBuffer>())).m_data as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(FastCircularBuffer),
//             "::",
//             stringify!(m_data)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<FastCircularBuffer>())).m_data_width as *const _ as usize
//         },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(FastCircularBuffer),
//             "::",
//             stringify!(m_data_width)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<FastCircularBuffer>())).m_read_offset as *const _ as usize
//         },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(FastCircularBuffer),
//             "::",
//             stringify!(m_read_offset)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<FastCircularBuffer>())).m_write_offset as *const _ as usize
//         },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(FastCircularBuffer),
//             "::",
//             stringify!(m_write_offset)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<FastCircularBuffer>())).m_max_elem as *const _ as usize
//         },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(FastCircularBuffer),
//             "::",
//             stringify!(m_max_elem)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn FastCircularBuffer_initialize(
        this: *mut self::root::FastCircularBuffer,
        num_elem: c_ulong,
        width: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn FastCircularBuffer_cleanup(this: *mut self::root::FastCircularBuffer);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn FastCircularBuffer_get(
        this: *mut self::root::FastCircularBuffer,
        data: *mut c_void,
        num_elem: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn FastCircularBuffer_put(
        this: *mut self::root::FastCircularBuffer,
        data: *mut c_void,
        num_elem: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}FastCircularBuffer"]
    pub fn FastCircularBuffer_FastCircularBuffer(this: *mut self::root::FastCircularBuffer);
}
extern "C" {
    #[link_name = "\u{1}FastCircularBuffer_destructor"]
    pub fn FastCircularBuffer_FastCircularBuffer_destructor(
        this: *mut self::root::FastCircularBuffer,
    );
}
impl Default for FastCircularBuffer {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl FastCircularBuffer {
    #[inline]
    pub unsafe fn initialize(&mut self, num_elem: c_ulong, width: c_ulong) -> c_ulong {
        FastCircularBuffer_initialize(self, num_elem, width)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        FastCircularBuffer_cleanup(self)
    }
    #[inline]
    pub unsafe fn get(&mut self, data: *mut c_void, num_elem: c_ulong) -> c_ulong {
        FastCircularBuffer_get(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn put(&mut self, data: *mut c_void, num_elem: c_ulong) -> c_ulong {
        FastCircularBuffer_put(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        FastCircularBuffer_FastCircularBuffer(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        FastCircularBuffer_FastCircularBuffer_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XCircleBuffer<T> {
    pub m_buffer: *mut T,
    pub m_length: c_long,
    pub m_writeIndex: c_long,
    pub m_readIndex: c_long,
    pub m_numElements: c_long,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for XCircleBuffer<T> {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Shred_Data {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Shred_Time {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Shred_Activation {
    pub when: f64,
    pub cycles: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Shred_Activation() {
//     assert_eq!(
//         size_of::<Shred_Activation>(),
//         16usize,
//         concat!("Size of: ", stringify!(Shred_Activation))
//     );
//     assert_eq!(
//         align_of::<Shred_Activation>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Shred_Activation))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Activation>())).when as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Activation),
//             "::",
//             stringify!(when)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Activation>())).cycles as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Activation),
//             "::",
//             stringify!(cycles)
//         )
//     );
// }
#[repr(C)]
pub struct Shred_Stat {
    pub cycles: c_ulong,
    pub xid: c_ulong,
    pub parent: c_ulong,
    pub state: c_ulong,
    pub shred_ref: *mut self::root::Chuck_VM_Shred,
    pub activations: c_ulong,
    pub average_ctrl: f64,
    pub average_cycles: f64,
    pub spork_time: f64,
    pub active_time: f64,
    pub wake_time: f64,
    pub free_time: f64,
    pub name: self::root::std::__cxx11::string,
    pub owner: self::root::std::__cxx11::string,
    pub source: self::root::std::__cxx11::string,
    pub diffs: self::root::std::queue<self::root::std::deque>,
    pub num_diffs: c_ulong,
    pub diff_total: f64,
    pub act_cycles: self::root::std::queue<self::root::std::deque>,
    pub act_cycles_total: c_ulong,
    pub last_cycles: c_ulong,
    pub children: self::root::std::vector,
    pub activationss: self::root::std::vector,
    pub mutex: self::root::XMutex,
    pub data: *mut self::root::Shred_Data,
    pub time: *mut self::root::Shred_Time,
}
// #[test]
// fn bindgen_test_layout_Shred_Stat() {
//     assert_eq!(
//         size_of::<Shred_Stat>(),
//         488usize,
//         concat!("Size of: ", stringify!(Shred_Stat))
//     );
//     assert_eq!(
//         align_of::<Shred_Stat>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Shred_Stat))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).cycles as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(cycles)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).xid as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(xid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).parent as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(parent)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).state as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(state)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).shred_ref as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(shred_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).activations as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(activations)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).average_ctrl as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(average_ctrl)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).average_cycles as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(average_cycles)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).spork_time as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(spork_time)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).active_time as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(active_time)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).wake_time as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(wake_time)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).free_time as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(free_time)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).name as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).owner as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(owner)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).source as *const _ as usize },
//         160usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(source)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).diffs as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(diffs)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).num_diffs as *const _ as usize },
//         272usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(num_diffs)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).diff_total as *const _ as usize },
//         280usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(diff_total)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).act_cycles as *const _ as usize },
//         288usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(act_cycles)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).act_cycles_total as *const _ as usize },
//         368usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(act_cycles_total)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).last_cycles as *const _ as usize },
//         376usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(last_cycles)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).children as *const _ as usize },
//         384usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(children)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).activationss as *const _ as usize },
//         408usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(activationss)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).mutex as *const _ as usize },
//         432usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(mutex)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).data as *const _ as usize },
//         472usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(data)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Shred_Stat>())).time as *const _ as usize },
//         480usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Shred_Stat),
//             "::",
//             stringify!(time)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}get_sporked"]
    pub fn Shred_Stat_get_sporked(
        this: *mut self::root::Shred_Stat,
        out: *mut self::root::std::vector,
    );
}
extern "C" {
    #[link_name = "\u{1}get_activations"]
    pub fn Shred_Stat_get_activations(
        this: *mut self::root::Shred_Stat,
        out: *mut self::root::std::vector,
    );
}
impl Default for Shred_Stat {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Shred_Stat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Shred_Stat {{ cycles: {:?}, xid: {:?}, parent: {:?}, state: {:?}, shred_ref: {:?}, activations: {:?}, average_ctrl: {:?}, average_cycles: {:?}, spork_time: {:?}, active_time: {:?}, wake_time: {:?}, free_time: {:?}, name: {:?}, owner: {:?}, source: {:?}, diffs: {:?}, num_diffs: {:?}, diff_total: {:?}, act_cycles: {:?}, act_cycles_total: {:?}, last_cycles: {:?}, children: {:?}, activationss: {:?}, mutex: {:?}, data: {:?}, time: {:?} }}" , self . cycles , self . xid , self . parent , self . state , self . shred_ref , self . activations , self . average_ctrl , self . average_cycles , self . spork_time , self . active_time , self . wake_time , self . free_time , self . name , self . owner , self . source , self . diffs , self . num_diffs , self . diff_total , self . act_cycles , self . act_cycles_total , self . last_cycles , self . children , self . activationss , self . mutex , self . data , self . time )
    }
}
impl Shred_Stat {
    #[inline]
    pub unsafe fn get_sporked(&mut self, out: *mut self::root::std::vector) {
        Shred_Stat_get_sporked(self, out)
    }
    #[inline]
    pub unsafe fn get_activations(&mut self, out: *mut self::root::std::vector) {
        Shred_Stat_get_activations(self, out)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_IO_Serial {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_VM_Stack {
    pub stack: *mut c_uchar,
    pub sp: *mut c_uchar,
    pub sp_max: *mut c_uchar,
    pub prev: *mut self::root::Chuck_VM_Stack,
    pub next: *mut self::root::Chuck_VM_Stack,
    pub m_is_init: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Chuck_VM_Stack() {
//     assert_eq!(
//         size_of::<Chuck_VM_Stack>(),
//         48usize,
//         concat!("Size of: ", stringify!(Chuck_VM_Stack))
//     );
//     assert_eq!(
//         align_of::<Chuck_VM_Stack>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VM_Stack))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Stack>())).stack as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Stack),
//             "::",
//             stringify!(stack)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Stack>())).sp as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Stack),
//             "::",
//             stringify!(sp)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Stack>())).sp_max as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Stack),
//             "::",
//             stringify!(sp_max)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Stack>())).prev as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Stack),
//             "::",
//             stringify!(prev)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Stack>())).next as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Stack),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Stack>())).m_is_init as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Stack),
//             "::",
//             stringify!(m_is_init)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_Stack_initialize(
        this: *mut self::root::Chuck_VM_Stack,
        size: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Stack_shutdown(this: *mut self::root::Chuck_VM_Stack) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Stack"]
    pub fn Chuck_VM_Stack_Chuck_VM_Stack(this: *mut self::root::Chuck_VM_Stack);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Stack_destructor"]
    pub fn Chuck_VM_Stack_Chuck_VM_Stack_destructor(this: *mut self::root::Chuck_VM_Stack);
}
impl Default for Chuck_VM_Stack {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Chuck_VM_Stack {
    #[inline]
    pub unsafe fn initialize(&mut self, size: c_ulong) -> c_ulong {
        Chuck_VM_Stack_initialize(self, size)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_Stack_shutdown(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_VM_Stack_Chuck_VM_Stack(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_VM_Stack_Chuck_VM_Stack_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_VM_Code {
    pub _base: self::root::Chuck_Object,
    pub instr: *mut *mut self::root::Chuck_Instr,
    pub num_instr: c_ulong,
    pub name: self::root::std::__cxx11::string,
    pub stack_depth: c_ulong,
    pub need_this: c_ulong,
    pub native_func: c_ulong,
    pub native_func_type: c_ulong,
    pub filename: self::root::std::__cxx11::string,
}
pub const Chuck_VM_Code_NATIVE_UNKNOWN: self::root::Chuck_VM_Code__bindgen_ty_1 = 0;
pub const Chuck_VM_Code_NATIVE_CTOR: self::root::Chuck_VM_Code__bindgen_ty_1 = 1;
pub const Chuck_VM_Code_NATIVE_DTOR: self::root::Chuck_VM_Code__bindgen_ty_1 = 2;
pub const Chuck_VM_Code_NATIVE_MFUN: self::root::Chuck_VM_Code__bindgen_ty_1 = 3;
pub const Chuck_VM_Code_NATIVE_SFUN: self::root::Chuck_VM_Code__bindgen_ty_1 = 4;
pub type Chuck_VM_Code__bindgen_ty_1 = u32;
// #[test]
// fn bindgen_test_layout_Chuck_VM_Code() {
//     assert_eq!(
//         size_of::<Chuck_VM_Code>(),
//         184usize,
//         concat!("Size of: ", stringify!(Chuck_VM_Code))
//     );
//     assert_eq!(
//         align_of::<Chuck_VM_Code>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VM_Code))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Code>())).instr as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Code),
//             "::",
//             stringify!(instr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Code>())).num_instr as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Code),
//             "::",
//             stringify!(num_instr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Code>())).name as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Code),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Code>())).stack_depth as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Code),
//             "::",
//             stringify!(stack_depth)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Code>())).need_this as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Code),
//             "::",
//             stringify!(need_this)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Code>())).native_func as *const _ as usize },
//         136usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Code),
//             "::",
//             stringify!(native_func)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Code>())).native_func_type as *const _ as usize
//         },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Code),
//             "::",
//             stringify!(native_func_type)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Code>())).filename as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Code),
//             "::",
//             stringify!(filename)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Code"]
    pub fn Chuck_VM_Code_Chuck_VM_Code(this: *mut self::root::Chuck_VM_Code);
}
impl Default for Chuck_VM_Code {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VM_Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_VM_Code {{ instr: {:?}, num_instr: {:?}, name: {:?}, stack_depth: {:?}, need_this: {:?}, native_func: {:?}, native_func_type: {:?}, filename: {:?} }}" , self . instr , self . num_instr , self . name , self . stack_depth , self . need_this , self . native_func , self . native_func_type , self . filename )
    }
}
impl Chuck_VM_Code {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_VM_Code_Chuck_VM_Code(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Code_destructor"]
    pub fn Chuck_VM_Code_Chuck_VM_Code_destructor(this: *mut self::root::Chuck_VM_Code);
}
#[repr(C)]
pub struct Chuck_VM_Shred {
    pub _base: self::root::Chuck_Object,
    pub mem: *mut self::root::Chuck_VM_Stack,
    pub reg: *mut self::root::Chuck_VM_Stack,
    pub base_ref: *mut self::root::Chuck_VM_Stack,
    pub code: *mut self::root::Chuck_VM_Code,
    pub code_orig: *mut self::root::Chuck_VM_Code,
    pub instr: *mut *mut self::root::Chuck_Instr,
    pub parent: *mut self::root::Chuck_VM_Shred,
    pub children: self::root::std::map,
    pub pc: c_ulong,
    pub vm_ref: *mut self::root::Chuck_VM,
    pub now: f64,
    pub start: f64,
    pub wake_time: f64,
    pub next_pc: c_ulong,
    pub is_done: c_ulong,
    pub is_running: c_ulong,
    pub is_abort: c_ulong,
    pub is_dumped: c_ulong,
    pub event: *mut self::root::Chuck_Event,
    pub m_ugen_map: self::root::std::map,
    pub m_parent_objects: self::root::std::vector,
    pub xid: c_ulong,
    pub name: self::root::std::__cxx11::string,
    pub args: self::root::std::vector,
    pub prev: *mut self::root::Chuck_VM_Shred,
    pub next: *mut self::root::Chuck_VM_Shred,
    pub stat: *mut self::root::Shred_Stat,
    pub m_loopCounters: self::root::std::vector,
    pub m_serials: *mut self::root::std::__cxx11::list,
}
// #[test]
// fn bindgen_test_layout_Chuck_VM_Shred() {
//     assert_eq!(
//         size_of::<Chuck_VM_Shred>(),
//         456usize,
//         concat!("Size of: ", stringify!(Chuck_VM_Shred))
//     );
//     assert_eq!(
//         align_of::<Chuck_VM_Shred>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VM_Shred))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).mem as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(mem)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).reg as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(reg)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).base_ref as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(base_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).code as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(code)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).code_orig as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(code_orig)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).instr as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(instr)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).parent as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(parent)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).children as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(children)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).pc as *const _ as usize },
//         176usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(pc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).vm_ref as *const _ as usize },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(vm_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).now as *const _ as usize },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(now)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).start as *const _ as usize },
//         200usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(start)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).wake_time as *const _ as usize },
//         208usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(wake_time)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).next_pc as *const _ as usize },
//         216usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(next_pc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).is_done as *const _ as usize },
//         224usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(is_done)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).is_running as *const _ as usize },
//         232usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(is_running)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).is_abort as *const _ as usize },
//         240usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(is_abort)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).is_dumped as *const _ as usize },
//         248usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(is_dumped)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).event as *const _ as usize },
//         256usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(event)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).m_ugen_map as *const _ as usize },
//         264usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(m_ugen_map)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shred>())).m_parent_objects as *const _ as usize
//         },
//         312usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(m_parent_objects)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).xid as *const _ as usize },
//         336usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(xid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).name as *const _ as usize },
//         344usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).args as *const _ as usize },
//         376usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(args)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).prev as *const _ as usize },
//         400usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(prev)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).next as *const _ as usize },
//         408usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(next)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).stat as *const _ as usize },
//         416usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(stat)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shred>())).m_loopCounters as *const _ as usize
//         },
//         424usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(m_loopCounters)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred>())).m_serials as *const _ as usize },
//         448usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred),
//             "::",
//             stringify!(m_serials)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_Shred_initialize(
        this: *mut self::root::Chuck_VM_Shred,
        c: *mut self::root::Chuck_VM_Code,
        mem_st_size: c_ulong,
        reg_st_size: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Shred_shutdown(this: *mut self::root::Chuck_VM_Shred) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn Chuck_VM_Shred_run(
        this: *mut self::root::Chuck_VM_Shred,
        vm: *mut self::root::Chuck_VM,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add"]
    pub fn Chuck_VM_Shred_add(
        this: *mut self::root::Chuck_VM_Shred,
        ugen: *mut self::root::Chuck_UGen,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_VM_Shred_remove(
        this: *mut self::root::Chuck_VM_Shred,
        ugen: *mut self::root::Chuck_UGen,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add_parent_ref"]
    pub fn Chuck_VM_Shred_add_parent_ref(
        this: *mut self::root::Chuck_VM_Shred,
        obj: *mut self::root::Chuck_Object,
    );
}
extern "C" {
    #[link_name = "\u{1}add_serialio"]
    pub fn Chuck_VM_Shred_add_serialio(
        this: *mut self::root::Chuck_VM_Shred,
        serial: *mut self::root::Chuck_IO_Serial,
    );
}
extern "C" {
    #[link_name = "\u{1}remove_serialio"]
    pub fn Chuck_VM_Shred_remove_serialio(
        this: *mut self::root::Chuck_VM_Shred,
        serial: *mut self::root::Chuck_IO_Serial,
    );
}
extern "C" {
    #[link_name = "\u{1}pushLoopCounter"]
    pub fn Chuck_VM_Shred_pushLoopCounter(this: *mut self::root::Chuck_VM_Shred) -> *mut c_ulong;
}
extern "C" {
    #[link_name = "\u{1}currentLoopCounter"]
    pub fn Chuck_VM_Shred_currentLoopCounter(this: *mut self::root::Chuck_VM_Shred)
        -> *mut c_ulong;
}
extern "C" {
    #[link_name = "\u{1}popLoopCounter"]
    pub fn Chuck_VM_Shred_popLoopCounter(this: *mut self::root::Chuck_VM_Shred) -> bool;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred"]
    pub fn Chuck_VM_Shred_Chuck_VM_Shred(this: *mut self::root::Chuck_VM_Shred);
}
impl Default for Chuck_VM_Shred {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VM_Shred {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_VM_Shred {{ mem: {:?}, reg: {:?}, base_ref: {:?}, code: {:?}, code_orig: {:?}, instr: {:?}, parent: {:?}, children: {:?}, pc: {:?}, vm_ref: {:?}, now: {:?}, start: {:?}, wake_time: {:?}, next_pc: {:?}, is_done: {:?}, is_running: {:?}, is_abort: {:?}, is_dumped: {:?}, event: {:?}, m_ugen_map: {:?}, m_parent_objects: {:?}, xid: {:?}, name: {:?}, args: {:?}, prev: {:?}, next: {:?}, stat: {:?}, m_loopCounters: {:?}, m_serials: {:?} }}" , self . mem , self . reg , self . base_ref , self . code , self . code_orig , self . instr , self . parent , self . children , self . pc , self . vm_ref , self . now , self . start , self . wake_time , self . next_pc , self . is_done , self . is_running , self . is_abort , self . is_dumped , self . event , self . m_ugen_map , self . m_parent_objects , self . xid , self . name , self . args , self . prev , self . next , self . stat , self . m_loopCounters , self . m_serials )
    }
}
impl Chuck_VM_Shred {
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        c: *mut self::root::Chuck_VM_Code,
        mem_st_size: c_ulong,
        reg_st_size: c_ulong,
    ) -> c_ulong {
        Chuck_VM_Shred_initialize(self, c, mem_st_size, reg_st_size)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_Shred_shutdown(self)
    }
    #[inline]
    pub unsafe fn run(&mut self, vm: *mut self::root::Chuck_VM) -> c_ulong {
        Chuck_VM_Shred_run(self, vm)
    }
    #[inline]
    pub unsafe fn add(&mut self, ugen: *mut self::root::Chuck_UGen) -> c_ulong {
        Chuck_VM_Shred_add(self, ugen)
    }
    #[inline]
    pub unsafe fn remove(&mut self, ugen: *mut self::root::Chuck_UGen) -> c_ulong {
        Chuck_VM_Shred_remove(self, ugen)
    }
    #[inline]
    pub unsafe fn add_parent_ref(&mut self, obj: *mut self::root::Chuck_Object) {
        Chuck_VM_Shred_add_parent_ref(self, obj)
    }
    #[inline]
    pub unsafe fn add_serialio(&mut self, serial: *mut self::root::Chuck_IO_Serial) {
        Chuck_VM_Shred_add_serialio(self, serial)
    }
    #[inline]
    pub unsafe fn remove_serialio(&mut self, serial: *mut self::root::Chuck_IO_Serial) {
        Chuck_VM_Shred_remove_serialio(self, serial)
    }
    #[inline]
    pub unsafe fn pushLoopCounter(&mut self) -> *mut c_ulong {
        Chuck_VM_Shred_pushLoopCounter(self)
    }
    #[inline]
    pub unsafe fn currentLoopCounter(&mut self) -> *mut c_ulong {
        Chuck_VM_Shred_currentLoopCounter(self)
    }
    #[inline]
    pub unsafe fn popLoopCounter(&mut self) -> bool {
        Chuck_VM_Shred_popLoopCounter(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_VM_Shred_Chuck_VM_Shred(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred_destructor"]
    pub fn Chuck_VM_Shred_Chuck_VM_Shred_destructor(this: *mut self::root::Chuck_VM_Shred);
}
#[repr(C)]
pub struct Chuck_VM_Shred_Status {
    pub _base: self::root::Chuck_Object,
    pub xid: c_ulong,
    pub name: self::root::std::__cxx11::string,
    pub start: f64,
    pub has_event: c_ulong,
}
// #[test]
// fn bindgen_test_layout_Chuck_VM_Shred_Status() {
//     assert_eq!(
//         size_of::<Chuck_VM_Shred_Status>(),
//         128usize,
//         concat!("Size of: ", stringify!(Chuck_VM_Shred_Status))
//     );
//     assert_eq!(
//         align_of::<Chuck_VM_Shred_Status>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VM_Shred_Status))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred_Status>())).xid as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred_Status),
//             "::",
//             stringify!(xid)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred_Status>())).name as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred_Status),
//             "::",
//             stringify!(name)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shred_Status>())).start as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred_Status),
//             "::",
//             stringify!(start)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shred_Status>())).has_event as *const _ as usize
//         },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shred_Status),
//             "::",
//             stringify!(has_event)
//         )
//     );
// }
impl Default for Chuck_VM_Shred_Status {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VM_Shred_Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_VM_Shred_Status {{ xid: {:?}, name: {:?}, start: {:?}, has_event: {:?} }}",
            self.xid, self.name, self.start, self.has_event
        )
    }
}
#[repr(C)]
pub struct Chuck_VM_Status {
    pub _base: self::root::Chuck_Object,
    pub srate: c_ulong,
    pub now_system: f64,
    pub t_second: c_ulong,
    pub t_minute: c_ulong,
    pub t_hour: c_ulong,
    pub list: self::root::std::vector,
}
// #[test]
// fn bindgen_test_layout_Chuck_VM_Status() {
//     assert_eq!(
//         size_of::<Chuck_VM_Status>(),
//         136usize,
//         concat!("Size of: ", stringify!(Chuck_VM_Status))
//     );
//     assert_eq!(
//         align_of::<Chuck_VM_Status>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VM_Status))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Status>())).srate as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Status),
//             "::",
//             stringify!(srate)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Status>())).now_system as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Status),
//             "::",
//             stringify!(now_system)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Status>())).t_second as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Status),
//             "::",
//             stringify!(t_second)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Status>())).t_minute as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Status),
//             "::",
//             stringify!(t_minute)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Status>())).t_hour as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Status),
//             "::",
//             stringify!(t_hour)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Status>())).list as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Status),
//             "::",
//             stringify!(list)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_VM_Status_clear(this: *mut self::root::Chuck_VM_Status);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Status"]
    pub fn Chuck_VM_Status_Chuck_VM_Status(this: *mut self::root::Chuck_VM_Status);
}
impl Default for Chuck_VM_Status {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VM_Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_VM_Status {{ srate: {:?}, now_system: {:?}, t_second: {:?}, t_minute: {:?}, t_hour: {:?}, list: {:?} }}" , self . srate , self . now_system , self . t_second , self . t_minute , self . t_hour , self . list )
    }
}
impl PartialEq for Chuck_VM_Status {
    fn eq(&self, other: &Chuck_VM_Status) -> bool {
        self._base == other._base
            && self.srate == other.srate
            && self.now_system == other.now_system
            && self.t_second == other.t_second
            && self.t_minute == other.t_minute
            && self.t_hour == other.t_hour
            && self.list == other.list
    }
}
impl Chuck_VM_Status {
    #[inline]
    pub unsafe fn clear(&mut self) {
        Chuck_VM_Status_clear(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_VM_Status_Chuck_VM_Status(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Status_destructor"]
    pub fn Chuck_VM_Status_Chuck_VM_Status_destructor(this: *mut self::root::Chuck_VM_Status);
}
#[repr(C)]
pub struct Chuck_VM_Shreduler {
    pub _base: self::root::Chuck_Object,
    pub now_system: f64,
    pub rt_audio: c_ulong,
    pub vm_ref: *mut self::root::Chuck_VM,
    pub shred_list: *mut self::root::Chuck_VM_Shred,
    pub blocked: self::root::std::map,
    pub m_current_shred: *mut self::root::Chuck_VM_Shred,
    pub m_dac: *mut self::root::Chuck_UGen,
    pub m_adc: *mut self::root::Chuck_UGen,
    pub m_bunghole: *mut self::root::Chuck_UGen,
    pub m_num_dac_channels: c_ulong,
    pub m_num_adc_channels: c_ulong,
    pub m_status: self::root::Chuck_VM_Status,
    pub m_max_block_size: c_ulong,
    pub m_adaptive: c_ulong,
    pub m_samps_until_next: f64,
}
// #[test]
// fn bindgen_test_layout_Chuck_VM_Shreduler() {
//     assert_eq!(
//         size_of::<Chuck_VM_Shreduler>(),
//         360usize,
//         concat!("Size of: ", stringify!(Chuck_VM_Shreduler))
//     );
//     assert_eq!(
//         align_of::<Chuck_VM_Shreduler>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VM_Shreduler))
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).now_system as *const _ as usize
//         },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(now_system)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).rt_audio as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(rt_audio)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).vm_ref as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(vm_ref)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).shred_list as *const _ as usize
//         },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(shred_list)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).blocked as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(blocked)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_current_shred as *const _ as usize
//         },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_current_shred)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_dac as *const _ as usize },
//         160usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_dac)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_adc as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_adc)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_bunghole as *const _ as usize
//         },
//         176usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_bunghole)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_num_dac_channels as *const _
//                 as usize
//         },
//         184usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_num_dac_channels)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_num_adc_channels as *const _
//                 as usize
//         },
//         192usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_num_adc_channels)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_status as *const _ as usize },
//         200usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_status)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_max_block_size as *const _ as usize
//         },
//         336usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_max_block_size)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_adaptive as *const _ as usize
//         },
//         344usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_adaptive)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM_Shreduler>())).m_samps_until_next as *const _
//                 as usize
//         },
//         352usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM_Shreduler),
//             "::",
//             stringify!(m_samps_until_next)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_Shreduler_initialize(this: *mut self::root::Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Shreduler_shutdown(this: *mut self::root::Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shredule"]
    pub fn Chuck_VM_Shreduler_shredule(
        this: *mut self::root::Chuck_VM_Shreduler,
        shred: *mut self::root::Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shredule"]
    pub fn Chuck_VM_Shreduler_shredule1(
        this: *mut self::root::Chuck_VM_Shreduler,
        shred: *mut self::root::Chuck_VM_Shred,
        wake_time: f64,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_VM_Shreduler_get(
        this: *mut self::root::Chuck_VM_Shreduler,
    ) -> *mut self::root::Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}advance"]
    pub fn Chuck_VM_Shreduler_advance(this: *mut self::root::Chuck_VM_Shreduler, N: c_long);
}
extern "C" {
    #[link_name = "\u{1}advance_v"]
    pub fn Chuck_VM_Shreduler_advance_v(
        this: *mut self::root::Chuck_VM_Shreduler,
        num_left: *mut c_long,
        offset: *mut c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}set_adaptive"]
    pub fn Chuck_VM_Shreduler_set_adaptive(
        this: *mut self::root::Chuck_VM_Shreduler,
        max_block_size: c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_VM_Shreduler_remove(
        this: *mut self::root::Chuck_VM_Shreduler,
        shred: *mut self::root::Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}replace"]
    pub fn Chuck_VM_Shreduler_replace(
        this: *mut self::root::Chuck_VM_Shreduler,
        out: *mut self::root::Chuck_VM_Shred,
        in_: *mut self::root::Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}lookup"]
    pub fn Chuck_VM_Shreduler_lookup(
        this: *mut self::root::Chuck_VM_Shreduler,
        xid: c_ulong,
    ) -> *mut self::root::Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}status"]
    pub fn Chuck_VM_Shreduler_status(this: *mut self::root::Chuck_VM_Shreduler);
}
extern "C" {
    #[link_name = "\u{1}status"]
    pub fn Chuck_VM_Shreduler_status1(
        this: *mut self::root::Chuck_VM_Shreduler,
        status: *mut self::root::Chuck_VM_Status,
    );
}
extern "C" {
    #[link_name = "\u{1}highest"]
    pub fn Chuck_VM_Shreduler_highest(this: *mut self::root::Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add_blocked"]
    pub fn Chuck_VM_Shreduler_add_blocked(
        this: *mut self::root::Chuck_VM_Shreduler,
        shred: *mut self::root::Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_blocked"]
    pub fn Chuck_VM_Shreduler_remove_blocked(
        this: *mut self::root::Chuck_VM_Shreduler,
        shred: *mut self::root::Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shreduler"]
    pub fn Chuck_VM_Shreduler_Chuck_VM_Shreduler(this: *mut self::root::Chuck_VM_Shreduler);
}
impl Default for Chuck_VM_Shreduler {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VM_Shreduler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_VM_Shreduler {{ now_system: {:?}, rt_audio: {:?}, vm_ref: {:?}, shred_list: {:?}, blocked: {:?}, m_current_shred: {:?}, m_dac: {:?}, m_adc: {:?}, m_bunghole: {:?}, m_num_dac_channels: {:?}, m_num_adc_channels: {:?}, m_status: {:?}, m_max_block_size: {:?}, m_adaptive: {:?}, m_samps_until_next: {:?} }}" , self . now_system , self . rt_audio , self . vm_ref , self . shred_list , self . blocked , self . m_current_shred , self . m_dac , self . m_adc , self . m_bunghole , self . m_num_dac_channels , self . m_num_adc_channels , self . m_status , self . m_max_block_size , self . m_adaptive , self . m_samps_until_next )
    }
}
impl PartialEq for Chuck_VM_Shreduler {
    fn eq(&self, other: &Chuck_VM_Shreduler) -> bool {
        self._base == other._base
            && self.now_system == other.now_system
            && self.rt_audio == other.rt_audio
            && self.vm_ref == other.vm_ref
            && self.shred_list == other.shred_list
            && self.blocked == other.blocked
            && self.m_current_shred == other.m_current_shred
            && self.m_dac == other.m_dac
            && self.m_adc == other.m_adc
            && self.m_bunghole == other.m_bunghole
            && self.m_num_dac_channels == other.m_num_dac_channels
            && self.m_num_adc_channels == other.m_num_adc_channels
            && self.m_status == other.m_status
            && self.m_max_block_size == other.m_max_block_size
            && self.m_adaptive == other.m_adaptive
            && self.m_samps_until_next == other.m_samps_until_next
    }
}
impl Chuck_VM_Shreduler {
    #[inline]
    pub unsafe fn initialize(&mut self) -> c_ulong {
        Chuck_VM_Shreduler_initialize(self)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_Shreduler_shutdown(self)
    }
    #[inline]
    pub unsafe fn shredule(&mut self, shred: *mut self::root::Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_shredule(self, shred)
    }
    #[inline]
    pub unsafe fn shredule1(
        &mut self,
        shred: *mut self::root::Chuck_VM_Shred,
        wake_time: f64,
    ) -> c_ulong {
        Chuck_VM_Shreduler_shredule1(self, shred, wake_time)
    }
    #[inline]
    pub unsafe fn get(&mut self) -> *mut self::root::Chuck_VM_Shred {
        Chuck_VM_Shreduler_get(self)
    }
    #[inline]
    pub unsafe fn advance(&mut self, N: c_long) {
        Chuck_VM_Shreduler_advance(self, N)
    }
    #[inline]
    pub unsafe fn advance_v(&mut self, num_left: *mut c_long, offset: *mut c_long) {
        Chuck_VM_Shreduler_advance_v(self, num_left, offset)
    }
    #[inline]
    pub unsafe fn set_adaptive(&mut self, max_block_size: c_ulong) {
        Chuck_VM_Shreduler_set_adaptive(self, max_block_size)
    }
    #[inline]
    pub unsafe fn remove(&mut self, shred: *mut self::root::Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_remove(self, shred)
    }
    #[inline]
    pub unsafe fn replace(
        &mut self,
        out: *mut self::root::Chuck_VM_Shred,
        in_: *mut self::root::Chuck_VM_Shred,
    ) -> c_ulong {
        Chuck_VM_Shreduler_replace(self, out, in_)
    }
    #[inline]
    pub unsafe fn lookup(&mut self, xid: c_ulong) -> *mut self::root::Chuck_VM_Shred {
        Chuck_VM_Shreduler_lookup(self, xid)
    }
    #[inline]
    pub unsafe fn status(&mut self) {
        Chuck_VM_Shreduler_status(self)
    }
    #[inline]
    pub unsafe fn status1(&mut self, status: *mut self::root::Chuck_VM_Status) {
        Chuck_VM_Shreduler_status1(self, status)
    }
    #[inline]
    pub unsafe fn highest(&mut self) -> c_ulong {
        Chuck_VM_Shreduler_highest(self)
    }
    #[inline]
    pub unsafe fn add_blocked(&mut self, shred: *mut self::root::Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_add_blocked(self, shred)
    }
    #[inline]
    pub unsafe fn remove_blocked(&mut self, shred: *mut self::root::Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_remove_blocked(self, shred)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_VM_Shreduler_Chuck_VM_Shreduler(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shreduler_destructor"]
    pub fn Chuck_VM_Shreduler_Chuck_VM_Shreduler_destructor(
        this: *mut self::root::Chuck_VM_Shreduler,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Set_Global_Int_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Get_Global_Int_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Set_Global_Float_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Get_Global_Float_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Signal_Global_Event_Request {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Global_Int_Container {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Global_Float_Container {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Global_Event_Container {
    _unused: [u8; 0],
}
pub const Chuck_Global_Request_Type_set_global_int_request: self::root::Chuck_Global_Request_Type =
    0;
pub const Chuck_Global_Request_Type_get_global_int_request: self::root::Chuck_Global_Request_Type =
    1;
pub const Chuck_Global_Request_Type_set_global_float_request:
    self::root::Chuck_Global_Request_Type = 2;
pub const Chuck_Global_Request_Type_get_global_float_request:
    self::root::Chuck_Global_Request_Type = 3;
pub const Chuck_Global_Request_Type_signal_global_event_request:
    self::root::Chuck_Global_Request_Type = 4;
pub const Chuck_Global_Request_Type_spork_shred_request: self::root::Chuck_Global_Request_Type = 5;
pub type Chuck_Global_Request_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Chuck_Global_Request {
    pub type_: self::root::Chuck_Global_Request_Type,
    pub __bindgen_anon_1: self::root::Chuck_Global_Request__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Chuck_Global_Request__bindgen_ty_1 {
    pub setIntRequest: *mut self::root::Chuck_Set_Global_Int_Request,
    pub getIntRequest: *mut self::root::Chuck_Get_Global_Int_Request,
    pub setFloatRequest: *mut self::root::Chuck_Set_Global_Float_Request,
    pub getFloatRequest: *mut self::root::Chuck_Get_Global_Float_Request,
    pub signalEventRequest: *mut self::root::Chuck_Signal_Global_Event_Request,
    pub shred: *mut self::root::Chuck_VM_Shred,
    _bindgen_union_align: u64,
}
// #[test]
// fn bindgen_test_layout_Chuck_Global_Request__bindgen_ty_1() {
//     assert_eq!(
//         size_of::<Chuck_Global_Request__bindgen_ty_1>(),
//         8usize,
//         concat!("Size of: ", stringify!(Chuck_Global_Request__bindgen_ty_1))
//     );
//     assert_eq!(
//         align_of::<Chuck_Global_Request__bindgen_ty_1>(),
//         8usize,
//         concat!(
//             "Alignment of ",
//             stringify!(Chuck_Global_Request__bindgen_ty_1)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Global_Request__bindgen_ty_1>())).setIntRequest
//                 as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Global_Request__bindgen_ty_1),
//             "::",
//             stringify!(setIntRequest)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Global_Request__bindgen_ty_1>())).getIntRequest
//                 as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Global_Request__bindgen_ty_1),
//             "::",
//             stringify!(getIntRequest)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Global_Request__bindgen_ty_1>())).setFloatRequest
//                 as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Global_Request__bindgen_ty_1),
//             "::",
//             stringify!(setFloatRequest)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Global_Request__bindgen_ty_1>())).getFloatRequest
//                 as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Global_Request__bindgen_ty_1),
//             "::",
//             stringify!(getFloatRequest)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Global_Request__bindgen_ty_1>())).signalEventRequest
//                 as *const _ as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Global_Request__bindgen_ty_1),
//             "::",
//             stringify!(signalEventRequest)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Global_Request__bindgen_ty_1>())).shred as *const _
//                 as usize
//         },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Global_Request__bindgen_ty_1),
//             "::",
//             stringify!(shred)
//         )
//     );
// }
impl Default for Chuck_Global_Request__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Global_Request__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_Global_Request__bindgen_ty_1 {{ union }}")
    }
}
// #[test]
// fn bindgen_test_layout_Chuck_Global_Request() {
//     assert_eq!(
//         size_of::<Chuck_Global_Request>(),
//         16usize,
//         concat!("Size of: ", stringify!(Chuck_Global_Request))
//     );
//     assert_eq!(
//         align_of::<Chuck_Global_Request>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Global_Request))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Global_Request>())).type_ as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Global_Request),
//             "::",
//             stringify!(type_)
//         )
//     );
// }
impl Default for Chuck_Global_Request {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Global_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Global_Request {{ type: {:?}, __bindgen_anon_1: {:?} }}",
            self.type_, self.__bindgen_anon_1
        )
    }
}
#[repr(C)]
pub struct Chuck_VM {
    pub _base: self::root::Chuck_Object,
    pub m_carrier: *mut self::root::Chuck_Carrier,
    pub m_adc: *mut self::root::Chuck_UGen,
    pub m_dac: *mut self::root::Chuck_UGen,
    pub m_bunghole: *mut self::root::Chuck_UGen,
    pub m_srate: c_ulong,
    pub m_num_adc_channels: c_ulong,
    pub m_num_dac_channels: c_ulong,
    pub m_halt: c_ulong,
    pub m_is_running: c_ulong,
    pub m_input_ref: *const f64,
    pub m_output_ref: *mut f64,
    pub m_init: c_ulong,
    pub m_last_error: self::root::std::__cxx11::string,
    pub m_shreds: *mut self::root::Chuck_VM_Shred,
    pub m_num_shreds: c_ulong,
    pub m_shred_id: c_ulong,
    pub m_shreduler: *mut self::root::Chuck_VM_Shreduler,
    pub m_shred_dump: self::root::std::vector,
    pub m_num_dumped_shreds: c_ulong,
    pub m_msg_buffer: *mut self::root::CBufferSimple,
    pub m_reply_buffer: *mut self::root::CBufferSimple,
    pub m_event_buffer: *mut self::root::CBufferSimple,
    pub m_event_buffers: self::root::std::__cxx11::list,
    pub m_global_ints: self::root::std::map,
    pub m_global_floats: self::root::std::map,
    pub m_global_events: self::root::std::map,
    pub m_global_request_queue: self::root::XCircleBuffer<self::root::Chuck_Global_Request>,
}
// #[test]
// fn bindgen_test_layout_Chuck_VM() {
//     assert_eq!(
//         size_of::<Chuck_VM>(),
//         496usize,
//         concat!("Size of: ", stringify!(Chuck_VM))
//     );
//     assert_eq!(
//         align_of::<Chuck_VM>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_VM))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_carrier as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_carrier)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_adc as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_adc)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_dac as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_dac)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_bunghole as *const _ as usize },
//         96usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_bunghole)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_srate as *const _ as usize },
//         104usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_srate)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_num_adc_channels as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_num_adc_channels)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_num_dac_channels as *const _ as usize },
//         120usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_num_dac_channels)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_halt as *const _ as usize },
//         128usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_halt)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_is_running as *const _ as usize },
//         136usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_is_running)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_input_ref as *const _ as usize },
//         144usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_input_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_output_ref as *const _ as usize },
//         152usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_output_ref)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_init as *const _ as usize },
//         160usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_init)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_last_error as *const _ as usize },
//         168usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_last_error)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_shreds as *const _ as usize },
//         200usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_shreds)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_num_shreds as *const _ as usize },
//         208usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_num_shreds)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_shred_id as *const _ as usize },
//         216usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_shred_id)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_shreduler as *const _ as usize },
//         224usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_shreduler)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_shred_dump as *const _ as usize },
//         232usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_shred_dump)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM>())).m_num_dumped_shreds as *const _ as usize
//         },
//         256usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_num_dumped_shreds)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_msg_buffer as *const _ as usize },
//         264usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_msg_buffer)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_reply_buffer as *const _ as usize },
//         272usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_reply_buffer)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_event_buffer as *const _ as usize },
//         280usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_event_buffer)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_event_buffers as *const _ as usize },
//         288usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_event_buffers)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_global_ints as *const _ as usize },
//         312usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_global_ints)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_global_floats as *const _ as usize },
//         360usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_global_floats)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_VM>())).m_global_events as *const _ as usize },
//         408usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_global_events)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_VM>())).m_global_request_queue as *const _ as usize
//         },
//         456usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_VM),
//             "::",
//             stringify!(m_global_request_queue)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_initialize(
        this: *mut self::root::Chuck_VM,
        srate: c_ulong,
        dac_chan: c_ulong,
        adc_chan: c_ulong,
        adaptive: c_ulong,
        halt: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}initialize_synthesis"]
    pub fn Chuck_VM_initialize_synthesis(this: *mut self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_shutdown(this: *mut self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}start"]
    pub fn Chuck_VM_start(this: *mut self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}running"]
    pub fn Chuck_VM_running(this: *mut self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}stop"]
    pub fn Chuck_VM_stop(this: *mut self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}spork"]
    pub fn Chuck_VM_spork(
        this: *mut self::root::Chuck_VM,
        code: *mut self::root::Chuck_VM_Code,
        parent: *mut self::root::Chuck_VM_Shred,
        immediate: c_ulong,
    ) -> *mut self::root::Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}shreduler"]
    pub fn Chuck_VM_shreduler(
        this: *const self::root::Chuck_VM,
    ) -> *mut self::root::Chuck_VM_Shreduler;
}
extern "C" {
    #[link_name = "\u{1}next_id"]
    pub fn Chuck_VM_next_id(this: *mut self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}srate"]
    pub fn Chuck_VM_srate(this: *const self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn Chuck_VM_run(
        this: *mut self::root::Chuck_VM,
        numFrames: c_long,
        input: *const f64,
        output: *mut f64,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}compute"]
    pub fn Chuck_VM_compute(this: *mut self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}abort_current_shred"]
    pub fn Chuck_VM_abort_current_shred(this: *mut self::root::Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}invoke_static"]
    pub fn Chuck_VM_invoke_static(
        this: *mut self::root::Chuck_VM,
        shred: *mut self::root::Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}gc"]
    pub fn Chuck_VM_gc(this: *mut self::root::Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}gc"]
    pub fn Chuck_VM_gc1(this: *mut self::root::Chuck_VM, amount: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}queue_msg"]
    pub fn Chuck_VM_queue_msg(
        this: *mut self::root::Chuck_VM,
        msg: *mut self::root::Chuck_Msg,
        num_msg: c_int,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_event"]
    pub fn Chuck_VM_queue_event(
        this: *mut self::root::Chuck_VM,
        event: *mut self::root::Chuck_Event,
        num_msg: c_int,
        buffer: *mut self::root::CBufferSimple,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}process_msg"]
    pub fn Chuck_VM_process_msg(
        this: *mut self::root::Chuck_VM,
        msg: *mut self::root::Chuck_Msg,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_reply"]
    pub fn Chuck_VM_get_reply(this: *mut self::root::Chuck_VM) -> *mut self::root::Chuck_Msg;
}
extern "C" {
    #[link_name = "\u{1}create_event_buffer"]
    pub fn Chuck_VM_create_event_buffer(
        this: *mut self::root::Chuck_VM,
    ) -> *mut self::root::CBufferSimple;
}
extern "C" {
    #[link_name = "\u{1}destroy_event_buffer"]
    pub fn Chuck_VM_destroy_event_buffer(
        this: *mut self::root::Chuck_VM,
        buffer: *mut self::root::CBufferSimple,
    );
}
extern "C" {
    #[link_name = "\u{1}get_global_int"]
    pub fn Chuck_VM_get_global_int(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: c_long)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_global_int"]
    pub fn Chuck_VM_set_global_int(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
        val: c_long,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_float"]
    pub fn Chuck_VM_get_global_float(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_global_float"]
    pub fn Chuck_VM_set_global_float(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
        val: f64,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signal_global_event"]
    pub fn Chuck_VM_signal_global_event(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}broadcast_global_event"]
    pub fn Chuck_VM_broadcast_global_event(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}init_global_int"]
    pub fn Chuck_VM_init_global_int(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_int_value"]
    pub fn Chuck_VM_get_global_int_value(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_int"]
    pub fn Chuck_VM_get_ptr_to_global_int(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> *mut c_long;
}
extern "C" {
    #[link_name = "\u{1}init_global_float"]
    pub fn Chuck_VM_init_global_float(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_float_value"]
    pub fn Chuck_VM_get_global_float_value(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> f64;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_float"]
    pub fn Chuck_VM_get_ptr_to_global_float(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> *mut f64;
}
extern "C" {
    #[link_name = "\u{1}init_global_event"]
    pub fn Chuck_VM_init_global_event(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
        type_: *mut self::root::Chuck_Type,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_event"]
    pub fn Chuck_VM_get_global_event(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> *mut self::root::Chuck_Event;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_event"]
    pub fn Chuck_VM_get_ptr_to_global_event(
        this: *mut self::root::Chuck_VM,
        name: self::root::std::__cxx11::string,
    ) -> *mut *mut self::root::Chuck_Event;
}
extern "C" {
    #[link_name = "\u{1}handle_global_queue_messages"]
    pub fn Chuck_VM_handle_global_queue_messages(this: *mut self::root::Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}spork"]
    pub fn Chuck_VM_spork1(
        this: *mut self::root::Chuck_VM,
        shred: *mut self::root::Chuck_VM_Shred,
    ) -> *mut self::root::Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}free"]
    pub fn Chuck_VM_free(
        this: *mut self::root::Chuck_VM,
        shred: *mut self::root::Chuck_VM_Shred,
        cascade: c_ulong,
        dec: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}dump"]
    pub fn Chuck_VM_dump(this: *mut self::root::Chuck_VM, shred: *mut self::root::Chuck_VM_Shred);
}
extern "C" {
    #[link_name = "\u{1}release_dump"]
    pub fn Chuck_VM_release_dump(this: *mut self::root::Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM"]
    pub fn Chuck_VM_Chuck_VM(this: *mut self::root::Chuck_VM);
}
impl Default for Chuck_VM {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_VM {{ m_carrier: {:?}, m_adc: {:?}, m_dac: {:?}, m_bunghole: {:?}, m_srate: {:?}, m_num_adc_channels: {:?}, m_num_dac_channels: {:?}, m_halt: {:?}, m_is_running: {:?}, m_input_ref: {:?}, m_output_ref: {:?}, m_init: {:?}, m_last_error: {:?}, m_shreds: {:?}, m_num_shreds: {:?}, m_shred_id: {:?}, m_shreduler: {:?}, m_shred_dump: {:?}, m_num_dumped_shreds: {:?}, m_msg_buffer: {:?}, m_reply_buffer: {:?}, m_event_buffer: {:?}, m_event_buffers: {:?}, m_global_ints: {:?}, m_global_floats: {:?}, m_global_events: {:?}, m_global_request_queue: {:?} }}" , self . m_carrier , self . m_adc , self . m_dac , self . m_bunghole , self . m_srate , self . m_num_adc_channels , self . m_num_dac_channels , self . m_halt , self . m_is_running , self . m_input_ref , self . m_output_ref , self . m_init , self . m_last_error , self . m_shreds , self . m_num_shreds , self . m_shred_id , self . m_shreduler , self . m_shred_dump , self . m_num_dumped_shreds , self . m_msg_buffer , self . m_reply_buffer , self . m_event_buffer , self . m_event_buffers , self . m_global_ints , self . m_global_floats , self . m_global_events , self . m_global_request_queue )
    }
}
impl Chuck_VM {
    #[inline]
    pub unsafe fn initialize(
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
    pub unsafe fn initialize_synthesis(&mut self) -> c_ulong {
        Chuck_VM_initialize_synthesis(self)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_shutdown(self)
    }
    #[inline]
    pub unsafe fn start(&mut self) -> c_ulong {
        Chuck_VM_start(self)
    }
    #[inline]
    pub unsafe fn running(&mut self) -> c_ulong {
        Chuck_VM_running(self)
    }
    #[inline]
    pub unsafe fn stop(&mut self) -> c_ulong {
        Chuck_VM_stop(self)
    }
    #[inline]
    pub unsafe fn spork(
        &mut self,
        code: *mut self::root::Chuck_VM_Code,
        parent: *mut self::root::Chuck_VM_Shred,
        immediate: c_ulong,
    ) -> *mut self::root::Chuck_VM_Shred {
        Chuck_VM_spork(self, code, parent, immediate)
    }
    #[inline]
    pub unsafe fn shreduler(&self) -> *mut self::root::Chuck_VM_Shreduler {
        Chuck_VM_shreduler(self)
    }
    #[inline]
    pub unsafe fn next_id(&mut self) -> c_ulong {
        Chuck_VM_next_id(self)
    }
    #[inline]
    pub unsafe fn srate(&self) -> c_ulong {
        Chuck_VM_srate(self)
    }
    #[inline]
    pub unsafe fn run(
        &mut self,
        numFrames: c_long,
        input: *const f64,
        output: *mut f64,
    ) -> c_ulong {
        Chuck_VM_run(self, numFrames, input, output)
    }
    #[inline]
    pub unsafe fn compute(&mut self) -> c_ulong {
        Chuck_VM_compute(self)
    }
    #[inline]
    pub unsafe fn abort_current_shred(&mut self) -> c_ulong {
        Chuck_VM_abort_current_shred(self)
    }
    #[inline]
    pub unsafe fn invoke_static(&mut self, shred: *mut self::root::Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_invoke_static(self, shred)
    }
    #[inline]
    pub unsafe fn gc(&mut self) {
        Chuck_VM_gc(self)
    }
    #[inline]
    pub unsafe fn gc1(&mut self, amount: c_ulong) {
        Chuck_VM_gc1(self, amount)
    }
    #[inline]
    pub unsafe fn queue_msg(&mut self, msg: *mut self::root::Chuck_Msg, num_msg: c_int) -> c_ulong {
        Chuck_VM_queue_msg(self, msg, num_msg)
    }
    #[inline]
    pub unsafe fn queue_event(
        &mut self,
        event: *mut self::root::Chuck_Event,
        num_msg: c_int,
        buffer: *mut self::root::CBufferSimple,
    ) -> c_ulong {
        Chuck_VM_queue_event(self, event, num_msg, buffer)
    }
    #[inline]
    pub unsafe fn process_msg(&mut self, msg: *mut self::root::Chuck_Msg) -> c_ulong {
        Chuck_VM_process_msg(self, msg)
    }
    #[inline]
    pub unsafe fn get_reply(&mut self) -> *mut self::root::Chuck_Msg {
        Chuck_VM_get_reply(self)
    }
    #[inline]
    pub unsafe fn create_event_buffer(&mut self) -> *mut self::root::CBufferSimple {
        Chuck_VM_create_event_buffer(self)
    }
    #[inline]
    pub unsafe fn destroy_event_buffer(&mut self, buffer: *mut self::root::CBufferSimple) {
        Chuck_VM_destroy_event_buffer(self, buffer)
    }
    #[inline]
    pub unsafe fn get_global_int(
        &mut self,
        name: self::root::std::__cxx11::string,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: c_long)>,
    ) -> c_ulong {
        Chuck_VM_get_global_int(self, name, callback)
    }
    #[inline]
    pub unsafe fn set_global_int(
        &mut self,
        name: self::root::std::__cxx11::string,
        val: c_long,
    ) -> c_ulong {
        Chuck_VM_set_global_int(self, name, val)
    }
    #[inline]
    pub unsafe fn get_global_float(
        &mut self,
        name: self::root::std::__cxx11::string,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> c_ulong {
        Chuck_VM_get_global_float(self, name, callback)
    }
    #[inline]
    pub unsafe fn set_global_float(
        &mut self,
        name: self::root::std::__cxx11::string,
        val: f64,
    ) -> c_ulong {
        Chuck_VM_set_global_float(self, name, val)
    }
    #[inline]
    pub unsafe fn signal_global_event(
        &mut self,
        name: self::root::std::__cxx11::string,
    ) -> c_ulong {
        Chuck_VM_signal_global_event(self, name)
    }
    #[inline]
    pub unsafe fn broadcast_global_event(
        &mut self,
        name: self::root::std::__cxx11::string,
    ) -> c_ulong {
        Chuck_VM_broadcast_global_event(self, name)
    }
    #[inline]
    pub unsafe fn init_global_int(&mut self, name: self::root::std::__cxx11::string) -> c_ulong {
        Chuck_VM_init_global_int(self, name)
    }
    #[inline]
    pub unsafe fn get_global_int_value(
        &mut self,
        name: self::root::std::__cxx11::string,
    ) -> c_long {
        Chuck_VM_get_global_int_value(self, name)
    }
    #[inline]
    pub unsafe fn get_ptr_to_global_int(
        &mut self,
        name: self::root::std::__cxx11::string,
    ) -> *mut c_long {
        Chuck_VM_get_ptr_to_global_int(self, name)
    }
    #[inline]
    pub unsafe fn init_global_float(&mut self, name: self::root::std::__cxx11::string) -> c_ulong {
        Chuck_VM_init_global_float(self, name)
    }
    #[inline]
    pub unsafe fn get_global_float_value(&mut self, name: self::root::std::__cxx11::string) -> f64 {
        Chuck_VM_get_global_float_value(self, name)
    }
    #[inline]
    pub unsafe fn get_ptr_to_global_float(
        &mut self,
        name: self::root::std::__cxx11::string,
    ) -> *mut f64 {
        Chuck_VM_get_ptr_to_global_float(self, name)
    }
    #[inline]
    pub unsafe fn init_global_event(
        &mut self,
        name: self::root::std::__cxx11::string,
        type_: *mut self::root::Chuck_Type,
    ) -> c_ulong {
        Chuck_VM_init_global_event(self, name, type_)
    }
    #[inline]
    pub unsafe fn get_global_event(
        &mut self,
        name: self::root::std::__cxx11::string,
    ) -> *mut self::root::Chuck_Event {
        Chuck_VM_get_global_event(self, name)
    }
    #[inline]
    pub unsafe fn get_ptr_to_global_event(
        &mut self,
        name: self::root::std::__cxx11::string,
    ) -> *mut *mut self::root::Chuck_Event {
        Chuck_VM_get_ptr_to_global_event(self, name)
    }
    #[inline]
    pub unsafe fn handle_global_queue_messages(&mut self) {
        Chuck_VM_handle_global_queue_messages(self)
    }
    #[inline]
    pub unsafe fn spork1(
        &mut self,
        shred: *mut self::root::Chuck_VM_Shred,
    ) -> *mut self::root::Chuck_VM_Shred {
        Chuck_VM_spork1(self, shred)
    }
    #[inline]
    pub unsafe fn free(
        &mut self,
        shred: *mut self::root::Chuck_VM_Shred,
        cascade: c_ulong,
        dec: c_ulong,
    ) -> c_ulong {
        Chuck_VM_free(self, shred, cascade, dec)
    }
    #[inline]
    pub unsafe fn dump(&mut self, shred: *mut self::root::Chuck_VM_Shred) {
        Chuck_VM_dump(self, shred)
    }
    #[inline]
    pub unsafe fn release_dump(&mut self) {
        Chuck_VM_release_dump(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_VM_Chuck_VM(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_destructor"]
    pub fn Chuck_VM_Chuck_VM_destructor(this: *mut self::root::Chuck_VM);
}
pub type ck_msg_func =
    ::std::option::Option<unsafe extern "C" fn(msg: *const self::root::Chuck_Msg)>;
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Chuck_Msg {
    pub type_: c_ulong,
    pub param: c_ulong,
    pub code: *mut self::root::Chuck_VM_Code,
    pub shred: *mut self::root::Chuck_VM_Shred,
    pub when: f64,
    pub user: *mut c_void,
    pub reply: self::root::ck_msg_func,
    pub replyA: c_ulong,
    pub replyB: c_ulong,
    pub replyC: *mut c_void,
    pub args: *mut self::root::std::vector,
}
// #[test]
// fn bindgen_test_layout_Chuck_Msg() {
//     assert_eq!(
//         size_of::<Chuck_Msg>(),
//         88usize,
//         concat!("Size of: ", stringify!(Chuck_Msg))
//     );
//     assert_eq!(
//         align_of::<Chuck_Msg>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Msg))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).type_ as *const _ as usize },
//         0usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(type_)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).param as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(param)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).code as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(code)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).shred as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(shred)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).when as *const _ as usize },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(when)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).user as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(user)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).reply as *const _ as usize },
//         48usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(reply)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).replyA as *const _ as usize },
//         56usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(replyA)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).replyB as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(replyB)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).replyC as *const _ as usize },
//         72usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(replyC)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Msg>())).args as *const _ as usize },
//         80usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Msg),
//             "::",
//             stringify!(args)
//         )
//     );
// }
impl Default for Chuck_Msg {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
pub struct Chuck_Compiler__bindgen_vtable(c_void);
#[repr(C)]
pub struct Chuck_Compiler {
    pub vtable_: *const Chuck_Compiler__bindgen_vtable,
    pub m_carrier: *mut self::root::Chuck_Carrier,
    pub emitter: *mut self::root::Chuck_Emitter,
    pub code: *mut self::root::Chuck_VM_Code,
    pub m_auto_depend: c_ulong,
    pub m_recent: self::root::std::map,
    pub m_dlls: self::root::std::__cxx11::list,
    pub m_cklibs_to_preload: self::root::std::__cxx11::list,
}
// #[test]
// fn bindgen_test_layout_Chuck_Compiler() {
//     assert_eq!(
//         size_of::<Chuck_Compiler>(),
//         136usize,
//         concat!("Size of: ", stringify!(Chuck_Compiler))
//     );
//     assert_eq!(
//         align_of::<Chuck_Compiler>(),
//         8usize,
//         concat!("Alignment of ", stringify!(Chuck_Compiler))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Compiler>())).m_carrier as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Compiler),
//             "::",
//             stringify!(m_carrier)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Compiler>())).emitter as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Compiler),
//             "::",
//             stringify!(emitter)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Compiler>())).code as *const _ as usize },
//         24usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Compiler),
//             "::",
//             stringify!(code)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Compiler>())).m_auto_depend as *const _ as usize
//         },
//         32usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Compiler),
//             "::",
//             stringify!(m_auto_depend)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Compiler>())).m_recent as *const _ as usize },
//         40usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Compiler),
//             "::",
//             stringify!(m_recent)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<Chuck_Compiler>())).m_dlls as *const _ as usize },
//         88usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Compiler),
//             "::",
//             stringify!(m_dlls)
//         )
//     );
//     assert_eq!(
//         unsafe {
//             &(*(::std::ptr::null::<Chuck_Compiler>())).m_cklibs_to_preload as *const _ as usize
//         },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(Chuck_Compiler),
//             "::",
//             stringify!(m_cklibs_to_preload)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_Compiler_initialize(
        this: *mut self::root::Chuck_Compiler,
        chugin_search_paths: *mut self::root::std::__cxx11::list,
        named_dls: *mut self::root::std::__cxx11::list,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_Compiler_shutdown(this: *mut self::root::Chuck_Compiler);
}
extern "C" {
    #[link_name = "\u{1}bind"]
    pub fn Chuck_Compiler_bind(
        this: *mut self::root::Chuck_Compiler,
        query_func: self::root::f_ck_query,
        name: *const self::root::std::__cxx11::string,
        nspc: *const self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_auto_depend"]
    pub fn Chuck_Compiler_set_auto_depend(this: *mut self::root::Chuck_Compiler, v: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}go"]
    pub fn Chuck_Compiler_go(
        this: *mut self::root::Chuck_Compiler,
        filename: *const self::root::std::__cxx11::string,
        fd: *mut self::root::FILE,
        str_src: *const c_char,
        full_path: *const self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}resolve"]
    pub fn Chuck_Compiler_resolve(
        this: *mut self::root::Chuck_Compiler,
        type_: *const self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}output"]
    pub fn Chuck_Compiler_output(
        this: *mut self::root::Chuck_Compiler,
    ) -> *mut self::root::Chuck_VM_Code;
}
extern "C" {
    #[link_name = "\u{1}do_entire_file"]
    pub fn Chuck_Compiler_do_entire_file(
        this: *mut self::root::Chuck_Compiler,
        context: *mut self::root::Chuck_Context,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_only_classes"]
    pub fn Chuck_Compiler_do_only_classes(
        this: *mut self::root::Chuck_Compiler,
        context: *mut self::root::Chuck_Context,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_all_except_classes"]
    pub fn Chuck_Compiler_do_all_except_classes(
        this: *mut self::root::Chuck_Compiler,
        context: *mut self::root::Chuck_Context,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_normal"]
    pub fn Chuck_Compiler_do_normal(
        this: *mut self::root::Chuck_Compiler,
        path: *const self::root::std::__cxx11::string,
        fd: *mut self::root::FILE,
        str_src: *const c_char,
        full_path: *const self::root::std::__cxx11::string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}find_recent_path"]
    pub fn Chuck_Compiler_find_recent_path(
        this: *mut self::root::Chuck_Compiler,
        path: *const self::root::std::__cxx11::string,
    ) -> *mut self::root::Chuck_Context;
}
extern "C" {
    #[link_name = "\u{1}find_recent_type"]
    pub fn Chuck_Compiler_find_recent_type(
        this: *mut self::root::Chuck_Compiler,
        type_: *const self::root::std::__cxx11::string,
    ) -> *mut self::root::Chuck_Context;
}
extern "C" {
    #[link_name = "\u{1}add_recent_path"]
    pub fn Chuck_Compiler_add_recent_path(
        this: *mut self::root::Chuck_Compiler,
        path: *const self::root::std::__cxx11::string,
        context: *mut self::root::Chuck_Context,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Compiler"]
    pub fn Chuck_Compiler_Chuck_Compiler(this: *mut self::root::Chuck_Compiler);
}
impl Default for Chuck_Compiler {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Compiler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Compiler {{ m_carrier: {:?}, emitter: {:?}, code: {:?}, m_auto_depend: {:?}, m_recent: {:?}, m_dlls: {:?}, m_cklibs_to_preload: {:?} }}" , self . m_carrier , self . emitter , self . code , self . m_auto_depend , self . m_recent , self . m_dlls , self . m_cklibs_to_preload )
    }
}
impl Chuck_Compiler {
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        chugin_search_paths: *mut self::root::std::__cxx11::list,
        named_dls: *mut self::root::std::__cxx11::list,
    ) -> c_ulong {
        Chuck_Compiler_initialize(self, chugin_search_paths, named_dls)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) {
        Chuck_Compiler_shutdown(self)
    }
    #[inline]
    pub unsafe fn bind(
        &mut self,
        query_func: self::root::f_ck_query,
        name: *const self::root::std::__cxx11::string,
        nspc: *const self::root::std::__cxx11::string,
    ) -> c_ulong {
        Chuck_Compiler_bind(self, query_func, name, nspc)
    }
    #[inline]
    pub unsafe fn set_auto_depend(&mut self, v: c_ulong) {
        Chuck_Compiler_set_auto_depend(self, v)
    }
    #[inline]
    pub unsafe fn go(
        &mut self,
        filename: *const self::root::std::__cxx11::string,
        fd: *mut self::root::FILE,
        str_src: *const c_char,
        full_path: *const self::root::std::__cxx11::string,
    ) -> c_ulong {
        Chuck_Compiler_go(self, filename, fd, str_src, full_path)
    }
    #[inline]
    pub unsafe fn resolve(&mut self, type_: *const self::root::std::__cxx11::string) -> c_ulong {
        Chuck_Compiler_resolve(self, type_)
    }
    #[inline]
    pub unsafe fn output(&mut self) -> *mut self::root::Chuck_VM_Code {
        Chuck_Compiler_output(self)
    }
    #[inline]
    pub unsafe fn do_entire_file(&mut self, context: *mut self::root::Chuck_Context) -> c_ulong {
        Chuck_Compiler_do_entire_file(self, context)
    }
    #[inline]
    pub unsafe fn do_only_classes(&mut self, context: *mut self::root::Chuck_Context) -> c_ulong {
        Chuck_Compiler_do_only_classes(self, context)
    }
    #[inline]
    pub unsafe fn do_all_except_classes(
        &mut self,
        context: *mut self::root::Chuck_Context,
    ) -> c_ulong {
        Chuck_Compiler_do_all_except_classes(self, context)
    }
    #[inline]
    pub unsafe fn do_normal(
        &mut self,
        path: *const self::root::std::__cxx11::string,
        fd: *mut self::root::FILE,
        str_src: *const c_char,
        full_path: *const self::root::std::__cxx11::string,
    ) -> c_ulong {
        Chuck_Compiler_do_normal(self, path, fd, str_src, full_path)
    }
    #[inline]
    pub unsafe fn find_recent_path(
        &mut self,
        path: *const self::root::std::__cxx11::string,
    ) -> *mut self::root::Chuck_Context {
        Chuck_Compiler_find_recent_path(self, path)
    }
    #[inline]
    pub unsafe fn find_recent_type(
        &mut self,
        type_: *const self::root::std::__cxx11::string,
    ) -> *mut self::root::Chuck_Context {
        Chuck_Compiler_find_recent_type(self, type_)
    }
    #[inline]
    pub unsafe fn add_recent_path(
        &mut self,
        path: *const self::root::std::__cxx11::string,
        context: *mut self::root::Chuck_Context,
    ) -> c_ulong {
        Chuck_Compiler_add_recent_path(self, path, context)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Compiler_Chuck_Compiler(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Compiler_destructor"]
    pub fn Chuck_Compiler_Chuck_Compiler_destructor(this: *mut self::root::Chuck_Compiler);
}
#[repr(C)]
pub struct ChucK__bindgen_vtable(c_void);
#[repr(C)]
pub struct ChucK {
    pub vtable_: *const ChucK__bindgen_vtable,
    pub m_carrier: *mut self::root::Chuck_Carrier,
    pub m_params: self::root::std::map,
    pub m_listParams: self::root::std::map,
    pub m_init: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}o_isGlobalInit"]
    pub static mut ChucK_o_isGlobalInit: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}enableSystemCall"]
    pub static mut ChucK_enableSystemCall: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}VERSION"]
    pub static mut ChucK_VERSION: [c_char; 0usize];
}
extern "C" {
    #[link_name = "\u{1}o_numVMs"]
    pub static mut ChucK_o_numVMs: c_ulong;
}
// #[test]
// fn bindgen_test_layout_ChucK() {
//     assert_eq!(
//         size_of::<ChucK>(),
//         120usize,
//         concat!("Size of: ", stringify!(ChucK))
//     );
//     assert_eq!(
//         align_of::<ChucK>(),
//         8usize,
//         concat!("Alignment of ", stringify!(ChucK))
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<ChucK>())).m_carrier as *const _ as usize },
//         8usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(ChucK),
//             "::",
//             stringify!(m_carrier)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<ChucK>())).m_params as *const _ as usize },
//         16usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(ChucK),
//             "::",
//             stringify!(m_params)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<ChucK>())).m_listParams as *const _ as usize },
//         64usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(ChucK),
//             "::",
//             stringify!(m_listParams)
//         )
//     );
//     assert_eq!(
//         unsafe { &(*(::std::ptr::null::<ChucK>())).m_init as *const _ as usize },
//         112usize,
//         concat!(
//             "Offset of field: ",
//             stringify!(ChucK),
//             "::",
//             stringify!(m_init)
//         )
//     );
// }
extern "C" {
    #[link_name = "\u{1}setParam"]
    pub fn ChucK_setParam(
        this: *mut self::root::ChucK,
        name: *const self::root::std::__cxx11::string,
        value: c_long,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}setParamFloat"]
    pub fn ChucK_setParamFloat(
        this: *mut self::root::ChucK,
        name: *const self::root::std::__cxx11::string,
        value: f64,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}setParam"]
    pub fn ChucK_setParam1(
        this: *mut self::root::ChucK,
        name: *const self::root::std::__cxx11::string,
        value: *const self::root::std::__cxx11::string,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}setParam"]
    pub fn ChucK_setParam2(
        this: *mut self::root::ChucK,
        name: *const self::root::std::__cxx11::string,
        value: *const self::root::std::__cxx11::list,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}getParamInt"]
    pub fn ChucK_getParamInt(
        this: *mut self::root::ChucK,
        key: *const self::root::std::__cxx11::string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}getParamFloat"]
    pub fn ChucK_getParamFloat(
        this: *mut self::root::ChucK,
        key: *const self::root::std::__cxx11::string,
    ) -> f64;
}
extern "C" {
    #[link_name = "\u{1}getParamString"]
    pub fn ChucK_getParamString(
        this: *mut self::root::ChucK,
        key: *const self::root::std::__cxx11::string,
    ) -> self::root::std::__cxx11::string;
}
extern "C" {
    #[link_name = "\u{1}getParamStringList"]
    pub fn ChucK_getParamStringList(
        this: *mut self::root::ChucK,
        key: *const self::root::std::__cxx11::string,
    ) -> self::root::std::__cxx11::list;
}
extern "C" {
    #[link_name = "\u{1}compileFile"]
    pub fn ChucK_compileFile(
        this: *mut self::root::ChucK,
        path: *const self::root::std::__cxx11::string,
        argsTogether: *const self::root::std::__cxx11::string,
        count: c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}compileCode"]
    pub fn ChucK_compileCode(
        this: *mut self::root::ChucK,
        code: *const self::root::std::__cxx11::string,
        argsTogether: *const self::root::std::__cxx11::string,
        count: c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn ChucK_init(this: *mut self::root::ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}start"]
    pub fn ChucK_start(this: *mut self::root::ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn ChucK_run(
        this: *mut self::root::ChucK,
        input: *mut f64,
        output: *mut f64,
        numFrames: c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}bind"]
    pub fn ChucK_bind(
        this: *mut self::root::ChucK,
        queryFunc: self::root::f_ck_query,
        name: *const self::root::std::__cxx11::string,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}setGlobalInt"]
    pub fn ChucK_setGlobalInt(
        this: *mut self::root::ChucK,
        name: *const c_char,
        val: c_long,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}getGlobalInt"]
    pub fn ChucK_getGlobalInt(
        this: *mut self::root::ChucK,
        name: *const c_char,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: c_long)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setGlobalFloat"]
    pub fn ChucK_setGlobalFloat(
        this: *mut self::root::ChucK,
        name: *const c_char,
        val: f64,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}getGlobalFloat"]
    pub fn ChucK_getGlobalFloat(
        this: *mut self::root::ChucK,
        name: *const c_char,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signalGlobalEvent"]
    pub fn ChucK_signalGlobalEvent(this: *mut self::root::ChucK, name: *const c_char) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}broadcastGlobalEvent"]
    pub fn ChucK_broadcastGlobalEvent(this: *mut self::root::ChucK, name: *const c_char)
        -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setChoutCallback"]
    pub fn ChucK_setChoutCallback(
        this: *mut self::root::ChucK,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setCherrCallback"]
    pub fn ChucK_setCherrCallback(
        this: *mut self::root::ChucK,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setStdoutCallback"]
    pub fn ChucK_setStdoutCallback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setStderrCallback"]
    pub fn ChucK_setStderrCallback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}globalInit"]
    pub fn ChucK_globalInit() -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}globalCleanup"]
    pub fn ChucK_globalCleanup();
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn ChucK_shutdown(this: *mut self::root::ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}version"]
    pub fn ChucK_version() -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}intSize"]
    pub fn ChucK_intSize() -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}poop"]
    pub fn ChucK_poop();
}
extern "C" {
    #[link_name = "\u{1}setLogLevel"]
    pub fn ChucK_setLogLevel(level: c_long);
}
extern "C" {
    #[link_name = "\u{1}getLogLevel"]
    pub fn ChucK_getLogLevel() -> c_long;
}
extern "C" {
    #[link_name = "\u{1}initDefaultParams"]
    pub fn ChucK_initDefaultParams(this: *mut self::root::ChucK);
}
extern "C" {
    #[link_name = "\u{1}initVM"]
    pub fn ChucK_initVM(this: *mut self::root::ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}initCompiler"]
    pub fn ChucK_initCompiler(this: *mut self::root::ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}initChugins"]
    pub fn ChucK_initChugins(this: *mut self::root::ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}initOTF"]
    pub fn ChucK_initOTF(this: *mut self::root::ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}ChucK"]
    pub fn ChucK_ChucK(this: *mut self::root::ChucK);
}
impl Default for ChucK {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for ChucK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "ChucK {{ m_carrier: {:?}, m_params: {:?}, m_listParams: {:?}, m_init: {:?} }}",
            self.m_carrier, self.m_params, self.m_listParams, self.m_init
        )
    }
}
impl ChucK {
    #[inline]
    pub unsafe fn setParam(
        &mut self,
        name: *const self::root::std::__cxx11::string,
        value: c_long,
    ) -> bool {
        ChucK_setParam(self, name, value)
    }
    #[inline]
    pub unsafe fn setParamFloat(
        &mut self,
        name: *const self::root::std::__cxx11::string,
        value: f64,
    ) -> bool {
        ChucK_setParamFloat(self, name, value)
    }
    #[inline]
    pub unsafe fn setParam1(
        &mut self,
        name: *const self::root::std::__cxx11::string,
        value: *const self::root::std::__cxx11::string,
    ) -> bool {
        ChucK_setParam1(self, name, value)
    }
    #[inline]
    pub unsafe fn setParam2(
        &mut self,
        name: *const self::root::std::__cxx11::string,
        value: *const self::root::std::__cxx11::list,
    ) -> bool {
        ChucK_setParam2(self, name, value)
    }
    #[inline]
    pub unsafe fn getParamInt(&mut self, key: *const self::root::std::__cxx11::string) -> c_long {
        ChucK_getParamInt(self, key)
    }
    #[inline]
    pub unsafe fn getParamFloat(&mut self, key: *const self::root::std::__cxx11::string) -> f64 {
        ChucK_getParamFloat(self, key)
    }
    #[inline]
    pub unsafe fn getParamString(
        &mut self,
        key: *const self::root::std::__cxx11::string,
    ) -> self::root::std::__cxx11::string {
        ChucK_getParamString(self, key)
    }
    #[inline]
    pub unsafe fn getParamStringList(
        &mut self,
        key: *const self::root::std::__cxx11::string,
    ) -> self::root::std::__cxx11::list {
        ChucK_getParamStringList(self, key)
    }
    #[inline]
    pub unsafe fn compileFile(
        &mut self,
        path: *const self::root::std::__cxx11::string,
        argsTogether: *const self::root::std::__cxx11::string,
        count: c_int,
    ) -> bool {
        ChucK_compileFile(self, path, argsTogether, count)
    }
    #[inline]
    pub unsafe fn compileCode(
        &mut self,
        code: *const self::root::std::__cxx11::string,
        argsTogether: *const self::root::std::__cxx11::string,
        count: c_int,
    ) -> bool {
        ChucK_compileCode(self, code, argsTogether, count)
    }
    #[inline]
    pub unsafe fn init(&mut self) -> bool {
        ChucK_init(self)
    }
    #[inline]
    pub unsafe fn start(&mut self) -> bool {
        ChucK_start(self)
    }
    #[inline]
    pub unsafe fn run(&mut self, input: *mut f64, output: *mut f64, numFrames: c_int) {
        ChucK_run(self, input, output, numFrames)
    }
    #[inline]
    pub unsafe fn bind(
        &mut self,
        queryFunc: self::root::f_ck_query,
        name: *const self::root::std::__cxx11::string,
    ) -> bool {
        ChucK_bind(self, queryFunc, name)
    }
    #[inline]
    pub unsafe fn setGlobalInt(&mut self, name: *const c_char, val: c_long) -> c_ulong {
        ChucK_setGlobalInt(self, name, val)
    }
    #[inline]
    pub unsafe fn getGlobalInt(
        &mut self,
        name: *const c_char,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: c_long)>,
    ) -> c_ulong {
        ChucK_getGlobalInt(self, name, callback)
    }
    #[inline]
    pub unsafe fn setGlobalFloat(&mut self, name: *const c_char, val: f64) -> c_ulong {
        ChucK_setGlobalFloat(self, name, val)
    }
    #[inline]
    pub unsafe fn getGlobalFloat(
        &mut self,
        name: *const c_char,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> c_ulong {
        ChucK_getGlobalFloat(self, name, callback)
    }
    #[inline]
    pub unsafe fn signalGlobalEvent(&mut self, name: *const c_char) -> c_ulong {
        ChucK_signalGlobalEvent(self, name)
    }
    #[inline]
    pub unsafe fn broadcastGlobalEvent(&mut self, name: *const c_char) -> c_ulong {
        ChucK_broadcastGlobalEvent(self, name)
    }
    #[inline]
    pub unsafe fn setChoutCallback(
        &mut self,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) -> c_ulong {
        ChucK_setChoutCallback(self, callback)
    }
    #[inline]
    pub unsafe fn setCherrCallback(
        &mut self,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) -> c_ulong {
        ChucK_setCherrCallback(self, callback)
    }
    #[inline]
    pub unsafe fn setStdoutCallback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) -> c_ulong {
        ChucK_setStdoutCallback(callback)
    }
    #[inline]
    pub unsafe fn setStderrCallback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) -> c_ulong {
        ChucK_setStderrCallback(callback)
    }
    #[inline]
    pub unsafe fn globalInit() -> c_ulong {
        ChucK_globalInit()
    }
    #[inline]
    pub unsafe fn globalCleanup() {
        ChucK_globalCleanup()
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> bool {
        ChucK_shutdown(self)
    }
    #[inline]
    pub unsafe fn version() -> *const c_char {
        ChucK_version()
    }
    #[inline]
    pub unsafe fn intSize() -> c_ulong {
        ChucK_intSize()
    }
    #[inline]
    pub unsafe fn poop() {
        ChucK_poop()
    }
    #[inline]
    pub unsafe fn setLogLevel(level: c_long) {
        ChucK_setLogLevel(level)
    }
    #[inline]
    pub unsafe fn getLogLevel() -> c_long {
        ChucK_getLogLevel()
    }
    #[inline]
    pub unsafe fn initDefaultParams(&mut self) {
        ChucK_initDefaultParams(self)
    }
    #[inline]
    pub unsafe fn initVM(&mut self) -> bool {
        ChucK_initVM(self)
    }
    #[inline]
    pub unsafe fn initCompiler(&mut self) -> bool {
        ChucK_initCompiler(self)
    }
    #[inline]
    pub unsafe fn initChugins(&mut self) -> bool {
        ChucK_initChugins(self)
    }
    #[inline]
    pub unsafe fn initOTF(&mut self) -> bool {
        ChucK_initOTF(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        ChucK_ChucK(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}ChucK_destructor"]
    pub fn ChucK_ChucK_destructor(this: *mut self::root::ChucK);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __locale_data {
    pub _address: u8,
}
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_stringstream_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_stringstream<c_char>>(),
//         392usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_stringstream<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_stringstream<c_char>>(
//         ),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_stringstream<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_fstream_open0_char_char_traits_open1_char_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::basic_fstream<c_char>>(),
//         528usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::basic_fstream<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::basic_fstream<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::basic_fstream<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_ptr_WvOut_ptr_WvOut__bindgen_ty_id_62358_open1_ptr_WvOut_close1_allocator_open1_pair_open2_ptr_WvOut_ptr_WvOut_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_ptr_WvOut_ptr_WvOut_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_pair_open0_ptr_WvOut_ptr_WvOut_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::pair<*mut self::root::WvOut, *mut self::root::WvOut>>(),
//         16usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::WvOut, *mut self::root::WvOut>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::pair<*mut self::root::WvOut, *mut self::root::WvOut>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::WvOut, *mut self::root::WvOut>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_iterator_open0_output_iterator_tag_void_void_void_void_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::iterator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::iterator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::iterator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::iterator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout___basic_file_open0_char_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::__basic_file>(),
//         16usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__basic_file)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__basic_file>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__basic_file)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_VM_Object_allocator_open1_ptr_Chuck_VM_Object_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_VM_Object_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Func_allocator_open1_ptr_Chuck_Func_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Func_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_unsigned_long_allocator_open1_unsigned_long_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_unsigned_long_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_unsigned_long_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_unsigned_long_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_3() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_unsigned_long_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_4() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_3() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_queue_open0_ptr_Chuck_VM_Shred_deque_open1_ptr_Chuck_VM_Shred_allocator_open2_ptr_Chuck_VM_Shred_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::queue<self::root::std::deque>>(),
//         80usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::queue<self::root::std::deque>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::queue<self::root::std::deque>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::queue<self::root::std::deque>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_deque_open0_ptr_Chuck_VM_Shred_allocator_open1_ptr_Chuck_VM_Shred_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::deque>(),
//         80usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::deque)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::deque>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::deque)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_VM_Shred_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Class_allocator_open1_ptr_Chuck_DL_Class_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Class_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Class_allocator_open1_ptr_Chuck_DL_Class_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Class_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Func_allocator_open1_ptr_Chuck_DL_Func_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Func_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Func_allocator_open1_ptr_Chuck_DL_Func_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Func_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Func_allocator_open1_ptr_Chuck_DL_Func_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Func_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Value_allocator_open1_ptr_Chuck_DL_Value_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Value_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Value_allocator_open1_ptr_Chuck_DL_Value_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Value_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Ctrl_allocator_open1_ptr_Chuck_DL_Ctrl_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Ctrl_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Class_allocator_open1_ptr_Chuck_DL_Class_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Class_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_5() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_4() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_DL_Value_allocator_open1_ptr_Chuck_DL_Value_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DL_Value_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_4(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_6() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_5() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_map_allocator_open1_ptr_map_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_S_Symbol_ptr_Chuck_VM_Object__bindgen_ty_id_65478_open1_ptr_S_Symbol__close1_allocator_open1_pair_open2_ptr_S_Symbol__ptr_Chuck_VM_Object_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_ptr_S_Symbol__ptr_Chuck_VM_Object_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_pair_open0_ptr_S_Symbol__ptr_Chuck_VM_Object_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>>(
//         ),
//         16usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>>(
//         ),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_map_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_ptr_S_Symbol__ptr_Chuck_VM_Object__bindgen_ty_id_65498_open1_ptr_S_Symbol__close1_allocator_open1_pair_open2_ptr_S_Symbol__ptr_Chuck_VM_Object_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_ptr_S_Symbol__ptr_Chuck_VM_Object_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_pair_open0_ptr_S_Symbol__ptr_Chuck_VM_Object_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>>(
//         ),
//         16usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>>(
//         ),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_S_Symbol_ptr_Chuck_VM_Object__bindgen_ty_id_65517_open1_ptr_S_Symbol__close1_allocator_open1_pair_open2_ptr_S_Symbol__ptr_Chuck_VM_Object_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_ptr_S_Symbol__ptr_Chuck_VM_Object_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_pair_open0_ptr_S_Symbol__ptr_Chuck_VM_Object_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>>(
//         ),
//         16usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>>(
//         ),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::S_Symbol_, *mut self::root::Chuck_VM_Object>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_Chuck_Scope_open0_ptr_Chuck_Type_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::Chuck_Scope>(),
//         72usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::Chuck_Scope)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::Chuck_Scope>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::Chuck_Scope)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_Chuck_Scope_open0_ptr_Chuck_Value_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::Chuck_Scope>(),
//         72usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::Chuck_Scope)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::Chuck_Scope>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::Chuck_Scope)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_Chuck_Scope_open0_ptr_Chuck_Func_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::Chuck_Scope>(),
//         72usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::Chuck_Scope)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::Chuck_Scope>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::Chuck_Scope)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Type_allocator_open1_ptr_Chuck_Type_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Type_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Func_allocator_open1_ptr_Chuck_Func_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Func_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_VM_Object_allocator_open1_ptr_Chuck_VM_Object_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_VM_Object_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_VM_Object_allocator_open1_ptr_Chuck_VM_Object_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_VM_Object_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_VM_Object_allocator_open1_ptr_Chuck_VM_Object_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_VM_Object_close0_instantiation_3() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_VM_Object_allocator_open1_ptr_Chuck_VM_Object_close1_close0_instantiation_4(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_VM_Object_close0_instantiation_4() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_ptr_Chuck_Namespace_ptr_Chuck_Namespace__bindgen_ty_id_65640_open1_ptr_Chuck_Namespace_close1_allocator_open1_pair_open2_ptr_Chuck_Namespace_ptr_Chuck_Namespace_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_ptr_Chuck_Namespace_ptr_Chuck_Namespace_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_pair_open0_ptr_Chuck_Namespace_ptr_Chuck_Namespace_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<
//             self::root::std::pair<*mut self::root::Chuck_Namespace, *mut self::root::Chuck_Namespace>,
//         >(),
//         16usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::Chuck_Namespace, *mut self::root::Chuck_Namespace>)
//         )
//     );
//     assert_eq!(
//         align_of::<
//             self::root::std::pair<*mut self::root::Chuck_Namespace, *mut self::root::Chuck_Namespace>,
//         >(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::pair<*mut self::root::Chuck_Namespace, *mut self::root::Chuck_Namespace>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Namespace_allocator_open1_ptr_Chuck_Namespace_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Namespace_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Type_allocator_open1_ptr_Chuck_Type_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Type_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Context_allocator_open1_ptr_Chuck_Context_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Context_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_a_Stmt_allocator_open1_ptr_a_Stmt__close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_a_Stmt__close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_unsigned_long_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_unsigned_long_close2_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_5(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_7() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_6() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_unsigned_long_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_6(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_8() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_7() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_unsigned_long_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_unsigned_long_close2_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_7(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_9() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_8() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_unsigned_long_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_8(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_10() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_9() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_unsigned_long_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_unsigned_long_close2_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_9(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_11() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_10() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_unsigned_long_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_10(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_12() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_11() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_string_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_4(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_11(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_13() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_12() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_12(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_14() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_13() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_13(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_15() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_14() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_14(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_16() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_15() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Local_allocator_open1_ptr_Chuck_Local_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Local_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Local_allocator_open1_ptr_Chuck_Local_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Local_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Local_allocator_open1_ptr_Chuck_Local_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Local_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Instr_allocator_open1_ptr_Chuck_Instr_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Instr_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Instr_Goto_allocator_open1_ptr_Chuck_Instr_Goto_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Instr_Goto_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Instr_Goto_allocator_open1_ptr_Chuck_Instr_Goto_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Instr_Goto_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Instr_Goto_allocator_open1_ptr_Chuck_Instr_Goto_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Instr_Goto_close0_instantiation_2() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Code_allocator_open1_ptr_Chuck_Code_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Code_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Local_allocator_open1_ptr_Chuck_Local_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Local_close0_instantiation_3() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_queue_open0_double_deque_open1_double_allocator_open2_double_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::queue<self::root::std::deque>>(),
//         80usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::queue<self::root::std::deque>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::queue<self::root::std::deque>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::queue<self::root::std::deque>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_deque_open0_double_allocator_open1_double_close1_close0_instantiation()
// {
//     assert_eq!(
//         size_of::<self::root::std::deque>(),
//         80usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::deque)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::deque>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::deque)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_double_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_queue_open0_unsigned_long_deque_open1_unsigned_long_allocator_open2_unsigned_long_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::queue<self::root::std::deque>>(),
//         80usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::queue<self::root::std::deque>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::queue<self::root::std::deque>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::queue<self::root::std::deque>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_deque_open0_unsigned_long_allocator_open1_unsigned_long_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::deque>(),
//         80usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::deque)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::deque>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::deque)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_unsigned_long_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Shred_Stat_allocator_open1_ptr_Shred_Stat_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Shred_Stat_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Shred_Stat_allocator_open1_ptr_Shred_Stat_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Shred_Stat_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_Shred_Activation_allocator_open1_Shred_Activation_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_Shred_Activation_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_Shred_Activation_allocator_open1_Shred_Activation_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_Shred_Activation_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_unsigned_long_ptr_Chuck_VM_Shred_less_open1_unsigned_long_close1_allocator_open1_pair_open2_const_unsigned_long_ptr_Chuck_VM_Shred_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_unsigned_long_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_const_unsigned_long_ptr_Chuck_VM_Shred_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_ptr_Chuck_UGen_ptr_Chuck_UGen__bindgen_ty_id_66929_open1_ptr_Chuck_UGen_close1_allocator_open1_pair_open2_ptr_Chuck_UGen_ptr_Chuck_UGen_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_ptr_Chuck_UGen_ptr_Chuck_UGen_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_Object_allocator_open1_ptr_Chuck_Object_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_Object_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_15(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_17() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_16() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_unsigned_long_allocator_open1_ptr_unsigned_long_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_unsigned_long_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_VM_Shred_Status_allocator_open1_ptr_Chuck_VM_Shred_Status_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_VM_Shred_Status_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_ptr_Chuck_VM_Shred_ptr_Chuck_VM_Shred__bindgen_ty_id_67043_open1_ptr_Chuck_VM_Shred_close1_allocator_open1_pair_open2_ptr_Chuck_VM_Shred_ptr_Chuck_VM_Shred_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_ptr_Chuck_VM_Shred_ptr_Chuck_VM_Shred_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_ptr_Chuck_VM_Shred_allocator_open1_ptr_Chuck_VM_Shred_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_VM_Shred_close0_instantiation_1() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_ptr_CBufferSimple_allocator_open1_ptr_CBufferSimple_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_CBufferSimple_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_ptr_Chuck_Global_Int_Container_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_ptr_Chuck_Global_Int_Container_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_5(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_16(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_18() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_17() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_ptr_Chuck_Global_Int_Container_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_17(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_19() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_18() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_ptr_Chuck_Global_Float_Container_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_ptr_Chuck_Global_Float_Container_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_6(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_18(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_20() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_19() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_ptr_Chuck_Global_Float_Container_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_19(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_21() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_20() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_ptr_Chuck_Global_Event_Container_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_ptr_Chuck_Global_Event_Container_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_7(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_20(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_22() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_21() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_ptr_Chuck_Global_Event_Container_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_21(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_23() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_22() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_XCircleBuffer_open0_Chuck_Global_Request_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::XCircleBuffer<self::root::Chuck_Global_Request>>(),
//         40usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::XCircleBuffer<self::root::Chuck_Global_Request>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::XCircleBuffer<self::root::Chuck_Global_Request>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::XCircleBuffer<self::root::Chuck_Global_Request>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_vector_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_4(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::vector>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::vector>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::vector)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_4(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_22(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_24() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_23() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_ptr_Chuck_Context_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_ptr_Chuck_Context_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_8(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_23(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_25() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_24() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_ptr_Chuck_Context_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_24(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_26() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_25() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_ptr_Chuck_DLL_allocator_open1_ptr_Chuck_DLL_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_ptr_Chuck_DLL_close0_instantiation() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_5(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_25(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_27() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_26() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_6(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_26(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_28() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_27() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_2(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_7(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_27(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_29() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_28() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_3(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_8(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_28(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_30() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_29() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_4(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_9(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_29(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_31() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_30() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_string_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_close2_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_9(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_30(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_32() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_31() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_1(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_31(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_33() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_32() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_32(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_34() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_33() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_map_open0_string_list_open1_string_allocator_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_close2_close1_less_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_allocator_open1_pair_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_list_open3_basic_string_open4_char_char_traits_open5_char_close5_allocator_open5_char_close5_close4_allocator_open4_basic_string_open5_char_char_traits_open6_char_close6_allocator_open6_char_close6_close5_close4_close3_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::map>(),
//         48usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::map>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::map)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_string_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation_5(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_10(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_33(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_35() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_34() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_less_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_10(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::less>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::less)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_34(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_36() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_35() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_pair_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_list_open2_basic_string_open3_char_char_traits_open4_char_close4_allocator_open4_char_close4_close3_allocator_open3_basic_string_open4_char_char_traits_open5_char_close5_allocator_open5_char_close5_close4_close3_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_35(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_37() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_36() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_list_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_allocator_open1_basic_string_open2_char_char_traits_open3_char_close3_allocator_open3_char_close3_close2_close1_close0_instantiation(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::list>(),
//         24usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::list>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::list)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_36(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_38() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_37() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_basic_string_open1_char_char_traits_open2_char_close2_allocator_open2_char_close2_close1_close0_instantiation_11(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_basic_string_open0_char_char_traits_open1_char_close1_allocator_open1_char_close1_close0_instantiation_37(
// ) {
//     assert_eq!(
//         size_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         32usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::__cxx11::basic_string<c_char>>(),
//         8usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::__cxx11::basic_string<c_char>)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_char_traits_open0_char_close0_instantiation_39() {
//     assert_eq!(
//         size_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::char_traits>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::char_traits)
//         )
//     );
// }
// #[test]
// fn __bindgen_test_layout_allocator_open0_char_close0_instantiation_38() {
//     assert_eq!(
//         size_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Size of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
//     assert_eq!(
//         align_of::<self::root::std::allocator>(),
//         1usize,
//         concat!(
//             "Alignment of template specialization: ",
//             stringify!(self::root::std::allocator)
//         )
//     );
// }
