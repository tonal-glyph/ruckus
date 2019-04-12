use std::cell::*;
use std::clone::*;
use std::cmp::*;
use std::default::*;
use std::fmt::*;
use std::marker::*;
use std::mem::*;
use std::option::*;
use std::os::raw::*;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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

pub const __llvm__: u32 = 1;
pub const __clang__: u32 = 1;
pub const __clang_major__: u32 = 8;
pub const __clang_minor__: u32 = 0;
pub const __clang_patchlevel__: u32 = 0;
pub const __clang_version__ : & 'static [ u8 ; 165usize ] = b"8.0.0 (https://github.com/llvm-mirror/clang 27ff8dcc77fd7c9f1bcf181b25eaa7d68777fdfe) (https://github.com/llvm-mirror/llvm 718039ebb75d709b91dcc3ca18eddedb283892fd)\0" ;
pub const __GNUC_MINOR__: u32 = 2;
pub const __GNUC_PATCHLEVEL__: u32 = 1;
pub const __GNUC__: u32 = 4;
pub const __GXX_ABI_VERSION: u32 = 1002;
pub const __ATOMIC_RELAXED: u32 = 0;
pub const __ATOMIC_CONSUME: u32 = 1;
pub const __ATOMIC_ACQUIRE: u32 = 2;
pub const __ATOMIC_RELEASE: u32 = 3;
pub const __ATOMIC_ACQ_REL: u32 = 4;
pub const __ATOMIC_SEQ_CST: u32 = 5;
pub const __OPENCL_MEMORY_SCOPE_WORK_ITEM: u32 = 0;
pub const __OPENCL_MEMORY_SCOPE_WORK_GROUP: u32 = 1;
pub const __OPENCL_MEMORY_SCOPE_DEVICE: u32 = 2;
pub const __OPENCL_MEMORY_SCOPE_ALL_SVM_DEVICES: u32 = 3;
pub const __OPENCL_MEMORY_SCOPE_SUB_GROUP: u32 = 4;
pub const __PRAGMA_REDEFINE_EXTNAME: u32 = 1;
pub const __VERSION__ : & 'static [ u8 ; 188usize ] = b"4.2.1 Compatible Clang 8.0.0 (https://github.com/llvm-mirror/clang 27ff8dcc77fd7c9f1bcf181b25eaa7d68777fdfe) (https://github.com/llvm-mirror/llvm 718039ebb75d709b91dcc3ca18eddedb283892fd)\0" ;
pub const __STRICT_ANSI__: u32 = 1;
pub const __GXX_EXPERIMENTAL_CXX0X__: u32 = 1;
pub const __OBJC_BOOL_IS_BOOL: u32 = 0;
pub const __cpp_rtti: u32 = 199711;
pub const __cpp_exceptions: u32 = 199711;
pub const __cpp_unicode_characters: u32 = 200704;
pub const __cpp_raw_strings: u32 = 200710;
pub const __cpp_unicode_literals: u32 = 200710;
pub const __cpp_user_defined_literals: u32 = 200809;
pub const __cpp_lambdas: u32 = 200907;
pub const __cpp_constexpr: u32 = 201603;
pub const __cpp_range_based_for: u32 = 201603;
pub const __cpp_static_assert: u32 = 201411;
pub const __cpp_decltype: u32 = 200707;
pub const __cpp_attributes: u32 = 200809;
pub const __cpp_rvalue_references: u32 = 200610;
pub const __cpp_variadic_templates: u32 = 200704;
pub const __cpp_initializer_lists: u32 = 200806;
pub const __cpp_delegating_constructors: u32 = 200604;
pub const __cpp_nsdmi: u32 = 200809;
pub const __cpp_inheriting_constructors: u32 = 201511;
pub const __cpp_ref_qualifiers: u32 = 200710;
pub const __cpp_alias_templates: u32 = 200704;
pub const __cpp_threadsafe_static_init: u32 = 200806;
pub const __cpp_binary_literals: u32 = 201304;
pub const __cpp_digit_separators: u32 = 201309;
pub const __cpp_init_captures: u32 = 201304;
pub const __cpp_generic_lambdas: u32 = 201304;
pub const __cpp_decltype_auto: u32 = 201304;
pub const __cpp_return_type_deduction: u32 = 201304;
pub const __cpp_aggregate_nsdmi: u32 = 201304;
pub const __cpp_variable_templates: u32 = 201304;
pub const __cpp_hex_float: u32 = 201603;
pub const __cpp_inline_variables: u32 = 201606;
pub const __cpp_noexcept_function_type: u32 = 201510;
pub const __cpp_capture_star_this: u32 = 201603;
pub const __cpp_if_constexpr: u32 = 201606;
pub const __cpp_deduction_guides: u32 = 201703;
pub const __cpp_template_auto: u32 = 201606;
pub const __cpp_namespace_attributes: u32 = 201411;
pub const __cpp_enumerator_attributes: u32 = 201411;
pub const __cpp_nested_namespace_definitions: u32 = 201411;
pub const __cpp_variadic_using: u32 = 201611;
pub const __cpp_aggregate_bases: u32 = 201603;
pub const __cpp_structured_bindings: u32 = 201606;
pub const __cpp_nontype_template_args: u32 = 201411;
pub const __cpp_fold_expressions: u32 = 201603;
pub const __cpp_guaranteed_copy_elision: u32 = 201606;
pub const __cpp_nontype_template_parameter_auto: u32 = 201606;
pub const __cpp_aligned_new: u32 = 201606;
pub const __CONSTANT_CFSTRINGS__: u32 = 1;
pub const __EXCEPTIONS: u32 = 1;
pub const __GXX_RTTI: u32 = 1;
pub const __DEPRECATED: u32 = 1;
pub const __GNUG__: u32 = 4;
pub const __GXX_WEAK__: u32 = 1;
pub const __OPTIMIZE__: u32 = 1;
pub const __ORDER_LITTLE_ENDIAN__: u32 = 1234;
pub const __ORDER_BIG_ENDIAN__: u32 = 4321;
pub const __ORDER_PDP_ENDIAN__: u32 = 3412;
pub const __BYTE_ORDER__: u32 = 1234;
pub const __LITTLE_ENDIAN__: u32 = 1;
pub const _LP64: u32 = 1;
pub const __LP64__: u32 = 1;
pub const __CHAR_BIT__: u32 = 8;
pub const __SCHAR_MAX__: u32 = 127;
pub const __SHRT_MAX__: u32 = 32767;
pub const __INT_MAX__: u32 = 2147483647;
pub const __LONG_MAX__: u64 = 9223372036854775807;
pub const __LONG_LONG_MAX__: u64 = 9223372036854775807;
pub const __WCHAR_MAX__: u32 = 2147483647;
pub const __WINT_MAX__: u32 = 4294967295;
pub const __INTMAX_MAX__: u64 = 9223372036854775807;
pub const __SIZE_MAX__: i32 = -1;
pub const __UINTMAX_MAX__: i32 = -1;
pub const __PTRDIFF_MAX__: u64 = 9223372036854775807;
pub const __INTPTR_MAX__: u64 = 9223372036854775807;
pub const __UINTPTR_MAX__: i32 = -1;
pub const __SIZEOF_DOUBLE__: u32 = 8;
pub const __SIZEOF_FLOAT__: u32 = 4;
pub const __SIZEOF_INT__: u32 = 4;
pub const __SIZEOF_LONG__: u32 = 8;
pub const __SIZEOF_LONG_DOUBLE__: u32 = 16;
pub const __SIZEOF_LONG_LONG__: u32 = 8;
pub const __SIZEOF_POINTER__: u32 = 8;
pub const __SIZEOF_SHORT__: u32 = 2;
pub const __SIZEOF_PTRDIFF_T__: u32 = 8;
pub const __SIZEOF_SIZE_T__: u32 = 8;
pub const __SIZEOF_WCHAR_T__: u32 = 4;
pub const __SIZEOF_WINT_T__: u32 = 4;
pub const __SIZEOF_INT128__: u32 = 16;
pub const __INTMAX_FMTd__: &'static [u8; 3usize] = b"ld\0";
pub const __INTMAX_FMTi__: &'static [u8; 3usize] = b"li\0";
pub const __UINTMAX_FMTo__: &'static [u8; 3usize] = b"lo\0";
pub const __UINTMAX_FMTu__: &'static [u8; 3usize] = b"lu\0";
pub const __UINTMAX_FMTx__: &'static [u8; 3usize] = b"lx\0";
pub const __UINTMAX_FMTX__: &'static [u8; 3usize] = b"lX\0";
pub const __INTMAX_WIDTH__: u32 = 64;
pub const __PTRDIFF_FMTd__: &'static [u8; 3usize] = b"ld\0";
pub const __PTRDIFF_FMTi__: &'static [u8; 3usize] = b"li\0";
pub const __PTRDIFF_WIDTH__: u32 = 64;
pub const __INTPTR_FMTd__: &'static [u8; 3usize] = b"ld\0";
pub const __INTPTR_FMTi__: &'static [u8; 3usize] = b"li\0";
pub const __INTPTR_WIDTH__: u32 = 64;
pub const __SIZE_FMTo__: &'static [u8; 3usize] = b"lo\0";
pub const __SIZE_FMTu__: &'static [u8; 3usize] = b"lu\0";
pub const __SIZE_FMTx__: &'static [u8; 3usize] = b"lx\0";
pub const __SIZE_FMTX__: &'static [u8; 3usize] = b"lX\0";
pub const __SIZE_WIDTH__: u32 = 64;
pub const __WCHAR_WIDTH__: u32 = 32;
pub const __WINT_WIDTH__: u32 = 32;
pub const __SIG_ATOMIC_WIDTH__: u32 = 32;
pub const __SIG_ATOMIC_MAX__: u32 = 2147483647;
pub const __UINTMAX_WIDTH__: u32 = 64;
pub const __UINTPTR_FMTo__: &'static [u8; 3usize] = b"lo\0";
pub const __UINTPTR_FMTu__: &'static [u8; 3usize] = b"lu\0";
pub const __UINTPTR_FMTx__: &'static [u8; 3usize] = b"lx\0";
pub const __UINTPTR_FMTX__: &'static [u8; 3usize] = b"lX\0";
pub const __UINTPTR_WIDTH__: u32 = 64;
pub const __FLT16_HAS_DENORM__: u32 = 1;
pub const __FLT16_DIG__: u32 = 3;
pub const __FLT16_DECIMAL_DIG__: u32 = 5;
pub const __FLT16_HAS_INFINITY__: u32 = 1;
pub const __FLT16_HAS_QUIET_NAN__: u32 = 1;
pub const __FLT16_MANT_DIG__: u32 = 11;
pub const __FLT16_MAX_10_EXP__: u32 = 4;
pub const __FLT16_MAX_EXP__: u32 = 15;
pub const __FLT16_MIN_10_EXP__: i32 = -13;
pub const __FLT16_MIN_EXP__: i32 = -14;
pub const __FLT_HAS_DENORM__: u32 = 1;
pub const __FLT_DIG__: u32 = 6;
pub const __FLT_DECIMAL_DIG__: u32 = 9;
pub const __FLT_HAS_INFINITY__: u32 = 1;
pub const __FLT_HAS_QUIET_NAN__: u32 = 1;
pub const __FLT_MANT_DIG__: u32 = 24;
pub const __FLT_MAX_10_EXP__: u32 = 38;
pub const __FLT_MAX_EXP__: u32 = 128;
pub const __FLT_MIN_10_EXP__: i32 = -37;
pub const __FLT_MIN_EXP__: i32 = -125;
pub const __DBL_HAS_DENORM__: u32 = 1;
pub const __DBL_DIG__: u32 = 15;
pub const __DBL_DECIMAL_DIG__: u32 = 17;
pub const __DBL_HAS_INFINITY__: u32 = 1;
pub const __DBL_HAS_QUIET_NAN__: u32 = 1;
pub const __DBL_MANT_DIG__: u32 = 53;
pub const __DBL_MAX_10_EXP__: u32 = 308;
pub const __DBL_MAX_EXP__: u32 = 1024;
pub const __DBL_MIN_10_EXP__: i32 = -307;
pub const __DBL_MIN_EXP__: i32 = -1021;
pub const __LDBL_HAS_DENORM__: u32 = 1;
pub const __LDBL_DIG__: u32 = 18;
pub const __LDBL_DECIMAL_DIG__: u32 = 21;
pub const __LDBL_HAS_INFINITY__: u32 = 1;
pub const __LDBL_HAS_QUIET_NAN__: u32 = 1;
pub const __LDBL_MANT_DIG__: u32 = 64;
pub const __LDBL_MAX_10_EXP__: u32 = 4932;
pub const __LDBL_MAX_EXP__: u32 = 16384;
pub const __LDBL_MIN_10_EXP__: i32 = -4931;
pub const __LDBL_MIN_EXP__: i32 = -16381;
pub const __POINTER_WIDTH__: u32 = 64;
pub const __BIGGEST_ALIGNMENT__: u32 = 16;
pub const __WINT_UNSIGNED__: u32 = 1;
pub const __INT8_FMTd__: &'static [u8; 4usize] = b"hhd\0";
pub const __INT8_FMTi__: &'static [u8; 4usize] = b"hhi\0";
pub const __INT16_FMTd__: &'static [u8; 3usize] = b"hd\0";
pub const __INT16_FMTi__: &'static [u8; 3usize] = b"hi\0";
pub const __INT32_FMTd__: &'static [u8; 2usize] = b"d\0";
pub const __INT32_FMTi__: &'static [u8; 2usize] = b"i\0";
pub const __INT64_FMTd__: &'static [u8; 3usize] = b"ld\0";
pub const __INT64_FMTi__: &'static [u8; 3usize] = b"li\0";
pub const __UINT8_FMTo__: &'static [u8; 4usize] = b"hho\0";
pub const __UINT8_FMTu__: &'static [u8; 4usize] = b"hhu\0";
pub const __UINT8_FMTx__: &'static [u8; 4usize] = b"hhx\0";
pub const __UINT8_FMTX__: &'static [u8; 4usize] = b"hhX\0";
pub const __UINT8_MAX__: u32 = 255;
pub const __INT8_MAX__: u32 = 127;
pub const __UINT16_FMTo__: &'static [u8; 3usize] = b"ho\0";
pub const __UINT16_FMTu__: &'static [u8; 3usize] = b"hu\0";
pub const __UINT16_FMTx__: &'static [u8; 3usize] = b"hx\0";
pub const __UINT16_FMTX__: &'static [u8; 3usize] = b"hX\0";
pub const __UINT16_MAX__: u32 = 65535;
pub const __INT16_MAX__: u32 = 32767;
pub const __UINT32_FMTo__: &'static [u8; 2usize] = b"o\0";
pub const __UINT32_FMTu__: &'static [u8; 2usize] = b"u\0";
pub const __UINT32_FMTx__: &'static [u8; 2usize] = b"x\0";
pub const __UINT32_FMTX__: &'static [u8; 2usize] = b"X\0";
pub const __UINT32_MAX__: u32 = 4294967295;
pub const __INT32_MAX__: u32 = 2147483647;
pub const __UINT64_FMTo__: &'static [u8; 3usize] = b"lo\0";
pub const __UINT64_FMTu__: &'static [u8; 3usize] = b"lu\0";
pub const __UINT64_FMTx__: &'static [u8; 3usize] = b"lx\0";
pub const __UINT64_FMTX__: &'static [u8; 3usize] = b"lX\0";
pub const __UINT64_MAX__: i32 = -1;
pub const __INT64_MAX__: u64 = 9223372036854775807;
pub const __INT_LEAST8_MAX__: u32 = 127;
pub const __INT_LEAST8_FMTd__: &'static [u8; 4usize] = b"hhd\0";
pub const __INT_LEAST8_FMTi__: &'static [u8; 4usize] = b"hhi\0";
pub const __UINT_LEAST8_MAX__: u32 = 255;
pub const __UINT_LEAST8_FMTo__: &'static [u8; 4usize] = b"hho\0";
pub const __UINT_LEAST8_FMTu__: &'static [u8; 4usize] = b"hhu\0";
pub const __UINT_LEAST8_FMTx__: &'static [u8; 4usize] = b"hhx\0";
pub const __UINT_LEAST8_FMTX__: &'static [u8; 4usize] = b"hhX\0";
pub const __INT_LEAST16_MAX__: u32 = 32767;
pub const __INT_LEAST16_FMTd__: &'static [u8; 3usize] = b"hd\0";
pub const __INT_LEAST16_FMTi__: &'static [u8; 3usize] = b"hi\0";
pub const __UINT_LEAST16_MAX__: u32 = 65535;
pub const __UINT_LEAST16_FMTo__: &'static [u8; 3usize] = b"ho\0";
pub const __UINT_LEAST16_FMTu__: &'static [u8; 3usize] = b"hu\0";
pub const __UINT_LEAST16_FMTx__: &'static [u8; 3usize] = b"hx\0";
pub const __UINT_LEAST16_FMTX__: &'static [u8; 3usize] = b"hX\0";
pub const __INT_LEAST32_MAX__: u32 = 2147483647;
pub const __INT_LEAST32_FMTd__: &'static [u8; 2usize] = b"d\0";
pub const __INT_LEAST32_FMTi__: &'static [u8; 2usize] = b"i\0";
pub const __UINT_LEAST32_MAX__: u32 = 4294967295;
pub const __UINT_LEAST32_FMTo__: &'static [u8; 2usize] = b"o\0";
pub const __UINT_LEAST32_FMTu__: &'static [u8; 2usize] = b"u\0";
pub const __UINT_LEAST32_FMTx__: &'static [u8; 2usize] = b"x\0";
pub const __UINT_LEAST32_FMTX__: &'static [u8; 2usize] = b"X\0";
pub const __INT_LEAST64_MAX__: u64 = 9223372036854775807;
pub const __INT_LEAST64_FMTd__: &'static [u8; 3usize] = b"ld\0";
pub const __INT_LEAST64_FMTi__: &'static [u8; 3usize] = b"li\0";
pub const __UINT_LEAST64_MAX__: i32 = -1;
pub const __UINT_LEAST64_FMTo__: &'static [u8; 3usize] = b"lo\0";
pub const __UINT_LEAST64_FMTu__: &'static [u8; 3usize] = b"lu\0";
pub const __UINT_LEAST64_FMTx__: &'static [u8; 3usize] = b"lx\0";
pub const __UINT_LEAST64_FMTX__: &'static [u8; 3usize] = b"lX\0";
pub const __INT_FAST8_MAX__: u32 = 127;
pub const __INT_FAST8_FMTd__: &'static [u8; 4usize] = b"hhd\0";
pub const __INT_FAST8_FMTi__: &'static [u8; 4usize] = b"hhi\0";
pub const __UINT_FAST8_MAX__: u32 = 255;
pub const __UINT_FAST8_FMTo__: &'static [u8; 4usize] = b"hho\0";
pub const __UINT_FAST8_FMTu__: &'static [u8; 4usize] = b"hhu\0";
pub const __UINT_FAST8_FMTx__: &'static [u8; 4usize] = b"hhx\0";
pub const __UINT_FAST8_FMTX__: &'static [u8; 4usize] = b"hhX\0";
pub const __INT_FAST16_MAX__: u32 = 32767;
pub const __INT_FAST16_FMTd__: &'static [u8; 3usize] = b"hd\0";
pub const __INT_FAST16_FMTi__: &'static [u8; 3usize] = b"hi\0";
pub const __UINT_FAST16_MAX__: u32 = 65535;
pub const __UINT_FAST16_FMTo__: &'static [u8; 3usize] = b"ho\0";
pub const __UINT_FAST16_FMTu__: &'static [u8; 3usize] = b"hu\0";
pub const __UINT_FAST16_FMTx__: &'static [u8; 3usize] = b"hx\0";
pub const __UINT_FAST16_FMTX__: &'static [u8; 3usize] = b"hX\0";
pub const __INT_FAST32_MAX__: u32 = 2147483647;
pub const __INT_FAST32_FMTd__: &'static [u8; 2usize] = b"d\0";
pub const __INT_FAST32_FMTi__: &'static [u8; 2usize] = b"i\0";
pub const __UINT_FAST32_MAX__: u32 = 4294967295;
pub const __UINT_FAST32_FMTo__: &'static [u8; 2usize] = b"o\0";
pub const __UINT_FAST32_FMTu__: &'static [u8; 2usize] = b"u\0";
pub const __UINT_FAST32_FMTx__: &'static [u8; 2usize] = b"x\0";
pub const __UINT_FAST32_FMTX__: &'static [u8; 2usize] = b"X\0";
pub const __INT_FAST64_MAX__: u64 = 9223372036854775807;
pub const __INT_FAST64_FMTd__: &'static [u8; 3usize] = b"ld\0";
pub const __INT_FAST64_FMTi__: &'static [u8; 3usize] = b"li\0";
pub const __UINT_FAST64_MAX__: i32 = -1;
pub const __UINT_FAST64_FMTo__: &'static [u8; 3usize] = b"lo\0";
pub const __UINT_FAST64_FMTu__: &'static [u8; 3usize] = b"lu\0";
pub const __UINT_FAST64_FMTx__: &'static [u8; 3usize] = b"lx\0";
pub const __UINT_FAST64_FMTX__: &'static [u8; 3usize] = b"lX\0";
pub const __FINITE_MATH_ONLY__: u32 = 0;
pub const __GNUC_GNU_INLINE__: u32 = 1;
pub const __GCC_ATOMIC_TEST_AND_SET_TRUEVAL: u32 = 1;
pub const __CLANG_ATOMIC_BOOL_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_CHAR_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_CHAR16_T_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_CHAR32_T_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_WCHAR_T_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_SHORT_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_INT_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_LONG_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_LLONG_LOCK_FREE: u32 = 2;
pub const __CLANG_ATOMIC_POINTER_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_BOOL_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_CHAR_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_CHAR16_T_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_CHAR32_T_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_WCHAR_T_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_SHORT_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_INT_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_LONG_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_LLONG_LOCK_FREE: u32 = 2;
pub const __GCC_ATOMIC_POINTER_LOCK_FREE: u32 = 2;
pub const __PIC__: u32 = 2;
pub const __pic__: u32 = 2;
pub const __FLT_EVAL_METHOD__: u32 = 0;
pub const __FLT_RADIX__: u32 = 2;
pub const __DECIMAL_DIG__: u32 = 21;
pub const __SSP_STRONG__: u32 = 2;
pub const __code_model_small_: u32 = 1;
pub const __amd64__: u32 = 1;
pub const __amd64: u32 = 1;
pub const __x86_64: u32 = 1;
pub const __x86_64__: u32 = 1;
pub const __bdver2: u32 = 1;
pub const __bdver2__: u32 = 1;
pub const __tune_bdver2__: u32 = 1;
pub const __NO_MATH_INLINES: u32 = 1;
pub const __AES__: u32 = 1;
pub const __PCLMUL__: u32 = 1;
pub const __LZCNT__: u32 = 1;
pub const __BMI__: u32 = 1;
pub const __POPCNT__: u32 = 1;
pub const __PRFCHW__: u32 = 1;
pub const __TBM__: u32 = 1;
pub const __LWP__: u32 = 1;
pub const __XOP__: u32 = 1;
pub const __FMA4__: u32 = 1;
pub const __SSE4A__: u32 = 1;
pub const __FMA__: u32 = 1;
pub const __F16C__: u32 = 1;
pub const __FXSR__: u32 = 1;
pub const __XSAVE__: u32 = 1;
pub const __AVX__: u32 = 1;
pub const __SSE4_2__: u32 = 1;
pub const __SSE4_1__: u32 = 1;
pub const __SSSE3__: u32 = 1;
pub const __SSE3__: u32 = 1;
pub const __SSE2__: u32 = 1;
pub const __SSE2_MATH__: u32 = 1;
pub const __SSE__: u32 = 1;
pub const __SSE_MATH__: u32 = 1;
pub const __MMX__: u32 = 1;
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_1: u32 = 1;
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_2: u32 = 1;
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_4: u32 = 1;
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_8: u32 = 1;
pub const __GCC_HAVE_SYNC_COMPARE_AND_SWAP_16: u32 = 1;
pub const __SIZEOF_FLOAT128__: u32 = 16;
pub const __unix: u32 = 1;
pub const __unix__: u32 = 1;
pub const __linux: u32 = 1;
pub const __linux__: u32 = 1;
pub const __gnu_linux__: u32 = 1;
pub const __ELF__: u32 = 1;
pub const _GNU_SOURCE: u32 = 1;
pub const __FLOAT128__: u32 = 1;
pub const __STDC__: u32 = 1;
pub const __STDC_HOSTED__: u32 = 1;
pub const __cplusplus: u32 = 201703;
pub const __STDCPP_DEFAULT_NEW_ALIGNMENT__: u32 = 16;
pub const __STDC_UTF_16__: u32 = 1;
pub const __STDC_UTF_32__: u32 = 1;
pub const HAVE_CONFIG_H: u32 = 1;
pub const HAVE_POLL: u32 = 1;
pub const HAVE_LIBPTHREAD: u32 = 1;
pub const ENABLE_THREADS: u32 = 1;
pub const __PLATFORM_LINUX__: u32 = 1;
pub const __LINUX_ALSA__: u32 = 1;
pub const __UNIX_JACK__: u32 = 1;
pub const USE_ALSA: u32 = 1;
pub const USE_DLTRICK_ALSA: u32 = 1;
pub const USE_OSS: u32 = 1;
pub const _GLIBCXX_STDLIB_H: u32 = 1;
pub const _GLIBCXX_CXX_CONFIG_H: u32 = 1;
pub const _GLIBCXX_RELEASE: u32 = 8;
pub const __GLIBCXX__: u32 = 20181127;
pub const _GLIBCXX_HAVE_ATTRIBUTE_VISIBILITY: u32 = 1;
pub const _GLIBCXX_USE_DEPRECATED: u32 = 1;
pub const _GLIBCXX_EXTERN_TEMPLATE: u32 = 1;
pub const _GLIBCXX_USE_DUAL_ABI: u32 = 1;
pub const _GLIBCXX_USE_CXX11_ABI: u32 = 1;
pub const _GLIBCXX_INLINE_VERSION: u32 = 0;
pub const _GLIBCXX_USE_ALLOCATOR_NEW: u32 = 1;
pub const _GLIBCXX_OS_DEFINES: u32 = 1;
pub const __NO_CTYPE: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _ISOC95_SOURCE: u32 = 1;
pub const _ISOC99_SOURCE: u32 = 1;
pub const _ISOC11_SOURCE: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const _XOPEN_SOURCE: u32 = 700;
pub const _XOPEN_SOURCE_EXTENDED: u32 = 1;
pub const _LARGEFILE64_SOURCE: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_ISOCXX11: u32 = 1;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const __USE_XOPEN: u32 = 1;
pub const __USE_XOPEN_EXTENDED: u32 = 1;
pub const __USE_UNIX98: u32 = 1;
pub const _LARGEFILE_SOURCE: u32 = 1;
pub const __USE_XOPEN2K8XSI: u32 = 1;
pub const __USE_XOPEN2KXSI: u32 = 1;
pub const __USE_LARGEFILE: u32 = 1;
pub const __USE_LARGEFILE64: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_GNU: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 28;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 0;
pub const __USE_EXTERN_INLINES: u32 = 1;
pub const _GLIBCXX_CPU_DEFINES: u32 = 1;
pub const _GLIBCXX_USE_WEAK_REF: u32 = 1;
pub const _GLIBCXX_USE_STD_SPEC_FUNCS: u32 = 1;
pub const _GLIBCXX_FAST_MATH: u32 = 0;
pub const _GLIBCXX_USE_FLOAT128: u32 = 1;
pub const _GLIBCXX_HAVE_ACOSF: u32 = 1;
pub const _GLIBCXX_HAVE_ACOSL: u32 = 1;
pub const _GLIBCXX_HAVE_ALIGNED_ALLOC: u32 = 1;
pub const _GLIBCXX_HAVE_ASINF: u32 = 1;
pub const _GLIBCXX_HAVE_ASINL: u32 = 1;
pub const _GLIBCXX_HAVE_AS_SYMVER_DIRECTIVE: u32 = 1;
pub const _GLIBCXX_HAVE_ATAN2F: u32 = 1;
pub const _GLIBCXX_HAVE_ATAN2L: u32 = 1;
pub const _GLIBCXX_HAVE_ATANF: u32 = 1;
pub const _GLIBCXX_HAVE_ATANL: u32 = 1;
pub const _GLIBCXX_HAVE_AT_QUICK_EXIT: u32 = 1;
pub const _GLIBCXX_HAVE_CEILF: u32 = 1;
pub const _GLIBCXX_HAVE_CEILL: u32 = 1;
pub const _GLIBCXX_HAVE_COMPLEX_H: u32 = 1;
pub const _GLIBCXX_HAVE_COSF: u32 = 1;
pub const _GLIBCXX_HAVE_COSHF: u32 = 1;
pub const _GLIBCXX_HAVE_COSHL: u32 = 1;
pub const _GLIBCXX_HAVE_COSL: u32 = 1;
pub const _GLIBCXX_HAVE_DIRENT_H: u32 = 1;
pub const _GLIBCXX_HAVE_DLFCN_H: u32 = 1;
pub const _GLIBCXX_HAVE_EBADMSG: u32 = 1;
pub const _GLIBCXX_HAVE_ECANCELED: u32 = 1;
pub const _GLIBCXX_HAVE_ECHILD: u32 = 1;
pub const _GLIBCXX_HAVE_EIDRM: u32 = 1;
pub const _GLIBCXX_HAVE_ENDIAN_H: u32 = 1;
pub const _GLIBCXX_HAVE_ENODATA: u32 = 1;
pub const _GLIBCXX_HAVE_ENOLINK: u32 = 1;
pub const _GLIBCXX_HAVE_ENOSPC: u32 = 1;
pub const _GLIBCXX_HAVE_ENOSR: u32 = 1;
pub const _GLIBCXX_HAVE_ENOSTR: u32 = 1;
pub const _GLIBCXX_HAVE_ENOTRECOVERABLE: u32 = 1;
pub const _GLIBCXX_HAVE_ENOTSUP: u32 = 1;
pub const _GLIBCXX_HAVE_EOVERFLOW: u32 = 1;
pub const _GLIBCXX_HAVE_EOWNERDEAD: u32 = 1;
pub const _GLIBCXX_HAVE_EPERM: u32 = 1;
pub const _GLIBCXX_HAVE_EPROTO: u32 = 1;
pub const _GLIBCXX_HAVE_ETIME: u32 = 1;
pub const _GLIBCXX_HAVE_ETIMEDOUT: u32 = 1;
pub const _GLIBCXX_HAVE_ETXTBSY: u32 = 1;
pub const _GLIBCXX_HAVE_EWOULDBLOCK: u32 = 1;
pub const _GLIBCXX_HAVE_EXCEPTION_PTR_SINCE_GCC46: u32 = 1;
pub const _GLIBCXX_HAVE_EXECINFO_H: u32 = 1;
pub const _GLIBCXX_HAVE_EXPF: u32 = 1;
pub const _GLIBCXX_HAVE_EXPL: u32 = 1;
pub const _GLIBCXX_HAVE_FABSF: u32 = 1;
pub const _GLIBCXX_HAVE_FABSL: u32 = 1;
pub const _GLIBCXX_HAVE_FCNTL_H: u32 = 1;
pub const _GLIBCXX_HAVE_FENV_H: u32 = 1;
pub const _GLIBCXX_HAVE_FINITE: u32 = 1;
pub const _GLIBCXX_HAVE_FINITEF: u32 = 1;
pub const _GLIBCXX_HAVE_FINITEL: u32 = 1;
pub const _GLIBCXX_HAVE_FLOAT_H: u32 = 1;
pub const _GLIBCXX_HAVE_FLOORF: u32 = 1;
pub const _GLIBCXX_HAVE_FLOORL: u32 = 1;
pub const _GLIBCXX_HAVE_FMODF: u32 = 1;
pub const _GLIBCXX_HAVE_FMODL: u32 = 1;
pub const _GLIBCXX_HAVE_FREXPF: u32 = 1;
pub const _GLIBCXX_HAVE_FREXPL: u32 = 1;
pub const _GLIBCXX_HAVE_GETIPINFO: u32 = 1;
pub const _GLIBCXX_HAVE_GETS: u32 = 1;
pub const _GLIBCXX_HAVE_HYPOT: u32 = 1;
pub const _GLIBCXX_HAVE_HYPOTF: u32 = 1;
pub const _GLIBCXX_HAVE_HYPOTL: u32 = 1;
pub const _GLIBCXX_HAVE_ICONV: u32 = 1;
pub const _GLIBCXX_HAVE_INT64_T: u32 = 1;
pub const _GLIBCXX_HAVE_INT64_T_LONG: u32 = 1;
pub const _GLIBCXX_HAVE_INTTYPES_H: u32 = 1;
pub const _GLIBCXX_HAVE_ISINFF: u32 = 1;
pub const _GLIBCXX_HAVE_ISINFL: u32 = 1;
pub const _GLIBCXX_HAVE_ISNANF: u32 = 1;
pub const _GLIBCXX_HAVE_ISNANL: u32 = 1;
pub const _GLIBCXX_HAVE_ISWBLANK: u32 = 1;
pub const _GLIBCXX_HAVE_LC_MESSAGES: u32 = 1;
pub const _GLIBCXX_HAVE_LDEXPF: u32 = 1;
pub const _GLIBCXX_HAVE_LDEXPL: u32 = 1;
pub const _GLIBCXX_HAVE_LIBINTL_H: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_AS: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_DATA: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_FSIZE: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_RSS: u32 = 1;
pub const _GLIBCXX_HAVE_LIMIT_VMEM: u32 = 0;
pub const _GLIBCXX_HAVE_LINUX_FUTEX: u32 = 1;
pub const _GLIBCXX_HAVE_LINUX_RANDOM_H: u32 = 1;
pub const _GLIBCXX_HAVE_LINUX_TYPES_H: u32 = 1;
pub const _GLIBCXX_HAVE_LOCALE_H: u32 = 1;
pub const _GLIBCXX_HAVE_LOG10F: u32 = 1;
pub const _GLIBCXX_HAVE_LOG10L: u32 = 1;
pub const _GLIBCXX_HAVE_LOGF: u32 = 1;
pub const _GLIBCXX_HAVE_LOGL: u32 = 1;
pub const _GLIBCXX_HAVE_MBSTATE_T: u32 = 1;
pub const _GLIBCXX_HAVE_MEMALIGN: u32 = 1;
pub const _GLIBCXX_HAVE_MEMORY_H: u32 = 1;
pub const _GLIBCXX_HAVE_MODF: u32 = 1;
pub const _GLIBCXX_HAVE_MODFF: u32 = 1;
pub const _GLIBCXX_HAVE_MODFL: u32 = 1;
pub const _GLIBCXX_HAVE_POLL: u32 = 1;
pub const _GLIBCXX_HAVE_POSIX_MEMALIGN: u32 = 1;
pub const _GLIBCXX_HAVE_POWF: u32 = 1;
pub const _GLIBCXX_HAVE_POWL: u32 = 1;
pub const _GLIBCXX_HAVE_QUICK_EXIT: u32 = 1;
pub const _GLIBCXX_HAVE_SETENV: u32 = 1;
pub const _GLIBCXX_HAVE_SINCOS: u32 = 1;
pub const _GLIBCXX_HAVE_SINCOSF: u32 = 1;
pub const _GLIBCXX_HAVE_SINCOSL: u32 = 1;
pub const _GLIBCXX_HAVE_SINF: u32 = 1;
pub const _GLIBCXX_HAVE_SINHF: u32 = 1;
pub const _GLIBCXX_HAVE_SINHL: u32 = 1;
pub const _GLIBCXX_HAVE_SINL: u32 = 1;
pub const _GLIBCXX_HAVE_SQRTF: u32 = 1;
pub const _GLIBCXX_HAVE_SQRTL: u32 = 1;
pub const _GLIBCXX_HAVE_STDALIGN_H: u32 = 1;
pub const _GLIBCXX_HAVE_STDBOOL_H: u32 = 1;
pub const _GLIBCXX_HAVE_STDINT_H: u32 = 1;
pub const _GLIBCXX_HAVE_STDLIB_H: u32 = 1;
pub const _GLIBCXX_HAVE_STRERROR_L: u32 = 1;
pub const _GLIBCXX_HAVE_STRERROR_R: u32 = 1;
pub const _GLIBCXX_HAVE_STRINGS_H: u32 = 1;
pub const _GLIBCXX_HAVE_STRING_H: u32 = 1;
pub const _GLIBCXX_HAVE_STRTOF: u32 = 1;
pub const _GLIBCXX_HAVE_STRTOLD: u32 = 1;
pub const _GLIBCXX_HAVE_STRUCT_DIRENT_D_TYPE: u32 = 1;
pub const _GLIBCXX_HAVE_STRXFRM_L: u32 = 1;
pub const _GLIBCXX_HAVE_SYMVER_SYMBOL_RENAMING_RUNTIME_SUPPORT: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_IOCTL_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_IPC_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_PARAM_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_RESOURCE_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_SEM_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_STATVFS_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_STAT_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_SYSINFO_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_TIME_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_TYPES_H: u32 = 1;
pub const _GLIBCXX_HAVE_SYS_UIO_H: u32 = 1;
pub const _GLIBCXX_HAVE_S_ISREG: u32 = 1;
pub const _GLIBCXX_HAVE_TANF: u32 = 1;
pub const _GLIBCXX_HAVE_TANHF: u32 = 1;
pub const _GLIBCXX_HAVE_TANHL: u32 = 1;
pub const _GLIBCXX_HAVE_TANL: u32 = 1;
pub const _GLIBCXX_HAVE_TGMATH_H: u32 = 1;
pub const _GLIBCXX_HAVE_TLS: u32 = 1;
pub const _GLIBCXX_HAVE_UCHAR_H: u32 = 1;
pub const _GLIBCXX_HAVE_UNISTD_H: u32 = 1;
pub const _GLIBCXX_HAVE_UTIME_H: u32 = 1;
pub const _GLIBCXX_HAVE_VFWSCANF: u32 = 1;
pub const _GLIBCXX_HAVE_VSWSCANF: u32 = 1;
pub const _GLIBCXX_HAVE_VWSCANF: u32 = 1;
pub const _GLIBCXX_HAVE_WCHAR_H: u32 = 1;
pub const _GLIBCXX_HAVE_WCSTOF: u32 = 1;
pub const _GLIBCXX_HAVE_WCTYPE_H: u32 = 1;
pub const _GLIBCXX_HAVE_WRITEV: u32 = 1;
pub const _GLIBCXX_HAVE___CXA_THREAD_ATEXIT_IMPL: u32 = 1;
pub const LT_OBJDIR: &'static [u8; 7usize] = b".libs/\0";
pub const _GLIBCXX_PACKAGE_BUGREPORT: &'static [u8; 1usize] = b"\0";
pub const _GLIBCXX_PACKAGE_NAME: &'static [u8; 15usize] = b"package-unused\0";
pub const _GLIBCXX_PACKAGE_STRING: &'static [u8; 30usize] = b"package-unused version-unused\0";
pub const _GLIBCXX_PACKAGE_TARNAME: &'static [u8; 10usize] = b"libstdc++\0";
pub const _GLIBCXX_PACKAGE_URL: &'static [u8; 1usize] = b"\0";
pub const _GLIBCXX_PACKAGE__GLIBCXX_VERSION: &'static [u8; 15usize] = b"version-unused\0";
pub const STDC_HEADERS: u32 = 1;
pub const _GLIBCXX11_USE_C99_COMPLEX: u32 = 1;
pub const _GLIBCXX11_USE_C99_MATH: u32 = 1;
pub const _GLIBCXX11_USE_C99_STDIO: u32 = 1;
pub const _GLIBCXX11_USE_C99_STDLIB: u32 = 1;
pub const _GLIBCXX11_USE_C99_WCHAR: u32 = 1;
pub const _GLIBCXX98_USE_C99_COMPLEX: u32 = 1;
pub const _GLIBCXX98_USE_C99_MATH: u32 = 1;
pub const _GLIBCXX98_USE_C99_STDIO: u32 = 1;
pub const _GLIBCXX98_USE_C99_STDLIB: u32 = 1;
pub const _GLIBCXX98_USE_C99_WCHAR: u32 = 1;
pub const _GLIBCXX_ATOMIC_BUILTINS: u32 = 1;
pub const _GLIBCXX_FULLY_DYNAMIC_STRING: u32 = 0;
pub const _GLIBCXX_HAS_GTHREADS: u32 = 1;
pub const _GLIBCXX_HOSTED: u32 = 1;
pub const _GLIBCXX_RES_LIMITS: u32 = 1;
pub const _GLIBCXX_STDIO_EOF: i32 = -1;
pub const _GLIBCXX_STDIO_SEEK_CUR: u32 = 1;
pub const _GLIBCXX_STDIO_SEEK_END: u32 = 2;
pub const _GLIBCXX_SYMVER: u32 = 1;
pub const _GLIBCXX_SYMVER_GNU: u32 = 1;
pub const _GLIBCXX_USE_C11_UCHAR_CXX11: u32 = 1;
pub const _GLIBCXX_USE_C99: u32 = 1;
pub const _GLIBCXX_USE_C99_COMPLEX_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_CTYPE_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_FENV_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_INTTYPES_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_INTTYPES_WCHAR_T_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_MATH_TR1: u32 = 1;
pub const _GLIBCXX_USE_C99_STDINT_TR1: u32 = 1;
pub const _GLIBCXX_USE_CLOCK_MONOTONIC: u32 = 1;
pub const _GLIBCXX_USE_CLOCK_REALTIME: u32 = 1;
pub const _GLIBCXX_USE_DECIMAL_FLOAT: u32 = 1;
pub const _GLIBCXX_USE_FCHMOD: u32 = 1;
pub const _GLIBCXX_USE_FCHMODAT: u32 = 1;
pub const _GLIBCXX_USE_GETTIMEOFDAY: u32 = 1;
pub const _GLIBCXX_USE_GET_NPROCS: u32 = 1;
pub const _GLIBCXX_USE_INT128: u32 = 1;
pub const _GLIBCXX_USE_LFS: u32 = 1;
pub const _GLIBCXX_USE_LONG_LONG: u32 = 1;
pub const _GLIBCXX_USE_NANOSLEEP: u32 = 1;
pub const _GLIBCXX_USE_NLS: u32 = 1;
pub const _GLIBCXX_USE_PTHREAD_RWLOCK_T: u32 = 1;
pub const _GLIBCXX_USE_RANDOM_TR1: u32 = 1;
pub const _GLIBCXX_USE_REALPATH: u32 = 1;
pub const _GLIBCXX_USE_SCHED_YIELD: u32 = 1;
pub const _GLIBCXX_USE_SC_NPROCESSORS_ONLN: u32 = 1;
pub const _GLIBCXX_USE_SENDFILE: u32 = 1;
pub const _GLIBCXX_USE_ST_MTIM: u32 = 1;
pub const _GLIBCXX_USE_TMPNAM: u32 = 1;
pub const _GLIBCXX_USE_UTIMENSAT: u32 = 1;
pub const _GLIBCXX_USE_WCHAR_T: u32 = 1;
pub const _GLIBCXX_VERBOSE: u32 = 1;
pub const _GLIBCXX_X86_RDRAND: u32 = 1;
pub const _GTHREAD_USE_MUTEX_TIMEDLOCK: u32 = 1;
pub const _GLIBCXX_CSTDLIB: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 1;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 1;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const _SYS_TYPES_H: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const __PTHREAD_MUTEX_LOCK_ELISION: u32 = 1;
pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND: u32 = 0;
pub const __PTHREAD_MUTEX_USE_UNION: u32 = 0;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const _ALLOCA_H: u32 = 1;
pub const _MEMORY_H: u32 = 1;
pub const _STRING_H: u32 = 1;
pub const _STRINGS_H: u32 = 1;
pub const _ASSERT_H: u32 = 1;
pub const sz_VOID: u32 = 0;
pub const sz_WORD: u32 = 4;
pub const kindof_VOID: u32 = 0;
pub const kindof_INT: u32 = 1;
pub const kindof_FLOAT: u32 = 2;
pub const kindof_COMPLEX: u32 = 3;
pub const kindof_VEC3: u32 = 4;
pub const kindof_VEC4: u32 = 5;
pub const SILENCE: f64 = 0.0;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const ONE_PI: f64 = 3.141592653589793;
pub const TWO_PI: f64 = 6.283185307179586;
pub const SQRT2: f64 = 1.4142135623730951;
pub const __CHUCK_STAT_TRACK__: u32 = 1;
pub const _STDIO_H: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const __cookie_io_functions_t_defined: u32 = 1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_DATA: u32 = 3;
pub const SEEK_HOLE: u32 = 4;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const L_cuserid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const RENAME_NOREPLACE: u32 = 1;
pub const RENAME_EXCHANGE: u32 = 2;
pub const RENAME_WHITEOUT: u32 = 4;
pub const _BITS_STDIO_H: u32 = 1;
pub const _GLIBCXX_STRING: u32 = 1;
pub const _STRINGFWD_H: u32 = 1;
pub const _MEMORYFWD_H: u32 = 1;
pub const _CHAR_TRAITS_H: u32 = 1;
pub const _STL_ALGOBASE_H: u32 = 1;
pub const _FUNCTEXCEPT_H: u32 = 1;
pub const _EXCEPTION_DEFINES_H: u32 = 1;
pub const _CPP_TYPE_TRAITS_H: u32 = 1;
pub const _EXT_TYPE_TRAITS: u32 = 1;
pub const _EXT_NUMERIC_TRAITS: u32 = 1;
pub const _STL_PAIR_H: u32 = 1;
pub const _MOVE_H: u32 = 1;
pub const _CONCEPT_CHECK_H: u32 = 1;
pub const _GLIBCXX_TYPE_TRAITS: u32 = 1;
pub const __cpp_lib_integral_constant_callable: u32 = 201304;
pub const __cpp_lib_bool_constant: u32 = 201505;
pub const __cpp_lib_logical_traits: u32 = 201510;
pub const __cpp_lib_is_null_pointer: u32 = 201309;
pub const __cpp_lib_is_final: u32 = 201402;
pub const __cpp_lib_transformation_trait_aliases: u32 = 201304;
pub const __cpp_lib_result_of_sfinae: u32 = 201210;
pub const __cpp_lib_void_t: u32 = 201411;
pub const __cpp_lib_is_swappable: u32 = 201603;
pub const __cpp_lib_is_invocable: u32 = 201703;
pub const __cpp_lib_type_trait_variable_templates: u32 = 201510;
pub const _GLIBCXX_HAVE_BUILTIN_HAS_UNIQ_OBJ_REP: u32 = 1;
pub const __cpp_lib_has_unique_object_representations: u32 = 201606;
pub const _GLIBCXX_HAVE_BUILTIN_IS_AGGREGATE: u32 = 1;
pub const __cpp_lib_is_aggregate: u32 = 201703;
pub const __cpp_lib_addressof_constexpr: u32 = 201603;
pub const _STL_ITERATOR_BASE_TYPES_H: u32 = 1;
pub const _STL_ITERATOR_BASE_FUNCS_H: u32 = 1;
pub const _GLIBCXX_DEBUG_ASSERTIONS_H: u32 = 1;
pub const _STL_ITERATOR_H: u32 = 1;
pub const _PTR_TRAITS_H: u32 = 1;
pub const __cpp_lib_array_constexpr: u32 = 201603;
pub const __cpp_lib_make_reverse_iterator: u32 = 201402;
pub const _GLIBCXX_DEBUG_MACRO_SWITCH_H: u32 = 1;
pub const _GLIBCXX_PREDEFINED_OPS_H: u32 = 1;
pub const __cpp_lib_robust_nonmodifying_seq_ops: u32 = 201304;
pub const _GLIBCXX_POSTYPES_H: u32 = 1;
pub const _WCHAR_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const __WCHAR_MAX: u32 = 2147483647;
pub const __WCHAR_MIN: i32 = -2147483648;
pub const __wint_t_defined: u32 = 1;
pub const _WINT_T: u32 = 1;
pub const __mbstate_t_defined: u32 = 1;
pub const WCHAR_MIN: i32 = -2147483648;
pub const WCHAR_MAX: u32 = 2147483647;
pub const WEOF: u32 = 4294967295;
pub const _GLIBCXX_CWCHAR: u32 = 1;
pub const __cpp_lib_constexpr_char_traits: u32 = 201611;
pub const _GLIBCXX_CSTDINT: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const INT8_WIDTH: u32 = 8;
pub const UINT8_WIDTH: u32 = 8;
pub const INT16_WIDTH: u32 = 16;
pub const UINT16_WIDTH: u32 = 16;
pub const INT32_WIDTH: u32 = 32;
pub const UINT32_WIDTH: u32 = 32;
pub const INT64_WIDTH: u32 = 64;
pub const UINT64_WIDTH: u32 = 64;
pub const INT_LEAST8_WIDTH: u32 = 8;
pub const UINT_LEAST8_WIDTH: u32 = 8;
pub const INT_LEAST16_WIDTH: u32 = 16;
pub const UINT_LEAST16_WIDTH: u32 = 16;
pub const INT_LEAST32_WIDTH: u32 = 32;
pub const UINT_LEAST32_WIDTH: u32 = 32;
pub const INT_LEAST64_WIDTH: u32 = 64;
pub const UINT_LEAST64_WIDTH: u32 = 64;
pub const INT_FAST8_WIDTH: u32 = 8;
pub const UINT_FAST8_WIDTH: u32 = 8;
pub const INT_FAST16_WIDTH: u32 = 64;
pub const UINT_FAST16_WIDTH: u32 = 64;
pub const INT_FAST32_WIDTH: u32 = 64;
pub const UINT_FAST32_WIDTH: u32 = 64;
pub const INT_FAST64_WIDTH: u32 = 64;
pub const UINT_FAST64_WIDTH: u32 = 64;
pub const INTPTR_WIDTH: u32 = 64;
pub const UINTPTR_WIDTH: u32 = 64;
pub const INTMAX_WIDTH: u32 = 64;
pub const UINTMAX_WIDTH: u32 = 64;
pub const PTRDIFF_WIDTH: u32 = 64;
pub const SIG_ATOMIC_WIDTH: u32 = 32;
pub const SIZE_WIDTH: u32 = 64;
pub const WCHAR_WIDTH: u32 = 32;
pub const WINT_WIDTH: u32 = 32;
pub const _ALLOCATOR_H: u32 = 1;
pub const _GLIBCXX_CXX_ALLOCATOR_H: u32 = 1;
pub const _NEW_ALLOCATOR_H: u32 = 1;
pub const __EXCEPTION_H: u32 = 1;
pub const __cpp_lib_uncaught_exceptions: u32 = 201411;
pub const _CXXABI_INIT_EXCEPTION_H: u32 = 1;
pub const _GLIBCXX_HAVE_CDTOR_CALLABI: u32 = 0;
pub const _HASH_BYTES_H: u32 = 1;
pub const __GXX_MERGED_TYPEINFO_NAMES: u32 = 0;
pub const __GXX_TYPEINFO_EQUALITY_INLINE: u32 = 1;
pub const _GLIBCXX_NESTED_EXCEPTION_H: u32 = 1;
pub const _GLIBCXX_HAVE_BUILTIN_LAUNDER: u32 = 1;
pub const __cpp_lib_launder: u32 = 201606;
pub const __cpp_lib_incomplete_container_elements: u32 = 201505;
pub const __cpp_lib_allocator_is_always_equal: u32 = 201411;
pub const _LOCALE_FWD_H: u32 = 1;
pub const _GLIBCXX_CXX_LOCALE_H: u32 = 1;
pub const _LOCALE_H: u32 = 1;
pub const _BITS_LOCALE_H: u32 = 1;
pub const __LC_CTYPE: u32 = 0;
pub const __LC_NUMERIC: u32 = 1;
pub const __LC_TIME: u32 = 2;
pub const __LC_COLLATE: u32 = 3;
pub const __LC_MONETARY: u32 = 4;
pub const __LC_MESSAGES: u32 = 5;
pub const __LC_ALL: u32 = 6;
pub const __LC_PAPER: u32 = 7;
pub const __LC_NAME: u32 = 8;
pub const __LC_ADDRESS: u32 = 9;
pub const __LC_TELEPHONE: u32 = 10;
pub const __LC_MEASUREMENT: u32 = 11;
pub const __LC_IDENTIFICATION: u32 = 12;
pub const LC_CTYPE: u32 = 0;
pub const LC_NUMERIC: u32 = 1;
pub const LC_TIME: u32 = 2;
pub const LC_COLLATE: u32 = 3;
pub const LC_MONETARY: u32 = 4;
pub const LC_MESSAGES: u32 = 5;
pub const LC_ALL: u32 = 6;
pub const LC_PAPER: u32 = 7;
pub const LC_NAME: u32 = 8;
pub const LC_ADDRESS: u32 = 9;
pub const LC_TELEPHONE: u32 = 10;
pub const LC_MEASUREMENT: u32 = 11;
pub const LC_IDENTIFICATION: u32 = 12;
pub const LC_CTYPE_MASK: u32 = 1;
pub const LC_NUMERIC_MASK: u32 = 2;
pub const LC_TIME_MASK: u32 = 4;
pub const LC_COLLATE_MASK: u32 = 8;
pub const LC_MONETARY_MASK: u32 = 16;
pub const LC_MESSAGES_MASK: u32 = 32;
pub const LC_PAPER_MASK: u32 = 128;
pub const LC_NAME_MASK: u32 = 256;
pub const LC_ADDRESS_MASK: u32 = 512;
pub const LC_TELEPHONE_MASK: u32 = 1024;
pub const LC_MEASUREMENT_MASK: u32 = 2048;
pub const LC_IDENTIFICATION_MASK: u32 = 4096;
pub const LC_ALL_MASK: u32 = 8127;
pub const _GLIBCXX_CLOCALE: u32 = 1;
pub const _GLIBCXX_C_LOCALE_GNU: u32 = 1;
pub const _GLIBCXX_NUM_CATEGORIES: u32 = 6;
pub const _GLIBCXX_IOSFWD: u32 = 1;
pub const _CTYPE_H: u32 = 1;
pub const _GLIBCXX_CCTYPE: u32 = 1;
pub const _OSTREAM_INSERT_H: u32 = 1;
pub const _CXXABI_FORCED_H: u32 = 1;
pub const _STL_FUNCTION_H: u32 = 1;
pub const __cpp_lib_transparent_operators: u32 = 201510;
pub const _BACKWARD_BINDERS_H: u32 = 1;
pub const _GLIBCXX_RANGE_ACCESS_H: u32 = 1;
pub const __cpp_lib_nonmember_container_access: u32 = 201411;
pub const _BASIC_STRING_H: u32 = 1;
pub const _GLIBCXX_ATOMICITY_H: u32 = 1;
pub const _GLIBCXX_GTHREAD_USE_WEAK: u32 = 1;
pub const __GTHREADS: u32 = 1;
pub const __GTHREADS_CXX0X: u32 = 1;
pub const _PTHREAD_H: u32 = 1;
pub const _SCHED_H: u32 = 1;
pub const _BITS_SCHED_H: u32 = 1;
pub const SCHED_OTHER: u32 = 0;
pub const SCHED_FIFO: u32 = 1;
pub const SCHED_RR: u32 = 2;
pub const SCHED_BATCH: u32 = 3;
pub const SCHED_ISO: u32 = 4;
pub const SCHED_IDLE: u32 = 5;
pub const SCHED_DEADLINE: u32 = 6;
pub const SCHED_RESET_ON_FORK: u32 = 1073741824;
pub const CSIGNAL: u32 = 255;
pub const CLONE_VM: u32 = 256;
pub const CLONE_FS: u32 = 512;
pub const CLONE_FILES: u32 = 1024;
pub const CLONE_SIGHAND: u32 = 2048;
pub const CLONE_PTRACE: u32 = 8192;
pub const CLONE_VFORK: u32 = 16384;
pub const CLONE_PARENT: u32 = 32768;
pub const CLONE_THREAD: u32 = 65536;
pub const CLONE_NEWNS: u32 = 131072;
pub const CLONE_SYSVSEM: u32 = 262144;
pub const CLONE_SETTLS: u32 = 524288;
pub const CLONE_PARENT_SETTID: u32 = 1048576;
pub const CLONE_CHILD_CLEARTID: u32 = 2097152;
pub const CLONE_DETACHED: u32 = 4194304;
pub const CLONE_UNTRACED: u32 = 8388608;
pub const CLONE_CHILD_SETTID: u32 = 16777216;
pub const CLONE_NEWCGROUP: u32 = 33554432;
pub const CLONE_NEWUTS: u32 = 67108864;
pub const CLONE_NEWIPC: u32 = 134217728;
pub const CLONE_NEWUSER: u32 = 268435456;
pub const CLONE_NEWPID: u32 = 536870912;
pub const CLONE_NEWNET: u32 = 1073741824;
pub const CLONE_IO: u32 = 2147483648;
pub const _BITS_TYPES_STRUCT_SCHED_PARAM: u32 = 1;
pub const _BITS_CPU_SET_H: u32 = 1;
pub const __CPU_SETSIZE: u32 = 1024;
pub const CPU_SETSIZE: u32 = 1024;
pub const _TIME_H: u32 = 1;
pub const _BITS_TIME_H: u32 = 1;
pub const CLOCK_REALTIME: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
pub const CLOCK_REALTIME_COARSE: u32 = 5;
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
pub const CLOCK_BOOTTIME: u32 = 7;
pub const CLOCK_REALTIME_ALARM: u32 = 8;
pub const CLOCK_BOOTTIME_ALARM: u32 = 9;
pub const CLOCK_TAI: u32 = 11;
pub const TIMER_ABSTIME: u32 = 1;
pub const _BITS_TIMEX_H: u32 = 1;
pub const ADJ_OFFSET: u32 = 1;
pub const ADJ_FREQUENCY: u32 = 2;
pub const ADJ_MAXERROR: u32 = 4;
pub const ADJ_ESTERROR: u32 = 8;
pub const ADJ_STATUS: u32 = 16;
pub const ADJ_TIMECONST: u32 = 32;
pub const ADJ_TAI: u32 = 128;
pub const ADJ_SETOFFSET: u32 = 256;
pub const ADJ_MICRO: u32 = 4096;
pub const ADJ_NANO: u32 = 8192;
pub const ADJ_TICK: u32 = 16384;
pub const ADJ_OFFSET_SINGLESHOT: u32 = 32769;
pub const ADJ_OFFSET_SS_READ: u32 = 40961;
pub const MOD_OFFSET: u32 = 1;
pub const MOD_FREQUENCY: u32 = 2;
pub const MOD_MAXERROR: u32 = 4;
pub const MOD_ESTERROR: u32 = 8;
pub const MOD_STATUS: u32 = 16;
pub const MOD_TIMECONST: u32 = 32;
pub const MOD_CLKB: u32 = 16384;
pub const MOD_CLKA: u32 = 32769;
pub const MOD_TAI: u32 = 128;
pub const MOD_MICRO: u32 = 4096;
pub const MOD_NANO: u32 = 8192;
pub const STA_PLL: u32 = 1;
pub const STA_PPSFREQ: u32 = 2;
pub const STA_PPSTIME: u32 = 4;
pub const STA_FLL: u32 = 8;
pub const STA_INS: u32 = 16;
pub const STA_DEL: u32 = 32;
pub const STA_UNSYNC: u32 = 64;
pub const STA_FREQHOLD: u32 = 128;
pub const STA_PPSSIGNAL: u32 = 256;
pub const STA_PPSJITTER: u32 = 512;
pub const STA_PPSWANDER: u32 = 1024;
pub const STA_PPSERROR: u32 = 2048;
pub const STA_CLOCKERR: u32 = 4096;
pub const STA_NANO: u32 = 8192;
pub const STA_MODE: u32 = 16384;
pub const STA_CLK: u32 = 32768;
pub const STA_RONLY: u32 = 65280;
pub const __struct_tm_defined: u32 = 1;
pub const __itimerspec_defined: u32 = 1;
pub const TIME_UTC: u32 = 1;
pub const _BITS_SETJMP_H: u32 = 1;
pub const PTHREAD_ONCE_INIT: u32 = 0;
pub const PTHREAD_BARRIER_SERIAL_THREAD: i32 = -1;
pub const __GTHREAD_HAS_COND: u32 = 1;
pub const __GTHREAD_ONCE_INIT: u32 = 0;
pub const _GLIBCXX_ATOMIC_WORD_H: u32 = 1;
pub const _EXT_ALLOC_TRAITS_H: u32 = 1;
pub const _ALLOC_TRAITS_H: u32 = 1;
pub const __cpp_lib_allocator_traits_is_always_equal: u32 = 201411;
pub const _GLIBCXX_STRING_VIEW: u32 = 1;
pub const _GLIBCXX_NUMERIC_LIMITS: u32 = 1;
pub const _FUNCTIONAL_HASH_H: u32 = 1;
pub const __cpp_lib_string_view: u32 = 201603;
pub const _GLIBCXX_STRING_VIEW_TCC: u32 = 1;
pub const _STRING_CONVERSIONS_H: u32 = 1;
pub const _GLIBCXX_CSTDIO: u32 = 1;
pub const _ERRNO_H: u32 = 1;
pub const _BITS_ERRNO_H: u32 = 1;
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const ENOTBLK: u32 = 15;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const ETXTBSY: u32 = 26;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const ERANGE: u32 = 34;
pub const EDEADLK: u32 = 35;
pub const ENAMETOOLONG: u32 = 36;
pub const ENOLCK: u32 = 37;
pub const ENOSYS: u32 = 38;
pub const ENOTEMPTY: u32 = 39;
pub const ELOOP: u32 = 40;
pub const EWOULDBLOCK: u32 = 11;
pub const ENOMSG: u32 = 42;
pub const EIDRM: u32 = 43;
pub const ECHRNG: u32 = 44;
pub const EL2NSYNC: u32 = 45;
pub const EL3HLT: u32 = 46;
pub const EL3RST: u32 = 47;
pub const ELNRNG: u32 = 48;
pub const EUNATCH: u32 = 49;
pub const ENOCSI: u32 = 50;
pub const EL2HLT: u32 = 51;
pub const EBADE: u32 = 52;
pub const EBADR: u32 = 53;
pub const EXFULL: u32 = 54;
pub const ENOANO: u32 = 55;
pub const EBADRQC: u32 = 56;
pub const EBADSLT: u32 = 57;
pub const EDEADLOCK: u32 = 35;
pub const EBFONT: u32 = 59;
pub const ENOSTR: u32 = 60;
pub const ENODATA: u32 = 61;
pub const ETIME: u32 = 62;
pub const ENOSR: u32 = 63;
pub const ENONET: u32 = 64;
pub const ENOPKG: u32 = 65;
pub const EREMOTE: u32 = 66;
pub const ENOLINK: u32 = 67;
pub const EADV: u32 = 68;
pub const ESRMNT: u32 = 69;
pub const ECOMM: u32 = 70;
pub const EPROTO: u32 = 71;
pub const EMULTIHOP: u32 = 72;
pub const EDOTDOT: u32 = 73;
pub const EBADMSG: u32 = 74;
pub const EOVERFLOW: u32 = 75;
pub const ENOTUNIQ: u32 = 76;
pub const EBADFD: u32 = 77;
pub const EREMCHG: u32 = 78;
pub const ELIBACC: u32 = 79;
pub const ELIBBAD: u32 = 80;
pub const ELIBSCN: u32 = 81;
pub const ELIBMAX: u32 = 82;
pub const ELIBEXEC: u32 = 83;
pub const EILSEQ: u32 = 84;
pub const ERESTART: u32 = 85;
pub const ESTRPIPE: u32 = 86;
pub const EUSERS: u32 = 87;
pub const ENOTSOCK: u32 = 88;
pub const EDESTADDRREQ: u32 = 89;
pub const EMSGSIZE: u32 = 90;
pub const EPROTOTYPE: u32 = 91;
pub const ENOPROTOOPT: u32 = 92;
pub const EPROTONOSUPPORT: u32 = 93;
pub const ESOCKTNOSUPPORT: u32 = 94;
pub const EOPNOTSUPP: u32 = 95;
pub const EPFNOSUPPORT: u32 = 96;
pub const EAFNOSUPPORT: u32 = 97;
pub const EADDRINUSE: u32 = 98;
pub const EADDRNOTAVAIL: u32 = 99;
pub const ENETDOWN: u32 = 100;
pub const ENETUNREACH: u32 = 101;
pub const ENETRESET: u32 = 102;
pub const ECONNABORTED: u32 = 103;
pub const ECONNRESET: u32 = 104;
pub const ENOBUFS: u32 = 105;
pub const EISCONN: u32 = 106;
pub const ENOTCONN: u32 = 107;
pub const ESHUTDOWN: u32 = 108;
pub const ETOOMANYREFS: u32 = 109;
pub const ETIMEDOUT: u32 = 110;
pub const ECONNREFUSED: u32 = 111;
pub const EHOSTDOWN: u32 = 112;
pub const EHOSTUNREACH: u32 = 113;
pub const EALREADY: u32 = 114;
pub const EINPROGRESS: u32 = 115;
pub const ESTALE: u32 = 116;
pub const EUCLEAN: u32 = 117;
pub const ENOTNAM: u32 = 118;
pub const ENAVAIL: u32 = 119;
pub const EISNAM: u32 = 120;
pub const EREMOTEIO: u32 = 121;
pub const EDQUOT: u32 = 122;
pub const ENOMEDIUM: u32 = 123;
pub const EMEDIUMTYPE: u32 = 124;
pub const ECANCELED: u32 = 125;
pub const ENOKEY: u32 = 126;
pub const EKEYEXPIRED: u32 = 127;
pub const EKEYREVOKED: u32 = 128;
pub const EKEYREJECTED: u32 = 129;
pub const EOWNERDEAD: u32 = 130;
pub const ENOTRECOVERABLE: u32 = 131;
pub const ERFKILL: u32 = 132;
pub const EHWPOISON: u32 = 133;
pub const ENOTSUP: u32 = 95;
pub const __error_t_defined: u32 = 1;
pub const _GLIBCXX_CERRNO: u32 = 1;
pub const __cpp_lib_string_udls: u32 = 201304;
pub const _BASIC_STRING_TCC: u32 = 1;
pub const _GLIBCXX_VECTOR: u32 = 1;
pub const _STL_CONSTRUCT_H: u32 = 1;
pub const _STL_UNINITIALIZED_H: u32 = 1;
pub const _GLIBCXX_UTILITY: u32 = 1;
pub const _STL_RELOPS_H: u32 = 1;
pub const __cpp_lib_tuple_element_t: u32 = 201402;
pub const __cpp_lib_tuples_by_type: u32 = 201304;
pub const __cpp_lib_exchange_function: u32 = 201304;
pub const _GLIBCXX_USE_MAKE_INTEGER_SEQ: u32 = 1;
pub const __cpp_lib_integer_sequence: u32 = 201304;
pub const __cpp_lib_as_const: u32 = 201510;
pub const _STL_VECTOR_H: u32 = 1;
pub const _STL_BVECTOR_H: u32 = 1;
pub const _VECTOR_TCC: u32 = 1;
pub mod std {

    pub type nullptr_t = *const c_void;
    pub mod __cxx11 {

        pub type string = self::std::__cxx11::basic_string<c_char>;
        pub type wstring = self::std::__cxx11::basic_string<u32>;
        pub type u16string = self::std::__cxx11::basic_string<u16>;
        pub type u32string = self::std::__cxx11::basic_string<u32>;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct basic_stringbuf {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct basic_istringstream {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct basic_ostringstream {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct basic_stringstream {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct numpunct {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct numpunct_byname {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct collate {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct collate_byname {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct time_get {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct time_get_byname {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct money_get {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct money_put {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct messages {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct messages_byname {
            pub _address: u8,
        }
        #[repr(C)]
        pub struct basic_string<_CharT> {
            pub _M_dataplus: self::std::__cxx11::basic_string__Alloc_hider,
            pub _M_string_length: self::std::__cxx11::basic_string_size_type,
            pub __bindgen_anon_1: self::std::__cxx11::basic_string__bindgen_ty_2<_CharT>,
            pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
        }
        pub type basic_string__Char_alloc_type = [u8; 0usize];
        pub type basic_string__Alloc_traits = self::__gnu_cxx::__alloc_traits;
        pub type basic_string_traits_type<_Traits> = _Traits;
        pub type basic_string_value_type = [u8; 0usize];
        pub type basic_string_allocator_type = self::std::__cxx11::basic_string__Char_alloc_type;
        pub type basic_string_size_type = [u8; 0usize];
        pub type basic_string_difference_type = [u8; 0usize];
        pub type basic_string_reference = [u8; 0usize];
        pub type basic_string_const_reference = [u8; 0usize];
        pub type basic_string_pointer = [u8; 0usize];
        pub type basic_string_const_pointer = [u8; 0usize];
        pub type basic_string_iterator =
            self::__gnu_cxx::__normal_iterator<self::std::__cxx11::basic_string_pointer>;
        pub type basic_string_const_iterator =
            self::__gnu_cxx::__normal_iterator<self::std::__cxx11::basic_string_const_pointer>;
        pub type basic_string_const_reverse_iterator =
            self::std::reverse_iterator<self::std::__cxx11::basic_string_const_iterator>;
        pub type basic_string_reverse_iterator =
            self::std::reverse_iterator<self::std::__cxx11::basic_string_iterator>;
        pub type basic_string___const_iterator = self::std::__cxx11::basic_string_const_iterator;
        pub type basic_string___sv_type<_CharT> = self::std::basic_string_view<_CharT>;
        pub type basic_string__If_sv = self::std::enable_if_t;
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct basic_string___sv_wrapper<_CharT> {
            pub _M_sv: self::std::__cxx11::basic_string___sv_type<_CharT>,
            pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
        }
        impl<_CharT> Default for basic_string___sv_wrapper<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        pub struct basic_string__Alloc_hider {
            pub _M_p: self::std::__cxx11::basic_string_pointer,
        }
        impl Default for basic_string__Alloc_hider {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl Debug for basic_string__Alloc_hider {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "basic_string__Alloc_hider {{  }}")
            }
        }
        impl PartialEq for basic_string__Alloc_hider {
            fn eq(&self, other: &basic_string__Alloc_hider) -> bool {
                self._M_p == other._M_p
            }
        }
        pub const basic_string__S_local_capacity: self::std::__cxx11::basic_string__bindgen_ty_1 =
            0;
        pub type basic_string__bindgen_ty_1 = i32;
        #[repr(C)]
        pub struct basic_string__bindgen_ty_2<_CharT> {
            pub _M_local_buf: self::__BindgenUnionField<*mut _CharT>,
            pub _M_allocated_capacity:
                self::__BindgenUnionField<self::std::__cxx11::basic_string_size_type>,
            pub bindgen_union_field: u64,
            pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
        }
        impl<_CharT> Default for basic_string__bindgen_ty_2<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl<_CharT> Debug for basic_string__bindgen_ty_2<_CharT> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "basic_string__bindgen_ty_2 {{ union }}")
            }
        }
        impl<_CharT> Default for basic_string<_CharT> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl<_CharT> Debug for basic_string<_CharT> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(
                    f,
                    "basic_string {{ _M_dataplus: {:?}, __bindgen_anon_1: {:?} }}",
                    self._M_dataplus, self.__bindgen_anon_1
                )
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct uses_allocator {
        pub _address: u8,
    }
    extern "C" {
        pub fn __throw_bad_exception();
    }
    extern "C" {
        pub fn __throw_bad_alloc();
    }
    extern "C" {
        pub fn __throw_bad_cast();
    }
    extern "C" {
        pub fn __throw_bad_typeid();
    }
    extern "C" {
        pub fn __throw_logic_error(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_domain_error(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_invalid_argument(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_length_error(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_out_of_range(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_out_of_range_fmt(arg1: *const c_char, ...);
    }
    extern "C" {
        pub fn __throw_runtime_error(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_range_error(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_overflow_error(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_underflow_error(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_ios_failure(arg1: *const c_char);
    }
    extern "C" {
        pub fn __throw_system_error(arg1: c_int);
    }
    extern "C" {
        pub fn __throw_future_error(arg1: c_int);
    }
    extern "C" {
        pub fn __throw_bad_function_call();
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __true_type {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __false_type {
        pub _address: u8,
    }
    pub type __truth_type___type = self::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __traitor {
        pub _address: u8,
    }
    pub const __traitor___value: self::std::__traitor__bindgen_ty_1 = 0;
    pub type __traitor__bindgen_ty_1 = i32;
    pub type __traitor___type = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __are_same {
        pub _address: u8,
    }
    pub const __are_same___value: self::std::__are_same__bindgen_ty_1 = 0;
    pub type __are_same__bindgen_ty_1 = i32;
    pub type __are_same___type = self::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_void {
        pub _address: u8,
    }
    pub const __is_void___value: self::std::__is_void__bindgen_ty_1 = 0;
    pub type __is_void__bindgen_ty_1 = i32;
    pub type __is_void___type = self::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_integer {
        pub _address: u8,
    }
    pub const __is_integer___value: self::std::__is_integer__bindgen_ty_1 = 0;
    pub type __is_integer__bindgen_ty_1 = i32;
    pub type __is_integer___type = self::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_floating {
        pub _address: u8,
    }
    pub const __is_floating___value: self::std::__is_floating__bindgen_ty_1 = 0;
    pub type __is_floating__bindgen_ty_1 = i32;
    pub type __is_floating___type = self::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_pointer {
        pub _address: u8,
    }
    pub const __is_pointer___value: self::std::__is_pointer__bindgen_ty_1 = 0;
    pub type __is_pointer__bindgen_ty_1 = i32;
    pub type __is_pointer___type = self::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_arithmetic {
        pub _address: u8,
    }
    impl Default for __is_arithmetic {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_scalar {
        pub _address: u8,
    }
    impl Default for __is_scalar {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_char {
        pub _address: u8,
    }
    pub const __is_char___value: self::std::__is_char__bindgen_ty_1 = 0;
    pub type __is_char__bindgen_ty_1 = i32;
    pub type __is_char___type = self::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_byte {
        pub _address: u8,
    }
    pub const __is_byte___value: self::std::__is_byte__bindgen_ty_1 = 0;
    pub type __is_byte__bindgen_ty_1 = i32;
    pub type __is_byte___type = self::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_move_iterator {
        pub _address: u8,
    }
    pub const __is_move_iterator___value: self::std::__is_move_iterator__bindgen_ty_1 = 0;
    pub type __is_move_iterator__bindgen_ty_1 = i32;
    pub type __is_move_iterator___type = self::std::__false_type;
    pub type integral_constant_value_type<_Tp> = _Tp;
    pub type integral_constant_type = u8;
    extern "C" {
        pub static value: _Tp;
    }
    pub type true_type = u8;
    pub type false_type = u8;
    pub type __bool_constant = u8;
    pub type bool_constant = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __or_ {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __and_ {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __not_ {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct conjunction {
        pub _address: u8,
    }
    impl Default for conjunction {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct disjunction {
        pub _address: u8,
    }
    impl Default for disjunction {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct negation {
        pub _address: u8,
    }
    impl Default for negation {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __success_type {
        pub _address: u8,
    }
    pub type __success_type_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __failure_type {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_void_helper {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_void {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_integral_helper {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_floating_point {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_array {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_pointer_helper {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_lvalue_reference {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_rvalue_reference {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_member_object_pointer_helper {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_member_object_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_member_function_pointer_helper {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_member_function_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_enum {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_union {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_class {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_function {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_null_pointer_helper {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_null_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nullptr_t {
        pub _address: u8,
    }
    impl Default for __is_nullptr_t {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_reference {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_arithmetic {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_fundamental {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_object {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_scalar {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_compound {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_member_pointer_helper {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_member_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_referenceable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_const {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_volatile {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivial {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_copyable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_standard_layout {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_pod {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_literal_type {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_empty {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_polymorphic {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_final {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_abstract {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_signed {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_unsigned {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_array_known_bounds {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_array_unknown_bounds {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __do_is_destructible_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_destructible_impl {
        pub _address: u8,
    }
    pub type __is_destructible_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_destructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __do_is_nt_destructible_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nt_destructible_impl {
        pub _address: u8,
    }
    pub type __is_nt_destructible_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_destructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __do_is_default_constructible_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_default_constructible_impl {
        pub _address: u8,
    }
    pub type __is_default_constructible_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_default_constructible_atom {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_default_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_copy_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_move_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nt_default_constructible_atom {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_default_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nt_constructible_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_copy_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_move_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_copy_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_move_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nt_assignable_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_copy_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_move_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_default_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __do_is_implicitly_default_constructible_impl {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}__test"]
        pub fn __do_is_implicitly_default_constructible_impl___test() -> self::std::false_type;
    }
    impl __do_is_implicitly_default_constructible_impl {
        #[inline]
        pub unsafe fn __test() -> self::std::false_type {
            __do_is_implicitly_default_constructible_impl___test()
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_implicitly_default_constructible_impl {
        pub _address: u8,
    }
    pub type __is_implicitly_default_constructible_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_implicitly_default_constructible_safe {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_implicitly_default_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_copy_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_move_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_copy_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_move_assignable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_trivially_destructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct has_virtual_destructor {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct alignment_of {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct rank {
        pub _base: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_same {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_base_of {
        pub _address: u8,
    }
    pub type __is_convertible_helper_type = self::std::is_void;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_convertible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct remove_const {
        pub _address: u8,
    }
    pub type remove_const_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct remove_volatile {
        pub _address: u8,
    }
    pub type remove_volatile_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct remove_cv {
        pub _address: u8,
    }
    pub type remove_cv_type = self::std::remove_const;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct add_const {
        pub _address: u8,
    }
    pub type add_const_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct add_volatile {
        pub _address: u8,
    }
    pub type add_volatile_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct add_cv {
        pub _address: u8,
    }
    pub type add_cv_type = self::std::add_const;
    pub type remove_const_t = self::std::remove_const;
    pub type remove_volatile_t = self::std::remove_volatile;
    pub type remove_cv_t = self::std::remove_cv;
    pub type add_const_t = self::std::add_const;
    pub type add_volatile_t = self::std::add_volatile;
    pub type add_cv_t = self::std::add_cv;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct remove_reference {
        pub _address: u8,
    }
    pub type remove_reference_type<_Tp> = _Tp;
    pub type __add_lvalue_reference_helper_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct add_lvalue_reference {
        pub _address: u8,
    }
    pub type __add_rvalue_reference_helper_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct add_rvalue_reference {
        pub _address: u8,
    }
    pub type remove_reference_t = self::std::remove_reference;
    pub type add_lvalue_reference_t = self::std::add_lvalue_reference;
    pub type add_rvalue_reference_t = self::std::add_rvalue_reference;
    pub type __match_cv_qualifiers___match = u8;
    pub type __match_cv_qualifiers___type = self::std::__match_cv_qualifiers___match;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __make_unsigned {
        pub _address: u8,
    }
    pub type __make_unsigned___type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct make_unsigned {
        pub _address: u8,
    }
    pub type make_unsigned_type = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __make_signed {
        pub _address: u8,
    }
    pub type __make_signed___type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct make_signed {
        pub _address: u8,
    }
    pub type make_signed_type = u8;
    pub type make_signed_t = self::std::make_signed;
    pub type make_unsigned_t = self::std::make_unsigned;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct remove_extent {
        pub _address: u8,
    }
    pub type remove_extent_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct remove_all_extents {
        pub _address: u8,
    }
    pub type remove_all_extents_type<_Tp> = _Tp;
    pub type remove_extent_t = self::std::remove_extent;
    pub type remove_all_extents_t = self::std::remove_all_extents;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __remove_pointer_helper {
        pub _address: u8,
    }
    pub type __remove_pointer_helper_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct remove_pointer {
        pub _address: u8,
    }
    impl Default for remove_pointer {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type __add_pointer_helper_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct add_pointer {
        pub _address: u8,
    }
    pub type remove_pointer_t = self::std::remove_pointer;
    pub type add_pointer_t = self::std::add_pointer;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union __aligned_storage_msa___type {
        pub __data: *mut c_uchar,
        pub __align: self::std::__aligned_storage_msa___type__bindgen_ty_1,
        _bindgen_union_align: u64,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __aligned_storage_msa___type__bindgen_ty_1 {
        pub _address: u8,
    }
    impl Default for __aligned_storage_msa___type {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Debug for __aligned_storage_msa___type {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "__aligned_storage_msa___type {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union aligned_storage_type {
        pub __data: *mut c_uchar,
        pub __align: self::std::aligned_storage_type__bindgen_ty_1,
        _bindgen_union_align: u64,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct aligned_storage_type__bindgen_ty_1 {
        pub _address: u8,
    }
    impl Default for aligned_storage_type {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Debug for aligned_storage_type {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "aligned_storage_type {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __strictest_alignment {
        pub _address: u8,
    }
    pub type aligned_union___strictest = self::std::__strictest_alignment;
    pub type aligned_union_type = u8;
    extern "C" {
        pub static alignment_value: usize;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct decay {
        pub _address: u8,
    }
    pub type decay___remove_type = self::std::remove_reference;
    pub type decay_type = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct reference_wrapper {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __strip_reference_wrapper {
        pub _address: u8,
    }
    pub type __strip_reference_wrapper___type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __decay_and_strip {
        pub _address: u8,
    }
    pub type __decay_and_strip___type = self::std::__strip_reference_wrapper;
    pub type _Require = u8;
    pub type conditional_type<_Iftrue> = _Iftrue;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct common_type {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __do_common_type_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __common_type_impl {
        pub _address: u8,
    }
    pub type __common_type_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __do_member_type_wrapper {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __member_type_wrapper {
        pub _address: u8,
    }
    pub type __member_type_wrapper_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __expanded_common_type_wrapper {
        pub _address: u8,
    }
    pub type __expanded_common_type_wrapper_type = self::std::common_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct underlying_type {
        pub _address: u8,
    }
    pub type underlying_type_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __declval_protector {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct result_of {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __invoke_memfun_ref {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __invoke_memfun_deref {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __invoke_memobj_ref {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __invoke_memobj_deref {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __invoke_other {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_success {
        pub _address: u8,
    }
    pub type __result_of_success___invoke_type<_Tag> = _Tag;
    impl Default for __result_of_success {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memfun_ref_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memfun_ref {
        pub _address: u8,
    }
    pub type __result_of_memfun_ref_type<_MemPtr> = _MemPtr;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memfun_deref_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memfun_deref {
        pub _address: u8,
    }
    pub type __result_of_memfun_deref_type<_MemPtr> = _MemPtr;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memobj_ref_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memobj_ref {
        pub _address: u8,
    }
    pub type __result_of_memobj_ref_type<_MemPtr> = _MemPtr;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memobj_deref_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memobj_deref {
        pub _address: u8,
    }
    pub type __result_of_memobj_deref_type<_MemPtr> = _MemPtr;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memobj {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_memfun {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __inv_unwrap {
        pub _address: u8,
    }
    pub type __inv_unwrap_type<_Tp> = _Tp;
    pub type __result_of_impl_type = self::std::__failure_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __result_of_other_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __invoke_result {
        pub _address: u8,
    }
    pub type aligned_storage_t = u8;
    pub type aligned_union_t = u8;
    pub type decay_t = self::std::decay;
    pub type enable_if_t = u8;
    pub type conditional_t = u8;
    pub type common_type_t = self::std::common_type;
    pub type underlying_type_t = self::std::underlying_type;
    pub type result_of_t = self::std::result_of;
    pub type __enable_if_t = u8;
    pub type __void_t = c_void;
    pub type void_t = c_void;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __detector {
        pub _address: u8,
    }
    pub type __detector_value_t = self::std::false_type;
    pub type __detector_type<_Default> = _Default;
    pub type __detected_or = self::std::__detector;
    pub type __detected_or_t = self::std::__detected_or;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct tuple {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_tuple_like_impl {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_tuple_like {
        pub _address: u8,
    }
    pub mod __swappable_details {

        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct __do_is_swappable_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct __do_is_nothrow_swappable_impl {
            pub _address: u8,
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_swappable_impl {
        pub _address: u8,
    }
    pub type __is_swappable_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nothrow_swappable_impl {
        pub _address: u8,
    }
    pub type __is_nothrow_swappable_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_swappable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nothrow_swappable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_swappable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_swappable {
        pub _address: u8,
    }
    pub mod __swappable_with_details {

        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct __do_is_swappable_with_impl {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct __do_is_nothrow_swappable_with_impl {
            pub _address: u8,
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_swappable_with_impl {
        pub _address: u8,
    }
    pub type __is_swappable_with_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nothrow_swappable_with_impl {
        pub _address: u8,
    }
    pub type __is_nothrow_swappable_with_impl_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_swappable_with {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_swappable_with {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_invocable_impl {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_invocable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __call_is_nothrow {
        pub _address: u8,
    }
    pub type __call_is_nothrow_ = self::std::__call_is_nothrow;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nothrow_invocable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __nonesuch {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct invoke_result {
        pub _address: u8,
    }
    impl Default for invoke_result {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type invoke_result_t = self::std::invoke_result;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_invocable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_invocable_r {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_invocable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nt_invocable_impl {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_nothrow_invocable_r {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct has_unique_object_representations {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_aggregate {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __move_if_noexcept_cond {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct piecewise_construct_t {
        pub _address: u8,
    }
    extern "C" {
        pub static piecewise_construct: self::std::piecewise_construct_t;
    }
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __nonesuch_no_braces {
        pub _address: u8,
    }
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
        pub _phantom_0: PhantomData<UnsafeCell<_T1>>,
        pub _phantom_1: PhantomData<UnsafeCell<_T2>>,
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
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct output_iterator_tag {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct forward_iterator_tag {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bidirectional_iterator_tag {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct random_access_iterator_tag {
        pub _address: u8,
    }
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
    pub type _RequireInputIter = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _List_iterator {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _List_const_iterator {
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
    pub type __get_first_arg_type = self::std::__undefined;
    pub type __get_first_arg_t = self::std::__get_first_arg;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __replace_first_arg {
        pub _address: u8,
    }
    pub type __replace_first_arg_t = self::std::__replace_first_arg;
    pub type __make_not_void = u8;
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
    pub type pointer_traits_element_type = self::std::__detected_or_t;
    pub type pointer_traits_difference_type = self::std::__detected_or_t;
    pub type pointer_traits_rebind = self::std::pointer_traits___rebind;
    pub type __ptr_rebind = self::std::pointer_traits;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct reverse_iterator<_Iterator> {
        pub current: _Iterator,
        pub _phantom_0: PhantomData<UnsafeCell<_Iterator>>,
    }
    pub type reverse_iterator___traits_type = self::std::iterator_traits;
    pub type reverse_iterator_iterator_type<_Iterator> = _Iterator;
    pub type reverse_iterator_difference_type = self::std::reverse_iterator___traits_type;
    pub type reverse_iterator_pointer = self::std::reverse_iterator___traits_type;
    pub type reverse_iterator_reference = self::std::reverse_iterator___traits_type;
    impl<_Iterator> Default for reverse_iterator<_Iterator> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct back_insert_iterator<_Container> {
        pub container: *mut _Container,
        pub _phantom_0: PhantomData<UnsafeCell<_Container>>,
    }
    pub type back_insert_iterator_container_type<_Container> = _Container;
    impl<_Container> Default for back_insert_iterator<_Container> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct front_insert_iterator<_Container> {
        pub container: *mut _Container,
        pub _phantom_0: PhantomData<UnsafeCell<_Container>>,
    }
    pub type front_insert_iterator_container_type<_Container> = _Container;
    impl<_Container> Default for front_insert_iterator<_Container> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    pub struct insert_iterator<_Container> {
        pub container: *mut _Container,
        pub iter: [u8; 0usize],
        pub _phantom_0: PhantomData<UnsafeCell<_Container>>,
    }
    pub type insert_iterator_container_type<_Container> = _Container;
    impl<_Container> Default for insert_iterator<_Container> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl<_Container> Debug for insert_iterator<_Container> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "insert_iterator {{ container: {:?} }}", self.container)
        }
    }
    impl<_Container> PartialEq for insert_iterator<_Container>
    where
        _Container: PartialEq,
    {
        fn eq(&self, other: &insert_iterator<_Container>) -> bool {
            self.container == other.container && self.iter == other.iter
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct move_iterator<_Iterator> {
        pub _M_current: _Iterator,
        pub _phantom_0: PhantomData<UnsafeCell<_Iterator>>,
    }
    pub type move_iterator___traits_type = self::std::iterator_traits;
    pub type move_iterator___base_ref = self::std::move_iterator___traits_type;
    pub type move_iterator_iterator_type<_Iterator> = _Iterator;
    pub type move_iterator_iterator_category = self::std::move_iterator___traits_type;
    pub type move_iterator_value_type = self::std::move_iterator___traits_type;
    pub type move_iterator_difference_type = self::std::move_iterator___traits_type;
    pub type move_iterator_pointer<_Iterator> = _Iterator;
    pub type move_iterator_reference = u8;
    impl<_Iterator> Default for move_iterator<_Iterator> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type __iter_key_t = self::std::remove_const_t;
    pub type __iter_val_t = self::std::iterator_traits;
    pub type __iter_to_alloc_t = self::std::pair<self::std::add_const_t, self::std::__iter_val_t>;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct istreambuf_iterator {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ostreambuf_iterator {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __lc_rai {
        pub _address: u8,
    }
    pub type streamoff = c_long;
    pub type streamsize = isize;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct fpos<_StateT> {
        pub _M_off: self::std::streamoff,
        pub _M_state: _StateT,
        pub _phantom_0: PhantomData<UnsafeCell<_StateT>>,
    }
    impl<_StateT> Default for fpos<_StateT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type streampos = self::std::fpos<self::mbstate_t>;
    pub type wstreampos = self::std::fpos<self::mbstate_t>;
    pub type u16streampos = self::std::fpos<self::mbstate_t>;
    pub type u32streampos = self::std::fpos<self::mbstate_t>;
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
    impl Default for exception {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}exception_destructor"]
        pub fn exception_exception_destructor(this: *mut self::std::exception);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn exception_what(this: *mut c_void) -> *const c_char;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_exception {
        pub _base: self::std::exception,
    }
    impl Default for bad_exception {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_exception_destructor"]
        pub fn bad_exception_bad_exception_destructor(this: *mut self::std::bad_exception);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_exception_what(this: *mut c_void) -> *const c_char;
    }
    pub type terminate_handler = Option<unsafe extern "C" fn()>;
    pub type unexpected_handler = Option<unsafe extern "C" fn()>;
    extern "C" {
        pub fn set_terminate(arg1: self::std::terminate_handler) -> self::std::terminate_handler;
    }
    extern "C" {
        pub fn get_terminate() -> self::std::terminate_handler;
    }
    extern "C" {
        pub fn terminate();
    }
    extern "C" {
        pub fn set_unexpected(arg1: self::std::unexpected_handler)
            -> self::std::unexpected_handler;
    }
    extern "C" {
        pub fn get_unexpected() -> self::std::unexpected_handler;
    }
    extern "C" {
        pub fn unexpected();
    }
    extern "C" {
        pub fn uncaught_exception() -> bool;
    }
    extern "C" {
        pub fn uncaught_exceptions() -> c_int;
    }
    extern "C" {
        pub fn _Hash_bytes(__ptr: *const c_void, __len: usize, __seed: usize) -> usize;
    }
    extern "C" {
        pub fn _Fnv_hash_bytes(__ptr: *const c_void, __len: usize, __seed: usize) -> usize;
    }
    #[repr(C)]
    pub struct type_info__bindgen_vtable(c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct type_info {
        pub vtable_: *const type_info__bindgen_vtable,
        pub __name: *const c_char,
    }
    impl Default for type_info {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}type_info_destructor"]
        pub fn type_info_type_info_destructor(this: *mut self::std::type_info);
    }
    extern "C" {
        #[link_name = "\u{1}__is_pointer_p"]
        pub fn type_info___is_pointer_p(this: *mut c_void) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}__is_function_p"]
        pub fn type_info___is_function_p(this: *mut c_void) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}__do_catch"]
        pub fn type_info___do_catch(
            this: *mut c_void,
            __thr_type: *const self::std::type_info,
            __thr_obj: *mut *mut c_void,
            __outer: c_uint,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}__do_upcast"]
        pub fn type_info___do_upcast(
            this: *mut c_void,
            __target: *const self::__cxxabiv1::__class_type_info,
            __obj_ptr: *mut *mut c_void,
        ) -> bool;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_cast {
        pub _base: self::std::exception,
    }
    impl Default for bad_cast {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_cast_destructor"]
        pub fn bad_cast_bad_cast_destructor(this: *mut self::std::bad_cast);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_cast_what(this: *mut c_void) -> *const c_char;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_typeid {
        pub _base: self::std::exception,
    }
    impl Default for bad_typeid {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_typeid_destructor"]
        pub fn bad_typeid_bad_typeid_destructor(this: *mut self::std::bad_typeid);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_typeid_what(this: *mut c_void) -> *const c_char;
    }
    pub mod __exception_ptr {

        #[repr(C)]
        #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct exception_ptr {
            pub _M_exception_object: *mut c_void,
        }
        extern "C" {
            #[link_name = "\u{1}swap"]
            pub fn exception_ptr_swap(
                this: *mut self::std::__exception_ptr::exception_ptr,
                arg1: *mut self::std::__exception_ptr::exception_ptr,
            );
        }
        extern "C" {
            #[link_name = "\u{1}__cxa_exception_type"]
            pub fn exception_ptr___cxa_exception_type(
                this: *const self::std::__exception_ptr::exception_ptr,
            ) -> *const self::std::type_info;
        }
        extern "C" {
            #[link_name = "\u{1}exception_ptr"]
            pub fn exception_ptr_exception_ptr(
                this: *mut self::std::__exception_ptr::exception_ptr,
            );
        }
        extern "C" {
            #[link_name = "\u{1}exception_ptr"]
            pub fn exception_ptr_exception_ptr1(
                this: *mut self::std::__exception_ptr::exception_ptr,
                arg1: *const self::std::__exception_ptr::exception_ptr,
            );
        }
        extern "C" {
            #[link_name = "\u{1}exception_ptr_destructor"]
            pub fn exception_ptr_exception_ptr_destructor(
                this: *mut self::std::__exception_ptr::exception_ptr,
            );
        }
        impl Default for exception_ptr {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        impl exception_ptr {
            #[inline]
            pub unsafe fn swap(&mut self, arg1: *mut self::std::__exception_ptr::exception_ptr) {
                exception_ptr_swap(self, arg1)
            }
            #[inline]
            pub unsafe fn __cxa_exception_type(&self) -> *const self::std::type_info {
                exception_ptr___cxa_exception_type(self)
            }
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = uninitialized();
                exception_ptr_exception_ptr(&mut __bindgen_tmp);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new1(arg1: *const self::std::__exception_ptr::exception_ptr) -> Self {
                let mut __bindgen_tmp = uninitialized();
                exception_ptr_exception_ptr1(&mut __bindgen_tmp, arg1);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                exception_ptr_exception_ptr_destructor(self)
            }
        }
    }
    extern "C" {
        pub fn current_exception() -> self::std::__exception_ptr::exception_ptr;
    }
    extern "C" {
        pub fn rethrow_exception(arg1: self::std::__exception_ptr::exception_ptr);
    }
    #[repr(C)]
    pub struct nested_exception__bindgen_vtable(c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct nested_exception {
        pub vtable_: *const nested_exception__bindgen_vtable,
        pub _M_ptr: self::std::__exception_ptr::exception_ptr,
    }
    impl Default for nested_exception {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}nested_exception_destructor"]
        pub fn nested_exception_nested_exception_destructor(this: *mut self::std::nested_exception);
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Nested_exception<_Except> {
        pub _base: _Except,
        pub _base_1: self::std::nested_exception,
        pub _phantom_0: PhantomData<UnsafeCell<_Except>>,
    }
    impl<_Except> Default for _Nested_exception<_Except> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type __rethrow_if_nested_cond = u8;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_alloc {
        pub _base: self::std::exception,
    }
    impl Default for bad_alloc {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_alloc_destructor"]
        pub fn bad_alloc_bad_alloc_destructor(this: *mut self::std::bad_alloc);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_alloc_what(this: *mut c_void) -> *const c_char;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_array_new_length {
        pub _base: self::std::bad_alloc,
    }
    impl Default for bad_array_new_length {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_array_new_length_destructor"]
        pub fn bad_array_new_length_bad_array_new_length_destructor(
            this: *mut self::std::bad_array_new_length,
        );
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_array_new_length_what(this: *mut c_void) -> *const c_char;
    }
    pub type align_val_t = u64;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct nothrow_t {
        pub _address: u8,
    }
    extern "C" {
        pub static nothrow: self::std::nothrow_t;
    }
    pub type new_handler = Option<unsafe extern "C" fn()>;
    extern "C" {
        pub fn set_new_handler(arg1: self::std::new_handler) -> self::std::new_handler;
    }
    extern "C" {
        pub fn get_new_handler() -> self::std::new_handler;
    }
    pub type __allocator_base = self::__gnu_cxx::new_allocator;
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
    pub type allocator_rebind_other = self::std::allocator;
    pub type allocator_propagate_on_container_move_assignment = self::std::true_type;
    pub type allocator_is_always_equal = self::std::true_type;
    impl Default for allocator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub type __c_locale = self::__locale_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ios_base {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ios {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_streambuf {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_istream {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ostream {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_iostream {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_filebuf {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ifstream {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ofstream {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_fstream {
        pub _address: u8,
    }
    pub type ios = self::std::basic_ios;
    pub type streambuf = self::std::basic_streambuf;
    pub type istream = self::std::basic_istream;
    pub type ostream = self::std::basic_ostream;
    pub type iostream = self::std::basic_iostream;
    pub type stringbuf = self::std::__cxx11::basic_stringbuf;
    pub type istringstream = self::std::__cxx11::basic_istringstream;
    pub type ostringstream = self::std::__cxx11::basic_ostringstream;
    pub type stringstream = self::std::__cxx11::basic_stringstream;
    pub type filebuf = self::std::basic_filebuf;
    pub type ifstream = self::std::basic_ifstream;
    pub type ofstream = self::std::basic_ofstream;
    pub type fstream = self::std::basic_fstream;
    pub type wios = self::std::basic_ios;
    pub type wstreambuf = self::std::basic_streambuf;
    pub type wistream = self::std::basic_istream;
    pub type wostream = self::std::basic_ostream;
    pub type wiostream = self::std::basic_iostream;
    pub type wstringbuf = self::std::__cxx11::basic_stringbuf;
    pub type wistringstream = self::std::__cxx11::basic_istringstream;
    pub type wostringstream = self::std::__cxx11::basic_ostringstream;
    pub type wstringstream = self::std::__cxx11::basic_stringstream;
    pub type wfilebuf = self::std::basic_filebuf;
    pub type wifstream = self::std::basic_ifstream;
    pub type wofstream = self::std::basic_ofstream;
    pub type wfstream = self::std::basic_fstream;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct locale {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ctype_base {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ctype {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ctype_byname {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct codecvt_base {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct codecvt {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct codecvt_byname {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct num_get {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct num_put {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct time_base {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct time_put {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
    #[derive(Debug, Copy, Clone)]
    pub struct __is_transparent {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct plus {
        pub _address: u8,
    }
    impl Default for plus {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct minus {
        pub _address: u8,
    }
    impl Default for minus {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct multiplies {
        pub _address: u8,
    }
    impl Default for multiplies {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct divides {
        pub _address: u8,
    }
    impl Default for divides {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct modulus {
        pub _address: u8,
    }
    impl Default for modulus {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct negate {
        pub _address: u8,
    }
    impl Default for negate {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct equal_to {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct not_equal_to {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct greater {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct less {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct greater_equal {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct less_equal {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct logical_and {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct logical_or {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct logical_not {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bit_and {
        pub _address: u8,
    }
    impl Default for bit_and {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bit_or {
        pub _address: u8,
    }
    impl Default for bit_or {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bit_xor {
        pub _address: u8,
    }
    impl Default for bit_xor {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bit_not {
        pub _address: u8,
    }
    impl Default for bit_not {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct unary_negate<_Predicate> {
        pub _M_pred: _Predicate,
        pub _phantom_0: PhantomData<UnsafeCell<_Predicate>>,
    }
    impl<_Predicate> Default for unary_negate<_Predicate> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct binary_negate<_Predicate> {
        pub _M_pred: _Predicate,
        pub _phantom_0: PhantomData<UnsafeCell<_Predicate>>,
    }
    impl<_Predicate> Default for binary_negate<_Predicate> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct pointer_to_unary_function<_Arg, _Result> {
        pub _M_ptr: Option<unsafe extern "C" fn(arg1: _Arg) -> _Result>,
        pub _phantom_0: PhantomData<UnsafeCell<_Arg>>,
        pub _phantom_1: PhantomData<UnsafeCell<_Result>>,
    }
    impl<_Arg, _Result> Default for pointer_to_unary_function<_Arg, _Result> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct pointer_to_binary_function<_Arg1, _Arg2, _Result> {
        pub _M_ptr: Option<unsafe extern "C" fn(arg1: _Arg1, arg2: _Arg2) -> _Result>,
        pub _phantom_0: PhantomData<UnsafeCell<_Arg1>>,
        pub _phantom_1: PhantomData<UnsafeCell<_Arg2>>,
        pub _phantom_2: PhantomData<UnsafeCell<_Result>>,
    }
    impl<_Arg1, _Arg2, _Result> Default for pointer_to_binary_function<_Arg1, _Arg2, _Result> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Identity {
        pub _address: u8,
    }
    impl Default for _Identity {
        fn default() -> Self {
            unsafe { zeroed() }
        }
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
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Select2nd {
        pub _address: u8,
    }
    impl Default for _Select2nd {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct mem_fun_t<_Ret> {
        pub _M_f: Option<unsafe extern "C" fn() -> _Ret>,
        pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
    }
    impl<_Ret> Default for mem_fun_t<_Ret> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct const_mem_fun_t<_Ret> {
        pub _M_f: Option<unsafe extern "C" fn() -> _Ret>,
        pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
    }
    impl<_Ret> Default for const_mem_fun_t<_Ret> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct mem_fun_ref_t<_Ret> {
        pub _M_f: Option<unsafe extern "C" fn() -> _Ret>,
        pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
    }
    impl<_Ret> Default for mem_fun_ref_t<_Ret> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct const_mem_fun_ref_t<_Ret> {
        pub _M_f: Option<unsafe extern "C" fn() -> _Ret>,
        pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
    }
    impl<_Ret> Default for const_mem_fun_ref_t<_Ret> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct mem_fun1_t<_Ret, _Arg> {
        pub _M_f: Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
        pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
        pub _phantom_1: PhantomData<UnsafeCell<_Arg>>,
    }
    impl<_Ret, _Arg> Default for mem_fun1_t<_Ret, _Arg> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct const_mem_fun1_t<_Ret, _Arg> {
        pub _M_f: Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
        pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
        pub _phantom_1: PhantomData<UnsafeCell<_Arg>>,
    }
    impl<_Ret, _Arg> Default for const_mem_fun1_t<_Ret, _Arg> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct mem_fun1_ref_t<_Ret, _Arg> {
        pub _M_f: Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
        pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
        pub _phantom_1: PhantomData<UnsafeCell<_Arg>>,
    }
    impl<_Ret, _Arg> Default for mem_fun1_ref_t<_Ret, _Arg> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct const_mem_fun1_ref_t<_Ret, _Arg> {
        pub _M_f: Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
        pub _phantom_0: PhantomData<UnsafeCell<_Ret>>,
        pub _phantom_1: PhantomData<UnsafeCell<_Arg>>,
    }
    impl<_Ret, _Arg> Default for const_mem_fun1_ref_t<_Ret, _Arg> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    pub struct binder1st<_Operation> {
        pub op: _Operation,
        pub value: [u8; 0usize],
        pub _phantom_0: PhantomData<UnsafeCell<_Operation>>,
    }
    impl<_Operation> Default for binder1st<_Operation> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl<_Operation> Debug for binder1st<_Operation> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "binder1st {{ op: Non-debuggable generic }}")
        }
    }
    impl<_Operation> PartialEq for binder1st<_Operation>
    where
        _Operation: PartialEq,
    {
        fn eq(&self, other: &binder1st<_Operation>) -> bool {
            self.op == other.op && self.value == other.value
        }
    }
    #[repr(C)]
    pub struct binder2nd<_Operation> {
        pub op: _Operation,
        pub value: [u8; 0usize],
        pub _phantom_0: PhantomData<UnsafeCell<_Operation>>,
    }
    impl<_Operation> Default for binder2nd<_Operation> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl<_Operation> Debug for binder2nd<_Operation> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "binder2nd {{ op: Non-debuggable generic }}")
        }
    }
    impl<_Operation> PartialEq for binder2nd<_Operation>
    where
        _Operation: PartialEq,
    {
        fn eq(&self, other: &binder2nd<_Operation>) -> bool {
            self.op == other.op && self.value == other.value
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct initializer_list<_E> {
        pub _M_array: self::std::initializer_list_iterator<_E>,
        pub _M_len: self::std::initializer_list_size_type,
        pub _phantom_0: PhantomData<UnsafeCell<_E>>,
    }
    pub type initializer_list_value_type<_E> = _E;
    pub type initializer_list_reference<_E> = *const _E;
    pub type initializer_list_const_reference<_E> = *const _E;
    pub type initializer_list_size_type = usize;
    pub type initializer_list_iterator<_E> = *const _E;
    pub type initializer_list_const_iterator<_E> = *const _E;
    impl<_E> Default for initializer_list<_E> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct valarray {
        pub _address: u8,
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
    pub type __alloc_rebind = self::std::__allocator_traits_base;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits {
        pub _address: u8,
    }
    pub type allocator_traits_allocator_type<_Alloc> = _Alloc;
    pub type allocator_traits_value_type = [u8; 0usize];
    pub type allocator_traits_pointer = self::std::__detected_or_t;
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
    pub type allocator_traits__Diff_type = self::std::pointer_traits;
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
    pub type allocator_traits_void_pointer = self::std::allocator_traits__Ptr;
    pub type allocator_traits_const_void_pointer = self::std::allocator_traits__Ptr;
    pub type allocator_traits_difference_type = [u8; 0usize];
    pub type allocator_traits_size_type = [u8; 0usize];
    pub type allocator_traits_propagate_on_container_copy_assignment = self::std::__detected_or_t;
    pub type allocator_traits_propagate_on_container_move_assignment = self::std::__detected_or_t;
    pub type allocator_traits_propagate_on_container_swap = self::std::__detected_or_t;
    pub type allocator_traits_is_always_equal = self::std::__detected_or_t;
    pub type allocator_traits_rebind_alloc = self::std::__alloc_rebind;
    pub type allocator_traits_rebind_traits = self::std::allocator_traits;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits___construct_helper {
        pub _address: u8,
    }
    pub type allocator_traits___construct_helper_type<_Alloc> = _Alloc;
    pub type allocator_traits___has_construct = self::std::allocator_traits___construct_helper;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_copy_insertable_impl {
        pub _address: u8,
    }
    pub type __is_copy_insertable_impl__Traits = self::std::allocator_traits;
    pub type __is_copy_insertable_impl_type<_Alloc> = _Alloc;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_copy_insertable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_allocator {
        pub _base: self::std::false_type,
    }
    pub type _RequireAllocator = u8;
    pub const float_round_style_round_indeterminate: self::std::float_round_style = -1;
    pub const float_round_style_round_toward_zero: self::std::float_round_style = 0;
    pub const float_round_style_round_to_nearest: self::std::float_round_style = 1;
    pub const float_round_style_round_toward_infinity: self::std::float_round_style = 2;
    pub const float_round_style_round_toward_neg_infinity: self::std::float_round_style = 3;
    pub type float_round_style = i32;
    pub const float_denorm_style_denorm_indeterminate: self::std::float_denorm_style = -1;
    pub const float_denorm_style_denorm_absent: self::std::float_denorm_style = 0;
    pub const float_denorm_style_denorm_present: self::std::float_denorm_style = 1;
    pub type float_denorm_style = i32;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __numeric_limits_base {
        pub _address: u8,
    }
    pub const __numeric_limits_base_is_specialized: bool = false;
    pub const __numeric_limits_base_digits: c_int = 0;
    pub const __numeric_limits_base_digits10: c_int = 0;
    pub const __numeric_limits_base_max_digits10: c_int = 0;
    pub const __numeric_limits_base_is_signed: bool = false;
    pub const __numeric_limits_base_is_integer: bool = false;
    pub const __numeric_limits_base_is_exact: bool = false;
    pub const __numeric_limits_base_radix: c_int = 0;
    pub const __numeric_limits_base_min_exponent: c_int = 0;
    pub const __numeric_limits_base_min_exponent10: c_int = 0;
    pub const __numeric_limits_base_max_exponent: c_int = 0;
    pub const __numeric_limits_base_max_exponent10: c_int = 0;
    pub const __numeric_limits_base_has_infinity: bool = false;
    pub const __numeric_limits_base_has_quiet_NaN: bool = false;
    pub const __numeric_limits_base_has_signaling_NaN: bool = false;
    extern "C" {
        #[link_name = "\u{1}has_denorm"]
        pub static __numeric_limits_base_has_denorm: self::std::float_denorm_style;
    }
    pub const __numeric_limits_base_has_denorm_loss: bool = false;
    pub const __numeric_limits_base_is_iec559: bool = false;
    pub const __numeric_limits_base_is_bounded: bool = false;
    pub const __numeric_limits_base_is_modulo: bool = false;
    pub const __numeric_limits_base_traps: bool = false;
    pub const __numeric_limits_base_tinyness_before: bool = false;
    extern "C" {
        #[link_name = "\u{1}round_style"]
        pub static __numeric_limits_base_round_style: self::std::float_round_style;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct numeric_limits {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __hash_base {
        pub _address: u8,
    }
    pub type __hash_base_result_type<_Result> = _Result;
    pub type __hash_base_argument_type<_Arg> = _Arg;
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __poison_hash {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct hash {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Hash_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Fnv_hash_impl {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_fast_hash {
        pub _base: self::std::true_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_string_view<_CharT> {
        pub _M_len: usize,
        pub _M_str: *const _CharT,
        pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
    }
    pub type basic_string_view_traits_type<_Traits> = _Traits;
    pub type basic_string_view_value_type<_CharT> = _CharT;
    pub type basic_string_view_pointer<_CharT> = *const _CharT;
    pub type basic_string_view_const_pointer<_CharT> = *const _CharT;
    pub type basic_string_view_reference<_CharT> = *const _CharT;
    pub type basic_string_view_const_reference<_CharT> = *const _CharT;
    pub type basic_string_view_const_iterator<_CharT> = *const _CharT;
    pub type basic_string_view_iterator<_CharT> =
        self::std::basic_string_view_const_iterator<_CharT>;
    pub type basic_string_view_const_reverse_iterator<_CharT> =
        self::std::reverse_iterator<self::std::basic_string_view_const_iterator<_CharT>>;
    pub type basic_string_view_reverse_iterator<_CharT> =
        self::std::basic_string_view_const_reverse_iterator<_CharT>;
    pub type basic_string_view_size_type = usize;
    pub type basic_string_view_difference_type = isize;
    impl<_CharT> Default for basic_string_view<_CharT> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub mod __detail {

        pub type __idt = self::std::common_type_t;
    }
    pub type string_view = self::std::basic_string_view<c_char>;
    pub type wstring_view = self::std::basic_string_view<u32>;
    pub type u16string_view = self::std::basic_string_view<u16>;
    pub type u32string_view = self::std::basic_string_view<u32>;
    extern "C" {
        pub fn getline(
            __in: *mut self::std::basic_istream,
            __str: *mut self::std::__cxx11::basic_string<c_char>,
            __delim: c_char,
        ) -> *mut self::std::basic_istream;
    }
    extern "C" {
        pub static npos: self::std::__cxx11::basic_string_size_type;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct tuple_size {
        pub _address: u8,
    }
    pub type __enable_if_has_tuple_size<_Tp> = _Tp;
    pub type __tuple_element_t = u8;
    pub type tuple_element_t = u8;
    pub type _Build_index_tuple__IdxTuple = u8;
    pub type _Build_index_tuple___type = u8;
    pub type integer_sequence_value_type<_Tp> = _Tp;
    pub type make_integer_sequence = u8;
    pub type index_sequence = u8;
    pub type make_index_sequence = self::std::make_integer_sequence;
    pub type index_sequence_for = self::std::make_index_sequence;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct in_place_t {
        pub _address: u8,
    }
    extern "C" {
        pub static in_place: self::std::in_place_t;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct in_place_type_t {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_in_place_type_impl {
        pub _base: self::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_in_place_type {
        pub _base: self::std::__is_in_place_type_impl,
    }
    impl Default for __is_in_place_type {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    pub struct _Vector_base {
        pub _M_impl: self::std::_Vector_base__Vector_impl,
    }
    pub type _Vector_base__Tp_alloc_type = [u8; 0usize];
    pub type _Vector_base_pointer = [u8; 0usize];
    #[repr(C)]
    pub struct _Vector_base__Vector_impl {
        pub _M_start: self::std::_Vector_base_pointer,
        pub _M_finish: self::std::_Vector_base_pointer,
        pub _M_end_of_storage: self::std::_Vector_base_pointer,
    }
    impl Default for _Vector_base__Vector_impl {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Debug for _Vector_base__Vector_impl {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    impl Debug for _Vector_base {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
        pub _base: self::std::_Vector_base,
    }
    pub type vector__Base = self::std::_Vector_base;
    pub type vector__Tp_alloc_type = self::std::vector__Base;
    pub type vector__Alloc_traits = self::__gnu_cxx::__alloc_traits;
    pub type vector_value_type<_Tp> = _Tp;
    pub type vector_pointer = self::std::vector__Base;
    pub type vector_const_pointer = self::std::vector__Alloc_traits;
    pub type vector_reference = self::std::vector__Alloc_traits;
    pub type vector_const_reference = self::std::vector__Alloc_traits;
    pub type vector_iterator = self::__gnu_cxx::__normal_iterator<self::std::vector_pointer>;
    pub type vector_const_iterator =
        self::__gnu_cxx::__normal_iterator<self::std::vector_const_pointer>;
    pub type vector_const_reverse_iterator =
        self::std::reverse_iterator<self::std::vector_const_iterator>;
    pub type vector_reverse_iterator = self::std::reverse_iterator<self::std::vector_iterator>;
    pub type vector_size_type = usize;
    pub type vector_difference_type = isize;
    pub type vector_allocator_type<_Alloc> = _Alloc;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct vector__Temporary_value {
        pub _M_this: *mut self::std::vector,
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
    impl Debug for vector {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "vector {{  }}")
        }
    }
    impl PartialEq for vector {
        fn eq(&self, other: &vector) -> bool {
            self._base == other._base
        }
    }
    pub type _Bit_type = c_ulong;
    pub const std__S_word_bit: self::std::_bindgen_ty_1 = 64;
    pub type _bindgen_ty_1 = u32;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Bit_reference {
        pub _M_p: *mut self::std::_Bit_type,
        pub _M_mask: self::std::_Bit_type,
    }
    impl Default for _Bit_reference {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Bit_iterator_base {
        pub _M_p: *mut self::std::_Bit_type,
        pub _M_offset: c_uint,
    }
    impl Default for _Bit_iterator_base {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Bit_iterator {
        pub _base: self::std::_Bit_iterator_base,
    }
    pub type _Bit_iterator_reference = self::std::_Bit_reference;
    pub type _Bit_iterator_pointer = *mut self::std::_Bit_reference;
    pub type _Bit_iterator_iterator = self::std::_Bit_iterator;
    impl Default for _Bit_iterator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Bit_const_iterator {
        pub _base: self::std::_Bit_iterator_base,
    }
    pub type _Bit_const_iterator_reference = bool;
    pub type _Bit_const_iterator_const_reference = bool;
    pub type _Bit_const_iterator_pointer = *const bool;
    pub type _Bit_const_iterator_const_iterator = self::std::_Bit_const_iterator;
    impl Default for _Bit_const_iterator {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    #[repr(C)]
    pub struct _Bvector_base {
        pub _M_impl: self::std::_Bvector_base__Bvector_impl,
    }
    pub type _Bvector_base__Bit_alloc_type = [u8; 0usize];
    pub type _Bvector_base__Bit_alloc_traits = self::__gnu_cxx::__alloc_traits;
    pub type _Bvector_base__Bit_pointer = [u8; 0usize];
    #[repr(C)]
    pub struct _Bvector_base__Bvector_impl_data {
        pub _M_start: self::std::_Bit_iterator,
        pub _M_finish: self::std::_Bit_iterator,
        pub _M_end_of_storage: self::std::_Bvector_base__Bit_pointer,
    }
    impl Default for _Bvector_base__Bvector_impl_data {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Debug for _Bvector_base__Bvector_impl_data {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "_Bvector_base__Bvector_impl_data {{ _M_start: {:?}, _M_finish: {:?} }}",
                self._M_start, self._M_finish
            )
        }
    }
    impl PartialEq for _Bvector_base__Bvector_impl_data {
        fn eq(&self, other: &_Bvector_base__Bvector_impl_data) -> bool {
            self._M_start == other._M_start
                && self._M_finish == other._M_finish
                && self._M_end_of_storage == other._M_end_of_storage
        }
    }
    #[repr(C)]
    pub struct _Bvector_base__Bvector_impl {
        pub _base_1: self::std::_Bvector_base__Bvector_impl_data,
    }
    impl Default for _Bvector_base__Bvector_impl {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Debug for _Bvector_base__Bvector_impl {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "_Bvector_base__Bvector_impl {{  }}")
        }
    }
    impl PartialEq for _Bvector_base__Bvector_impl {
        fn eq(&self, other: &_Bvector_base__Bvector_impl) -> bool {
            self._base_1 == other._base_1
        }
    }
    pub type _Bvector_base_allocator_type<_Alloc> = _Alloc;
    impl Default for _Bvector_base {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    impl Debug for _Bvector_base {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "_Bvector_base {{ _M_impl: {:?} }}", self._M_impl)
        }
    }
    impl PartialEq for _Bvector_base {
        fn eq(&self, other: &_Bvector_base) -> bool {
            self._M_impl == other._M_impl
        }
    }
}
pub mod __gnu_cxx {
    pub type __conditional_type___type<_Iftrue> = _Iftrue;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __add_unsigned {
        pub _address: u8,
    }
    pub type __add_unsigned___if_type = u8;
    pub type __add_unsigned___type = self::__gnu_cxx::__add_unsigned___if_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __remove_unsigned {
        pub _address: u8,
    }
    pub type __remove_unsigned___if_type = u8;
    pub type __remove_unsigned___type = self::__gnu_cxx::__remove_unsigned___if_type;
    pub type __promote___type = f64;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __promote_2 {
        pub _address: u8,
    }
    pub type __promote_2___type<_Tp2> = _Tp2;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __promote_3 {
        pub _address: u8,
    }
    pub type __promote_3___type<_Tp2> = _Tp2;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __promote_4 {
        pub _address: u8,
    }
    pub type __promote_4___type<_Tp2> = _Tp2;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __numeric_traits {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __normal_iterator<_Iterator> {
        pub _M_current: _Iterator,
        pub _phantom_0: PhantomData<UnsafeCell<_Iterator>>,
    }
    pub type __normal_iterator___traits_type = self::std::iterator_traits;
    pub type __normal_iterator_iterator_type<_Iterator> = _Iterator;
    pub type __normal_iterator_iterator_category = self::__gnu_cxx::__normal_iterator___traits_type;
    pub type __normal_iterator_value_type = self::__gnu_cxx::__normal_iterator___traits_type;
    pub type __normal_iterator_difference_type = self::__gnu_cxx::__normal_iterator___traits_type;
    pub type __normal_iterator_reference = self::__gnu_cxx::__normal_iterator___traits_type;
    pub type __normal_iterator_pointer = self::__gnu_cxx::__normal_iterator___traits_type;
    impl<_Iterator> Default for __normal_iterator<_Iterator> {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
    pub mod __ops {
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_less_iter {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_less_val {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Val_less_iter {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_equal_to_iter {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_equal_to_val {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_comp_iter<_Compare> {
            pub _M_comp: _Compare,
            pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
        }
        impl<_Compare> Default for _Iter_comp_iter<_Compare> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_comp_val<_Compare> {
            pub _M_comp: _Compare,
            pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
        }
        impl<_Compare> Default for _Iter_comp_val<_Compare> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Val_comp_iter<_Compare> {
            pub _M_comp: _Compare,
            pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
        }
        impl<_Compare> Default for _Val_comp_iter<_Compare> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_equals_val<_Value> {
            pub _M_value: *mut _Value,
            pub _phantom_0: PhantomData<UnsafeCell<_Value>>,
        }
        impl<_Value> Default for _Iter_equals_val<_Value> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_equals_iter<_Iterator1> {
            pub _M_it1: _Iterator1,
            pub _phantom_0: PhantomData<UnsafeCell<_Iterator1>>,
        }
        impl<_Iterator1> Default for _Iter_equals_iter<_Iterator1> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_pred<_Predicate> {
            pub _M_pred: _Predicate,
            pub _phantom_0: PhantomData<UnsafeCell<_Predicate>>,
        }
        impl<_Predicate> Default for _Iter_pred<_Predicate> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_comp_to_val<_Compare, _Value> {
            pub _M_comp: _Compare,
            pub _M_value: *mut _Value,
            pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
            pub _phantom_1: PhantomData<UnsafeCell<_Value>>,
        }
        impl<_Compare, _Value> Default for _Iter_comp_to_val<_Compare, _Value> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_comp_to_iter<_Compare, _Iterator1> {
            pub _M_comp: _Compare,
            pub _M_it1: _Iterator1,
            pub _phantom_0: PhantomData<UnsafeCell<_Compare>>,
            pub _phantom_1: PhantomData<UnsafeCell<_Iterator1>>,
        }
        impl<_Compare, _Iterator1> Default for _Iter_comp_to_iter<_Compare, _Iterator1> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_negate<_Predicate> {
            pub _M_pred: _Predicate,
            pub _phantom_0: PhantomData<UnsafeCell<_Predicate>>,
        }
        impl<_Predicate> Default for _Iter_negate<_Predicate> {
            fn default() -> Self {
                unsafe { zeroed() }
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Char_types {
        pub _address: u8,
    }
    pub type _Char_types_int_type = c_ulong;
    pub type _Char_types_pos_type = self::std::streampos;
    pub type _Char_types_off_type = self::std::streamoff;
    pub type _Char_types_state_type = self::mbstate_t;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct char_traits {
        pub _address: u8,
    }
    pub type char_traits_char_type<_CharT> = _CharT;
    pub type char_traits_int_type = self::__gnu_cxx::_Char_types;
    pub type char_traits_pos_type = self::__gnu_cxx::_Char_types;
    pub type char_traits_off_type = self::__gnu_cxx::_Char_types;
    pub type char_traits_state_type = self::__gnu_cxx::_Char_types;
    extern "C" {
        pub fn __verbose_terminate_handler();
    }
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
    pub type new_allocator_rebind_other = self::__gnu_cxx::new_allocator;
    pub type new_allocator_propagate_on_container_move_assignment = self::std::true_type;
    extern "C" {
        pub fn __uselocale(arg1: self::locale_t) -> self::locale_t;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __alloc_traits {
        pub _address: u8,
    }
    pub type __alloc_traits_allocator_type<_Alloc> = _Alloc;
    pub type __alloc_traits__Base_type = self::std::allocator_traits;
    pub type __alloc_traits_value_type = self::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_pointer = self::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_const_pointer = self::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_size_type = self::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_difference_type = self::__gnu_cxx::__alloc_traits__Base_type;
    pub type __alloc_traits_reference = *mut self::__gnu_cxx::__alloc_traits_value_type;
    pub type __alloc_traits_const_reference = *const self::__gnu_cxx::__alloc_traits_value_type;
    pub type __alloc_traits___is_custom_pointer = self::std::__and_;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __alloc_traits_rebind {
        pub _address: u8,
    }
    pub type __alloc_traits_rebind_other = self::__gnu_cxx::__alloc_traits__Base_type;
    impl Default for __alloc_traits {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct div_t {
    pub quot: c_int,
    pub rem: c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ldiv_t {
    pub quot: c_long,
    pub rem: c_long,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct lldiv_t {
    pub quot: c_longlong,
    pub rem: c_longlong,
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const c_char) -> c_int;
}
extern "C" {
    pub fn atol(__nptr: *const c_char) -> c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const c_char) -> c_longlong;
}
extern "C" {
    pub fn strtod(__nptr: *const c_char, __endptr: *mut *mut c_char) -> f64;
}
extern "C" {
    pub fn strtof(__nptr: *const c_char, __endptr: *mut *mut c_char) -> f32;
}
extern "C" {
    pub fn strtold(__nptr: *const c_char, __endptr: *mut *mut c_char) -> f64;
}
extern "C" {
    pub fn strtof32(__nptr: *const c_char, __endptr: *mut *mut c_char) -> self::_Float32;
}
extern "C" {
    pub fn strtof64(__nptr: *const c_char, __endptr: *mut *mut c_char) -> self::_Float64;
}
extern "C" {
    pub fn strtof32x(__nptr: *const c_char, __endptr: *mut *mut c_char) -> self::_Float32x;
}
extern "C" {
    pub fn strtof64x(__nptr: *const c_char, __endptr: *mut *mut c_char) -> self::_Float64x;
}
extern "C" {
    pub fn strtol(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int) -> c_long;
}
extern "C" {
    pub fn strtoul(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int) -> c_ulong;
}
extern "C" {
    pub fn strtoq(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn strtouq(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int)
        -> c_ulonglong;
}
extern "C" {
    pub fn strtoll(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
    ) -> c_ulonglong;
}
extern "C" {
    pub fn strfromd(__dest: *mut c_char, __size: usize, __format: *const c_char, __f: f64)
        -> c_int;
}
extern "C" {
    pub fn strfromf(__dest: *mut c_char, __size: usize, __format: *const c_char, __f: f32)
        -> c_int;
}
extern "C" {
    pub fn strfroml(__dest: *mut c_char, __size: usize, __format: *const c_char, __f: f64)
        -> c_int;
}
extern "C" {
    pub fn strfromf32(
        __dest: *mut c_char,
        __size: usize,
        __format: *const c_char,
        __f: self::_Float32,
    ) -> c_int;
}
extern "C" {
    pub fn strfromf64(
        __dest: *mut c_char,
        __size: usize,
        __format: *const c_char,
        __f: self::_Float64,
    ) -> c_int;
}
extern "C" {
    pub fn strfromf32x(
        __dest: *mut c_char,
        __size: usize,
        __format: *const c_char,
        __f: self::_Float32x,
    ) -> c_int;
}
extern "C" {
    pub fn strfromf64x(
        __dest: *mut c_char,
        __size: usize,
        __format: *const c_char,
        __f: self::_Float64x,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __locale_struct {
    pub __locales: [*mut self::__locale_data; 13usize],
    pub __ctype_b: *const c_ushort,
    pub __ctype_tolower: *const c_int,
    pub __ctype_toupper: *const c_int,
    pub __names: [*const c_char; 13usize],
}
impl Default for __locale_struct {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type __locale_t = *mut self::__locale_struct;
pub type locale_t = self::__locale_t;
extern "C" {
    pub fn strtol_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
        __loc: self::locale_t,
    ) -> c_long;
}
extern "C" {
    pub fn strtoul_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
        __loc: self::locale_t,
    ) -> c_ulong;
}
extern "C" {
    pub fn strtoll_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
        __loc: self::locale_t,
    ) -> c_longlong;
}
extern "C" {
    pub fn strtoull_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
        __loc: self::locale_t,
    ) -> c_ulonglong;
}
extern "C" {
    pub fn strtod_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __loc: self::locale_t,
    ) -> f64;
}
extern "C" {
    pub fn strtof_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __loc: self::locale_t,
    ) -> f32;
}
extern "C" {
    pub fn strtold_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __loc: self::locale_t,
    ) -> f64;
}
extern "C" {
    pub fn strtof32_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __loc: self::locale_t,
    ) -> self::_Float32;
}
extern "C" {
    pub fn strtof64_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __loc: self::locale_t,
    ) -> self::_Float64;
}
extern "C" {
    pub fn strtof32x_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __loc: self::locale_t,
    ) -> self::_Float32x;
}
extern "C" {
    pub fn strtof64x_l(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __loc: self::locale_t,
    ) -> self::_Float64x;
}
extern "C" {
    pub fn l64a(__n: c_long) -> *mut c_char;
}
extern "C" {
    pub fn a64l(__s: *const c_char) -> c_long;
}
pub type __u_char = c_uchar;
pub type __u_short = c_ushort;
pub type __u_int = c_uint;
pub type __u_long = c_ulong;
pub type __int8_t = c_schar;
pub type __uint8_t = c_uchar;
pub type __int16_t = c_short;
pub type __uint16_t = c_ushort;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type __int64_t = c_long;
pub type __uint64_t = c_ulong;
pub type __int_least8_t = self::__int8_t;
pub type __uint_least8_t = self::__uint8_t;
pub type __int_least16_t = self::__int16_t;
pub type __uint_least16_t = self::__uint16_t;
pub type __int_least32_t = self::__int32_t;
pub type __uint_least32_t = self::__uint32_t;
pub type __int_least64_t = self::__int64_t;
pub type __uint_least64_t = self::__uint64_t;
pub type __quad_t = c_long;
pub type __u_quad_t = c_ulong;
pub type __intmax_t = c_long;
pub type __uintmax_t = c_ulong;
pub type __dev_t = c_ulong;
pub type __uid_t = c_uint;
pub type __gid_t = c_uint;
pub type __ino_t = c_ulong;
pub type __ino64_t = c_ulong;
pub type __mode_t = c_uint;
pub type __nlink_t = c_ulong;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type __pid_t = c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __fsid_t {
    pub __val: [c_int; 2usize],
}
pub type __clock_t = c_long;
pub type __rlim_t = c_ulong;
pub type __rlim64_t = c_ulong;
pub type __id_t = c_uint;
pub type __time_t = c_long;
pub type __useconds_t = c_uint;
pub type __suseconds_t = c_long;
pub type __daddr_t = c_int;
pub type __key_t = c_int;
pub type __clockid_t = c_int;
pub type __timer_t = *mut c_void;
pub type __blksize_t = c_long;
pub type __blkcnt_t = c_long;
pub type __blkcnt64_t = c_long;
pub type __fsblkcnt_t = c_ulong;
pub type __fsblkcnt64_t = c_ulong;
pub type __fsfilcnt_t = c_ulong;
pub type __fsfilcnt64_t = c_ulong;
pub type __fsword_t = c_long;
pub type __ssize_t = c_long;
pub type __syscall_slong_t = c_long;
pub type __syscall_ulong_t = c_ulong;
pub type __loff_t = self::__off64_t;
pub type __caddr_t = *mut c_char;
pub type __intptr_t = c_long;
pub type __socklen_t = c_uint;
pub type __sig_atomic_t = c_int;
pub type u_char = self::__u_char;
pub type u_short = self::__u_short;
pub type u_int = self::__u_int;
pub type u_long = self::__u_long;
pub type quad_t = self::__quad_t;
pub type u_quad_t = self::__u_quad_t;
pub type fsid_t = self::__fsid_t;
pub type loff_t = self::__loff_t;
pub type ino_t = self::__ino_t;
pub type ino64_t = self::__ino64_t;
pub type dev_t = self::__dev_t;
pub type gid_t = self::__gid_t;
pub type mode_t = self::__mode_t;
pub type nlink_t = self::__nlink_t;
pub type uid_t = self::__uid_t;
pub type off_t = self::__off_t;
pub type off64_t = self::__off64_t;
pub type pid_t = self::__pid_t;
pub type id_t = self::__id_t;
pub type daddr_t = self::__daddr_t;
pub type caddr_t = self::__caddr_t;
pub type key_t = self::__key_t;
pub type clock_t = self::__clock_t;
pub type clockid_t = self::__clockid_t;
pub type time_t = self::__time_t;
pub type timer_t = self::__timer_t;
pub type useconds_t = self::__useconds_t;
pub type suseconds_t = self::__suseconds_t;
pub type ulong = c_ulong;
pub type ushort = c_ushort;
pub type uint = c_uint;
pub type u_int8_t = c_uchar;
pub type u_int16_t = c_ushort;
pub type u_int32_t = c_uint;
pub type u_int64_t = c_ulong;
pub type register_t = c_long;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16usize],
}
pub type sigset_t = self::__sigset_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct timeval {
    pub tv_sec: self::__time_t,
    pub tv_usec: self::__suseconds_t,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct timespec {
    pub tv_sec: self::__time_t,
    pub tv_nsec: self::__syscall_slong_t,
}
pub type __fd_mask = c_long;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct fd_set {
    pub fds_bits: [self::__fd_mask; 16usize],
}
pub type fd_mask = self::__fd_mask;
extern "C" {
    pub fn select(
        __nfds: c_int,
        __readfds: *mut self::fd_set,
        __writefds: *mut self::fd_set,
        __exceptfds: *mut self::fd_set,
        __timeout: *mut self::timeval,
    ) -> c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: c_int,
        __readfds: *mut self::fd_set,
        __writefds: *mut self::fd_set,
        __exceptfds: *mut self::fd_set,
        __timeout: *const self::timespec,
        __sigmask: *const self::__sigset_t,
    ) -> c_int;
}
pub type blksize_t = self::__blksize_t;
pub type blkcnt_t = self::__blkcnt_t;
pub type fsblkcnt_t = self::__fsblkcnt_t;
pub type fsfilcnt_t = self::__fsfilcnt_t;
pub type blkcnt64_t = self::__blkcnt64_t;
pub type fsblkcnt64_t = self::__fsblkcnt64_t;
pub type fsfilcnt64_t = self::__fsfilcnt64_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_internal_list {
    pub __prev: *mut self::__pthread_internal_list,
    pub __next: *mut self::__pthread_internal_list,
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type __pthread_list_t = self::__pthread_internal_list;
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
    pub __list: self::__pthread_list_t,
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: self::__pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: self::__pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [c_uint; 2usize],
    pub __g_size: [c_uint; 2usize],
    pub __g1_orig_size: c_uint,
    pub __wrefs: c_uint,
    pub __g_signals: [c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: c_ulonglong,
    pub __wseq32: self::__pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: c_uint,
    pub __high: c_uint,
}
impl Default for __pthread_cond_s__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for __pthread_cond_s__bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "__pthread_cond_s__bindgen_ty_1 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: c_ulonglong,
    pub __g1_start32: self::__pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: c_uint,
    pub __high: c_uint,
}
impl Default for __pthread_cond_s__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for __pthread_cond_s__bindgen_ty_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "__pthread_cond_s__bindgen_ty_2 {{ union }}")
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for __pthread_cond_s {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write ! ( f , "__pthread_cond_s {{ __bindgen_anon_1: {:?}, __bindgen_anon_2: {:?}, __g_refs: {:?}, __g_size: {:?}, __g1_orig_size: {:?}, __wrefs: {:?}, __g_signals: {:?} }}" , self . __bindgen_anon_1 , self . __bindgen_anon_2 , self . __g_refs , self . __g_size , self . __g1_orig_size , self . __wrefs , self . __g_signals )
    }
}
pub type pthread_t = c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_mutexattr_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_mutexattr_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_condattr_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_condattr_t {{ union }}")
    }
}
pub type pthread_key_t = c_uint;
pub type pthread_once_t = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [c_char; 56usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 7usize],
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_attr_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_attr_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: self::__pthread_mutex_s,
    pub __size: [c_char; 40usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 5usize],
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_mutex_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_mutex_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: self::__pthread_cond_s,
    pub __size: [c_char; 48usize],
    pub __align: c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_cond_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_cond_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: self::__pthread_rwlock_arch_t,
    pub __size: [c_char; 56usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 7usize],
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_rwlock_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_rwlock_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [c_char; 8usize],
    pub __align: c_long,
    _bindgen_union_align: u64,
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_rwlockattr_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_rwlockattr_t {{ union }}")
    }
}
pub type pthread_spinlock_t = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [c_char; 32usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 4usize],
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_barrier_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_barrier_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for pthread_barrierattr_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "pthread_barrierattr_t {{ union }}")
    }
}
extern "C" {
    pub fn random() -> c_long;
}
extern "C" {
    pub fn srandom(__seed: c_uint);
}
extern "C" {
    pub fn initstate(__seed: c_uint, __statebuf: *mut c_char, __statelen: usize) -> *mut c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut c_char) -> *mut c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: c_int,
    pub rand_deg: c_int,
    pub rand_sep: c_int,
    pub end_ptr: *mut i32,
}
impl Default for random_data {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
extern "C" {
    pub fn random_r(__buf: *mut self::random_data, __result: *mut i32) -> c_int;
}
extern "C" {
    pub fn srandom_r(__seed: c_uint, __buf: *mut self::random_data) -> c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: c_uint,
        __statebuf: *mut c_char,
        __statelen: usize,
        __buf: *mut self::random_data,
    ) -> c_int;
}
extern "C" {
    pub fn setstate_r(__statebuf: *mut c_char, __buf: *mut self::random_data) -> c_int;
}
extern "C" {
    pub fn rand() -> c_int;
}
extern "C" {
    pub fn srand(__seed: c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut c_ushort) -> c_long;
}
extern "C" {
    pub fn mrand48() -> c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut c_ushort) -> c_long;
}
extern "C" {
    pub fn srand48(__seedval: c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut c_ushort) -> *mut c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut c_ushort);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct drand48_data {
    pub __x: [c_ushort; 3usize],
    pub __old_x: [c_ushort; 3usize],
    pub __c: c_ushort,
    pub __init: c_ushort,
    pub __a: c_ulonglong,
}
extern "C" {
    pub fn drand48_r(__buffer: *mut self::drand48_data, __result: *mut f64) -> c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut c_ushort,
        __buffer: *mut self::drand48_data,
        __result: *mut f64,
    ) -> c_int;
}
extern "C" {
    pub fn lrand48_r(__buffer: *mut self::drand48_data, __result: *mut c_long) -> c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut c_ushort,
        __buffer: *mut self::drand48_data,
        __result: *mut c_long,
    ) -> c_int;
}
extern "C" {
    pub fn mrand48_r(__buffer: *mut self::drand48_data, __result: *mut c_long) -> c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut c_ushort,
        __buffer: *mut self::drand48_data,
        __result: *mut c_long,
    ) -> c_int;
}
extern "C" {
    pub fn srand48_r(__seedval: c_long, __buffer: *mut self::drand48_data) -> c_int;
}
extern "C" {
    pub fn seed48_r(__seed16v: *mut c_ushort, __buffer: *mut self::drand48_data) -> c_int;
}
extern "C" {
    pub fn lcong48_r(__param: *mut c_ushort, __buffer: *mut self::drand48_data) -> c_int;
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn realloc(__ptr: *mut c_void, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn reallocarray(__ptr: *mut c_void, __nmemb: usize, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn free(__ptr: *mut c_void);
}
extern "C" {
    pub fn alloca(__size: usize) -> *mut c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut c_void;
}
extern "C" {
    pub fn posix_memalign(__memptr: *mut *mut c_void, __alignment: usize, __size: usize) -> c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: Option<unsafe extern "C" fn()>) -> c_int;
}
extern "C" {
    pub fn at_quick_exit(__func: Option<unsafe extern "C" fn()>) -> c_int;
}
extern "C" {
    pub fn on_exit(
        __func: Option<unsafe extern "C" fn(__status: c_int, __arg: *mut c_void)>,
        __arg: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn exit(__status: c_int);
}
extern "C" {
    pub fn quick_exit(__status: c_int);
}
extern "C" {
    pub fn _Exit(__status: c_int);
}
extern "C" {
    pub fn getenv(__name: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn secure_getenv(__name: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn putenv(__string: *mut c_char) -> c_int;
}
extern "C" {
    pub fn setenv(__name: *const c_char, __value: *const c_char, __replace: c_int) -> c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const c_char) -> c_int;
}
extern "C" {
    pub fn clearenv() -> c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut c_char) -> c_int;
}
extern "C" {
    pub fn mkstemp64(__template: *mut c_char) -> c_int;
}
extern "C" {
    pub fn mkstemps(__template: *mut c_char, __suffixlen: c_int) -> c_int;
}
extern "C" {
    pub fn mkstemps64(__template: *mut c_char, __suffixlen: c_int) -> c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn mkostemp(__template: *mut c_char, __flags: c_int) -> c_int;
}
extern "C" {
    pub fn mkostemp64(__template: *mut c_char, __flags: c_int) -> c_int;
}
extern "C" {
    pub fn mkostemps(__template: *mut c_char, __suffixlen: c_int, __flags: c_int) -> c_int;
}
extern "C" {
    pub fn mkostemps64(__template: *mut c_char, __suffixlen: c_int, __flags: c_int) -> c_int;
}
extern "C" {
    pub fn system(__command: *const c_char) -> c_int;
}
extern "C" {
    pub fn canonicalize_file_name(__name: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn realpath(__name: *const c_char, __resolved: *mut c_char) -> *mut c_char;
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void) -> c_int>;
pub type comparison_fn_t = self::__compar_fn_t;
pub type __compar_d_fn_t = Option<
    unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void, arg3: *mut c_void) -> c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const c_void,
        __base: *const c_void,
        __nmemb: usize,
        __size: usize,
        __compar: self::__compar_fn_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn qsort(__base: *mut c_void, __nmemb: usize, __size: usize, __compar: self::__compar_fn_t);
}
extern "C" {
    pub fn qsort_r(
        __base: *mut c_void,
        __nmemb: usize,
        __size: usize,
        __compar: self::__compar_d_fn_t,
        __arg: *mut c_void,
    );
}
extern "C" {
    pub fn abs(__x: c_int) -> c_int;
}
extern "C" {
    pub fn labs(__x: c_long) -> c_long;
}
extern "C" {
    pub fn llabs(__x: c_longlong) -> c_longlong;
}
extern "C" {
    pub fn div(__numer: c_int, __denom: c_int) -> self::div_t;
}
extern "C" {
    pub fn ldiv(__numer: c_long, __denom: c_long) -> self::ldiv_t;
}
extern "C" {
    pub fn lldiv(__numer: c_longlong, __denom: c_longlong) -> self::lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: c_int,
        __decpt: *mut c_int,
        __sign: *mut c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: c_int,
        __decpt: *mut c_int,
        __sign: *mut c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn gcvt(__value: f64, __ndigit: c_int, __buf: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn qecvt(
        __value: f64,
        __ndigit: c_int,
        __decpt: *mut c_int,
        __sign: *mut c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: f64,
        __ndigit: c_int,
        __decpt: *mut c_int,
        __sign: *mut c_int,
    ) -> *mut c_char;
}
extern "C" {
    pub fn qgcvt(__value: f64, __ndigit: c_int, __buf: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: c_int,
        __decpt: *mut c_int,
        __sign: *mut c_int,
        __buf: *mut c_char,
        __len: usize,
    ) -> c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: c_int,
        __decpt: *mut c_int,
        __sign: *mut c_int,
        __buf: *mut c_char,
        __len: usize,
    ) -> c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: f64,
        __ndigit: c_int,
        __decpt: *mut c_int,
        __sign: *mut c_int,
        __buf: *mut c_char,
        __len: usize,
    ) -> c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: f64,
        __ndigit: c_int,
        __decpt: *mut c_int,
        __sign: *mut c_int,
        __buf: *mut c_char,
        __len: usize,
    ) -> c_int;
}
extern "C" {
    pub fn mblen(__s: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn mbtowc(__pwc: *mut u32, __s: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut c_char, __wchar: u32) -> c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut u32, __s: *const c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut c_char, __pwcs: *const u32, __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const c_char) -> c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut c_char,
        __tokens: *const *mut c_char,
        __valuep: *mut *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn posix_openpt(__oflag: c_int) -> c_int;
}
extern "C" {
    pub fn grantpt(__fd: c_int) -> c_int;
}
extern "C" {
    pub fn unlockpt(__fd: c_int) -> c_int;
}
extern "C" {
    pub fn ptsname(__fd: c_int) -> *mut c_char;
}
extern "C" {
    pub fn ptsname_r(__fd: c_int, __buf: *mut c_char, __buflen: usize) -> c_int;
}
extern "C" {
    pub fn getpt() -> c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: c_int) -> c_int;
}
extern "C" {
    pub fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn memmove(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn memccpy(
        __dest: *mut c_void,
        __src: *const c_void,
        __c: c_int,
        __n: usize,
    ) -> *mut c_void;
}
extern "C" {
    pub fn memset(__s: *mut c_void, __c: c_int, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: usize) -> c_int;
}
extern "C" {
    pub fn memchr(__s: *const c_void, __c: c_int, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn rawmemchr(__s: *const c_void, __c: c_int) -> *mut c_void;
}
extern "C" {
    pub fn memrchr(__s: *const c_void, __c: c_int, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn strcpy(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strncpy(__dest: *mut c_char, __src: *const c_char, __n: usize) -> *mut c_char;
}
extern "C" {
    pub fn strcat(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strncat(__dest: *mut c_char, __src: *const c_char, __n: usize) -> *mut c_char;
}
extern "C" {
    pub fn strcmp(__s1: *const c_char, __s2: *const c_char) -> c_int;
}
extern "C" {
    pub fn strncmp(__s1: *const c_char, __s2: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn strcoll(__s1: *const c_char, __s2: *const c_char) -> c_int;
}
extern "C" {
    pub fn strxfrm(__dest: *mut c_char, __src: *const c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn strcoll_l(__s1: *const c_char, __s2: *const c_char, __l: self::locale_t) -> c_int;
}
extern "C" {
    pub fn strxfrm_l(
        __dest: *mut c_char,
        __src: *const c_char,
        __n: usize,
        __l: self::locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strdup(__s: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strndup(__string: *const c_char, __n: usize) -> *mut c_char;
}
extern "C" {
    pub fn strchr(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strrchr(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strchrnul(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strcspn(__s: *const c_char, __reject: *const c_char) -> usize;
}
extern "C" {
    pub fn strspn(__s: *const c_char, __accept: *const c_char) -> usize;
}
extern "C" {
    pub fn strpbrk(__s: *const c_char, __accept: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strstr(__haystack: *const c_char, __needle: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strtok(__s: *mut c_char, __delim: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn __strtok_r(
        __s: *mut c_char,
        __delim: *const c_char,
        __save_ptr: *mut *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strtok_r(
        __s: *mut c_char,
        __delim: *const c_char,
        __save_ptr: *mut *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strcasestr(__haystack: *const c_char, __needle: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn memmem(
        __haystack: *const c_void,
        __haystacklen: usize,
        __needle: *const c_void,
        __needlelen: usize,
    ) -> *mut c_void;
}
extern "C" {
    pub fn __mempcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn mempcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn strlen(__s: *const c_char) -> usize;
}
extern "C" {
    pub fn strnlen(__string: *const c_char, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn strerror(__errnum: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strerror_r(__errnum: c_int, __buf: *mut c_char, __buflen: usize) -> *mut c_char;
}
extern "C" {
    pub fn strerror_l(__errnum: c_int, __l: self::locale_t) -> *mut c_char;
}
extern "C" {
    pub fn bcmp(__s1: *const c_void, __s2: *const c_void, __n: usize) -> c_int;
}
extern "C" {
    pub fn bcopy(__src: *const c_void, __dest: *mut c_void, __n: usize);
}
extern "C" {
    pub fn bzero(__s: *mut c_void, __n: usize);
}
extern "C" {
    pub fn index(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn rindex(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn ffs(__i: c_int) -> c_int;
}
extern "C" {
    pub fn ffsl(__l: c_long) -> c_int;
}
extern "C" {
    pub fn ffsll(__ll: c_longlong) -> c_int;
}
extern "C" {
    pub fn strcasecmp(__s1: *const c_char, __s2: *const c_char) -> c_int;
}
extern "C" {
    pub fn strncasecmp(__s1: *const c_char, __s2: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn strcasecmp_l(__s1: *const c_char, __s2: *const c_char, __loc: self::locale_t) -> c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        __s1: *const c_char,
        __s2: *const c_char,
        __n: usize,
        __loc: self::locale_t,
    ) -> c_int;
}
extern "C" {
    pub fn explicit_bzero(__s: *mut c_void, __n: usize);
}
extern "C" {
    pub fn strsep(__stringp: *mut *mut c_char, __delim: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strsignal(__sig: c_int) -> *mut c_char;
}
extern "C" {
    pub fn __stpcpy(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn stpcpy(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn __stpncpy(__dest: *mut c_char, __src: *const c_char, __n: usize) -> *mut c_char;
}
extern "C" {
    pub fn stpncpy(__dest: *mut c_char, __src: *const c_char, __n: usize) -> *mut c_char;
}
extern "C" {
    pub fn strverscmp(__s1: *const c_char, __s2: *const c_char) -> c_int;
}
extern "C" {
    pub fn strfry(__string: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn memfrob(__s: *mut c_void, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn basename(__filename: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    );
}
extern "C" {
    pub fn __assert_perror_fail(
        __errnum: c_int,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    );
}
extern "C" {
    pub fn __assert(__assertion: *const c_char, __file: *const c_char, __line: c_int);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX {
    pub re: f64,
    pub im: f64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKPOLAR {
    pub modulus: f64,
    pub phase: f64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct t_CKVECTOR {
    pub N: c_ulong,
    pub values: *mut f64,
}
impl Default for t_CKVECTOR {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type c_str = *mut c_char;
pub type c_constr = *const c_char;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX_SAMPLE {
    pub re: f64,
    pub im: f64,
}
pub type U_boolList = *mut self::U_boolList_;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct U_boolList_ {
    pub head: c_ulong,
    pub tail: self::U_boolList,
}
impl Default for U_boolList_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
extern "C" {
    pub fn checked_malloc(size: c_int) -> *mut c_void;
}
extern "C" {
    pub fn cc_str(arg1: *mut c_char) -> self::c_str;
}
extern "C" {
    pub fn U_BoolList(head: c_ulong, tail: self::U_boolList) -> self::U_boolList;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S_Symbol_ {
    _unused: [u8; 0],
}
pub type S_Symbol = *mut self::S_Symbol_;
extern "C" {
    pub fn insert_symbol(arg1: self::c_constr) -> self::S_Symbol;
}
extern "C" {
    pub fn S_name(arg1: self::S_Symbol) -> self::c_str;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TAB_table_ {
    _unused: [u8; 0],
}
pub type S_table = *mut self::TAB_table_;
extern "C" {
    pub fn S_empty() -> self::S_table;
}
extern "C" {
    pub fn S_empty2(size: c_uint) -> self::S_table;
}
extern "C" {
    pub fn S_enter(t: self::S_table, sym: self::S_Symbol, value: *mut c_void);
}
extern "C" {
    pub fn S_enter2(t: self::S_table, str: self::c_constr, value: *mut c_void);
}
extern "C" {
    pub fn S_look(t: self::S_table, sym: self::S_Symbol) -> *mut c_void;
}
extern "C" {
    pub fn S_look2(t: self::S_table, str: self::c_constr) -> *mut c_void;
}
extern "C" {
    pub fn S_beginScope(t: self::S_table);
}
extern "C" {
    pub fn S_endScope(t: self::S_table);
}
extern "C" {
    pub fn S_pop(t: self::S_table);
}
pub type a_Pos = c_int;
pub const ae_Operator_ae_op_plus: self::ae_Operator = 0;
pub const ae_Operator_ae_op_minus: self::ae_Operator = 1;
pub const ae_Operator_ae_op_times: self::ae_Operator = 2;
pub const ae_Operator_ae_op_divide: self::ae_Operator = 3;
pub const ae_Operator_ae_op_eq: self::ae_Operator = 4;
pub const ae_Operator_ae_op_neq: self::ae_Operator = 5;
pub const ae_Operator_ae_op_lt: self::ae_Operator = 6;
pub const ae_Operator_ae_op_le: self::ae_Operator = 7;
pub const ae_Operator_ae_op_gt: self::ae_Operator = 8;
pub const ae_Operator_ae_op_ge: self::ae_Operator = 9;
pub const ae_Operator_ae_op_and: self::ae_Operator = 10;
pub const ae_Operator_ae_op_or: self::ae_Operator = 11;
pub const ae_Operator_ae_op_s_or: self::ae_Operator = 12;
pub const ae_Operator_ae_op_s_and: self::ae_Operator = 13;
pub const ae_Operator_ae_op_shift_left: self::ae_Operator = 14;
pub const ae_Operator_ae_op_shift_right: self::ae_Operator = 15;
pub const ae_Operator_ae_op_percent: self::ae_Operator = 16;
pub const ae_Operator_ae_op_s_xor: self::ae_Operator = 17;
pub const ae_Operator_ae_op_chuck: self::ae_Operator = 18;
pub const ae_Operator_ae_op_plus_chuck: self::ae_Operator = 19;
pub const ae_Operator_ae_op_minus_chuck: self::ae_Operator = 20;
pub const ae_Operator_ae_op_times_chuck: self::ae_Operator = 21;
pub const ae_Operator_ae_op_divide_chuck: self::ae_Operator = 22;
pub const ae_Operator_ae_op_s_and_chuck: self::ae_Operator = 23;
pub const ae_Operator_ae_op_s_or_chuck: self::ae_Operator = 24;
pub const ae_Operator_ae_op_s_xor_chuck: self::ae_Operator = 25;
pub const ae_Operator_ae_op_shift_right_chuck: self::ae_Operator = 26;
pub const ae_Operator_ae_op_shift_left_chuck: self::ae_Operator = 27;
pub const ae_Operator_ae_op_percent_chuck: self::ae_Operator = 28;
pub const ae_Operator_ae_op_s_chuck: self::ae_Operator = 29;
pub const ae_Operator_ae_op_plusplus: self::ae_Operator = 30;
pub const ae_Operator_ae_op_minusminus: self::ae_Operator = 31;
pub const ae_Operator_ae_op_tilda: self::ae_Operator = 32;
pub const ae_Operator_ae_op_exclamation: self::ae_Operator = 33;
pub const ae_Operator_ae_op_at_chuck: self::ae_Operator = 34;
pub const ae_Operator_ae_op_unchuck: self::ae_Operator = 35;
pub const ae_Operator_ae_op_upchuck: self::ae_Operator = 36;
pub const ae_Operator_ae_op_spork: self::ae_Operator = 37;
pub const ae_Operator_ae_op_typeof: self::ae_Operator = 38;
pub const ae_Operator_ae_op_sizeof: self::ae_Operator = 39;
pub const ae_Operator_ae_op_new: self::ae_Operator = 40;
pub const ae_Operator_ae_op_arrow_left: self::ae_Operator = 41;
pub const ae_Operator_ae_op_arrow_right: self::ae_Operator = 42;
pub type ae_Operator = u32;
extern "C" {
    pub fn op2str(op: self::ae_Operator) -> *const c_char;
}
pub const ae_Keyword_ae_key_this: self::ae_Keyword = 0;
pub const ae_Keyword_ae_key_me: self::ae_Keyword = 1;
pub const ae_Keyword_ae_key_func: self::ae_Keyword = 2;
pub const ae_Keyword_ae_key_public: self::ae_Keyword = 3;
pub const ae_Keyword_ae_key_protected: self::ae_Keyword = 4;
pub const ae_Keyword_ae_key_private: self::ae_Keyword = 5;
pub const ae_Keyword_ae_key_static: self::ae_Keyword = 6;
pub const ae_Keyword_ae_key_instance: self::ae_Keyword = 7;
pub const ae_Keyword_ae_key_abstract: self::ae_Keyword = 8;
pub type ae_Keyword = u32;
pub type a_Program = *mut self::a_Program_;
pub type a_Section = *mut self::a_Section_;
pub type a_Stmt_List = *mut self::a_Stmt_List_;
pub type a_Class_Def = *mut self::a_Class_Def_;
pub type a_Func_Def = *mut self::a_Func_Def_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Code_Segment_ {
    _unused: [u8; 0],
}
pub type a_Code_Segment = *mut self::a_Code_Segment_;
pub type a_Stmt = *mut self::a_Stmt_;
pub type a_Exp = *mut self::a_Exp_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Chuck_ {
    _unused: [u8; 0],
}
pub type a_Exp_Chuck = *mut self::a_Exp_Chuck_;
pub type a_Exp_Binary = *mut self::a_Exp_Binary_;
pub type a_Exp_Cast = *mut self::a_Exp_Cast_;
pub type a_Exp_Unary = *mut self::a_Exp_Unary_;
pub type a_Exp_Postfix = *mut self::a_Exp_Postfix_;
pub type a_Exp_Primary = *mut self::a_Exp_Primary_;
pub type a_Exp_Dur = *mut self::a_Exp_Dur_;
pub type a_Exp_Array = *mut self::a_Exp_Array_;
pub type a_Exp_Func_Call = *mut self::a_Exp_Func_Call_;
pub type a_Exp_Dot_Member = *mut self::a_Exp_Dot_Member_;
pub type a_Exp_If = *mut self::a_Exp_If_;
pub type a_Exp_Decl = *mut self::a_Exp_Decl_;
pub type a_Exp_Hack = *mut self::a_Exp_Hack_;
pub type a_Stmt_Code = *mut self::a_Stmt_Code_;
pub type a_Stmt_If = *mut self::a_Stmt_If_;
pub type a_Stmt_While = *mut self::a_Stmt_While_;
pub type a_Stmt_Until = *mut self::a_Stmt_Until_;
pub type a_Stmt_For = *mut self::a_Stmt_For_;
pub type a_Stmt_Loop = *mut self::a_Stmt_Loop_;
pub type a_Stmt_Switch = *mut self::a_Stmt_Switch_;
pub type a_Stmt_Break = *mut self::a_Stmt_Break_;
pub type a_Stmt_Continue = *mut self::a_Stmt_Continue_;
pub type a_Stmt_Return = *mut self::a_Stmt_Return_;
pub type a_Stmt_Case = *mut self::a_Stmt_Case_;
pub type a_Stmt_GotoLabel = *mut self::a_Stmt_GotoLabel_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Decl_ {
    _unused: [u8; 0],
}
pub type a_Decl = *mut self::a_Decl_;
pub type a_Var_Decl = *mut self::a_Var_Decl_;
pub type a_Var_Decl_List = *mut self::a_Var_Decl_List_;
pub type a_Type_Decl = *mut self::a_Type_Decl_;
pub type a_Arg_List = *mut self::a_Arg_List_;
pub type a_Id_List = *mut self::a_Id_List_;
pub type a_Class_Ext = *mut self::a_Class_Ext_;
pub type a_Class_Body = *mut self::a_Class_Body_;
pub type a_Array_Sub = *mut self::a_Array_Sub_;
pub type a_Complex = *mut self::a_Complex_;
pub type a_Polar = *mut self::a_Polar_;
pub type a_Vec = *mut self::a_Vec_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Type {
    _unused: [u8; 0],
}
pub type t_CKTYPE = *mut self::Chuck_Type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Value {
    _unused: [u8; 0],
}
pub type t_CKVALUE = *mut self::Chuck_Value;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Func {
    _unused: [u8; 0],
}
pub type t_CKFUNC = *mut self::Chuck_Func;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Namespace {
    _unused: [u8; 0],
}
pub type t_CKNSPC = *mut self::Chuck_Namespace;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_Code {
    _unused: [u8; 0],
}
pub type t_CKVMCODE = *mut self::Chuck_VM_Code;
extern "C" {
    pub fn new_program(section: self::a_Section, pos: c_int) -> self::a_Program;
}
extern "C" {
    pub fn prepend_program(
        section: self::a_Section,
        program: self::a_Program,
        pos: c_int,
    ) -> self::a_Program;
}
extern "C" {
    pub fn new_section_stmt(stmt_list: self::a_Stmt_List, pos: c_int) -> self::a_Section;
}
extern "C" {
    pub fn new_section_func_def(func_def: self::a_Func_Def, pos: c_int) -> self::a_Section;
}
extern "C" {
    pub fn new_section_class_def(class_def: self::a_Class_Def, pos: c_int) -> self::a_Section;
}
extern "C" {
    pub fn new_stmt_list(stmt: self::a_Stmt, pos: c_int) -> self::a_Stmt_List;
}
extern "C" {
    pub fn prepend_stmt_list(
        stmt: self::a_Stmt,
        stmt_list: self::a_Stmt_List,
        pos: c_int,
    ) -> self::a_Stmt_List;
}
extern "C" {
    pub fn new_stmt_from_expression(exp: self::a_Exp, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_code(code: self::a_Stmt_List, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_while(cond: self::a_Exp, body: self::a_Stmt, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_do_while(
        cond: self::a_Exp,
        body: self::a_Stmt,
        pos: c_int,
    ) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_until(cond: self::a_Exp, body: self::a_Stmt, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_do_until(
        cond: self::a_Exp,
        body: self::a_Stmt,
        pos: c_int,
    ) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_for(
        c1: self::a_Stmt,
        c2: self::a_Stmt,
        c3: self::a_Exp,
        body: self::a_Stmt,
        pos: c_int,
    ) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_loop(cond: self::a_Exp, body: self::a_Stmt, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_if(
        cond: self::a_Exp,
        if_body: self::a_Stmt,
        else_body: self::a_Stmt,
        pos: c_int,
    ) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_switch(exp: self::a_Exp, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_break(pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_continue(pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_return(exp: self::a_Exp, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_label(xid: self::c_str, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_case(exp: self::a_Exp, pos: c_int) -> self::a_Stmt;
}
extern "C" {
    pub fn prepend_expression(exp: self::a_Exp, list: self::a_Exp, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_binary(
        lhs: self::a_Exp,
        oper: self::ae_Operator,
        rhs: self::a_Exp,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary(oper: self::ae_Operator, exp: self::a_Exp, pos: c_int)
        -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary2(
        oper: self::ae_Operator,
        type_: self::a_Type_Decl,
        array: self::a_Array_Sub,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary3(
        oper: self::ae_Operator,
        code: self::a_Stmt,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_cast(type_: self::a_Type_Decl, exp: self::a_Exp, pos: c_int)
        -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_array(
        base: self::a_Exp,
        indices: self::a_Array_Sub,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_array_lit(exp_list: self::a_Array_Sub, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_func_call(base: self::a_Exp, args: self::a_Exp, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_member_dot(
        base: self::a_Exp,
        member: self::c_str,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_postfix(
        base: self::a_Exp,
        op: self::ae_Operator,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_dur(base: self::a_Exp, unit: self::a_Exp, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_id(xid: self::c_str, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_int(num: c_long, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_uint(num: c_ulong, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_float(num: f64, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_str(str: self::c_str, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_char(chr: self::c_str, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_if(
        cond: self::a_Exp,
        lhs: self::a_Exp,
        rhs: self::a_Exp,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_complex(arg1: self::a_Complex, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_polar(arg1: self::a_Polar, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_vec(arg1: self::a_Vec, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_decl_external(
        type_decl: self::a_Type_Decl,
        var_decl_list: self::a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_decl_global(
        type_decl: self::a_Type_Decl,
        var_decl_list: self::a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_decl(
        type_decl: self::a_Type_Decl,
        var_decl_list: self::a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_hack(exp: self::a_Exp, pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_exp_from_nil(pos: c_int) -> self::a_Exp;
}
extern "C" {
    pub fn new_var_decl_list(var_decl: self::a_Var_Decl, pos: c_int) -> self::a_Var_Decl_List;
}
extern "C" {
    pub fn prepend_var_decl_list(
        var_decl: self::a_Var_Decl,
        list: self::a_Var_Decl_List,
        pos: c_int,
    ) -> self::a_Var_Decl_List;
}
extern "C" {
    pub fn new_var_decl(
        xid: self::c_constr,
        array: self::a_Array_Sub,
        pos: c_int,
    ) -> self::a_Var_Decl;
}
extern "C" {
    pub fn new_type_decl(xid: self::a_Id_List, ref_: c_int, pos: c_int) -> self::a_Type_Decl;
}
extern "C" {
    pub fn add_type_decl_array(
        type_decl: self::a_Type_Decl,
        array: self::a_Array_Sub,
        pos: c_int,
    ) -> self::a_Type_Decl;
}
extern "C" {
    pub fn new_arg_list(
        type_decl: self::a_Type_Decl,
        var_decl: self::a_Var_Decl,
        pos: c_int,
    ) -> self::a_Arg_List;
}
extern "C" {
    pub fn prepend_arg_list(
        type_decl: self::a_Type_Decl,
        var_decl: self::a_Var_Decl,
        arg_list: self::a_Arg_List,
        pos: c_int,
    ) -> self::a_Arg_List;
}
extern "C" {
    pub fn new_array_sub(exp: self::a_Exp, pos: c_int) -> self::a_Array_Sub;
}
extern "C" {
    pub fn prepend_array_sub(
        array: self::a_Array_Sub,
        exp: self::a_Exp,
        pos: c_int,
    ) -> self::a_Array_Sub;
}
extern "C" {
    pub fn new_complex(re: self::a_Exp, pos: c_int) -> self::a_Complex;
}
extern "C" {
    pub fn new_polar(mod_: self::a_Exp, pos: c_int) -> self::a_Polar;
}
extern "C" {
    pub fn new_vec(e: self::a_Exp, pos: c_int) -> self::a_Vec;
}
extern "C" {
    pub fn new_class_def(
        class_decl: self::ae_Keyword,
        xid: self::a_Id_List,
        ext: self::a_Class_Ext,
        body: self::a_Class_Body,
        pos: c_int,
    ) -> self::a_Class_Def;
}
extern "C" {
    pub fn new_class_body(section: self::a_Section, pos: c_int) -> self::a_Class_Body;
}
extern "C" {
    pub fn prepend_class_body(
        section: self::a_Section,
        body: self::a_Class_Body,
        pos: c_int,
    ) -> self::a_Class_Body;
}
extern "C" {
    pub fn new_class_ext(
        extend_id: self::a_Id_List,
        impl_list: self::a_Id_List,
        pos: c_int,
    ) -> self::a_Class_Ext;
}
extern "C" {
    pub fn new_iface_def(
        class_decl: self::ae_Keyword,
        xid: self::a_Id_List,
        ext: self::a_Class_Ext,
        body: self::a_Class_Body,
        pos: c_int,
    ) -> self::a_Class_Def;
}
extern "C" {
    pub fn new_id_list(xid: self::c_constr, pos: c_int) -> self::a_Id_List;
}
extern "C" {
    pub fn prepend_id_list(
        xid: self::c_constr,
        list: self::a_Id_List,
        pos: c_int,
    ) -> self::a_Id_List;
}
extern "C" {
    pub fn clean_exp(exp: self::a_Exp);
}
extern "C" {
    pub fn new_func_def(
        func_decl: self::ae_Keyword,
        static_decl: self::ae_Keyword,
        type_decl: self::a_Type_Decl,
        name: self::c_str,
        arg_list: self::a_Arg_List,
        code: self::a_Stmt,
        pos: c_int,
    ) -> self::a_Func_Def;
}
extern "C" {
    pub fn delete_id_list(x: self::a_Id_List);
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Binary_ {
    pub lhs: self::a_Exp,
    pub op: self::ae_Operator,
    pub rhs: self::a_Exp,
    pub ck_func: self::t_CKFUNC,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Binary_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Cast_ {
    pub type_: self::a_Type_Decl,
    pub exp: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Cast_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Unary_ {
    pub op: self::ae_Operator,
    pub exp: self::a_Exp,
    pub type_: self::a_Type_Decl,
    pub array: self::a_Array_Sub,
    pub code: self::a_Stmt,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Unary_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Postfix_ {
    pub exp: self::a_Exp,
    pub op: self::ae_Operator,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Postfix_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Dur_ {
    pub base: self::a_Exp,
    pub unit: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Dur_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Array_ {
    pub base: self::a_Exp,
    pub indices: self::a_Array_Sub,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Array_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Func_Call_ {
    pub func: self::a_Exp,
    pub args: self::a_Exp,
    pub ret_type: self::t_CKTYPE,
    pub ck_func: self::t_CKFUNC,
    pub ck_vm_code: self::t_CKVMCODE,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Func_Call_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Dot_Member_ {
    pub base: self::a_Exp,
    pub t_base: self::t_CKTYPE,
    pub xid: self::S_Symbol,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Dot_Member_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_If_ {
    pub cond: self::a_Exp,
    pub if_exp: self::a_Exp,
    pub else_exp: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_If_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Decl_ {
    pub type_: self::a_Type_Decl,
    pub var_decl_list: self::a_Var_Decl_List,
    pub num_var_decls: c_int,
    pub is_static: c_int,
    pub is_global: c_int,
    pub ck_type: self::t_CKTYPE,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Decl_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Hack_ {
    pub exp: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Exp_Hack_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Var_Decl_List_ {
    pub var_decl: self::a_Var_Decl,
    pub next: self::a_Var_Decl_List,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Var_Decl_List_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Var_Decl_ {
    pub xid: self::S_Symbol,
    pub var_decl: self::a_Var_Decl,
    pub array: self::a_Array_Sub,
    pub value: self::t_CKVALUE,
    pub addr: *mut c_void,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Var_Decl_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Type_Decl_ {
    pub xid: self::a_Id_List,
    pub array: self::a_Array_Sub,
    pub ref_: c_int,
    pub linepos: c_int,
}
impl Default for a_Type_Decl_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Array_Sub_ {
    pub depth: c_ulong,
    pub exp_list: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Exp,
    pub err_num: c_int,
    pub err_pos: c_int,
}
impl Default for a_Array_Sub_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Arg_List_ {
    pub type_decl: self::a_Type_Decl,
    pub var_decl: self::a_Var_Decl,
    pub type_: self::t_CKTYPE,
    pub next: self::a_Arg_List,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Arg_List_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Complex_ {
    pub re: self::a_Exp,
    pub im: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Complex_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Polar_ {
    pub mod_: self::a_Exp,
    pub phase: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Polar_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Vec_ {
    pub args: self::a_Exp,
    pub numdims: c_int,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
impl Default for a_Vec_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const ae_Exp_Primary_Type_ae_primary_var: self::ae_Exp_Primary_Type = 0;
pub const ae_Exp_Primary_Type_ae_primary_num: self::ae_Exp_Primary_Type = 1;
pub const ae_Exp_Primary_Type_ae_primary_float: self::ae_Exp_Primary_Type = 2;
pub const ae_Exp_Primary_Type_ae_primary_str: self::ae_Exp_Primary_Type = 3;
pub const ae_Exp_Primary_Type_ae_primary_array: self::ae_Exp_Primary_Type = 4;
pub const ae_Exp_Primary_Type_ae_primary_exp: self::ae_Exp_Primary_Type = 5;
pub const ae_Exp_Primary_Type_ae_primary_hack: self::ae_Exp_Primary_Type = 6;
pub const ae_Exp_Primary_Type_ae_primary_complex: self::ae_Exp_Primary_Type = 7;
pub const ae_Exp_Primary_Type_ae_primary_polar: self::ae_Exp_Primary_Type = 8;
pub const ae_Exp_Primary_Type_ae_primary_vec: self::ae_Exp_Primary_Type = 9;
pub const ae_Exp_Primary_Type_ae_primary_char: self::ae_Exp_Primary_Type = 10;
pub const ae_Exp_Primary_Type_ae_primary_nil: self::ae_Exp_Primary_Type = 11;
pub type ae_Exp_Primary_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Exp_Primary_ {
    pub s_type: self::ae_Exp_Primary_Type,
    pub value: self::t_CKVALUE,
    pub __bindgen_anon_1: self::a_Exp_Primary___bindgen_ty_1,
    pub linepos: c_int,
    pub self_: self::a_Exp,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Exp_Primary___bindgen_ty_1 {
    pub var: self::S_Symbol,
    pub num: c_long,
    pub fnum: f64,
    pub str: self::c_str,
    pub chr: self::c_str,
    pub array: self::a_Array_Sub,
    pub exp: self::a_Exp,
    pub complex: self::a_Complex,
    pub polar: self::a_Polar,
    pub vec: self::a_Vec,
    _bindgen_union_align: u64,
}
impl Default for a_Exp_Primary___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for a_Exp_Primary___bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "a_Exp_Primary___bindgen_ty_1 {{ union }}")
    }
}
impl Default for a_Exp_Primary_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for a_Exp_Primary_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write ! ( f , "a_Exp_Primary_ {{ s_type: {:?}, value: {:?}, __bindgen_anon_1: {:?}, linepos: {:?}, self: {:?} }}" , self . s_type , self . value , self . __bindgen_anon_1 , self . linepos , self . self_ )
    }
}
pub const ae_Exp_Type_ae_exp_binary: self::ae_Exp_Type = 0;
pub const ae_Exp_Type_ae_exp_unary: self::ae_Exp_Type = 1;
pub const ae_Exp_Type_ae_exp_cast: self::ae_Exp_Type = 2;
pub const ae_Exp_Type_ae_exp_postfix: self::ae_Exp_Type = 3;
pub const ae_Exp_Type_ae_exp_dur: self::ae_Exp_Type = 4;
pub const ae_Exp_Type_ae_exp_primary: self::ae_Exp_Type = 5;
pub const ae_Exp_Type_ae_exp_array: self::ae_Exp_Type = 6;
pub const ae_Exp_Type_ae_exp_func_call: self::ae_Exp_Type = 7;
pub const ae_Exp_Type_ae_exp_dot_member: self::ae_Exp_Type = 8;
pub const ae_Exp_Type_ae_exp_if: self::ae_Exp_Type = 9;
pub const ae_Exp_Type_ae_exp_decl: self::ae_Exp_Type = 10;
pub type ae_Exp_Type = u32;
pub const ae_Exp_Meta_ae_meta_value: self::ae_Exp_Meta = 0;
pub const ae_Exp_Meta_ae_meta_var: self::ae_Exp_Meta = 1;
pub type ae_Exp_Meta = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Exp_ {
    pub s_type: self::ae_Exp_Type,
    pub s_meta: self::ae_Exp_Meta,
    pub type_: self::t_CKTYPE,
    pub owner: self::t_CKNSPC,
    pub next: self::a_Exp,
    pub group_size: c_ulong,
    pub cast_to: self::t_CKTYPE,
    pub emit_var: c_ulong,
    pub __bindgen_anon_1: self::a_Exp___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Exp___bindgen_ty_1 {
    pub binary: self::a_Exp_Binary_,
    pub unary: self::a_Exp_Unary_,
    pub cast: self::a_Exp_Cast_,
    pub postfix: self::a_Exp_Postfix_,
    pub dur: self::a_Exp_Dur_,
    pub primary: self::a_Exp_Primary_,
    pub array: self::a_Exp_Array_,
    pub func_call: self::a_Exp_Func_Call_,
    pub dot_member: self::a_Exp_Dot_Member_,
    pub exp_if: self::a_Exp_If_,
    pub decl: self::a_Exp_Decl_,
    _bindgen_union_align: [u64; 7usize],
}
impl Default for a_Exp___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for a_Exp___bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "a_Exp___bindgen_ty_1 {{ union }}")
    }
}
impl Default for a_Exp_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for a_Exp_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write ! ( f , "a_Exp_ {{ s_type: {:?}, s_meta: {:?}, type: {:?}, owner: {:?}, next: {:?}, group_size: {:?}, cast_to: {:?}, emit_var: {:?}, __bindgen_anon_1: {:?}, linepos: {:?} }}" , self . s_type , self . s_meta , self . type_ , self . owner , self . next , self . group_size , self . cast_to , self . emit_var , self . __bindgen_anon_1 , self . linepos )
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_While_ {
    pub is_do: c_int,
    pub cond: self::a_Exp,
    pub body: self::a_Stmt,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_While_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Until_ {
    pub is_do: c_int,
    pub cond: self::a_Exp,
    pub body: self::a_Stmt,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_Until_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_For_ {
    pub c1: self::a_Stmt,
    pub c2: self::a_Stmt,
    pub c3: self::a_Exp,
    pub body: self::a_Stmt,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_For_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Loop_ {
    pub cond: self::a_Exp,
    pub body: self::a_Stmt,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_Loop_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Code_ {
    pub stmt_list: self::a_Stmt_List,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_Code_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_If_ {
    pub cond: self::a_Exp,
    pub if_body: self::a_Stmt,
    pub else_body: self::a_Stmt,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_If_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Switch_ {
    pub val: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_Switch_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Break_ {
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_Break_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Continue_ {
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_Continue_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Return_ {
    pub val: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_Return_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Case_ {
    pub exp: self::a_Exp,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_Case_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_GotoLabel_ {
    pub name: self::S_Symbol,
    pub linepos: c_int,
    pub self_: self::a_Stmt,
}
impl Default for a_Stmt_GotoLabel_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const ae_Stmt_Type_ae_stmt_exp: self::ae_Stmt_Type = 0;
pub const ae_Stmt_Type_ae_stmt_while: self::ae_Stmt_Type = 1;
pub const ae_Stmt_Type_ae_stmt_until: self::ae_Stmt_Type = 2;
pub const ae_Stmt_Type_ae_stmt_for: self::ae_Stmt_Type = 3;
pub const ae_Stmt_Type_ae_stmt_loop: self::ae_Stmt_Type = 4;
pub const ae_Stmt_Type_ae_stmt_if: self::ae_Stmt_Type = 5;
pub const ae_Stmt_Type_ae_stmt_code: self::ae_Stmt_Type = 6;
pub const ae_Stmt_Type_ae_stmt_switch: self::ae_Stmt_Type = 7;
pub const ae_Stmt_Type_ae_stmt_break: self::ae_Stmt_Type = 8;
pub const ae_Stmt_Type_ae_stmt_continue: self::ae_Stmt_Type = 9;
pub const ae_Stmt_Type_ae_stmt_return: self::ae_Stmt_Type = 10;
pub const ae_Stmt_Type_ae_stmt_case: self::ae_Stmt_Type = 11;
pub const ae_Stmt_Type_ae_stmt_gotolabel: self::ae_Stmt_Type = 12;
pub type ae_Stmt_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Stmt_ {
    pub s_type: self::ae_Stmt_Type,
    pub skip: c_int,
    pub __bindgen_anon_1: self::a_Stmt___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Stmt___bindgen_ty_1 {
    pub stmt_exp: self::a_Exp,
    pub stmt_code: self::a_Stmt_Code_,
    pub stmt_while: self::a_Stmt_While_,
    pub stmt_until: self::a_Stmt_Until_,
    pub stmt_loop: self::a_Stmt_Loop_,
    pub stmt_for: self::a_Stmt_For_,
    pub stmt_if: self::a_Stmt_If_,
    pub stmt_switch: self::a_Stmt_Switch_,
    pub stmt_break: self::a_Stmt_Break_,
    pub stmt_continue: self::a_Stmt_Continue_,
    pub stmt_return: self::a_Stmt_Return_,
    pub stmt_case: self::a_Stmt_Case_,
    pub stmt_gotolabel: self::a_Stmt_GotoLabel_,
    _bindgen_union_align: [u64; 6usize],
}
impl Default for a_Stmt___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for a_Stmt___bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "a_Stmt___bindgen_ty_1 {{ union }}")
    }
}
impl Default for a_Stmt_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for a_Stmt_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    pub stmt: self::a_Stmt,
    pub next: self::a_Stmt_List,
    pub linepos: c_int,
}
impl Default for a_Stmt_List_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Def_ {
    pub decl: self::ae_Keyword,
    pub name: self::a_Id_List,
    pub ext: self::a_Class_Ext,
    pub body: self::a_Class_Body,
    pub type_: self::t_CKTYPE,
    pub iface: c_int,
    pub home: self::t_CKNSPC,
    pub linepos: c_int,
}
impl Default for a_Class_Def_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Ext_ {
    pub extend_id: self::a_Id_List,
    pub impl_list: self::a_Id_List,
    pub linepos: c_int,
}
impl Default for a_Class_Ext_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Body_ {
    pub section: self::a_Section,
    pub next: self::a_Class_Body,
    pub linepos: c_int,
}
impl Default for a_Class_Body_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Id_List_ {
    pub xid: self::S_Symbol,
    pub next: self::a_Id_List,
    pub linepos: c_int,
}
impl Default for a_Id_List_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const ae_Func_Type_ae_func_user: self::ae_Func_Type = 0;
pub const ae_Func_Type_ae_func_builtin: self::ae_Func_Type = 1;
pub type ae_Func_Type = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Func_Def_ {
    pub func_decl: self::ae_Keyword,
    pub static_decl: self::ae_Keyword,
    pub type_decl: self::a_Type_Decl,
    pub ret_type: self::t_CKTYPE,
    pub name: self::S_Symbol,
    pub arg_list: self::a_Arg_List,
    pub code: self::a_Stmt,
    pub ck_func: self::t_CKFUNC,
    pub global: c_uint,
    pub s_type: c_uint,
    pub stack_depth: c_uint,
    pub dl_func_ptr: *mut c_void,
    pub linepos: c_int,
}
impl Default for a_Func_Def_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const ae_Section_Type_ae_section_stmt: self::ae_Section_Type = 0;
pub const ae_Section_Type_ae_section_func: self::ae_Section_Type = 1;
pub const ae_Section_Type_ae_section_class: self::ae_Section_Type = 2;
pub type ae_Section_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Section_ {
    pub s_type: self::ae_Section_Type,
    pub __bindgen_anon_1: self::a_Section___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Section___bindgen_ty_1 {
    pub stmt_list: self::a_Stmt_List,
    pub class_def: self::a_Class_Def,
    pub func_def: self::a_Func_Def,
    _bindgen_union_align: u64,
}
impl Default for a_Section___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for a_Section___bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "a_Section___bindgen_ty_1 {{ union }}")
    }
}
impl Default for a_Section_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for a_Section_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    pub section: self::a_Section,
    pub next: self::a_Program,
    pub linepos: c_int,
}
impl Default for a_Program_ {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type __gnuc_va_list = self::__builtin_va_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: self::__mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4usize],
    _bindgen_union_align: u32,
}
impl Default for __mbstate_t__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for __mbstate_t__bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "__mbstate_t__bindgen_ty_1 {{ union }}")
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for __mbstate_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "__mbstate_t {{ __count: {:?}, __value: {:?} }}",
            self.__count, self.__value
        )
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: self::__off_t,
    pub __state: self::__mbstate_t,
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for _G_fpos_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "_G_fpos_t {{ __pos: {:?}, __state: {:?} }}",
            self.__pos, self.__state
        )
    }
}
pub type __fpos_t = self::_G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: self::__off64_t,
    pub __state: self::__mbstate_t,
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for _G_fpos64_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "_G_fpos64_t {{ __pos: {:?}, __state: {:?} }}",
            self.__pos, self.__state
        )
    }
}
pub type __fpos64_t = self::_G_fpos64_t;
pub type __FILE = self::_IO_FILE;
pub type FILE = self::_IO_FILE;
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
    pub _markers: *mut self::_IO_marker,
    pub _chain: *mut self::_IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: self::__off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1usize],
    pub _lock: *mut self::_IO_lock_t,
    pub _offset: self::__off64_t,
    pub _codecvt: *mut self::_IO_codecvt,
    pub _wide_data: *mut self::_IO_wide_data,
    pub _freeres_list: *mut self::_IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub __pad5: usize,
    pub _mode: c_int,
    pub _unused2: [c_char; 20usize],
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type cookie_read_function_t = Option<
    unsafe extern "C" fn(
        __cookie: *mut c_void,
        __buf: *mut c_char,
        __nbytes: usize,
    ) -> self::__ssize_t,
>;
pub type cookie_write_function_t = Option<
    unsafe extern "C" fn(
        __cookie: *mut c_void,
        __buf: *const c_char,
        __nbytes: usize,
    ) -> self::__ssize_t,
>;
pub type cookie_seek_function_t = Option<
    unsafe extern "C" fn(__cookie: *mut c_void, __pos: *mut self::__off64_t, __w: c_int) -> c_int,
>;
pub type cookie_close_function_t = Option<unsafe extern "C" fn(__cookie: *mut c_void) -> c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _IO_cookie_io_functions_t {
    pub read: self::cookie_read_function_t,
    pub write: self::cookie_write_function_t,
    pub seek: self::cookie_seek_function_t,
    pub close: self::cookie_close_function_t,
}
impl Default for _IO_cookie_io_functions_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub type cookie_io_functions_t = self::_IO_cookie_io_functions_t;
pub type va_list = self::__gnuc_va_list;
pub type fpos_t = self::__fpos_t;
pub type fpos64_t = self::__fpos64_t;
extern "C" {
    pub static mut stdin: *mut self::FILE;
}
extern "C" {
    pub static mut stdout: *mut self::FILE;
}
extern "C" {
    pub static mut stderr: *mut self::FILE;
}
extern "C" {
    pub fn remove(__filename: *const c_char) -> c_int;
}
extern "C" {
    pub fn rename(__old: *const c_char, __new: *const c_char) -> c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: c_int,
        __old: *const c_char,
        __newfd: c_int,
        __new: *const c_char,
    ) -> c_int;
}
extern "C" {
    pub fn renameat2(
        __oldfd: c_int,
        __old: *const c_char,
        __newfd: c_int,
        __new: *const c_char,
        __flags: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut self::FILE;
}
extern "C" {
    pub fn tmpfile64() -> *mut self::FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn tempnam(__dir: *const c_char, __pfx: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fcloseall() -> c_int;
}
extern "C" {
    pub fn fopen(__filename: *const c_char, __modes: *const c_char) -> *mut self::FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const c_char,
        __modes: *const c_char,
        __stream: *mut self::FILE,
    ) -> *mut self::FILE;
}
extern "C" {
    pub fn fopen64(__filename: *const c_char, __modes: *const c_char) -> *mut self::FILE;
}
extern "C" {
    pub fn freopen64(
        __filename: *const c_char,
        __modes: *const c_char,
        __stream: *mut self::FILE,
    ) -> *mut self::FILE;
}
extern "C" {
    pub fn fdopen(__fd: c_int, __modes: *const c_char) -> *mut self::FILE;
}
extern "C" {
    pub fn fopencookie(
        __magic_cookie: *mut c_void,
        __modes: *const c_char,
        __io_funcs: self::cookie_io_functions_t,
    ) -> *mut self::FILE;
}
extern "C" {
    pub fn fmemopen(__s: *mut c_void, __len: usize, __modes: *const c_char) -> *mut self::FILE;
}
extern "C" {
    pub fn open_memstream(__bufloc: *mut *mut c_char, __sizeloc: *mut usize) -> *mut self::FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut self::FILE, __buf: *mut c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut self::FILE,
        __buf: *mut c_char,
        __modes: c_int,
        __n: usize,
    ) -> c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut self::FILE, __buf: *mut c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut self::FILE);
}
extern "C" {
    pub fn fprintf(__stream: *mut self::FILE, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn printf(__format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn sprintf(__s: *mut c_char, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut self::FILE,
        __format: *const c_char,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vprintf(__format: *const c_char, __arg: *mut self::__va_list_tag) -> c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut c_char,
        __format: *const c_char,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn snprintf(__s: *mut c_char, __maxlen: usize, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut c_char,
        __maxlen: usize,
        __format: *const c_char,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vasprintf(
        __ptr: *mut *mut c_char,
        __f: *const c_char,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn __asprintf(__ptr: *mut *mut c_char, __fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn asprintf(__ptr: *mut *mut c_char, __fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vdprintf(__fd: c_int, __fmt: *const c_char, __arg: *mut self::__va_list_tag) -> c_int;
}
extern "C" {
    pub fn dprintf(__fd: c_int, __fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn fscanf(__stream: *mut self::FILE, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn scanf(__format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn sscanf(__s: *const c_char, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut self::FILE,
        __format: *const c_char,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vscanf(__format: *const c_char, __arg: *mut self::__va_list_tag) -> c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const c_char,
        __format: *const c_char,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn getc(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn getchar() -> c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fputc(__c: c_int, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn putc(__c: c_int, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn putchar(__c: c_int) -> c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: c_int, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: c_int, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: c_int) -> c_int;
}
extern "C" {
    pub fn getw(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn putw(__w: c_int, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fgets(__s: *mut c_char, __n: c_int, __stream: *mut self::FILE) -> *mut c_char;
}
extern "C" {
    pub fn fgets_unlocked(__s: *mut c_char, __n: c_int, __stream: *mut self::FILE) -> *mut c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut c_char,
        __n: *mut usize,
        __delimiter: c_int,
        __stream: *mut self::FILE,
    ) -> self::__ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut c_char,
        __n: *mut usize,
        __delimiter: c_int,
        __stream: *mut self::FILE,
    ) -> self::__ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut c_char,
        __n: *mut usize,
        __stream: *mut self::FILE,
    ) -> self::__ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const c_char, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn puts(__s: *const c_char) -> c_int;
}
extern "C" {
    pub fn ungetc(__c: c_int, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fread(__ptr: *mut c_void, __size: usize, __n: usize, __stream: *mut self::FILE)
        -> usize;
}
extern "C" {
    pub fn fwrite(__ptr: *const c_void, __size: usize, __n: usize, __s: *mut self::FILE) -> usize;
}
extern "C" {
    pub fn fputs_unlocked(__s: *const c_char, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut c_void,
        __size: usize,
        __n: usize,
        __stream: *mut self::FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const c_void,
        __size: usize,
        __n: usize,
        __stream: *mut self::FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(__stream: *mut self::FILE, __off: c_long, __whence: c_int) -> c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut self::FILE) -> c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut self::FILE);
}
extern "C" {
    pub fn fseeko(__stream: *mut self::FILE, __off: self::__off_t, __whence: c_int) -> c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut self::FILE) -> self::__off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut self::FILE, __pos: *mut self::fpos_t) -> c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut self::FILE, __pos: *const self::fpos_t) -> c_int;
}
extern "C" {
    pub fn fseeko64(__stream: *mut self::FILE, __off: self::__off64_t, __whence: c_int) -> c_int;
}
extern "C" {
    pub fn ftello64(__stream: *mut self::FILE) -> self::__off64_t;
}
extern "C" {
    pub fn fgetpos64(__stream: *mut self::FILE, __pos: *mut self::fpos64_t) -> c_int;
}
extern "C" {
    pub fn fsetpos64(__stream: *mut self::FILE, __pos: *const self::fpos64_t) -> c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut self::FILE);
}
extern "C" {
    pub fn feof(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut self::FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn perror(__s: *const c_char);
}
extern "C" {
    pub static mut sys_nerr: c_int;
}
extern "C" {
    pub static mut sys_errlist: [*const c_char; 0usize];
}
extern "C" {
    pub static mut _sys_nerr: c_int;
}
extern "C" {
    pub static mut _sys_errlist: [*const c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn popen(__command: *const c_char, __modes: *const c_char) -> *mut self::FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn cuserid(__s: *mut c_char) -> *mut c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct obstack {
    _unused: [u8; 0],
}
extern "C" {
    pub fn obstack_printf(__obstack: *mut self::obstack, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn obstack_vprintf(
        __obstack: *mut self::obstack,
        __format: *const c_char,
        __args: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn flockfile(__stream: *mut self::FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut self::FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut self::FILE, arg2: c_int) -> c_int;
}
pub mod __gnu_debug {}
pub type wint_t = c_uint;
pub type mbstate_t = self::__mbstate_t;
extern "C" {
    pub fn wcscpy(__dest: *mut u32, __src: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcsncpy(__dest: *mut u32, __src: *const u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn wcscat(__dest: *mut u32, __src: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcsncat(__dest: *mut u32, __src: *const u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn wcscmp(__s1: *const u32, __s2: *const u32) -> c_int;
}
extern "C" {
    pub fn wcsncmp(__s1: *const u32, __s2: *const u32, __n: usize) -> c_int;
}
extern "C" {
    pub fn wcscasecmp(__s1: *const u32, __s2: *const u32) -> c_int;
}
extern "C" {
    pub fn wcsncasecmp(__s1: *const u32, __s2: *const u32, __n: usize) -> c_int;
}
extern "C" {
    pub fn wcscasecmp_l(__s1: *const u32, __s2: *const u32, __loc: self::locale_t) -> c_int;
}
extern "C" {
    pub fn wcsncasecmp_l(
        __s1: *const u32,
        __s2: *const u32,
        __n: usize,
        __loc: self::locale_t,
    ) -> c_int;
}
extern "C" {
    pub fn wcscoll(__s1: *const u32, __s2: *const u32) -> c_int;
}
extern "C" {
    pub fn wcsxfrm(__s1: *mut u32, __s2: *const u32, __n: usize) -> usize;
}
extern "C" {
    pub fn wcscoll_l(__s1: *const u32, __s2: *const u32, __loc: self::locale_t) -> c_int;
}
extern "C" {
    pub fn wcsxfrm_l(__s1: *mut u32, __s2: *const u32, __n: usize, __loc: self::locale_t) -> usize;
}
extern "C" {
    pub fn wcsdup(__s: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcschr(__wcs: *const u32, __wc: u32) -> *mut u32;
}
extern "C" {
    pub fn wcsrchr(__wcs: *const u32, __wc: u32) -> *mut u32;
}
extern "C" {
    pub fn wcschrnul(__s: *const u32, __wc: u32) -> *mut u32;
}
extern "C" {
    pub fn wcscspn(__wcs: *const u32, __reject: *const u32) -> usize;
}
extern "C" {
    pub fn wcsspn(__wcs: *const u32, __accept: *const u32) -> usize;
}
extern "C" {
    pub fn wcspbrk(__wcs: *const u32, __accept: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcsstr(__haystack: *const u32, __needle: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcstok(__s: *mut u32, __delim: *const u32, __ptr: *mut *mut u32) -> *mut u32;
}
extern "C" {
    pub fn wcslen(__s: *const u32) -> usize;
}
extern "C" {
    pub fn wcswcs(__haystack: *const u32, __needle: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcsnlen(__s: *const u32, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn wmemchr(__s: *const u32, __c: u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn wmemcmp(__s1: *const u32, __s2: *const u32, __n: usize) -> c_int;
}
extern "C" {
    pub fn wmemcpy(__s1: *mut u32, __s2: *const u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn wmemmove(__s1: *mut u32, __s2: *const u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn wmemset(__s: *mut u32, __c: u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn wmempcpy(__s1: *mut u32, __s2: *const u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn btowc(__c: c_int) -> self::wint_t;
}
extern "C" {
    pub fn wctob(__c: self::wint_t) -> c_int;
}
extern "C" {
    pub fn mbsinit(__ps: *const self::mbstate_t) -> c_int;
}
extern "C" {
    pub fn mbrtowc(
        __pwc: *mut u32,
        __s: *const c_char,
        __n: usize,
        __p: *mut self::mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcrtomb(__s: *mut c_char, __wc: u32, __ps: *mut self::mbstate_t) -> usize;
}
extern "C" {
    pub fn __mbrlen(__s: *const c_char, __n: usize, __ps: *mut self::mbstate_t) -> usize;
}
extern "C" {
    pub fn mbrlen(__s: *const c_char, __n: usize, __ps: *mut self::mbstate_t) -> usize;
}
extern "C" {
    pub fn __btowc_alias(__c: c_int) -> self::wint_t;
}
extern "C" {
    pub fn __wctob_alias(__c: self::wint_t) -> c_int;
}
extern "C" {
    pub fn mbsrtowcs(
        __dst: *mut u32,
        __src: *mut *const c_char,
        __len: usize,
        __ps: *mut self::mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsrtombs(
        __dst: *mut c_char,
        __src: *mut *const u32,
        __len: usize,
        __ps: *mut self::mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn mbsnrtowcs(
        __dst: *mut u32,
        __src: *mut *const c_char,
        __nmc: usize,
        __len: usize,
        __ps: *mut self::mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsnrtombs(
        __dst: *mut c_char,
        __src: *mut *const u32,
        __nwc: usize,
        __len: usize,
        __ps: *mut self::mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcwidth(__c: u32) -> c_int;
}
extern "C" {
    pub fn wcswidth(__s: *const u32, __n: usize) -> c_int;
}
extern "C" {
    pub fn wcstod(__nptr: *const u32, __endptr: *mut *mut u32) -> f64;
}
extern "C" {
    pub fn wcstof(__nptr: *const u32, __endptr: *mut *mut u32) -> f32;
}
extern "C" {
    pub fn wcstold(__nptr: *const u32, __endptr: *mut *mut u32) -> f64;
}
extern "C" {
    pub fn wcstof32(__nptr: *const u32, __endptr: *mut *mut u32) -> self::_Float32;
}
extern "C" {
    pub fn wcstof64(__nptr: *const u32, __endptr: *mut *mut u32) -> self::_Float64;
}
extern "C" {
    pub fn wcstof32x(__nptr: *const u32, __endptr: *mut *mut u32) -> self::_Float32x;
}
extern "C" {
    pub fn wcstof64x(__nptr: *const u32, __endptr: *mut *mut u32) -> self::_Float64x;
}
extern "C" {
    pub fn wcstol(__nptr: *const u32, __endptr: *mut *mut u32, __base: c_int) -> c_long;
}
extern "C" {
    pub fn wcstoul(__nptr: *const u32, __endptr: *mut *mut u32, __base: c_int) -> c_ulong;
}
extern "C" {
    pub fn wcstoll(__nptr: *const u32, __endptr: *mut *mut u32, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn wcstoull(__nptr: *const u32, __endptr: *mut *mut u32, __base: c_int) -> c_ulonglong;
}
extern "C" {
    pub fn wcstoq(__nptr: *const u32, __endptr: *mut *mut u32, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn wcstouq(__nptr: *const u32, __endptr: *mut *mut u32, __base: c_int) -> c_ulonglong;
}
extern "C" {
    pub fn wcstol_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: c_int,
        __loc: self::locale_t,
    ) -> c_long;
}
extern "C" {
    pub fn wcstoul_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: c_int,
        __loc: self::locale_t,
    ) -> c_ulong;
}
extern "C" {
    pub fn wcstoll_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: c_int,
        __loc: self::locale_t,
    ) -> c_longlong;
}
extern "C" {
    pub fn wcstoull_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: c_int,
        __loc: self::locale_t,
    ) -> c_ulonglong;
}
extern "C" {
    pub fn wcstod_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: self::locale_t) -> f64;
}
extern "C" {
    pub fn wcstof_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: self::locale_t) -> f32;
}
extern "C" {
    pub fn wcstold_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: self::locale_t) -> f64;
}
extern "C" {
    pub fn wcstof32_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __loc: self::locale_t,
    ) -> self::_Float32;
}
extern "C" {
    pub fn wcstof64_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __loc: self::locale_t,
    ) -> self::_Float64;
}
extern "C" {
    pub fn wcstof32x_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __loc: self::locale_t,
    ) -> self::_Float32x;
}
extern "C" {
    pub fn wcstof64x_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __loc: self::locale_t,
    ) -> self::_Float64x;
}
extern "C" {
    pub fn wcpcpy(__dest: *mut u32, __src: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcpncpy(__dest: *mut u32, __src: *const u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn open_wmemstream(__bufloc: *mut *mut u32, __sizeloc: *mut usize) -> *mut self::__FILE;
}
extern "C" {
    pub fn fwide(__fp: *mut self::__FILE, __mode: c_int) -> c_int;
}
extern "C" {
    pub fn fwprintf(__stream: *mut self::__FILE, __format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn wprintf(__format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn swprintf(__s: *mut u32, __n: usize, __format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn vfwprintf(
        __s: *mut self::__FILE,
        __format: *const u32,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vwprintf(__format: *const u32, __arg: *mut self::__va_list_tag) -> c_int;
}
extern "C" {
    pub fn vswprintf(
        __s: *mut u32,
        __n: usize,
        __format: *const u32,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn fwscanf(__stream: *mut self::__FILE, __format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn wscanf(__format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn swscanf(__s: *const u32, __format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn vfwscanf(
        __s: *mut self::__FILE,
        __format: *const u32,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vwscanf(__format: *const u32, __arg: *mut self::__va_list_tag) -> c_int;
}
extern "C" {
    pub fn vswscanf(
        __s: *const u32,
        __format: *const u32,
        __arg: *mut self::__va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn fgetwc(__stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn getwc(__stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn getwchar() -> self::wint_t;
}
extern "C" {
    pub fn fputwc(__wc: u32, __stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn putwc(__wc: u32, __stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn putwchar(__wc: u32) -> self::wint_t;
}
extern "C" {
    pub fn fgetws(__ws: *mut u32, __n: c_int, __stream: *mut self::__FILE) -> *mut u32;
}
extern "C" {
    pub fn fputws(__ws: *const u32, __stream: *mut self::__FILE) -> c_int;
}
extern "C" {
    pub fn ungetwc(__wc: self::wint_t, __stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn getwc_unlocked(__stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn getwchar_unlocked() -> self::wint_t;
}
extern "C" {
    pub fn fgetwc_unlocked(__stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn fputwc_unlocked(__wc: u32, __stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn putwc_unlocked(__wc: u32, __stream: *mut self::__FILE) -> self::wint_t;
}
extern "C" {
    pub fn putwchar_unlocked(__wc: u32) -> self::wint_t;
}
extern "C" {
    pub fn fgetws_unlocked(__ws: *mut u32, __n: c_int, __stream: *mut self::__FILE) -> *mut u32;
}
extern "C" {
    pub fn fputws_unlocked(__ws: *const u32, __stream: *mut self::__FILE) -> c_int;
}
extern "C" {
    pub fn wcsftime(
        __s: *mut u32,
        __maxsize: usize,
        __format: *const u32,
        __tp: *const self::tm,
    ) -> usize;
}
extern "C" {
    pub fn wcsftime_l(
        __s: *mut u32,
        __maxsize: usize,
        __format: *const u32,
        __tp: *const self::tm,
        __loc: self::locale_t,
    ) -> usize;
}
pub type int_least8_t = self::__int_least8_t;
pub type int_least16_t = self::__int_least16_t;
pub type int_least32_t = self::__int_least32_t;
pub type int_least64_t = self::__int_least64_t;
pub type uint_least8_t = self::__uint_least8_t;
pub type uint_least16_t = self::__uint_least16_t;
pub type uint_least32_t = self::__uint_least32_t;
pub type uint_least64_t = self::__uint_least64_t;
pub type int_fast8_t = c_schar;
pub type int_fast16_t = c_long;
pub type int_fast32_t = c_long;
pub type int_fast64_t = c_long;
pub type uint_fast8_t = c_uchar;
pub type uint_fast16_t = c_ulong;
pub type uint_fast32_t = c_ulong;
pub type uint_fast64_t = c_ulong;
pub type intmax_t = self::__intmax_t;
pub type uintmax_t = self::__uintmax_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct max_align_t {
    pub __max_align_ll: c_longlong,
    pub __bindgen_padding_0: u64,
    pub __max_align_ld: f64,
}
pub mod __cxxabiv1 {
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
            tinfo: *mut self::std::type_info,
            dest: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        ) -> *mut self::__cxxabiv1::__cxa_refcounted_exception;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __class_type_info {
        _unused: [u8; 0],
    }
    #[repr(C)]
    pub struct __forced_unwind__bindgen_vtable(c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __forced_unwind {
        pub vtable_: *const __forced_unwind__bindgen_vtable,
    }
    impl Default for __forced_unwind {
        fn default() -> Self {
            unsafe { zeroed() }
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct lconv {
    pub decimal_point: *mut c_char,
    pub thousands_sep: *mut c_char,
    pub grouping: *mut c_char,
    pub int_curr_symbol: *mut c_char,
    pub currency_symbol: *mut c_char,
    pub mon_decimal_point: *mut c_char,
    pub mon_thousands_sep: *mut c_char,
    pub mon_grouping: *mut c_char,
    pub positive_sign: *mut c_char,
    pub negative_sign: *mut c_char,
    pub int_frac_digits: c_char,
    pub frac_digits: c_char,
    pub p_cs_precedes: c_char,
    pub p_sep_by_space: c_char,
    pub n_cs_precedes: c_char,
    pub n_sep_by_space: c_char,
    pub p_sign_posn: c_char,
    pub n_sign_posn: c_char,
    pub int_p_cs_precedes: c_char,
    pub int_p_sep_by_space: c_char,
    pub int_n_cs_precedes: c_char,
    pub int_n_sep_by_space: c_char,
    pub int_p_sign_posn: c_char,
    pub int_n_sign_posn: c_char,
}
impl Default for lconv {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
extern "C" {
    pub fn setlocale(__category: c_int, __locale: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn localeconv() -> *mut self::lconv;
}
extern "C" {
    pub fn newlocale(
        __category_mask: c_int,
        __locale: *const c_char,
        __base: self::locale_t,
    ) -> self::locale_t;
}
extern "C" {
    pub fn duplocale(__dataset: self::locale_t) -> self::locale_t;
}
extern "C" {
    pub fn freelocale(__dataset: self::locale_t);
}
extern "C" {
    pub fn uselocale(__dataset: self::locale_t) -> self::locale_t;
}
pub const _ISupper: self::_bindgen_ty_39 = 256;
pub const _ISlower: self::_bindgen_ty_39 = 512;
pub const _ISalpha: self::_bindgen_ty_39 = 1024;
pub const _ISdigit: self::_bindgen_ty_39 = 2048;
pub const _ISxdigit: self::_bindgen_ty_39 = 4096;
pub const _ISspace: self::_bindgen_ty_39 = 8192;
pub const _ISprint: self::_bindgen_ty_39 = 16384;
pub const _ISgraph: self::_bindgen_ty_39 = 32768;
pub const _ISblank: self::_bindgen_ty_39 = 1;
pub const _IScntrl: self::_bindgen_ty_39 = 2;
pub const _ISpunct: self::_bindgen_ty_39 = 4;
pub const _ISalnum: self::_bindgen_ty_39 = 8;
pub type _bindgen_ty_39 = u32;
extern "C" {
    pub fn __ctype_b_loc() -> *mut *const c_ushort;
}
extern "C" {
    pub fn __ctype_tolower_loc() -> *mut *const self::__int32_t;
}
extern "C" {
    pub fn __ctype_toupper_loc() -> *mut *const self::__int32_t;
}
extern "C" {
    pub fn isalnum(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isalpha(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn iscntrl(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isdigit(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn islower(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isgraph(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isprint(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn ispunct(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isspace(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isupper(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isxdigit(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn tolower(__c: c_int) -> c_int;
}
extern "C" {
    pub fn toupper(__c: c_int) -> c_int;
}
extern "C" {
    pub fn isblank(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isctype(__c: c_int, __mask: c_int) -> c_int;
}
extern "C" {
    pub fn isascii(__c: c_int) -> c_int;
}
extern "C" {
    pub fn toascii(__c: c_int) -> c_int;
}
extern "C" {
    pub fn _toupper(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn _tolower(arg1: c_int) -> c_int;
}
extern "C" {
    pub fn isalnum_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn isalpha_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn iscntrl_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn isdigit_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn islower_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn isgraph_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn isprint_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn ispunct_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn isspace_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn isupper_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn isxdigit_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn isblank_l(arg1: c_int, arg2: self::locale_t) -> c_int;
}
extern "C" {
    pub fn __tolower_l(__c: c_int, __l: self::locale_t) -> c_int;
}
extern "C" {
    pub fn tolower_l(__c: c_int, __l: self::locale_t) -> c_int;
}
extern "C" {
    pub fn __toupper_l(__c: c_int, __l: self::locale_t) -> c_int;
}
extern "C" {
    pub fn toupper_l(__c: c_int, __l: self::locale_t) -> c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct sched_param {
    pub sched_priority: c_int,
}
extern "C" {
    pub fn clone(
        __fn: Option<unsafe extern "C" fn(__arg: *mut c_void) -> c_int>,
        __child_stack: *mut c_void,
        __flags: c_int,
        __arg: *mut c_void,
        ...
    ) -> c_int;
}
extern "C" {
    pub fn unshare(__flags: c_int) -> c_int;
}
extern "C" {
    pub fn sched_getcpu() -> c_int;
}
extern "C" {
    pub fn setns(__fd: c_int, __nstype: c_int) -> c_int;
}
pub type __cpu_mask = c_ulong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cpu_set_t {
    pub __bits: [self::__cpu_mask; 16usize],
}
extern "C" {
    pub fn __sched_cpucount(__setsize: usize, __setp: *const self::cpu_set_t) -> c_int;
}
extern "C" {
    pub fn __sched_cpualloc(__count: usize) -> *mut self::cpu_set_t;
}
extern "C" {
    pub fn __sched_cpufree(__set: *mut self::cpu_set_t);
}
extern "C" {
    pub fn sched_setparam(__pid: self::__pid_t, __param: *const self::sched_param) -> c_int;
}
extern "C" {
    pub fn sched_getparam(__pid: self::__pid_t, __param: *mut self::sched_param) -> c_int;
}
extern "C" {
    pub fn sched_setscheduler(
        __pid: self::__pid_t,
        __policy: c_int,
        __param: *const self::sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn sched_getscheduler(__pid: self::__pid_t) -> c_int;
}
extern "C" {
    pub fn sched_yield() -> c_int;
}
extern "C" {
    pub fn sched_get_priority_max(__algorithm: c_int) -> c_int;
}
extern "C" {
    pub fn sched_get_priority_min(__algorithm: c_int) -> c_int;
}
extern "C" {
    pub fn sched_rr_get_interval(__pid: self::__pid_t, __t: *mut self::timespec) -> c_int;
}
extern "C" {
    pub fn sched_setaffinity(
        __pid: self::__pid_t,
        __cpusetsize: usize,
        __cpuset: *const self::cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn sched_getaffinity(
        __pid: self::__pid_t,
        __cpusetsize: usize,
        __cpuset: *mut self::cpu_set_t,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct timex {
    pub modes: c_uint,
    pub offset: self::__syscall_slong_t,
    pub freq: self::__syscall_slong_t,
    pub maxerror: self::__syscall_slong_t,
    pub esterror: self::__syscall_slong_t,
    pub status: c_int,
    pub constant: self::__syscall_slong_t,
    pub precision: self::__syscall_slong_t,
    pub tolerance: self::__syscall_slong_t,
    pub time: self::timeval,
    pub tick: self::__syscall_slong_t,
    pub ppsfreq: self::__syscall_slong_t,
    pub jitter: self::__syscall_slong_t,
    pub shift: c_int,
    pub stabil: self::__syscall_slong_t,
    pub jitcnt: self::__syscall_slong_t,
    pub calcnt: self::__syscall_slong_t,
    pub errcnt: self::__syscall_slong_t,
    pub stbcnt: self::__syscall_slong_t,
    pub tai: c_int,
    pub _bitfield_1: self::__BindgenBitfieldUnit<[u8; 44usize], u8>,
}
impl timex {
    #[inline]
    pub fn new_bitfield_1() -> self::__BindgenBitfieldUnit<[u8; 44usize], u8> {
        let mut __bindgen_bitfield_unit: self::__BindgenBitfieldUnit<[u8; 44usize], u8> =
            Default::default();
        __bindgen_bitfield_unit
    }
}
extern "C" {
    pub fn clock_adjtime(__clock_id: self::__clockid_t, __utx: *mut self::timex) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}
impl Default for tm {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct itimerspec {
    pub it_interval: self::timespec,
    pub it_value: self::timespec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent {
    _unused: [u8; 0],
}
extern "C" {
    pub fn clock() -> self::clock_t;
}
extern "C" {
    pub fn time(__timer: *mut self::time_t) -> self::time_t;
}
extern "C" {
    pub fn difftime(__time1: self::time_t, __time0: self::time_t) -> f64;
}
extern "C" {
    pub fn mktime(__tp: *mut self::tm) -> self::time_t;
}
extern "C" {
    pub fn strftime(
        __s: *mut c_char,
        __maxsize: usize,
        __format: *const c_char,
        __tp: *const self::tm,
    ) -> usize;
}
extern "C" {
    pub fn strptime(__s: *const c_char, __fmt: *const c_char, __tp: *mut self::tm) -> *mut c_char;
}
extern "C" {
    pub fn strftime_l(
        __s: *mut c_char,
        __maxsize: usize,
        __format: *const c_char,
        __tp: *const self::tm,
        __loc: self::locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strptime_l(
        __s: *const c_char,
        __fmt: *const c_char,
        __tp: *mut self::tm,
        __loc: self::locale_t,
    ) -> *mut c_char;
}
extern "C" {
    pub fn gmtime(__timer: *const self::time_t) -> *mut self::tm;
}
extern "C" {
    pub fn localtime(__timer: *const self::time_t) -> *mut self::tm;
}
extern "C" {
    pub fn gmtime_r(__timer: *const self::time_t, __tp: *mut self::tm) -> *mut self::tm;
}
extern "C" {
    pub fn localtime_r(__timer: *const self::time_t, __tp: *mut self::tm) -> *mut self::tm;
}
extern "C" {
    pub fn asctime(__tp: *const self::tm) -> *mut c_char;
}
extern "C" {
    pub fn ctime(__timer: *const self::time_t) -> *mut c_char;
}
extern "C" {
    pub fn asctime_r(__tp: *const self::tm, __buf: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn ctime_r(__timer: *const self::time_t, __buf: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub static mut __tzname: [*mut c_char; 2usize];
}
extern "C" {
    pub static mut __daylight: c_int;
}
extern "C" {
    pub static mut __timezone: c_long;
}
extern "C" {
    pub static mut tzname: [*mut c_char; 2usize];
}
extern "C" {
    pub fn tzset();
}
extern "C" {
    pub static mut daylight: c_int;
}
extern "C" {
    pub static mut timezone: c_long;
}
extern "C" {
    pub fn stime(__when: *const self::time_t) -> c_int;
}
extern "C" {
    pub fn timegm(__tp: *mut self::tm) -> self::time_t;
}
extern "C" {
    pub fn timelocal(__tp: *mut self::tm) -> self::time_t;
}
extern "C" {
    pub fn dysize(__year: c_int) -> c_int;
}
extern "C" {
    pub fn nanosleep(
        __requested_time: *const self::timespec,
        __remaining: *mut self::timespec,
    ) -> c_int;
}
extern "C" {
    pub fn clock_getres(__clock_id: self::clockid_t, __res: *mut self::timespec) -> c_int;
}
extern "C" {
    pub fn clock_gettime(__clock_id: self::clockid_t, __tp: *mut self::timespec) -> c_int;
}
extern "C" {
    pub fn clock_settime(__clock_id: self::clockid_t, __tp: *const self::timespec) -> c_int;
}
extern "C" {
    pub fn clock_nanosleep(
        __clock_id: self::clockid_t,
        __flags: c_int,
        __req: *const self::timespec,
        __rem: *mut self::timespec,
    ) -> c_int;
}
extern "C" {
    pub fn clock_getcpuclockid(__pid: self::pid_t, __clock_id: *mut self::clockid_t) -> c_int;
}
extern "C" {
    pub fn timer_create(
        __clock_id: self::clockid_t,
        __evp: *mut self::sigevent,
        __timerid: *mut self::timer_t,
    ) -> c_int;
}
extern "C" {
    pub fn timer_delete(__timerid: self::timer_t) -> c_int;
}
extern "C" {
    pub fn timer_settime(
        __timerid: self::timer_t,
        __flags: c_int,
        __value: *const self::itimerspec,
        __ovalue: *mut self::itimerspec,
    ) -> c_int;
}
extern "C" {
    pub fn timer_gettime(__timerid: self::timer_t, __value: *mut self::itimerspec) -> c_int;
}
extern "C" {
    pub fn timer_getoverrun(__timerid: self::timer_t) -> c_int;
}
extern "C" {
    pub fn timespec_get(__ts: *mut self::timespec, __base: c_int) -> c_int;
}
extern "C" {
    pub static mut getdate_err: c_int;
}
extern "C" {
    pub fn getdate(__string: *const c_char) -> *mut self::tm;
}
extern "C" {
    pub fn getdate_r(__string: *const c_char, __resbufp: *mut self::tm) -> c_int;
}
pub type __jmp_buf = [c_long; 8usize];
pub const PTHREAD_CREATE_JOINABLE: self::_bindgen_ty_40 = 0;
pub const PTHREAD_CREATE_DETACHED: self::_bindgen_ty_40 = 1;
pub type _bindgen_ty_40 = u32;
pub const PTHREAD_MUTEX_TIMED_NP: self::_bindgen_ty_41 = 0;
pub const PTHREAD_MUTEX_RECURSIVE_NP: self::_bindgen_ty_41 = 1;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: self::_bindgen_ty_41 = 2;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: self::_bindgen_ty_41 = 3;
pub const PTHREAD_MUTEX_NORMAL: self::_bindgen_ty_41 = 0;
pub const PTHREAD_MUTEX_RECURSIVE: self::_bindgen_ty_41 = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: self::_bindgen_ty_41 = 2;
pub const PTHREAD_MUTEX_DEFAULT: self::_bindgen_ty_41 = 0;
pub const PTHREAD_MUTEX_FAST_NP: self::_bindgen_ty_41 = 0;
pub type _bindgen_ty_41 = u32;
pub const PTHREAD_MUTEX_STALLED: self::_bindgen_ty_42 = 0;
pub const PTHREAD_MUTEX_STALLED_NP: self::_bindgen_ty_42 = 0;
pub const PTHREAD_MUTEX_ROBUST: self::_bindgen_ty_42 = 1;
pub const PTHREAD_MUTEX_ROBUST_NP: self::_bindgen_ty_42 = 1;
pub type _bindgen_ty_42 = u32;
pub const PTHREAD_PRIO_NONE: self::_bindgen_ty_43 = 0;
pub const PTHREAD_PRIO_INHERIT: self::_bindgen_ty_43 = 1;
pub const PTHREAD_PRIO_PROTECT: self::_bindgen_ty_43 = 2;
pub type _bindgen_ty_43 = u32;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: self::_bindgen_ty_44 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: self::_bindgen_ty_44 = 1;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: self::_bindgen_ty_44 = 2;
pub const PTHREAD_RWLOCK_DEFAULT_NP: self::_bindgen_ty_44 = 0;
pub type _bindgen_ty_44 = u32;
pub const PTHREAD_INHERIT_SCHED: self::_bindgen_ty_45 = 0;
pub const PTHREAD_EXPLICIT_SCHED: self::_bindgen_ty_45 = 1;
pub type _bindgen_ty_45 = u32;
pub const PTHREAD_SCOPE_SYSTEM: self::_bindgen_ty_46 = 0;
pub const PTHREAD_SCOPE_PROCESS: self::_bindgen_ty_46 = 1;
pub type _bindgen_ty_46 = u32;
pub const PTHREAD_PROCESS_PRIVATE: self::_bindgen_ty_47 = 0;
pub const PTHREAD_PROCESS_SHARED: self::_bindgen_ty_47 = 1;
pub type _bindgen_ty_47 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _pthread_cleanup_buffer {
    pub __routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __arg: *mut c_void,
    pub __canceltype: c_int,
    pub __prev: *mut self::_pthread_cleanup_buffer,
}
impl Default for _pthread_cleanup_buffer {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const PTHREAD_CANCEL_ENABLE: self::_bindgen_ty_48 = 0;
pub const PTHREAD_CANCEL_DISABLE: self::_bindgen_ty_48 = 1;
pub type _bindgen_ty_48 = u32;
pub const PTHREAD_CANCEL_DEFERRED: self::_bindgen_ty_49 = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: self::_bindgen_ty_49 = 1;
pub type _bindgen_ty_49 = u32;
extern "C" {
    pub fn pthread_create(
        __newthread: *mut self::pthread_t,
        __attr: *const self::pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>,
        __arg: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_exit(__retval: *mut c_void);
}
extern "C" {
    pub fn pthread_join(__th: self::pthread_t, __thread_return: *mut *mut c_void) -> c_int;
}
extern "C" {
    pub fn pthread_tryjoin_np(__th: self::pthread_t, __thread_return: *mut *mut c_void) -> c_int;
}
extern "C" {
    pub fn pthread_timedjoin_np(
        __th: self::pthread_t,
        __thread_return: *mut *mut c_void,
        __abstime: *const self::timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_detach(__th: self::pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_self() -> self::pthread_t;
}
extern "C" {
    pub fn pthread_equal(__thread1: self::pthread_t, __thread2: self::pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_init(__attr: *mut self::pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_destroy(__attr: *mut self::pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getdetachstate(
        __attr: *const self::pthread_attr_t,
        __detachstate: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setdetachstate(
        __attr: *mut self::pthread_attr_t,
        __detachstate: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getguardsize(
        __attr: *const self::pthread_attr_t,
        __guardsize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setguardsize(
        __attr: *mut self::pthread_attr_t,
        __guardsize: usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getschedparam(
        __attr: *const self::pthread_attr_t,
        __param: *mut self::sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setschedparam(
        __attr: *mut self::pthread_attr_t,
        __param: *const self::sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getschedpolicy(
        __attr: *const self::pthread_attr_t,
        __policy: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setschedpolicy(__attr: *mut self::pthread_attr_t, __policy: c_int)
        -> c_int;
}
extern "C" {
    pub fn pthread_attr_getinheritsched(
        __attr: *const self::pthread_attr_t,
        __inherit: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setinheritsched(
        __attr: *mut self::pthread_attr_t,
        __inherit: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getscope(__attr: *const self::pthread_attr_t, __scope: *mut c_int)
        -> c_int;
}
extern "C" {
    pub fn pthread_attr_setscope(__attr: *mut self::pthread_attr_t, __scope: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstackaddr(
        __attr: *const self::pthread_attr_t,
        __stackaddr: *mut *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstackaddr(
        __attr: *mut self::pthread_attr_t,
        __stackaddr: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstacksize(
        __attr: *const self::pthread_attr_t,
        __stacksize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstacksize(
        __attr: *mut self::pthread_attr_t,
        __stacksize: usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstack(
        __attr: *const self::pthread_attr_t,
        __stackaddr: *mut *mut c_void,
        __stacksize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstack(
        __attr: *mut self::pthread_attr_t,
        __stackaddr: *mut c_void,
        __stacksize: usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setaffinity_np(
        __attr: *mut self::pthread_attr_t,
        __cpusetsize: usize,
        __cpuset: *const self::cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getaffinity_np(
        __attr: *const self::pthread_attr_t,
        __cpusetsize: usize,
        __cpuset: *mut self::cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getattr_default_np(__attr: *mut self::pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_setattr_default_np(__attr: *const self::pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_getattr_np(__th: self::pthread_t, __attr: *mut self::pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_setschedparam(
        __target_thread: self::pthread_t,
        __policy: c_int,
        __param: *const self::sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getschedparam(
        __target_thread: self::pthread_t,
        __policy: *mut c_int,
        __param: *mut self::sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_setschedprio(__target_thread: self::pthread_t, __prio: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_getname_np(
        __target_thread: self::pthread_t,
        __buf: *mut c_char,
        __buflen: usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_setname_np(__target_thread: self::pthread_t, __name: *const c_char) -> c_int;
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
        __th: self::pthread_t,
        __cpusetsize: usize,
        __cpuset: *const self::cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getaffinity_np(
        __th: self::pthread_t,
        __cpusetsize: usize,
        __cpuset: *mut self::cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_once(
        __once_control: *mut self::pthread_once_t,
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
    pub fn pthread_cancel(__th: self::pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_testcancel();
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [self::__pthread_unwind_buf_t__bindgen_ty_1; 1usize],
    pub __pad: [*mut c_void; 4usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_unwind_buf_t__bindgen_ty_1 {
    pub __cancel_jmp_buf: self::__jmp_buf,
    pub __mask_was_saved: c_int,
}
impl Default for __pthread_unwind_buf_t {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __cancel_arg: *mut c_void,
    pub __do_it: c_int,
    pub __cancel_type: c_int,
}
impl Default for __pthread_cleanup_frame {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_cleanup_class {
    pub __cancel_routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __cancel_arg: *mut c_void,
    pub __do_it: c_int,
    pub __cancel_type: c_int,
}
impl Default for __pthread_cleanup_class {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
    _unused: [u8; 0],
}
extern "C" {
    pub fn __sigsetjmp(__env: *mut self::__jmp_buf_tag, __savemask: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_init(
        __mutex: *mut self::pthread_mutex_t,
        __mutexattr: *const self::pthread_mutexattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_destroy(__mutex: *mut self::pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_trylock(__mutex: *mut self::pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_lock(__mutex: *mut self::pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_timedlock(
        __mutex: *mut self::pthread_mutex_t,
        __abstime: *const self::timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_unlock(__mutex: *mut self::pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_getprioceiling(
        __mutex: *const self::pthread_mutex_t,
        __prioceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_setprioceiling(
        __mutex: *mut self::pthread_mutex_t,
        __prioceiling: c_int,
        __old_ceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent(__mutex: *mut self::pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent_np(__mutex: *mut self::pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_init(__attr: *mut self::pthread_mutexattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_destroy(__attr: *mut self::pthread_mutexattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getpshared(
        __attr: *const self::pthread_mutexattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setpshared(
        __attr: *mut self::pthread_mutexattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_gettype(
        __attr: *const self::pthread_mutexattr_t,
        __kind: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_settype(
        __attr: *mut self::pthread_mutexattr_t,
        __kind: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprotocol(
        __attr: *const self::pthread_mutexattr_t,
        __protocol: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprotocol(
        __attr: *mut self::pthread_mutexattr_t,
        __protocol: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprioceiling(
        __attr: *const self::pthread_mutexattr_t,
        __prioceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprioceiling(
        __attr: *mut self::pthread_mutexattr_t,
        __prioceiling: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust(
        __attr: *const self::pthread_mutexattr_t,
        __robustness: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust_np(
        __attr: *const self::pthread_mutexattr_t,
        __robustness: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust(
        __attr: *mut self::pthread_mutexattr_t,
        __robustness: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust_np(
        __attr: *mut self::pthread_mutexattr_t,
        __robustness: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_init(
        __rwlock: *mut self::pthread_rwlock_t,
        __attr: *const self::pthread_rwlockattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_destroy(__rwlock: *mut self::pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_rdlock(__rwlock: *mut self::pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_tryrdlock(__rwlock: *mut self::pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedrdlock(
        __rwlock: *mut self::pthread_rwlock_t,
        __abstime: *const self::timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_wrlock(__rwlock: *mut self::pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_trywrlock(__rwlock: *mut self::pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedwrlock(
        __rwlock: *mut self::pthread_rwlock_t,
        __abstime: *const self::timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_unlock(__rwlock: *mut self::pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_init(__attr: *mut self::pthread_rwlockattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_destroy(__attr: *mut self::pthread_rwlockattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getpshared(
        __attr: *const self::pthread_rwlockattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setpshared(
        __attr: *mut self::pthread_rwlockattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getkind_np(
        __attr: *const self::pthread_rwlockattr_t,
        __pref: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setkind_np(
        __attr: *mut self::pthread_rwlockattr_t,
        __pref: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_cond_init(
        __cond: *mut self::pthread_cond_t,
        __cond_attr: *const self::pthread_condattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_cond_destroy(__cond: *mut self::pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_signal(__cond: *mut self::pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_broadcast(__cond: *mut self::pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_wait(
        __cond: *mut self::pthread_cond_t,
        __mutex: *mut self::pthread_mutex_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_cond_timedwait(
        __cond: *mut self::pthread_cond_t,
        __mutex: *mut self::pthread_mutex_t,
        __abstime: *const self::timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_init(__attr: *mut self::pthread_condattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_destroy(__attr: *mut self::pthread_condattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_getpshared(
        __attr: *const self::pthread_condattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_setpshared(
        __attr: *mut self::pthread_condattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_getclock(
        __attr: *const self::pthread_condattr_t,
        __clock_id: *mut self::__clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_setclock(
        __attr: *mut self::pthread_condattr_t,
        __clock_id: self::__clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_spin_init(__lock: *mut self::pthread_spinlock_t, __pshared: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_spin_destroy(__lock: *mut self::pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_lock(__lock: *mut self::pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_trylock(__lock: *mut self::pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_unlock(__lock: *mut self::pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_init(
        __barrier: *mut self::pthread_barrier_t,
        __attr: *const self::pthread_barrierattr_t,
        __count: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_destroy(__barrier: *mut self::pthread_barrier_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_wait(__barrier: *mut self::pthread_barrier_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_init(__attr: *mut self::pthread_barrierattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_destroy(__attr: *mut self::pthread_barrierattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_getpshared(
        __attr: *const self::pthread_barrierattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_setpshared(
        __attr: *mut self::pthread_barrierattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_key_create(
        __key: *mut self::pthread_key_t,
        __destr_function: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_key_delete(__key: self::pthread_key_t) -> c_int;
}
extern "C" {
    pub fn pthread_getspecific(__key: self::pthread_key_t) -> *mut c_void;
}
extern "C" {
    pub fn pthread_setspecific(__key: self::pthread_key_t, __pointer: *const c_void) -> c_int;
}
extern "C" {
    pub fn pthread_getcpuclockid(
        __thread_id: self::pthread_t,
        __clock_id: *mut self::__clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_atfork(
        __prepare: Option<unsafe extern "C" fn()>,
        __parent: Option<unsafe extern "C" fn()>,
        __child: Option<unsafe extern "C" fn()>,
    ) -> c_int;
}
pub type __gthread_t = self::pthread_t;
pub type __gthread_key_t = self::pthread_key_t;
pub type __gthread_once_t = self::pthread_once_t;
pub type __gthread_mutex_t = self::pthread_mutex_t;
pub type __gthread_recursive_mutex_t = self::pthread_mutex_t;
pub type __gthread_cond_t = self::pthread_cond_t;
pub type __gthread_time_t = self::timespec;
pub type _Atomic_word = c_int;
extern "C" {
    pub fn __errno_location() -> *mut c_int;
}
extern "C" {
    pub static mut program_invocation_name: *mut c_char;
}
extern "C" {
    pub static mut program_invocation_short_name: *mut c_char;
}
pub type error_t = c_int;
extern "C" {
    pub static mut g_program: self::a_Program;
}
extern "C" {
    pub fn yyparse() -> c_int;
}
extern "C" {
    pub fn yyrestart(arg1: *mut self::FILE);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yy_buffer_state {
    _unused: [u8; 0],
}
pub type YY_BUFFER_STATE = *mut self::yy_buffer_state;
extern "C" {
    pub fn yy_scan_string(arg1: *const c_char) -> self::YY_BUFFER_STATE;
}
extern "C" {
    pub fn yy_delete_buffer(arg1: self::YY_BUFFER_STATE);
}
extern "C" {
    pub fn open_cat_ck(filename: self::c_str) -> *mut self::FILE;
}
extern "C" {
    pub fn chuck_parse(fname: self::c_constr, fd: *mut self::FILE, code: self::c_constr)
        -> c_ulong;
}
extern "C" {
    pub fn reset_parse();
}
pub const SyntaxType_COMMA: self::SyntaxType = 0;
pub const SyntaxType_SEMICOLON: self::SyntaxType = 1;
pub const SyntaxType_DBLCOLON: self::SyntaxType = 2;
pub const SyntaxType_PAREN: self::SyntaxType = 3;
pub const SyntaxType_DOT: self::SyntaxType = 4;
pub const SyntaxType_CHUCK_OP: self::SyntaxType = 5;
pub const SyntaxType_OPERATOR: self::SyntaxType = 6;
pub const SyntaxType_KEYWORD: self::SyntaxType = 7;
pub const SyntaxType_DEBUG_PRINT: self::SyntaxType = 8;
pub const SyntaxType_SPORK: self::SyntaxType = 9;
pub const SyntaxType_INTEGER: self::SyntaxType = 10;
pub const SyntaxType_FLOATING: self::SyntaxType = 11;
pub const SyntaxType_STRING: self::SyntaxType = 12;
pub const SyntaxType_COMMENT: self::SyntaxType = 13;
pub const SyntaxType_OTHER: self::SyntaxType = 14;
pub const SyntaxType_NUM_SYNTAX_TYPES: self::SyntaxType = 15;
pub type SyntaxType = u32;
#[repr(C)]
pub struct SyntaxToken {
    pub token: self::std::__cxx11::string,
    pub type_: c_ulong,
    pub begin: self::SyntaxToken_size_type,
    pub end: self::SyntaxToken_size_type,
}
impl Default for SyntaxToken {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for SyntaxToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "SyntaxToken {{ token: {:?}, type: {:?}, begin: {:?}, end: {:?} }}",
            self.token, self.type_, self.begin, self.end
        )
    }
}
#[repr(C)]
pub struct SyntaxTokenList {
    pub list: self::std::vector,
    pub howmany: self::SyntaxTokenList_size_type,
}
impl Default for SyntaxTokenList {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for SyntaxTokenList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "SyntaxTokenList {{ list: {:?}, howmany: {:?} }}",
            self.list, self.howmany
        )
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct SyntaxQuery {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}parseLine"]
    pub fn SyntaxQuery_parseLine(
        this: *mut self::SyntaxQuery,
        line: *const self::std::__cxx11::string,
        tokens: *mut self::SyntaxTokenList,
    ) -> c_ulong;
}
impl SyntaxQuery {
    #[inline]
    pub unsafe fn parseLine(
        &mut self,
        line: *const self::std::__cxx11::string,
        tokens: *mut self::SyntaxTokenList,
    ) -> c_ulong {
        SyntaxQuery_parseLine(self, line, tokens)
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __locale_data {
    pub _address: u8,
}
pub type __builtin_va_list = [self::__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}
impl Default for __va_list_tag {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_1 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_2 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_3 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_4 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_5 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_6 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_7 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_8 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_9 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_10 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_11 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_12 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_13 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_14 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_15 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_16 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_17 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_18 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_19 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_20 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_21 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_22 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_23 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_24 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_25 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_26 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_27 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_28 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_29 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_30 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_31 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_32 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_33 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_34 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_35 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_36 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_37 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_38 {
    pub _address: u8,
}
pub type SyntaxTokenList_size_type = usize;
