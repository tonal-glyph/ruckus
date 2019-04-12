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
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}

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
pub const _GLIBCXX_MAP: u32 = 1;
pub const _STL_TREE_H: u32 = 1;
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
pub const _MEMORYFWD_H: u32 = 1;
pub const __cpp_lib_incomplete_container_elements: u32 = 201505;
pub const __cpp_lib_allocator_is_always_equal: u32 = 201411;
pub const _STL_FUNCTION_H: u32 = 1;
pub const __cpp_lib_transparent_operators: u32 = 201510;
pub const _BACKWARD_BINDERS_H: u32 = 1;
pub const _EXT_ALLOC_TRAITS_H: u32 = 1;
pub const _ALLOC_TRAITS_H: u32 = 1;
pub const __cpp_lib_allocator_traits_is_always_equal: u32 = 201411;
pub const _ALIGNED_BUFFER_H: u32 = 1;
pub const _NODE_HANDLE: u32 = 1;
pub const __cpp_lib_node_extract: u32 = 201606;
pub const _GLIBCXX_OPTIONAL: u32 = 1;
pub const _GLIBCXX_UTILITY: u32 = 1;
pub const _STL_RELOPS_H: u32 = 1;
pub const __cpp_lib_tuple_element_t: u32 = 201402;
pub const __cpp_lib_tuples_by_type: u32 = 201304;
pub const __cpp_lib_exchange_function: u32 = 201304;
pub const _GLIBCXX_USE_MAKE_INTEGER_SEQ: u32 = 1;
pub const __cpp_lib_integer_sequence: u32 = 201304;
pub const __cpp_lib_as_const: u32 = 201510;
pub const _GLIBCXX_STDEXCEPT: u32 = 1;
pub const _GLIBCXX_STRING: u32 = 1;
pub const _STRINGFWD_H: u32 = 1;
pub const _CHAR_TRAITS_H: u32 = 1;
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
pub const _GLIBCXX_RANGE_ACCESS_H: u32 = 1;
pub const __cpp_lib_nonmember_container_access: u32 = 201411;
pub const _BASIC_STRING_H: u32 = 1;
pub const _GLIBCXX_ATOMICITY_H: u32 = 1;
pub const _GLIBCXX_GTHREAD_USE_WEAK: u32 = 1;
pub const __GTHREADS: u32 = 1;
pub const __GTHREADS_CXX0X: u32 = 1;
pub const __GTHREAD_HAS_COND: u32 = 1;
pub const __GTHREAD_ONCE_INIT: u32 = 0;
pub const _GLIBCXX_ATOMIC_WORD_H: u32 = 1;
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
pub const _ENABLE_SPECIAL_MEMBERS_H: u32 = 1;
pub const __cpp_lib_optional: u32 = 201603;
pub const __cpp_lib_generic_associative_lookup: u32 = 201304;
pub const _STL_MAP_H: u32 = 1;
pub const _GLIBCXX_TUPLE: u32 = 1;
pub const _GLIBCXX_ARRAY: u32 = 1;
pub const _USES_ALLOCATOR_H: u32 = 1;
pub const _GLIBCXX_INVOKE_H: u32 = 1;
pub const __cpp_lib_apply: u32 = 201603;
pub const __cpp_lib_make_from_tuple: u32 = 201606;
pub const __cpp_lib_map_try_emplace: u32 = 201411;
pub const __cpp_lib_map_insertion: u32 = 201411;
pub const _STL_MULTIMAP_H: u32 = 1;
pub const _GLIBCXX_VECTOR: u32 = 1;
pub const _STL_CONSTRUCT_H: u32 = 1;
pub const _STL_UNINITIALIZED_H: u32 = 1;
pub const _STL_VECTOR_H: u32 = 1;
pub const _STL_BVECTOR_H: u32 = 1;
pub const _VECTOR_TCC: u32 = 1;
pub const _GLIBCXX_QUEUE: u32 = 1;
pub const _GLIBCXX_DEQUE: u32 = 1;
pub const _STL_DEQUE_H: u32 = 1;
pub const _GLIBCXX_DEQUE_BUF_SIZE: u32 = 512;
pub const _DEQUE_TCC: u32 = 1;
pub const _STL_HEAP_H: u32 = 1;
pub const _STL_QUEUE_H: u32 = 1;
pub const _GLIBCXX_FSTREAM: u32 = 1;
pub const _GLIBCXX_ISTREAM: u32 = 1;
pub const _GLIBCXX_IOS: u32 = 1;
pub const _IOS_BASE_H: u32 = 1;
pub const _LOCALE_CLASSES_H: u32 = 1;
pub const _LOCALE_CLASSES_TCC: u32 = 1;
pub const _GLIBCXX_SYSTEM_ERROR: u32 = 1;
pub const _GLIBCXX_ERROR_CONSTANTS: u32 = 1;
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
pub const _GLIBCXX_OSTREAM: u32 = 1;
pub const _OSTREAM_TCC: u32 = 1;
pub const _ISTREAM_TCC: u32 = 1;
pub const _CODECVT_H: u32 = 1;
pub const _GLIBCXX_BASIC_FILE_STDIO_H: u32 = 1;
pub const _GLIBCXX_CXX_IO_H: u32 = 1;
pub const _FSTREAM_TCC: u32 = 1;
pub const _GLIBCXX_SSTREAM: u32 = 1;
pub const _SSTREAM_TCC: u32 = 1;
pub const _DIRENT_H: u32 = 1;
pub const _DIRENT_MATCHES_DIRENT64: u32 = 1;
pub const _BITS_POSIX1_LIM_H: u32 = 1;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_FD_SETSIZE: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX_QLIMIT: u32 = 1;
pub const _POSIX_HIWAT: u32 = 512;
pub const _POSIX_UIO_MAXIOV: u32 = 16;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;
pub const PATH_MAX: u32 = 4096;
pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const PTHREAD_KEYS_MAX: u32 = 1024;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const AIO_PRIO_DELTA_MAX: u32 = 20;
pub const PTHREAD_STACK_MIN: u32 = 16384;
pub const DELAYTIMER_MAX: u32 = 2147483647;
pub const TTY_NAME_MAX: u32 = 32;
pub const LOGIN_NAME_MAX: u32 = 256;
pub const HOST_NAME_MAX: u32 = 64;
pub const MQ_PRIO_MAX: u32 = 32768;
pub const SEM_VALUE_MAX: u32 = 2147483647;
pub const MAXNAMLEN: u32 = 255;
pub const CHUCK_ARRAY4_DATAKIND: u32 = 1;
pub const CHUCK_ARRAY8_DATAKIND: u32 = 2;
pub const CHUCK_ARRAY16_DATAKIND: u32 = 3;
pub const CHUCK_ARRAY24_DATAKIND: u32 = 4;
pub const CHUCK_ARRAY32_DATAKIND: u32 = 5;
pub const CK_DLL_VERSION_MAJOR: u32 = 7;
pub const CK_DLL_VERSION_MINOR: u32 = 0;
pub const CK_QUERY_FUNC: &'static [u8; 9usize] = b"ck_query\0";
pub const CK_DECLVERSION_FUNC: &'static [u8; 11usize] = b"ck_version\0";
pub const CK_INVALID_OFFSET: u32 = 4294967295;
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
pub const CK_LOG_CRAZY: u32 = 10;
pub const CK_LOG_FINEST: u32 = 9;
pub const CK_LOG_FINER: u32 = 8;
pub const CK_LOG_FINE: u32 = 7;
pub const CK_LOG_CONFIG: u32 = 6;
pub const CK_LOG_INFO: u32 = 5;
pub const CK_LOG_WARNING: u32 = 4;
pub const CK_LOG_SEVERE: u32 = 3;
pub const CK_LOG_SYSTEM: u32 = 2;
pub const CK_LOG_CORE: u32 = 1;
pub const CK_LOG_NONE: u32 = 0;
pub const UGEN_OP_PASS: i32 = -1;
pub const UGEN_OP_STOP: u32 = 0;
pub const UGEN_OP_TICK: u32 = 1;
pub const _GLIBCXX_IOSTREAM: u32 = 1;
pub const _GLIBCXX_LIST: u32 = 1;
pub const _STL_LIST_H: u32 = 1;
pub const _ALLOCATED_PTR_H: u32 = 1;
pub const _LIST_TCC: u32 = 1;
pub const CK_DEBUG_MEMORY_MGMT: u32 = 0;
pub const CVM_MEM_STACK_SIZE: u32 = 65536;
pub const CVM_REG_STACK_SIZE: u32 = 16384;
pub const _GLIBCXX_MATH_H: u32 = 1;
pub const _MATH_H: u32 = 1;
pub const _BITS_LIBM_SIMD_DECL_STUBS_H: u32 = 1;
pub const __GLIBC_FLT_EVAL_METHOD: u32 = 0;
pub const __FP_LOGB0_IS_MIN: u32 = 1;
pub const __FP_LOGBNAN_IS_MIN: u32 = 1;
pub const FP_ILOGB0: i32 = -2147483648;
pub const FP_ILOGBNAN: i32 = -2147483648;
pub const __FP_LONG_MAX: u64 = 9223372036854775807;
pub const FP_LLOGB0: i64 = -9223372036854775808;
pub const FP_LLOGBNAN: i64 = -9223372036854775808;
pub const FP_INT_UPWARD: u32 = 0;
pub const FP_INT_DOWNWARD: u32 = 1;
pub const FP_INT_TOWARDZERO: u32 = 2;
pub const FP_INT_TONEARESTFROMZERO: u32 = 3;
pub const FP_INT_TONEAREST: u32 = 4;
pub const __MATH_DECLARING_DOUBLE: u32 = 1;
pub const __MATH_DECLARING_FLOATN: u32 = 0;
pub const __MATH_DECLARE_LDOUBLE: u32 = 1;
pub const FP_NAN: u32 = 0;
pub const FP_INFINITE: u32 = 1;
pub const FP_ZERO: u32 = 2;
pub const FP_SUBNORMAL: u32 = 3;
pub const FP_NORMAL: u32 = 4;
pub const MATH_ERRNO: u32 = 1;
pub const MATH_ERREXCEPT: u32 = 2;
pub const math_errhandling: u32 = 3;
pub const M_E: f64 = 2.718281828459045;
pub const M_LOG2E: f64 = 1.4426950408889634;
pub const M_LOG10E: f64 = 0.4342944819032518;
pub const M_LN2: f64 = 0.6931471805599453;
pub const M_LN10: f64 = 2.302585092994046;
pub const M_PI: f64 = 3.141592653589793;
pub const M_PI_2: f64 = 1.5707963267948966;
pub const M_PI_4: f64 = 0.7853981633974483;
pub const M_1_PI: f64 = 0.3183098861837907;
pub const M_2_PI: f64 = 0.6366197723675814;
pub const M_2_SQRTPI: f64 = 1.1283791670955126;
pub const M_SQRT2: f64 = 1.4142135623730951;
pub const M_SQRT1_2: f64 = 0.7071067811865476;
pub const M_El: f64 = 2.718281828459045;
pub const M_LOG2El: f64 = 1.4426950408889634;
pub const M_LOG10El: f64 = 0.4342944819032518;
pub const M_LN2l: f64 = 0.6931471805599453;
pub const M_LN10l: f64 = 2.302585092994046;
pub const M_PIl: f64 = 3.141592653589793;
pub const M_PI_2l: f64 = 1.5707963267948966;
pub const M_PI_4l: f64 = 0.7853981633974483;
pub const M_1_PIl: f64 = 0.3183098861837907;
pub const M_2_PIl: f64 = 0.6366197723675814;
pub const M_2_SQRTPIl: f64 = 1.1283791670955126;
pub const M_SQRT2l: f64 = 1.4142135623730951;
pub const M_SQRT1_2l: f64 = 0.7071067811865476;
pub const _GLIBCXX_CMATH: u32 = 1;
pub const __cpp_lib_hypot: u32 = 201603;
pub const _GLIBCXX_BITS_SPECFUN_H: u32 = 1;
pub const __STDCPP_MATH_SPEC_FUNCS__: u32 = 201003;
pub const __cpp_lib_math_special_functions: u32 = 201603;
pub const _GLIBCXX_TR1_GAMMA_TCC: u32 = 1;
pub const _GLIBCXX_TR1_SPECIAL_FUNCTION_UTIL_H: u32 = 1;
pub const _GLIBCXX_TR1_BESSEL_FUNCTION_TCC: u32 = 1;
pub const _GLIBCXX_TR1_BETA_FUNCTION_TCC: u32 = 1;
pub const _GLIBCXX_TR1_ELL_INTEGRAL_TCC: u32 = 1;
pub const _GLIBCXX_TR1_EXP_INTEGRAL_TCC: u32 = 1;
pub const _GLIBCXX_TR1_HYPERGEOMETRIC_TCC: u32 = 1;
pub const _GLIBCXX_TR1_LEGENDRE_FUNCTION_TCC: u32 = 1;
pub const _GLIBCXX_TR1_MODIFIED_BESSEL_FUNC_TCC: u32 = 1;
pub const _GLIBCXX_TR1_POLY_HERMITE_TCC: u32 = 1;
pub const _GLIBCXX_TR1_POLY_LAGUERRE_TCC: u32 = 1;
pub const _GLIBCXX_TR1_RIEMANN_ZETA_TCC: u32 = 1;
pub const genX_tableSize: u32 = 4096;
pub const genX_MAX_COEFFS: u32 = 100;
pub const MAX_CURVE_PTS: u32 = 256;
pub mod std {

    pub type nullptr_t = *const ::std::os::raw::c_void;
    pub mod __cxx11 {

        pub type string = basic_string<::std::os::raw::c_char>;
        pub type wstring = basic_string<u32>;
        pub type u16string = basic_string<u16>;
        pub type u32string = basic_string<u32>;
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
            pub _M_dataplus: basic_string__Alloc_hider,
            pub _M_string_length: basic_string_size_type,
            pub __bindgen_anon_1: basic_string__bindgen_ty_2<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
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
        pub type basic_string_const_reverse_iterator =
            reverse_iterator<basic_string_const_iterator>;
        pub type basic_string_reverse_iterator = reverse_iterator<basic_string_iterator>;
        pub type basic_string___const_iterator = basic_string_const_iterator;
        pub type basic_string___sv_type<_CharT> = basic_string_view<_CharT>;
        pub type basic_string__If_sv = enable_if_t;
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct basic_string___sv_wrapper<_CharT> {
            pub _M_sv: basic_string___sv_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        impl<_CharT> Default for basic_string___sv_wrapper<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        pub struct basic_string__Alloc_hider {
            pub _M_p: basic_string_pointer,
        }
        impl Default for basic_string__Alloc_hider {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl ::std::fmt::Debug for basic_string__Alloc_hider {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "basic_string__Alloc_hider {{  }}")
            }
        }
        impl ::std::cmp::PartialEq for basic_string__Alloc_hider {
            fn eq(&self, other: &basic_string__Alloc_hider) -> bool {
                self._M_p == other._M_p
            }
        }
        pub const basic_string__S_local_capacity: basic_string__bindgen_ty_1 =
            basic_string__bindgen_ty_1::_S_local_capacity;
        #[repr(i32)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum basic_string__bindgen_ty_1 {
            _S_local_capacity = 0,
        }
        #[repr(C)]
        pub struct basic_string__bindgen_ty_2<_CharT> {
            pub _M_local_buf: root::__BindgenUnionField<*mut _CharT>,
            pub _M_allocated_capacity: root::__BindgenUnionField<basic_string_size_type>,
            pub bindgen_union_field: u64,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        impl<_CharT> Default for basic_string__bindgen_ty_2<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_string__bindgen_ty_2<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "basic_string__bindgen_ty_2 {{ union }}")
            }
        }
        impl<_CharT> Default for basic_string<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
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
        extern "C" {
            pub fn stoi(
                __str: *const string,
                __idx: *mut usize,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int;
        }
        extern "C" {
            pub fn stol(
                __str: *const string,
                __idx: *mut usize,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_long;
        }
        extern "C" {
            pub fn stoul(
                __str: *const string,
                __idx: *mut usize,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_ulong;
        }
        extern "C" {
            pub fn stoll(
                __str: *const string,
                __idx: *mut usize,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_longlong;
        }
        extern "C" {
            pub fn stoull(
                __str: *const string,
                __idx: *mut usize,
                __base: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_ulonglong;
        }
        extern "C" {
            pub fn stof(__str: *const string, __idx: *mut usize) -> f32;
        }
        extern "C" {
            pub fn stod(__str: *const string, __idx: *mut usize) -> f64;
        }
        extern "C" {
            pub fn stold(__str: *const string, __idx: *mut usize) -> u128;
        }
        extern "C" {
            pub fn to_string(__val: ::std::os::raw::c_int) -> string;
        }
        extern "C" {
            pub fn to_wstring(__val: ::std::os::raw::c_int) -> wstring;
        }
        #[repr(C)]
        #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct collate {
            pub _base: locale_facet,
            pub _M_c_locale_collate: __c_locale,
        }
        pub type collate_char_type<_CharT> = _CharT;
        pub type collate_string_type = basic_string<_CharT>;
        impl Default for collate {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct collate_byname {
            pub _base: collate,
        }
        pub type collate_byname_char_type<_CharT> = _CharT;
        pub type collate_byname_string_type = basic_string<_CharT>;
        impl Default for collate_byname {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct numpunct<_CharT> {
            pub _base: locale_facet,
            pub _M_data: *mut numpunct___cache_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type numpunct_char_type<_CharT> = _CharT;
        pub type numpunct_string_type = basic_string<_CharT>;
        pub type numpunct___cache_type<_CharT> = __numpunct_cache<_CharT>;
        impl<_CharT> Default for numpunct<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        extern "C" {
            pub static mut id: locale_id;
        }
        #[repr(C)]
        #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct numpunct_byname<_CharT> {
            pub _base: numpunct<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type numpunct_byname_char_type<_CharT> = _CharT;
        pub type numpunct_byname_string_type = basic_string<_CharT>;
        impl<_CharT> Default for numpunct_byname<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        pub struct basic_stringbuf<_CharT> {
            pub _base: basic_streambuf<_CharT>,
            pub _M_mode: ios_base_openmode,
            pub _M_string: basic_stringbuf___string_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type basic_stringbuf_char_type<_CharT> = _CharT;
        pub type basic_stringbuf_traits_type<_Traits> = _Traits;
        pub type basic_stringbuf_allocator_type<_Alloc> = _Alloc;
        pub type basic_stringbuf_int_type = [u8; 0usize];
        pub type basic_stringbuf_pos_type = [u8; 0usize];
        pub type basic_stringbuf_off_type = [u8; 0usize];
        pub type basic_stringbuf___streambuf_type<_CharT> =
            basic_streambuf<basic_stringbuf_char_type<_CharT>>;
        pub type basic_stringbuf___string_type<_CharT> =
            basic_string<basic_stringbuf_char_type<_CharT>>;
        pub type basic_stringbuf___size_type<_CharT> = basic_stringbuf___string_type<_CharT>;
        #[repr(C)]
        pub struct basic_stringbuf___xfer_bufptrs<_CharT> {
            pub _M_to: *mut basic_stringbuf<_CharT>,
            pub _M_goff: [basic_stringbuf_off_type; 3usize],
            pub _M_poff: [basic_stringbuf_off_type; 3usize],
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        impl<_CharT> Default for basic_stringbuf___xfer_bufptrs<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_stringbuf___xfer_bufptrs<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write ! ( f , "basic_stringbuf___xfer_bufptrs {{ _M_to: {:?}, _M_goff: {:?}, _M_poff: {:?} }}" , self . _M_to , self . _M_goff , self . _M_poff )
            }
        }
        impl<_CharT> Default for basic_stringbuf<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
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
        pub struct basic_istringstream<_CharT> {
            pub _base: basic_istream<_CharT>,
            pub _M_stringbuf: basic_istringstream___stringbuf_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type basic_istringstream_char_type<_CharT> = _CharT;
        pub type basic_istringstream_traits_type<_Traits> = _Traits;
        pub type basic_istringstream_allocator_type<_Alloc> = _Alloc;
        pub type basic_istringstream_int_type = [u8; 0usize];
        pub type basic_istringstream_pos_type = [u8; 0usize];
        pub type basic_istringstream_off_type = [u8; 0usize];
        pub type basic_istringstream___string_type<_CharT> = basic_string<_CharT>;
        pub type basic_istringstream___stringbuf_type<_CharT> = basic_stringbuf<_CharT>;
        pub type basic_istringstream___istream_type<_CharT> =
            basic_istream<basic_istringstream_char_type<_CharT>>;
        impl<_CharT> Default for basic_istringstream<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_istringstream<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "basic_istringstream {{ _M_stringbuf: {:?} }}",
                    self._M_stringbuf
                )
            }
        }
        #[repr(C)]
        pub struct basic_ostringstream<_CharT> {
            pub _base: basic_ostream<_CharT>,
            pub _M_stringbuf: basic_ostringstream___stringbuf_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type basic_ostringstream_char_type<_CharT> = _CharT;
        pub type basic_ostringstream_traits_type<_Traits> = _Traits;
        pub type basic_ostringstream_allocator_type<_Alloc> = _Alloc;
        pub type basic_ostringstream_int_type = [u8; 0usize];
        pub type basic_ostringstream_pos_type = [u8; 0usize];
        pub type basic_ostringstream_off_type = [u8; 0usize];
        pub type basic_ostringstream___string_type<_CharT> = basic_string<_CharT>;
        pub type basic_ostringstream___stringbuf_type<_CharT> = basic_stringbuf<_CharT>;
        pub type basic_ostringstream___ostream_type<_CharT> =
            basic_ostream<basic_ostringstream_char_type<_CharT>>;
        impl<_CharT> Default for basic_ostringstream<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl<_CharT> ::std::fmt::Debug for basic_ostringstream<_CharT> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "basic_ostringstream {{ _M_stringbuf: {:?} }}",
                    self._M_stringbuf
                )
            }
        }
        #[repr(C)]
        pub struct basic_stringstream<_CharT> {
            pub _base: basic_iostream<_CharT>,
            pub _M_stringbuf: basic_stringstream___stringbuf_type<_CharT>,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type basic_stringstream_char_type<_CharT> = _CharT;
        pub type basic_stringstream_traits_type<_Traits> = _Traits;
        pub type basic_stringstream_allocator_type<_Alloc> = _Alloc;
        pub type basic_stringstream_int_type = [u8; 0usize];
        pub type basic_stringstream_pos_type = [u8; 0usize];
        pub type basic_stringstream_off_type = [u8; 0usize];
        pub type basic_stringstream___string_type<_CharT> = basic_string<_CharT>;
        pub type basic_stringstream___stringbuf_type<_CharT> = basic_stringbuf<_CharT>;
        pub type basic_stringstream___iostream_type<_CharT> =
            basic_iostream<basic_stringstream_char_type<_CharT>>;
        impl<_CharT> Default for basic_stringstream<_CharT> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
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
            pub _M_impl: _List_base__List_impl,
        }
        pub type _List_base__Tp_alloc_type = [u8; 0usize];
        pub type _List_base__Tp_alloc_traits = __alloc_traits;
        pub type _List_base__Node_alloc_type = [u8; 0usize];
        pub type _List_base__Node_alloc_traits = __alloc_traits;
        #[repr(C)]
        pub struct _List_base__List_impl {
            pub _M_node: _List_node_header,
        }
        impl Default for _List_base__List_impl {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl ::std::fmt::Debug for _List_base__List_impl {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "_List_base__List_impl {{ _M_node: {:?} }}", self._M_node)
            }
        }
        impl ::std::cmp::PartialEq for _List_base__List_impl {
            fn eq(&self, other: &_List_base__List_impl) -> bool {
                self._M_node == other._M_node
            }
        }
        pub type _List_base_allocator_type<_Alloc> = _Alloc;
        impl Default for _List_base {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl ::std::fmt::Debug for _List_base {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "_List_base {{ _M_impl: {:?} }}", self._M_impl)
            }
        }
        impl ::std::cmp::PartialEq for _List_base {
            fn eq(&self, other: &_List_base) -> bool {
                self._M_impl == other._M_impl
            }
        }
        #[repr(C)]
        pub struct list {
            pub _base: _List_base,
        }
        pub type list__Base = _List_base;
        pub type list__Tp_alloc_type = list__Base;
        pub type list__Tp_alloc_traits = list__Base;
        pub type list__Node_alloc_type = list__Base;
        pub type list__Node_alloc_traits = list__Base;
        pub type list_value_type<_Tp> = _Tp;
        pub type list_pointer = list__Tp_alloc_traits;
        pub type list_const_pointer = list__Tp_alloc_traits;
        pub type list_reference = list__Tp_alloc_traits;
        pub type list_const_reference = list__Tp_alloc_traits;
        pub type list_iterator = _List_iterator;
        pub type list_const_iterator = _List_const_iterator;
        pub type list_const_reverse_iterator = reverse_iterator<list_const_iterator>;
        pub type list_reverse_iterator = reverse_iterator<list_iterator>;
        pub type list_size_type = usize;
        pub type list_difference_type = isize;
        pub type list_allocator_type<_Alloc> = _Alloc;
        pub type list__Node = _List_node;
        impl Default for list {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl ::std::fmt::Debug for list {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "list {{  }}")
            }
        }
        impl ::std::cmp::PartialEq for list {
            fn eq(&self, other: &list) -> bool {
                self._base == other._base
            }
        }
    }
    extern "C" {
        pub fn abs(__i: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn div(__i: ::std::os::raw::c_long, __j: ::std::os::raw::c_long) -> ldiv_t;
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
        pub fn __throw_logic_error(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_domain_error(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_invalid_argument(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_length_error(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_out_of_range(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_out_of_range_fmt(arg1: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn __throw_runtime_error(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_range_error(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_overflow_error(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_underflow_error(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_ios_failure(arg1: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn __throw_system_error(arg1: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn __throw_future_error(arg1: ::std::os::raw::c_int);
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
    pub type __truth_type___type = __false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __traitor {
        pub _address: u8,
    }
    pub const __traitor___value: __traitor__bindgen_ty_1 = __traitor__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __traitor__bindgen_ty_1 {
        __value = 0,
    }
    pub type __traitor___type = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __are_same {
        pub _address: u8,
    }
    pub const __are_same___value: __are_same__bindgen_ty_1 = __are_same__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __are_same__bindgen_ty_1 {
        __value = 0,
    }
    pub type __are_same___type = __false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_void {
        pub _address: u8,
    }
    pub const __is_void___value: __is_void__bindgen_ty_1 = __is_void__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __is_void__bindgen_ty_1 {
        __value = 0,
    }
    pub type __is_void___type = __false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_integer {
        pub _address: u8,
    }
    pub const __is_integer___value: __is_integer__bindgen_ty_1 =
        __is_integer__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __is_integer__bindgen_ty_1 {
        __value = 0,
    }
    pub type __is_integer___type = __false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_floating {
        pub _address: u8,
    }
    pub const __is_floating___value: __is_floating__bindgen_ty_1 =
        __is_floating__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __is_floating__bindgen_ty_1 {
        __value = 0,
    }
    pub type __is_floating___type = __false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_pointer {
        pub _address: u8,
    }
    pub const __is_pointer___value: __is_pointer__bindgen_ty_1 =
        __is_pointer__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __is_pointer__bindgen_ty_1 {
        __value = 0,
    }
    pub type __is_pointer___type = __false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_arithmetic {
        pub _address: u8,
    }
    impl Default for __is_arithmetic {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_scalar {
        pub _address: u8,
    }
    impl Default for __is_scalar {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_char {
        pub _address: u8,
    }
    pub const __is_char___value: __is_char__bindgen_ty_1 = __is_char__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __is_char__bindgen_ty_1 {
        __value = 0,
    }
    pub type __is_char___type = __false_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_byte {
        pub _address: u8,
    }
    pub const __is_byte___value: __is_byte__bindgen_ty_1 = __is_byte__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __is_byte__bindgen_ty_1 {
        __value = 0,
    }
    pub type __is_byte___type = __false_type;
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum byte {
        __bindgen_cannot_repr_c_on_empty_enum = 0,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_move_iterator {
        pub _address: u8,
    }
    pub const __is_move_iterator___value: __is_move_iterator__bindgen_ty_1 =
        __is_move_iterator__bindgen_ty_1::__value;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __is_move_iterator__bindgen_ty_1 {
        __value = 0,
    }
    pub type __is_move_iterator___type = __false_type;
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct disjunction {
        pub _address: u8,
    }
    impl Default for disjunction {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct negation {
        pub _address: u8,
    }
    impl Default for negation {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
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
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_void {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_integral_helper {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_integral {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_floating_point_helper {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_floating_point {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_array {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_pointer_helper {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_lvalue_reference {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_rvalue_reference {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_member_object_pointer_helper {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_member_object_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_member_function_pointer_helper {
        pub _base: false_type,
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
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_null_pointer_helper {
        pub _base: false_type,
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
            unsafe { ::std::mem::zeroed() }
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
        pub _base: false_type,
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
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_volatile {
        pub _base: false_type,
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
        pub fn __do_is_implicitly_default_constructible_impl___test() -> false_type;
    }
    impl __do_is_implicitly_default_constructible_impl {
        #[inline]
        pub unsafe fn __test() -> false_type {
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
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_base_of {
        pub _address: u8,
    }
    pub type __is_convertible_helper_type = is_void;
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
    pub type remove_cv_type = remove_const;
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
    pub type add_cv_type = add_const;
    pub type remove_const_t = remove_const;
    pub type remove_volatile_t = remove_volatile;
    pub type remove_cv_t = remove_cv;
    pub type add_const_t = add_const;
    pub type add_volatile_t = add_volatile;
    pub type add_cv_t = add_cv;
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
    pub type remove_reference_t = remove_reference;
    pub type add_lvalue_reference_t = add_lvalue_reference;
    pub type add_rvalue_reference_t = add_rvalue_reference;
    pub type __match_cv_qualifiers___match = u8;
    pub type __match_cv_qualifiers___type = __match_cv_qualifiers___match;
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
    pub type make_signed_t = make_signed;
    pub type make_unsigned_t = make_unsigned;
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
    pub type remove_extent_t = remove_extent;
    pub type remove_all_extents_t = remove_all_extents;
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type __add_pointer_helper_type<_Tp> = _Tp;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct add_pointer {
        pub _address: u8,
    }
    pub type remove_pointer_t = remove_pointer;
    pub type add_pointer_t = add_pointer;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union __aligned_storage_msa___type {
        pub __data: *mut ::std::os::raw::c_uchar,
        pub __align: __aligned_storage_msa___type__bindgen_ty_1,
        _bindgen_union_align: u64,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __aligned_storage_msa___type__bindgen_ty_1 {
        pub _address: u8,
    }
    impl Default for __aligned_storage_msa___type {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for __aligned_storage_msa___type {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "__aligned_storage_msa___type {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union aligned_storage_type {
        pub __data: *mut ::std::os::raw::c_uchar,
        pub __align: aligned_storage_type__bindgen_ty_1,
        _bindgen_union_align: u64,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct aligned_storage_type__bindgen_ty_1 {
        pub _address: u8,
    }
    impl Default for aligned_storage_type {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for aligned_storage_type {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "aligned_storage_type {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __strictest_alignment {
        pub _address: u8,
    }
    pub type aligned_union___strictest = __strictest_alignment;
    pub type aligned_union_type = u8;
    extern "C" {
        pub static alignment_value: usize;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct decay {
        pub _address: u8,
    }
    pub type decay___remove_type = remove_reference;
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
    pub type __decay_and_strip___type = __strip_reference_wrapper;
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
    pub type __expanded_common_type_wrapper_type = common_type;
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
            unsafe { ::std::mem::zeroed() }
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
    pub type __result_of_impl_type = __failure_type;
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
    pub type decay_t = decay;
    pub type enable_if_t = u8;
    pub type conditional_t = u8;
    pub type common_type_t = common_type;
    pub type underlying_type_t = underlying_type;
    pub type result_of_t = result_of;
    pub type __enable_if_t = u8;
    pub type __void_t = ::std::os::raw::c_void;
    pub type void_t = ::std::os::raw::c_void;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __detector {
        pub _address: u8,
    }
    pub type __detector_value_t = false_type;
    pub type __detector_type<_Default> = _Default;
    pub type __detected_or = __detector;
    pub type __detected_or_t = __detected_or;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_tuple_like_impl {
        pub _base: false_type,
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
        pub _base: false_type,
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
    pub type __call_is_nothrow_ = __call_is_nothrow;
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
    extern "C" {
        #[link_name = "\u{1}__nonesuch"]
        pub fn __nonesuch___nonesuch(this: *mut __nonesuch);
    }
    extern "C" {
        #[link_name = "\u{1}__nonesuch"]
        pub fn __nonesuch___nonesuch1(this: *mut __nonesuch, arg1: *const __nonesuch);
    }
    extern "C" {
        #[link_name = "\u{1}__nonesuch_destructor"]
        pub fn __nonesuch___nonesuch_destructor(this: *mut __nonesuch);
    }
    impl __nonesuch {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            __nonesuch___nonesuch(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const __nonesuch) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            __nonesuch___nonesuch1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn destruct(&mut self) {
            __nonesuch___nonesuch_destructor(self)
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct invoke_result {
        pub _address: u8,
    }
    impl Default for invoke_result {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type invoke_result_t = invoke_result;
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
        pub _base: false_type,
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
        #[link_name = "\u{1}piecewise_construct_t"]
        pub fn piecewise_construct_t_piecewise_construct_t(this: *mut piecewise_construct_t);
    }
    impl piecewise_construct_t {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            piecewise_construct_t_piecewise_construct_t(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        pub static piecewise_construct: piecewise_construct_t;
    }
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __nonesuch_no_braces {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}__nonesuch_no_braces"]
        pub fn __nonesuch_no_braces___nonesuch_no_braces(
            this: *mut __nonesuch_no_braces,
            arg1: *const __nonesuch,
        );
    }
    impl __nonesuch_no_braces {
        #[inline]
        pub unsafe fn new(arg1: *const __nonesuch) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            __nonesuch_no_braces___nonesuch_no_braces(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
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
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_T1>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_T2>>,
    }
    pub type pair_first_type<_T1> = _T1;
    pub type pair_second_type<_T2> = _T2;
    pub type pair__PCCP = u8;
    pub type pair__PCCFP = u8;
    impl<_T1, _T2> Default for pair<_T1, _T2> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
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
    #[derive(Debug, Copy, Clone)]
    pub struct __undefined {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __get_first_arg {
        pub _address: u8,
    }
    pub type __get_first_arg_type = __undefined;
    pub type __get_first_arg_t = __get_first_arg;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __replace_first_arg {
        pub _address: u8,
    }
    pub type __replace_first_arg_t = __replace_first_arg;
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type pointer_traits_pointer<_Ptr> = _Ptr;
    pub type pointer_traits_element_type = __detected_or_t;
    pub type pointer_traits_difference_type = __detected_or_t;
    pub type pointer_traits_rebind = pointer_traits___rebind;
    pub type __ptr_rebind = pointer_traits;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct reverse_iterator<_Iterator> {
        pub current: _Iterator,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
    }
    pub type reverse_iterator___traits_type = iterator_traits;
    pub type reverse_iterator_iterator_type<_Iterator> = _Iterator;
    pub type reverse_iterator_difference_type = reverse_iterator___traits_type;
    pub type reverse_iterator_pointer = reverse_iterator___traits_type;
    pub type reverse_iterator_reference = reverse_iterator___traits_type;
    impl<_Iterator> Default for reverse_iterator<_Iterator> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct back_insert_iterator<_Container> {
        pub container: *mut _Container,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Container>>,
    }
    pub type back_insert_iterator_container_type<_Container> = _Container;
    impl<_Container> Default for back_insert_iterator<_Container> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct front_insert_iterator<_Container> {
        pub container: *mut _Container,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Container>>,
    }
    pub type front_insert_iterator_container_type<_Container> = _Container;
    impl<_Container> Default for front_insert_iterator<_Container> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    pub struct insert_iterator<_Container> {
        pub container: *mut _Container,
        pub iter: [u8; 0usize],
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Container>>,
    }
    pub type insert_iterator_container_type<_Container> = _Container;
    impl<_Container> Default for insert_iterator<_Container> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_Container> ::std::fmt::Debug for insert_iterator<_Container> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "insert_iterator {{ container: {:?} }}", self.container)
        }
    }
    impl<_Container> ::std::cmp::PartialEq for insert_iterator<_Container>
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
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
    }
    pub type move_iterator___traits_type = iterator_traits;
    pub type move_iterator___base_ref = move_iterator___traits_type;
    pub type move_iterator_iterator_type<_Iterator> = _Iterator;
    pub type move_iterator_iterator_category = move_iterator___traits_type;
    pub type move_iterator_value_type = move_iterator___traits_type;
    pub type move_iterator_difference_type = move_iterator___traits_type;
    pub type move_iterator_pointer<_Iterator> = _Iterator;
    pub type move_iterator_reference = u8;
    impl<_Iterator> Default for move_iterator<_Iterator> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type __iter_key_t = remove_const_t;
    pub type __iter_val_t = iterator_traits;
    pub type __iter_to_alloc_t = pair<add_const_t, __iter_val_t>;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __lc_rai {
        pub _address: u8,
    }
    extern "C" {
        pub fn __lg(__n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    #[repr(C)]
    pub struct exception__bindgen_vtable(::std::os::raw::c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct exception {
        pub vtable_: *const exception__bindgen_vtable,
    }
    extern "C" {
        #[link_name = "\u{1}exception"]
        pub fn exception_exception(this: *mut exception);
    }
    impl Default for exception {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl exception {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            exception_exception(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}exception_destructor"]
        pub fn exception_exception_destructor(this: *mut exception);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn exception_what(this: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_exception {
        pub _base: exception,
    }
    extern "C" {
        #[link_name = "\u{1}bad_exception"]
        pub fn bad_exception_bad_exception(this: *mut bad_exception);
    }
    impl Default for bad_exception {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl bad_exception {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            bad_exception_bad_exception(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_exception_destructor"]
        pub fn bad_exception_bad_exception_destructor(this: *mut bad_exception);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_exception_what(
            this: *mut ::std::os::raw::c_void,
        ) -> *const ::std::os::raw::c_char;
    }
    pub type terminate_handler = ::std::option::Option<unsafe extern "C" fn()>;
    pub type unexpected_handler = ::std::option::Option<unsafe extern "C" fn()>;
    extern "C" {
        pub fn set_terminate(arg1: terminate_handler) -> terminate_handler;
    }
    extern "C" {
        pub fn get_terminate() -> terminate_handler;
    }
    extern "C" {
        pub fn terminate();
    }
    extern "C" {
        pub fn set_unexpected(arg1: unexpected_handler) -> unexpected_handler;
    }
    extern "C" {
        pub fn get_unexpected() -> unexpected_handler;
    }
    extern "C" {
        pub fn unexpected();
    }
    extern "C" {
        pub fn uncaught_exception() -> bool;
    }
    extern "C" {
        pub fn uncaught_exceptions() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn _Hash_bytes(
            __ptr: *const ::std::os::raw::c_void,
            __len: usize,
            __seed: usize,
        ) -> usize;
    }
    extern "C" {
        pub fn _Fnv_hash_bytes(
            __ptr: *const ::std::os::raw::c_void,
            __len: usize,
            __seed: usize,
        ) -> usize;
    }
    #[repr(C)]
    pub struct type_info__bindgen_vtable(::std::os::raw::c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct type_info {
        pub vtable_: *const type_info__bindgen_vtable,
        pub __name: *const ::std::os::raw::c_char,
    }
    extern "C" {
        #[link_name = "\u{1}name"]
        pub fn type_info_name(this: *const type_info) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}before"]
        pub fn type_info_before(this: *const type_info, __arg: *const type_info) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}hash_code"]
        pub fn type_info_hash_code(this: *const type_info) -> usize;
    }
    extern "C" {
        #[link_name = "\u{1}type_info"]
        pub fn type_info_type_info(this: *mut type_info, __n: *const ::std::os::raw::c_char);
    }
    impl Default for type_info {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl type_info {
        #[inline]
        pub unsafe fn name(&self) -> *const ::std::os::raw::c_char {
            type_info_name(self)
        }
        #[inline]
        pub unsafe fn before(&self, __arg: *const type_info) -> bool {
            type_info_before(self, __arg)
        }
        #[inline]
        pub unsafe fn hash_code(&self) -> usize {
            type_info_hash_code(self)
        }
        #[inline]
        pub unsafe fn new(__n: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            type_info_type_info(&mut __bindgen_tmp, __n);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}type_info_destructor"]
        pub fn type_info_type_info_destructor(this: *mut type_info);
    }
    extern "C" {
        #[link_name = "\u{1}__is_pointer_p"]
        pub fn type_info___is_pointer_p(this: *mut ::std::os::raw::c_void) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}__is_function_p"]
        pub fn type_info___is_function_p(this: *mut ::std::os::raw::c_void) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}__do_catch"]
        pub fn type_info___do_catch(
            this: *mut ::std::os::raw::c_void,
            __thr_type: *const type_info,
            __thr_obj: *mut *mut ::std::os::raw::c_void,
            __outer: ::std::os::raw::c_uint,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}__do_upcast"]
        pub fn type_info___do_upcast(
            this: *mut ::std::os::raw::c_void,
            __target: *const __class_type_info,
            __obj_ptr: *mut *mut ::std::os::raw::c_void,
        ) -> bool;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_cast {
        pub _base: exception,
    }
    extern "C" {
        #[link_name = "\u{1}bad_cast"]
        pub fn bad_cast_bad_cast(this: *mut bad_cast);
    }
    impl Default for bad_cast {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl bad_cast {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            bad_cast_bad_cast(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_cast_destructor"]
        pub fn bad_cast_bad_cast_destructor(this: *mut bad_cast);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_cast_what(this: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_typeid {
        pub _base: exception,
    }
    extern "C" {
        #[link_name = "\u{1}bad_typeid"]
        pub fn bad_typeid_bad_typeid(this: *mut bad_typeid);
    }
    impl Default for bad_typeid {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl bad_typeid {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            bad_typeid_bad_typeid(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_typeid_destructor"]
        pub fn bad_typeid_bad_typeid_destructor(this: *mut bad_typeid);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_typeid_what(this: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
    }
    pub mod __exception_ptr {

        #[repr(C)]
        #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct exception_ptr {
            pub _M_exception_object: *mut ::std::os::raw::c_void,
        }
        extern "C" {
            #[link_name = "\u{1}swap"]
            pub fn exception_ptr_swap(this: *mut exception_ptr, arg1: *mut exception_ptr);
        }
        extern "C" {
            #[link_name = "\u{1}__cxa_exception_type"]
            pub fn exception_ptr___cxa_exception_type(
                this: *const exception_ptr,
            ) -> *const type_info;
        }
        extern "C" {
            #[link_name = "\u{1}exception_ptr"]
            pub fn exception_ptr_exception_ptr(this: *mut exception_ptr);
        }
        extern "C" {
            #[link_name = "\u{1}exception_ptr"]
            pub fn exception_ptr_exception_ptr1(
                this: *mut exception_ptr,
                arg1: *const exception_ptr,
            );
        }
        extern "C" {
            #[link_name = "\u{1}exception_ptr"]
            pub fn exception_ptr_exception_ptr2(this: *mut exception_ptr, arg1: nullptr_t);
        }
        extern "C" {
            #[link_name = "\u{1}exception_ptr"]
            pub fn exception_ptr_exception_ptr3(this: *mut exception_ptr, __o: *mut exception_ptr);
        }
        extern "C" {
            #[link_name = "\u{1}exception_ptr_destructor"]
            pub fn exception_ptr_exception_ptr_destructor(this: *mut exception_ptr);
        }
        impl Default for exception_ptr {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
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
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                exception_ptr_exception_ptr(&mut __bindgen_tmp);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new1(arg1: *const exception_ptr) -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                exception_ptr_exception_ptr1(&mut __bindgen_tmp, arg1);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new2(arg1: nullptr_t) -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                exception_ptr_exception_ptr2(&mut __bindgen_tmp, arg1);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new3(__o: *mut exception_ptr) -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                exception_ptr_exception_ptr3(&mut __bindgen_tmp, __o);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn destruct(&mut self) {
                exception_ptr_exception_ptr_destructor(self)
            }
        }
        extern "C" {
            pub fn swap(__lhs: *mut exception_ptr, __rhs: *mut exception_ptr);
        }
    }
    extern "C" {
        pub fn current_exception() -> exception_ptr;
    }
    extern "C" {
        pub fn rethrow_exception(arg1: exception_ptr);
    }
    #[repr(C)]
    pub struct nested_exception__bindgen_vtable(::std::os::raw::c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct nested_exception {
        pub vtable_: *const nested_exception__bindgen_vtable,
        pub _M_ptr: exception_ptr,
    }
    extern "C" {
        #[link_name = "\u{1}rethrow_nested"]
        pub fn nested_exception_rethrow_nested(this: *const nested_exception);
    }
    extern "C" {
        #[link_name = "\u{1}nested_ptr"]
        pub fn nested_exception_nested_ptr(this: *const nested_exception) -> exception_ptr;
    }
    extern "C" {
        #[link_name = "\u{1}nested_exception"]
        pub fn nested_exception_nested_exception(this: *mut nested_exception);
    }
    extern "C" {
        #[link_name = "\u{1}nested_exception"]
        pub fn nested_exception_nested_exception1(
            this: *mut nested_exception,
            arg1: *const nested_exception,
        );
    }
    impl Default for nested_exception {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl nested_exception {
        #[inline]
        pub unsafe fn rethrow_nested(&self) {
            nested_exception_rethrow_nested(self)
        }
        #[inline]
        pub unsafe fn nested_ptr(&self) -> exception_ptr {
            nested_exception_nested_ptr(self)
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            nested_exception_nested_exception(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const nested_exception) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            nested_exception_nested_exception1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}nested_exception_destructor"]
        pub fn nested_exception_nested_exception_destructor(this: *mut nested_exception);
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Nested_exception<_Except> {
        pub _base: _Except,
        pub _base_1: nested_exception,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Except>>,
    }
    impl<_Except> Default for _Nested_exception<_Except> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type __rethrow_if_nested_cond = u8;
    extern "C" {
        pub fn __rethrow_if_nested_impl(arg1: *const ::std::os::raw::c_void);
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_alloc {
        pub _base: exception,
    }
    extern "C" {
        #[link_name = "\u{1}bad_alloc"]
        pub fn bad_alloc_bad_alloc(this: *mut bad_alloc);
    }
    impl Default for bad_alloc {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl bad_alloc {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            bad_alloc_bad_alloc(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_alloc_destructor"]
        pub fn bad_alloc_bad_alloc_destructor(this: *mut bad_alloc);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_alloc_what(this: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_array_new_length {
        pub _base: bad_alloc,
    }
    extern "C" {
        #[link_name = "\u{1}bad_array_new_length"]
        pub fn bad_array_new_length_bad_array_new_length(this: *mut bad_array_new_length);
    }
    impl Default for bad_array_new_length {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl bad_array_new_length {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            bad_array_new_length_bad_array_new_length(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}bad_array_new_length_destructor"]
        pub fn bad_array_new_length_bad_array_new_length_destructor(
            this: *mut bad_array_new_length,
        );
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_array_new_length_what(
            this: *mut ::std::os::raw::c_void,
        ) -> *const ::std::os::raw::c_char;
    }
    #[repr(u64)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum align_val_t {
        __bindgen_cannot_repr_c_on_empty_enum = 0,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct nothrow_t {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}nothrow_t"]
        pub fn nothrow_t_nothrow_t(this: *mut nothrow_t);
    }
    impl nothrow_t {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            nothrow_t_nothrow_t(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        pub static nothrow: nothrow_t;
    }
    pub type new_handler = ::std::option::Option<unsafe extern "C" fn()>;
    extern "C" {
        pub fn set_new_handler(arg1: new_handler) -> new_handler;
    }
    extern "C" {
        pub fn get_new_handler() -> new_handler;
    }
    extern "C" {
        pub fn launder(arg1: *mut ::std::os::raw::c_void);
    }
    pub type __allocator_base = new_allocator;
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
    pub type allocator_rebind_other = allocator;
    pub type allocator_propagate_on_container_move_assignment = true_type;
    pub type allocator_is_always_equal = true_type;
    impl Default for allocator {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct minus {
        pub _address: u8,
    }
    impl Default for minus {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct multiplies {
        pub _address: u8,
    }
    impl Default for multiplies {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct divides {
        pub _address: u8,
    }
    impl Default for divides {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct modulus {
        pub _address: u8,
    }
    impl Default for modulus {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct negate {
        pub _address: u8,
    }
    impl Default for negate {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bit_or {
        pub _address: u8,
    }
    impl Default for bit_or {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bit_xor {
        pub _address: u8,
    }
    impl Default for bit_xor {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bit_not {
        pub _address: u8,
    }
    impl Default for bit_not {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct unary_negate<_Predicate> {
        pub _M_pred: _Predicate,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Predicate>>,
    }
    impl<_Predicate> Default for unary_negate<_Predicate> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct binary_negate<_Predicate> {
        pub _M_pred: _Predicate,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Predicate>>,
    }
    impl<_Predicate> Default for binary_negate<_Predicate> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct pointer_to_unary_function<_Arg, _Result> {
        pub _M_ptr: ::std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Result>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Arg>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Result>>,
    }
    impl<_Arg, _Result> Default for pointer_to_unary_function<_Arg, _Result> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct pointer_to_binary_function<_Arg1, _Arg2, _Result> {
        pub _M_ptr:
            ::std::option::Option<unsafe extern "C" fn(arg1: _Arg1, arg2: _Arg2) -> _Result>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Arg1>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Arg2>>,
        pub _phantom_2: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Result>>,
    }
    impl<_Arg1, _Arg2, _Result> Default for pointer_to_binary_function<_Arg1, _Arg2, _Result> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Identity {
        pub _address: u8,
    }
    impl Default for _Identity {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Select1st {
        pub _address: u8,
    }
    impl Default for _Select1st {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Select2nd {
        pub _address: u8,
    }
    impl Default for _Select2nd {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct mem_fun_t<_Ret> {
        pub _M_f: ::std::option::Option<unsafe extern "C" fn() -> _Ret>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Ret>>,
    }
    impl<_Ret> Default for mem_fun_t<_Ret> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct const_mem_fun_t<_Ret> {
        pub _M_f: ::std::option::Option<unsafe extern "C" fn() -> _Ret>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Ret>>,
    }
    impl<_Ret> Default for const_mem_fun_t<_Ret> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct mem_fun_ref_t<_Ret> {
        pub _M_f: ::std::option::Option<unsafe extern "C" fn() -> _Ret>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Ret>>,
    }
    impl<_Ret> Default for mem_fun_ref_t<_Ret> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct const_mem_fun_ref_t<_Ret> {
        pub _M_f: ::std::option::Option<unsafe extern "C" fn() -> _Ret>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Ret>>,
    }
    impl<_Ret> Default for const_mem_fun_ref_t<_Ret> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct mem_fun1_t<_Ret, _Arg> {
        pub _M_f: ::std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Ret>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Arg>>,
    }
    impl<_Ret, _Arg> Default for mem_fun1_t<_Ret, _Arg> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct const_mem_fun1_t<_Ret, _Arg> {
        pub _M_f: ::std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Ret>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Arg>>,
    }
    impl<_Ret, _Arg> Default for const_mem_fun1_t<_Ret, _Arg> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct mem_fun1_ref_t<_Ret, _Arg> {
        pub _M_f: ::std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Ret>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Arg>>,
    }
    impl<_Ret, _Arg> Default for mem_fun1_ref_t<_Ret, _Arg> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct const_mem_fun1_ref_t<_Ret, _Arg> {
        pub _M_f: ::std::option::Option<unsafe extern "C" fn(arg1: _Arg) -> _Ret>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Ret>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Arg>>,
    }
    impl<_Ret, _Arg> Default for const_mem_fun1_ref_t<_Ret, _Arg> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    pub struct binder1st<_Operation> {
        pub op: _Operation,
        pub value: [u8; 0usize],
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Operation>>,
    }
    impl<_Operation> Default for binder1st<_Operation> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_Operation> ::std::fmt::Debug for binder1st<_Operation> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "binder1st {{ op: Non-debuggable generic }}")
        }
    }
    impl<_Operation> ::std::cmp::PartialEq for binder1st<_Operation>
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
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Operation>>,
    }
    impl<_Operation> Default for binder2nd<_Operation> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_Operation> ::std::fmt::Debug for binder2nd<_Operation> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "binder2nd {{ op: Non-debuggable generic }}")
        }
    }
    impl<_Operation> ::std::cmp::PartialEq for binder2nd<_Operation>
    where
        _Operation: PartialEq,
    {
        fn eq(&self, other: &binder2nd<_Operation>) -> bool {
            self.op == other.op && self.value == other.value
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
            unsafe { ::std::mem::zeroed() }
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
    pub type __alloc_rebind = __allocator_traits_base;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits {
        pub _address: u8,
    }
    pub type allocator_traits_allocator_type<_Alloc> = _Alloc;
    pub type allocator_traits_value_type = [u8; 0usize];
    pub type allocator_traits_pointer = __detected_or_t;
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
    pub type allocator_traits__Diff_type = pointer_traits;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits__Size {
        pub _address: u8,
    }
    impl Default for allocator_traits__Size {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
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
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_traits___construct_helper {
        pub _address: u8,
    }
    pub type allocator_traits___construct_helper_type<_Alloc> = _Alloc;
    pub type allocator_traits___has_construct = allocator_traits___construct_helper;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_copy_insertable_impl {
        pub _address: u8,
    }
    pub type __is_copy_insertable_impl__Traits = allocator_traits;
    pub type __is_copy_insertable_impl_type<_Alloc> = _Alloc;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_copy_insertable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_allocator {
        pub _base: false_type,
    }
    pub type _RequireAllocator = u8;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct initializer_list<_E> {
        pub _M_array: initializer_list_iterator<_E>,
        pub _M_len: initializer_list_size_type,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_E>>,
    }
    pub type initializer_list_value_type<_E> = _E;
    pub type initializer_list_reference<_E> = *const _E;
    pub type initializer_list_const_reference<_E> = *const _E;
    pub type initializer_list_size_type = usize;
    pub type initializer_list_iterator<_E> = *const _E;
    pub type initializer_list_const_iterator<_E> = *const _E;
    impl<_E> Default for initializer_list<_E> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
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
    pub type make_index_sequence = make_integer_sequence;
    pub type index_sequence_for = make_index_sequence;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct in_place_t {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}in_place_t"]
        pub fn in_place_t_in_place_t(this: *mut in_place_t);
    }
    impl in_place_t {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            in_place_t_in_place_t(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        pub static in_place: in_place_t;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct in_place_type_t {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_in_place_type_impl {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_in_place_type {
        pub _base: __is_in_place_type_impl,
    }
    impl Default for __is_in_place_type {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    extern "C" {
        pub fn wcschr(__p: *mut u32, __c: u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcspbrk(__s1: *mut u32, __s2: *const u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcsrchr(__p: *mut u32, __c: u32) -> *mut u32;
    }
    extern "C" {
        pub fn wcsstr(__s1: *mut u32, __s2: *const u32) -> *mut u32;
    }
    extern "C" {
        pub fn wmemchr(__p: *mut u32, __c: u32, __n: usize) -> *mut u32;
    }
    pub type streamoff = ::std::os::raw::c_long;
    pub type streamsize = isize;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct fpos<_StateT> {
        pub _M_off: streamoff,
        pub _M_state: _StateT,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_StateT>>,
    }
    impl<_StateT> Default for fpos<_StateT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type streampos = fpos<mbstate_t>;
    pub type wstreampos = fpos<mbstate_t>;
    pub type u16streampos = fpos<mbstate_t>;
    pub type u32streampos = fpos<mbstate_t>;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct char_traits {
        pub _address: u8,
    }
    impl Default for char_traits {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type __c_locale = __locale_t;
    extern "C" {
        pub fn __convert_from_v(
            __cloc: *const __c_locale,
            __out: *mut ::std::os::raw::c_char,
            __size: ::std::os::raw::c_int,
            __fmt: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    pub type ios = basic_ios<::std::os::raw::c_char>;
    pub type streambuf = basic_streambuf<::std::os::raw::c_char>;
    pub type istream = basic_istream<::std::os::raw::c_char>;
    pub type ostream = basic_ostream<::std::os::raw::c_char>;
    pub type iostream = basic_iostream<::std::os::raw::c_char>;
    pub type stringbuf = basic_stringbuf<::std::os::raw::c_char>;
    pub type istringstream = basic_istringstream<::std::os::raw::c_char>;
    pub type ostringstream = basic_ostringstream<::std::os::raw::c_char>;
    pub type stringstream = basic_stringstream<::std::os::raw::c_char>;
    pub type filebuf = basic_filebuf<::std::os::raw::c_char>;
    pub type ifstream = basic_ifstream<::std::os::raw::c_char>;
    pub type ofstream = basic_ofstream<::std::os::raw::c_char>;
    pub type fstream = basic_fstream<::std::os::raw::c_char>;
    pub type wios = basic_ios<u32>;
    pub type wstreambuf = basic_streambuf<u32>;
    pub type wistream = basic_istream<u32>;
    pub type wostream = basic_ostream<u32>;
    pub type wiostream = basic_iostream<u32>;
    pub type wstringbuf = basic_stringbuf<u32>;
    pub type wistringstream = basic_istringstream<u32>;
    pub type wostringstream = basic_ostringstream<u32>;
    pub type wstringstream = basic_stringstream<u32>;
    pub type wfilebuf = basic_filebuf<u32>;
    pub type wifstream = basic_ifstream<u32>;
    pub type wofstream = basic_ofstream<u32>;
    pub type wfstream = basic_fstream<u32>;
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
    pub struct valarray {
        pub _address: u8,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum float_round_style {
        round_indeterminate = -1,
        round_toward_zero = 0,
        round_to_nearest = 1,
        round_toward_infinity = 2,
        round_toward_neg_infinity = 3,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum float_denorm_style {
        denorm_indeterminate = -1,
        denorm_absent = 0,
        denorm_present = 1,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __numeric_limits_base {
        pub _address: u8,
    }
    pub const __numeric_limits_base_is_specialized: bool = false;
    pub const __numeric_limits_base_digits: ::std::os::raw::c_int = 0;
    pub const __numeric_limits_base_digits10: ::std::os::raw::c_int = 0;
    pub const __numeric_limits_base_max_digits10: ::std::os::raw::c_int = 0;
    pub const __numeric_limits_base_is_signed: bool = false;
    pub const __numeric_limits_base_is_integer: bool = false;
    pub const __numeric_limits_base_is_exact: bool = false;
    pub const __numeric_limits_base_radix: ::std::os::raw::c_int = 0;
    pub const __numeric_limits_base_min_exponent: ::std::os::raw::c_int = 0;
    pub const __numeric_limits_base_min_exponent10: ::std::os::raw::c_int = 0;
    pub const __numeric_limits_base_max_exponent: ::std::os::raw::c_int = 0;
    pub const __numeric_limits_base_max_exponent10: ::std::os::raw::c_int = 0;
    pub const __numeric_limits_base_has_infinity: bool = false;
    pub const __numeric_limits_base_has_quiet_NaN: bool = false;
    pub const __numeric_limits_base_has_signaling_NaN: bool = false;
    extern "C" {
        #[link_name = "\u{1}has_denorm"]
        pub static __numeric_limits_base_has_denorm: float_denorm_style;
    }
    pub const __numeric_limits_base_has_denorm_loss: bool = false;
    pub const __numeric_limits_base_is_iec559: bool = false;
    pub const __numeric_limits_base_is_bounded: bool = false;
    pub const __numeric_limits_base_is_modulo: bool = false;
    pub const __numeric_limits_base_traps: bool = false;
    pub const __numeric_limits_base_tinyness_before: bool = false;
    extern "C" {
        #[link_name = "\u{1}round_style"]
        pub static __numeric_limits_base_round_style: float_round_style;
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
    extern "C" {
        #[link_name = "\u{1}hash"]
        pub fn _Hash_impl_hash(
            __ptr: *const ::std::os::raw::c_void,
            __clength: usize,
            __seed: usize,
        ) -> usize;
    }
    impl _Hash_impl {
        #[inline]
        pub unsafe fn hash(
            __ptr: *const ::std::os::raw::c_void,
            __clength: usize,
            __seed: usize,
        ) -> usize {
            _Hash_impl_hash(__ptr, __clength, __seed)
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Fnv_hash_impl {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}hash"]
        pub fn _Fnv_hash_impl_hash(
            __ptr: *const ::std::os::raw::c_void,
            __clength: usize,
            __seed: usize,
        ) -> usize;
    }
    impl _Fnv_hash_impl {
        #[inline]
        pub unsafe fn hash(
            __ptr: *const ::std::os::raw::c_void,
            __clength: usize,
            __seed: usize,
        ) -> usize {
            _Fnv_hash_impl_hash(__ptr, __clength, __seed)
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_fast_hash {
        pub _base: true_type,
    }
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
    pub type basic_string_view_iterator<_CharT> = basic_string_view_const_iterator<_CharT>;
    pub type basic_string_view_const_reverse_iterator<_CharT> =
        reverse_iterator<basic_string_view_const_iterator<_CharT>>;
    pub type basic_string_view_reverse_iterator<_CharT> =
        basic_string_view_const_reverse_iterator<_CharT>;
    pub type basic_string_view_size_type = usize;
    pub type basic_string_view_difference_type = isize;
    impl<_CharT> Default for basic_string_view<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub mod __detail {

        pub type __idt = common_type_t;
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _List_node_base {
            pub _M_next: *mut _List_node_base,
            pub _M_prev: *mut _List_node_base,
        }
        extern "C" {
            #[link_name = "\u{1}swap"]
            pub fn _List_node_base_swap(__x: *mut _List_node_base, __y: *mut _List_node_base);
        }
        extern "C" {
            #[link_name = "\u{1}_M_transfer"]
            pub fn _List_node_base__M_transfer(
                this: *mut _List_node_base,
                __first: *mut _List_node_base,
                __last: *mut _List_node_base,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_M_reverse"]
            pub fn _List_node_base__M_reverse(this: *mut _List_node_base);
        }
        extern "C" {
            #[link_name = "\u{1}_M_hook"]
            pub fn _List_node_base__M_hook(
                this: *mut _List_node_base,
                __position: *mut _List_node_base,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_M_unhook"]
            pub fn _List_node_base__M_unhook(this: *mut _List_node_base);
        }
        impl Default for _List_node_base {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl _List_node_base {
            #[inline]
            pub unsafe fn swap(__x: *mut _List_node_base, __y: *mut _List_node_base) {
                _List_node_base_swap(__x, __y)
            }
            #[inline]
            pub unsafe fn _M_transfer(
                &mut self,
                __first: *mut _List_node_base,
                __last: *mut _List_node_base,
            ) {
                _List_node_base__M_transfer(self, __first, __last)
            }
            #[inline]
            pub unsafe fn _M_reverse(&mut self) {
                _List_node_base__M_reverse(self)
            }
            #[inline]
            pub unsafe fn _M_hook(&mut self, __position: *mut _List_node_base) {
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
            pub _base: _List_node_base,
            pub _M_size: usize,
        }
        extern "C" {
            #[link_name = "\u{1}_M_move_nodes"]
            pub fn _List_node_header__M_move_nodes(
                this: *mut _List_node_header,
                __x: *mut _List_node_header,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_M_init"]
            pub fn _List_node_header__M_init(this: *mut _List_node_header);
        }
        extern "C" {
            #[link_name = "\u{1}_List_node_header"]
            pub fn _List_node_header__List_node_header(this: *mut _List_node_header);
        }
        extern "C" {
            #[link_name = "\u{1}_List_node_header"]
            pub fn _List_node_header__List_node_header1(
                this: *mut _List_node_header,
                __x: *mut _List_node_header,
            );
        }
        impl Default for _List_node_header {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl _List_node_header {
            #[inline]
            pub unsafe fn _M_move_nodes(&mut self, __x: *mut _List_node_header) {
                _List_node_header__M_move_nodes(self, __x)
            }
            #[inline]
            pub unsafe fn _M_init(&mut self) {
                _List_node_header__M_init(self)
            }
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                _List_node_header__List_node_header(&mut __bindgen_tmp);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new1(__x: *mut _List_node_header) -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                _List_node_header__List_node_header1(&mut __bindgen_tmp, __x);
                __bindgen_tmp
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct __floating_point_constant {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct __numeric_constants {
            pub _address: u8,
        }
    }
    pub type string_view = basic_string_view<::std::os::raw::c_char>;
    pub type wstring_view = basic_string_view<u32>;
    pub type u16string_view = basic_string_view<u16>;
    pub type u32string_view = basic_string_view<u32>;
    extern "C" {
        pub fn getline(
            __in: *mut basic_istream<::std::os::raw::c_char>,
            __str: *mut basic_string<::std::os::raw::c_char>,
            __delim: ::std::os::raw::c_char,
        ) -> *mut basic_istream<::std::os::raw::c_char>;
    }
    extern "C" {
        pub static npos: basic_string_size_type;
    }

    #[repr(C)]
    pub struct __cow_string {
        pub __bindgen_anon_1: __cow_string__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union __cow_string__bindgen_ty_1 {
        pub _M_p: *const ::std::os::raw::c_char,
        pub _M_bytes: [::std::os::raw::c_char; 8usize],
        _bindgen_union_align: u64,
    }
    impl Default for __cow_string__bindgen_ty_1 {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for __cow_string__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "__cow_string__bindgen_ty_1 {{ union }}")
        }
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string(this: *mut __cow_string);
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string1(this: *mut __cow_string, arg1: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string2(
            this: *mut __cow_string,
            arg1: *const ::std::os::raw::c_char,
            arg2: usize,
        );
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string3(this: *mut __cow_string, arg1: *const __cow_string);
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string"]
        pub fn __cow_string___cow_string4(this: *mut __cow_string, arg1: *mut __cow_string);
    }
    extern "C" {
        #[link_name = "\u{1}__cow_string_destructor"]
        pub fn __cow_string___cow_string_destructor(this: *mut __cow_string);
    }
    impl Default for __cow_string {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
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
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            __cow_string___cow_string(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            __cow_string___cow_string1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new2(arg1: *const ::std::os::raw::c_char, arg2: usize) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            __cow_string___cow_string2(&mut __bindgen_tmp, arg1, arg2);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new3(arg1: *const __cow_string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            __cow_string___cow_string3(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new4(arg1: *mut __cow_string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            __cow_string___cow_string4(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn destruct(&mut self) {
            __cow_string___cow_string_destructor(self)
        }
    }
    pub type __sso_string = basic_string<::std::os::raw::c_char>;
    #[repr(C)]
    pub struct logic_error {
        pub _base: exception,
        pub _M_msg: __cow_string,
    }
    extern "C" {
        #[link_name = "\u{1}logic_error"]
        pub fn logic_error_logic_error(this: *mut logic_error, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}logic_error"]
        pub fn logic_error_logic_error1(
            this: *mut logic_error,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        #[link_name = "\u{1}logic_error"]
        pub fn logic_error_logic_error2(this: *mut logic_error, arg1: *const logic_error);
    }
    impl Default for logic_error {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for logic_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "logic_error {{ _M_msg: {:?} }}", self._M_msg)
        }
    }
    impl logic_error {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            logic_error_logic_error(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            logic_error_logic_error1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new2(arg1: *const logic_error) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            logic_error_logic_error2(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}logic_error_destructor"]
        pub fn logic_error_logic_error_destructor(this: *mut logic_error);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn logic_error_what(this: *mut ::std::os::raw::c_void)
            -> *const ::std::os::raw::c_char;
    }
    #[repr(C)]
    pub struct domain_error {
        pub _base: logic_error,
    }
    extern "C" {
        #[link_name = "\u{1}domain_error"]
        pub fn domain_error_domain_error(this: *mut domain_error, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}domain_error"]
        pub fn domain_error_domain_error1(
            this: *mut domain_error,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    impl Default for domain_error {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for domain_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "domain_error {{  }}")
        }
    }
    impl domain_error {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            domain_error_domain_error(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            domain_error_domain_error1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}domain_error_destructor"]
        pub fn domain_error_domain_error_destructor(this: *mut domain_error);
    }
    #[repr(C)]
    pub struct invalid_argument {
        pub _base: logic_error,
    }
    extern "C" {
        #[link_name = "\u{1}invalid_argument"]
        pub fn invalid_argument_invalid_argument(this: *mut invalid_argument, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}invalid_argument"]
        pub fn invalid_argument_invalid_argument1(
            this: *mut invalid_argument,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    impl Default for invalid_argument {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for invalid_argument {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "invalid_argument {{  }}")
        }
    }
    impl invalid_argument {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            invalid_argument_invalid_argument(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            invalid_argument_invalid_argument1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}invalid_argument_destructor"]
        pub fn invalid_argument_invalid_argument_destructor(this: *mut invalid_argument);
    }
    #[repr(C)]
    pub struct length_error {
        pub _base: logic_error,
    }
    extern "C" {
        #[link_name = "\u{1}length_error"]
        pub fn length_error_length_error(this: *mut length_error, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}length_error"]
        pub fn length_error_length_error1(
            this: *mut length_error,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    impl Default for length_error {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for length_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "length_error {{  }}")
        }
    }
    impl length_error {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            length_error_length_error(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            length_error_length_error1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}length_error_destructor"]
        pub fn length_error_length_error_destructor(this: *mut length_error);
    }
    #[repr(C)]
    pub struct out_of_range {
        pub _base: logic_error,
    }
    extern "C" {
        #[link_name = "\u{1}out_of_range"]
        pub fn out_of_range_out_of_range(this: *mut out_of_range, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}out_of_range"]
        pub fn out_of_range_out_of_range1(
            this: *mut out_of_range,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    impl Default for out_of_range {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for out_of_range {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "out_of_range {{  }}")
        }
    }
    impl out_of_range {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            out_of_range_out_of_range(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            out_of_range_out_of_range1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}out_of_range_destructor"]
        pub fn out_of_range_out_of_range_destructor(this: *mut out_of_range);
    }
    #[repr(C)]
    pub struct runtime_error {
        pub _base: exception,
        pub _M_msg: __cow_string,
    }
    extern "C" {
        #[link_name = "\u{1}runtime_error"]
        pub fn runtime_error_runtime_error(this: *mut runtime_error, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}runtime_error"]
        pub fn runtime_error_runtime_error1(
            this: *mut runtime_error,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        #[link_name = "\u{1}runtime_error"]
        pub fn runtime_error_runtime_error2(this: *mut runtime_error, arg1: *const runtime_error);
    }
    impl Default for runtime_error {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for runtime_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "runtime_error {{ _M_msg: {:?} }}", self._M_msg)
        }
    }
    impl runtime_error {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            runtime_error_runtime_error(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            runtime_error_runtime_error1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new2(arg1: *const runtime_error) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            runtime_error_runtime_error2(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}runtime_error_destructor"]
        pub fn runtime_error_runtime_error_destructor(this: *mut runtime_error);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn runtime_error_what(
            this: *mut ::std::os::raw::c_void,
        ) -> *const ::std::os::raw::c_char;
    }
    #[repr(C)]
    pub struct range_error {
        pub _base: runtime_error,
    }
    extern "C" {
        #[link_name = "\u{1}range_error"]
        pub fn range_error_range_error(this: *mut range_error, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}range_error"]
        pub fn range_error_range_error1(
            this: *mut range_error,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    impl Default for range_error {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for range_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "range_error {{  }}")
        }
    }
    impl range_error {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            range_error_range_error(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            range_error_range_error1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}range_error_destructor"]
        pub fn range_error_range_error_destructor(this: *mut range_error);
    }
    #[repr(C)]
    pub struct overflow_error {
        pub _base: runtime_error,
    }
    extern "C" {
        #[link_name = "\u{1}overflow_error"]
        pub fn overflow_error_overflow_error(this: *mut overflow_error, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}overflow_error"]
        pub fn overflow_error_overflow_error1(
            this: *mut overflow_error,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    impl Default for overflow_error {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for overflow_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "overflow_error {{  }}")
        }
    }
    impl overflow_error {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            overflow_error_overflow_error(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            overflow_error_overflow_error1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}overflow_error_destructor"]
        pub fn overflow_error_overflow_error_destructor(this: *mut overflow_error);
    }
    #[repr(C)]
    pub struct underflow_error {
        pub _base: runtime_error,
    }
    extern "C" {
        #[link_name = "\u{1}underflow_error"]
        pub fn underflow_error_underflow_error(this: *mut underflow_error, __arg: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}underflow_error"]
        pub fn underflow_error_underflow_error1(
            this: *mut underflow_error,
            arg1: *const ::std::os::raw::c_char,
        );
    }
    impl Default for underflow_error {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for underflow_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "underflow_error {{  }}")
        }
    }
    impl underflow_error {
        #[inline]
        pub unsafe fn new(__arg: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            underflow_error_underflow_error(&mut __bindgen_tmp, __arg);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            underflow_error_underflow_error1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}underflow_error_destructor"]
        pub fn underflow_error_underflow_error_destructor(this: *mut underflow_error);
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Enable_default_constructor_tag {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}_Enable_default_constructor_tag"]
        pub fn _Enable_default_constructor_tag__Enable_default_constructor_tag(
            this: *mut _Enable_default_constructor_tag,
        );
    }
    impl _Enable_default_constructor_tag {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Enable_default_constructor_tag__Enable_default_constructor_tag(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct nullopt_t {
        pub _address: u8,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum nullopt_t__Construct {
        _Token = 0,
    }
    extern "C" {
        #[link_name = "\u{1}nullopt_t"]
        pub fn nullopt_t_nullopt_t(this: *mut nullopt_t, arg1: nullopt_t__Construct);
    }
    impl nullopt_t {
        #[inline]
        pub unsafe fn new(arg1: nullopt_t__Construct) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            nullopt_t_nullopt_t(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        pub static nullopt: nullopt_t;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct bad_optional_access {
        pub _base: exception,
    }
    extern "C" {
        #[link_name = "\u{1}bad_optional_access"]
        pub fn bad_optional_access_bad_optional_access(this: *mut bad_optional_access);
    }
    impl Default for bad_optional_access {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl bad_optional_access {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            bad_optional_access_bad_optional_access(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn bad_optional_access_what(
            this: *mut ::std::os::raw::c_void,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}bad_optional_access_destructor"]
        pub fn bad_optional_access_bad_optional_access_destructor(this: *mut bad_optional_access);
    }
    extern "C" {
        pub fn __throw_bad_optional_access();
    }
    pub type _Optional_payload__Stored_type = remove_const_t;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Optional_payload__Empty_byte {
        pub _address: u8,
    }
    #[repr(C)]
    pub union _Optional_payload__bindgen_ty_1 {
        pub _M_empty: _Optional_payload__Empty_byte,
        pub _M_payload: _Optional_payload__Stored_type,
        _bindgen_union_align: [u8; 0usize],
    }
    impl Default for _Optional_payload__bindgen_ty_1 {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Optional_payload__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Optional_payload__bindgen_ty_1 {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Optional_base_impl {
        pub _address: u8,
    }
    pub type _Optional_base_impl__Stored_type = remove_const_t;
    pub type __converts_from_optional = __or_;
    pub type __assigns_from_optional = __or_;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct optional {
        pub _address: u8,
    }
    pub type optional__Base = u8;
    pub type optional_value_type<_Tp> = _Tp;
    pub type __optional_relop_t = enable_if_t;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Node_handle_common {
        pub _M_ptr: _Node_handle_common__AllocTraits,
        pub _M_alloc: optional,
    }
    pub type _Node_handle_common__AllocTraits = allocator_traits;
    pub type _Node_handle_common_allocator_type = __alloc_rebind;
    impl Default for _Node_handle_common {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Node_handle {
        pub _base: _Node_handle_common,
        pub _M_pkey: _Node_handle___pointer,
        pub _M_pmapped: _Node_handle___pointer,
    }
    pub type _Node_handle_key_type<_Key> = _Key;
    pub type _Node_handle_mapped_type = [u8; 0usize];
    pub type _Node_handle__AllocTraits = allocator_traits;
    pub type _Node_handle___pointer = __ptr_rebind;
    impl Default for _Node_handle {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum _Rb_tree_color {
        _S_red = 0,
        _S_black = 1,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_node_base {
        pub _M_color: _Rb_tree_color,
        pub _M_parent: _Rb_tree_node_base__Base_ptr,
        pub _M_left: _Rb_tree_node_base__Base_ptr,
        pub _M_right: _Rb_tree_node_base__Base_ptr,
    }
    pub type _Rb_tree_node_base__Base_ptr = *mut _Rb_tree_node_base;
    pub type _Rb_tree_node_base__Const_Base_ptr = *const _Rb_tree_node_base;
    extern "C" {
        #[link_name = "\u{1}_S_minimum"]
        pub fn _Rb_tree_node_base__S_minimum(
            __x: _Rb_tree_node_base__Base_ptr,
        ) -> _Rb_tree_node_base__Base_ptr;
    }
    extern "C" {
        #[link_name = "\u{1}_S_minimum"]
        pub fn _Rb_tree_node_base__S_minimum1(
            __x: _Rb_tree_node_base__Const_Base_ptr,
        ) -> _Rb_tree_node_base__Const_Base_ptr;
    }
    extern "C" {
        #[link_name = "\u{1}_S_maximum"]
        pub fn _Rb_tree_node_base__S_maximum(
            __x: _Rb_tree_node_base__Base_ptr,
        ) -> _Rb_tree_node_base__Base_ptr;
    }
    extern "C" {
        #[link_name = "\u{1}_S_maximum"]
        pub fn _Rb_tree_node_base__S_maximum1(
            __x: _Rb_tree_node_base__Const_Base_ptr,
        ) -> _Rb_tree_node_base__Const_Base_ptr;
    }
    impl Default for _Rb_tree_node_base {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl _Rb_tree_node_base {
        #[inline]
        pub unsafe fn _S_minimum(
            __x: _Rb_tree_node_base__Base_ptr,
        ) -> _Rb_tree_node_base__Base_ptr {
            _Rb_tree_node_base__S_minimum(__x)
        }
        #[inline]
        pub unsafe fn _S_minimum1(
            __x: _Rb_tree_node_base__Const_Base_ptr,
        ) -> _Rb_tree_node_base__Const_Base_ptr {
            _Rb_tree_node_base__S_minimum1(__x)
        }
        #[inline]
        pub unsafe fn _S_maximum(
            __x: _Rb_tree_node_base__Base_ptr,
        ) -> _Rb_tree_node_base__Base_ptr {
            _Rb_tree_node_base__S_maximum(__x)
        }
        #[inline]
        pub unsafe fn _S_maximum1(
            __x: _Rb_tree_node_base__Const_Base_ptr,
        ) -> _Rb_tree_node_base__Const_Base_ptr {
            _Rb_tree_node_base__S_maximum1(__x)
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_header {
        pub _M_header: _Rb_tree_node_base,
        pub _M_node_count: usize,
    }
    extern "C" {
        #[link_name = "\u{1}_M_move_data"]
        pub fn _Rb_tree_header__M_move_data(
            this: *mut _Rb_tree_header,
            __from: *mut _Rb_tree_header,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_M_reset"]
        pub fn _Rb_tree_header__M_reset(this: *mut _Rb_tree_header);
    }
    extern "C" {
        #[link_name = "\u{1}_Rb_tree_header"]
        pub fn _Rb_tree_header__Rb_tree_header(this: *mut _Rb_tree_header);
    }
    extern "C" {
        #[link_name = "\u{1}_Rb_tree_header"]
        pub fn _Rb_tree_header__Rb_tree_header1(
            this: *mut _Rb_tree_header,
            __x: *mut _Rb_tree_header,
        );
    }
    impl Default for _Rb_tree_header {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl _Rb_tree_header {
        #[inline]
        pub unsafe fn _M_move_data(&mut self, __from: *mut _Rb_tree_header) {
            _Rb_tree_header__M_move_data(self, __from)
        }
        #[inline]
        pub unsafe fn _M_reset(&mut self) {
            _Rb_tree_header__M_reset(self)
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Rb_tree_header__Rb_tree_header(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(__x: *mut _Rb_tree_header) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Rb_tree_header__Rb_tree_header1(&mut __bindgen_tmp, __x);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_node {
        pub _base: _Rb_tree_node_base,
        pub _M_storage: __aligned_membuf,
    }
    pub type _Rb_tree_node__Link_type = *mut _Rb_tree_node;
    impl Default for _Rb_tree_node {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    extern "C" {
        pub fn _Rb_tree_increment(__x: *mut _Rb_tree_node_base) -> *mut _Rb_tree_node_base;
    }
    extern "C" {
        pub fn _Rb_tree_decrement(__x: *mut _Rb_tree_node_base) -> *mut _Rb_tree_node_base;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_iterator {
        pub _M_node: _Rb_tree_iterator__Base_ptr,
    }
    pub type _Rb_tree_iterator_value_type<_Tp> = _Tp;
    pub type _Rb_tree_iterator_reference<_Tp> = *mut _Tp;
    pub type _Rb_tree_iterator_pointer<_Tp> = *mut _Tp;
    pub type _Rb_tree_iterator_iterator_category = bidirectional_iterator_tag;
    pub type _Rb_tree_iterator_difference_type = isize;
    pub type _Rb_tree_iterator__Self = _Rb_tree_iterator;
    pub type _Rb_tree_iterator__Base_ptr = _Rb_tree_node_base__Base_ptr;
    pub type _Rb_tree_iterator__Link_type = *mut _Rb_tree_node;
    impl Default for _Rb_tree_iterator {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_const_iterator {
        pub _M_node: _Rb_tree_const_iterator__Base_ptr,
    }
    pub type _Rb_tree_const_iterator_value_type<_Tp> = _Tp;
    pub type _Rb_tree_const_iterator_reference<_Tp> = *const _Tp;
    pub type _Rb_tree_const_iterator_pointer<_Tp> = *const _Tp;
    pub type _Rb_tree_const_iterator_iterator = _Rb_tree_iterator;
    pub type _Rb_tree_const_iterator_iterator_category = bidirectional_iterator_tag;
    pub type _Rb_tree_const_iterator_difference_type = isize;
    pub type _Rb_tree_const_iterator__Self = _Rb_tree_const_iterator;
    pub type _Rb_tree_const_iterator__Base_ptr = _Rb_tree_node_base__Const_Base_ptr;
    pub type _Rb_tree_const_iterator__Link_type = *const _Rb_tree_node;
    impl Default for _Rb_tree_const_iterator {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    extern "C" {
        pub fn _Rb_tree_insert_and_rebalance(
            __insert_left: bool,
            __x: *mut _Rb_tree_node_base,
            __p: *mut _Rb_tree_node_base,
            __header: *mut _Rb_tree_node_base,
        );
    }
    extern "C" {
        pub fn _Rb_tree_rebalance_for_erase(
            __z: *mut _Rb_tree_node_base,
            __header: *mut _Rb_tree_node_base,
        ) -> *mut _Rb_tree_node_base;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __has_is_transparent {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree_merge_helper {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree {
        pub _M_impl: u8,
    }
    pub type _Rb_tree__Node_allocator = [u8; 0usize];
    pub type _Rb_tree__Alloc_traits = __alloc_traits;
    pub type _Rb_tree__Base_ptr = *mut _Rb_tree_node_base;
    pub type _Rb_tree__Const_Base_ptr = *const _Rb_tree_node_base;
    pub type _Rb_tree__Link_type = *mut _Rb_tree_node;
    pub type _Rb_tree__Const_Link_type = *const _Rb_tree_node;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree__Reuse_or_alloc_node {
        pub _M_root: _Rb_tree__Base_ptr,
        pub _M_nodes: _Rb_tree__Base_ptr,
        pub _M_t: *mut _Rb_tree,
    }
    impl Default for _Rb_tree__Reuse_or_alloc_node {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Rb_tree__Alloc_node {
        pub _M_t: *mut _Rb_tree,
    }
    impl Default for _Rb_tree__Alloc_node {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type _Rb_tree_key_type<_Key> = _Key;
    pub type _Rb_tree_value_type<_Val> = _Val;
    pub type _Rb_tree_pointer<_Val> = *mut _Rb_tree_value_type<_Val>;
    pub type _Rb_tree_const_pointer<_Val> = *const _Rb_tree_value_type<_Val>;
    pub type _Rb_tree_reference<_Val> = *mut _Rb_tree_value_type<_Val>;
    pub type _Rb_tree_const_reference<_Val> = *const _Rb_tree_value_type<_Val>;
    pub type _Rb_tree_size_type = usize;
    pub type _Rb_tree_difference_type = isize;
    pub type _Rb_tree_allocator_type<_Alloc> = _Alloc;
    pub type _Rb_tree_reverse_iterator = reverse_iterator<_Rb_tree_iterator>;
    pub type _Rb_tree_const_reverse_iterator = reverse_iterator<_Rb_tree_const_iterator>;
    pub type _Rb_tree_node_type = _Node_handle;
    pub type _Rb_tree_insert_return_type = _Node_insert_return<_Iterator, _NodeHandle>;
    pub type _Rb_tree__Compatible_tree = _Rb_tree;
    pub type _Rb_tree__Rb_tree_impl__Base_key_compare<_Key_compare> =
        _Rb_tree_key_compare<_Key_compare>;
    extern "C" {
        pub fn _Rb_tree_black_count(
            __node: *const _Rb_tree_node_base,
            __root: *const _Rb_tree_node_base,
        ) -> ::std::os::raw::c_uint;
    }
    pub type __array_traits__Type<_Tp> = *mut _Tp;
    pub type __array_traits__Is_swappable = __is_swappable;
    pub type __array_traits__Is_nothrow_swappable = __is_nothrow_swappable;
    pub type array_value_type<_Tp> = _Tp;
    pub type array_pointer<_Tp> = *mut array_value_type<_Tp>;
    pub type array_const_pointer<_Tp> = *const array_value_type<_Tp>;
    pub type array_reference<_Tp> = *mut array_value_type<_Tp>;
    pub type array_const_reference<_Tp> = *const array_value_type<_Tp>;
    pub type array_iterator<_Tp> = *mut array_value_type<_Tp>;
    pub type array_const_iterator<_Tp> = *const array_value_type<_Tp>;
    pub type array_size_type = usize;
    pub type array_difference_type = isize;
    pub type array_reverse_iterator<_Tp> = reverse_iterator<array_iterator<_Tp>>;
    pub type array_const_reverse_iterator<_Tp> = reverse_iterator<array_const_iterator<_Tp>>;
    pub type array__AT_Type = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __erased_type {
        pub _address: u8,
    }
    pub type __is_erased_or_convertible = __or_;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct allocator_arg_t {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}allocator_arg_t"]
        pub fn allocator_arg_t_allocator_arg_t(this: *mut allocator_arg_t);
    }
    impl allocator_arg_t {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            allocator_arg_t_allocator_arg_t(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        pub static allocator_arg: allocator_arg_t;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __uses_allocator_helper {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct uses_allocator {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __uses_alloc_base {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __uses_alloc0 {
        pub _M_a: __uses_alloc0__Sink,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __uses_alloc0__Sink {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __uses_alloc1<_Alloc> {
        pub _M_a: *const _Alloc,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Alloc>>,
    }
    impl<_Alloc> Default for __uses_alloc1<_Alloc> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __uses_alloc2<_Alloc> {
        pub _M_a: *const _Alloc,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Alloc>>,
    }
    impl<_Alloc> Default for __uses_alloc2<_Alloc> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type __uses_alloc_t = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_uses_allocator_predicate {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_uses_allocator_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_nothrow_uses_allocator_constructible {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_empty_non_tuple {
        pub _address: u8,
    }
    impl Default for __is_empty_non_tuple {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type __empty_not_final = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct tuple {
        pub _address: u8,
    }
    pub type tuple__Inherited = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct tuple__TC2 {
        pub _address: u8,
    }
    pub type tuple__TCC = u8;
    pub type tuple__TMC = u8;
    pub type tuple__TMCT = u8;
    pub type tuple__TNTC = u8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __do_make_tuple {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __make_tuple {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __combine_tuples {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __tuple_cat_result {
        pub _address: u8,
    }
    pub type __tuple_cat_result___type = __combine_tuples;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __make_1st_indices {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __tuple_concater {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Swallow_assign {
        pub _address: u8,
    }
    extern "C" {
        pub static ignore: _Swallow_assign;
    }
    #[repr(C)]
    pub struct map {
        pub _M_t: map__Rep_type,
    }
    pub type map_key_type<_Key> = _Key;
    pub type map_mapped_type<_Tp> = _Tp;
    pub type map_value_type<_Key, _Tp> = pair<_Key, _Tp>;
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type map__Pair_alloc_type = [u8; 0usize];
    pub type map__Rep_type = _Rb_tree;
    pub type map__Alloc_traits = __alloc_traits;
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for map {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "map {{ _M_t: {:?} }}", self._M_t)
        }
    }
    impl ::std::cmp::PartialEq for map {
        fn eq(&self, other: &map) -> bool {
            self._M_t == other._M_t
        }
    }
    #[repr(C)]
    pub struct multimap {
        pub _M_t: multimap__Rep_type,
    }
    pub type multimap_key_type<_Key> = _Key;
    pub type multimap_mapped_type<_Tp> = _Tp;
    pub type multimap_value_type<_Key, _Tp> = pair<_Key, _Tp>;
    pub type multimap_key_compare<_Compare> = _Compare;
    pub type multimap_allocator_type<_Alloc> = _Alloc;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct multimap_value_compare<_Compare> {
        pub comp: _Compare,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Compare>>,
    }
    impl<_Compare> Default for multimap_value_compare<_Compare> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type multimap__Pair_alloc_type = [u8; 0usize];
    pub type multimap__Rep_type = _Rb_tree;
    pub type multimap__Alloc_traits = __alloc_traits;
    pub type multimap_pointer = [u8; 0usize];
    pub type multimap_const_pointer = [u8; 0usize];
    pub type multimap_reference = [u8; 0usize];
    pub type multimap_const_reference = [u8; 0usize];
    pub type multimap_iterator = [u8; 0usize];
    pub type multimap_const_iterator = [u8; 0usize];
    pub type multimap_size_type = [u8; 0usize];
    pub type multimap_difference_type = [u8; 0usize];
    pub type multimap_reverse_iterator = [u8; 0usize];
    pub type multimap_const_reverse_iterator = [u8; 0usize];
    pub type multimap_node_type = [u8; 0usize];
    impl Default for multimap {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for multimap {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "multimap {{ _M_t: {:?} }}", self._M_t)
        }
    }
    impl ::std::cmp::PartialEq for multimap {
        fn eq(&self, other: &multimap) -> bool {
            self._M_t == other._M_t
        }
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
    impl Default for _Vector_base__Vector_impl {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Vector_base__Vector_impl {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Vector_base__Vector_impl {{  }}")
        }
    }
    impl ::std::cmp::PartialEq for _Vector_base__Vector_impl {
        fn eq(&self, other: &_Vector_base__Vector_impl) -> bool {
            self._M_start == other._M_start
                && self._M_finish == other._M_finish
                && self._M_end_of_storage == other._M_end_of_storage
        }
    }
    pub type _Vector_base_allocator_type<_Alloc> = _Alloc;
    impl Default for _Vector_base {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Vector_base {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Vector_base {{ _M_impl: {:?} }}", self._M_impl)
        }
    }
    impl ::std::cmp::PartialEq for _Vector_base {
        fn eq(&self, other: &_Vector_base) -> bool {
            self._M_impl == other._M_impl
        }
    }
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
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct vector__Temporary_value {
        pub _M_this: *mut vector,
        pub __buf: u8,
    }
    impl Default for vector__Temporary_value {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl Default for vector {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for vector {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "vector {{  }}")
        }
    }
    impl ::std::cmp::PartialEq for vector {
        fn eq(&self, other: &vector) -> bool {
            self._base == other._base
        }
    }
    pub type _Bit_type = ::std::os::raw::c_ulong;
    pub const std__S_word_bit: _bindgen_ty_1 = _bindgen_ty_1::_S_word_bit;
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum _bindgen_ty_1 {
        _S_word_bit = 64,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Bit_reference {
        pub _M_p: *mut _Bit_type,
        pub _M_mask: _Bit_type,
    }
    extern "C" {
        #[link_name = "\u{1}flip"]
        pub fn _Bit_reference_flip(this: *mut _Bit_reference);
    }
    extern "C" {
        #[link_name = "\u{1}_Bit_reference"]
        pub fn _Bit_reference__Bit_reference(
            this: *mut _Bit_reference,
            __x: *mut _Bit_type,
            __y: _Bit_type,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_Bit_reference"]
        pub fn _Bit_reference__Bit_reference1(this: *mut _Bit_reference);
    }
    impl Default for _Bit_reference {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl _Bit_reference {
        #[inline]
        pub unsafe fn flip(&mut self) {
            _Bit_reference_flip(self)
        }
        #[inline]
        pub unsafe fn new(__x: *mut _Bit_type, __y: _Bit_type) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Bit_reference__Bit_reference(&mut __bindgen_tmp, __x, __y);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Bit_reference__Bit_reference1(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        pub fn swap(__x: _Bit_reference, __y: _Bit_reference);
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Bit_iterator_base {
        pub _M_p: *mut _Bit_type,
        pub _M_offset: ::std::os::raw::c_uint,
    }
    extern "C" {
        #[link_name = "\u{1}_M_bump_up"]
        pub fn _Bit_iterator_base__M_bump_up(this: *mut _Bit_iterator_base);
    }
    extern "C" {
        #[link_name = "\u{1}_M_bump_down"]
        pub fn _Bit_iterator_base__M_bump_down(this: *mut _Bit_iterator_base);
    }
    extern "C" {
        #[link_name = "\u{1}_M_incr"]
        pub fn _Bit_iterator_base__M_incr(this: *mut _Bit_iterator_base, __i: isize);
    }
    extern "C" {
        #[link_name = "\u{1}_Bit_iterator_base"]
        pub fn _Bit_iterator_base__Bit_iterator_base(
            this: *mut _Bit_iterator_base,
            __x: *mut _Bit_type,
            __y: ::std::os::raw::c_uint,
        );
    }
    impl Default for _Bit_iterator_base {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl _Bit_iterator_base {
        #[inline]
        pub unsafe fn _M_bump_up(&mut self) {
            _Bit_iterator_base__M_bump_up(self)
        }
        #[inline]
        pub unsafe fn _M_bump_down(&mut self) {
            _Bit_iterator_base__M_bump_down(self)
        }
        #[inline]
        pub unsafe fn _M_incr(&mut self, __i: isize) {
            _Bit_iterator_base__M_incr(self, __i)
        }
        #[inline]
        pub unsafe fn new(__x: *mut _Bit_type, __y: ::std::os::raw::c_uint) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Bit_iterator_base__Bit_iterator_base(&mut __bindgen_tmp, __x, __y);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Bit_iterator {
        pub _base: _Bit_iterator_base,
    }
    pub type _Bit_iterator_reference = _Bit_reference;
    pub type _Bit_iterator_pointer = *mut _Bit_reference;
    pub type _Bit_iterator_iterator = _Bit_iterator;
    extern "C" {
        #[link_name = "\u{1}_M_const_cast"]
        pub fn _Bit_iterator__M_const_cast(this: *const _Bit_iterator) -> _Bit_iterator_iterator;
    }
    extern "C" {
        #[link_name = "\u{1}_Bit_iterator"]
        pub fn _Bit_iterator__Bit_iterator(this: *mut _Bit_iterator);
    }
    extern "C" {
        #[link_name = "\u{1}_Bit_iterator"]
        pub fn _Bit_iterator__Bit_iterator1(
            this: *mut _Bit_iterator,
            __x: *mut _Bit_type,
            __y: ::std::os::raw::c_uint,
        );
    }
    impl Default for _Bit_iterator {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl _Bit_iterator {
        #[inline]
        pub unsafe fn _M_const_cast(&self) -> _Bit_iterator_iterator {
            _Bit_iterator__M_const_cast(self)
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Bit_iterator__Bit_iterator(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(__x: *mut _Bit_type, __y: ::std::os::raw::c_uint) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Bit_iterator__Bit_iterator1(&mut __bindgen_tmp, __x, __y);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Bit_const_iterator {
        pub _base: _Bit_iterator_base,
    }
    pub type _Bit_const_iterator_reference = bool;
    pub type _Bit_const_iterator_const_reference = bool;
    pub type _Bit_const_iterator_pointer = *const bool;
    pub type _Bit_const_iterator_const_iterator = _Bit_const_iterator;
    extern "C" {
        #[link_name = "\u{1}_M_const_cast"]
        pub fn _Bit_const_iterator__M_const_cast(this: *const _Bit_const_iterator)
            -> _Bit_iterator;
    }
    extern "C" {
        #[link_name = "\u{1}_Bit_const_iterator"]
        pub fn _Bit_const_iterator__Bit_const_iterator(this: *mut _Bit_const_iterator);
    }
    extern "C" {
        #[link_name = "\u{1}_Bit_const_iterator"]
        pub fn _Bit_const_iterator__Bit_const_iterator1(
            this: *mut _Bit_const_iterator,
            __x: *mut _Bit_type,
            __y: ::std::os::raw::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_Bit_const_iterator"]
        pub fn _Bit_const_iterator__Bit_const_iterator2(
            this: *mut _Bit_const_iterator,
            __x: *const _Bit_iterator,
        );
    }
    impl Default for _Bit_const_iterator {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl _Bit_const_iterator {
        #[inline]
        pub unsafe fn _M_const_cast(&self) -> _Bit_iterator {
            _Bit_const_iterator__M_const_cast(self)
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Bit_const_iterator__Bit_const_iterator(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(__x: *mut _Bit_type, __y: ::std::os::raw::c_uint) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Bit_const_iterator__Bit_const_iterator1(&mut __bindgen_tmp, __x, __y);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new2(__x: *const _Bit_iterator) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            _Bit_const_iterator__Bit_const_iterator2(&mut __bindgen_tmp, __x);
            __bindgen_tmp
        }
    }
    extern "C" {
        pub fn __fill_bvector(
            __v: *mut _Bit_type,
            __first: ::std::os::raw::c_uint,
            __last: ::std::os::raw::c_uint,
            __x: bool,
        );
    }
    extern "C" {
        pub fn fill(__first: _Bit_iterator, __last: _Bit_iterator, __x: *const bool);
    }
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
    impl Default for _Bvector_base__Bvector_impl_data {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Bvector_base__Bvector_impl_data {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "_Bvector_base__Bvector_impl_data {{ _M_start: {:?}, _M_finish: {:?} }}",
                self._M_start, self._M_finish
            )
        }
    }
    impl ::std::cmp::PartialEq for _Bvector_base__Bvector_impl_data {
        fn eq(&self, other: &_Bvector_base__Bvector_impl_data) -> bool {
            self._M_start == other._M_start
                && self._M_finish == other._M_finish
                && self._M_end_of_storage == other._M_end_of_storage
        }
    }
    #[repr(C)]
    pub struct _Bvector_base__Bvector_impl {
        pub _base_1: _Bvector_base__Bvector_impl_data,
    }
    impl Default for _Bvector_base__Bvector_impl {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Bvector_base__Bvector_impl {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Bvector_base__Bvector_impl {{  }}")
        }
    }
    impl ::std::cmp::PartialEq for _Bvector_base__Bvector_impl {
        fn eq(&self, other: &_Bvector_base__Bvector_impl) -> bool {
            self._base_1 == other._base_1
        }
    }
    pub type _Bvector_base_allocator_type<_Alloc> = _Alloc;
    impl Default for _Bvector_base {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Bvector_base {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Bvector_base {{ _M_impl: {:?} }}", self._M_impl)
        }
    }
    impl ::std::cmp::PartialEq for _Bvector_base {
        fn eq(&self, other: &_Bvector_base) -> bool {
            self._M_impl == other._M_impl
        }
    }
    extern "C" {
        pub fn __deque_buf_size(__size: usize) -> usize;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
    impl Default for _Deque_iterator {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
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
    impl Default for _Deque_base__Deque_impl {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
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
    impl ::std::cmp::PartialEq for _Deque_base__Deque_impl {
        fn eq(&self, other: &_Deque_base__Deque_impl) -> bool {
            self._M_map == other._M_map
                && self._M_map_size == other._M_map_size
                && self._M_start == other._M_start
                && self._M_finish == other._M_finish
        }
    }
    pub const _Deque_base__S_initial_map_size: _Deque_base__bindgen_ty_1 =
        _Deque_base__bindgen_ty_1::_S_initial_map_size;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum _Deque_base__bindgen_ty_1 {
        _S_initial_map_size = 0,
    }
    impl Default for _Deque_base {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for _Deque_base {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "_Deque_base {{ _M_impl: {:?} }}", self._M_impl)
        }
    }
    impl ::std::cmp::PartialEq for _Deque_base {
        fn eq(&self, other: &_Deque_base) -> bool {
            self._M_impl == other._M_impl
        }
    }
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
    impl Default for deque {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for deque {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "deque {{  }}")
        }
    }
    impl ::std::cmp::PartialEq for deque {
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
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct priority_queue<_Sequence, _Compare> {
        pub c: _Sequence,
        pub comp: _Compare,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Sequence>>,
        pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Compare>>,
    }
    pub type priority_queue__Uses = u8;
    pub type priority_queue_value_type = [u8; 0usize];
    pub type priority_queue_reference = [u8; 0usize];
    pub type priority_queue_const_reference = [u8; 0usize];
    pub type priority_queue_size_type = [u8; 0usize];
    pub type priority_queue_container_type<_Sequence> = _Sequence;
    pub type priority_queue_value_compare<_Compare> = _Compare;
    impl<_Sequence, _Compare> Default for priority_queue<_Sequence, _Compare> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct locale {
        pub _M_impl: *mut locale__Impl,
    }
    pub type locale_category = ::std::os::raw::c_int;
    pub const locale__S_categories_size: locale__bindgen_ty_1 =
        locale__bindgen_ty_1::_S_categories_size;
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum locale__bindgen_ty_1 {
        _S_categories_size = 12,
    }
    pub const locale_none: locale_category = 0;
    pub const locale_ctype: locale_category = 1;
    pub const locale_numeric: locale_category = 2;
    pub const locale_collate: locale_category = 4;
    pub const locale_time: locale_category = 8;
    pub const locale_monetary: locale_category = 16;
    pub const locale_messages: locale_category = 32;
    pub const locale_all: locale_category = 63;
    extern "C" {
        #[link_name = "\u{1}_S_classic"]
        pub static mut locale__S_classic: *mut locale__Impl;
    }
    extern "C" {
        #[link_name = "\u{1}_S_global"]
        pub static mut locale__S_global: *mut locale__Impl;
    }
    extern "C" {
        #[link_name = "\u{1}_S_categories"]
        pub static locale__S_categories: *const *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}_S_once"]
        pub static mut locale__S_once: __gthread_once_t;
    }
    extern "C" {
        #[link_name = "\u{1}_S_twinned_facets"]
        pub static mut locale__S_twinned_facets: [*const locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}name"]
        pub fn locale_name(this: *const locale) -> string;
    }
    extern "C" {
        #[link_name = "\u{1}global"]
        pub fn locale_global(__loc: *const locale) -> locale;
    }
    extern "C" {
        #[link_name = "\u{1}classic"]
        pub fn locale_classic() -> *const locale;
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale(this: *mut locale);
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale1(this: *mut locale, __other: *const locale);
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale2(this: *mut locale, __s: *const ::std::os::raw::c_char);
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale3(
            this: *mut locale,
            __base: *const locale,
            __s: *const ::std::os::raw::c_char,
            __cat: locale_category,
        );
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale4(this: *mut locale, __s: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale5(
            this: *mut locale,
            __base: *const locale,
            __s: *const string,
            __cat: locale_category,
        );
    }
    extern "C" {
        #[link_name = "\u{1}locale"]
        pub fn locale_locale6(
            this: *mut locale,
            __base: *const locale,
            __add: *const locale,
            __cat: locale_category,
        );
    }
    extern "C" {
        #[link_name = "\u{1}locale_destructor"]
        pub fn locale_locale_destructor(this: *mut locale);
    }
    impl Default for locale {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
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
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_locale(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(__other: *const locale) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_locale1(&mut __bindgen_tmp, __other);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new2(__s: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_locale2(&mut __bindgen_tmp, __s);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new3(
            __base: *const locale,
            __s: *const ::std::os::raw::c_char,
            __cat: locale_category,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_locale3(&mut __bindgen_tmp, __base, __s, __cat);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new4(__s: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_locale4(&mut __bindgen_tmp, __s);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new5(
            __base: *const locale,
            __s: *const string,
            __cat: locale_category,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_locale5(&mut __bindgen_tmp, __base, __s, __cat);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new6(
            __base: *const locale,
            __add: *const locale,
            __cat: locale_category,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_locale6(&mut __bindgen_tmp, __base, __add, __cat);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn destruct(&mut self) {
            locale_locale_destructor(self)
        }
    }
    #[repr(C)]
    pub struct locale_facet__bindgen_vtable(::std::os::raw::c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
        #[link_name = "\u{1}_S_c_locale"]
        pub static mut locale_facet__S_c_locale: __c_locale;
    }
    extern "C" {
        #[link_name = "\u{1}_S_c_name"]
        pub static mut locale_facet__S_c_name: [::std::os::raw::c_char; 2usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_once"]
        pub static mut locale_facet__S_once: __gthread_once_t;
    }
    extern "C" {
        #[link_name = "\u{1}_S_create_c_locale"]
        pub fn locale_facet__S_create_c_locale(
            __cloc: *mut __c_locale,
            __s: *const ::std::os::raw::c_char,
            __old: __c_locale,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_S_clone_c_locale"]
        pub fn locale_facet__S_clone_c_locale(__cloc: *mut __c_locale) -> __c_locale;
    }
    extern "C" {
        #[link_name = "\u{1}_S_destroy_c_locale"]
        pub fn locale_facet__S_destroy_c_locale(__cloc: *mut __c_locale);
    }
    extern "C" {
        #[link_name = "\u{1}_S_lc_ctype_c_locale"]
        pub fn locale_facet__S_lc_ctype_c_locale(
            __cloc: __c_locale,
            __s: *const ::std::os::raw::c_char,
        ) -> __c_locale;
    }
    extern "C" {
        #[link_name = "\u{1}_S_get_c_locale"]
        pub fn locale_facet__S_get_c_locale() -> __c_locale;
    }
    extern "C" {
        #[link_name = "\u{1}_S_get_c_name"]
        pub fn locale_facet__S_get_c_name() -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}facet"]
        pub fn locale_facet_facet(this: *mut locale_facet, __refs: usize);
    }
    extern "C" {
        #[link_name = "\u{1}facet"]
        pub fn locale_facet_facet1(this: *mut locale_facet, arg1: *const locale_facet);
    }
    impl Default for locale_facet {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl locale_facet {
        #[inline]
        pub unsafe fn _S_create_c_locale(
            __cloc: *mut __c_locale,
            __s: *const ::std::os::raw::c_char,
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
        pub unsafe fn _S_lc_ctype_c_locale(
            __cloc: __c_locale,
            __s: *const ::std::os::raw::c_char,
        ) -> __c_locale {
            locale_facet__S_lc_ctype_c_locale(__cloc, __s)
        }
        #[inline]
        pub unsafe fn _S_get_c_locale() -> __c_locale {
            locale_facet__S_get_c_locale()
        }
        #[inline]
        pub unsafe fn _S_get_c_name() -> *const ::std::os::raw::c_char {
            locale_facet__S_get_c_name()
        }
        #[inline]
        pub unsafe fn new(__refs: usize) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_facet_facet(&mut __bindgen_tmp, __refs);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const locale_facet) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_facet_facet1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}facet_destructor"]
        pub fn locale_facet_facet_destructor(this: *mut locale_facet);
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct locale_id {
        pub _M_index: usize,
    }
    extern "C" {
        #[link_name = "\u{1}_S_refcount"]
        pub static mut locale_id__S_refcount: _Atomic_word;
    }
    extern "C" {
        #[link_name = "\u{1}_M_id"]
        pub fn locale_id__M_id(this: *const locale_id) -> usize;
    }
    extern "C" {
        #[link_name = "\u{1}id"]
        pub fn locale_id_id(this: *mut locale_id);
    }
    impl locale_id {
        #[inline]
        pub unsafe fn _M_id(&self) -> usize {
            locale_id__M_id(self)
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            locale_id_id(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct locale__Impl {
        pub _M_refcount: _Atomic_word,
        pub _M_facets: *mut *const locale_facet,
        pub _M_facets_size: usize,
        pub _M_caches: *mut *const locale_facet,
        pub _M_names: *mut *mut ::std::os::raw::c_char,
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_ctype"]
        pub static mut locale__Impl__S_id_ctype: [*const locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_numeric"]
        pub static mut locale__Impl__S_id_numeric: [*const locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_collate"]
        pub static mut locale__Impl__S_id_collate: [*const locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_time"]
        pub static mut locale__Impl__S_id_time: [*const locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_monetary"]
        pub static mut locale__Impl__S_id_monetary: [*const locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_id_messages"]
        pub static mut locale__Impl__S_id_messages: [*const locale_id; 0usize];
    }
    extern "C" {
        #[link_name = "\u{1}_S_facet_categories"]
        pub static mut locale__Impl__S_facet_categories: [*const *const locale_id; 0usize];
    }
    impl Default for locale__Impl {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type collate_char_type<_CharT> = _CharT;
    pub type collate_string_type = basic_string<_CharT>;
    extern "C" {
        pub static mut id: locale_id;
    }
    pub type collate_byname_char_type<_CharT> = _CharT;
    pub type collate_byname_string_type = basic_string<_CharT>;
    impl errc {
        pub const operation_not_supported: errc = errc::not_supported;
    }
    impl errc {
        pub const resource_unavailable_try_again: errc = errc::operation_would_block;
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum errc {
        address_family_not_supported = 97,
        address_in_use = 98,
        address_not_available = 99,
        already_connected = 106,
        argument_list_too_long = 7,
        argument_out_of_domain = 33,
        bad_address = 14,
        bad_file_descriptor = 9,
        bad_message = 74,
        broken_pipe = 32,
        connection_aborted = 103,
        connection_already_in_progress = 114,
        connection_refused = 111,
        connection_reset = 104,
        cross_device_link = 18,
        destination_address_required = 89,
        device_or_resource_busy = 16,
        directory_not_empty = 39,
        executable_format_error = 8,
        file_exists = 17,
        file_too_large = 27,
        filename_too_long = 36,
        function_not_supported = 38,
        host_unreachable = 113,
        identifier_removed = 43,
        illegal_byte_sequence = 84,
        inappropriate_io_control_operation = 25,
        interrupted = 4,
        invalid_argument = 22,
        invalid_seek = 29,
        io_error = 5,
        is_a_directory = 21,
        message_size = 90,
        network_down = 100,
        network_reset = 102,
        network_unreachable = 101,
        no_buffer_space = 105,
        no_child_process = 10,
        no_link = 67,
        no_lock_available = 37,
        no_message_available = 61,
        no_message = 42,
        no_protocol_option = 92,
        no_space_on_device = 28,
        no_stream_resources = 63,
        no_such_device_or_address = 6,
        no_such_device = 19,
        no_such_file_or_directory = 2,
        no_such_process = 3,
        not_a_directory = 20,
        not_a_socket = 88,
        not_a_stream = 60,
        not_connected = 107,
        not_enough_memory = 12,
        not_supported = 95,
        operation_canceled = 125,
        operation_in_progress = 115,
        operation_not_permitted = 1,
        operation_would_block = 11,
        owner_dead = 130,
        permission_denied = 13,
        protocol_error = 71,
        protocol_not_supported = 93,
        read_only_file_system = 30,
        resource_deadlock_would_occur = 35,
        result_out_of_range = 34,
        state_not_recoverable = 131,
        stream_timeout = 62,
        text_file_busy = 26,
        timed_out = 110,
        too_many_files_open_in_system = 23,
        too_many_files_open = 24,
        too_many_links = 31,
        too_many_symbolic_link_levels = 40,
        value_too_large = 75,
        wrong_protocol_type = 91,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_error_code_enum {
        pub _base: false_type,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct is_error_condition_enum {
        pub _base: false_type,
    }
    pub mod _V2 {

        #[repr(C)]
        pub struct error_category__bindgen_vtable(::std::os::raw::c_void);
        #[repr(C)]
        #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct error_category {
            pub vtable_: *const error_category__bindgen_vtable,
        }
        extern "C" {
            #[link_name = "\u{1}error_category"]
            pub fn error_category_error_category(this: *mut error_category);
        }
        extern "C" {
            #[link_name = "\u{1}error_category"]
            pub fn error_category_error_category1(
                this: *mut error_category,
                arg1: *const error_category,
            );
        }
        impl Default for error_category {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        impl error_category {
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                error_category_error_category(&mut __bindgen_tmp);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new1(arg1: *const error_category) -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                error_category_error_category1(&mut __bindgen_tmp, arg1);
                __bindgen_tmp
            }
        }
        extern "C" {
            #[link_name = "\u{1}error_category_destructor"]
            pub fn error_category_error_category_destructor(this: *mut error_category);
        }
        extern "C" {
            #[link_name = "\u{1}default_error_condition"]
            pub fn error_category_default_error_condition(
                this: *mut ::std::os::raw::c_void,
                __i: ::std::os::raw::c_int,
            ) -> error_condition;
        }
        extern "C" {
            #[link_name = "\u{1}equivalent"]
            pub fn error_category_equivalent(
                this: *mut ::std::os::raw::c_void,
                __i: ::std::os::raw::c_int,
                __cond: *const error_condition,
            ) -> bool;
        }
        extern "C" {
            #[link_name = "\u{1}equivalent"]
            pub fn error_category_equivalent1(
                this: *mut ::std::os::raw::c_void,
                __code: *const error_code,
                __i: ::std::os::raw::c_int,
            ) -> bool;
        }
        extern "C" {
            pub fn system_category() -> *const error_category;
        }
        extern "C" {
            pub fn generic_category() -> *const error_category;
        }
    }
    extern "C" {
        pub fn make_error_code(arg1: errc) -> error_code;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct error_code {
        pub _M_value: ::std::os::raw::c_int,
        pub _M_cat: *const error_category,
    }
    extern "C" {
        #[link_name = "\u{1}assign"]
        pub fn error_code_assign(
            this: *mut error_code,
            __v: ::std::os::raw::c_int,
            __cat: *const error_category,
        );
    }
    extern "C" {
        #[link_name = "\u{1}clear"]
        pub fn error_code_clear(this: *mut error_code);
    }
    extern "C" {
        #[link_name = "\u{1}value"]
        pub fn error_code_value(this: *const error_code) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}category"]
        pub fn error_code_category(this: *const error_code) -> *const error_category;
    }
    extern "C" {
        #[link_name = "\u{1}default_error_condition"]
        pub fn error_code_default_error_condition(this: *const error_code) -> error_condition;
    }
    extern "C" {
        #[link_name = "\u{1}message"]
        pub fn error_code_message(this: *const error_code) -> string;
    }
    extern "C" {
        #[link_name = "\u{1}error_code"]
        pub fn error_code_error_code(this: *mut error_code);
    }
    extern "C" {
        #[link_name = "\u{1}error_code"]
        pub fn error_code_error_code1(
            this: *mut error_code,
            __v: ::std::os::raw::c_int,
            __cat: *const error_category,
        );
    }
    impl Default for error_code {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl error_code {
        #[inline]
        pub unsafe fn assign(&mut self, __v: ::std::os::raw::c_int, __cat: *const error_category) {
            error_code_assign(self, __v, __cat)
        }
        #[inline]
        pub unsafe fn clear(&mut self) {
            error_code_clear(self)
        }
        #[inline]
        pub unsafe fn value(&self) -> ::std::os::raw::c_int {
            error_code_value(self)
        }
        #[inline]
        pub unsafe fn category(&self) -> *const error_category {
            error_code_category(self)
        }
        #[inline]
        pub unsafe fn default_error_condition(&self) -> error_condition {
            error_code_default_error_condition(self)
        }
        #[inline]
        pub unsafe fn message(&self) -> string {
            error_code_message(self)
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            error_code_error_code(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(__v: ::std::os::raw::c_int, __cat: *const error_category) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            error_code_error_code1(&mut __bindgen_tmp, __v, __cat);
            __bindgen_tmp
        }
    }
    extern "C" {
        pub fn make_error_condition(arg1: errc) -> error_condition;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct error_condition {
        pub _M_value: ::std::os::raw::c_int,
        pub _M_cat: *const error_category,
    }
    extern "C" {
        #[link_name = "\u{1}assign"]
        pub fn error_condition_assign(
            this: *mut error_condition,
            __v: ::std::os::raw::c_int,
            __cat: *const error_category,
        );
    }
    extern "C" {
        #[link_name = "\u{1}clear"]
        pub fn error_condition_clear(this: *mut error_condition);
    }
    extern "C" {
        #[link_name = "\u{1}value"]
        pub fn error_condition_value(this: *const error_condition) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}category"]
        pub fn error_condition_category(this: *const error_condition) -> *const error_category;
    }
    extern "C" {
        #[link_name = "\u{1}message"]
        pub fn error_condition_message(this: *const error_condition) -> string;
    }
    extern "C" {
        #[link_name = "\u{1}error_condition"]
        pub fn error_condition_error_condition(this: *mut error_condition);
    }
    extern "C" {
        #[link_name = "\u{1}error_condition"]
        pub fn error_condition_error_condition1(
            this: *mut error_condition,
            __v: ::std::os::raw::c_int,
            __cat: *const error_category,
        );
    }
    impl Default for error_condition {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl error_condition {
        #[inline]
        pub unsafe fn assign(&mut self, __v: ::std::os::raw::c_int, __cat: *const error_category) {
            error_condition_assign(self, __v, __cat)
        }
        #[inline]
        pub unsafe fn clear(&mut self) {
            error_condition_clear(self)
        }
        #[inline]
        pub unsafe fn value(&self) -> ::std::os::raw::c_int {
            error_condition_value(self)
        }
        #[inline]
        pub unsafe fn category(&self) -> *const error_category {
            error_condition_category(self)
        }
        #[inline]
        pub unsafe fn message(&self) -> string {
            error_condition_message(self)
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            error_condition_error_condition(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(__v: ::std::os::raw::c_int, __cat: *const error_category) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            error_condition_error_condition1(&mut __bindgen_tmp, __v, __cat);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    pub struct system_error {
        pub _base: runtime_error,
        pub _M_code: error_code,
    }
    extern "C" {
        #[link_name = "\u{1}code"]
        pub fn system_error_code(this: *const system_error) -> *const error_code;
    }
    extern "C" {
        #[link_name = "\u{1}system_error"]
        pub fn system_error_system_error(this: *mut system_error, __ec: error_code);
    }
    extern "C" {
        #[link_name = "\u{1}system_error"]
        pub fn system_error_system_error1(
            this: *mut system_error,
            __ec: error_code,
            __what: *const string,
        );
    }
    extern "C" {
        #[link_name = "\u{1}system_error"]
        pub fn system_error_system_error2(
            this: *mut system_error,
            __ec: error_code,
            __what: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        #[link_name = "\u{1}system_error"]
        pub fn system_error_system_error3(
            this: *mut system_error,
            __v: ::std::os::raw::c_int,
            __ecat: *const error_category,
            __what: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        #[link_name = "\u{1}system_error"]
        pub fn system_error_system_error4(
            this: *mut system_error,
            __v: ::std::os::raw::c_int,
            __ecat: *const error_category,
        );
    }
    extern "C" {
        #[link_name = "\u{1}system_error"]
        pub fn system_error_system_error5(
            this: *mut system_error,
            __v: ::std::os::raw::c_int,
            __ecat: *const error_category,
            __what: *const string,
        );
    }
    impl Default for system_error {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for system_error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "system_error {{ _M_code: {:?} }}", self._M_code)
        }
    }
    impl system_error {
        #[inline]
        pub unsafe fn code(&self) -> *const error_code {
            system_error_code(self)
        }
        #[inline]
        pub unsafe fn new(__ec: error_code) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            system_error_system_error(&mut __bindgen_tmp, __ec);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(__ec: error_code, __what: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            system_error_system_error1(&mut __bindgen_tmp, __ec, __what);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new2(__ec: error_code, __what: *const ::std::os::raw::c_char) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            system_error_system_error2(&mut __bindgen_tmp, __ec, __what);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new3(
            __v: ::std::os::raw::c_int,
            __ecat: *const error_category,
            __what: *const ::std::os::raw::c_char,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            system_error_system_error3(&mut __bindgen_tmp, __v, __ecat, __what);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new4(__v: ::std::os::raw::c_int, __ecat: *const error_category) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            system_error_system_error4(&mut __bindgen_tmp, __v, __ecat);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new5(
            __v: ::std::os::raw::c_int,
            __ecat: *const error_category,
            __what: *const string,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            system_error_system_error5(&mut __bindgen_tmp, __v, __ecat, __what);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}system_error_destructor"]
        pub fn system_error_system_error_destructor(this: *mut system_error);
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum _Ios_Fmtflags {
        _S_boolalpha = 1,
        _S_dec = 2,
        _S_fixed = 4,
        _S_hex = 8,
        _S_internal = 16,
        _S_left = 32,
        _S_oct = 64,
        _S_right = 128,
        _S_scientific = 256,
        _S_showbase = 512,
        _S_showpoint = 1024,
        _S_showpos = 2048,
        _S_skipws = 4096,
        _S_unitbuf = 8192,
        _S_uppercase = 16384,
        _S_adjustfield = 176,
        _S_basefield = 74,
        _S_floatfield = 260,
        _S_ios_fmtflags_end = 65536,
        _S_ios_fmtflags_max = 2147483647,
        _S_ios_fmtflags_min = -2147483648,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum _Ios_Openmode {
        _S_app = 1,
        _S_ate = 2,
        _S_bin = 4,
        _S_in = 8,
        _S_out = 16,
        _S_trunc = 32,
        _S_ios_openmode_end = 65536,
        _S_ios_openmode_max = 2147483647,
        _S_ios_openmode_min = -2147483648,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum _Ios_Iostate {
        _S_goodbit = 0,
        _S_badbit = 1,
        _S_eofbit = 2,
        _S_failbit = 4,
        _S_ios_iostate_end = 65536,
        _S_ios_iostate_max = 2147483647,
        _S_ios_iostate_min = -2147483648,
    }
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum _Ios_Seekdir {
        _S_beg = 0,
        _S_cur = 1,
        _S_end = 2,
        _S_ios_seekdir_end = 65536,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum io_errc {
        stream = 1,
    }
    extern "C" {
        pub fn iostream_category() -> *const error_category;
    }
    #[repr(C)]
    pub struct ios_base__bindgen_vtable(::std::os::raw::c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
        pub _M_word_size: ::std::os::raw::c_int,
        pub _M_word: *mut ios_base__Words,
        pub _M_ios_locale: locale,
    }
    #[repr(C)]
    pub struct ios_base_failure {
        pub _base: system_error,
    }
    extern "C" {
        #[link_name = "\u{1}failure"]
        pub fn ios_base_failure_failure(this: *mut ios_base_failure, __str: *const string);
    }
    extern "C" {
        #[link_name = "\u{1}failure"]
        pub fn ios_base_failure_failure1(
            this: *mut ios_base_failure,
            arg1: *const string,
            arg2: *const error_code,
        );
    }
    extern "C" {
        #[link_name = "\u{1}failure"]
        pub fn ios_base_failure_failure2(
            this: *mut ios_base_failure,
            arg1: *const ::std::os::raw::c_char,
            arg2: *const error_code,
        );
    }
    impl Default for ios_base_failure {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ::std::fmt::Debug for ios_base_failure {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "ios_base_failure {{  }}")
        }
    }
    impl ios_base_failure {
        #[inline]
        pub unsafe fn new(__str: *const string) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            ios_base_failure_failure(&mut __bindgen_tmp, __str);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const string, arg2: *const error_code) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            ios_base_failure_failure1(&mut __bindgen_tmp, arg1, arg2);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new2(arg1: *const ::std::os::raw::c_char, arg2: *const error_code) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            ios_base_failure_failure2(&mut __bindgen_tmp, arg1, arg2);
            __bindgen_tmp
        }
    }
    pub use _Ios_Fmtflags as ios_base_fmtflags;
    pub use _Ios_Iostate as ios_base_iostate;
    pub use _Ios_Openmode as ios_base_openmode;
    pub use _Ios_Seekdir as ios_base_seekdir;
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum ios_base_event {
        erase_event = 0,
        imbue_event = 1,
        copyfmt_event = 2,
    }
    pub type ios_base_event_callback = ::std::option::Option<
        unsafe extern "C" fn(__e: ios_base_event, __b: *mut ios_base, __i: ::std::os::raw::c_int),
    >;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ios_base__Callback_list {
        pub _M_next: *mut ios_base__Callback_list,
        pub _M_fn: ios_base_event_callback,
        pub _M_index: ::std::os::raw::c_int,
        pub _M_refcount: _Atomic_word,
    }
    extern "C" {
        #[link_name = "\u{1}_M_add_reference"]
        pub fn ios_base__Callback_list__M_add_reference(this: *mut ios_base__Callback_list);
    }
    extern "C" {
        #[link_name = "\u{1}_M_remove_reference"]
        pub fn ios_base__Callback_list__M_remove_reference(
            this: *mut ios_base__Callback_list,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}_Callback_list"]
        pub fn ios_base__Callback_list__Callback_list(
            this: *mut ios_base__Callback_list,
            __fn: ios_base_event_callback,
            __index: ::std::os::raw::c_int,
            __cb: *mut ios_base__Callback_list,
        );
    }
    impl Default for ios_base__Callback_list {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ios_base__Callback_list {
        #[inline]
        pub unsafe fn _M_add_reference(&mut self) {
            ios_base__Callback_list__M_add_reference(self)
        }
        #[inline]
        pub unsafe fn _M_remove_reference(&mut self) -> ::std::os::raw::c_int {
            ios_base__Callback_list__M_remove_reference(self)
        }
        #[inline]
        pub unsafe fn new(
            __fn: ios_base_event_callback,
            __index: ::std::os::raw::c_int,
            __cb: *mut ios_base__Callback_list,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            ios_base__Callback_list__Callback_list(&mut __bindgen_tmp, __fn, __index, __cb);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ios_base__Words {
        pub _M_pword: *mut ::std::os::raw::c_void,
        pub _M_iword: ::std::os::raw::c_long,
    }
    extern "C" {
        #[link_name = "\u{1}_Words"]
        pub fn ios_base__Words__Words(this: *mut ios_base__Words);
    }
    impl Default for ios_base__Words {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ios_base__Words {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            ios_base__Words__Words(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    pub const ios_base__S_local_word_size: ios_base__bindgen_ty_1 =
        ios_base__bindgen_ty_1::_S_local_word_size;
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum ios_base__bindgen_ty_1 {
        _S_local_word_size = 8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ios_base_Init {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}_S_refcount"]
        pub static mut ios_base_Init__S_refcount: _Atomic_word;
    }
    extern "C" {
        #[link_name = "\u{1}_S_synced_with_stdio"]
        pub static mut ios_base_Init__S_synced_with_stdio: bool;
    }
    extern "C" {
        #[link_name = "\u{1}Init"]
        pub fn ios_base_Init_Init(this: *mut ios_base_Init);
    }
    extern "C" {
        #[link_name = "\u{1}Init_destructor"]
        pub fn ios_base_Init_Init_destructor(this: *mut ios_base_Init);
    }
    impl ios_base_Init {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
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
        pub static ios_base_boolalpha: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}dec"]
        pub static ios_base_dec: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}fixed"]
        pub static ios_base_fixed: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}hex"]
        pub static ios_base_hex: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}internal"]
        pub static ios_base_internal: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}left"]
        pub static ios_base_left: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}oct"]
        pub static ios_base_oct: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}right"]
        pub static ios_base_right: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}scientific"]
        pub static ios_base_scientific: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}showbase"]
        pub static ios_base_showbase: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}showpoint"]
        pub static ios_base_showpoint: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}showpos"]
        pub static ios_base_showpos: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}skipws"]
        pub static ios_base_skipws: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}unitbuf"]
        pub static ios_base_unitbuf: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}uppercase"]
        pub static ios_base_uppercase: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}adjustfield"]
        pub static ios_base_adjustfield: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}basefield"]
        pub static ios_base_basefield: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}floatfield"]
        pub static ios_base_floatfield: ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}badbit"]
        pub static ios_base_badbit: ios_base_iostate;
    }
    extern "C" {
        #[link_name = "\u{1}eofbit"]
        pub static ios_base_eofbit: ios_base_iostate;
    }
    extern "C" {
        #[link_name = "\u{1}failbit"]
        pub static ios_base_failbit: ios_base_iostate;
    }
    extern "C" {
        #[link_name = "\u{1}goodbit"]
        pub static ios_base_goodbit: ios_base_iostate;
    }
    extern "C" {
        #[link_name = "\u{1}app"]
        pub static ios_base_app: ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}ate"]
        pub static ios_base_ate: ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}binary"]
        pub static ios_base_binary: ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}in"]
        pub static ios_base_in: ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}out"]
        pub static ios_base_out: ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}trunc"]
        pub static ios_base_trunc: ios_base_openmode;
    }
    extern "C" {
        #[link_name = "\u{1}beg"]
        pub static ios_base_beg: ios_base_seekdir;
    }
    extern "C" {
        #[link_name = "\u{1}cur"]
        pub static ios_base_cur: ios_base_seekdir;
    }
    extern "C" {
        #[link_name = "\u{1}end"]
        pub static ios_base_end: ios_base_seekdir;
    }
    extern "C" {
        #[link_name = "\u{1}register_callback"]
        pub fn ios_base_register_callback(
            this: *mut ios_base,
            __fn: ios_base_event_callback,
            __index: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_M_call_callbacks"]
        pub fn ios_base__M_call_callbacks(this: *mut ios_base, __ev: ios_base_event);
    }
    extern "C" {
        #[link_name = "\u{1}_M_dispose_callbacks"]
        pub fn ios_base__M_dispose_callbacks(this: *mut ios_base);
    }
    extern "C" {
        #[link_name = "\u{1}_M_grow_words"]
        pub fn ios_base__M_grow_words(
            this: *mut ios_base,
            __index: ::std::os::raw::c_int,
            __iword: bool,
        ) -> *mut ios_base__Words;
    }
    extern "C" {
        #[link_name = "\u{1}_M_init"]
        pub fn ios_base__M_init(this: *mut ios_base);
    }
    extern "C" {
        #[link_name = "\u{1}flags"]
        pub fn ios_base_flags(this: *const ios_base) -> ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}flags"]
        pub fn ios_base_flags1(
            this: *mut ios_base,
            __fmtfl: ios_base_fmtflags,
        ) -> ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}setf"]
        pub fn ios_base_setf(this: *mut ios_base, __fmtfl: ios_base_fmtflags) -> ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}setf"]
        pub fn ios_base_setf1(
            this: *mut ios_base,
            __fmtfl: ios_base_fmtflags,
            __mask: ios_base_fmtflags,
        ) -> ios_base_fmtflags;
    }
    extern "C" {
        #[link_name = "\u{1}unsetf"]
        pub fn ios_base_unsetf(this: *mut ios_base, __mask: ios_base_fmtflags);
    }
    extern "C" {
        #[link_name = "\u{1}precision"]
        pub fn ios_base_precision(this: *const ios_base) -> streamsize;
    }
    extern "C" {
        #[link_name = "\u{1}precision"]
        pub fn ios_base_precision1(this: *mut ios_base, __prec: streamsize) -> streamsize;
    }
    extern "C" {
        #[link_name = "\u{1}width"]
        pub fn ios_base_width(this: *const ios_base) -> streamsize;
    }
    extern "C" {
        #[link_name = "\u{1}width"]
        pub fn ios_base_width1(this: *mut ios_base, __wide: streamsize) -> streamsize;
    }
    extern "C" {
        #[link_name = "\u{1}sync_with_stdio"]
        pub fn ios_base_sync_with_stdio(__sync: bool) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}imbue"]
        pub fn ios_base_imbue(this: *mut ios_base, __loc: *const locale) -> locale;
    }
    extern "C" {
        #[link_name = "\u{1}getloc"]
        pub fn ios_base_getloc(this: *const ios_base) -> locale;
    }
    extern "C" {
        #[link_name = "\u{1}_M_getloc"]
        pub fn ios_base__M_getloc(this: *const ios_base) -> *const locale;
    }
    extern "C" {
        #[link_name = "\u{1}xalloc"]
        pub fn ios_base_xalloc() -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}iword"]
        pub fn ios_base_iword(
            this: *mut ios_base,
            __ix: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_long;
    }
    extern "C" {
        #[link_name = "\u{1}pword"]
        pub fn ios_base_pword(
            this: *mut ios_base,
            __ix: ::std::os::raw::c_int,
        ) -> *mut *mut ::std::os::raw::c_void;
    }
    extern "C" {
        #[link_name = "\u{1}_M_move"]
        pub fn ios_base__M_move(this: *mut ios_base, arg1: *mut ios_base);
    }
    extern "C" {
        #[link_name = "\u{1}_M_swap"]
        pub fn ios_base__M_swap(this: *mut ios_base, __rhs: *mut ios_base);
    }
    extern "C" {
        #[link_name = "\u{1}ios_base"]
        pub fn ios_base_ios_base(this: *mut ios_base);
    }
    extern "C" {
        #[link_name = "\u{1}ios_base"]
        pub fn ios_base_ios_base1(this: *mut ios_base, arg1: *const ios_base);
    }
    impl Default for ios_base {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl ios_base {
        #[inline]
        pub unsafe fn register_callback(
            &mut self,
            __fn: ios_base_event_callback,
            __index: ::std::os::raw::c_int,
        ) {
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
        pub unsafe fn _M_grow_words(
            &mut self,
            __index: ::std::os::raw::c_int,
            __iword: bool,
        ) -> *mut ios_base__Words {
            ios_base__M_grow_words(self, __index, __iword)
        }
        #[inline]
        pub unsafe fn _M_init(&mut self) {
            ios_base__M_init(self)
        }
        #[inline]
        pub unsafe fn flags(&self) -> ios_base_fmtflags {
            ios_base_flags(self)
        }
        #[inline]
        pub unsafe fn flags1(&mut self, __fmtfl: ios_base_fmtflags) -> ios_base_fmtflags {
            ios_base_flags1(self, __fmtfl)
        }
        #[inline]
        pub unsafe fn setf(&mut self, __fmtfl: ios_base_fmtflags) -> ios_base_fmtflags {
            ios_base_setf(self, __fmtfl)
        }
        #[inline]
        pub unsafe fn setf1(
            &mut self,
            __fmtfl: ios_base_fmtflags,
            __mask: ios_base_fmtflags,
        ) -> ios_base_fmtflags {
            ios_base_setf1(self, __fmtfl, __mask)
        }
        #[inline]
        pub unsafe fn unsetf(&mut self, __mask: ios_base_fmtflags) {
            ios_base_unsetf(self, __mask)
        }
        #[inline]
        pub unsafe fn precision(&self) -> streamsize {
            ios_base_precision(self)
        }
        #[inline]
        pub unsafe fn precision1(&mut self, __prec: streamsize) -> streamsize {
            ios_base_precision1(self, __prec)
        }
        #[inline]
        pub unsafe fn width(&self) -> streamsize {
            ios_base_width(self)
        }
        #[inline]
        pub unsafe fn width1(&mut self, __wide: streamsize) -> streamsize {
            ios_base_width1(self, __wide)
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
        pub unsafe fn getloc(&self) -> locale {
            ios_base_getloc(self)
        }
        #[inline]
        pub unsafe fn _M_getloc(&self) -> *const locale {
            ios_base__M_getloc(self)
        }
        #[inline]
        pub unsafe fn xalloc() -> ::std::os::raw::c_int {
            ios_base_xalloc()
        }
        #[inline]
        pub unsafe fn iword(&mut self, __ix: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_long {
            ios_base_iword(self, __ix)
        }
        #[inline]
        pub unsafe fn pword(
            &mut self,
            __ix: ::std::os::raw::c_int,
        ) -> *mut *mut ::std::os::raw::c_void {
            ios_base_pword(self, __ix)
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
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            ios_base_ios_base(&mut __bindgen_tmp);
            __bindgen_tmp
        }
        #[inline]
        pub unsafe fn new1(arg1: *const ios_base) -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            ios_base_ios_base1(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}failure_destructor"]
        pub fn ios_base_failure_failure_destructor(this: *mut ios_base_failure);
    }
    extern "C" {
        #[link_name = "\u{1}what"]
        pub fn ios_base_failure_what(
            this: *mut ::std::os::raw::c_void,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}ios_base_destructor"]
        pub fn ios_base_ios_base_destructor(this: *mut ios_base);
    }
    extern "C" {
        pub fn boolalpha(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn noboolalpha(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn showbase(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn noshowbase(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn showpoint(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn noshowpoint(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn showpos(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn noshowpos(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn skipws(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn noskipws(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn uppercase(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn nouppercase(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn unitbuf(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn nounitbuf(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn internal(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn left(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn right(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn dec(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn hex(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn oct(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn fixed(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn scientific(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn hexfloat(__base: *mut ios_base) -> *mut ios_base;
    }
    extern "C" {
        pub fn defaultfloat(__base: *mut ios_base) -> *mut ios_base;
    }
    #[repr(C)]
    pub struct basic_streambuf__bindgen_vtable(::std::os::raw::c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_streambuf<_CharT> {
        pub vtable_: *const basic_streambuf__bindgen_vtable,
        pub _M_in_beg: *mut basic_streambuf_char_type<_CharT>,
        pub _M_in_cur: *mut basic_streambuf_char_type<_CharT>,
        pub _M_in_end: *mut basic_streambuf_char_type<_CharT>,
        pub _M_out_beg: *mut basic_streambuf_char_type<_CharT>,
        pub _M_out_cur: *mut basic_streambuf_char_type<_CharT>,
        pub _M_out_end: *mut basic_streambuf_char_type<_CharT>,
        pub _M_buf_locale: locale,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_streambuf_char_type<_CharT> = _CharT;
    pub type basic_streambuf_traits_type<_Traits> = _Traits;
    pub type basic_streambuf_int_type = [u8; 0usize];
    pub type basic_streambuf_pos_type = [u8; 0usize];
    pub type basic_streambuf_off_type = [u8; 0usize];
    pub type basic_streambuf___streambuf_type<_CharT> =
        basic_streambuf<basic_streambuf_char_type<_CharT>>;
    impl<_CharT> Default for basic_streambuf<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    extern "C" {
        pub fn __copy_streambufs_eof(
            __sbin: *mut basic_streambuf<::std::os::raw::c_char>,
            __sbout: *mut basic_streambuf<::std::os::raw::c_char>,
            __ineof: *mut bool,
        ) -> streamsize;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ctype_base {
        pub _address: u8,
    }
    pub type ctype_base___to_type = *const ::std::os::raw::c_int;
    pub type ctype_base_mask = ::std::os::raw::c_ushort;
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
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type istreambuf_iterator_char_type<_CharT> = _CharT;
    pub type istreambuf_iterator_traits_type<_Traits> = _Traits;
    pub type istreambuf_iterator_int_type = [u8; 0usize];
    pub type istreambuf_iterator_streambuf_type<_CharT> = basic_streambuf<_CharT>;
    pub type istreambuf_iterator_istream_type<_CharT> = basic_istream<_CharT>;
    impl<_CharT> Default for istreambuf_iterator<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for istreambuf_iterator<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "istreambuf_iterator {{ _M_sbuf: {:?} }}", self._M_sbuf)
        }
    }
    impl<_CharT> ::std::cmp::PartialEq for istreambuf_iterator<_CharT>
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
        pub _M_sbuf: *mut ostreambuf_iterator_streambuf_type<_CharT>,
        pub _M_failed: bool,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type ostreambuf_iterator_char_type<_CharT> = _CharT;
    pub type ostreambuf_iterator_traits_type<_Traits> = _Traits;
    pub type ostreambuf_iterator_streambuf_type<_CharT> = basic_streambuf<_CharT>;
    pub type ostreambuf_iterator_ostream_type<_CharT> = basic_ostream<_CharT>;
    impl<_CharT> Default for ostreambuf_iterator<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    extern "C" {
        pub fn __convert_to_v(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut f32,
            arg3: *mut ios_base_iostate,
            arg4: *const __c_locale,
        );
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __pad {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __ctype_abstract_base {
        pub _base: locale_facet,
    }
    pub type __ctype_abstract_base_char_type<_CharT> = _CharT;
    impl Default for __ctype_abstract_base {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ctype {
        pub _base: __ctype_abstract_base,
    }
    pub type ctype_char_type<_CharT> = _CharT;
    pub type ctype_mask = __ctype_abstract_base;
    impl Default for ctype {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct ctype_byname {
        pub _base: ctype,
    }
    pub type ctype_byname_mask = ctype;
    impl Default for ctype_byname {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __num_base {
        pub _address: u8,
    }
    pub const __num_base__S_ominus: __num_base__bindgen_ty_1 = __num_base__bindgen_ty_1::_S_ominus;
    pub const __num_base__S_oplus: __num_base__bindgen_ty_1 = __num_base__bindgen_ty_1::_S_oplus;
    pub const __num_base__S_ox: __num_base__bindgen_ty_1 = __num_base__bindgen_ty_1::_S_ox;
    pub const __num_base__S_oX: __num_base__bindgen_ty_1 = __num_base__bindgen_ty_1::_S_oX;
    pub const __num_base__S_odigits: __num_base__bindgen_ty_1 =
        __num_base__bindgen_ty_1::_S_odigits;
    pub const __num_base__S_odigits_end: __num_base__bindgen_ty_1 =
        __num_base__bindgen_ty_1::_S_odigits_end;
    pub const __num_base__S_oudigits: __num_base__bindgen_ty_1 =
        __num_base__bindgen_ty_1::_S_odigits_end;
    pub const __num_base__S_oudigits_end: __num_base__bindgen_ty_1 =
        __num_base__bindgen_ty_1::_S_oudigits_end;
    pub const __num_base__S_oe: __num_base__bindgen_ty_1 = __num_base__bindgen_ty_1::_S_oe;
    pub const __num_base__S_oE: __num_base__bindgen_ty_1 = __num_base__bindgen_ty_1::_S_oE;
    pub const __num_base__S_oend: __num_base__bindgen_ty_1 =
        __num_base__bindgen_ty_1::_S_oudigits_end;
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __num_base__bindgen_ty_1 {
        _S_ominus = 0,
        _S_oplus = 1,
        _S_ox = 2,
        _S_oX = 3,
        _S_odigits = 4,
        _S_odigits_end = 20,
        _S_oudigits_end = 36,
        _S_oe = 18,
        _S_oE = 34,
    }
    pub const __num_base__S_iminus: __num_base__bindgen_ty_2 = __num_base__bindgen_ty_2::_S_iminus;
    pub const __num_base__S_iplus: __num_base__bindgen_ty_2 = __num_base__bindgen_ty_2::_S_iplus;
    pub const __num_base__S_ix: __num_base__bindgen_ty_2 = __num_base__bindgen_ty_2::_S_ix;
    pub const __num_base__S_iX: __num_base__bindgen_ty_2 = __num_base__bindgen_ty_2::_S_iX;
    pub const __num_base__S_izero: __num_base__bindgen_ty_2 = __num_base__bindgen_ty_2::_S_izero;
    pub const __num_base__S_ie: __num_base__bindgen_ty_2 = __num_base__bindgen_ty_2::_S_ie;
    pub const __num_base__S_iE: __num_base__bindgen_ty_2 = __num_base__bindgen_ty_2::_S_iE;
    pub const __num_base__S_iend: __num_base__bindgen_ty_2 = __num_base__bindgen_ty_2::_S_iend;
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum __num_base__bindgen_ty_2 {
        _S_iminus = 0,
        _S_iplus = 1,
        _S_ix = 2,
        _S_iX = 3,
        _S_izero = 4,
        _S_ie = 18,
        _S_iE = 24,
        _S_iend = 26,
    }
    extern "C" {
        #[link_name = "\u{1}_S_atoms_out"]
        pub static mut __num_base__S_atoms_out: *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}_S_atoms_in"]
        pub static mut __num_base__S_atoms_in: *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}_S_format_float"]
        pub fn __num_base__S_format_float(
            __io: *const ios_base,
            __fptr: *mut ::std::os::raw::c_char,
            __mod: ::std::os::raw::c_char,
        );
    }
    impl __num_base {
        #[inline]
        pub unsafe fn _S_format_float(
            __io: *const ios_base,
            __fptr: *mut ::std::os::raw::c_char,
            __mod: ::std::os::raw::c_char,
        ) {
            __num_base__S_format_float(__io, __fptr, __mod)
        }
    }
    #[repr(C)]
    pub struct __numpunct_cache<_CharT> {
        pub _base: locale_facet,
        pub _M_grouping: *const ::std::os::raw::c_char,
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
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    impl<_CharT> Default for __numpunct_cache<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for __numpunct_cache<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! ( f , "__numpunct_cache {{ _M_grouping: {:?}, _M_grouping_size: {:?}, _M_use_grouping: {:?}, _M_truename: {:?}, _M_truename_size: {:?}, _M_falsename: {:?}, _M_falsename_size: {:?}, _M_decimal_point: Non-debuggable generic, _M_thousands_sep: Non-debuggable generic, _M_atoms_out: Array with length 36, _M_atoms_in: Array with length 26, _M_allocated: {:?} }}" , self . _M_grouping , self . _M_grouping_size , self . _M_use_grouping , self . _M_truename , self . _M_truename_size , self . _M_falsename , self . _M_falsename_size , self . _M_allocated )
        }
    }
    impl<_CharT> ::std::cmp::PartialEq for __numpunct_cache<_CharT>
    where
        _CharT: PartialEq,
    {
        fn eq(&self, other: &__numpunct_cache<_CharT>) -> bool {
            self._base == other._base
                && self._M_grouping == other._M_grouping
                && self._M_grouping_size == other._M_grouping_size
                && self._M_use_grouping == other._M_use_grouping
                && self._M_truename == other._M_truename
                && self._M_truename_size == other._M_truename_size
                && self._M_falsename == other._M_falsename
                && self._M_falsename_size == other._M_falsename_size
                && self._M_decimal_point == other._M_decimal_point
                && self._M_thousands_sep == other._M_thousands_sep
                && &self._M_atoms_out[..] == &other._M_atoms_out[..]
                && self._M_atoms_in == other._M_atoms_in
                && self._M_allocated == other._M_allocated
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct num_get {
        pub _base: locale_facet,
    }
    pub type num_get_char_type<_CharT> = _CharT;
    pub type num_get_iter_type<_InIter> = _InIter;
    impl Default for num_get {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct num_put {
        pub _base: locale_facet,
    }
    pub type num_put_char_type<_CharT> = _CharT;
    pub type num_put_iter_type<_OutIter> = _OutIter;
    impl Default for num_put {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __use_cache {
        pub _address: u8,
    }
    extern "C" {
        pub fn __verify_grouping(
            __grouping: *const ::std::os::raw::c_char,
            __grouping_size: usize,
            __grouping_tmp: *const string,
        ) -> bool;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ios<_CharT> {
        pub _base: ios_base,
        pub _M_tie: *mut basic_ostream<_CharT>,
        pub _M_fill: basic_ios_char_type<_CharT>,
        pub _M_fill_init: bool,
        pub _M_streambuf: *mut basic_streambuf<_CharT>,
        pub _M_ctype: *const basic_ios___ctype_type,
        pub _M_num_put: *const basic_ios___num_put_type,
        pub _M_num_get: *const basic_ios___num_get_type,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_ios_char_type<_CharT> = _CharT;
    pub type basic_ios_int_type = [u8; 0usize];
    pub type basic_ios_pos_type = [u8; 0usize];
    pub type basic_ios_off_type = [u8; 0usize];
    pub type basic_ios_traits_type<_Traits> = _Traits;
    pub type basic_ios___ctype_type = ctype;
    pub type basic_ios___num_put_type = num_put;
    pub type basic_ios___num_get_type = num_get;
    impl<_CharT> Default for basic_ios<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
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
    pub type basic_ostream___streambuf_type<_CharT> = basic_streambuf<_CharT>;
    pub type basic_ostream___ios_type<_CharT> = basic_ios<_CharT>;
    pub type basic_ostream___ostream_type<_CharT> = basic_ostream<_CharT>;
    pub type basic_ostream___num_put_type = num_put;
    pub type basic_ostream___ctype_type = ctype;
    impl<_CharT> Default for basic_ostream<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_ostream_sentry {
        pub _M_ok: bool,
        pub _M_os: *mut basic_ostream<_CharT>,
    }
    impl Default for basic_ostream_sentry {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_convertible_to_basic_ostream_impl {
        pub _address: u8,
    }
    pub type __is_convertible_to_basic_ostream_impl___ostream_type = ::std::os::raw::c_void;
    pub type __do_is_convertible_to_basic_ostream_impl = remove_reference;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_convertible_to_basic_ostream {
        pub _address: u8,
    }
    pub type __is_convertible_to_basic_ostream_type = __not_;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_insertable {
        pub _base: false_type,
    }
    pub type __rvalue_ostream_type = __is_convertible_to_basic_ostream;
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_istream<_CharT> {
        pub _M_gcount: streamsize,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
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
    impl<_CharT> Default for basic_istream<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_istream_sentry {
        pub _M_ok: bool,
    }
    pub type basic_istream_sentry_traits_type = _Traits;
    pub type basic_istream_sentry___streambuf_type = basic_streambuf<_CharT>;
    pub type basic_istream_sentry___istream_type = basic_istream<_CharT>;
    pub type basic_istream_sentry___ctype_type = basic_istream___ctype_type;
    pub type basic_istream_sentry___int_type = [u8; 0usize];
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct basic_iostream<_CharT> {
        pub _base: basic_istream<_CharT>,
        pub _base_1: basic_ostream<_CharT>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_iostream_char_type<_CharT> = _CharT;
    pub type basic_iostream_int_type = [u8; 0usize];
    pub type basic_iostream_pos_type = [u8; 0usize];
    pub type basic_iostream_off_type = [u8; 0usize];
    pub type basic_iostream_traits_type<_Traits> = _Traits;
    pub type basic_iostream___istream_type<_CharT> = basic_istream<_CharT>;
    pub type basic_iostream___ostream_type<_CharT> = basic_ostream<_CharT>;
    impl<_CharT> Default for basic_iostream<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_convertible_to_basic_istream_impl {
        pub _address: u8,
    }
    pub type __is_convertible_to_basic_istream_impl___istream_type = ::std::os::raw::c_void;
    pub type __do_is_convertible_to_basic_istream_impl = remove_reference;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_convertible_to_basic_istream {
        pub _address: u8,
    }
    pub type __is_convertible_to_basic_istream_type = __not_;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __is_extractable {
        pub _base: false_type,
    }
    pub type __rvalue_istream_type = __is_convertible_to_basic_istream;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct codecvt_base {
        pub _address: u8,
    }
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum codecvt_base_result {
        ok = 0,
        partial = 1,
        error = 2,
        noconv = 3,
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __codecvt_abstract_base {
        pub _base: locale_facet,
    }
    pub use codecvt_base_result as __codecvt_abstract_base_result;
    pub type __codecvt_abstract_base_intern_type<_InternT> = _InternT;
    pub type __codecvt_abstract_base_extern_type<_ExternT> = _ExternT;
    pub type __codecvt_abstract_base_state_type<_StateT> = _StateT;
    impl Default for __codecvt_abstract_base {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct codecvt {
        pub _base: __codecvt_abstract_base,
        pub _M_c_locale_codecvt: __c_locale,
    }
    pub use codecvt_base_result as codecvt_result;
    pub type codecvt_intern_type<_InternT> = _InternT;
    pub type codecvt_extern_type<_ExternT> = _ExternT;
    pub type codecvt_state_type<_StateT> = _StateT;
    impl Default for codecvt {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct codecvt_byname {
        pub _base: codecvt,
    }
    impl Default for codecvt_byname {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub type __c_lock = __gthread_mutex_t;
    pub type __c_file = FILE;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __basic_file {
        pub _address: u8,
    }
    pub type _If_fs_path = enable_if_t;
    #[repr(C)]
    pub struct basic_filebuf<_CharT> {
        pub _base: basic_streambuf<_CharT>,
        pub _M_lock: __c_lock,
        pub _M_file: basic_filebuf___file_type,
        pub _M_mode: ios_base_openmode,
        pub _M_state_beg: basic_filebuf___state_type,
        pub _M_state_cur: basic_filebuf___state_type,
        pub _M_state_last: basic_filebuf___state_type,
        pub _M_buf: *mut basic_filebuf_char_type<_CharT>,
        pub _M_buf_size: usize,
        pub _M_buf_allocated: bool,
        pub _M_reading: bool,
        pub _M_writing: bool,
        pub _M_pback: basic_filebuf_char_type<_CharT>,
        pub _M_pback_cur_save: *mut basic_filebuf_char_type<_CharT>,
        pub _M_pback_end_save: *mut basic_filebuf_char_type<_CharT>,
        pub _M_pback_init: bool,
        pub _M_codecvt: *const basic_filebuf___codecvt_type,
        pub _M_ext_buf: *mut ::std::os::raw::c_char,
        pub _M_ext_buf_size: streamsize,
        pub _M_ext_next: *const ::std::os::raw::c_char,
        pub _M_ext_end: *mut ::std::os::raw::c_char,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_filebuf___chk_state = __and_;
    pub type basic_filebuf_char_type<_CharT> = _CharT;
    pub type basic_filebuf_traits_type<_Traits> = _Traits;
    pub type basic_filebuf_int_type = [u8; 0usize];
    pub type basic_filebuf_pos_type = [u8; 0usize];
    pub type basic_filebuf_off_type = [u8; 0usize];
    pub type basic_filebuf___streambuf_type<_CharT> =
        basic_streambuf<basic_filebuf_char_type<_CharT>>;
    pub type basic_filebuf___filebuf_type<_CharT> = basic_filebuf<basic_filebuf_char_type<_CharT>>;
    pub type basic_filebuf___file_type = __basic_file;
    pub type basic_filebuf___state_type = [u8; 0usize];
    pub type basic_filebuf___codecvt_type = codecvt;
    impl<_CharT> Default for basic_filebuf<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for basic_filebuf<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! ( f , "basic_filebuf {{ _M_lock: {:?}, _M_file: {:?}, _M_mode: {:?}, _M_buf: {:?}, _M_buf_size: {:?}, _M_buf_allocated: {:?}, _M_reading: {:?}, _M_writing: {:?}, _M_pback: Non-debuggable generic, _M_pback_cur_save: {:?}, _M_pback_end_save: {:?}, _M_pback_init: {:?}, _M_codecvt: {:?}, _M_ext_buf: {:?}, _M_ext_buf_size: {:?}, _M_ext_next: {:?}, _M_ext_end: {:?} }}" , self . _M_lock , self . _M_file , self . _M_mode , self . _M_buf , self . _M_buf_size , self . _M_buf_allocated , self . _M_reading , self . _M_writing , self . _M_pback_cur_save , self . _M_pback_end_save , self . _M_pback_init , self . _M_codecvt , self . _M_ext_buf , self . _M_ext_buf_size , self . _M_ext_next , self . _M_ext_end )
        }
    }
    #[repr(C)]
    pub struct basic_ifstream<_CharT> {
        pub _base: basic_istream<_CharT>,
        pub _M_filebuf: basic_ifstream___filebuf_type<_CharT>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_ifstream_char_type<_CharT> = _CharT;
    pub type basic_ifstream_traits_type<_Traits> = _Traits;
    pub type basic_ifstream_int_type = [u8; 0usize];
    pub type basic_ifstream_pos_type = [u8; 0usize];
    pub type basic_ifstream_off_type = [u8; 0usize];
    pub type basic_ifstream___filebuf_type<_CharT> =
        basic_filebuf<basic_ifstream_char_type<_CharT>>;
    pub type basic_ifstream___istream_type<_CharT> =
        basic_istream<basic_ifstream_char_type<_CharT>>;
    impl<_CharT> Default for basic_ifstream<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for basic_ifstream<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "basic_ifstream {{ _M_filebuf: {:?} }}", self._M_filebuf)
        }
    }
    #[repr(C)]
    pub struct basic_ofstream<_CharT> {
        pub _base: basic_ostream<_CharT>,
        pub _M_filebuf: basic_ofstream___filebuf_type<_CharT>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_ofstream_char_type<_CharT> = _CharT;
    pub type basic_ofstream_traits_type<_Traits> = _Traits;
    pub type basic_ofstream_int_type = [u8; 0usize];
    pub type basic_ofstream_pos_type = [u8; 0usize];
    pub type basic_ofstream_off_type = [u8; 0usize];
    pub type basic_ofstream___filebuf_type<_CharT> =
        basic_filebuf<basic_ofstream_char_type<_CharT>>;
    pub type basic_ofstream___ostream_type<_CharT> =
        basic_ostream<basic_ofstream_char_type<_CharT>>;
    impl<_CharT> Default for basic_ofstream<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for basic_ofstream<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "basic_ofstream {{ _M_filebuf: {:?} }}", self._M_filebuf)
        }
    }
    #[repr(C)]
    pub struct basic_fstream<_CharT> {
        pub _base: basic_iostream<_CharT>,
        pub _M_filebuf: basic_fstream___filebuf_type<_CharT>,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    }
    pub type basic_fstream_char_type<_CharT> = _CharT;
    pub type basic_fstream_traits_type<_Traits> = _Traits;
    pub type basic_fstream_int_type = [u8; 0usize];
    pub type basic_fstream_pos_type = [u8; 0usize];
    pub type basic_fstream_off_type = [u8; 0usize];
    pub type basic_fstream___filebuf_type<_CharT> = basic_filebuf<basic_fstream_char_type<_CharT>>;
    pub type basic_fstream___ios_type<_CharT> = basic_ios<basic_fstream_char_type<_CharT>>;
    pub type basic_fstream___iostream_type<_CharT> =
        basic_iostream<basic_fstream_char_type<_CharT>>;
    impl<_CharT> Default for basic_fstream<_CharT> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl<_CharT> ::std::fmt::Debug for basic_fstream<_CharT> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "basic_fstream {{ _M_filebuf: {:?} }}", self._M_filebuf)
        }
    }
    extern "C" {
        pub static mut cin: istream;
    }
    extern "C" {
        pub static mut cout: ostream;
    }
    extern "C" {
        pub static mut cerr: ostream;
    }
    extern "C" {
        pub static mut clog: ostream;
    }
    extern "C" {
        pub static mut wcin: wistream;
    }
    extern "C" {
        pub static mut wcout: wostream;
    }
    extern "C" {
        pub static mut wcerr: wostream;
    }
    extern "C" {
        pub static mut wclog: wostream;
    }
    extern "C" {
        pub static mut __ioinit: ios_base_Init;
    }
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __allocated_ptr<_Alloc> {
        pub _M_alloc: *mut _Alloc,
        pub _M_ptr: __allocated_ptr_pointer,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Alloc>>,
    }
    pub type __allocated_ptr_pointer = allocator_traits;
    pub type __allocated_ptr_value_type = allocator_traits;
    impl<_Alloc> Default for __allocated_ptr<_Alloc> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _List_node {
        pub _base: _List_node_base,
        pub _M_storage: __aligned_membuf,
    }
    impl Default for _List_node {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _List_iterator {
        pub _M_node: *mut _List_node_base,
    }
    pub type _List_iterator__Self = _List_iterator;
    pub type _List_iterator__Node = _List_node;
    pub type _List_iterator_difference_type = isize;
    pub type _List_iterator_iterator_category = bidirectional_iterator_tag;
    pub type _List_iterator_value_type<_Tp> = _Tp;
    pub type _List_iterator_pointer<_Tp> = *mut _Tp;
    pub type _List_iterator_reference<_Tp> = *mut _Tp;
    impl Default for _List_iterator {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _List_const_iterator {
        pub _M_node: *const _List_node_base,
    }
    pub type _List_const_iterator__Self = _List_const_iterator;
    pub type _List_const_iterator__Node = _List_node;
    pub type _List_const_iterator_iterator = _List_iterator;
    pub type _List_const_iterator_difference_type = isize;
    pub type _List_const_iterator_iterator_category = bidirectional_iterator_tag;
    pub type _List_const_iterator_value_type<_Tp> = _Tp;
    pub type _List_const_iterator_pointer<_Tp> = *const _Tp;
    pub type _List_const_iterator_reference<_Tp> = *const _Tp;
    impl Default for _List_const_iterator {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    extern "C" {
        pub fn acos(__x: f32) -> f32;
    }
    extern "C" {
        pub fn asin(__x: f32) -> f32;
    }
    extern "C" {
        pub fn atan(__x: f32) -> f32;
    }
    extern "C" {
        pub fn atan2(__y: f32, __x: f32) -> f32;
    }
    extern "C" {
        pub fn ceil(__x: f32) -> f32;
    }
    extern "C" {
        pub fn cos(__x: f32) -> f32;
    }
    extern "C" {
        pub fn cosh(__x: f32) -> f32;
    }
    extern "C" {
        pub fn exp(__x: f32) -> f32;
    }
    extern "C" {
        pub fn fabs(__x: f32) -> f32;
    }
    extern "C" {
        pub fn floor(__x: f32) -> f32;
    }
    extern "C" {
        pub fn fmod(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn frexp(__x: f32, __exp: *mut ::std::os::raw::c_int) -> f32;
    }
    extern "C" {
        pub fn ldexp(__x: f32, __exp: ::std::os::raw::c_int) -> f32;
    }
    extern "C" {
        pub fn log(__x: f32) -> f32;
    }
    extern "C" {
        pub fn log10(__x: f32) -> f32;
    }
    extern "C" {
        pub fn modf(__x: f32, __iptr: *mut f32) -> f32;
    }
    extern "C" {
        pub fn pow(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn sin(__x: f32) -> f32;
    }
    extern "C" {
        pub fn sinh(__x: f32) -> f32;
    }
    extern "C" {
        pub fn sqrt(__x: f32) -> f32;
    }
    extern "C" {
        pub fn tan(__x: f32) -> f32;
    }
    extern "C" {
        pub fn tanh(__x: f32) -> f32;
    }
    extern "C" {
        pub fn fpclassify(__x: f32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn isfinite(__x: f32) -> bool;
    }
    extern "C" {
        pub fn isinf(__x: f32) -> bool;
    }
    extern "C" {
        pub fn isnan(__x: f32) -> bool;
    }
    extern "C" {
        pub fn isnormal(__x: f32) -> bool;
    }
    extern "C" {
        pub fn signbit(__x: f32) -> bool;
    }
    extern "C" {
        pub fn isgreater(__x: f32, __y: f32) -> bool;
    }
    extern "C" {
        pub fn isgreaterequal(__x: f32, __y: f32) -> bool;
    }
    extern "C" {
        pub fn isless(__x: f32, __y: f32) -> bool;
    }
    extern "C" {
        pub fn islessequal(__x: f32, __y: f32) -> bool;
    }
    extern "C" {
        pub fn islessgreater(__x: f32, __y: f32) -> bool;
    }
    extern "C" {
        pub fn isunordered(__x: f32, __y: f32) -> bool;
    }
    extern "C" {
        pub fn acosh(__x: f32) -> f32;
    }
    extern "C" {
        pub fn asinh(__x: f32) -> f32;
    }
    extern "C" {
        pub fn atanh(__x: f32) -> f32;
    }
    extern "C" {
        pub fn cbrt(__x: f32) -> f32;
    }
    extern "C" {
        pub fn copysign(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn erf(__x: f32) -> f32;
    }
    extern "C" {
        pub fn erfc(__x: f32) -> f32;
    }
    extern "C" {
        pub fn exp2(__x: f32) -> f32;
    }
    extern "C" {
        pub fn expm1(__x: f32) -> f32;
    }
    extern "C" {
        pub fn fdim(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn fma(__x: f32, __y: f32, __z: f32) -> f32;
    }
    extern "C" {
        pub fn fmax(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn fmin(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn hypot(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn ilogb(__x: f32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lgamma(__x: f32) -> f32;
    }
    extern "C" {
        pub fn llrint(__x: f32) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn llround(__x: f32) -> ::std::os::raw::c_longlong;
    }
    extern "C" {
        pub fn log1p(__x: f32) -> f32;
    }
    extern "C" {
        pub fn log2(__x: f32) -> f32;
    }
    extern "C" {
        pub fn logb(__x: f32) -> f32;
    }
    extern "C" {
        pub fn lrint(__x: f32) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn lround(__x: f32) -> ::std::os::raw::c_long;
    }
    extern "C" {
        pub fn nearbyint(__x: f32) -> f32;
    }
    extern "C" {
        pub fn nextafter(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn nexttoward(__x: f32, __y: u128) -> f32;
    }
    extern "C" {
        pub fn remainder(__x: f32, __y: f32) -> f32;
    }
    extern "C" {
        pub fn remquo(__x: f32, __y: f32, __pquo: *mut ::std::os::raw::c_int) -> f32;
    }
    extern "C" {
        pub fn rint(__x: f32) -> f32;
    }
    extern "C" {
        pub fn round(__x: f32) -> f32;
    }
    extern "C" {
        pub fn scalbln(__x: f32, __ex: ::std::os::raw::c_long) -> f32;
    }
    extern "C" {
        pub fn scalbn(__x: f32, __ex: ::std::os::raw::c_int) -> f32;
    }
    extern "C" {
        pub fn tgamma(__x: f32) -> f32;
    }
    extern "C" {
        pub fn trunc(__x: f32) -> f32;
    }
    extern "C" {
        pub fn assoc_laguerref(
            __n: ::std::os::raw::c_uint,
            __m: ::std::os::raw::c_uint,
            __x: f32,
        ) -> f32;
    }
    extern "C" {
        pub fn assoc_laguerrel(
            __n: ::std::os::raw::c_uint,
            __m: ::std::os::raw::c_uint,
            __x: u128,
        ) -> u128;
    }
    extern "C" {
        pub fn assoc_legendref(
            __l: ::std::os::raw::c_uint,
            __m: ::std::os::raw::c_uint,
            __x: f32,
        ) -> f32;
    }
    extern "C" {
        pub fn assoc_legendrel(
            __l: ::std::os::raw::c_uint,
            __m: ::std::os::raw::c_uint,
            __x: u128,
        ) -> u128;
    }
    extern "C" {
        pub fn betaf(__a: f32, __b: f32) -> f32;
    }
    extern "C" {
        pub fn betal(__a: u128, __b: u128) -> u128;
    }
    extern "C" {
        pub fn comp_ellint_1f(__k: f32) -> f32;
    }
    extern "C" {
        pub fn comp_ellint_1l(__k: u128) -> u128;
    }
    extern "C" {
        pub fn comp_ellint_2f(__k: f32) -> f32;
    }
    extern "C" {
        pub fn comp_ellint_2l(__k: u128) -> u128;
    }
    extern "C" {
        pub fn comp_ellint_3f(__k: f32, __nu: f32) -> f32;
    }
    extern "C" {
        pub fn comp_ellint_3l(__k: u128, __nu: u128) -> u128;
    }
    extern "C" {
        pub fn cyl_bessel_if(__nu: f32, __x: f32) -> f32;
    }
    extern "C" {
        pub fn cyl_bessel_il(__nu: u128, __x: u128) -> u128;
    }
    extern "C" {
        pub fn cyl_bessel_jf(__nu: f32, __x: f32) -> f32;
    }
    extern "C" {
        pub fn cyl_bessel_jl(__nu: u128, __x: u128) -> u128;
    }
    extern "C" {
        pub fn cyl_bessel_kf(__nu: f32, __x: f32) -> f32;
    }
    extern "C" {
        pub fn cyl_bessel_kl(__nu: u128, __x: u128) -> u128;
    }
    extern "C" {
        pub fn cyl_neumannf(__nu: f32, __x: f32) -> f32;
    }
    extern "C" {
        pub fn cyl_neumannl(__nu: u128, __x: u128) -> u128;
    }
    extern "C" {
        pub fn ellint_1f(__k: f32, __phi: f32) -> f32;
    }
    extern "C" {
        pub fn ellint_1l(__k: u128, __phi: u128) -> u128;
    }
    extern "C" {
        pub fn ellint_2f(__k: f32, __phi: f32) -> f32;
    }
    extern "C" {
        pub fn ellint_2l(__k: u128, __phi: u128) -> u128;
    }
    extern "C" {
        pub fn ellint_3f(__k: f32, __nu: f32, __phi: f32) -> f32;
    }
    extern "C" {
        pub fn ellint_3l(__k: u128, __nu: u128, __phi: u128) -> u128;
    }
    extern "C" {
        pub fn expintf(__x: f32) -> f32;
    }
    extern "C" {
        pub fn expintl(__x: u128) -> u128;
    }
    extern "C" {
        pub fn hermitef(__n: ::std::os::raw::c_uint, __x: f32) -> f32;
    }
    extern "C" {
        pub fn hermitel(__n: ::std::os::raw::c_uint, __x: u128) -> u128;
    }
    extern "C" {
        pub fn laguerref(__n: ::std::os::raw::c_uint, __x: f32) -> f32;
    }
    extern "C" {
        pub fn laguerrel(__n: ::std::os::raw::c_uint, __x: u128) -> u128;
    }
    extern "C" {
        pub fn legendref(__l: ::std::os::raw::c_uint, __x: f32) -> f32;
    }
    extern "C" {
        pub fn legendrel(__l: ::std::os::raw::c_uint, __x: u128) -> u128;
    }
    extern "C" {
        pub fn riemann_zetaf(__s: f32) -> f32;
    }
    extern "C" {
        pub fn riemann_zetal(__s: u128) -> u128;
    }
    extern "C" {
        pub fn sph_besself(__n: ::std::os::raw::c_uint, __x: f32) -> f32;
    }
    extern "C" {
        pub fn sph_bessell(__n: ::std::os::raw::c_uint, __x: u128) -> u128;
    }
    extern "C" {
        pub fn sph_legendref(
            __l: ::std::os::raw::c_uint,
            __m: ::std::os::raw::c_uint,
            __theta: f32,
        ) -> f32;
    }
    extern "C" {
        pub fn sph_legendrel(
            __l: ::std::os::raw::c_uint,
            __m: ::std::os::raw::c_uint,
            __theta: u128,
        ) -> u128;
    }
    extern "C" {
        pub fn sph_neumannf(__n: ::std::os::raw::c_uint, __x: f32) -> f32;
    }
    extern "C" {
        pub fn sph_neumannl(__n: ::std::os::raw::c_uint, __x: u128) -> u128;
    }
}
pub mod __gnu_cxx {

    extern "C" {
        pub fn div(__n: ::std::os::raw::c_longlong, __d: ::std::os::raw::c_longlong) -> lldiv_t;
    }
    pub type __conditional_type___type<_Iftrue> = _Iftrue;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __add_unsigned {
        pub _address: u8,
    }
    pub type __add_unsigned___if_type = u8;
    pub type __add_unsigned___type = __add_unsigned___if_type;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __remove_unsigned {
        pub _address: u8,
    }
    pub type __remove_unsigned___if_type = u8;
    pub type __remove_unsigned___type = __remove_unsigned___if_type;
    extern "C" {
        pub fn __is_null_pointer(arg1: nullptr_t) -> bool;
    }
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
        pub static __digits: ::std::os::raw::c_int;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __numeric_traits_floating {
        pub _address: u8,
    }
    extern "C" {
        pub static __max_digits10: ::std::os::raw::c_int;
    }
    extern "C" {
        pub static __digits10: ::std::os::raw::c_int;
    }
    extern "C" {
        pub static __max_exponent10: ::std::os::raw::c_int;
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
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
    }
    pub type __normal_iterator___traits_type = iterator_traits;
    pub type __normal_iterator_iterator_type<_Iterator> = _Iterator;
    pub type __normal_iterator_iterator_category = __normal_iterator___traits_type;
    pub type __normal_iterator_value_type = __normal_iterator___traits_type;
    pub type __normal_iterator_difference_type = __normal_iterator___traits_type;
    pub type __normal_iterator_reference = __normal_iterator___traits_type;
    pub type __normal_iterator_pointer = __normal_iterator___traits_type;
    impl<_Iterator> Default for __normal_iterator<_Iterator> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    pub mod __ops {

        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_less_iter {
            pub _address: u8,
        }
        extern "C" {
            pub fn __iter_less_iter() -> _Iter_less_iter;
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_less_val {
            pub _address: u8,
        }
        extern "C" {
            #[link_name = "\u{1}_Iter_less_val"]
            pub fn _Iter_less_val__Iter_less_val(this: *mut _Iter_less_val);
        }
        extern "C" {
            #[link_name = "\u{1}_Iter_less_val"]
            pub fn _Iter_less_val__Iter_less_val1(this: *mut _Iter_less_val, arg1: _Iter_less_iter);
        }
        impl _Iter_less_val {
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                _Iter_less_val__Iter_less_val(&mut __bindgen_tmp);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new1(arg1: _Iter_less_iter) -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                _Iter_less_val__Iter_less_val1(&mut __bindgen_tmp, arg1);
                __bindgen_tmp
            }
        }
        extern "C" {
            pub fn __iter_less_val() -> _Iter_less_val;
        }
        extern "C" {
            pub fn __iter_comp_val(arg1: _Iter_less_iter) -> _Iter_less_val;
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Val_less_iter {
            pub _address: u8,
        }
        extern "C" {
            #[link_name = "\u{1}_Val_less_iter"]
            pub fn _Val_less_iter__Val_less_iter(this: *mut _Val_less_iter);
        }
        extern "C" {
            #[link_name = "\u{1}_Val_less_iter"]
            pub fn _Val_less_iter__Val_less_iter1(this: *mut _Val_less_iter, arg1: _Iter_less_iter);
        }
        impl _Val_less_iter {
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                _Val_less_iter__Val_less_iter(&mut __bindgen_tmp);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new1(arg1: _Iter_less_iter) -> Self {
                let mut __bindgen_tmp = ::std::mem::uninitialized();
                _Val_less_iter__Val_less_iter1(&mut __bindgen_tmp, arg1);
                __bindgen_tmp
            }
        }
        extern "C" {
            pub fn __val_less_iter() -> _Val_less_iter;
        }
        extern "C" {
            pub fn __val_comp_iter(arg1: _Iter_less_iter) -> _Val_less_iter;
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_equal_to_iter {
            pub _address: u8,
        }
        extern "C" {
            pub fn __iter_equal_to_iter() -> _Iter_equal_to_iter;
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_equal_to_val {
            pub _address: u8,
        }
        extern "C" {
            pub fn __iter_equal_to_val() -> _Iter_equal_to_val;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_comp_iter<_Compare> {
            pub _M_comp: _Compare,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Compare>>,
        }
        impl<_Compare> Default for _Iter_comp_iter<_Compare> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_comp_val<_Compare> {
            pub _M_comp: _Compare,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Compare>>,
        }
        impl<_Compare> Default for _Iter_comp_val<_Compare> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Val_comp_iter<_Compare> {
            pub _M_comp: _Compare,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Compare>>,
        }
        impl<_Compare> Default for _Val_comp_iter<_Compare> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_equals_val<_Value> {
            pub _M_value: *mut _Value,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Value>>,
        }
        impl<_Value> Default for _Iter_equals_val<_Value> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_equals_iter<_Iterator1> {
            pub _M_it1: _Iterator1,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator1>>,
        }
        impl<_Iterator1> Default for _Iter_equals_iter<_Iterator1> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_pred<_Predicate> {
            pub _M_pred: _Predicate,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Predicate>>,
        }
        impl<_Predicate> Default for _Iter_pred<_Predicate> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_comp_to_val<_Compare, _Value> {
            pub _M_comp: _Compare,
            pub _M_value: *mut _Value,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Compare>>,
            pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Value>>,
        }
        impl<_Compare, _Value> Default for _Iter_comp_to_val<_Compare, _Value> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_comp_to_iter<_Compare, _Iterator1> {
            pub _M_comp: _Compare,
            pub _M_it1: _Iterator1,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Compare>>,
            pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator1>>,
        }
        impl<_Compare, _Iterator1> Default for _Iter_comp_to_iter<_Compare, _Iterator1> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct _Iter_negate<_Predicate> {
            pub _M_pred: _Predicate,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Predicate>>,
        }
        impl<_Predicate> Default for _Iter_negate<_Predicate> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
    }
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
    pub type new_allocator_rebind_other = new_allocator;
    pub type new_allocator_propagate_on_container_move_assignment = true_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __alloc_traits_rebind {
        pub _address: u8,
    }
    pub type __alloc_traits_rebind_other = __alloc_traits__Base_type;
    impl Default for __alloc_traits {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __aligned_membuf {
        pub _M_storage: *mut ::std::os::raw::c_uchar,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __aligned_membuf__Tp2<_Tp> {
        pub _M_t: _Tp,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Tp>>,
    }
    impl<_Tp> Default for __aligned_membuf__Tp2<_Tp> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl Default for __aligned_membuf {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __aligned_buffer {
        pub _M_storage: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct _Char_types {
        pub _address: u8,
    }
    pub type _Char_types_int_type = ::std::os::raw::c_ulong;
    pub type _Char_types_pos_type = streampos;
    pub type _Char_types_off_type = streamoff;
    pub type _Char_types_state_type = mbstate_t;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct char_traits {
        pub _address: u8,
    }
    pub type char_traits_char_type<_CharT> = _CharT;
    pub type char_traits_int_type = _Char_types;
    pub type char_traits_pos_type = _Char_types;
    pub type char_traits_off_type = _Char_types;
    pub type char_traits_state_type = _Char_types;
    extern "C" {
        pub fn __uselocale(arg1: locale_t) -> locale_t;
    }
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtold(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> u128;
}
extern "C" {
    pub fn strtof32(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float32;
}
extern "C" {
    pub fn strtof64(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float64;
}
extern "C" {
    pub fn strtof32x(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float32x;
}
extern "C" {
    pub fn strtof64x(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> _Float64x;
}
extern "C" {
    pub fn strtol(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strfromd(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfroml(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: u128,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf32(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: _Float32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf64(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: _Float64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf32x(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: _Float32x,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfromf64x(
        __dest: *mut ::std::os::raw::c_char,
        __size: usize,
        __format: *const ::std::os::raw::c_char,
        __f: _Float64x,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
impl Default for __locale_struct {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn strtol_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoll_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtod_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> f64;
}
extern "C" {
    pub fn strtof_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> f32;
}
extern "C" {
    pub fn strtold_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> u128;
}
extern "C" {
    pub fn strtof32_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float32;
}
extern "C" {
    pub fn strtof64_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float64;
}
extern "C" {
    pub fn strtof32x_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float32x;
}
extern "C" {
    pub fn strtof64x_l(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> _Float64x;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type ino64_t = __ino64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type useconds_t = __useconds_t;
pub type suseconds_t = __suseconds_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulong;
pub type register_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16usize],
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type blkcnt64_t = __blkcnt64_t;
pub type fsblkcnt64_t = __fsblkcnt64_t;
pub type fsfilcnt64_t = __fsfilcnt64_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: ::std::os::raw::c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
impl Default for __pthread_cond_s__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for __pthread_cond_s__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "__pthread_cond_s__bindgen_ty_1 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: ::std::os::raw::c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
impl Default for __pthread_cond_s__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for __pthread_cond_s__bindgen_ty_2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "__pthread_cond_s__bindgen_ty_2 {{ union }}")
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for __pthread_cond_s {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "__pthread_cond_s {{ __bindgen_anon_1: {:?}, __bindgen_anon_2: {:?}, __g_refs: {:?}, __g_size: {:?}, __g1_orig_size: {:?}, __wrefs: {:?}, __g_signals: {:?} }}" , self . __bindgen_anon_1 , self . __bindgen_anon_2 , self . __g_refs , self . __g_size , self . __g1_orig_size , self . __wrefs , self . __g_signals )
    }
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_mutexattr_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_mutexattr_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_condattr_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_condattr_t {{ union }}")
    }
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_attr_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_attr_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 5usize],
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_mutex_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_mutex_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_cond_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_cond_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_rwlock_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_rwlock_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: u64,
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_rwlockattr_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_rwlockattr_t {{ union }}")
    }
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 4usize],
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_barrier_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_barrier_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for pthread_barrierattr_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "pthread_barrierattr_t {{ union }}")
    }
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
impl Default for random_data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn reallocarray(
        __ptr: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn alloca(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        __func: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: ::std::option::Option<
            unsafe extern "C" fn(__func: ::std::os::raw::c_int, __arg: *mut ::std::os::raw::c_void),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn secure_getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemp64(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps64(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkostemp(
        __template: *mut ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemp64(
        __template: *mut ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkostemps64(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalize_file_name(
        __name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type comparison_fn_t = __compar_fn_t;
pub type __compar_d_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    );
}
extern "C" {
    pub fn qsort_r(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_d_fn_t,
        __arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qecvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qgcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(
        __pwc: *mut u32,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut u32, __s: *const ::std::os::raw::c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const u32, __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn posix_openpt(__oflag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grantpt(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlockpt(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ptsname(__fd: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ptsname_r(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpt() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memmove(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memccpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn rawmemchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memrchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> usize;
}
extern "C" {
    pub fn strcoll_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __l: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm_l(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
        __l: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strdup(__s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strndup(
        __string: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchrnul(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __reject: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    pub fn strpbrk(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strstr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcasestr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn memmem(
        __haystack: *const ::std::os::raw::c_void,
        __haystacklen: usize,
        __needle: *const ::std::os::raw::c_void,
        __needlelen: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn __mempcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mempcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> usize;
}
extern "C" {
    pub fn strnlen(__string: *const ::std::os::raw::c_char, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strerror_l(
        __errnum: ::std::os::raw::c_int,
        __l: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcopy(
        __src: *const ::std::os::raw::c_void,
        __dest: *mut ::std::os::raw::c_void,
        __n: usize,
    );
}
extern "C" {
    pub fn bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn index(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rindex(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ffs(__i: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsl(__l: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsll(__ll: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn explicit_bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strverscmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strfry(__string: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn memfrob(__s: *mut ::std::os::raw::c_void, __n: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn basename(__filename: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __assert_fail(
        __assertion: *const ::std::os::raw::c_char,
        __file: *const ::std::os::raw::c_char,
        __line: ::std::os::raw::c_uint,
        __function: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn __assert_perror_fail(
        __errnum: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __line: ::std::os::raw::c_uint,
        __function: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn __assert(
        __assertion: *const ::std::os::raw::c_char,
        __file: *const ::std::os::raw::c_char,
        __line: ::std::os::raw::c_int,
    );
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
    pub N: ::std::os::raw::c_ulong,
    pub values: *mut f64,
}
impl Default for t_CKVECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type c_str = *mut ::std::os::raw::c_char;
pub type c_constr = *const ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX_SAMPLE {
    pub re: f64,
    pub im: f64,
}
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
    _bindgen_union_align: u32,
}
impl Default for __mbstate_t__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for __mbstate_t__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "__mbstate_t__bindgen_ty_1 {{ union }}")
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _G_fpos_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "_G_fpos_t {{ __pos: {:?}, __state: {:?} }}",
            self.__pos, self.__state
        )
    }
}
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _G_fpos64_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "_G_fpos64_t {{ __pos: {:?}, __state: {:?} }}",
            self.__pos, self.__state
        )
    }
}
pub type __fpos64_t = _G_fpos64_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
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
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cookie_read_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_write_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *const ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_seek_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __pos: *mut __off64_t,
        __w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type cookie_close_function_t = ::std::option::Option<
    unsafe extern "C" fn(__cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _IO_cookie_io_functions_t {
    pub read: cookie_read_function_t,
    pub write: cookie_write_function_t,
    pub seek: cookie_seek_function_t,
    pub close: cookie_close_function_t,
}
impl Default for _IO_cookie_io_functions_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
pub type va_list = __gnuc_va_list;
pub type fpos_t = __fpos_t;
pub type fpos64_t = __fpos64_t;
extern "C" {
    pub static mut stdin: *mut FILE;
}
extern "C" {
    pub static mut stdout: *mut FILE;
}
extern "C" {
    pub static mut stderr: *mut FILE;
}
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat2(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpfile64() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fopen64(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen64(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
extern "C" {
    pub fn fopencookie(
        __magic_cookie: *mut ::std::os::raw::c_void,
        __modes: *const ::std::os::raw::c_char,
        __io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: usize,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut usize,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vasprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __f: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fgets_unlocked(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __s: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fputs_unlocked(
        __s: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseeko64(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello64(__stream: *mut FILE) -> __off64_t;
}
extern "C" {
    pub fn fgetpos64(__stream: *mut FILE, __pos: *mut fpos64_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos64(__stream: *mut FILE, __pos: *const fpos64_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub static mut sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub static mut _sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut _sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn cuserid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct obstack {
    _unused: [u8; 0],
}
extern "C" {
    pub fn obstack_printf(
        __obstack: *mut obstack,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const ::std::os::raw::c_char,
        __args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct sched_param {
    pub sched_priority: ::std::os::raw::c_int,
}
extern "C" {
    pub fn clone(
        __fn: ::std::option::Option<
            unsafe extern "C" fn(__fn: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
        __child_stack: *mut ::std::os::raw::c_void,
        __flags: ::std::os::raw::c_int,
        __arg: *mut ::std::os::raw::c_void,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unshare(__flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_getcpu() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setns(
        __fd: ::std::os::raw::c_int,
        __nstype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type __cpu_mask = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16usize],
}
extern "C" {
    pub fn __sched_cpucount(__setsize: usize, __setp: *const cpu_set_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __sched_cpualloc(__count: usize) -> *mut cpu_set_t;
}
extern "C" {
    pub fn __sched_cpufree(__set: *mut cpu_set_t);
}
extern "C" {
    pub fn sched_setparam(__pid: __pid_t, __param: *const sched_param) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_getparam(__pid: __pid_t, __param: *mut sched_param) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_setscheduler(
        __pid: __pid_t,
        __policy: ::std::os::raw::c_int,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_getscheduler(__pid: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_yield() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_get_priority_max(__algorithm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_get_priority_min(__algorithm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_rr_get_interval(__pid: __pid_t, __t: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_setaffinity(
        __pid: __pid_t,
        __cpusetsize: usize,
        __cpuset: *const cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sched_getaffinity(
        __pid: __pid_t,
        __cpusetsize: usize,
        __cpuset: *mut cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct timex {
    pub modes: ::std::os::raw::c_uint,
    pub offset: __syscall_slong_t,
    pub freq: __syscall_slong_t,
    pub maxerror: __syscall_slong_t,
    pub esterror: __syscall_slong_t,
    pub status: ::std::os::raw::c_int,
    pub constant: __syscall_slong_t,
    pub precision: __syscall_slong_t,
    pub tolerance: __syscall_slong_t,
    pub time: timeval,
    pub tick: __syscall_slong_t,
    pub ppsfreq: __syscall_slong_t,
    pub jitter: __syscall_slong_t,
    pub shift: ::std::os::raw::c_int,
    pub stabil: __syscall_slong_t,
    pub jitcnt: __syscall_slong_t,
    pub calcnt: __syscall_slong_t,
    pub errcnt: __syscall_slong_t,
    pub stbcnt: __syscall_slong_t,
    pub tai: ::std::os::raw::c_int,
    pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 44usize], u8>,
}
impl timex {
    #[inline]
    pub fn new_bitfield_1() -> root::__BindgenBitfieldUnit<[u8; 44usize], u8> {
        let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 44usize], u8> =
            Default::default();
        __bindgen_bitfield_unit
    }
}
extern "C" {
    pub fn clock_adjtime(__clock_id: __clockid_t, __utx: *mut timex) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}
impl Default for tm {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent {
    _unused: [u8; 0],
}
extern "C" {
    pub fn clock() -> clock_t;
}
extern "C" {
    pub fn time(__timer: *mut time_t) -> time_t;
}
extern "C" {
    pub fn difftime(__time1: time_t, __time0: time_t) -> f64;
}
extern "C" {
    pub fn mktime(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn strftime(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
    ) -> usize;
}
extern "C" {
    pub fn strptime(
        __s: *const ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        __tp: *mut tm,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strftime_l(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strptime_l(
        __s: *const ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        __tp: *mut tm,
        __loc: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gmtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn localtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn asctime(__tp: *const tm) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn asctime_r(
        __tp: *const tm,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime_r(
        __timer: *const time_t,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut __tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    pub static mut __daylight: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut __timezone: ::std::os::raw::c_long;
}
extern "C" {
    pub static mut tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    pub fn tzset();
}
extern "C" {
    pub static mut daylight: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut timezone: ::std::os::raw::c_long;
}
extern "C" {
    pub fn stime(__when: *const time_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timegm(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn timelocal(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn dysize(__year: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: ::std::os::raw::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_delete(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: ::std::os::raw::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_getoverrun(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timespec_get(
        __ts: *mut timespec,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut getdate_err: ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdate(__string: *const ::std::os::raw::c_char) -> *mut tm;
}
extern "C" {
    pub fn getdate_r(
        __string: *const ::std::os::raw::c_char,
        __resbufp: *mut tm,
    ) -> ::std::os::raw::c_int;
}
pub type __jmp_buf = [::std::os::raw::c_long; 8usize];
pub const PTHREAD_CREATE_JOINABLE: _bindgen_ty_1 = _bindgen_ty_1::PTHREAD_CREATE_JOINABLE;
pub const PTHREAD_CREATE_DETACHED: _bindgen_ty_1 = _bindgen_ty_1::PTHREAD_CREATE_DETACHED;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_1 {
    PTHREAD_CREATE_JOINABLE = 0,
    PTHREAD_CREATE_DETACHED = 1,
}
pub const PTHREAD_MUTEX_TIMED_NP: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_TIMED_NP;
pub const PTHREAD_MUTEX_RECURSIVE_NP: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_RECURSIVE_NP;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_ERRORCHECK_NP;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_ADAPTIVE_NP;
pub const PTHREAD_MUTEX_NORMAL: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_TIMED_NP;
pub const PTHREAD_MUTEX_RECURSIVE: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_RECURSIVE_NP;
pub const PTHREAD_MUTEX_ERRORCHECK: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_ERRORCHECK_NP;
pub const PTHREAD_MUTEX_DEFAULT: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_TIMED_NP;
pub const PTHREAD_MUTEX_FAST_NP: _bindgen_ty_2 = _bindgen_ty_2::PTHREAD_MUTEX_TIMED_NP;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_2 {
    PTHREAD_MUTEX_TIMED_NP = 0,
    PTHREAD_MUTEX_RECURSIVE_NP = 1,
    PTHREAD_MUTEX_ERRORCHECK_NP = 2,
    PTHREAD_MUTEX_ADAPTIVE_NP = 3,
}
pub const PTHREAD_MUTEX_STALLED: _bindgen_ty_3 = _bindgen_ty_3::PTHREAD_MUTEX_STALLED;
pub const PTHREAD_MUTEX_STALLED_NP: _bindgen_ty_3 = _bindgen_ty_3::PTHREAD_MUTEX_STALLED;
pub const PTHREAD_MUTEX_ROBUST: _bindgen_ty_3 = _bindgen_ty_3::PTHREAD_MUTEX_ROBUST;
pub const PTHREAD_MUTEX_ROBUST_NP: _bindgen_ty_3 = _bindgen_ty_3::PTHREAD_MUTEX_ROBUST;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_3 {
    PTHREAD_MUTEX_STALLED = 0,
    PTHREAD_MUTEX_ROBUST = 1,
}
pub const PTHREAD_PRIO_NONE: _bindgen_ty_4 = _bindgen_ty_4::PTHREAD_PRIO_NONE;
pub const PTHREAD_PRIO_INHERIT: _bindgen_ty_4 = _bindgen_ty_4::PTHREAD_PRIO_INHERIT;
pub const PTHREAD_PRIO_PROTECT: _bindgen_ty_4 = _bindgen_ty_4::PTHREAD_PRIO_PROTECT;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_4 {
    PTHREAD_PRIO_NONE = 0,
    PTHREAD_PRIO_INHERIT = 1,
    PTHREAD_PRIO_PROTECT = 2,
}
pub const PTHREAD_RWLOCK_PREFER_READER_NP: _bindgen_ty_5 =
    _bindgen_ty_5::PTHREAD_RWLOCK_PREFER_READER_NP;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: _bindgen_ty_5 =
    _bindgen_ty_5::PTHREAD_RWLOCK_PREFER_WRITER_NP;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: _bindgen_ty_5 =
    _bindgen_ty_5::PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP;
pub const PTHREAD_RWLOCK_DEFAULT_NP: _bindgen_ty_5 = _bindgen_ty_5::PTHREAD_RWLOCK_PREFER_READER_NP;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_5 {
    PTHREAD_RWLOCK_PREFER_READER_NP = 0,
    PTHREAD_RWLOCK_PREFER_WRITER_NP = 1,
    PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP = 2,
}
pub const PTHREAD_INHERIT_SCHED: _bindgen_ty_6 = _bindgen_ty_6::PTHREAD_INHERIT_SCHED;
pub const PTHREAD_EXPLICIT_SCHED: _bindgen_ty_6 = _bindgen_ty_6::PTHREAD_EXPLICIT_SCHED;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_6 {
    PTHREAD_INHERIT_SCHED = 0,
    PTHREAD_EXPLICIT_SCHED = 1,
}
pub const PTHREAD_SCOPE_SYSTEM: _bindgen_ty_7 = _bindgen_ty_7::PTHREAD_SCOPE_SYSTEM;
pub const PTHREAD_SCOPE_PROCESS: _bindgen_ty_7 = _bindgen_ty_7::PTHREAD_SCOPE_PROCESS;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_7 {
    PTHREAD_SCOPE_SYSTEM = 0,
    PTHREAD_SCOPE_PROCESS = 1,
}
pub const PTHREAD_PROCESS_PRIVATE: _bindgen_ty_8 = _bindgen_ty_8::PTHREAD_PROCESS_PRIVATE;
pub const PTHREAD_PROCESS_SHARED: _bindgen_ty_8 = _bindgen_ty_8::PTHREAD_PROCESS_SHARED;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_8 {
    PTHREAD_PROCESS_PRIVATE = 0,
    PTHREAD_PROCESS_SHARED = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _pthread_cleanup_buffer {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __canceltype: ::std::os::raw::c_int,
    pub __prev: *mut _pthread_cleanup_buffer,
}
impl Default for _pthread_cleanup_buffer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub const PTHREAD_CANCEL_ENABLE: _bindgen_ty_9 = _bindgen_ty_9::PTHREAD_CANCEL_ENABLE;
pub const PTHREAD_CANCEL_DISABLE: _bindgen_ty_9 = _bindgen_ty_9::PTHREAD_CANCEL_DISABLE;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_9 {
    PTHREAD_CANCEL_ENABLE = 0,
    PTHREAD_CANCEL_DISABLE = 1,
}
pub const PTHREAD_CANCEL_DEFERRED: _bindgen_ty_10 = _bindgen_ty_10::PTHREAD_CANCEL_DEFERRED;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: _bindgen_ty_10 = _bindgen_ty_10::PTHREAD_CANCEL_ASYNCHRONOUS;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_10 {
    PTHREAD_CANCEL_DEFERRED = 0,
    PTHREAD_CANCEL_ASYNCHRONOUS = 1,
}
extern "C" {
    pub fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: ::std::option::Option<
            unsafe extern "C" fn(
                __newthread: *mut ::std::os::raw::c_void,
            ) -> *mut ::std::os::raw::c_void,
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_exit(__retval: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_tryjoin_np(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_timedjoin_np(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_detach(__th: pthread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_self() -> pthread_t;
}
extern "C" {
    pub fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getdetachstate(
        __attr: *const pthread_attr_t,
        __detachstate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getguardsize(
        __attr: *const pthread_attr_t,
        __guardsize: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setguardsize(
        __attr: *mut pthread_attr_t,
        __guardsize: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getschedparam(
        __attr: *const pthread_attr_t,
        __param: *mut sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setschedparam(
        __attr: *mut pthread_attr_t,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getschedpolicy(
        __attr: *const pthread_attr_t,
        __policy: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setschedpolicy(
        __attr: *mut pthread_attr_t,
        __policy: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getinheritsched(
        __attr: *const pthread_attr_t,
        __inherit: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setinheritsched(
        __attr: *mut pthread_attr_t,
        __inherit: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getscope(
        __attr: *const pthread_attr_t,
        __scope: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setscope(
        __attr: *mut pthread_attr_t,
        __scope: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getstackaddr(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setstackaddr(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getstack(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut ::std::os::raw::c_void,
        __stacksize: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setstack(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut ::std::os::raw::c_void,
        __stacksize: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_setaffinity_np(
        __attr: *mut pthread_attr_t,
        __cpusetsize: usize,
        __cpuset: *const cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_attr_getaffinity_np(
        __attr: *const pthread_attr_t,
        __cpusetsize: usize,
        __cpuset: *mut cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getattr_default_np(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setattr_default_np(__attr: *const pthread_attr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getattr_np(
        __th: pthread_t,
        __attr: *mut pthread_attr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: ::std::os::raw::c_int,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut ::std::os::raw::c_int,
        __param: *mut sched_param,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setschedprio(
        __target_thread: pthread_t,
        __prio: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getname_np(
        __target_thread: pthread_t,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setname_np(
        __target_thread: pthread_t,
        __name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getconcurrency() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setconcurrency(__level: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_yield() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: usize,
        __cpuset: *const cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getaffinity_np(
        __th: pthread_t,
        __cpusetsize: usize,
        __cpuset: *mut cpu_set_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setcancelstate(
        __state: ::std::os::raw::c_int,
        __oldstate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_setcanceltype(
        __type: ::std::os::raw::c_int,
        __oldtype: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cancel(__th: pthread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_testcancel();
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__pthread_unwind_buf_t__bindgen_ty_1; 1usize],
    pub __pad: [*mut ::std::os::raw::c_void; 4usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_unwind_buf_t__bindgen_ty_1 {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: ::std::os::raw::c_int,
}
impl Default for __pthread_unwind_buf_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __cancel_arg: *mut ::std::os::raw::c_void,
    pub __do_it: ::std::os::raw::c_int,
    pub __cancel_type: ::std::os::raw::c_int,
}
impl Default for __pthread_cleanup_frame {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __pthread_cleanup_class {
    pub __cancel_routine:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __cancel_arg: *mut ::std::os::raw::c_void,
    pub __do_it: ::std::os::raw::c_int,
    pub __cancel_type: ::std::os::raw::c_int,
}
extern "C" {
    #[link_name = "\u{1}__setdoit"]
    pub fn __pthread_cleanup_class___setdoit(
        this: *mut __pthread_cleanup_class,
        __newval: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}__defer"]
    pub fn __pthread_cleanup_class___defer(this: *mut __pthread_cleanup_class);
}
extern "C" {
    #[link_name = "\u{1}__restore"]
    pub fn __pthread_cleanup_class___restore(this: *const __pthread_cleanup_class);
}
extern "C" {
    #[link_name = "\u{1}__pthread_cleanup_class"]
    pub fn __pthread_cleanup_class___pthread_cleanup_class(
        this: *mut __pthread_cleanup_class,
        __fct: ::std::option::Option<
            unsafe extern "C" fn(
                this: *mut __pthread_cleanup_class,
                __fct: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[link_name = "\u{1}__pthread_cleanup_class_destructor"]
    pub fn __pthread_cleanup_class___pthread_cleanup_class_destructor(
        this: *mut __pthread_cleanup_class,
    );
}
impl Default for __pthread_cleanup_class {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl __pthread_cleanup_class {
    #[inline]
    pub unsafe fn __setdoit(&mut self, __newval: ::std::os::raw::c_int) {
        __pthread_cleanup_class___setdoit(self, __newval)
    }
    #[inline]
    pub unsafe fn __defer(&mut self) {
        __pthread_cleanup_class___defer(self)
    }
    #[inline]
    pub unsafe fn __restore(&self) {
        __pthread_cleanup_class___restore(self)
    }
    #[inline]
    pub unsafe fn new(
        __fct: ::std::option::Option<
            unsafe extern "C" fn(
                this: *mut __pthread_cleanup_class,
                __fct: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        __pthread_cleanup_class___pthread_cleanup_class(&mut __bindgen_tmp, __fct, __arg);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        __pthread_cleanup_class___pthread_cleanup_class_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
    _unused: [u8; 0],
}
extern "C" {
    pub fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_getprioceiling(
        __mutex: *const pthread_mutex_t,
        __prioceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_setprioceiling(
        __mutex: *mut pthread_mutex_t,
        __prioceiling: ::std::os::raw::c_int,
        __old_ceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent_np(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getpshared(
        __attr: *const pthread_mutexattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprotocol(
        __attr: *const pthread_mutexattr_t,
        __protocol: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprotocol(
        __attr: *mut pthread_mutexattr_t,
        __protocol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprioceiling(
        __attr: *const pthread_mutexattr_t,
        __prioceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprioceiling(
        __attr: *mut pthread_mutexattr_t,
        __prioceiling: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust_np(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust_np(
        __attr: *mut pthread_mutexattr_t,
        __robustness: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_tryrdlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_trywrlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getpshared(
        __attr: *const pthread_rwlockattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setpshared(
        __attr: *mut pthread_rwlockattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getkind_np(
        __attr: *const pthread_rwlockattr_t,
        __pref: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setkind_np(
        __attr: *mut pthread_rwlockattr_t,
        __pref: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_getpshared(
        __attr: *const pthread_condattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_setpshared(
        __attr: *mut pthread_condattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut __clockid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_destroy(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrier_init(
        __barrier: *mut pthread_barrier_t,
        __attr: *const pthread_barrierattr_t,
        __count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrier_destroy(__barrier: *mut pthread_barrier_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrier_wait(__barrier: *mut pthread_barrier_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrierattr_init(__attr: *mut pthread_barrierattr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrierattr_destroy(__attr: *mut pthread_barrierattr_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrierattr_getpshared(
        __attr: *const pthread_barrierattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_barrierattr_setpshared(
        __attr: *mut pthread_barrierattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: ::std::option::Option<
            unsafe extern "C" fn(__key: *mut ::std::os::raw::c_void),
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_key_delete(__key: pthread_key_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getspecific(__key: pthread_key_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_getcpuclockid(
        __thread_id: pthread_t,
        __clock_id: *mut __clockid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pthread_atfork(
        __prepare: ::std::option::Option<unsafe extern "C" fn()>,
        __parent: ::std::option::Option<unsafe extern "C" fn()>,
        __child: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
pub type THREAD_HANDLE = pthread_t;
pub type THREAD_RETURN = *mut ::std::os::raw::c_void;
pub type THREAD_FUNCTION = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
>;
pub type MUTEX = pthread_mutex_t;
#[repr(C)]
#[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XThread {
    pub thread: THREAD_HANDLE,
}
extern "C" {
    #[link_name = "\u{1}start"]
    pub fn XThread_start(
        this: *mut XThread,
        routine: THREAD_FUNCTION,
        ptr: *mut ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}wait"]
    pub fn XThread_wait(
        this: *mut XThread,
        milliseconds: ::std::os::raw::c_long,
        cancel: bool,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}test"]
    pub fn XThread_test();
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn XThread_clear(this: *mut XThread);
}
extern "C" {
    #[link_name = "\u{1}XThread"]
    pub fn XThread_XThread(this: *mut XThread);
}
extern "C" {
    #[link_name = "\u{1}XThread_destructor"]
    pub fn XThread_XThread_destructor(this: *mut XThread);
}
impl XThread {
    #[inline]
    pub unsafe fn start(
        &mut self,
        routine: THREAD_FUNCTION,
        ptr: *mut ::std::os::raw::c_void,
    ) -> bool {
        XThread_start(self, routine, ptr)
    }
    #[inline]
    pub unsafe fn wait(&mut self, milliseconds: ::std::os::raw::c_long, cancel: bool) -> bool {
        XThread_wait(self, milliseconds, cancel)
    }
    #[inline]
    pub unsafe fn test() {
        XThread_test()
    }
    #[inline]
    pub unsafe fn clear(&mut self) {
        XThread_clear(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        XThread_XThread(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        XThread_XThread_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XThreadUtil {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}our_priority"]
    pub static mut XThreadUtil_our_priority: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_priority"]
    pub fn XThreadUtil_set_priority(
        tid: pthread_t,
        priority: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_priority"]
    pub fn XThreadUtil_set_priority1(priority: ::std::os::raw::c_long) -> ::std::os::raw::c_ulong;
}
impl XThreadUtil {
    #[inline]
    pub unsafe fn set_priority(
        tid: pthread_t,
        priority: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong {
        XThreadUtil_set_priority(tid, priority)
    }
    #[inline]
    pub unsafe fn set_priority1(priority: ::std::os::raw::c_long) -> ::std::os::raw::c_ulong {
        XThreadUtil_set_priority1(priority)
    }
}
#[repr(C)]
pub struct XMutex {
    pub mutex: MUTEX,
}
extern "C" {
    #[link_name = "\u{1}acquire"]
    pub fn XMutex_acquire(this: *mut XMutex);
}
extern "C" {
    #[link_name = "\u{1}release"]
    pub fn XMutex_release(this: *mut XMutex);
}
extern "C" {
    #[link_name = "\u{1}XMutex"]
    pub fn XMutex_XMutex(this: *mut XMutex);
}
extern "C" {
    #[link_name = "\u{1}XMutex_destructor"]
    pub fn XMutex_XMutex_destructor(this: *mut XMutex);
}
impl Default for XMutex {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub m_thread_exit: ::std::os::raw::c_ulong,
    pub m_thread: XThread,
    pub m_data_buffer: *mut FastCircularBuffer,
    pub m_bytes_in_buffer: usize,
    pub m_thread_buffer: *mut ::std::os::raw::c_uchar,
    pub m_stream: *mut FILE,
    pub m_msg_buffer: *mut CircularBuffer<XWriteThread_Message>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XWriteThread_Message {
    pub operation: XWriteThread_Message__bindgen_ty_1,
    pub file: *mut FILE,
    pub __bindgen_anon_1: XWriteThread_Message__bindgen_ty_2,
}
pub const XWriteThread_Message_WRITE: XWriteThread_Message__bindgen_ty_1 =
    XWriteThread_Message__bindgen_ty_1::WRITE;
pub const XWriteThread_Message_SEEK: XWriteThread_Message__bindgen_ty_1 =
    XWriteThread_Message__bindgen_ty_1::SEEK;
pub const XWriteThread_Message_FLUSH: XWriteThread_Message__bindgen_ty_1 =
    XWriteThread_Message__bindgen_ty_1::FLUSH;
pub const XWriteThread_Message_CLOSE: XWriteThread_Message__bindgen_ty_1 =
    XWriteThread_Message__bindgen_ty_1::CLOSE;
pub const XWriteThread_Message_SHUTDOWN: XWriteThread_Message__bindgen_ty_1 =
    XWriteThread_Message__bindgen_ty_1::SHUTDOWN;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum XWriteThread_Message__bindgen_ty_1 {
    WRITE = 0,
    SEEK = 1,
    FLUSH = 2,
    CLOSE = 3,
    SHUTDOWN = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union XWriteThread_Message__bindgen_ty_2 {
    pub write: XWriteThread_Message__bindgen_ty_2__bindgen_ty_1,
    pub seek: XWriteThread_Message__bindgen_ty_2__bindgen_ty_2,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XWriteThread_Message__bindgen_ty_2__bindgen_ty_1 {
    pub data_size: usize,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XWriteThread_Message__bindgen_ty_2__bindgen_ty_2 {
    pub offset: ::std::os::raw::c_long,
    pub whence: ::std::os::raw::c_int,
}
impl Default for XWriteThread_Message__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XWriteThread_Message__bindgen_ty_2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "XWriteThread_Message__bindgen_ty_2 {{ union }}")
    }
}
impl Default for XWriteThread_Message {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    pub static mut XWriteThread_o_defaultWriteThread: *mut XWriteThread;
}
extern "C" {
    #[link_name = "\u{1}shared"]
    pub fn XWriteThread_shared() -> *mut XWriteThread;
}
extern "C" {
    #[link_name = "\u{1}fwrite"]
    pub fn XWriteThread_fwrite(
        this: *mut XWriteThread,
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        nitems: usize,
        stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    #[link_name = "\u{1}fseek"]
    pub fn XWriteThread_fseek(
        this: *mut XWriteThread,
        stream: *mut FILE,
        offset: ::std::os::raw::c_long,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}fflush"]
    pub fn XWriteThread_fflush(this: *mut XWriteThread, stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}fclose"]
    pub fn XWriteThread_fclose(this: *mut XWriteThread, stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn XWriteThread_shutdown(this: *mut XWriteThread);
}
extern "C" {
    #[link_name = "\u{1}XWriteThread"]
    pub fn XWriteThread_XWriteThread(
        this: *mut XWriteThread,
        data_buffer_size: usize,
        msg_buffer_size: usize,
    );
}
impl Default for XWriteThread {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl XWriteThread {
    #[inline]
    pub unsafe fn shared() -> *mut XWriteThread {
        XWriteThread_shared()
    }
    #[inline]
    pub unsafe fn fwrite(
        &mut self,
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        nitems: usize,
        stream: *mut FILE,
    ) -> usize {
        XWriteThread_fwrite(self, ptr, size, nitems, stream)
    }
    #[inline]
    pub unsafe fn fseek(
        &mut self,
        stream: *mut FILE,
        offset: ::std::os::raw::c_long,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        XWriteThread_fseek(self, stream, offset, whence)
    }
    #[inline]
    pub unsafe fn fflush(&mut self, stream: *mut FILE) -> ::std::os::raw::c_int {
        XWriteThread_fflush(self, stream)
    }
    #[inline]
    pub unsafe fn fclose(&mut self, stream: *mut FILE) -> ::std::os::raw::c_int {
        XWriteThread_fclose(self, stream)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) {
        XWriteThread_shutdown(self)
    }
    #[inline]
    pub unsafe fn new(data_buffer_size: usize, msg_buffer_size: usize) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        XWriteThread_XWriteThread(&mut __bindgen_tmp, data_buffer_size, msg_buffer_size);
        __bindgen_tmp
    }
}
pub mod __gnu_debug {}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct max_align_t {
    pub __max_align_ll: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __max_align_ld: u128,
}
pub mod __cxxabiv1 {

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __cxa_refcounted_exception {
        _unused: [u8; 0],
    }
    extern "C" {
        pub fn __cxa_allocate_exception(arg1: usize) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn __cxa_free_exception(arg1: *mut ::std::os::raw::c_void);
    }
    extern "C" {
        pub fn __cxa_init_primary_exception(
            object: *mut ::std::os::raw::c_void,
            tinfo: *mut type_info,
            dest: ::std::option::Option<unsafe extern "C" fn(object: *mut ::std::os::raw::c_void)>,
        ) -> *mut __cxa_refcounted_exception;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __class_type_info {
        _unused: [u8; 0],
    }
    #[repr(C)]
    pub struct __forced_unwind__bindgen_vtable(::std::os::raw::c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct __forced_unwind {
        pub vtable_: *const __forced_unwind__bindgen_vtable,
    }
    impl Default for __forced_unwind {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
}
pub type wint_t = ::std::os::raw::c_uint;
pub type mbstate_t = __mbstate_t;
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
    pub fn wcscmp(__s1: *const u32, __s2: *const u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncmp(__s1: *const u32, __s2: *const u32, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscasecmp(__s1: *const u32, __s2: *const u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncasecmp(__s1: *const u32, __s2: *const u32, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscasecmp_l(
        __s1: *const u32,
        __s2: *const u32,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsncasecmp_l(
        __s1: *const u32,
        __s2: *const u32,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcscoll(__s1: *const u32, __s2: *const u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsxfrm(__s1: *mut u32, __s2: *const u32, __n: usize) -> usize;
}
extern "C" {
    pub fn wcscoll_l(__s1: *const u32, __s2: *const u32, __loc: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsxfrm_l(__s1: *mut u32, __s2: *const u32, __n: usize, __loc: locale_t) -> usize;
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
    pub fn wmemcmp(__s1: *const u32, __s2: *const u32, __n: usize) -> ::std::os::raw::c_int;
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
    pub fn btowc(__c: ::std::os::raw::c_int) -> wint_t;
}
extern "C" {
    pub fn wctob(__c: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbsinit(__ps: *const mbstate_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbrtowc(
        __pwc: *mut u32,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
        __p: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcrtomb(__s: *mut ::std::os::raw::c_char, __wc: u32, __ps: *mut mbstate_t) -> usize;
}
extern "C" {
    pub fn __mbrlen(__s: *const ::std::os::raw::c_char, __n: usize, __ps: *mut mbstate_t) -> usize;
}
extern "C" {
    pub fn mbrlen(__s: *const ::std::os::raw::c_char, __n: usize, __ps: *mut mbstate_t) -> usize;
}
extern "C" {
    pub fn __btowc_alias(__c: ::std::os::raw::c_int) -> wint_t;
}
extern "C" {
    pub fn __wctob_alias(__c: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbsrtowcs(
        __dst: *mut u32,
        __src: *mut *const ::std::os::raw::c_char,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsrtombs(
        __dst: *mut ::std::os::raw::c_char,
        __src: *mut *const u32,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn mbsnrtowcs(
        __dst: *mut u32,
        __src: *mut *const ::std::os::raw::c_char,
        __nmc: usize,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsnrtombs(
        __dst: *mut ::std::os::raw::c_char,
        __src: *mut *const u32,
        __nwc: usize,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcwidth(__c: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcswidth(__s: *const u32, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcstod(__nptr: *const u32, __endptr: *mut *mut u32) -> f64;
}
extern "C" {
    pub fn wcstof(__nptr: *const u32, __endptr: *mut *mut u32) -> f32;
}
extern "C" {
    pub fn wcstold(__nptr: *const u32, __endptr: *mut *mut u32) -> u128;
}
extern "C" {
    pub fn wcstof32(__nptr: *const u32, __endptr: *mut *mut u32) -> _Float32;
}
extern "C" {
    pub fn wcstof64(__nptr: *const u32, __endptr: *mut *mut u32) -> _Float64;
}
extern "C" {
    pub fn wcstof32x(__nptr: *const u32, __endptr: *mut *mut u32) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x(__nptr: *const u32, __endptr: *mut *mut u32) -> _Float64x;
}
extern "C" {
    pub fn wcstol(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn wcstoul(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn wcstoll(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstoull(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcstoq(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstouq(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcstol_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn wcstoul_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn wcstoll_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn wcstoull_l(
        __nptr: *const u32,
        __endptr: *mut *mut u32,
        __base: ::std::os::raw::c_int,
        __loc: locale_t,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn wcstod_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> f64;
}
extern "C" {
    pub fn wcstof_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> f32;
}
extern "C" {
    pub fn wcstold_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> u128;
}
extern "C" {
    pub fn wcstof32_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> _Float32;
}
extern "C" {
    pub fn wcstof64_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> _Float64;
}
extern "C" {
    pub fn wcstof32x_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x_l(__nptr: *const u32, __endptr: *mut *mut u32, __loc: locale_t) -> _Float64x;
}
extern "C" {
    pub fn wcpcpy(__dest: *mut u32, __src: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcpncpy(__dest: *mut u32, __src: *const u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn open_wmemstream(__bufloc: *mut *mut u32, __sizeloc: *mut usize) -> *mut __FILE;
}
extern "C" {
    pub fn fwide(__fp: *mut __FILE, __mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fwprintf(__stream: *mut __FILE, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wprintf(__format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swprintf(__s: *mut u32, __n: usize, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfwprintf(
        __s: *mut __FILE,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vwprintf(__format: *const u32, __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vswprintf(
        __s: *mut u32,
        __n: usize,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fwscanf(__stream: *mut __FILE, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wscanf(__format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swscanf(__s: *const u32, __format: *const u32, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfwscanf(
        __s: *mut __FILE,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vwscanf(__format: *const u32, __arg: *mut __va_list_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vswscanf(
        __s: *const u32,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetwc(__stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwc(__stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar() -> wint_t;
}
extern "C" {
    pub fn fputwc(__wc: u32, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwc(__wc: u32, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar(__wc: u32) -> wint_t;
}
extern "C" {
    pub fn fgetws(__ws: *mut u32, __n: ::std::os::raw::c_int, __stream: *mut __FILE) -> *mut u32;
}
extern "C" {
    pub fn fputws(__ws: *const u32, __stream: *mut __FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetwc(__wc: wint_t, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwc_unlocked(__stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar_unlocked() -> wint_t;
}
extern "C" {
    pub fn fgetwc_unlocked(__stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn fputwc_unlocked(__wc: u32, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwc_unlocked(__wc: u32, __stream: *mut __FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar_unlocked(__wc: u32) -> wint_t;
}
extern "C" {
    pub fn fgetws_unlocked(
        __ws: *mut u32,
        __n: ::std::os::raw::c_int,
        __stream: *mut __FILE,
    ) -> *mut u32;
}
extern "C" {
    pub fn fputws_unlocked(__ws: *const u32, __stream: *mut __FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcsftime(
        __s: *mut u32,
        __maxsize: usize,
        __format: *const u32,
        __tp: *const tm,
    ) -> usize;
}
extern "C" {
    pub fn wcsftime_l(
        __s: *mut u32,
        __maxsize: usize,
        __format: *const u32,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct lconv {
    pub decimal_point: *mut ::std::os::raw::c_char,
    pub thousands_sep: *mut ::std::os::raw::c_char,
    pub grouping: *mut ::std::os::raw::c_char,
    pub int_curr_symbol: *mut ::std::os::raw::c_char,
    pub currency_symbol: *mut ::std::os::raw::c_char,
    pub mon_decimal_point: *mut ::std::os::raw::c_char,
    pub mon_thousands_sep: *mut ::std::os::raw::c_char,
    pub mon_grouping: *mut ::std::os::raw::c_char,
    pub positive_sign: *mut ::std::os::raw::c_char,
    pub negative_sign: *mut ::std::os::raw::c_char,
    pub int_frac_digits: ::std::os::raw::c_char,
    pub frac_digits: ::std::os::raw::c_char,
    pub p_cs_precedes: ::std::os::raw::c_char,
    pub p_sep_by_space: ::std::os::raw::c_char,
    pub n_cs_precedes: ::std::os::raw::c_char,
    pub n_sep_by_space: ::std::os::raw::c_char,
    pub p_sign_posn: ::std::os::raw::c_char,
    pub n_sign_posn: ::std::os::raw::c_char,
    pub int_p_cs_precedes: ::std::os::raw::c_char,
    pub int_p_sep_by_space: ::std::os::raw::c_char,
    pub int_n_cs_precedes: ::std::os::raw::c_char,
    pub int_n_sep_by_space: ::std::os::raw::c_char,
    pub int_p_sign_posn: ::std::os::raw::c_char,
    pub int_n_sign_posn: ::std::os::raw::c_char,
}
impl Default for lconv {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn setlocale(
        __category: ::std::os::raw::c_int,
        __locale: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn localeconv() -> *mut lconv;
}
extern "C" {
    pub fn newlocale(
        __category_mask: ::std::os::raw::c_int,
        __locale: *const ::std::os::raw::c_char,
        __base: locale_t,
    ) -> locale_t;
}
extern "C" {
    pub fn duplocale(__dataset: locale_t) -> locale_t;
}
extern "C" {
    pub fn freelocale(__dataset: locale_t);
}
extern "C" {
    pub fn uselocale(__dataset: locale_t) -> locale_t;
}
pub const _ISupper: _bindgen_ty_50 = _bindgen_ty_50::_ISupper;
pub const _ISlower: _bindgen_ty_50 = _bindgen_ty_50::_ISlower;
pub const _ISalpha: _bindgen_ty_50 = _bindgen_ty_50::_ISalpha;
pub const _ISdigit: _bindgen_ty_50 = _bindgen_ty_50::_ISdigit;
pub const _ISxdigit: _bindgen_ty_50 = _bindgen_ty_50::_ISxdigit;
pub const _ISspace: _bindgen_ty_50 = _bindgen_ty_50::_ISspace;
pub const _ISprint: _bindgen_ty_50 = _bindgen_ty_50::_ISprint;
pub const _ISgraph: _bindgen_ty_50 = _bindgen_ty_50::_ISgraph;
pub const _ISblank: _bindgen_ty_50 = _bindgen_ty_50::_ISblank;
pub const _IScntrl: _bindgen_ty_50 = _bindgen_ty_50::_IScntrl;
pub const _ISpunct: _bindgen_ty_50 = _bindgen_ty_50::_ISpunct;
pub const _ISalnum: _bindgen_ty_50 = _bindgen_ty_50::_ISalnum;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_50 {
    _ISupper = 256,
    _ISlower = 512,
    _ISalpha = 1024,
    _ISdigit = 2048,
    _ISxdigit = 4096,
    _ISspace = 8192,
    _ISprint = 16384,
    _ISgraph = 32768,
    _ISblank = 1,
    _IScntrl = 2,
    _ISpunct = 4,
    _ISalnum = 8,
}
extern "C" {
    pub fn __ctype_b_loc() -> *mut *const ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn isalnum(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isctype(
        __c: ::std::os::raw::c_int,
        __mask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _toupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tolower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalnum_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __tolower_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __toupper_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
pub type __gthread_t = pthread_t;
pub type __gthread_key_t = pthread_key_t;
pub type __gthread_once_t = pthread_once_t;
pub type __gthread_mutex_t = pthread_mutex_t;
pub type __gthread_recursive_mutex_t = pthread_mutex_t;
pub type __gthread_cond_t = pthread_cond_t;
pub type __gthread_time_t = timespec;
pub type _Atomic_word = ::std::os::raw::c_int;
extern "C" {
    pub fn __errno_location() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub static mut program_invocation_name: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut program_invocation_short_name: *mut ::std::os::raw::c_char;
}
pub type error_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ChucK {
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
    pub otf_port: ::std::os::raw::c_long,
    pub otf_thread: pthread_t,
    pub stk_writeThread: *mut XWriteThread,
    pub stk_wvOutMap: map,
}
extern "C" {
    #[link_name = "\u{1}hintIsRealtimeAudio"]
    pub fn Chuck_Carrier_hintIsRealtimeAudio(this: *mut Chuck_Carrier) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Carrier"]
    pub fn Chuck_Carrier_Chuck_Carrier(this: *mut Chuck_Carrier);
}
impl Default for Chuck_Carrier {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Carrier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Carrier {{ chuck: {:?}, compiler: {:?}, env: {:?}, vm: {:?}, chout: {:?}, cherr: {:?}, otf_socket: {:?}, otf_port: {:?}, otf_thread: {:?}, stk_writeThread: {:?}, stk_wvOutMap: {:?} }}" , self . chuck , self . compiler , self . env , self . vm , self . chout , self . cherr , self . otf_socket , self . otf_port , self . otf_thread , self . stk_writeThread , self . stk_wvOutMap )
    }
}
impl ::std::cmp::PartialEq for Chuck_Carrier {
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
    pub unsafe fn hintIsRealtimeAudio(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_Carrier_hintIsRealtimeAudio(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Carrier_Chuck_Carrier(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
pub type wctype_t = ::std::os::raw::c_ulong;
pub const __ISwupper: _bindgen_ty_53 = _bindgen_ty_53::__ISwupper;
pub const __ISwlower: _bindgen_ty_53 = _bindgen_ty_53::__ISwlower;
pub const __ISwalpha: _bindgen_ty_53 = _bindgen_ty_53::__ISwalpha;
pub const __ISwdigit: _bindgen_ty_53 = _bindgen_ty_53::__ISwdigit;
pub const __ISwxdigit: _bindgen_ty_53 = _bindgen_ty_53::__ISwxdigit;
pub const __ISwspace: _bindgen_ty_53 = _bindgen_ty_53::__ISwspace;
pub const __ISwprint: _bindgen_ty_53 = _bindgen_ty_53::__ISwprint;
pub const __ISwgraph: _bindgen_ty_53 = _bindgen_ty_53::__ISwgraph;
pub const __ISwblank: _bindgen_ty_53 = _bindgen_ty_53::__ISwblank;
pub const __ISwcntrl: _bindgen_ty_53 = _bindgen_ty_53::__ISwcntrl;
pub const __ISwpunct: _bindgen_ty_53 = _bindgen_ty_53::__ISwpunct;
pub const __ISwalnum: _bindgen_ty_53 = _bindgen_ty_53::__ISwalnum;
pub const _ISwupper: _bindgen_ty_53 = _bindgen_ty_53::_ISwupper;
pub const _ISwlower: _bindgen_ty_53 = _bindgen_ty_53::_ISwlower;
pub const _ISwalpha: _bindgen_ty_53 = _bindgen_ty_53::_ISwalpha;
pub const _ISwdigit: _bindgen_ty_53 = _bindgen_ty_53::_ISwdigit;
pub const _ISwxdigit: _bindgen_ty_53 = _bindgen_ty_53::_ISwxdigit;
pub const _ISwspace: _bindgen_ty_53 = _bindgen_ty_53::_ISwspace;
pub const _ISwprint: _bindgen_ty_53 = _bindgen_ty_53::_ISwprint;
pub const _ISwgraph: _bindgen_ty_53 = _bindgen_ty_53::_ISwgraph;
pub const _ISwblank: _bindgen_ty_53 = _bindgen_ty_53::_ISwblank;
pub const _ISwcntrl: _bindgen_ty_53 = _bindgen_ty_53::_ISwcntrl;
pub const _ISwpunct: _bindgen_ty_53 = _bindgen_ty_53::_ISwpunct;
pub const _ISwalnum: _bindgen_ty_53 = _bindgen_ty_53::_ISwalnum;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_53 {
    __ISwupper = 0,
    __ISwlower = 1,
    __ISwalpha = 2,
    __ISwdigit = 3,
    __ISwxdigit = 4,
    __ISwspace = 5,
    __ISwprint = 6,
    __ISwgraph = 7,
    __ISwblank = 8,
    __ISwcntrl = 9,
    __ISwpunct = 10,
    __ISwalnum = 11,
    _ISwupper = 16777216,
    _ISwlower = 33554432,
    _ISwalpha = 67108864,
    _ISwdigit = 134217728,
    _ISwxdigit = 268435456,
    _ISwspace = 536870912,
    _ISwprint = 1073741824,
    _ISwgraph = -2147483648,
    _ISwblank = 65536,
    _ISwcntrl = 131072,
    _ISwpunct = 262144,
    _ISwalnum = 524288,
}
extern "C" {
    pub fn iswalnum(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswalpha(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswcntrl(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswdigit(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswgraph(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswlower(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswprint(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswpunct(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswspace(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswupper(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswxdigit(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswblank(__wc: wint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctype(__property: *const ::std::os::raw::c_char) -> wctype_t;
}
extern "C" {
    pub fn iswctype(__wc: wint_t, __desc: wctype_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn towlower(__wc: wint_t) -> wint_t;
}
extern "C" {
    pub fn towupper(__wc: wint_t) -> wint_t;
}
pub type wctrans_t = *const __int32_t;
extern "C" {
    pub fn wctrans(__property: *const ::std::os::raw::c_char) -> wctrans_t;
}
extern "C" {
    pub fn towctrans(__wc: wint_t, __desc: wctrans_t) -> wint_t;
}
extern "C" {
    pub fn iswalnum_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswalpha_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswcntrl_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswdigit_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswgraph_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswlower_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswprint_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswpunct_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswspace_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswupper_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswxdigit_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iswblank_l(__wc: wint_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctype_l(__property: *const ::std::os::raw::c_char, __locale: locale_t) -> wctype_t;
}
extern "C" {
    pub fn iswctype_l(__wc: wint_t, __desc: wctype_t, __locale: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn towlower_l(__wc: wint_t, __locale: locale_t) -> wint_t;
}
extern "C" {
    pub fn towupper_l(__wc: wint_t, __locale: locale_t) -> wint_t;
}
extern "C" {
    pub fn wctrans_l(__property: *const ::std::os::raw::c_char, __locale: locale_t) -> wctrans_t;
}
extern "C" {
    pub fn towctrans_l(__wc: wint_t, __desc: wctrans_t, __locale: locale_t) -> wint_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::std::os::raw::c_ushort,
    pub d_type: ::std::os::raw::c_uchar,
    pub d_name: [::std::os::raw::c_char; 256usize],
}
impl Default for dirent {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for dirent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "dirent {{ d_ino: {:?}, d_off: {:?}, d_reclen: {:?}, d_type: {:?}, d_name: [{}] }}",
            self.d_ino,
            self.d_off,
            self.d_reclen,
            self.d_type,
            self.d_name
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{}{:?}", if i > 0 { ", " } else { "" }, v))
                .collect::<String>()
        )
    }
}
impl ::std::cmp::PartialEq for dirent {
    fn eq(&self, other: &dirent) -> bool {
        self.d_ino == other.d_ino
            && self.d_off == other.d_off
            && self.d_reclen == other.d_reclen
            && self.d_type == other.d_type
            && &self.d_name[..] == &other.d_name[..]
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirent64 {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: ::std::os::raw::c_ushort,
    pub d_type: ::std::os::raw::c_uchar,
    pub d_name: [::std::os::raw::c_char; 256usize],
}
impl Default for dirent64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for dirent64 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "dirent64 {{ d_ino: {:?}, d_off: {:?}, d_reclen: {:?}, d_type: {:?}, d_name: [{}] }}",
            self.d_ino,
            self.d_off,
            self.d_reclen,
            self.d_type,
            self.d_name
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{}{:?}", if i > 0 { ", " } else { "" }, v))
                .collect::<String>()
        )
    }
}
impl ::std::cmp::PartialEq for dirent64 {
    fn eq(&self, other: &dirent64) -> bool {
        self.d_ino == other.d_ino
            && self.d_off == other.d_off
            && self.d_reclen == other.d_reclen
            && self.d_type == other.d_type
            && &self.d_name[..] == &other.d_name[..]
    }
}
pub const DT_UNKNOWN: _bindgen_ty_54 = _bindgen_ty_54::DT_UNKNOWN;
pub const DT_FIFO: _bindgen_ty_54 = _bindgen_ty_54::DT_FIFO;
pub const DT_CHR: _bindgen_ty_54 = _bindgen_ty_54::DT_CHR;
pub const DT_DIR: _bindgen_ty_54 = _bindgen_ty_54::DT_DIR;
pub const DT_BLK: _bindgen_ty_54 = _bindgen_ty_54::DT_BLK;
pub const DT_REG: _bindgen_ty_54 = _bindgen_ty_54::DT_REG;
pub const DT_LNK: _bindgen_ty_54 = _bindgen_ty_54::DT_LNK;
pub const DT_SOCK: _bindgen_ty_54 = _bindgen_ty_54::DT_SOCK;
pub const DT_WHT: _bindgen_ty_54 = _bindgen_ty_54::DT_WHT;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_54 {
    DT_UNKNOWN = 0,
    DT_FIFO = 1,
    DT_CHR = 2,
    DT_DIR = 4,
    DT_BLK = 6,
    DT_REG = 8,
    DT_LNK = 10,
    DT_SOCK = 12,
    DT_WHT = 14,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dirstream {
    _unused: [u8; 0],
}
pub type DIR = __dirstream;
extern "C" {
    pub fn opendir(__name: *const ::std::os::raw::c_char) -> *mut DIR;
}
extern "C" {
    pub fn fdopendir(__fd: ::std::os::raw::c_int) -> *mut DIR;
}
extern "C" {
    pub fn closedir(__dirp: *mut DIR) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
extern "C" {
    pub fn readdir64(__dirp: *mut DIR) -> *mut dirent64;
}
extern "C" {
    pub fn readdir_r(
        __dirp: *mut DIR,
        __entry: *mut dirent,
        __result: *mut *mut dirent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readdir64_r(
        __dirp: *mut DIR,
        __entry: *mut dirent64,
        __result: *mut *mut dirent64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewinddir(__dirp: *mut DIR);
}
extern "C" {
    pub fn seekdir(__dirp: *mut DIR, __pos: ::std::os::raw::c_long);
}
extern "C" {
    pub fn telldir(__dirp: *mut DIR) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn dirfd(__dirp: *mut DIR) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scandir(
        __dir: *const ::std::os::raw::c_char,
        __namelist: *mut *mut *mut dirent,
        __selector: ::std::option::Option<
            unsafe extern "C" fn(__dir: *const dirent) -> ::std::os::raw::c_int,
        >,
        __cmp: ::std::option::Option<
            unsafe extern "C" fn(
                __dir: *mut *const dirent,
                __namelist: *mut *const dirent,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scandir64(
        __dir: *const ::std::os::raw::c_char,
        __namelist: *mut *mut *mut dirent64,
        __selector: ::std::option::Option<
            unsafe extern "C" fn(__dir: *const dirent64) -> ::std::os::raw::c_int,
        >,
        __cmp: ::std::option::Option<
            unsafe extern "C" fn(
                __dir: *mut *const dirent64,
                __namelist: *mut *const dirent64,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scandirat(
        __dfd: ::std::os::raw::c_int,
        __dir: *const ::std::os::raw::c_char,
        __namelist: *mut *mut *mut dirent,
        __selector: ::std::option::Option<
            unsafe extern "C" fn(__dfd: *const dirent) -> ::std::os::raw::c_int,
        >,
        __cmp: ::std::option::Option<
            unsafe extern "C" fn(
                __dfd: *mut *const dirent,
                __dir: *mut *const dirent,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scandirat64(
        __dfd: ::std::os::raw::c_int,
        __dir: *const ::std::os::raw::c_char,
        __namelist: *mut *mut *mut dirent64,
        __selector: ::std::option::Option<
            unsafe extern "C" fn(__dfd: *const dirent64) -> ::std::os::raw::c_int,
        >,
        __cmp: ::std::option::Option<
            unsafe extern "C" fn(
                __dfd: *mut *const dirent64,
                __dir: *mut *const dirent64,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alphasort(__e1: *mut *const dirent, __e2: *mut *const dirent) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alphasort64(
        __e1: *mut *const dirent64,
        __e2: *mut *const dirent64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdirentries(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
        __basep: *mut __off_t,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdirentries64(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
        __basep: *mut __off64_t,
    ) -> __ssize_t;
}
extern "C" {
    pub fn versionsort(__e1: *mut *const dirent, __e2: *mut *const dirent)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn versionsort64(
        __e1: *mut *const dirent64,
        __e2: *mut *const dirent64,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
pub struct Chuck_VM_Object__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_VM_Object {
    pub vtable_: *const Chuck_VM_Object__bindgen_vtable,
    pub m_ref_count: ::std::os::raw::c_ulong,
    pub m_pooled: ::std::os::raw::c_ulong,
    pub m_locked: ::std::os::raw::c_ulong,
    pub m_v_ref: *mut vector,
}
extern "C" {
    #[link_name = "\u{1}our_locks_in_effect"]
    pub static mut Chuck_VM_Object_our_locks_in_effect: ::std::os::raw::c_ulong;
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
impl Default for Chuck_VM_Object {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub fn Chuck_VM_Object_add_ref(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}release"]
    pub fn Chuck_VM_Object_release(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}lock"]
    pub fn Chuck_VM_Object_lock(this: *mut ::std::os::raw::c_void);
}
#[repr(C)]
pub struct Chuck_VTable {
    pub funcs: vector,
}
impl Default for Chuck_VTable {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_VTable {{ funcs: {:?} }}", self.funcs)
    }
}
impl ::std::cmp::PartialEq for Chuck_VTable {
    fn eq(&self, other: &Chuck_VTable) -> bool {
        self.funcs == other.funcs
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_Object {
    pub _base: Chuck_VM_Object,
    pub vtable: *mut Chuck_VTable,
    pub type_ref: *mut Chuck_Type,
    pub size: ::std::os::raw::c_ulong,
    pub data: *mut ::std::os::raw::c_uchar,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Object"]
    pub fn Chuck_Object_Chuck_Object(this: *mut Chuck_Object);
}
impl Default for Chuck_Object {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Chuck_Object {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Object_Chuck_Object(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Object_destructor"]
    pub fn Chuck_Object_Chuck_Object_destructor(this: *mut Chuck_Object);
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_Array {
    pub _base: Chuck_Object,
    pub m_array_type: *mut Chuck_Type,
}
impl Default for Chuck_Array {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Chuck_Array4 {
    pub _base: Chuck_Array,
    pub m_vector: vector,
    pub m_map: map,
    pub m_is_obj: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr(
        this: *mut Chuck_Array4,
        i: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr1(
        this: *mut Chuck_Array4,
        key: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get(
        this: *mut Chuck_Array4,
        i: ::std::os::raw::c_long,
        val: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get1(
        this: *mut Chuck_Array4,
        key: *const string,
        val: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set(
        this: *mut Chuck_Array4,
        i: ::std::os::raw::c_long,
        val: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set1(
        this: *mut Chuck_Array4,
        key: *const string,
        val: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array4_push_back(
        this: *mut Chuck_Array4,
        val: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array4_pop_back(this: *mut Chuck_Array4) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array4_back(
        this: *const Chuck_Array4,
        val: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array4_zero(
        this: *mut Chuck_Array4,
        start: ::std::os::raw::c_ulong,
        end: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4"]
    pub fn Chuck_Array4_Chuck_Array4(
        this: *mut Chuck_Array4,
        is_obj: ::std::os::raw::c_ulong,
        capacity: ::std::os::raw::c_long,
    );
}
impl Default for Chuck_Array4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    pub unsafe fn addr(&mut self, i: ::std::os::raw::c_long) -> ::std::os::raw::c_ulong {
        Chuck_Array4_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const string) -> ::std::os::raw::c_ulong {
        Chuck_Array4_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        i: ::std::os::raw::c_long,
        val: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long {
        Chuck_Array4_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const string,
        val: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long {
        Chuck_Array4_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(
        &mut self,
        i: ::std::os::raw::c_long,
        val: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long {
        Chuck_Array4_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const string,
        val: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long {
        Chuck_Array4_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: ::std::os::raw::c_ulong) -> ::std::os::raw::c_long {
        Chuck_Array4_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> ::std::os::raw::c_long {
        Chuck_Array4_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut ::std::os::raw::c_ulong) -> ::std::os::raw::c_long {
        Chuck_Array4_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: ::std::os::raw::c_ulong, end: ::std::os::raw::c_ulong) {
        Chuck_Array4_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(is_obj: ::std::os::raw::c_ulong, capacity: ::std::os::raw::c_long) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub fn Chuck_Array4_clear(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array4_size(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array4_capacity(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array4_set_size(
        this: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array4_set_capacity(
        this: *mut ::std::os::raw::c_void,
        capacity: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array4_find(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array4_erase(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array4_data_type_size(this: *mut ::std::os::raw::c_void)
        -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array4_data_type_kind(this: *mut ::std::os::raw::c_void)
        -> ::std::os::raw::c_long;
}
#[repr(C)]
pub struct Chuck_Array8 {
    pub _base: Chuck_Array,
    pub m_vector: vector,
    pub m_map: map,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array8_addr(
        this: *mut Chuck_Array8,
        i: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array8_addr1(
        this: *mut Chuck_Array8,
        key: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array8_get(
        this: *mut Chuck_Array8,
        i: ::std::os::raw::c_long,
        val: *mut f64,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array8_get1(
        this: *mut Chuck_Array8,
        key: *const string,
        val: *mut f64,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array8_set(
        this: *mut Chuck_Array8,
        i: ::std::os::raw::c_long,
        val: f64,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array8_set1(
        this: *mut Chuck_Array8,
        key: *const string,
        val: f64,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array8_push_back(this: *mut Chuck_Array8, val: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array8_pop_back(this: *mut Chuck_Array8) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array8_back(this: *const Chuck_Array8, val: *mut f64) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array8_zero(
        this: *mut Chuck_Array8,
        start: ::std::os::raw::c_ulong,
        end: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array8"]
    pub fn Chuck_Array8_Chuck_Array8(this: *mut Chuck_Array8, capacity: ::std::os::raw::c_long);
}
impl Default for Chuck_Array8 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Array8 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Array8 {{ m_vector: {:?}, m_map: {:?} }}",
            self.m_vector, self.m_map
        )
    }
}
impl Chuck_Array8 {
    #[inline]
    pub unsafe fn addr(&mut self, i: ::std::os::raw::c_long) -> ::std::os::raw::c_ulong {
        Chuck_Array8_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const string) -> ::std::os::raw::c_ulong {
        Chuck_Array8_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        i: ::std::os::raw::c_long,
        val: *mut f64,
    ) -> ::std::os::raw::c_long {
        Chuck_Array8_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(&mut self, key: *const string, val: *mut f64) -> ::std::os::raw::c_long {
        Chuck_Array8_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: ::std::os::raw::c_long, val: f64) -> ::std::os::raw::c_long {
        Chuck_Array8_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(&mut self, key: *const string, val: f64) -> ::std::os::raw::c_long {
        Chuck_Array8_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: f64) -> ::std::os::raw::c_long {
        Chuck_Array8_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> ::std::os::raw::c_long {
        Chuck_Array8_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut f64) -> ::std::os::raw::c_long {
        Chuck_Array8_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: ::std::os::raw::c_ulong, end: ::std::os::raw::c_ulong) {
        Chuck_Array8_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: ::std::os::raw::c_long) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub fn Chuck_Array8_clear(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array8_size(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array8_capacity(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array8_set_size(
        this: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array8_set_capacity(
        this: *mut ::std::os::raw::c_void,
        capacity: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array8_find(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array8_erase(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array8_data_type_size(this: *mut ::std::os::raw::c_void)
        -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array8_data_type_kind(this: *mut ::std::os::raw::c_void)
        -> ::std::os::raw::c_long;
}
#[repr(C)]
pub struct Chuck_Array16 {
    pub _base: Chuck_Array,
    pub m_vector: vector,
    pub m_map: map,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array16_addr(
        this: *mut Chuck_Array16,
        i: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array16_addr1(
        this: *mut Chuck_Array16,
        key: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array16_get(
        this: *mut Chuck_Array16,
        i: ::std::os::raw::c_long,
        val: *mut t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array16_get1(
        this: *mut Chuck_Array16,
        key: *const string,
        val: *mut t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array16_set(
        this: *mut Chuck_Array16,
        i: ::std::os::raw::c_long,
        val: *const t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array16_set1(
        this: *mut Chuck_Array16,
        key: *const string,
        val: *const t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array16_push_back(
        this: *mut Chuck_Array16,
        val: *const t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array16_pop_back(this: *mut Chuck_Array16) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array16_back(
        this: *const Chuck_Array16,
        val: *mut t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array16_zero(
        this: *mut Chuck_Array16,
        start: ::std::os::raw::c_ulong,
        end: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array16"]
    pub fn Chuck_Array16_Chuck_Array16(this: *mut Chuck_Array16, capacity: ::std::os::raw::c_long);
}
impl Default for Chuck_Array16 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Array16 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Array16 {{ m_vector: {:?}, m_map: {:?} }}",
            self.m_vector, self.m_map
        )
    }
}
impl Chuck_Array16 {
    #[inline]
    pub unsafe fn addr(&mut self, i: ::std::os::raw::c_long) -> ::std::os::raw::c_ulong {
        Chuck_Array16_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const string) -> ::std::os::raw::c_ulong {
        Chuck_Array16_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        i: ::std::os::raw::c_long,
        val: *mut t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long {
        Chuck_Array16_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const string,
        val: *mut t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long {
        Chuck_Array16_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(
        &mut self,
        i: ::std::os::raw::c_long,
        val: *const t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long {
        Chuck_Array16_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const string,
        val: *const t_CKCOMPLEX,
    ) -> ::std::os::raw::c_long {
        Chuck_Array16_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKCOMPLEX) -> ::std::os::raw::c_long {
        Chuck_Array16_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> ::std::os::raw::c_long {
        Chuck_Array16_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKCOMPLEX) -> ::std::os::raw::c_long {
        Chuck_Array16_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: ::std::os::raw::c_ulong, end: ::std::os::raw::c_ulong) {
        Chuck_Array16_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: ::std::os::raw::c_long) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub fn Chuck_Array16_clear(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array16_size(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array16_capacity(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array16_set_size(
        this: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array16_set_capacity(
        this: *mut ::std::os::raw::c_void,
        capacity: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array16_find(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array16_erase(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array16_data_type_size(
        this: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array16_data_type_kind(
        this: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
#[repr(C)]
pub struct Chuck_Array24 {
    pub _base: Chuck_Array,
    pub m_vector: vector,
    pub m_map: map,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array24_addr(
        this: *mut Chuck_Array24,
        i: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array24_addr1(
        this: *mut Chuck_Array24,
        key: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array24_get(
        this: *mut Chuck_Array24,
        i: ::std::os::raw::c_long,
        val: *mut t_CKVEC3,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array24_get1(
        this: *mut Chuck_Array24,
        key: *const string,
        val: *mut t_CKVEC3,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array24_set(
        this: *mut Chuck_Array24,
        i: ::std::os::raw::c_long,
        val: *const t_CKVEC3,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array24_set1(
        this: *mut Chuck_Array24,
        key: *const string,
        val: *const t_CKVEC3,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array24_push_back(
        this: *mut Chuck_Array24,
        val: *const t_CKVEC3,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array24_pop_back(this: *mut Chuck_Array24) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array24_back(
        this: *const Chuck_Array24,
        val: *mut t_CKVEC3,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array24_zero(
        this: *mut Chuck_Array24,
        start: ::std::os::raw::c_ulong,
        end: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array24"]
    pub fn Chuck_Array24_Chuck_Array24(this: *mut Chuck_Array24, capacity: ::std::os::raw::c_long);
}
impl Default for Chuck_Array24 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Array24 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Array24 {{ m_vector: {:?}, m_map: {:?} }}",
            self.m_vector, self.m_map
        )
    }
}
impl Chuck_Array24 {
    #[inline]
    pub unsafe fn addr(&mut self, i: ::std::os::raw::c_long) -> ::std::os::raw::c_ulong {
        Chuck_Array24_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const string) -> ::std::os::raw::c_ulong {
        Chuck_Array24_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        i: ::std::os::raw::c_long,
        val: *mut t_CKVEC3,
    ) -> ::std::os::raw::c_long {
        Chuck_Array24_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const string,
        val: *mut t_CKVEC3,
    ) -> ::std::os::raw::c_long {
        Chuck_Array24_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(
        &mut self,
        i: ::std::os::raw::c_long,
        val: *const t_CKVEC3,
    ) -> ::std::os::raw::c_long {
        Chuck_Array24_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const string,
        val: *const t_CKVEC3,
    ) -> ::std::os::raw::c_long {
        Chuck_Array24_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKVEC3) -> ::std::os::raw::c_long {
        Chuck_Array24_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> ::std::os::raw::c_long {
        Chuck_Array24_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKVEC3) -> ::std::os::raw::c_long {
        Chuck_Array24_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: ::std::os::raw::c_ulong, end: ::std::os::raw::c_ulong) {
        Chuck_Array24_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: ::std::os::raw::c_long) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub fn Chuck_Array24_clear(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array24_size(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array24_capacity(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array24_set_size(
        this: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array24_set_capacity(
        this: *mut ::std::os::raw::c_void,
        capacity: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array24_find(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array24_erase(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array24_data_type_size(
        this: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array24_data_type_kind(
        this: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
#[repr(C)]
pub struct Chuck_Array32 {
    pub _base: Chuck_Array,
    pub m_vector: vector,
    pub m_map: map,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array32_addr(
        this: *mut Chuck_Array32,
        i: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array32_addr1(
        this: *mut Chuck_Array32,
        key: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array32_get(
        this: *mut Chuck_Array32,
        i: ::std::os::raw::c_long,
        val: *mut t_CKVEC4,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array32_get1(
        this: *mut Chuck_Array32,
        key: *const string,
        val: *mut t_CKVEC4,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array32_set(
        this: *mut Chuck_Array32,
        i: ::std::os::raw::c_long,
        val: *const t_CKVEC4,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array32_set1(
        this: *mut Chuck_Array32,
        key: *const string,
        val: *const t_CKVEC4,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array32_push_back(
        this: *mut Chuck_Array32,
        val: *const t_CKVEC4,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array32_pop_back(this: *mut Chuck_Array32) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array32_back(
        this: *const Chuck_Array32,
        val: *mut t_CKVEC4,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array32_zero(
        this: *mut Chuck_Array32,
        start: ::std::os::raw::c_ulong,
        end: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array32"]
    pub fn Chuck_Array32_Chuck_Array32(this: *mut Chuck_Array32, capacity: ::std::os::raw::c_long);
}
impl Default for Chuck_Array32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Array32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Array32 {{ m_vector: {:?}, m_map: {:?} }}",
            self.m_vector, self.m_map
        )
    }
}
impl Chuck_Array32 {
    #[inline]
    pub unsafe fn addr(&mut self, i: ::std::os::raw::c_long) -> ::std::os::raw::c_ulong {
        Chuck_Array32_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const string) -> ::std::os::raw::c_ulong {
        Chuck_Array32_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        i: ::std::os::raw::c_long,
        val: *mut t_CKVEC4,
    ) -> ::std::os::raw::c_long {
        Chuck_Array32_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const string,
        val: *mut t_CKVEC4,
    ) -> ::std::os::raw::c_long {
        Chuck_Array32_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(
        &mut self,
        i: ::std::os::raw::c_long,
        val: *const t_CKVEC4,
    ) -> ::std::os::raw::c_long {
        Chuck_Array32_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const string,
        val: *const t_CKVEC4,
    ) -> ::std::os::raw::c_long {
        Chuck_Array32_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKVEC4) -> ::std::os::raw::c_long {
        Chuck_Array32_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> ::std::os::raw::c_long {
        Chuck_Array32_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKVEC4) -> ::std::os::raw::c_long {
        Chuck_Array32_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: ::std::os::raw::c_ulong, end: ::std::os::raw::c_ulong) {
        Chuck_Array32_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: ::std::os::raw::c_long) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub fn Chuck_Array32_clear(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array32_size(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array32_capacity(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array32_set_size(
        this: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array32_set_capacity(
        this: *mut ::std::os::raw::c_void,
        capacity: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array32_find(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array32_erase(
        this: *mut ::std::os::raw::c_void,
        key: *const string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array32_data_type_size(
        this: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array32_data_type_kind(
        this: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
#[repr(C)]
pub struct Chuck_Event {
    pub _base: Chuck_Object,
    pub m_queue: queue<deque>,
    pub m_queue_lock: XMutex,
}
extern "C" {
    #[link_name = "\u{1}our_can_wait"]
    pub static mut Chuck_Event_our_can_wait: ::std::os::raw::c_ulong;
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
    pub fn Chuck_Event_remove(
        this: *mut Chuck_Event,
        shred: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_broadcast"]
    pub fn Chuck_Event_queue_broadcast(this: *mut Chuck_Event, event_buffer: *mut CBufferSimple);
}
impl Default for Chuck_Event {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    pub unsafe fn wait(&mut self, shred: *mut Chuck_VM_Shred, vm: *mut Chuck_VM) {
        Chuck_Event_wait(self, shred, vm)
    }
    #[inline]
    pub unsafe fn remove(&mut self, shred: *mut Chuck_VM_Shred) -> ::std::os::raw::c_ulong {
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
    pub m_charptr: *const ::std::os::raw::c_char,
    pub m_str: string,
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_String_set(this: *mut Chuck_String, s: *const string);
}
extern "C" {
    #[link_name = "\u{1}str"]
    pub fn Chuck_String_str(this: *mut Chuck_String) -> *const string;
}
extern "C" {
    #[link_name = "\u{1}c_str"]
    pub fn Chuck_String_c_str(this: *mut Chuck_String) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_String"]
    pub fn Chuck_String_Chuck_String(this: *mut Chuck_String, s: *const string);
}
impl Default for Chuck_String {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
impl Chuck_String {
    #[inline]
    pub unsafe fn set(&mut self, s: *const string) {
        Chuck_String_set(self, s)
    }
    #[inline]
    pub unsafe fn str(&mut self) -> *const string {
        Chuck_String_str(self)
    }
    #[inline]
    pub unsafe fn c_str(&mut self) -> *const ::std::os::raw::c_char {
        Chuck_String_c_str(self)
    }
    #[inline]
    pub unsafe fn new(s: *const string) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub RETURN: *mut ::std::os::raw::c_void,
    pub intArg: ::std::os::raw::c_long,
    pub floatArg: f64,
    pub stringArg: string,
}
impl Default for Chuck_IO_async_args {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_IO_async_args {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_IO_async_args {{ fileio_obj: {:?}, RETURN: {:?}, intArg: {:?}, floatArg: {:?}, stringArg: {:?} }}" , self . fileio_obj , self . RETURN , self . intArg , self . floatArg , self . stringArg )
    }
}
extern "C" {
    #[link_name = "\u{1}INT32"]
    pub static Chuck_IO_INT32: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}INT16"]
    pub static Chuck_IO_INT16: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}INT8"]
    pub static Chuck_IO_INT8: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}MODE_SYNC"]
    pub static Chuck_IO_MODE_SYNC: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}MODE_ASYNC"]
    pub static Chuck_IO_MODE_ASYNC: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO"]
    pub fn Chuck_IO_Chuck_IO(this: *mut Chuck_IO);
}
impl Default for Chuck_IO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub m_flags: ::std::os::raw::c_long,
    pub m_iomode: ::std::os::raw::c_long,
    pub m_io: fstream,
    pub m_dir: *mut DIR,
    pub m_dir_start: ::std::os::raw::c_long,
    pub m_path: string,
    pub m_vmRef: *mut Chuck_VM,
}
extern "C" {
    #[link_name = "\u{1}FLAG_READ_WRITE"]
    pub static Chuck_IO_File_FLAG_READ_WRITE: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_READONLY"]
    pub static Chuck_IO_File_FLAG_READONLY: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_WRITEONLY"]
    pub static Chuck_IO_File_FLAG_WRITEONLY: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_APPEND"]
    pub static Chuck_IO_File_FLAG_APPEND: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}TYPE_ASCII"]
    pub static Chuck_IO_File_TYPE_ASCII: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}TYPE_BINARY"]
    pub static Chuck_IO_File_TYPE_BINARY: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}writeStr_thread"]
    pub fn Chuck_IO_File_writeStr_thread(data: *mut ::std::os::raw::c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeInt_thread"]
    pub fn Chuck_IO_File_writeInt_thread(data: *mut ::std::os::raw::c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeFloat_thread"]
    pub fn Chuck_IO_File_writeFloat_thread(data: *mut ::std::os::raw::c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File"]
    pub fn Chuck_IO_File_Chuck_IO_File(this: *mut Chuck_IO_File, vm: *mut Chuck_VM);
}
impl Default for Chuck_IO_File {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_IO_File {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_IO_File {{ m_flags: {:?}, m_iomode: {:?}, m_io: {:?}, m_dir: {:?}, m_dir_start: {:?}, m_path: {:?}, m_vmRef: {:?} }}" , self . m_flags , self . m_iomode , self . m_io , self . m_dir , self . m_dir_start , self . m_path , self . m_vmRef )
    }
}
impl Chuck_IO_File {
    #[inline]
    pub unsafe fn writeStr_thread(data: *mut ::std::os::raw::c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeStr_thread(data)
    }
    #[inline]
    pub unsafe fn writeInt_thread(data: *mut ::std::os::raw::c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeInt_thread(data)
    }
    #[inline]
    pub unsafe fn writeFloat_thread(data: *mut ::std::os::raw::c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeFloat_thread(data)
    }
    #[inline]
    pub unsafe fn new(vm: *mut Chuck_VM) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
        this: *mut ::std::os::raw::c_void,
        path: *const string,
        flags: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_IO_File_good(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_File_close(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_File_flush(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_File_mode(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_File_mode1(this: *mut ::std::os::raw::c_void, flag: ::std::os::raw::c_long);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_IO_File_size(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}seek"]
    pub fn Chuck_IO_File_seek(this: *mut ::std::os::raw::c_void, pos: ::std::os::raw::c_long);
}
extern "C" {
    #[link_name = "\u{1}tell"]
    pub fn Chuck_IO_File_tell(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}isDir"]
    pub fn Chuck_IO_File_isDir(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}dirList"]
    pub fn Chuck_IO_File_dirList(this: *mut ::std::os::raw::c_void) -> *mut Chuck_Array4;
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_File_readLine(this: *mut ::std::os::raw::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_File_readInt(
        this: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_File_readFloat(this: *mut ::std::os::raw::c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_File_readString(
        this: *mut ::std::os::raw::c_void,
        str: *mut string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_File_eof(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write(this: *mut ::std::os::raw::c_void, val: *const string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write1(this: *mut ::std::os::raw::c_void, val: ::std::os::raw::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write2(
        this: *mut ::std::os::raw::c_void,
        val: ::std::os::raw::c_long,
        flags: ::std::os::raw::c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write3(this: *mut ::std::os::raw::c_void, val: f64);
}
#[repr(C)]
pub struct Chuck_IO_Chout {
    pub _base: Chuck_IO,
    pub m_callback:
        ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>,
    pub m_buffer: stringstream,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Chout_set_output_callback(
        this: *mut Chuck_IO_Chout,
        fp: ::std::option::Option<
            unsafe extern "C" fn(this: *mut Chuck_IO_Chout, fp: *const ::std::os::raw::c_char),
        >,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout"]
    pub fn Chuck_IO_Chout_Chuck_IO_Chout(this: *mut Chuck_IO_Chout, carrier: *mut Chuck_Carrier);
}
impl Default for Chuck_IO_Chout {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        fp: ::std::option::Option<
            unsafe extern "C" fn(this: *mut Chuck_IO_Chout, fp: *const ::std::os::raw::c_char),
        >,
    ) {
        Chuck_IO_Chout_set_output_callback(self, fp)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub fn Chuck_IO_Chout_good(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_Chout_close(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_Chout_flush(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Chout_mode(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Chout_mode1(this: *mut ::std::os::raw::c_void, flag: ::std::os::raw::c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_Chout_readLine(this: *mut ::std::os::raw::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_Chout_readInt(
        this: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_Chout_readFloat(this: *mut ::std::os::raw::c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_Chout_readString(
        this: *mut ::std::os::raw::c_void,
        str: *mut string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Chout_eof(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write(this: *mut ::std::os::raw::c_void, val: *const string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write1(this: *mut ::std::os::raw::c_void, val: ::std::os::raw::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write2(
        this: *mut ::std::os::raw::c_void,
        val: ::std::os::raw::c_long,
        flags: ::std::os::raw::c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write3(this: *mut ::std::os::raw::c_void, val: f64);
}
#[repr(C)]
pub struct Chuck_IO_Cherr {
    pub _base: Chuck_IO,
    pub m_callback:
        ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>,
    pub m_buffer: stringstream,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Cherr_set_output_callback(
        this: *mut Chuck_IO_Cherr,
        fp: ::std::option::Option<
            unsafe extern "C" fn(this: *mut Chuck_IO_Cherr, fp: *const ::std::os::raw::c_char),
        >,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr"]
    pub fn Chuck_IO_Cherr_Chuck_IO_Cherr(this: *mut Chuck_IO_Cherr, carrier: *mut Chuck_Carrier);
}
impl Default for Chuck_IO_Cherr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        fp: ::std::option::Option<
            unsafe extern "C" fn(this: *mut Chuck_IO_Cherr, fp: *const ::std::os::raw::c_char),
        >,
    ) {
        Chuck_IO_Cherr_set_output_callback(self, fp)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub fn Chuck_IO_Cherr_good(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_Cherr_close(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_Cherr_flush(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Cherr_mode(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Cherr_mode1(this: *mut ::std::os::raw::c_void, flag: ::std::os::raw::c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_Cherr_readLine(this: *mut ::std::os::raw::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_Cherr_readInt(
        this: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_Cherr_readFloat(this: *mut ::std::os::raw::c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_Cherr_readString(
        this: *mut ::std::os::raw::c_void,
        str: *mut string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Cherr_eof(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write(this: *mut ::std::os::raw::c_void, val: *const string);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write1(this: *mut ::std::os::raw::c_void, val: ::std::os::raw::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write2(
        this: *mut ::std::os::raw::c_void,
        val: ::std::os::raw::c_long,
        flags: ::std::os::raw::c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write3(this: *mut ::std::os::raw::c_void, val: f64);
}
extern "C" {
    pub static mut g_default_chugin_path: [::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub static mut g_chugin_path_envvar: [::std::os::raw::c_char; 0usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_UAnaBlobProxy {
    _unused: [u8; 0],
}
pub mod Chuck_DL_Api {

    pub type Object = *mut ::std::os::raw::c_void;
    pub type Type = *mut ::std::os::raw::c_void;
    pub type String = *mut ::std::os::raw::c_void;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Api {
        pub vm: *mut Api_VMApi,
        pub object: *mut Api_ObjectApi,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Api_VMApi {
        pub get_srate: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                arg2: *mut Chuck_VM_Shred,
            ) -> ::std::os::raw::c_ulong,
        >,
    }
    extern "C" {
        #[link_name = "\u{1}ObjectApi"]
        pub fn Api_ObjectApi_ObjectApi(this: *mut Api_ObjectApi);
    }
    impl Default for Api_ObjectApi {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl Api_ObjectApi {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            Api_ObjectApi_ObjectApi(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}g_api"]
        pub static mut Api_g_api: Api;
    }
    extern "C" {
        #[link_name = "\u{1}instance"]
        pub fn Api_instance() -> *const Api;
    }
    extern "C" {
        #[link_name = "\u{1}Api"]
        pub fn Api_Api(this: *mut Api);
    }
    impl Default for Api {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    impl Api {
        #[inline]
        pub unsafe fn instance() -> *const Api {
            Api_instance()
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            Api_Api(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
}
pub type CK_DL_API = *const Api;
pub type f_ck_declversion =
    ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_ulong>;
pub type f_ck_query = ::std::option::Option<
    unsafe extern "C" fn(QUERY: *mut Chuck_DL_Query) -> ::std::os::raw::c_ulong,
>;
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
        ARGS: *mut ::std::os::raw::c_void,
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
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_sfun = ::std::option::Option<
    unsafe extern "C" fn(
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_tick = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        in_: f64,
        out: *mut f64,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong,
>;
pub type f_tickf = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        in_: *mut f64,
        out: *mut f64,
        nframes: ::std::os::raw::c_ulong,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong,
>;
pub type f_ctrl = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_cget = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ),
>;
pub type f_pmsg = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        MSG: *const ::std::os::raw::c_char,
        ARGS: *mut ::std::os::raw::c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong,
>;
pub type f_tock = ::std::option::Option<
    unsafe extern "C" fn(
        SELF: *mut Chuck_Object,
        UANA: *mut Chuck_UAna,
        BLOB: *mut Chuck_UAnaBlobProxy,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong,
>;
pub type f_mainthreadhook = ::std::option::Option<
    unsafe extern "C" fn(bindle: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_ulong,
>;
pub type f_mainthreadquit = ::std::option::Option<
    unsafe extern "C" fn(bindle: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_ulong,
>;
pub type f_setname = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query, name: *const ::std::os::raw::c_char),
>;
pub type f_begin_class = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        name: *const ::std::os::raw::c_char,
        parent: *const ::std::os::raw::c_char,
    ),
>;
pub type f_add_ctor =
    ::std::option::Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, ctor: f_ctor)>;
pub type f_add_dtor =
    ::std::option::Option<unsafe extern "C" fn(query: *mut Chuck_DL_Query, dtor: f_dtor)>;
pub type f_add_mfun = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        mfun: f_mfun,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ),
>;
pub type f_add_sfun = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        sfun: f_sfun,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ),
>;
pub type f_add_mvar = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        is_const: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong,
>;
pub type f_add_svar = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        is_const: ::std::os::raw::c_ulong,
        static_addr: *mut ::std::os::raw::c_void,
    ),
>;
pub type f_add_arg = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ),
>;
pub type f_add_ugen_func = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        tick: f_tick,
        pmsg: f_pmsg,
        num_in: ::std::os::raw::c_ulong,
        num_out: ::std::os::raw::c_ulong,
    ),
>;
pub type f_add_ugen_funcf = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        tickf: f_tickf,
        pmsg: f_pmsg,
        num_in: ::std::os::raw::c_ulong,
        num_out: ::std::os::raw::c_ulong,
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
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ),
>;
pub type f_end_class = ::std::option::Option<
    unsafe extern "C" fn(query: *mut Chuck_DL_Query) -> ::std::os::raw::c_ulong,
>;
pub type f_doc_class = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        doc: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong,
>;
pub type f_add_example = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        ex: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong,
>;
pub type f_doc_func = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        doc: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong,
>;
pub type f_doc_var = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Chuck_DL_Query,
        doc: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong,
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
    pub reserved: *mut ::std::os::raw::c_void,
    pub srate: ::std::os::raw::c_ulong,
    pub linepos: ::std::os::raw::c_int,
    pub dll_name: string,
    pub curr_class: *mut Chuck_DL_Class,
    pub curr_func: *mut Chuck_DL_Func,
    pub name: string,
    pub classes: vector,
    pub stack: vector,
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
impl Default for Chuck_DL_Query {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Query {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DL_Query {{ m_carrier: {:?}, setname: {:?}, begin_class: {:?}, add_ctor: {:?}, add_dtor: {:?}, add_mfun: {:?}, add_sfun: {:?}, add_mvar: {:?}, add_svar: {:?}, add_arg: {:?}, add_ugen_func: {:?}, add_ugen_funcf: {:?}, add_ugen_funcf_auto_num_channels: {:?}, add_ugen_ctrl: {:?}, end_class: {:?}, last_var: {:?}, doc_class: {:?}, doc_func: {:?}, doc_var: {:?}, add_ex: {:?}, dll_ref: {:?}, reserved: {:?}, srate: {:?}, linepos: {:?}, dll_name: {:?}, curr_class: {:?}, curr_func: {:?}, name: {:?}, classes: {:?}, stack: {:?} }}" , self . m_carrier , self . setname , self . begin_class , self . add_ctor , self . add_dtor , self . add_mfun , self . add_sfun , self . add_mvar , self . add_svar , self . add_arg , self . add_ugen_func , self . add_ugen_funcf , self . add_ugen_funcf_auto_num_channels , self . add_ugen_ctrl , self . end_class , self . last_var , self . doc_class , self . doc_func , self . doc_var , self . add_ex , self . dll_ref , self . reserved , self . srate , self . linepos , self . dll_name , self . curr_class , self . curr_func , self . name , self . classes , self . stack )
    }
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
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub name: string,
    pub parent: string,
    pub ctors: vector,
    pub dtor: *mut Chuck_DL_Func,
    pub mfuns: vector,
    pub sfuns: vector,
    pub mvars: vector,
    pub svars: vector,
    pub ugen_tick: f_tick,
    pub ugen_tickf: f_tickf,
    pub ugen_pmsg: f_pmsg,
    pub ugen_ctrl: vector,
    pub uana_tock: f_tock,
    pub classes: vector,
    pub current_mvar_offset: ::std::os::raw::c_ulong,
    pub ugen_num_in: ::std::os::raw::c_ulong,
    pub ugen_num_out: ::std::os::raw::c_ulong,
    pub doc: string,
    pub examples: vector,
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Class"]
    pub fn Chuck_DL_Class_Chuck_DL_Class(this: *mut Chuck_DL_Class);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Class_destructor"]
    pub fn Chuck_DL_Class_Chuck_DL_Class_destructor(this: *mut Chuck_DL_Class);
}
impl Default for Chuck_DL_Class {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Class {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DL_Class {{ name: {:?}, parent: {:?}, ctors: {:?}, dtor: {:?}, mfuns: {:?}, sfuns: {:?}, mvars: {:?}, svars: {:?}, ugen_tick: {:?}, ugen_tickf: {:?}, ugen_pmsg: {:?}, ugen_ctrl: {:?}, uana_tock: {:?}, classes: {:?}, current_mvar_offset: {:?}, ugen_num_in: {:?}, ugen_num_out: {:?}, doc: {:?}, examples: {:?} }}" , self . name , self . parent , self . ctors , self . dtor , self . mfuns , self . sfuns , self . mvars , self . svars , self . ugen_tick , self . ugen_tickf , self . ugen_pmsg , self . ugen_ctrl , self . uana_tock , self . classes , self . current_mvar_offset , self . ugen_num_in , self . ugen_num_out , self . doc , self . examples )
    }
}
impl Chuck_DL_Class {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub name: string,
    pub type_: string,
    pub is_const: ::std::os::raw::c_ulong,
    pub static_addr: *mut ::std::os::raw::c_void,
    pub doc: string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Value"]
    pub fn Chuck_DL_Value_Chuck_DL_Value(this: *mut Chuck_DL_Value);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Value"]
    pub fn Chuck_DL_Value_Chuck_DL_Value1(
        this: *mut Chuck_DL_Value,
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
        c: ::std::os::raw::c_ulong,
        a: *mut ::std::os::raw::c_void,
    );
}
impl Default for Chuck_DL_Value {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DL_Value {{ name: {:?}, type: {:?}, is_const: {:?}, static_addr: {:?}, doc: {:?} }}" , self . name , self . type_ , self . is_const , self . static_addr , self . doc )
    }
}
impl Chuck_DL_Value {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_DL_Value_Chuck_DL_Value(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
        c: ::std::os::raw::c_ulong,
        a: *mut ::std::os::raw::c_void,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_DL_Value_Chuck_DL_Value1(&mut __bindgen_tmp, t, n, c, a);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DL_Func {
    pub name: string,
    pub type_: string,
    pub __bindgen_anon_1: Chuck_DL_Func__bindgen_ty_1,
    pub args: vector,
    pub doc: string,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Chuck_DL_Func__bindgen_ty_1 {
    pub ctor: f_ctor,
    pub dtor: f_dtor,
    pub mfun: f_mfun,
    pub sfun: f_sfun,
    pub addr: ::std::os::raw::c_ulong,
    _bindgen_union_align: u64,
}
impl Default for Chuck_DL_Func__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Func__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_DL_Func__bindgen_ty_1 {{ union }}")
    }
}
extern "C" {
    #[link_name = "\u{1}add_arg"]
    pub fn Chuck_DL_Func_add_arg(
        this: *mut Chuck_DL_Func,
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func"]
    pub fn Chuck_DL_Func_Chuck_DL_Func(this: *mut Chuck_DL_Func);
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func"]
    pub fn Chuck_DL_Func_Chuck_DL_Func1(
        this: *mut Chuck_DL_Func,
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
        a: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Func_destructor"]
    pub fn Chuck_DL_Func_Chuck_DL_Func_destructor(this: *mut Chuck_DL_Func);
}
impl Default for Chuck_DL_Func {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Func {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_DL_Func {{ name: {:?}, type: {:?}, __bindgen_anon_1: {:?}, args: {:?}, doc: {:?} }}" , self . name , self . type_ , self . __bindgen_anon_1 , self . args , self . doc )
    }
}
impl Chuck_DL_Func {
    #[inline]
    pub unsafe fn add_arg(
        &mut self,
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
    ) {
        Chuck_DL_Func_add_arg(self, t, n)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_DL_Func_Chuck_DL_Func(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
        a: ::std::os::raw::c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub name: string,
    pub type_: string,
    pub types: vector,
    pub ctrl: f_ctrl,
    pub cget: f_cget,
}
impl Default for Chuck_DL_Ctrl {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
extern "C" {
    pub fn make_new_mfun(
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
        mfun: f_mfun,
    ) -> *mut Chuck_DL_Func;
}
extern "C" {
    pub fn make_new_sfun(
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
        sfun: f_sfun,
    ) -> *mut Chuck_DL_Func;
}
extern "C" {
    pub fn make_new_arg(
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
    ) -> *mut Chuck_DL_Value;
}
extern "C" {
    pub fn make_new_mvar(
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
        c: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_DL_Value;
}
extern "C" {
    pub fn make_new_svar(
        t: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_char,
        c: ::std::os::raw::c_ulong,
        a: *mut ::std::os::raw::c_void,
    ) -> *mut Chuck_DL_Value;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Chuck_DL_Return {
    pub v_int: ::std::os::raw::c_long,
    pub v_uint: ::std::os::raw::c_ulong,
    pub v_float: f64,
    pub v_dur: f64,
    pub v_time: f64,
    pub v_complex: t_CKCOMPLEX,
    pub v_polar: t_CKPOLAR,
    pub v_vec3: t_CKVEC3,
    pub v_vec4: t_CKVEC4,
    pub v_object: *mut Chuck_Object,
    pub v_string: *mut Chuck_String,
    _bindgen_union_align: [u64; 4usize],
}
extern "C" {
    #[link_name = "\u{1}Chuck_DL_Return"]
    pub fn Chuck_DL_Return_Chuck_DL_Return(this: *mut Chuck_DL_Return);
}
impl Default for Chuck_DL_Return {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_DL_Return {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_DL_Return {{ union }}")
    }
}
impl Chuck_DL_Return {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_DL_Return_Chuck_DL_Return(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DLL {
    pub _base: Chuck_VM_Object,
    pub m_handle: *mut ::std::os::raw::c_void,
    pub m_last_error: string,
    pub m_filename: string,
    pub m_id: string,
    pub m_func: string,
    pub m_done_query: ::std::os::raw::c_ulong,
    pub m_version_func: f_ck_declversion,
    pub m_query_func: f_ck_query,
    pub m_query: Chuck_DL_Query,
}
extern "C" {
    #[link_name = "\u{1}load"]
    pub fn Chuck_DLL_load(
        this: *mut Chuck_DLL,
        filename: *const ::std::os::raw::c_char,
        func: *const ::std::os::raw::c_char,
        lazy: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}load"]
    pub fn Chuck_DLL_load1(
        this: *mut Chuck_DLL,
        query_func: f_ck_query,
        lazy: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_addr"]
    pub fn Chuck_DLL_get_addr(
        this: *mut Chuck_DLL,
        symbol: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}last_error"]
    pub fn Chuck_DLL_last_error(this: *const Chuck_DLL) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}unload"]
    pub fn Chuck_DLL_unload(this: *mut Chuck_DLL) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}query"]
    pub fn Chuck_DLL_query(this: *mut Chuck_DLL) -> *const Chuck_DL_Query;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_DLL_good(this: *const Chuck_DLL) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}name"]
    pub fn Chuck_DLL_name(this: *const Chuck_DLL) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_DLL"]
    pub fn Chuck_DLL_Chuck_DLL(
        this: *mut Chuck_DLL,
        carrier: *mut Chuck_Carrier,
        xid: *const ::std::os::raw::c_char,
    );
}
impl Default for Chuck_DLL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        filename: *const ::std::os::raw::c_char,
        func: *const ::std::os::raw::c_char,
        lazy: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        Chuck_DLL_load(self, filename, func, lazy)
    }
    #[inline]
    pub unsafe fn load1(
        &mut self,
        query_func: f_ck_query,
        lazy: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        Chuck_DLL_load1(self, query_func, lazy)
    }
    #[inline]
    pub unsafe fn get_addr(
        &mut self,
        symbol: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void {
        Chuck_DLL_get_addr(self, symbol)
    }
    #[inline]
    pub unsafe fn last_error(&self) -> *const ::std::os::raw::c_char {
        Chuck_DLL_last_error(self)
    }
    #[inline]
    pub unsafe fn unload(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_DLL_unload(self)
    }
    #[inline]
    pub unsafe fn query(&mut self) -> *const Chuck_DL_Query {
        Chuck_DLL_query(self)
    }
    #[inline]
    pub unsafe fn good(&self) -> ::std::os::raw::c_ulong {
        Chuck_DLL_good(self)
    }
    #[inline]
    pub unsafe fn name(&self) -> *const ::std::os::raw::c_char {
        Chuck_DLL_name(self)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier, xid: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_DLL_Chuck_DLL(&mut __bindgen_tmp, carrier, xid);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_DLL_destructor"]
    pub fn Chuck_DLL_Chuck_DLL_destructor(this: *mut Chuck_DLL);
}
extern "C" {
    pub fn _dl_mcount_wrapper_check(__selfpc: *mut ::std::os::raw::c_void);
}
pub type Lmid_t = ::std::os::raw::c_long;
extern "C" {
    pub fn dlopen(
        __file: *const ::std::os::raw::c_char,
        __mode: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn dlclose(__handle: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dlsym(
        __handle: *mut ::std::os::raw::c_void,
        __name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn dlmopen(
        __nsid: Lmid_t,
        __file: *const ::std::os::raw::c_char,
        __mode: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn dlvsym(
        __handle: *mut ::std::os::raw::c_void,
        __name: *const ::std::os::raw::c_char,
        __version: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn dlerror() -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Dl_info {
    pub dli_fname: *const ::std::os::raw::c_char,
    pub dli_fbase: *mut ::std::os::raw::c_void,
    pub dli_sname: *const ::std::os::raw::c_char,
    pub dli_saddr: *mut ::std::os::raw::c_void,
}
impl Default for Dl_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn dladdr(
        __address: *const ::std::os::raw::c_void,
        __info: *mut Dl_info,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dladdr1(
        __address: *const ::std::os::raw::c_void,
        __info: *mut Dl_info,
        __extra_info: *mut *mut ::std::os::raw::c_void,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const RTLD_DL_SYMENT: _bindgen_ty_55 = _bindgen_ty_55::RTLD_DL_SYMENT;
pub const RTLD_DL_LINKMAP: _bindgen_ty_55 = _bindgen_ty_55::RTLD_DL_LINKMAP;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_55 {
    RTLD_DL_SYMENT = 1,
    RTLD_DL_LINKMAP = 2,
}
extern "C" {
    pub fn dlinfo(
        __handle: *mut ::std::os::raw::c_void,
        __request: ::std::os::raw::c_int,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
pub const RTLD_DI_LMID: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_LMID;
pub const RTLD_DI_LINKMAP: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_LINKMAP;
pub const RTLD_DI_CONFIGADDR: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_CONFIGADDR;
pub const RTLD_DI_SERINFO: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_SERINFO;
pub const RTLD_DI_SERINFOSIZE: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_SERINFOSIZE;
pub const RTLD_DI_ORIGIN: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_ORIGIN;
pub const RTLD_DI_PROFILENAME: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_PROFILENAME;
pub const RTLD_DI_PROFILEOUT: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_PROFILEOUT;
pub const RTLD_DI_TLS_MODID: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_TLS_MODID;
pub const RTLD_DI_TLS_DATA: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_TLS_DATA;
pub const RTLD_DI_MAX: _bindgen_ty_56 = _bindgen_ty_56::RTLD_DI_TLS_DATA;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _bindgen_ty_56 {
    RTLD_DI_LMID = 1,
    RTLD_DI_LINKMAP = 2,
    RTLD_DI_CONFIGADDR = 3,
    RTLD_DI_SERINFO = 4,
    RTLD_DI_SERINFOSIZE = 5,
    RTLD_DI_ORIGIN = 6,
    RTLD_DI_PROFILENAME = 7,
    RTLD_DI_PROFILEOUT = 8,
    RTLD_DI_TLS_MODID = 9,
    RTLD_DI_TLS_DATA = 10,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Dl_serpath {
    pub dls_name: *mut ::std::os::raw::c_char,
    pub dls_flags: ::std::os::raw::c_uint,
}
impl Default for Dl_serpath {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Dl_serinfo {
    pub dls_size: usize,
    pub dls_cnt: ::std::os::raw::c_uint,
    pub dls_serpath: [Dl_serpath; 1usize],
}
impl Default for Dl_serinfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn osc_query(query: *mut Chuck_DL_Query) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn osc_ctor(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_dtor(
        SELF: *mut Chuck_Object,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_tick(
        SELF: *mut Chuck_Object,
        in_: f64,
        out: *mut f64,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn osc_pmsg(
        SELF: *mut Chuck_Object,
        MSG: *const ::std::os::raw::c_char,
        ARGS: *mut ::std::os::raw::c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn osc_ctrl_freq(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_cget_freq(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_ctrl_period(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_cget_period(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_ctrl_phase(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_cget_phase(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_ctrl_width(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_cget_width(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_ctrl_sync(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn osc_cget_sync(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn sinosc_tick(
        SELF: *mut Chuck_Object,
        in_: f64,
        out: *mut f64,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn pulseosc_tick(
        SELF: *mut Chuck_Object,
        in_: f64,
        out: *mut f64,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn triosc_tick(
        SELF: *mut Chuck_Object,
        in_: f64,
        out: *mut f64,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn sawosc_ctor(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn sawosc_ctrl_width(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn sqrosc_ctor(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn sqrosc_ctrl_width(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn genX_query(query: *mut Chuck_DL_Query) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn genX_ctor(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn genX_dtor(
        SELF: *mut Chuck_Object,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn genX_tick(
        SELF: *mut Chuck_Object,
        in_: f64,
        out: *mut f64,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn genX_pmsg(
        SELF: *mut Chuck_Object,
        MSG: *const ::std::os::raw::c_char,
        ARGS: *mut ::std::os::raw::c_void,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn genX_lookup(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn genX_coeffs(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn gen5_coeffs(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn gen7_coeffs(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn gen9_coeffs(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn gen10_coeffs(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn gen17_coeffs(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn curve_coeffs(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn warp_coeffs(
        SELF: *mut Chuck_Object,
        ARGS: *mut ::std::os::raw::c_void,
        RETURN: *mut Chuck_DL_Return,
        VM: *mut Chuck_VM,
        SHRED: *mut Chuck_VM_Shred,
        API: CK_DL_API,
    );
}
extern "C" {
    pub fn _asymwarp(inval: f64, k: f64) -> f64;
}
extern "C" {
    pub fn _symwarp(inval: f64, k: f64) -> f64;
}
pub type U_boolList = *mut U_boolList_;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct U_boolList_ {
    pub head: ::std::os::raw::c_ulong,
    pub tail: U_boolList,
}
impl Default for U_boolList_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn checked_malloc(size: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn cc_str(arg1: *mut ::std::os::raw::c_char) -> c_str;
}
extern "C" {
    pub fn U_BoolList(head: ::std::os::raw::c_ulong, tail: U_boolList) -> U_boolList;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S_Symbol_ {
    _unused: [u8; 0],
}
pub type S_Symbol = *mut S_Symbol_;
extern "C" {
    pub fn insert_symbol(arg1: c_constr) -> S_Symbol;
}
extern "C" {
    pub fn S_name(arg1: S_Symbol) -> c_str;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TAB_table_ {
    _unused: [u8; 0],
}
pub type S_table = *mut TAB_table_;
extern "C" {
    pub fn S_empty() -> S_table;
}
extern "C" {
    pub fn S_empty2(size: ::std::os::raw::c_uint) -> S_table;
}
extern "C" {
    pub fn S_enter(t: S_table, sym: S_Symbol, value: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn S_enter2(t: S_table, str: c_constr, value: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn S_look(t: S_table, sym: S_Symbol) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn S_look2(t: S_table, str: c_constr) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn S_beginScope(t: S_table);
}
extern "C" {
    pub fn S_endScope(t: S_table);
}
extern "C" {
    pub fn S_pop(t: S_table);
}
pub type a_Pos = ::std::os::raw::c_int;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ae_Operator {
    ae_op_plus = 0,
    ae_op_minus = 1,
    ae_op_times = 2,
    ae_op_divide = 3,
    ae_op_eq = 4,
    ae_op_neq = 5,
    ae_op_lt = 6,
    ae_op_le = 7,
    ae_op_gt = 8,
    ae_op_ge = 9,
    ae_op_and = 10,
    ae_op_or = 11,
    ae_op_s_or = 12,
    ae_op_s_and = 13,
    ae_op_shift_left = 14,
    ae_op_shift_right = 15,
    ae_op_percent = 16,
    ae_op_s_xor = 17,
    ae_op_chuck = 18,
    ae_op_plus_chuck = 19,
    ae_op_minus_chuck = 20,
    ae_op_times_chuck = 21,
    ae_op_divide_chuck = 22,
    ae_op_s_and_chuck = 23,
    ae_op_s_or_chuck = 24,
    ae_op_s_xor_chuck = 25,
    ae_op_shift_right_chuck = 26,
    ae_op_shift_left_chuck = 27,
    ae_op_percent_chuck = 28,
    ae_op_s_chuck = 29,
    ae_op_plusplus = 30,
    ae_op_minusminus = 31,
    ae_op_tilda = 32,
    ae_op_exclamation = 33,
    ae_op_at_chuck = 34,
    ae_op_unchuck = 35,
    ae_op_upchuck = 36,
    ae_op_spork = 37,
    ae_op_typeof = 38,
    ae_op_sizeof = 39,
    ae_op_new = 40,
    ae_op_arrow_left = 41,
    ae_op_arrow_right = 42,
}
extern "C" {
    pub fn op2str(op: ae_Operator) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ae_Keyword {
    ae_key_this = 0,
    ae_key_me = 1,
    ae_key_func = 2,
    ae_key_public = 3,
    ae_key_protected = 4,
    ae_key_private = 5,
    ae_key_static = 6,
    ae_key_instance = 7,
    ae_key_abstract = 8,
}
pub type a_Program = *mut a_Program_;
pub type a_Section = *mut a_Section_;
pub type a_Stmt_List = *mut a_Stmt_List_;
pub type a_Class_Def = *mut a_Class_Def_;
pub type a_Func_Def = *mut a_Func_Def_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Code_Segment_ {
    _unused: [u8; 0],
}
pub type a_Code_Segment = *mut a_Code_Segment_;
pub type a_Stmt = *mut a_Stmt_;
pub type a_Exp = *mut a_Exp_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Chuck_ {
    _unused: [u8; 0],
}
pub type a_Exp_Chuck = *mut a_Exp_Chuck_;
pub type a_Exp_Binary = *mut a_Exp_Binary_;
pub type a_Exp_Cast = *mut a_Exp_Cast_;
pub type a_Exp_Unary = *mut a_Exp_Unary_;
pub type a_Exp_Postfix = *mut a_Exp_Postfix_;
pub type a_Exp_Primary = *mut a_Exp_Primary_;
pub type a_Exp_Dur = *mut a_Exp_Dur_;
pub type a_Exp_Array = *mut a_Exp_Array_;
pub type a_Exp_Func_Call = *mut a_Exp_Func_Call_;
pub type a_Exp_Dot_Member = *mut a_Exp_Dot_Member_;
pub type a_Exp_If = *mut a_Exp_If_;
pub type a_Exp_Decl = *mut a_Exp_Decl_;
pub type a_Exp_Hack = *mut a_Exp_Hack_;
pub type a_Stmt_Code = *mut a_Stmt_Code_;
pub type a_Stmt_If = *mut a_Stmt_If_;
pub type a_Stmt_While = *mut a_Stmt_While_;
pub type a_Stmt_Until = *mut a_Stmt_Until_;
pub type a_Stmt_For = *mut a_Stmt_For_;
pub type a_Stmt_Loop = *mut a_Stmt_Loop_;
pub type a_Stmt_Switch = *mut a_Stmt_Switch_;
pub type a_Stmt_Break = *mut a_Stmt_Break_;
pub type a_Stmt_Continue = *mut a_Stmt_Continue_;
pub type a_Stmt_Return = *mut a_Stmt_Return_;
pub type a_Stmt_Case = *mut a_Stmt_Case_;
pub type a_Stmt_GotoLabel = *mut a_Stmt_GotoLabel_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Decl_ {
    _unused: [u8; 0],
}
pub type a_Decl = *mut a_Decl_;
pub type a_Var_Decl = *mut a_Var_Decl_;
pub type a_Var_Decl_List = *mut a_Var_Decl_List_;
pub type a_Type_Decl = *mut a_Type_Decl_;
pub type a_Arg_List = *mut a_Arg_List_;
pub type a_Id_List = *mut a_Id_List_;
pub type a_Class_Ext = *mut a_Class_Ext_;
pub type a_Class_Body = *mut a_Class_Body_;
pub type a_Array_Sub = *mut a_Array_Sub_;
pub type a_Complex = *mut a_Complex_;
pub type a_Polar = *mut a_Polar_;
pub type a_Vec = *mut a_Vec_;
pub type t_CKTYPE = *mut Chuck_Type;
pub type t_CKVALUE = *mut Chuck_Value;
pub type t_CKFUNC = *mut Chuck_Func;
pub type t_CKNSPC = *mut Chuck_Namespace;
pub type t_CKVMCODE = *mut Chuck_VM_Code;
extern "C" {
    pub fn new_program(section: a_Section, pos: ::std::os::raw::c_int) -> a_Program;
}
extern "C" {
    pub fn prepend_program(
        section: a_Section,
        program: a_Program,
        pos: ::std::os::raw::c_int,
    ) -> a_Program;
}
extern "C" {
    pub fn new_section_stmt(stmt_list: a_Stmt_List, pos: ::std::os::raw::c_int) -> a_Section;
}
extern "C" {
    pub fn new_section_func_def(func_def: a_Func_Def, pos: ::std::os::raw::c_int) -> a_Section;
}
extern "C" {
    pub fn new_section_class_def(class_def: a_Class_Def, pos: ::std::os::raw::c_int) -> a_Section;
}
extern "C" {
    pub fn new_stmt_list(stmt: a_Stmt, pos: ::std::os::raw::c_int) -> a_Stmt_List;
}
extern "C" {
    pub fn prepend_stmt_list(
        stmt: a_Stmt,
        stmt_list: a_Stmt_List,
        pos: ::std::os::raw::c_int,
    ) -> a_Stmt_List;
}
extern "C" {
    pub fn new_stmt_from_expression(exp: a_Exp, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_code(code: a_Stmt_List, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_while(cond: a_Exp, body: a_Stmt, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_do_while(cond: a_Exp, body: a_Stmt, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_until(cond: a_Exp, body: a_Stmt, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_do_until(cond: a_Exp, body: a_Stmt, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_for(
        c1: a_Stmt,
        c2: a_Stmt,
        c3: a_Exp,
        body: a_Stmt,
        pos: ::std::os::raw::c_int,
    ) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_loop(cond: a_Exp, body: a_Stmt, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_if(
        cond: a_Exp,
        if_body: a_Stmt,
        else_body: a_Stmt,
        pos: ::std::os::raw::c_int,
    ) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_switch(exp: a_Exp, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_break(pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_continue(pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_return(exp: a_Exp, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_label(xid: c_str, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_case(exp: a_Exp, pos: ::std::os::raw::c_int) -> a_Stmt;
}
extern "C" {
    pub fn prepend_expression(exp: a_Exp, list: a_Exp, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_binary(
        lhs: a_Exp,
        oper: ae_Operator,
        rhs: a_Exp,
        pos: ::std::os::raw::c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary(oper: ae_Operator, exp: a_Exp, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary2(
        oper: ae_Operator,
        type_: a_Type_Decl,
        array: a_Array_Sub,
        pos: ::std::os::raw::c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary3(
        oper: ae_Operator,
        code: a_Stmt,
        pos: ::std::os::raw::c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_cast(type_: a_Type_Decl, exp: a_Exp, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_array(
        base: a_Exp,
        indices: a_Array_Sub,
        pos: ::std::os::raw::c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_array_lit(exp_list: a_Array_Sub, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_func_call(base: a_Exp, args: a_Exp, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_member_dot(base: a_Exp, member: c_str, pos: ::std::os::raw::c_int)
        -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_postfix(base: a_Exp, op: ae_Operator, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_dur(base: a_Exp, unit: a_Exp, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_id(xid: c_str, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_int(num: ::std::os::raw::c_long, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_uint(num: ::std::os::raw::c_ulong, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_float(num: f64, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_str(str: c_str, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_char(chr: c_str, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_if(
        cond: a_Exp,
        lhs: a_Exp,
        rhs: a_Exp,
        pos: ::std::os::raw::c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_complex(arg1: a_Complex, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_polar(arg1: a_Polar, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_vec(arg1: a_Vec, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl_external(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: ::std::os::raw::c_int,
        pos: ::std::os::raw::c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl_global(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: ::std::os::raw::c_int,
        pos: ::std::os::raw::c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: ::std::os::raw::c_int,
        pos: ::std::os::raw::c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_hack(exp: a_Exp, pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_nil(pos: ::std::os::raw::c_int) -> a_Exp;
}
extern "C" {
    pub fn new_var_decl_list(var_decl: a_Var_Decl, pos: ::std::os::raw::c_int) -> a_Var_Decl_List;
}
extern "C" {
    pub fn prepend_var_decl_list(
        var_decl: a_Var_Decl,
        list: a_Var_Decl_List,
        pos: ::std::os::raw::c_int,
    ) -> a_Var_Decl_List;
}
extern "C" {
    pub fn new_var_decl(
        xid: c_constr,
        array: a_Array_Sub,
        pos: ::std::os::raw::c_int,
    ) -> a_Var_Decl;
}
extern "C" {
    pub fn new_type_decl(
        xid: a_Id_List,
        ref_: ::std::os::raw::c_int,
        pos: ::std::os::raw::c_int,
    ) -> a_Type_Decl;
}
extern "C" {
    pub fn add_type_decl_array(
        type_decl: a_Type_Decl,
        array: a_Array_Sub,
        pos: ::std::os::raw::c_int,
    ) -> a_Type_Decl;
}
extern "C" {
    pub fn new_arg_list(
        type_decl: a_Type_Decl,
        var_decl: a_Var_Decl,
        pos: ::std::os::raw::c_int,
    ) -> a_Arg_List;
}
extern "C" {
    pub fn prepend_arg_list(
        type_decl: a_Type_Decl,
        var_decl: a_Var_Decl,
        arg_list: a_Arg_List,
        pos: ::std::os::raw::c_int,
    ) -> a_Arg_List;
}
extern "C" {
    pub fn new_array_sub(exp: a_Exp, pos: ::std::os::raw::c_int) -> a_Array_Sub;
}
extern "C" {
    pub fn prepend_array_sub(
        array: a_Array_Sub,
        exp: a_Exp,
        pos: ::std::os::raw::c_int,
    ) -> a_Array_Sub;
}
extern "C" {
    pub fn new_complex(re: a_Exp, pos: ::std::os::raw::c_int) -> a_Complex;
}
extern "C" {
    pub fn new_polar(mod_: a_Exp, pos: ::std::os::raw::c_int) -> a_Polar;
}
extern "C" {
    pub fn new_vec(e: a_Exp, pos: ::std::os::raw::c_int) -> a_Vec;
}
extern "C" {
    pub fn new_class_def(
        class_decl: ae_Keyword,
        xid: a_Id_List,
        ext: a_Class_Ext,
        body: a_Class_Body,
        pos: ::std::os::raw::c_int,
    ) -> a_Class_Def;
}
extern "C" {
    pub fn new_class_body(section: a_Section, pos: ::std::os::raw::c_int) -> a_Class_Body;
}
extern "C" {
    pub fn prepend_class_body(
        section: a_Section,
        body: a_Class_Body,
        pos: ::std::os::raw::c_int,
    ) -> a_Class_Body;
}
extern "C" {
    pub fn new_class_ext(
        extend_id: a_Id_List,
        impl_list: a_Id_List,
        pos: ::std::os::raw::c_int,
    ) -> a_Class_Ext;
}
extern "C" {
    pub fn new_iface_def(
        class_decl: ae_Keyword,
        xid: a_Id_List,
        ext: a_Class_Ext,
        body: a_Class_Body,
        pos: ::std::os::raw::c_int,
    ) -> a_Class_Def;
}
extern "C" {
    pub fn new_id_list(xid: c_constr, pos: ::std::os::raw::c_int) -> a_Id_List;
}
extern "C" {
    pub fn prepend_id_list(xid: c_constr, list: a_Id_List, pos: ::std::os::raw::c_int)
        -> a_Id_List;
}
extern "C" {
    pub fn clean_exp(exp: a_Exp);
}
extern "C" {
    pub fn new_func_def(
        func_decl: ae_Keyword,
        static_decl: ae_Keyword,
        type_decl: a_Type_Decl,
        name: c_str,
        arg_list: a_Arg_List,
        code: a_Stmt,
        pos: ::std::os::raw::c_int,
    ) -> a_Func_Def;
}
extern "C" {
    pub fn delete_id_list(x: a_Id_List);
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Binary_ {
    pub lhs: a_Exp,
    pub op: ae_Operator,
    pub rhs: a_Exp,
    pub ck_func: t_CKFUNC,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Binary_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Cast_ {
    pub type_: a_Type_Decl,
    pub exp: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Cast_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Unary_ {
    pub op: ae_Operator,
    pub exp: a_Exp,
    pub type_: a_Type_Decl,
    pub array: a_Array_Sub,
    pub code: a_Stmt,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Unary_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Postfix_ {
    pub exp: a_Exp,
    pub op: ae_Operator,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Postfix_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Dur_ {
    pub base: a_Exp,
    pub unit: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Dur_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Array_ {
    pub base: a_Exp,
    pub indices: a_Array_Sub,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Array_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Func_Call_ {
    pub func: a_Exp,
    pub args: a_Exp,
    pub ret_type: t_CKTYPE,
    pub ck_func: t_CKFUNC,
    pub ck_vm_code: t_CKVMCODE,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Func_Call_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Dot_Member_ {
    pub base: a_Exp,
    pub t_base: t_CKTYPE,
    pub xid: S_Symbol,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Dot_Member_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_If_ {
    pub cond: a_Exp,
    pub if_exp: a_Exp,
    pub else_exp: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_If_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Decl_ {
    pub type_: a_Type_Decl,
    pub var_decl_list: a_Var_Decl_List,
    pub num_var_decls: ::std::os::raw::c_int,
    pub is_static: ::std::os::raw::c_int,
    pub is_global: ::std::os::raw::c_int,
    pub ck_type: t_CKTYPE,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Decl_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Exp_Hack_ {
    pub exp: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Exp_Hack_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Var_Decl_List_ {
    pub var_decl: a_Var_Decl,
    pub next: a_Var_Decl_List,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Var_Decl_List_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Var_Decl_ {
    pub xid: S_Symbol,
    pub var_decl: a_Var_Decl,
    pub array: a_Array_Sub,
    pub value: t_CKVALUE,
    pub addr: *mut ::std::os::raw::c_void,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Var_Decl_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Type_Decl_ {
    pub xid: a_Id_List,
    pub array: a_Array_Sub,
    pub ref_: ::std::os::raw::c_int,
    pub linepos: ::std::os::raw::c_int,
}
impl Default for a_Type_Decl_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Array_Sub_ {
    pub depth: ::std::os::raw::c_ulong,
    pub exp_list: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
    pub err_num: ::std::os::raw::c_int,
    pub err_pos: ::std::os::raw::c_int,
}
impl Default for a_Array_Sub_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Arg_List_ {
    pub type_decl: a_Type_Decl,
    pub var_decl: a_Var_Decl,
    pub type_: t_CKTYPE,
    pub next: a_Arg_List,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Arg_List_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Complex_ {
    pub re: a_Exp,
    pub im: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Complex_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Polar_ {
    pub mod_: a_Exp,
    pub phase: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Polar_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Vec_ {
    pub args: a_Exp,
    pub numdims: ::std::os::raw::c_int,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
impl Default for a_Vec_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ae_Exp_Primary_Type {
    ae_primary_var = 0,
    ae_primary_num = 1,
    ae_primary_float = 2,
    ae_primary_str = 3,
    ae_primary_array = 4,
    ae_primary_exp = 5,
    ae_primary_hack = 6,
    ae_primary_complex = 7,
    ae_primary_polar = 8,
    ae_primary_vec = 9,
    ae_primary_char = 10,
    ae_primary_nil = 11,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Exp_Primary_ {
    pub s_type: ae_Exp_Primary_Type,
    pub value: t_CKVALUE,
    pub __bindgen_anon_1: a_Exp_Primary___bindgen_ty_1,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Exp_Primary___bindgen_ty_1 {
    pub var: S_Symbol,
    pub num: ::std::os::raw::c_long,
    pub fnum: f64,
    pub str: c_str,
    pub chr: c_str,
    pub array: a_Array_Sub,
    pub exp: a_Exp,
    pub complex: a_Complex,
    pub polar: a_Polar,
    pub vec: a_Vec,
    _bindgen_union_align: u64,
}
impl Default for a_Exp_Primary___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for a_Exp_Primary___bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "a_Exp_Primary___bindgen_ty_1 {{ union }}")
    }
}
impl Default for a_Exp_Primary_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for a_Exp_Primary_ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "a_Exp_Primary_ {{ s_type: {:?}, value: {:?}, __bindgen_anon_1: {:?}, linepos: {:?}, self: {:?} }}" , self . s_type , self . value , self . __bindgen_anon_1 , self . linepos , self . self_ )
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ae_Exp_Type {
    ae_exp_binary = 0,
    ae_exp_unary = 1,
    ae_exp_cast = 2,
    ae_exp_postfix = 3,
    ae_exp_dur = 4,
    ae_exp_primary = 5,
    ae_exp_array = 6,
    ae_exp_func_call = 7,
    ae_exp_dot_member = 8,
    ae_exp_if = 9,
    ae_exp_decl = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ae_Exp_Meta {
    ae_meta_value = 0,
    ae_meta_var = 1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Exp_ {
    pub s_type: ae_Exp_Type,
    pub s_meta: ae_Exp_Meta,
    pub type_: t_CKTYPE,
    pub owner: t_CKNSPC,
    pub next: a_Exp,
    pub group_size: ::std::os::raw::c_ulong,
    pub cast_to: t_CKTYPE,
    pub emit_var: ::std::os::raw::c_ulong,
    pub __bindgen_anon_1: a_Exp___bindgen_ty_1,
    pub linepos: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Exp___bindgen_ty_1 {
    pub binary: a_Exp_Binary_,
    pub unary: a_Exp_Unary_,
    pub cast: a_Exp_Cast_,
    pub postfix: a_Exp_Postfix_,
    pub dur: a_Exp_Dur_,
    pub primary: a_Exp_Primary_,
    pub array: a_Exp_Array_,
    pub func_call: a_Exp_Func_Call_,
    pub dot_member: a_Exp_Dot_Member_,
    pub exp_if: a_Exp_If_,
    pub decl: a_Exp_Decl_,
    _bindgen_union_align: [u64; 7usize],
}
impl Default for a_Exp___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for a_Exp___bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "a_Exp___bindgen_ty_1 {{ union }}")
    }
}
impl Default for a_Exp_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    pub is_do: ::std::os::raw::c_int,
    pub cond: a_Exp,
    pub body: a_Stmt,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_While_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Until_ {
    pub is_do: ::std::os::raw::c_int,
    pub cond: a_Exp,
    pub body: a_Stmt,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_Until_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_For_ {
    pub c1: a_Stmt,
    pub c2: a_Stmt,
    pub c3: a_Exp,
    pub body: a_Stmt,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_For_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Loop_ {
    pub cond: a_Exp,
    pub body: a_Stmt,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_Loop_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Code_ {
    pub stmt_list: a_Stmt_List,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_Code_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_If_ {
    pub cond: a_Exp,
    pub if_body: a_Stmt,
    pub else_body: a_Stmt,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_If_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Switch_ {
    pub val: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_Switch_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Break_ {
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_Break_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Continue_ {
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_Continue_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Return_ {
    pub val: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_Return_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_Case_ {
    pub exp: a_Exp,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_Case_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Stmt_GotoLabel_ {
    pub name: S_Symbol,
    pub linepos: ::std::os::raw::c_int,
    pub self_: a_Stmt,
}
impl Default for a_Stmt_GotoLabel_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ae_Stmt_Type {
    ae_stmt_exp = 0,
    ae_stmt_while = 1,
    ae_stmt_until = 2,
    ae_stmt_for = 3,
    ae_stmt_loop = 4,
    ae_stmt_if = 5,
    ae_stmt_code = 6,
    ae_stmt_switch = 7,
    ae_stmt_break = 8,
    ae_stmt_continue = 9,
    ae_stmt_return = 10,
    ae_stmt_case = 11,
    ae_stmt_gotolabel = 12,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Stmt_ {
    pub s_type: ae_Stmt_Type,
    pub skip: ::std::os::raw::c_int,
    pub __bindgen_anon_1: a_Stmt___bindgen_ty_1,
    pub linepos: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Stmt___bindgen_ty_1 {
    pub stmt_exp: a_Exp,
    pub stmt_code: a_Stmt_Code_,
    pub stmt_while: a_Stmt_While_,
    pub stmt_until: a_Stmt_Until_,
    pub stmt_loop: a_Stmt_Loop_,
    pub stmt_for: a_Stmt_For_,
    pub stmt_if: a_Stmt_If_,
    pub stmt_switch: a_Stmt_Switch_,
    pub stmt_break: a_Stmt_Break_,
    pub stmt_continue: a_Stmt_Continue_,
    pub stmt_return: a_Stmt_Return_,
    pub stmt_case: a_Stmt_Case_,
    pub stmt_gotolabel: a_Stmt_GotoLabel_,
    _bindgen_union_align: [u64; 6usize],
}
impl Default for a_Stmt___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for a_Stmt___bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "a_Stmt___bindgen_ty_1 {{ union }}")
    }
}
impl Default for a_Stmt_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    pub stmt: a_Stmt,
    pub next: a_Stmt_List,
    pub linepos: ::std::os::raw::c_int,
}
impl Default for a_Stmt_List_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Def_ {
    pub decl: ae_Keyword,
    pub name: a_Id_List,
    pub ext: a_Class_Ext,
    pub body: a_Class_Body,
    pub type_: t_CKTYPE,
    pub iface: ::std::os::raw::c_int,
    pub home: t_CKNSPC,
    pub linepos: ::std::os::raw::c_int,
}
impl Default for a_Class_Def_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Ext_ {
    pub extend_id: a_Id_List,
    pub impl_list: a_Id_List,
    pub linepos: ::std::os::raw::c_int,
}
impl Default for a_Class_Ext_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Class_Body_ {
    pub section: a_Section,
    pub next: a_Class_Body,
    pub linepos: ::std::os::raw::c_int,
}
impl Default for a_Class_Body_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Id_List_ {
    pub xid: S_Symbol,
    pub next: a_Id_List,
    pub linepos: ::std::os::raw::c_int,
}
impl Default for a_Id_List_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ae_Func_Type {
    ae_func_user = 0,
    ae_func_builtin = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct a_Func_Def_ {
    pub func_decl: ae_Keyword,
    pub static_decl: ae_Keyword,
    pub type_decl: a_Type_Decl,
    pub ret_type: t_CKTYPE,
    pub name: S_Symbol,
    pub arg_list: a_Arg_List,
    pub code: a_Stmt,
    pub ck_func: t_CKFUNC,
    pub global: ::std::os::raw::c_uint,
    pub s_type: ::std::os::raw::c_uint,
    pub stack_depth: ::std::os::raw::c_uint,
    pub dl_func_ptr: *mut ::std::os::raw::c_void,
    pub linepos: ::std::os::raw::c_int,
}
impl Default for a_Func_Def_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ae_Section_Type {
    ae_section_stmt = 0,
    ae_section_func = 1,
    ae_section_class = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Section_ {
    pub s_type: ae_Section_Type,
    pub __bindgen_anon_1: a_Section___bindgen_ty_1,
    pub linepos: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Section___bindgen_ty_1 {
    pub stmt_list: a_Stmt_List,
    pub class_def: a_Class_Def,
    pub func_def: a_Func_Def,
    _bindgen_union_align: u64,
}
impl Default for a_Section___bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for a_Section___bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "a_Section___bindgen_ty_1 {{ union }}")
    }
}
impl Default for a_Section_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    pub section: a_Section,
    pub next: a_Program,
    pub linepos: ::std::os::raw::c_int,
}
impl Default for a_Program_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub static mut EM_anyErrors: ::std::os::raw::c_ulong;
}
extern "C" {
    pub static mut EM_tokPos: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut EM_lineNum: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut EM_extLineNum: ::std::os::raw::c_int;
}
extern "C" {
    pub fn EM_newline();
}
extern "C" {
    pub fn ck_fprintf_stdout(format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn ck_fprintf_stderr(format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn ck_fflush_stdout();
}
extern "C" {
    pub fn ck_fflush_stderr();
}
extern "C" {
    pub fn ck_vfprintf_stdout(format: *const ::std::os::raw::c_char, args: *mut __va_list_tag);
}
extern "C" {
    pub fn ck_vfprintf_stderr(format: *const ::std::os::raw::c_char, args: *mut __va_list_tag);
}
extern "C" {
    pub fn ck_set_stdout_callback(
        callback: ::std::option::Option<
            unsafe extern "C" fn(callback: *const ::std::os::raw::c_char),
        >,
    );
}
extern "C" {
    pub fn ck_set_stderr_callback(
        callback: ::std::option::Option<
            unsafe extern "C" fn(callback: *const ::std::os::raw::c_char),
        >,
    );
}
#[repr(C)]
pub struct ChuckOutStream {
    pub m_stream: stringstream,
    pub m_callback:
        ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>,
    pub m_isErr: bool,
}
extern "C" {
    #[link_name = "\u{1}set_callback"]
    pub fn ChuckOutStream_set_callback(
        this: *mut ChuckOutStream,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                this: *mut ChuckOutStream,
                callback: *const ::std::os::raw::c_char,
            ),
        >,
    );
}
extern "C" {
    #[link_name = "\u{1}ChuckOutStream"]
    pub fn ChuckOutStream_ChuckOutStream(this: *mut ChuckOutStream, isErr: bool);
}
extern "C" {
    #[link_name = "\u{1}ChuckOutStream_destructor"]
    pub fn ChuckOutStream_ChuckOutStream_destructor(this: *mut ChuckOutStream);
}
impl Default for ChuckOutStream {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ChuckOutStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "ChuckOutStream {{ m_stream: {:?}, m_callback: {:?}, m_isErr: {:?} }}",
            self.m_stream, self.m_callback, self.m_isErr
        )
    }
}
impl ChuckOutStream {
    #[inline]
    pub unsafe fn set_callback(
        &mut self,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                this: *mut ChuckOutStream,
                callback: *const ::std::os::raw::c_char,
            ),
        >,
    ) {
        ChuckOutStream_set_callback(self, callback)
    }
    #[inline]
    pub unsafe fn new(isErr: bool) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        ChuckOutStream_ChuckOutStream(&mut __bindgen_tmp, isErr);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        ChuckOutStream_ChuckOutStream_destructor(self)
    }
}
extern "C" {
    pub static mut g_ck_stdoutstream: ChuckOutStream;
}
extern "C" {
    pub static mut g_ck_stderrstream: ChuckOutStream;
}
extern "C" {
    pub fn EM_log(arg1: ::std::os::raw::c_long, arg2: c_constr, ...);
}
extern "C" {
    pub fn EM_setlog(arg1: ::std::os::raw::c_long);
}
extern "C" {
    pub fn EM_pushlog();
}
extern "C" {
    pub fn EM_poplog();
}
extern "C" {
    pub static mut g_loglevel: ::std::os::raw::c_long;
}
extern "C" {
    pub fn EM_error(arg1: ::std::os::raw::c_int, arg2: c_constr, ...);
}
extern "C" {
    pub fn EM_error2(arg1: ::std::os::raw::c_int, arg2: c_constr, ...);
}
extern "C" {
    pub fn EM_error2b(arg1: ::std::os::raw::c_int, arg2: c_constr, ...);
}
extern "C" {
    pub fn EM_error3(arg1: c_constr, ...);
}
extern "C" {
    pub fn EM_impossible(arg1: c_constr, ...);
}
extern "C" {
    pub fn EM_reset(filename: c_constr, fd: *mut FILE) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn EM_change_file(filename: c_constr);
}
extern "C" {
    pub fn EM_lasterror() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn EM_reset_msg();
}
extern "C" {
    pub fn mini(str: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mini_type(str: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum te_Type {
    te_none = 0,
    te_int = 1,
    te_uint = 2,
    te_single = 3,
    te_float = 4,
    te_double = 5,
    te_time = 6,
    te_dur = 7,
    te_complex = 8,
    te_polar = 9,
    te_string = 10,
    te_thread = 11,
    te_shred = 12,
    te_class = 13,
    te_function = 14,
    te_object = 15,
    te_user = 16,
    te_array = 17,
    te_null = 18,
    te_ugen = 19,
    te_uana = 20,
    te_event = 21,
    te_void = 22,
    te_stdout = 23,
    te_stderr = 24,
    te_adc = 25,
    te_dac = 26,
    te_bunghole = 27,
    te_uanablob = 28,
    te_io = 29,
    te_fileio = 30,
    te_chout = 31,
    te_cherr = 32,
    te_multi = 33,
    te_vec3 = 34,
    te_vec4 = 35,
    te_vector = 36,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum te_GlobalType {
    te_globalInt = 0,
    te_globalFloat = 1,
    te_globalEvent = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum te_HowMuch {
    te_do_all = 0,
    te_do_classes_only = 1,
    te_do_no_classes = 2,
}
#[repr(C)]
pub struct Chuck_Scope {
    pub scope: vector,
    pub commit_map: map,
}
impl Default for Chuck_Scope {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
impl ::std::cmp::PartialEq for Chuck_Scope {
    fn eq(&self, other: &Chuck_Scope) -> bool {
        self.scope == other.scope && self.commit_map == other.commit_map
    }
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
    pub class_data: *mut ::std::os::raw::c_uchar,
    pub class_data_size: ::std::os::raw::c_ulong,
    pub name: string,
    pub pre_ctor: *mut Chuck_VM_Code,
    pub dtor: *mut Chuck_VM_Code,
    pub parent: *mut Chuck_Namespace,
    pub offset: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}lookup_type"]
    pub fn Chuck_Namespace_lookup_type(
        this: *mut Chuck_Namespace,
        name: *const string,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}lookup_type"]
    pub fn Chuck_Namespace_lookup_type1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}lookup_value"]
    pub fn Chuck_Namespace_lookup_value(
        this: *mut Chuck_Namespace,
        name: *const string,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}lookup_value"]
    pub fn Chuck_Namespace_lookup_value1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}lookup_func"]
    pub fn Chuck_Namespace_lookup_func(
        this: *mut Chuck_Namespace,
        name: *const string,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}lookup_func"]
    pub fn Chuck_Namespace_lookup_func1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}commit"]
    pub fn Chuck_Namespace_commit(this: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}rollback"]
    pub fn Chuck_Namespace_rollback(this: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}get_types"]
    pub fn Chuck_Namespace_get_types(this: *mut Chuck_Namespace, out: *mut vector);
}
extern "C" {
    #[link_name = "\u{1}get_values"]
    pub fn Chuck_Namespace_get_values(this: *mut Chuck_Namespace, out: *mut vector);
}
extern "C" {
    #[link_name = "\u{1}get_funcs"]
    pub fn Chuck_Namespace_get_funcs(this: *mut Chuck_Namespace, out: *mut vector);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Namespace"]
    pub fn Chuck_Namespace_Chuck_Namespace(this: *mut Chuck_Namespace);
}
impl Default for Chuck_Namespace {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        name: *const string,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Type {
        Chuck_Namespace_lookup_type(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_type1(
        &mut self,
        name: S_Symbol,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Type {
        Chuck_Namespace_lookup_type1(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_value(
        &mut self,
        name: *const string,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Value {
        Chuck_Namespace_lookup_value(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_value1(
        &mut self,
        name: S_Symbol,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Value {
        Chuck_Namespace_lookup_value1(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_func(
        &mut self,
        name: *const string,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Func {
        Chuck_Namespace_lookup_func(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_func1(
        &mut self,
        name: S_Symbol,
        climb: ::std::os::raw::c_long,
    ) -> *mut Chuck_Func {
        Chuck_Namespace_lookup_func1(self, name, climb)
    }
    #[inline]
    pub unsafe fn commit(&mut self) {
        Chuck_Namespace_commit(self)
    }
    #[inline]
    pub unsafe fn rollback(&mut self) {
        Chuck_Namespace_rollback(self)
    }
    #[inline]
    pub unsafe fn get_types(&mut self, out: *mut vector) {
        Chuck_Namespace_get_types(self, out)
    }
    #[inline]
    pub unsafe fn get_values(&mut self, out: *mut vector) {
        Chuck_Namespace_get_values(self, out)
    }
    #[inline]
    pub unsafe fn get_funcs(&mut self, out: *mut vector) {
        Chuck_Namespace_get_funcs(self, out)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Namespace_Chuck_Namespace(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Namespace_destructor"]
    pub fn Chuck_Namespace_Chuck_Namespace_destructor(this: *mut Chuck_Namespace);
}
#[repr(C)]
pub struct Chuck_Context {
    pub _base: Chuck_VM_Object,
    pub filename: string,
    pub full_path: string,
    pub parse_tree: a_Program,
    pub nspc: *mut Chuck_Namespace,
    pub public_class_def: a_Class_Def,
    pub has_error: ::std::os::raw::c_ulong,
    pub progress: ::std::os::raw::c_ulong,
    pub new_types: vector,
    pub new_values: vector,
    pub new_funcs: vector,
    pub new_nspc: vector,
    pub commit_map: map,
}
pub const Chuck_Context_P_NONE: Chuck_Context__bindgen_ty_1 = Chuck_Context__bindgen_ty_1::P_NONE;
pub const Chuck_Context_P_CLASSES_ONLY: Chuck_Context__bindgen_ty_1 =
    Chuck_Context__bindgen_ty_1::P_CLASSES_ONLY;
pub const Chuck_Context_P_ALL: Chuck_Context__bindgen_ty_1 = Chuck_Context__bindgen_ty_1::P_ALL;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Chuck_Context__bindgen_ty_1 {
    P_NONE = 0,
    P_CLASSES_ONLY = 1,
    P_ALL = 2,
}
extern "C" {
    #[link_name = "\u{1}add_commit_candidate"]
    pub fn Chuck_Context_add_commit_candidate(this: *mut Chuck_Context, nspc: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}commit"]
    pub fn Chuck_Context_commit(this: *mut Chuck_Context);
}
extern "C" {
    #[link_name = "\u{1}rollback"]
    pub fn Chuck_Context_rollback(this: *mut Chuck_Context);
}
extern "C" {
    #[link_name = "\u{1}code"]
    pub fn Chuck_Context_code(this: *mut Chuck_Context) -> *mut Chuck_VM_Code;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Type"]
    pub fn Chuck_Context_new_Chuck_Type(
        this: *mut Chuck_Context,
        env: *mut Chuck_Env,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Value"]
    pub fn Chuck_Context_new_Chuck_Value(
        this: *mut Chuck_Context,
        t: *mut Chuck_Type,
        name: *const string,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Func"]
    pub fn Chuck_Context_new_Chuck_Func(this: *mut Chuck_Context) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Namespace"]
    pub fn Chuck_Context_new_Chuck_Namespace(this: *mut Chuck_Context) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Context"]
    pub fn Chuck_Context_Chuck_Context(this: *mut Chuck_Context);
}
impl Default for Chuck_Context {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Context {{ filename: {:?}, full_path: {:?}, parse_tree: {:?}, nspc: {:?}, public_class_def: {:?}, has_error: {:?}, progress: {:?}, new_types: {:?}, new_values: {:?}, new_funcs: {:?}, new_nspc: {:?}, commit_map: {:?} }}" , self . filename , self . full_path , self . parse_tree , self . nspc , self . public_class_def , self . has_error , self . progress , self . new_types , self . new_values , self . new_funcs , self . new_nspc , self . commit_map )
    }
}
impl Chuck_Context {
    #[inline]
    pub unsafe fn add_commit_candidate(&mut self, nspc: *mut Chuck_Namespace) {
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
    pub unsafe fn code(&mut self) -> *mut Chuck_VM_Code {
        Chuck_Context_code(self)
    }
    #[inline]
    pub unsafe fn new_Chuck_Type(&mut self, env: *mut Chuck_Env) -> *mut Chuck_Type {
        Chuck_Context_new_Chuck_Type(self, env)
    }
    #[inline]
    pub unsafe fn new_Chuck_Value(
        &mut self,
        t: *mut Chuck_Type,
        name: *const string,
    ) -> *mut Chuck_Value {
        Chuck_Context_new_Chuck_Value(self, t, name)
    }
    #[inline]
    pub unsafe fn new_Chuck_Func(&mut self) -> *mut Chuck_Func {
        Chuck_Context_new_Chuck_Func(self)
    }
    #[inline]
    pub unsafe fn new_Chuck_Namespace(&mut self) -> *mut Chuck_Namespace {
        Chuck_Context_new_Chuck_Namespace(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Context_Chuck_Context(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Context_destructor"]
    pub fn Chuck_Context_Chuck_Context_destructor(this: *mut Chuck_Context);
}
#[repr(C)]
pub struct Chuck_Env {
    pub _base: Chuck_VM_Object,
    pub m_carrier: *mut Chuck_Carrier,
    pub global_nspc: *mut Chuck_Namespace,
    pub global_context: Chuck_Context,
    pub user_nspc: *mut Chuck_Namespace,
    pub nspc_stack: vector,
    pub curr: *mut Chuck_Namespace,
    pub class_stack: vector,
    pub class_def: *mut Chuck_Type,
    pub func: *mut Chuck_Func,
    pub class_scope: ::std::os::raw::c_ulong,
    pub contexts: vector,
    pub context: *mut Chuck_Context,
    pub breaks: vector,
    pub key_words: map,
    pub key_types: map,
    pub key_values: map,
    pub deprecated: map,
    pub deprecate_level: ::std::os::raw::c_long,
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
    pub fn Chuck_Env_set_carrier(this: *mut Chuck_Env, carrier: *mut Chuck_Carrier);
}
extern "C" {
    #[link_name = "\u{1}vm"]
    pub fn Chuck_Env_vm(this: *mut Chuck_Env) -> *mut Chuck_VM;
}
extern "C" {
    #[link_name = "\u{1}global"]
    pub fn Chuck_Env_global(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}user"]
    pub fn Chuck_Env_user(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}reset"]
    pub fn Chuck_Env_reset(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}load_user_namespace"]
    pub fn Chuck_Env_load_user_namespace(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}clear_user_namespace"]
    pub fn Chuck_Env_clear_user_namespace(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}nspc_top"]
    pub fn Chuck_Env_nspc_top(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}class_top"]
    pub fn Chuck_Env_class_top(this: *mut Chuck_Env) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}is_global"]
    pub fn Chuck_Env_is_global(this: *mut Chuck_Env) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Env"]
    pub fn Chuck_Env_Chuck_Env(this: *mut Chuck_Env);
}
impl Default for Chuck_Env {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Env {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Env {{ m_carrier: {:?}, global_nspc: {:?}, global_context: {:?}, user_nspc: {:?}, nspc_stack: {:?}, curr: {:?}, class_stack: {:?}, class_def: {:?}, func: {:?}, class_scope: {:?}, contexts: {:?}, context: {:?}, breaks: {:?}, key_words: {:?}, key_types: {:?}, key_values: {:?}, deprecated: {:?}, deprecate_level: {:?}, t_void: {:?}, t_int: {:?}, t_float: {:?}, t_time: {:?}, t_dur: {:?}, t_complex: {:?}, t_polar: {:?}, t_vec3: {:?}, t_vec4: {:?}, t_null: {:?}, t_function: {:?}, t_object: {:?}, t_array: {:?}, t_string: {:?}, t_event: {:?}, t_ugen: {:?}, t_uana: {:?}, t_uanablob: {:?}, t_shred: {:?}, t_io: {:?}, t_fileio: {:?}, t_chout: {:?}, t_cherr: {:?}, t_thread: {:?}, t_class: {:?}, t_dac: {:?}, t_adc: {:?} }}" , self . m_carrier , self . global_nspc , self . global_context , self . user_nspc , self . nspc_stack , self . curr , self . class_stack , self . class_def , self . func , self . class_scope , self . contexts , self . context , self . breaks , self . key_words , self . key_types , self . key_values , self . deprecated , self . deprecate_level , self . t_void , self . t_int , self . t_float , self . t_time , self . t_dur , self . t_complex , self . t_polar , self . t_vec3 , self . t_vec4 , self . t_null , self . t_function , self . t_object , self . t_array , self . t_string , self . t_event , self . t_ugen , self . t_uana , self . t_uanablob , self . t_shred , self . t_io , self . t_fileio , self . t_chout , self . t_cherr , self . t_thread , self . t_class , self . t_dac , self . t_adc )
    }
}
impl Chuck_Env {
    #[inline]
    pub unsafe fn set_carrier(&mut self, carrier: *mut Chuck_Carrier) {
        Chuck_Env_set_carrier(self, carrier)
    }
    #[inline]
    pub unsafe fn vm(&mut self) -> *mut Chuck_VM {
        Chuck_Env_vm(self)
    }
    #[inline]
    pub unsafe fn global(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_global(self)
    }
    #[inline]
    pub unsafe fn user(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_user(self)
    }
    #[inline]
    pub unsafe fn reset(&mut self) {
        Chuck_Env_reset(self)
    }
    #[inline]
    pub unsafe fn load_user_namespace(&mut self) {
        Chuck_Env_load_user_namespace(self)
    }
    #[inline]
    pub unsafe fn clear_user_namespace(&mut self) {
        Chuck_Env_clear_user_namespace(self)
    }
    #[inline]
    pub unsafe fn nspc_top(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_nspc_top(self)
    }
    #[inline]
    pub unsafe fn class_top(&mut self) -> *mut Chuck_Type {
        Chuck_Env_class_top(self)
    }
    #[inline]
    pub unsafe fn is_global(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_Env_is_global(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Env_Chuck_Env(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Env_destructor"]
    pub fn Chuck_Env_Chuck_Env_destructor(this: *mut Chuck_Env);
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_UGen_Info {
    pub _base: Chuck_VM_Object,
    pub tick: f_tick,
    pub tickf: f_tickf,
    pub pmsg: f_pmsg,
    pub num_ins: ::std::os::raw::c_ulong,
    pub num_outs: ::std::os::raw::c_ulong,
    pub tock: f_tock,
    pub num_ins_ana: ::std::os::raw::c_ulong,
    pub num_outs_ana: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen_Info"]
    pub fn Chuck_UGen_Info_Chuck_UGen_Info(this: *mut Chuck_UGen_Info);
}
impl Default for Chuck_UGen_Info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Chuck_UGen_Info {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_UGen_Info_Chuck_UGen_Info(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_Type {
    pub _base: Chuck_VM_Object,
    pub xid: te_Type,
    pub name: string,
    pub parent: *mut Chuck_Type,
    pub size: ::std::os::raw::c_ulong,
    pub owner: *mut Chuck_Namespace,
    pub __bindgen_anon_1: Chuck_Type__bindgen_ty_1,
    pub array_depth: ::std::os::raw::c_ulong,
    pub obj_size: ::std::os::raw::c_ulong,
    pub info: *mut Chuck_Namespace,
    pub func: *mut Chuck_Func,
    pub def: a_Class_Def,
    pub ugen_info: *mut Chuck_UGen_Info,
    pub is_copy: ::std::os::raw::c_ulong,
    pub is_complete: ::std::os::raw::c_ulong,
    pub has_constructor: ::std::os::raw::c_ulong,
    pub has_destructor: ::std::os::raw::c_ulong,
    pub allocator: f_alloc,
    pub doc: string,
    pub examples: vector,
    pub ret: string,
    pub m_env: *mut Chuck_Env,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Chuck_Type__bindgen_ty_1 {
    pub array_type: *mut Chuck_Type,
    pub actual_type: *mut Chuck_Type,
    _bindgen_union_align: u64,
}
impl Default for Chuck_Type__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Type__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_Type__bindgen_ty_1 {{ union }}")
    }
}
extern "C" {
    #[link_name = "\u{1}reset"]
    pub fn Chuck_Type_reset(this: *mut Chuck_Type);
}
extern "C" {
    #[link_name = "\u{1}copy"]
    pub fn Chuck_Type_copy(this: *const Chuck_Type, env: *mut Chuck_Env) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}str"]
    pub fn Chuck_Type_str(this: *mut Chuck_Type) -> *const string;
}
extern "C" {
    #[link_name = "\u{1}c_name"]
    pub fn Chuck_Type_c_name(this: *mut Chuck_Type) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Type"]
    pub fn Chuck_Type_Chuck_Type(
        this: *mut Chuck_Type,
        env: *mut Chuck_Env,
        _id: te_Type,
        _n: *const string,
        _p: *mut Chuck_Type,
        _s: ::std::os::raw::c_ulong,
    );
}
impl Default for Chuck_Type {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Type {{ xid: {:?}, name: {:?}, parent: {:?}, size: {:?}, owner: {:?}, __bindgen_anon_1: {:?}, array_depth: {:?}, obj_size: {:?}, info: {:?}, func: {:?}, def: {:?}, ugen_info: {:?}, is_copy: {:?}, is_complete: {:?}, has_constructor: {:?}, has_destructor: {:?}, allocator: {:?}, doc: {:?}, examples: {:?}, ret: {:?}, m_env: {:?} }}" , self . xid , self . name , self . parent , self . size , self . owner , self . __bindgen_anon_1 , self . array_depth , self . obj_size , self . info , self . func , self . def , self . ugen_info , self . is_copy , self . is_complete , self . has_constructor , self . has_destructor , self . allocator , self . doc , self . examples , self . ret , self . m_env )
    }
}
impl Chuck_Type {
    #[inline]
    pub unsafe fn reset(&mut self) {
        Chuck_Type_reset(self)
    }
    #[inline]
    pub unsafe fn copy(&self, env: *mut Chuck_Env) -> *mut Chuck_Type {
        Chuck_Type_copy(self, env)
    }
    #[inline]
    pub unsafe fn str(&mut self) -> *const string {
        Chuck_Type_str(self)
    }
    #[inline]
    pub unsafe fn c_name(&mut self) -> *const ::std::os::raw::c_char {
        Chuck_Type_c_name(self)
    }
    #[inline]
    pub unsafe fn new(
        env: *mut Chuck_Env,
        _id: te_Type,
        _n: *const string,
        _p: *mut Chuck_Type,
        _s: ::std::os::raw::c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Type_Chuck_Type(&mut __bindgen_tmp, env, _id, _n, _p, _s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Type_destructor"]
    pub fn Chuck_Type_Chuck_Type_destructor(this: *mut Chuck_Type);
}
#[repr(C)]
pub struct Chuck_Value {
    pub _base: Chuck_VM_Object,
    pub type_: *mut Chuck_Type,
    pub name: string,
    pub offset: ::std::os::raw::c_ulong,
    pub addr: *mut ::std::os::raw::c_void,
    pub is_const: ::std::os::raw::c_ulong,
    pub is_member: ::std::os::raw::c_ulong,
    pub is_static: ::std::os::raw::c_ulong,
    pub is_context_global: ::std::os::raw::c_ulong,
    pub is_decl_checked: ::std::os::raw::c_ulong,
    pub is_global: ::std::os::raw::c_ulong,
    pub access: ::std::os::raw::c_ulong,
    pub owner: *mut Chuck_Namespace,
    pub owner_class: *mut Chuck_Type,
    pub func_ref: *mut Chuck_Func,
    pub func_num_overloads: ::std::os::raw::c_long,
    pub doc: string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Value"]
    pub fn Chuck_Value_Chuck_Value(
        this: *mut Chuck_Value,
        t: *mut Chuck_Type,
        n: *const string,
        a: *mut ::std::os::raw::c_void,
        c: ::std::os::raw::c_ulong,
        acc: ::std::os::raw::c_ulong,
        o: *mut Chuck_Namespace,
        oc: *mut Chuck_Type,
        s: ::std::os::raw::c_ulong,
    );
}
impl Default for Chuck_Value {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Value {{ type: {:?}, name: {:?}, offset: {:?}, addr: {:?}, is_const: {:?}, is_member: {:?}, is_static: {:?}, is_context_global: {:?}, is_decl_checked: {:?}, is_global: {:?}, access: {:?}, owner: {:?}, owner_class: {:?}, func_ref: {:?}, func_num_overloads: {:?}, doc: {:?} }}" , self . type_ , self . name , self . offset , self . addr , self . is_const , self . is_member , self . is_static , self . is_context_global , self . is_decl_checked , self . is_global , self . access , self . owner , self . owner_class , self . func_ref , self . func_num_overloads , self . doc )
    }
}
impl Chuck_Value {
    #[inline]
    pub unsafe fn new(
        t: *mut Chuck_Type,
        n: *const string,
        a: *mut ::std::os::raw::c_void,
        c: ::std::os::raw::c_ulong,
        acc: ::std::os::raw::c_ulong,
        o: *mut Chuck_Namespace,
        oc: *mut Chuck_Type,
        s: ::std::os::raw::c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Value_Chuck_Value(&mut __bindgen_tmp, t, n, a, c, acc, o, oc, s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Value_destructor"]
    pub fn Chuck_Value_Chuck_Value_destructor(this: *mut Chuck_Value);
}
#[repr(C)]
pub struct Chuck_Func {
    pub _base: Chuck_VM_Object,
    pub name: string,
    pub def: a_Func_Def,
    pub code: *mut Chuck_VM_Code,
    pub is_member: ::std::os::raw::c_ulong,
    pub vt_index: ::std::os::raw::c_ulong,
    pub value_ref: *mut Chuck_Value,
    pub next: *mut Chuck_Func,
    pub up: *mut Chuck_Value,
    pub doc: string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Func"]
    pub fn Chuck_Func_Chuck_Func(this: *mut Chuck_Func);
}
impl Default for Chuck_Func {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Func {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Func {{ name: {:?}, def: {:?}, code: {:?}, is_member: {:?}, vt_index: {:?}, value_ref: {:?}, next: {:?}, up: {:?}, doc: {:?} }}" , self . name , self . def , self . code , self . is_member , self . vt_index , self . value_ref , self . next , self . up , self . doc )
    }
}
impl Chuck_Func {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Func_Chuck_Func(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Func_destructor"]
    pub fn Chuck_Func_Chuck_Func_destructor(this: *mut Chuck_Func);
}
extern "C" {
    pub fn type_engine_init(carrier: *mut Chuck_Carrier) -> *mut Chuck_Env;
}
extern "C" {
    pub fn type_engine_shutdown(env: *mut Chuck_Env);
}
extern "C" {
    pub fn type_engine_load_context(
        env: *mut Chuck_Env,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_unload_context(env: *mut Chuck_Env) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_check_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        filename: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_make_context(prog: a_Program, filename: *const string)
        -> *mut Chuck_Context;
}
extern "C" {
    pub fn type_engine_check_context(
        env: *mut Chuck_Env,
        context: *mut Chuck_Context,
        how_much: te_HowMuch,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_check_stmt(env: *mut Chuck_Env, stmt: a_Stmt) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_check_exp(env: *mut Chuck_Env, exp: a_Exp) -> t_CKTYPE;
}
extern "C" {
    pub fn type_engine_add_dll(
        env: *mut Chuck_Env,
        dll: *mut Chuck_DLL,
        nspc: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_add_dll2(
        env: *mut Chuck_Env,
        dll: *mut Chuck_DLL,
        dest: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_add_class_from_dl(
        env: *mut Chuck_Env,
        c: *mut Chuck_DL_Class,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn equals(lhs: *mut Chuck_Type, rhs: *mut Chuck_Type) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn isa(lhs: *mut Chuck_Type, rhs: *mut Chuck_Type) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn isprim(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn isobj(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn isfunc(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn iskindofint(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn getkindof(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_import_class_begin(
        env: *mut Chuck_Env,
        type_: *mut Chuck_Type,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        doc: *const ::std::os::raw::c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_import_ugen_begin(
        env: *mut Chuck_Env,
        name: *const ::std::os::raw::c_char,
        parent: *const ::std::os::raw::c_char,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        tick: f_tick,
        tickf: f_tickf,
        pmsg: f_pmsg,
        num_ins: ::std::os::raw::c_ulong,
        num_outs: ::std::os::raw::c_ulong,
        doc: *const ::std::os::raw::c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_import_uana_begin(
        env: *mut Chuck_Env,
        name: *const ::std::os::raw::c_char,
        parent: *const ::std::os::raw::c_char,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        tick: f_tick,
        tock: f_tock,
        pmsg: f_pmsg,
        num_ins: ::std::os::raw::c_ulong,
        num_outs: ::std::os::raw::c_ulong,
        num_ins_ana: ::std::os::raw::c_ulong,
        num_outs_ana: ::std::os::raw::c_ulong,
        doc: *const ::std::os::raw::c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_import_mfun(
        env: *mut Chuck_Env,
        mfun: *mut Chuck_DL_Func,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_import_sfun(
        env: *mut Chuck_Env,
        sfun: *mut Chuck_DL_Func,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_import_mvar(
        env: *mut Chuck_Env,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        is_const: ::std::os::raw::c_ulong,
        doc: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_import_svar(
        env: *mut Chuck_Env,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        is_const: ::std::os::raw::c_ulong,
        addr: ::std::os::raw::c_ulong,
        doc: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_import_ugen_ctrl(
        env: *mut Chuck_Env,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        ctrl: f_ctrl,
        write: ::std::os::raw::c_ulong,
        read: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_import_add_ex(
        env: *mut Chuck_Env,
        ex: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_import_class_end(env: *mut Chuck_Env) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_register_deprecate(
        env: *mut Chuck_Env,
        former: *const string,
        latter: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_check_reserved(
        env: *mut Chuck_Env,
        xid: *const string,
        pos: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_check_primitive(
        env: *mut Chuck_Env,
        type_: *mut Chuck_Type,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_compat_func(
        lhs: a_Func_Def,
        rhs: a_Func_Def,
        pos: ::std::os::raw::c_int,
        err: *mut string,
        print: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_get_deprecate(
        env: *mut Chuck_Env,
        from: *const string,
        to: *mut string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_find_common_anc(
        lhs: *mut Chuck_Type,
        rhs: *mut Chuck_Type,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_find_type(env: *mut Chuck_Env, path: a_Id_List) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_find_value(type_: *mut Chuck_Type, xid: *const string) -> *mut Chuck_Value;
}
extern "C" {
    pub fn type_engine_find_nspc(env: *mut Chuck_Env, path: a_Id_List) -> *mut Chuck_Namespace;
}
extern "C" {
    #[doc = " spencer: added this into function to provide the same logic path"]
    #[doc = " for type_engine_check_exp_decl() and ck_add_mvar() when they determine"]
    #[doc = " offsets for mvars -- added 1.3.0.0"]
    pub fn type_engine_next_offset(
        current_offset: ::std::os::raw::c_ulong,
        type_: *mut Chuck_Type,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn verify_array(array: a_Array_Sub) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn new_array_type(
        env: *mut Chuck_Env,
        array_parent: *mut Chuck_Type,
        depth: ::std::os::raw::c_ulong,
        base_type: *mut Chuck_Type,
        owner_nspc: *mut Chuck_Namespace,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_path(path: a_Id_List) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn str2list(path: *const string) -> a_Id_List;
}
extern "C" {
    pub fn howmuch2str(how_much: te_HowMuch) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn escape_str(
        str_lit: *mut ::std::os::raw::c_char,
        linepos: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn str2char(
        char_lit: *const ::std::os::raw::c_char,
        linepos: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Chuck_UGen {
    pub _base: Chuck_Object,
    pub tick: f_tick,
    pub tickf: f_tickf,
    pub pmsg: f_pmsg,
    pub m_multi_chan: *mut *mut Chuck_UGen,
    pub m_multi_chan_size: ::std::os::raw::c_ulong,
    pub m_num_ins: ::std::os::raw::c_ulong,
    pub m_num_outs: ::std::os::raw::c_ulong,
    pub m_src_list: *mut *mut Chuck_UGen,
    pub m_src_cap: ::std::os::raw::c_ulong,
    pub m_num_src: ::std::os::raw::c_ulong,
    pub m_dest_list: *mut *mut Chuck_UGen,
    pub m_dest_cap: ::std::os::raw::c_ulong,
    pub m_num_dest: ::std::os::raw::c_ulong,
    pub m_src_uana_list: *mut *mut Chuck_UGen,
    pub m_src_uana_cap: ::std::os::raw::c_ulong,
    pub m_num_uana_src: ::std::os::raw::c_ulong,
    pub m_dest_uana_list: *mut *mut Chuck_UGen,
    pub m_dest_uana_cap: ::std::os::raw::c_ulong,
    pub m_num_uana_dest: ::std::os::raw::c_ulong,
    pub m_max_src: ::std::os::raw::c_ulong,
    pub m_time: f64,
    pub m_valid: ::std::os::raw::c_ulong,
    pub m_use_next: ::std::os::raw::c_ulong,
    pub m_sum: f64,
    pub m_current: f64,
    pub m_next: f64,
    pub m_last: f64,
    pub m_gain: f64,
    pub m_pan: f64,
    pub m_op: ::std::os::raw::c_long,
    pub m_max_block_size: ::std::os::raw::c_long,
    pub m_multi_in_v: *mut f64,
    pub m_multi_out_v: *mut f64,
    pub m_is_subgraph: ::std::os::raw::c_ulong,
    pub m_inlet: *mut Chuck_UGen,
    pub m_outlet: *mut Chuck_UGen,
    pub m_sum_v: *mut f64,
    pub m_current_v: *mut f64,
    pub shred: *mut Chuck_VM_Shred,
    pub vm: *mut Chuck_VM,
    pub owner: *mut Chuck_UGen,
    pub m_is_uana: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}add"]
    pub fn Chuck_UGen_add(
        this: *mut Chuck_UGen,
        src: *mut Chuck_UGen,
        isUpChuck: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_UGen_remove(
        this: *mut Chuck_UGen,
        src: *mut Chuck_UGen,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_all"]
    pub fn Chuck_UGen_remove_all(this: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}set_max_src"]
    pub fn Chuck_UGen_set_max_src(
        this: *mut Chuck_UGen,
        num: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_num_src"]
    pub fn Chuck_UGen_get_num_src(this: *mut Chuck_UGen) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}is_connected_from"]
    pub fn Chuck_UGen_is_connected_from(
        this: *mut Chuck_UGen,
        src: *mut Chuck_UGen,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}disconnect"]
    pub fn Chuck_UGen_disconnect(
        this: *mut Chuck_UGen,
        recursive: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}system_tick"]
    pub fn Chuck_UGen_system_tick(this: *mut Chuck_UGen, now: f64) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}system_tick_v"]
    pub fn Chuck_UGen_system_tick_v(
        this: *mut Chuck_UGen,
        now: f64,
        numFrames: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}alloc_v"]
    pub fn Chuck_UGen_alloc_v(
        this: *mut Chuck_UGen,
        size: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}src_chan"]
    pub fn Chuck_UGen_src_chan(
        this: *mut Chuck_UGen,
        chan: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}dst_for_src_chan"]
    pub fn Chuck_UGen_dst_for_src_chan(
        this: *mut Chuck_UGen,
        chan: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}add_by"]
    pub fn Chuck_UGen_add_by(
        this: *mut Chuck_UGen,
        dest: *mut Chuck_UGen,
        isUpChuck: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}remove_by"]
    pub fn Chuck_UGen_remove_by(this: *mut Chuck_UGen, dest: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}alloc_multi_chan"]
    pub fn Chuck_UGen_alloc_multi_chan(
        this: *mut Chuck_UGen,
        num_ins: ::std::os::raw::c_ulong,
        num_outs: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}init_subgraph"]
    pub fn Chuck_UGen_init_subgraph(this: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}inlet"]
    pub fn Chuck_UGen_inlet(this: *mut Chuck_UGen) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}outlet"]
    pub fn Chuck_UGen_outlet(this: *mut Chuck_UGen) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen"]
    pub fn Chuck_UGen_Chuck_UGen(this: *mut Chuck_UGen);
}
impl Default for Chuck_UGen {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Chuck_UGen {
    #[inline]
    pub unsafe fn add(
        &mut self,
        src: *mut Chuck_UGen,
        isUpChuck: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        Chuck_UGen_add(self, src, isUpChuck)
    }
    #[inline]
    pub unsafe fn remove(&mut self, src: *mut Chuck_UGen) -> ::std::os::raw::c_ulong {
        Chuck_UGen_remove(self, src)
    }
    #[inline]
    pub unsafe fn remove_all(&mut self) {
        Chuck_UGen_remove_all(self)
    }
    #[inline]
    pub unsafe fn set_max_src(&mut self, num: ::std::os::raw::c_ulong) -> ::std::os::raw::c_ulong {
        Chuck_UGen_set_max_src(self, num)
    }
    #[inline]
    pub unsafe fn get_num_src(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_UGen_get_num_src(self)
    }
    #[inline]
    pub unsafe fn is_connected_from(&mut self, src: *mut Chuck_UGen) -> ::std::os::raw::c_ulong {
        Chuck_UGen_is_connected_from(self, src)
    }
    #[inline]
    pub unsafe fn disconnect(
        &mut self,
        recursive: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        Chuck_UGen_disconnect(self, recursive)
    }
    #[inline]
    pub unsafe fn system_tick(&mut self, now: f64) -> ::std::os::raw::c_ulong {
        Chuck_UGen_system_tick(self, now)
    }
    #[inline]
    pub unsafe fn system_tick_v(
        &mut self,
        now: f64,
        numFrames: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        Chuck_UGen_system_tick_v(self, now, numFrames)
    }
    #[inline]
    pub unsafe fn alloc_v(&mut self, size: ::std::os::raw::c_ulong) -> ::std::os::raw::c_ulong {
        Chuck_UGen_alloc_v(self, size)
    }
    #[inline]
    pub unsafe fn src_chan(&mut self, chan: ::std::os::raw::c_ulong) -> *mut Chuck_UGen {
        Chuck_UGen_src_chan(self, chan)
    }
    #[inline]
    pub unsafe fn dst_for_src_chan(&mut self, chan: ::std::os::raw::c_ulong) -> *mut Chuck_UGen {
        Chuck_UGen_dst_for_src_chan(self, chan)
    }
    #[inline]
    pub unsafe fn add_by(&mut self, dest: *mut Chuck_UGen, isUpChuck: ::std::os::raw::c_ulong) {
        Chuck_UGen_add_by(self, dest, isUpChuck)
    }
    #[inline]
    pub unsafe fn remove_by(&mut self, dest: *mut Chuck_UGen) {
        Chuck_UGen_remove_by(self, dest)
    }
    #[inline]
    pub unsafe fn alloc_multi_chan(
        &mut self,
        num_ins: ::std::os::raw::c_ulong,
        num_outs: ::std::os::raw::c_ulong,
    ) {
        Chuck_UGen_alloc_multi_chan(self, num_ins, num_outs)
    }
    #[inline]
    pub unsafe fn init_subgraph(&mut self) {
        Chuck_UGen_init_subgraph(self)
    }
    #[inline]
    pub unsafe fn inlet(&mut self) -> *mut Chuck_UGen {
        Chuck_UGen_inlet(self)
    }
    #[inline]
    pub unsafe fn outlet(&mut self) -> *mut Chuck_UGen {
        Chuck_UGen_outlet(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_UGen_Chuck_UGen(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen_destructor"]
    pub fn Chuck_UGen_Chuck_UGen_destructor(this: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn Chuck_UGen_init(this: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}done"]
    pub fn Chuck_UGen_done(this: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Chuck_UAna {
    pub _base: Chuck_UGen,
    pub tock: f_tock,
    pub m_uana_time: f64,
}
extern "C" {
    #[link_name = "\u{1}system_tock"]
    pub fn Chuck_UAna_system_tock(this: *mut Chuck_UAna, now: f64) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}is_up_connected_from"]
    pub fn Chuck_UAna_is_up_connected_from(
        this: *mut Chuck_UAna,
        src: *mut Chuck_UAna,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}numIncomingUAnae"]
    pub fn Chuck_UAna_numIncomingUAnae(this: *const Chuck_UAna) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}getIncomingUAna"]
    pub fn Chuck_UAna_getIncomingUAna(
        this: *const Chuck_UAna,
        index: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_UAna;
}
extern "C" {
    #[link_name = "\u{1}getIncomingBlob"]
    pub fn Chuck_UAna_getIncomingBlob(
        this: *const Chuck_UAna,
        index: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_UAnaBlobProxy;
}
extern "C" {
    #[link_name = "\u{1}blobProxy"]
    pub fn Chuck_UAna_blobProxy(this: *const Chuck_UAna) -> *mut Chuck_UAnaBlobProxy;
}
extern "C" {
    #[link_name = "\u{1}Chuck_UAna"]
    pub fn Chuck_UAna_Chuck_UAna(this: *mut Chuck_UAna);
}
impl Default for Chuck_UAna {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Chuck_UAna {
    #[inline]
    pub unsafe fn system_tock(&mut self, now: f64) -> ::std::os::raw::c_ulong {
        Chuck_UAna_system_tock(self, now)
    }
    #[inline]
    pub unsafe fn is_up_connected_from(&mut self, src: *mut Chuck_UAna) -> ::std::os::raw::c_ulong {
        Chuck_UAna_is_up_connected_from(self, src)
    }
    #[inline]
    pub unsafe fn numIncomingUAnae(&self) -> ::std::os::raw::c_long {
        Chuck_UAna_numIncomingUAnae(self)
    }
    #[inline]
    pub unsafe fn getIncomingUAna(&self, index: ::std::os::raw::c_ulong) -> *mut Chuck_UAna {
        Chuck_UAna_getIncomingUAna(self, index)
    }
    #[inline]
    pub unsafe fn getIncomingBlob(
        &self,
        index: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_UAnaBlobProxy {
        Chuck_UAna_getIncomingBlob(self, index)
    }
    #[inline]
    pub unsafe fn blobProxy(&self) -> *mut Chuck_UAnaBlobProxy {
        Chuck_UAna_blobProxy(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_UAna_Chuck_UAna(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_UAna_destructor"]
    pub fn Chuck_UAna_Chuck_UAna_destructor(this: *mut Chuck_UAna);
}
extern "C" {
    pub fn ugen_generic_num_in(
        obj: *mut Chuck_Object,
        isArray: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn ugen_generic_get_src(
        obj: *mut Chuck_Object,
        chan: ::std::os::raw::c_long,
        isArray: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_UGen;
}
extern "C" {
    pub fn ugen_generic_get_dst(
        obj: *mut Chuck_Object,
        chan: ::std::os::raw::c_long,
        isArray: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_UGen;
}
extern "C" {
    pub static mut g_program: a_Program;
}
extern "C" {
    pub fn yyparse() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn yyrestart(arg1: *mut FILE);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yy_buffer_state {
    _unused: [u8; 0],
}
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
extern "C" {
    pub fn yy_scan_string(arg1: *const ::std::os::raw::c_char) -> YY_BUFFER_STATE;
}
extern "C" {
    pub fn yy_delete_buffer(arg1: YY_BUFFER_STATE);
}
extern "C" {
    pub fn open_cat_ck(filename: c_str) -> *mut FILE;
}
extern "C" {
    pub fn chuck_parse(fname: c_constr, fd: *mut FILE, code: c_constr) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn reset_parse();
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SyntaxType {
    COMMA = 0,
    SEMICOLON = 1,
    DBLCOLON = 2,
    PAREN = 3,
    DOT = 4,
    CHUCK_OP = 5,
    OPERATOR = 6,
    KEYWORD = 7,
    DEBUG_PRINT = 8,
    SPORK = 9,
    INTEGER = 10,
    FLOATING = 11,
    STRING = 12,
    COMMENT = 13,
    OTHER = 14,
    NUM_SYNTAX_TYPES = 15,
}
#[repr(C)]
pub struct SyntaxToken {
    pub token: string,
    pub type_: ::std::os::raw::c_ulong,
    pub begin: size_type,
    pub end: size_type,
}
extern "C" {
    #[link_name = "\u{1}SyntaxToken"]
    pub fn SyntaxToken_SyntaxToken(this: *mut SyntaxToken);
}
extern "C" {
    #[link_name = "\u{1}SyntaxToken"]
    pub fn SyntaxToken_SyntaxToken1(this: *mut SyntaxToken, rhs: *const SyntaxToken);
}
impl Default for SyntaxToken {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SyntaxToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "SyntaxToken {{ token: {:?}, type: {:?}, begin: {:?}, end: {:?} }}",
            self.token, self.type_, self.begin, self.end
        )
    }
}
impl SyntaxToken {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        SyntaxToken_SyntaxToken(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(rhs: *const SyntaxToken) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        SyntaxToken_SyntaxToken1(&mut __bindgen_tmp, rhs);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct SyntaxTokenList {
    pub list: vector,
    pub howmany: SyntaxTokenList_size_type,
}
extern "C" {
    #[link_name = "\u{1}SyntaxTokenList"]
    pub fn SyntaxTokenList_SyntaxTokenList(this: *mut SyntaxTokenList, rhs: *const SyntaxTokenList);
}
extern "C" {
    #[link_name = "\u{1}SyntaxTokenList"]
    pub fn SyntaxTokenList_SyntaxTokenList1(this: *mut SyntaxTokenList);
}
impl Default for SyntaxTokenList {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SyntaxTokenList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "SyntaxTokenList {{ list: {:?}, howmany: {:?} }}",
            self.list, self.howmany
        )
    }
}
impl SyntaxTokenList {
    #[inline]
    pub unsafe fn new(rhs: *const SyntaxTokenList) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        SyntaxTokenList_SyntaxTokenList(&mut __bindgen_tmp, rhs);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        SyntaxTokenList_SyntaxTokenList1(&mut __bindgen_tmp);
        __bindgen_tmp
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
        this: *mut SyntaxQuery,
        line: *const string,
        tokens: *mut SyntaxTokenList,
    ) -> ::std::os::raw::c_ulong;
}
impl SyntaxQuery {
    #[inline]
    pub unsafe fn parseLine(
        &mut self,
        line: *const string,
        tokens: *mut SyntaxTokenList,
    ) -> ::std::os::raw::c_ulong {
        SyntaxQuery_parseLine(self, line, tokens)
    }
}
extern "C" {
    pub fn type_engine_scan0_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan0_class_def(
        env: *mut Chuck_Env,
        def: a_Class_Def,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_class_def(
        env: *mut Chuck_Env,
        def: a_Class_Def,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_func_def(
        env: *mut Chuck_Env,
        def: a_Func_Def,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_exp_decl(
        env: *mut Chuck_Env,
        decl: a_Exp_Decl,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_class_def(
        env: *mut Chuck_Env,
        def: a_Class_Def,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_func_def(
        env: *mut Chuck_Env,
        def: a_Func_Def,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_exp_decl(
        env: *mut Chuck_Env,
        decl: a_Exp_Decl,
    ) -> ::std::os::raw::c_ulong;
}
#[repr(C)]
pub struct Chuck_Local {
    pub name: string,
    pub size: ::std::os::raw::c_ulong,
    pub is_ref: ::std::os::raw::c_ulong,
    pub is_obj: ::std::os::raw::c_ulong,
    pub is_global: ::std::os::raw::c_ulong,
    pub offset: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Local"]
    pub fn Chuck_Local_Chuck_Local(this: *mut Chuck_Local);
}
impl Default for Chuck_Local {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Local {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Local {{ name: {:?}, size: {:?}, is_ref: {:?}, is_obj: {:?}, is_global: {:?}, offset: {:?} }}" , self . name , self . size , self . is_ref , self . is_obj , self . is_global , self . offset )
    }
}
impl Chuck_Local {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Local_Chuck_Local(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_Frame {
    pub name: string,
    pub curr_offset: ::std::os::raw::c_ulong,
    pub num_access: ::std::os::raw::c_ulong,
    pub stack: vector,
}
extern "C" {
    #[link_name = "\u{1}push_scope"]
    pub fn Chuck_Frame_push_scope(this: *mut Chuck_Frame);
}
extern "C" {
    #[link_name = "\u{1}alloc_local"]
    pub fn Chuck_Frame_alloc_local(
        this: *mut Chuck_Frame,
        size: ::std::os::raw::c_ulong,
        name: *const string,
        is_ref: ::std::os::raw::c_ulong,
        is_obj: ::std::os::raw::c_ulong,
        is_global: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_Local;
}
extern "C" {
    #[link_name = "\u{1}get_scope"]
    pub fn Chuck_Frame_get_scope(this: *const Chuck_Frame, out: *mut vector);
}
extern "C" {
    #[link_name = "\u{1}pop_scope"]
    pub fn Chuck_Frame_pop_scope(this: *mut Chuck_Frame, out: *mut vector);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Frame"]
    pub fn Chuck_Frame_Chuck_Frame(this: *mut Chuck_Frame);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Frame_destructor"]
    pub fn Chuck_Frame_Chuck_Frame_destructor(this: *mut Chuck_Frame);
}
impl Default for Chuck_Frame {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        size: ::std::os::raw::c_ulong,
        name: *const string,
        is_ref: ::std::os::raw::c_ulong,
        is_obj: ::std::os::raw::c_ulong,
        is_global: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_Local {
        Chuck_Frame_alloc_local(self, size, name, is_ref, is_obj, is_global)
    }
    #[inline]
    pub unsafe fn get_scope(&self, out: *mut vector) {
        Chuck_Frame_get_scope(self, out)
    }
    #[inline]
    pub unsafe fn pop_scope(&mut self, out: *mut vector) {
        Chuck_Frame_pop_scope(self, out)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Frame_Chuck_Frame(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
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
pub struct Chuck_Instr_Goto {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_Code {
    pub name: string,
    pub stack_depth: ::std::os::raw::c_ulong,
    pub need_this: ::std::os::raw::c_ulong,
    pub frame: *mut Chuck_Frame,
    pub code: vector,
    pub stack_cont: vector,
    pub stack_break: vector,
    pub stack_return: vector,
    pub filename: string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Code"]
    pub fn Chuck_Code_Chuck_Code(this: *mut Chuck_Code);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Code_destructor"]
    pub fn Chuck_Code_Chuck_Code_destructor(this: *mut Chuck_Code);
}
impl Default for Chuck_Code {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Code {{ name: {:?}, stack_depth: {:?}, need_this: {:?}, frame: {:?}, code: {:?}, stack_cont: {:?}, stack_break: {:?}, stack_return: {:?}, filename: {:?} }}" , self . name , self . stack_depth , self . need_this , self . frame , self . code , self . stack_cont , self . stack_break , self . stack_return , self . filename )
    }
}
impl Chuck_Code {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Code_Chuck_Code(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
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
    pub stack: vector,
    pub locals: vector,
    pub dump: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}append"]
    pub fn Chuck_Emitter_append(this: *mut Chuck_Emitter, instr: *mut Chuck_Instr);
}
extern "C" {
    #[link_name = "\u{1}next_index"]
    pub fn Chuck_Emitter_next_index(this: *mut Chuck_Emitter) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}push_scope"]
    pub fn Chuck_Emitter_push_scope(this: *mut Chuck_Emitter);
}
extern "C" {
    #[link_name = "\u{1}alloc_local"]
    pub fn Chuck_Emitter_alloc_local(
        this: *mut Chuck_Emitter,
        size: ::std::os::raw::c_ulong,
        name: *const string,
        is_ref: ::std::os::raw::c_ulong,
        is_obj: ::std::os::raw::c_ulong,
        is_global: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_Local;
}
extern "C" {
    #[link_name = "\u{1}addref_on_scope"]
    pub fn Chuck_Emitter_addref_on_scope(this: *mut Chuck_Emitter);
}
extern "C" {
    #[link_name = "\u{1}pop_scope"]
    pub fn Chuck_Emitter_pop_scope(this: *mut Chuck_Emitter);
}
extern "C" {
    #[link_name = "\u{1}find_dur"]
    pub fn Chuck_Emitter_find_dur(
        this: *mut Chuck_Emitter,
        name: *const string,
        out: *mut f64,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Emitter"]
    pub fn Chuck_Emitter_Chuck_Emitter(this: *mut Chuck_Emitter);
}
impl Default for Chuck_Emitter {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Emitter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Emitter {{ env: {:?}, code: {:?}, context: {:?}, nspc: {:?}, func: {:?}, stack: {:?}, locals: {:?}, dump: {:?} }}" , self . env , self . code , self . context , self . nspc , self . func , self . stack , self . locals , self . dump )
    }
}
impl ::std::cmp::PartialEq for Chuck_Emitter {
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
    pub unsafe fn append(&mut self, instr: *mut Chuck_Instr) {
        Chuck_Emitter_append(self, instr)
    }
    #[inline]
    pub unsafe fn next_index(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_Emitter_next_index(self)
    }
    #[inline]
    pub unsafe fn push_scope(&mut self) {
        Chuck_Emitter_push_scope(self)
    }
    #[inline]
    pub unsafe fn alloc_local(
        &mut self,
        size: ::std::os::raw::c_ulong,
        name: *const string,
        is_ref: ::std::os::raw::c_ulong,
        is_obj: ::std::os::raw::c_ulong,
        is_global: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_Local {
        Chuck_Emitter_alloc_local(self, size, name, is_ref, is_obj, is_global)
    }
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
        name: *const string,
        out: *mut f64,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Emitter_find_dur(self, name, out)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Emitter_Chuck_Emitter(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Emitter_destructor"]
    pub fn Chuck_Emitter_Chuck_Emitter_destructor(this: *mut Chuck_Emitter);
}
extern "C" {
    pub fn emit_engine_init(env: *mut Chuck_Env) -> *mut Chuck_Emitter;
}
extern "C" {
    pub fn emit_engine_shutdown(emit: *mut *mut Chuck_Emitter) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn emit_engine_emit_prog(
        emit: *mut Chuck_Emitter,
        prog: a_Program,
        how_much: te_HowMuch,
    ) -> *mut Chuck_VM_Code;
}
extern "C" {
    pub fn emit_to_code(
        in_: *mut Chuck_Code,
        out: *mut Chuck_VM_Code,
        dump: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_VM_Code;
}
extern "C" {
    pub fn emit_engine_addr_map(
        emit: *mut Chuck_Emitter,
        shred: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn emit_engine_resolve() -> ::std::os::raw::c_ulong;
}
#[repr(C)]
pub struct CBufferAdvance {
    pub m_data: *mut ::std::os::raw::c_uchar,
    pub m_data_width: ::std::os::raw::c_ulong,
    pub m_read_offsets: vector,
    pub m_free: queue<deque>,
    pub m_write_offset: ::std::os::raw::c_long,
    pub m_max_elem: ::std::os::raw::c_long,
    pub m_mutex: XMutex,
    pub m_event_buffer: *mut CBufferSimple,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CBufferAdvance_ReadOffset {
    pub read_offset: ::std::os::raw::c_long,
    pub event: *mut Chuck_Event,
}
extern "C" {
    #[link_name = "\u{1}ReadOffset"]
    pub fn CBufferAdvance_ReadOffset_ReadOffset(
        this: *mut CBufferAdvance_ReadOffset,
        ro: ::std::os::raw::c_long,
        e: *mut Chuck_Event,
    );
}
impl Default for CBufferAdvance_ReadOffset {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl CBufferAdvance_ReadOffset {
    #[inline]
    pub unsafe fn new(ro: ::std::os::raw::c_long, e: *mut Chuck_Event) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        CBufferAdvance_ReadOffset_ReadOffset(&mut __bindgen_tmp, ro, e);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn CBufferAdvance_initialize(
        this: *mut CBufferAdvance,
        num_elem: ::std::os::raw::c_ulong,
        width: ::std::os::raw::c_ulong,
        event_buffer: *mut CBufferSimple,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn CBufferAdvance_cleanup(this: *mut CBufferAdvance);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn CBufferAdvance_get(
        this: *mut CBufferAdvance,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
        read_offset_index: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn CBufferAdvance_put(
        this: *mut CBufferAdvance,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}empty"]
    pub fn CBufferAdvance_empty(
        this: *mut CBufferAdvance,
        read_offset_index: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}join"]
    pub fn CBufferAdvance_join(
        this: *mut CBufferAdvance,
        event: *mut Chuck_Event,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}resign"]
    pub fn CBufferAdvance_resign(
        this: *mut CBufferAdvance,
        read_offset_index: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}CBufferAdvance"]
    pub fn CBufferAdvance_CBufferAdvance(this: *mut CBufferAdvance);
}
extern "C" {
    #[link_name = "\u{1}CBufferAdvance_destructor"]
    pub fn CBufferAdvance_CBufferAdvance_destructor(this: *mut CBufferAdvance);
}
impl Default for CBufferAdvance {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CBufferAdvance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "CBufferAdvance {{ m_data: {:?}, m_data_width: {:?}, m_read_offsets: {:?}, m_free: {:?}, m_write_offset: {:?}, m_max_elem: {:?}, m_mutex: {:?}, m_event_buffer: {:?} }}" , self . m_data , self . m_data_width , self . m_read_offsets , self . m_free , self . m_write_offset , self . m_max_elem , self . m_mutex , self . m_event_buffer )
    }
}
impl CBufferAdvance {
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        num_elem: ::std::os::raw::c_ulong,
        width: ::std::os::raw::c_ulong,
        event_buffer: *mut CBufferSimple,
    ) -> ::std::os::raw::c_ulong {
        CBufferAdvance_initialize(self, num_elem, width, event_buffer)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        CBufferAdvance_cleanup(self)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
        read_offset_index: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        CBufferAdvance_get(self, data, num_elem, read_offset_index)
    }
    #[inline]
    pub unsafe fn put(
        &mut self,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    ) {
        CBufferAdvance_put(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn empty(
        &mut self,
        read_offset_index: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        CBufferAdvance_empty(self, read_offset_index)
    }
    #[inline]
    pub unsafe fn join(&mut self, event: *mut Chuck_Event) -> ::std::os::raw::c_ulong {
        CBufferAdvance_join(self, event)
    }
    #[inline]
    pub unsafe fn resign(&mut self, read_offset_index: ::std::os::raw::c_ulong) {
        CBufferAdvance_resign(self, read_offset_index)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        CBufferAdvance_CBufferAdvance(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        CBufferAdvance_CBufferAdvance_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CBufferSimple {
    pub m_data: *mut ::std::os::raw::c_uchar,
    pub m_data_width: ::std::os::raw::c_ulong,
    pub m_read_offset: ::std::os::raw::c_ulong,
    pub m_write_offset: ::std::os::raw::c_ulong,
    pub m_max_elem: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn CBufferSimple_initialize(
        this: *mut CBufferSimple,
        num_elem: ::std::os::raw::c_ulong,
        width: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn CBufferSimple_cleanup(this: *mut CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn CBufferSimple_get(
        this: *mut CBufferSimple,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn CBufferSimple_put(
        this: *mut CBufferSimple,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}CBufferSimple"]
    pub fn CBufferSimple_CBufferSimple(this: *mut CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}CBufferSimple_destructor"]
    pub fn CBufferSimple_CBufferSimple_destructor(this: *mut CBufferSimple);
}
impl Default for CBufferSimple {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl CBufferSimple {
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        num_elem: ::std::os::raw::c_ulong,
        width: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        CBufferSimple_initialize(self, num_elem, width)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        CBufferSimple_cleanup(self)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        CBufferSimple_get(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn put(
        &mut self,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    ) {
        CBufferSimple_put(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
pub struct AccumBuffer {
    pub m_data: *mut f64,
    pub m_write_offset: ::std::os::raw::c_ulong,
    pub m_max_elem: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}resize"]
    pub fn AccumBuffer_resize(
        this: *mut AccumBuffer,
        new_size: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn AccumBuffer_cleanup(this: *mut AccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn AccumBuffer_put(this: *mut AccumBuffer, next: f64);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn AccumBuffer_get(
        this: *mut AccumBuffer,
        buffer: *mut f64,
        num_elem: ::std::os::raw::c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}AccumBuffer"]
    pub fn AccumBuffer_AccumBuffer(this: *mut AccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}AccumBuffer_destructor"]
    pub fn AccumBuffer_AccumBuffer_destructor(this: *mut AccumBuffer);
}
impl Default for AccumBuffer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl AccumBuffer {
    #[inline]
    pub unsafe fn resize(&mut self, new_size: ::std::os::raw::c_long) -> ::std::os::raw::c_long {
        AccumBuffer_resize(self, new_size)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        AccumBuffer_cleanup(self)
    }
    #[inline]
    pub unsafe fn put(&mut self, next: f64) {
        AccumBuffer_put(self, next)
    }
    #[inline]
    pub unsafe fn get(&mut self, buffer: *mut f64, num_elem: ::std::os::raw::c_long) {
        AccumBuffer_get(self, buffer, num_elem)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        AccumBuffer_AccumBuffer(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        AccumBuffer_AccumBuffer_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct DeccumBuffer {
    pub m_data: *mut f64,
    pub m_read_offset: ::std::os::raw::c_ulong,
    pub m_max_elem: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}resize"]
    pub fn DeccumBuffer_resize(
        this: *mut DeccumBuffer,
        new_size: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn DeccumBuffer_cleanup(this: *mut DeccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn DeccumBuffer_put(
        this: *mut DeccumBuffer,
        next: *mut f64,
        num_elem: ::std::os::raw::c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn DeccumBuffer_get(this: *mut DeccumBuffer, out: *mut f64);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn DeccumBuffer_get1(
        this: *mut DeccumBuffer,
        buffer: *mut f64,
        num_elem: ::std::os::raw::c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}DeccumBuffer"]
    pub fn DeccumBuffer_DeccumBuffer(this: *mut DeccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}DeccumBuffer_destructor"]
    pub fn DeccumBuffer_DeccumBuffer_destructor(this: *mut DeccumBuffer);
}
impl Default for DeccumBuffer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DeccumBuffer {
    #[inline]
    pub unsafe fn resize(&mut self, new_size: ::std::os::raw::c_long) -> ::std::os::raw::c_long {
        DeccumBuffer_resize(self, new_size)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        DeccumBuffer_cleanup(self)
    }
    #[inline]
    pub unsafe fn put(&mut self, next: *mut f64, num_elem: ::std::os::raw::c_long) {
        DeccumBuffer_put(self, next, num_elem)
    }
    #[inline]
    pub unsafe fn get(&mut self, out: *mut f64) {
        DeccumBuffer_get(self, out)
    }
    #[inline]
    pub unsafe fn get1(&mut self, buffer: *mut f64, num_elem: ::std::os::raw::c_long) {
        DeccumBuffer_get1(self, buffer, num_elem)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        DeccumBuffer_DeccumBuffer(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        DeccumBuffer_DeccumBuffer_destructor(self)
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
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct FastCircularBuffer {
    pub m_data: *mut ::std::os::raw::c_uchar,
    pub m_data_width: ::std::os::raw::c_ulong,
    pub m_read_offset: ::std::os::raw::c_ulong,
    pub m_write_offset: ::std::os::raw::c_ulong,
    pub m_max_elem: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn FastCircularBuffer_initialize(
        this: *mut FastCircularBuffer,
        num_elem: ::std::os::raw::c_ulong,
        width: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn FastCircularBuffer_cleanup(this: *mut FastCircularBuffer);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn FastCircularBuffer_get(
        this: *mut FastCircularBuffer,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn FastCircularBuffer_put(
        this: *mut FastCircularBuffer,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}hasMore"]
    pub fn FastCircularBuffer_hasMore(this: *mut FastCircularBuffer) -> bool;
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn FastCircularBuffer_clear(this: *mut FastCircularBuffer);
}
extern "C" {
    #[link_name = "\u{1}FastCircularBuffer"]
    pub fn FastCircularBuffer_FastCircularBuffer(this: *mut FastCircularBuffer);
}
extern "C" {
    #[link_name = "\u{1}FastCircularBuffer_destructor"]
    pub fn FastCircularBuffer_FastCircularBuffer_destructor(this: *mut FastCircularBuffer);
}
impl Default for FastCircularBuffer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl FastCircularBuffer {
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        num_elem: ::std::os::raw::c_ulong,
        width: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        FastCircularBuffer_initialize(self, num_elem, width)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        FastCircularBuffer_cleanup(self)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        FastCircularBuffer_get(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn put(
        &mut self,
        data: *mut ::std::os::raw::c_void,
        num_elem: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        FastCircularBuffer_put(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn hasMore(&mut self) -> bool {
        FastCircularBuffer_hasMore(self)
    }
    #[inline]
    pub unsafe fn clear(&mut self) {
        FastCircularBuffer_clear(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub m_length: ::std::os::raw::c_long,
    pub m_writeIndex: ::std::os::raw::c_long,
    pub m_readIndex: ::std::os::raw::c_long,
    pub m_numElements: ::std::os::raw::c_long,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for XCircleBuffer<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    pub cycles: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Shred_Activation"]
    pub fn Shred_Activation_Shred_Activation(
        this: *mut Shred_Activation,
        a: f64,
        b: ::std::os::raw::c_ulong,
    );
}
impl Shred_Activation {
    #[inline]
    pub unsafe fn new(a: f64, b: ::std::os::raw::c_ulong) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Shred_Activation_Shred_Activation(&mut __bindgen_tmp, a, b);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Shred_Stat {
    pub cycles: ::std::os::raw::c_ulong,
    pub xid: ::std::os::raw::c_ulong,
    pub parent: ::std::os::raw::c_ulong,
    pub state: ::std::os::raw::c_ulong,
    pub shred_ref: *mut Chuck_VM_Shred,
    pub activations: ::std::os::raw::c_ulong,
    pub average_ctrl: f64,
    pub average_cycles: f64,
    pub spork_time: f64,
    pub active_time: f64,
    pub wake_time: f64,
    pub free_time: f64,
    pub name: string,
    pub owner: string,
    pub source: string,
    pub diffs: queue<deque>,
    pub num_diffs: ::std::os::raw::c_ulong,
    pub diff_total: f64,
    pub act_cycles: queue<deque>,
    pub act_cycles_total: ::std::os::raw::c_ulong,
    pub last_cycles: ::std::os::raw::c_ulong,
    pub children: vector,
    pub activationss: vector,
    pub mutex: XMutex,
    pub data: *mut Shred_Data,
    pub time: *mut Shred_Time,
}
extern "C" {
    #[link_name = "\u{1}get_sporked"]
    pub fn Shred_Stat_get_sporked(this: *mut Shred_Stat, out: *mut vector);
}
extern "C" {
    #[link_name = "\u{1}get_activations"]
    pub fn Shred_Stat_get_activations(this: *mut Shred_Stat, out: *mut vector);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Shred_Stat_clear(this: *mut Shred_Stat);
}
extern "C" {
    #[link_name = "\u{1}Shred_Stat"]
    pub fn Shred_Stat_Shred_Stat(this: *mut Shred_Stat);
}
impl Default for Shred_Stat {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Shred_Stat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Shred_Stat {{ cycles: {:?}, xid: {:?}, parent: {:?}, state: {:?}, shred_ref: {:?}, activations: {:?}, average_ctrl: {:?}, average_cycles: {:?}, spork_time: {:?}, active_time: {:?}, wake_time: {:?}, free_time: {:?}, name: {:?}, owner: {:?}, source: {:?}, diffs: {:?}, num_diffs: {:?}, diff_total: {:?}, act_cycles: {:?}, act_cycles_total: {:?}, last_cycles: {:?}, children: {:?}, activationss: {:?}, mutex: {:?}, data: {:?}, time: {:?} }}" , self . cycles , self . xid , self . parent , self . state , self . shred_ref , self . activations , self . average_ctrl , self . average_cycles , self . spork_time , self . active_time , self . wake_time , self . free_time , self . name , self . owner , self . source , self . diffs , self . num_diffs , self . diff_total , self . act_cycles , self . act_cycles_total , self . last_cycles , self . children , self . activationss , self . mutex , self . data , self . time )
    }
}
impl Shred_Stat {
    #[inline]
    pub unsafe fn get_sporked(&mut self, out: *mut vector) {
        Shred_Stat_get_sporked(self, out)
    }
    #[inline]
    pub unsafe fn get_activations(&mut self, out: *mut vector) {
        Shred_Stat_get_activations(self, out)
    }
    #[inline]
    pub unsafe fn clear(&mut self) {
        Shred_Stat_clear(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Shred_Stat_Shred_Stat(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_Stats {
    pub vm: *mut Chuck_VM,
    pub shreds: map,
    pub done: vector,
    pub mutex: XMutex,
}
extern "C" {
    #[link_name = "\u{1}activations_yes"]
    pub static mut Chuck_Stats_activations_yes: ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}our_instance"]
    pub static mut Chuck_Stats_our_instance: *mut Chuck_Stats;
}
extern "C" {
    #[link_name = "\u{1}instance"]
    pub fn Chuck_Stats_instance() -> *mut Chuck_Stats;
}
extern "C" {
    #[link_name = "\u{1}set_vm_ref"]
    pub fn Chuck_Stats_set_vm_ref(this: *mut Chuck_Stats, _vm: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}add_shred"]
    pub fn Chuck_Stats_add_shred(this: *mut Chuck_Stats, shred: *mut Chuck_VM_Shred);
}
extern "C" {
    #[link_name = "\u{1}activate_shred"]
    pub fn Chuck_Stats_activate_shred(this: *mut Chuck_Stats, shred: *mut Chuck_VM_Shred);
}
extern "C" {
    #[link_name = "\u{1}advance_time"]
    pub fn Chuck_Stats_advance_time(this: *mut Chuck_Stats, shred: *mut Chuck_VM_Shred, to: f64);
}
extern "C" {
    #[link_name = "\u{1}deactivate_shred"]
    pub fn Chuck_Stats_deactivate_shred(this: *mut Chuck_Stats, shred: *mut Chuck_VM_Shred);
}
extern "C" {
    #[link_name = "\u{1}remove_shred"]
    pub fn Chuck_Stats_remove_shred(this: *mut Chuck_Stats, shred: *mut Chuck_VM_Shred);
}
extern "C" {
    #[link_name = "\u{1}get_shred"]
    pub fn Chuck_Stats_get_shred(
        this: *mut Chuck_Stats,
        xid: ::std::os::raw::c_ulong,
    ) -> *mut Shred_Stat;
}
extern "C" {
    #[link_name = "\u{1}get_shreds"]
    pub fn Chuck_Stats_get_shreds(this: *mut Chuck_Stats, out: *mut vector, d: *mut map);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Stats"]
    pub fn Chuck_Stats_Chuck_Stats(this: *mut Chuck_Stats);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Stats_destructor"]
    pub fn Chuck_Stats_Chuck_Stats_destructor(this: *mut Chuck_Stats);
}
impl Default for Chuck_Stats {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Stats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Chuck_Stats {{ vm: {:?}, shreds: {:?}, done: {:?}, mutex: {:?} }}",
            self.vm, self.shreds, self.done, self.mutex
        )
    }
}
impl Chuck_Stats {
    #[inline]
    pub unsafe fn instance() -> *mut Chuck_Stats {
        Chuck_Stats_instance()
    }
    #[inline]
    pub unsafe fn set_vm_ref(&mut self, _vm: *mut Chuck_VM) {
        Chuck_Stats_set_vm_ref(self, _vm)
    }
    #[inline]
    pub unsafe fn add_shred(&mut self, shred: *mut Chuck_VM_Shred) {
        Chuck_Stats_add_shred(self, shred)
    }
    #[inline]
    pub unsafe fn activate_shred(&mut self, shred: *mut Chuck_VM_Shred) {
        Chuck_Stats_activate_shred(self, shred)
    }
    #[inline]
    pub unsafe fn advance_time(&mut self, shred: *mut Chuck_VM_Shred, to: f64) {
        Chuck_Stats_advance_time(self, shred, to)
    }
    #[inline]
    pub unsafe fn deactivate_shred(&mut self, shred: *mut Chuck_VM_Shred) {
        Chuck_Stats_deactivate_shred(self, shred)
    }
    #[inline]
    pub unsafe fn remove_shred(&mut self, shred: *mut Chuck_VM_Shred) {
        Chuck_Stats_remove_shred(self, shred)
    }
    #[inline]
    pub unsafe fn get_shred(&mut self, xid: ::std::os::raw::c_ulong) -> *mut Shred_Stat {
        Chuck_Stats_get_shred(self, xid)
    }
    #[inline]
    pub unsafe fn get_shreds(&mut self, out: *mut vector, d: *mut map) {
        Chuck_Stats_get_shreds(self, out, d)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Stats_Chuck_Stats(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_Stats_Chuck_Stats_destructor(self)
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
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Chuck_VM_Stack {
    pub stack: *mut ::std::os::raw::c_uchar,
    pub sp: *mut ::std::os::raw::c_uchar,
    pub sp_max: *mut ::std::os::raw::c_uchar,
    pub prev: *mut Chuck_VM_Stack,
    pub next: *mut Chuck_VM_Stack,
    pub m_is_init: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_Stack_initialize(
        this: *mut Chuck_VM_Stack,
        size: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Stack_shutdown(this: *mut Chuck_VM_Stack) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Stack"]
    pub fn Chuck_VM_Stack_Chuck_VM_Stack(this: *mut Chuck_VM_Stack);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Stack_destructor"]
    pub fn Chuck_VM_Stack_Chuck_VM_Stack_destructor(this: *mut Chuck_VM_Stack);
}
impl Default for Chuck_VM_Stack {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Chuck_VM_Stack {
    #[inline]
    pub unsafe fn initialize(&mut self, size: ::std::os::raw::c_ulong) -> ::std::os::raw::c_ulong {
        Chuck_VM_Stack_initialize(self, size)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_Stack_shutdown(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
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
    pub _base: Chuck_Object,
    pub instr: *mut *mut Chuck_Instr,
    pub num_instr: ::std::os::raw::c_ulong,
    pub name: string,
    pub stack_depth: ::std::os::raw::c_ulong,
    pub need_this: ::std::os::raw::c_ulong,
    pub native_func: ::std::os::raw::c_ulong,
    pub native_func_type: ::std::os::raw::c_ulong,
    pub filename: string,
}
pub const Chuck_VM_Code_NATIVE_UNKNOWN: Chuck_VM_Code__bindgen_ty_1 =
    Chuck_VM_Code__bindgen_ty_1::NATIVE_UNKNOWN;
pub const Chuck_VM_Code_NATIVE_CTOR: Chuck_VM_Code__bindgen_ty_1 =
    Chuck_VM_Code__bindgen_ty_1::NATIVE_CTOR;
pub const Chuck_VM_Code_NATIVE_DTOR: Chuck_VM_Code__bindgen_ty_1 =
    Chuck_VM_Code__bindgen_ty_1::NATIVE_DTOR;
pub const Chuck_VM_Code_NATIVE_MFUN: Chuck_VM_Code__bindgen_ty_1 =
    Chuck_VM_Code__bindgen_ty_1::NATIVE_MFUN;
pub const Chuck_VM_Code_NATIVE_SFUN: Chuck_VM_Code__bindgen_ty_1 =
    Chuck_VM_Code__bindgen_ty_1::NATIVE_SFUN;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Chuck_VM_Code__bindgen_ty_1 {
    NATIVE_UNKNOWN = 0,
    NATIVE_CTOR = 1,
    NATIVE_DTOR = 2,
    NATIVE_MFUN = 3,
    NATIVE_SFUN = 4,
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Code"]
    pub fn Chuck_VM_Code_Chuck_VM_Code(this: *mut Chuck_VM_Code);
}
impl Default for Chuck_VM_Code {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_VM_Code_Chuck_VM_Code(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Code_destructor"]
    pub fn Chuck_VM_Code_Chuck_VM_Code_destructor(this: *mut Chuck_VM_Code);
}
#[repr(C)]
pub struct Chuck_VM_Shred {
    pub _base: Chuck_Object,
    pub mem: *mut Chuck_VM_Stack,
    pub reg: *mut Chuck_VM_Stack,
    pub base_ref: *mut Chuck_VM_Stack,
    pub code: *mut Chuck_VM_Code,
    pub code_orig: *mut Chuck_VM_Code,
    pub instr: *mut *mut Chuck_Instr,
    pub parent: *mut Chuck_VM_Shred,
    pub children: map,
    pub pc: ::std::os::raw::c_ulong,
    pub vm_ref: *mut Chuck_VM,
    pub now: f64,
    pub start: f64,
    pub wake_time: f64,
    pub next_pc: ::std::os::raw::c_ulong,
    pub is_done: ::std::os::raw::c_ulong,
    pub is_running: ::std::os::raw::c_ulong,
    pub is_abort: ::std::os::raw::c_ulong,
    pub is_dumped: ::std::os::raw::c_ulong,
    pub event: *mut Chuck_Event,
    pub m_ugen_map: map,
    pub m_parent_objects: vector,
    pub xid: ::std::os::raw::c_ulong,
    pub name: string,
    pub args: vector,
    pub prev: *mut Chuck_VM_Shred,
    pub next: *mut Chuck_VM_Shred,
    pub stat: *mut Shred_Stat,
    pub m_loopCounters: vector,
    pub m_serials: *mut list,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_Shred_initialize(
        this: *mut Chuck_VM_Shred,
        c: *mut Chuck_VM_Code,
        mem_st_size: ::std::os::raw::c_ulong,
        reg_st_size: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Shred_shutdown(this: *mut Chuck_VM_Shred) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn Chuck_VM_Shred_run(
        this: *mut Chuck_VM_Shred,
        vm: *mut Chuck_VM,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add"]
    pub fn Chuck_VM_Shred_add(
        this: *mut Chuck_VM_Shred,
        ugen: *mut Chuck_UGen,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_VM_Shred_remove(
        this: *mut Chuck_VM_Shred,
        ugen: *mut Chuck_UGen,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add_parent_ref"]
    pub fn Chuck_VM_Shred_add_parent_ref(this: *mut Chuck_VM_Shred, obj: *mut Chuck_Object);
}
extern "C" {
    #[link_name = "\u{1}add_serialio"]
    pub fn Chuck_VM_Shred_add_serialio(this: *mut Chuck_VM_Shred, serial: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}remove_serialio"]
    pub fn Chuck_VM_Shred_remove_serialio(this: *mut Chuck_VM_Shred, serial: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}pushLoopCounter"]
    pub fn Chuck_VM_Shred_pushLoopCounter(
        this: *mut Chuck_VM_Shred,
    ) -> *mut ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}currentLoopCounter"]
    pub fn Chuck_VM_Shred_currentLoopCounter(
        this: *mut Chuck_VM_Shred,
    ) -> *mut ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}popLoopCounter"]
    pub fn Chuck_VM_Shred_popLoopCounter(this: *mut Chuck_VM_Shred) -> bool;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred"]
    pub fn Chuck_VM_Shred_Chuck_VM_Shred(this: *mut Chuck_VM_Shred);
}
impl Default for Chuck_VM_Shred {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        c: *mut Chuck_VM_Code,
        mem_st_size: ::std::os::raw::c_ulong,
        reg_st_size: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shred_initialize(self, c, mem_st_size, reg_st_size)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shred_shutdown(self)
    }
    #[inline]
    pub unsafe fn run(&mut self, vm: *mut Chuck_VM) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shred_run(self, vm)
    }
    #[inline]
    pub unsafe fn add(&mut self, ugen: *mut Chuck_UGen) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shred_add(self, ugen)
    }
    #[inline]
    pub unsafe fn remove(&mut self, ugen: *mut Chuck_UGen) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shred_remove(self, ugen)
    }
    #[inline]
    pub unsafe fn add_parent_ref(&mut self, obj: *mut Chuck_Object) {
        Chuck_VM_Shred_add_parent_ref(self, obj)
    }
    #[inline]
    pub unsafe fn add_serialio(&mut self, serial: *mut Chuck_IO_Serial) {
        Chuck_VM_Shred_add_serialio(self, serial)
    }
    #[inline]
    pub unsafe fn remove_serialio(&mut self, serial: *mut Chuck_IO_Serial) {
        Chuck_VM_Shred_remove_serialio(self, serial)
    }
    #[inline]
    pub unsafe fn pushLoopCounter(&mut self) -> *mut ::std::os::raw::c_ulong {
        Chuck_VM_Shred_pushLoopCounter(self)
    }
    #[inline]
    pub unsafe fn currentLoopCounter(&mut self) -> *mut ::std::os::raw::c_ulong {
        Chuck_VM_Shred_currentLoopCounter(self)
    }
    #[inline]
    pub unsafe fn popLoopCounter(&mut self) -> bool {
        Chuck_VM_Shred_popLoopCounter(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_VM_Shred_Chuck_VM_Shred(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred_destructor"]
    pub fn Chuck_VM_Shred_Chuck_VM_Shred_destructor(this: *mut Chuck_VM_Shred);
}
#[repr(C)]
pub struct Chuck_VM_Shred_Status {
    pub _base: Chuck_Object,
    pub xid: ::std::os::raw::c_ulong,
    pub name: string,
    pub start: f64,
    pub has_event: ::std::os::raw::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred_Status"]
    pub fn Chuck_VM_Shred_Status_Chuck_VM_Shred_Status(
        this: *mut Chuck_VM_Shred_Status,
        _id: ::std::os::raw::c_ulong,
        n: *const string,
        _start: f64,
        e: ::std::os::raw::c_ulong,
    );
}
impl Default for Chuck_VM_Shred_Status {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
impl Chuck_VM_Shred_Status {
    #[inline]
    pub unsafe fn new(
        _id: ::std::os::raw::c_ulong,
        n: *const string,
        _start: f64,
        e: ::std::os::raw::c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_VM_Shred_Status_Chuck_VM_Shred_Status(&mut __bindgen_tmp, _id, n, _start, e);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_VM_Status {
    pub _base: Chuck_Object,
    pub srate: ::std::os::raw::c_ulong,
    pub now_system: f64,
    pub t_second: ::std::os::raw::c_ulong,
    pub t_minute: ::std::os::raw::c_ulong,
    pub t_hour: ::std::os::raw::c_ulong,
    pub list: vector,
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_VM_Status_clear(this: *mut Chuck_VM_Status);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Status"]
    pub fn Chuck_VM_Status_Chuck_VM_Status(this: *mut Chuck_VM_Status);
}
impl Default for Chuck_VM_Status {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VM_Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_VM_Status {{ srate: {:?}, now_system: {:?}, t_second: {:?}, t_minute: {:?}, t_hour: {:?}, list: {:?} }}" , self . srate , self . now_system , self . t_second , self . t_minute , self . t_hour , self . list )
    }
}
impl ::std::cmp::PartialEq for Chuck_VM_Status {
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
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_VM_Status_Chuck_VM_Status(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Status_destructor"]
    pub fn Chuck_VM_Status_Chuck_VM_Status_destructor(this: *mut Chuck_VM_Status);
}
#[repr(C)]
pub struct Chuck_VM_Shreduler {
    pub _base: Chuck_Object,
    pub now_system: f64,
    pub rt_audio: ::std::os::raw::c_ulong,
    pub vm_ref: *mut Chuck_VM,
    pub shred_list: *mut Chuck_VM_Shred,
    pub blocked: map,
    pub m_current_shred: *mut Chuck_VM_Shred,
    pub m_dac: *mut Chuck_UGen,
    pub m_adc: *mut Chuck_UGen,
    pub m_bunghole: *mut Chuck_UGen,
    pub m_num_dac_channels: ::std::os::raw::c_ulong,
    pub m_num_adc_channels: ::std::os::raw::c_ulong,
    pub m_status: Chuck_VM_Status,
    pub m_max_block_size: ::std::os::raw::c_ulong,
    pub m_adaptive: ::std::os::raw::c_ulong,
    pub m_samps_until_next: f64,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_Shreduler_initialize(this: *mut Chuck_VM_Shreduler) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Shreduler_shutdown(this: *mut Chuck_VM_Shreduler) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shredule"]
    pub fn Chuck_VM_Shreduler_shredule(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shredule"]
    pub fn Chuck_VM_Shreduler_shredule1(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
        wake_time: f64,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_VM_Shreduler_get(this: *mut Chuck_VM_Shreduler) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}advance"]
    pub fn Chuck_VM_Shreduler_advance(this: *mut Chuck_VM_Shreduler, N: ::std::os::raw::c_long);
}
extern "C" {
    #[link_name = "\u{1}advance_v"]
    pub fn Chuck_VM_Shreduler_advance_v(
        this: *mut Chuck_VM_Shreduler,
        num_left: *mut ::std::os::raw::c_long,
        offset: *mut ::std::os::raw::c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}set_adaptive"]
    pub fn Chuck_VM_Shreduler_set_adaptive(
        this: *mut Chuck_VM_Shreduler,
        max_block_size: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_VM_Shreduler_remove(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}replace"]
    pub fn Chuck_VM_Shreduler_replace(
        this: *mut Chuck_VM_Shreduler,
        out: *mut Chuck_VM_Shred,
        in_: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}lookup"]
    pub fn Chuck_VM_Shreduler_lookup(
        this: *mut Chuck_VM_Shreduler,
        xid: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}status"]
    pub fn Chuck_VM_Shreduler_status(this: *mut Chuck_VM_Shreduler);
}
extern "C" {
    #[link_name = "\u{1}status"]
    pub fn Chuck_VM_Shreduler_status1(this: *mut Chuck_VM_Shreduler, status: *mut Chuck_VM_Status);
}
extern "C" {
    #[link_name = "\u{1}highest"]
    pub fn Chuck_VM_Shreduler_highest(this: *mut Chuck_VM_Shreduler) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add_blocked"]
    pub fn Chuck_VM_Shreduler_add_blocked(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_blocked"]
    pub fn Chuck_VM_Shreduler_remove_blocked(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shreduler"]
    pub fn Chuck_VM_Shreduler_Chuck_VM_Shreduler(this: *mut Chuck_VM_Shreduler);
}
impl Default for Chuck_VM_Shreduler {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_VM_Shreduler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_VM_Shreduler {{ now_system: {:?}, rt_audio: {:?}, vm_ref: {:?}, shred_list: {:?}, blocked: {:?}, m_current_shred: {:?}, m_dac: {:?}, m_adc: {:?}, m_bunghole: {:?}, m_num_dac_channels: {:?}, m_num_adc_channels: {:?}, m_status: {:?}, m_max_block_size: {:?}, m_adaptive: {:?}, m_samps_until_next: {:?} }}" , self . now_system , self . rt_audio , self . vm_ref , self . shred_list , self . blocked , self . m_current_shred , self . m_dac , self . m_adc , self . m_bunghole , self . m_num_dac_channels , self . m_num_adc_channels , self . m_status , self . m_max_block_size , self . m_adaptive , self . m_samps_until_next )
    }
}
impl ::std::cmp::PartialEq for Chuck_VM_Shreduler {
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
    pub unsafe fn initialize(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_initialize(self)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_shutdown(self)
    }
    #[inline]
    pub unsafe fn shredule(&mut self, shred: *mut Chuck_VM_Shred) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_shredule(self, shred)
    }
    #[inline]
    pub unsafe fn shredule1(
        &mut self,
        shred: *mut Chuck_VM_Shred,
        wake_time: f64,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_shredule1(self, shred, wake_time)
    }
    #[inline]
    pub unsafe fn get(&mut self) -> *mut Chuck_VM_Shred {
        Chuck_VM_Shreduler_get(self)
    }
    #[inline]
    pub unsafe fn advance(&mut self, N: ::std::os::raw::c_long) {
        Chuck_VM_Shreduler_advance(self, N)
    }
    #[inline]
    pub unsafe fn advance_v(
        &mut self,
        num_left: *mut ::std::os::raw::c_long,
        offset: *mut ::std::os::raw::c_long,
    ) {
        Chuck_VM_Shreduler_advance_v(self, num_left, offset)
    }
    #[inline]
    pub unsafe fn set_adaptive(&mut self, max_block_size: ::std::os::raw::c_ulong) {
        Chuck_VM_Shreduler_set_adaptive(self, max_block_size)
    }
    #[inline]
    pub unsafe fn remove(&mut self, shred: *mut Chuck_VM_Shred) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_remove(self, shred)
    }
    #[inline]
    pub unsafe fn replace(
        &mut self,
        out: *mut Chuck_VM_Shred,
        in_: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_replace(self, out, in_)
    }
    #[inline]
    pub unsafe fn lookup(&mut self, xid: ::std::os::raw::c_ulong) -> *mut Chuck_VM_Shred {
        Chuck_VM_Shreduler_lookup(self, xid)
    }
    #[inline]
    pub unsafe fn status(&mut self) {
        Chuck_VM_Shreduler_status(self)
    }
    #[inline]
    pub unsafe fn status1(&mut self, status: *mut Chuck_VM_Status) {
        Chuck_VM_Shreduler_status1(self, status)
    }
    #[inline]
    pub unsafe fn highest(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_highest(self)
    }
    #[inline]
    pub unsafe fn add_blocked(&mut self, shred: *mut Chuck_VM_Shred) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_add_blocked(self, shred)
    }
    #[inline]
    pub unsafe fn remove_blocked(&mut self, shred: *mut Chuck_VM_Shred) -> ::std::os::raw::c_ulong {
        Chuck_VM_Shreduler_remove_blocked(self, shred)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_VM_Shreduler_Chuck_VM_Shreduler(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shreduler_destructor"]
    pub fn Chuck_VM_Shreduler_Chuck_VM_Shreduler_destructor(this: *mut Chuck_VM_Shreduler);
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
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Chuck_Global_Request_Type {
    set_global_int_request = 0,
    get_global_int_request = 1,
    set_global_float_request = 2,
    get_global_float_request = 3,
    signal_global_event_request = 4,
    spork_shred_request = 5,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Chuck_Global_Request {
    pub type_: Chuck_Global_Request_Type,
    pub __bindgen_anon_1: Chuck_Global_Request__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Chuck_Global_Request__bindgen_ty_1 {
    pub setIntRequest: *mut Chuck_Set_Global_Int_Request,
    pub getIntRequest: *mut Chuck_Get_Global_Int_Request,
    pub setFloatRequest: *mut Chuck_Set_Global_Float_Request,
    pub getFloatRequest: *mut Chuck_Get_Global_Float_Request,
    pub signalEventRequest: *mut Chuck_Signal_Global_Event_Request,
    pub shred: *mut Chuck_VM_Shred,
    _bindgen_union_align: u64,
}
impl Default for Chuck_Global_Request__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Global_Request__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Chuck_Global_Request__bindgen_ty_1 {{ union }}")
    }
}
impl Default for Chuck_Global_Request {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
    pub _base: Chuck_Object,
    pub m_carrier: *mut Chuck_Carrier,
    pub m_adc: *mut Chuck_UGen,
    pub m_dac: *mut Chuck_UGen,
    pub m_bunghole: *mut Chuck_UGen,
    pub m_srate: ::std::os::raw::c_ulong,
    pub m_num_adc_channels: ::std::os::raw::c_ulong,
    pub m_num_dac_channels: ::std::os::raw::c_ulong,
    pub m_halt: ::std::os::raw::c_ulong,
    pub m_is_running: ::std::os::raw::c_ulong,
    pub m_input_ref: *const f64,
    pub m_output_ref: *mut f64,
    pub m_init: ::std::os::raw::c_ulong,
    pub m_last_error: string,
    pub m_shreds: *mut Chuck_VM_Shred,
    pub m_num_shreds: ::std::os::raw::c_ulong,
    pub m_shred_id: ::std::os::raw::c_ulong,
    pub m_shreduler: *mut Chuck_VM_Shreduler,
    pub m_shred_dump: vector,
    pub m_num_dumped_shreds: ::std::os::raw::c_ulong,
    pub m_msg_buffer: *mut CBufferSimple,
    pub m_reply_buffer: *mut CBufferSimple,
    pub m_event_buffer: *mut CBufferSimple,
    pub m_event_buffers: list,
    pub m_global_ints: map,
    pub m_global_floats: map,
    pub m_global_events: map,
    pub m_global_request_queue: XCircleBuffer<Chuck_Global_Request>,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_initialize(
        this: *mut Chuck_VM,
        srate: ::std::os::raw::c_ulong,
        dac_chan: ::std::os::raw::c_ulong,
        adc_chan: ::std::os::raw::c_ulong,
        adaptive: ::std::os::raw::c_ulong,
        halt: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}initialize_synthesis"]
    pub fn Chuck_VM_initialize_synthesis(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setCarrier"]
    pub fn Chuck_VM_setCarrier(
        this: *mut Chuck_VM,
        c: *mut Chuck_Carrier,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_shutdown(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}has_init"]
    pub fn Chuck_VM_has_init(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}start"]
    pub fn Chuck_VM_start(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}running"]
    pub fn Chuck_VM_running(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}stop"]
    pub fn Chuck_VM_stop(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}runningState"]
    pub fn Chuck_VM_runningState(this: *mut Chuck_VM) -> *mut ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}spork"]
    pub fn Chuck_VM_spork(
        this: *mut Chuck_VM,
        code: *mut Chuck_VM_Code,
        parent: *mut Chuck_VM_Shred,
        immediate: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}shreduler"]
    pub fn Chuck_VM_shreduler(this: *const Chuck_VM) -> *mut Chuck_VM_Shreduler;
}
extern "C" {
    #[link_name = "\u{1}next_id"]
    pub fn Chuck_VM_next_id(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}srate"]
    pub fn Chuck_VM_srate(this: *const Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn Chuck_VM_run(
        this: *mut Chuck_VM,
        numFrames: ::std::os::raw::c_long,
        input: *const f64,
        output: *mut f64,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}compute"]
    pub fn Chuck_VM_compute(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}abort_current_shred"]
    pub fn Chuck_VM_abort_current_shred(this: *mut Chuck_VM) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}invoke_static"]
    pub fn Chuck_VM_invoke_static(
        this: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}gc"]
    pub fn Chuck_VM_gc(this: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}gc"]
    pub fn Chuck_VM_gc1(this: *mut Chuck_VM, amount: ::std::os::raw::c_ulong);
}
extern "C" {
    #[link_name = "\u{1}queue_msg"]
    pub fn Chuck_VM_queue_msg(
        this: *mut Chuck_VM,
        msg: *mut Chuck_Msg,
        num_msg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_event"]
    pub fn Chuck_VM_queue_event(
        this: *mut Chuck_VM,
        event: *mut Chuck_Event,
        num_msg: ::std::os::raw::c_int,
        buffer: *mut CBufferSimple,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}process_msg"]
    pub fn Chuck_VM_process_msg(
        this: *mut Chuck_VM,
        msg: *mut Chuck_Msg,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_reply"]
    pub fn Chuck_VM_get_reply(this: *mut Chuck_VM) -> *mut Chuck_Msg;
}
extern "C" {
    #[link_name = "\u{1}create_event_buffer"]
    pub fn Chuck_VM_create_event_buffer(this: *mut Chuck_VM) -> *mut CBufferSimple;
}
extern "C" {
    #[link_name = "\u{1}destroy_event_buffer"]
    pub fn Chuck_VM_destroy_event_buffer(this: *mut Chuck_VM, buffer: *mut CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}last_error"]
    pub fn Chuck_VM_last_error(this: *const Chuck_VM) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}get_global_int"]
    pub fn Chuck_VM_get_global_int(
        this: *mut Chuck_VM,
        name: string,
        callback: ::std::option::Option<
            unsafe extern "C" fn(this: *mut Chuck_VM, name: ::std::os::raw::c_long),
        >,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_global_int"]
    pub fn Chuck_VM_set_global_int(
        this: *mut Chuck_VM,
        name: string,
        val: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_float"]
    pub fn Chuck_VM_get_global_float(
        this: *mut Chuck_VM,
        name: string,
        callback: ::std::option::Option<unsafe extern "C" fn(this: *mut Chuck_VM, name: f64)>,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_global_float"]
    pub fn Chuck_VM_set_global_float(
        this: *mut Chuck_VM,
        name: string,
        val: f64,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signal_global_event"]
    pub fn Chuck_VM_signal_global_event(
        this: *mut Chuck_VM,
        name: string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}broadcast_global_event"]
    pub fn Chuck_VM_broadcast_global_event(
        this: *mut Chuck_VM,
        name: string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}init_global_int"]
    pub fn Chuck_VM_init_global_int(this: *mut Chuck_VM, name: string) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_int_value"]
    pub fn Chuck_VM_get_global_int_value(
        this: *mut Chuck_VM,
        name: string,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_int"]
    pub fn Chuck_VM_get_ptr_to_global_int(
        this: *mut Chuck_VM,
        name: string,
    ) -> *mut ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}init_global_float"]
    pub fn Chuck_VM_init_global_float(this: *mut Chuck_VM, name: string)
        -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_float_value"]
    pub fn Chuck_VM_get_global_float_value(this: *mut Chuck_VM, name: string) -> f64;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_float"]
    pub fn Chuck_VM_get_ptr_to_global_float(this: *mut Chuck_VM, name: string) -> *mut f64;
}
extern "C" {
    #[link_name = "\u{1}init_global_event"]
    pub fn Chuck_VM_init_global_event(
        this: *mut Chuck_VM,
        name: string,
        type_: *mut Chuck_Type,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_event"]
    pub fn Chuck_VM_get_global_event(this: *mut Chuck_VM, name: string) -> *mut Chuck_Event;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_event"]
    pub fn Chuck_VM_get_ptr_to_global_event(
        this: *mut Chuck_VM,
        name: string,
    ) -> *mut *mut Chuck_Event;
}
extern "C" {
    #[link_name = "\u{1}handle_global_queue_messages"]
    pub fn Chuck_VM_handle_global_queue_messages(this: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}carrier"]
    pub fn Chuck_VM_carrier(this: *const Chuck_VM) -> *mut Chuck_Carrier;
}
extern "C" {
    #[link_name = "\u{1}env"]
    pub fn Chuck_VM_env(this: *const Chuck_VM) -> *mut Chuck_Env;
}
extern "C" {
    #[link_name = "\u{1}chout"]
    pub fn Chuck_VM_chout(this: *const Chuck_VM) -> *mut Chuck_IO_Chout;
}
extern "C" {
    #[link_name = "\u{1}cherr"]
    pub fn Chuck_VM_cherr(this: *const Chuck_VM) -> *mut Chuck_IO_Cherr;
}
extern "C" {
    #[link_name = "\u{1}input_ref"]
    pub fn Chuck_VM_input_ref(this: *mut Chuck_VM) -> *const f64;
}
extern "C" {
    #[link_name = "\u{1}output_ref"]
    pub fn Chuck_VM_output_ref(this: *mut Chuck_VM) -> *mut f64;
}
extern "C" {
    #[link_name = "\u{1}spork"]
    pub fn Chuck_VM_spork1(this: *mut Chuck_VM, shred: *mut Chuck_VM_Shred) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}free"]
    pub fn Chuck_VM_free(
        this: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
        cascade: ::std::os::raw::c_ulong,
        dec: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}dump"]
    pub fn Chuck_VM_dump(this: *mut Chuck_VM, shred: *mut Chuck_VM_Shred);
}
extern "C" {
    #[link_name = "\u{1}release_dump"]
    pub fn Chuck_VM_release_dump(this: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM"]
    pub fn Chuck_VM_Chuck_VM(this: *mut Chuck_VM);
}
impl Default for Chuck_VM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
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
        srate: ::std::os::raw::c_ulong,
        dac_chan: ::std::os::raw::c_ulong,
        adc_chan: ::std::os::raw::c_ulong,
        adaptive: ::std::os::raw::c_ulong,
        halt: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_initialize(self, srate, dac_chan, adc_chan, adaptive, halt)
    }
    #[inline]
    pub unsafe fn initialize_synthesis(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_initialize_synthesis(self)
    }
    #[inline]
    pub unsafe fn setCarrier(&mut self, c: *mut Chuck_Carrier) -> ::std::os::raw::c_ulong {
        Chuck_VM_setCarrier(self, c)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_shutdown(self)
    }
    #[inline]
    pub unsafe fn has_init(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_has_init(self)
    }
    #[inline]
    pub unsafe fn start(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_start(self)
    }
    #[inline]
    pub unsafe fn running(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_running(self)
    }
    #[inline]
    pub unsafe fn stop(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_stop(self)
    }
    #[inline]
    pub unsafe fn runningState(&mut self) -> *mut ::std::os::raw::c_ulong {
        Chuck_VM_runningState(self)
    }
    #[inline]
    pub unsafe fn spork(
        &mut self,
        code: *mut Chuck_VM_Code,
        parent: *mut Chuck_VM_Shred,
        immediate: ::std::os::raw::c_ulong,
    ) -> *mut Chuck_VM_Shred {
        Chuck_VM_spork(self, code, parent, immediate)
    }
    #[inline]
    pub unsafe fn shreduler(&self) -> *mut Chuck_VM_Shreduler {
        Chuck_VM_shreduler(self)
    }
    #[inline]
    pub unsafe fn next_id(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_next_id(self)
    }
    #[inline]
    pub unsafe fn srate(&self) -> ::std::os::raw::c_ulong {
        Chuck_VM_srate(self)
    }
    #[inline]
    pub unsafe fn run(
        &mut self,
        numFrames: ::std::os::raw::c_long,
        input: *const f64,
        output: *mut f64,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_run(self, numFrames, input, output)
    }
    #[inline]
    pub unsafe fn compute(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_compute(self)
    }
    #[inline]
    pub unsafe fn abort_current_shred(&mut self) -> ::std::os::raw::c_ulong {
        Chuck_VM_abort_current_shred(self)
    }
    #[inline]
    pub unsafe fn invoke_static(&mut self, shred: *mut Chuck_VM_Shred) -> ::std::os::raw::c_ulong {
        Chuck_VM_invoke_static(self, shred)
    }
    #[inline]
    pub unsafe fn gc(&mut self) {
        Chuck_VM_gc(self)
    }
    #[inline]
    pub unsafe fn gc1(&mut self, amount: ::std::os::raw::c_ulong) {
        Chuck_VM_gc1(self, amount)
    }
    #[inline]
    pub unsafe fn queue_msg(
        &mut self,
        msg: *mut Chuck_Msg,
        num_msg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_queue_msg(self, msg, num_msg)
    }
    #[inline]
    pub unsafe fn queue_event(
        &mut self,
        event: *mut Chuck_Event,
        num_msg: ::std::os::raw::c_int,
        buffer: *mut CBufferSimple,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_queue_event(self, event, num_msg, buffer)
    }
    #[inline]
    pub unsafe fn process_msg(&mut self, msg: *mut Chuck_Msg) -> ::std::os::raw::c_ulong {
        Chuck_VM_process_msg(self, msg)
    }
    #[inline]
    pub unsafe fn get_reply(&mut self) -> *mut Chuck_Msg {
        Chuck_VM_get_reply(self)
    }
    #[inline]
    pub unsafe fn create_event_buffer(&mut self) -> *mut CBufferSimple {
        Chuck_VM_create_event_buffer(self)
    }
    #[inline]
    pub unsafe fn destroy_event_buffer(&mut self, buffer: *mut CBufferSimple) {
        Chuck_VM_destroy_event_buffer(self, buffer)
    }
    #[inline]
    pub unsafe fn last_error(&self) -> *const ::std::os::raw::c_char {
        Chuck_VM_last_error(self)
    }
    #[inline]
    pub unsafe fn get_global_int(
        &mut self,
        name: string,
        callback: ::std::option::Option<
            unsafe extern "C" fn(this: *mut Chuck_VM, name: ::std::os::raw::c_long),
        >,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_get_global_int(self, name, callback)
    }
    #[inline]
    pub unsafe fn set_global_int(
        &mut self,
        name: string,
        val: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_set_global_int(self, name, val)
    }
    #[inline]
    pub unsafe fn get_global_float(
        &mut self,
        name: string,
        callback: ::std::option::Option<unsafe extern "C" fn(this: *mut Chuck_VM, name: f64)>,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_get_global_float(self, name, callback)
    }
    #[inline]
    pub unsafe fn set_global_float(&mut self, name: string, val: f64) -> ::std::os::raw::c_ulong {
        Chuck_VM_set_global_float(self, name, val)
    }
    #[inline]
    pub unsafe fn signal_global_event(&mut self, name: string) -> ::std::os::raw::c_ulong {
        Chuck_VM_signal_global_event(self, name)
    }
    #[inline]
    pub unsafe fn broadcast_global_event(&mut self, name: string) -> ::std::os::raw::c_ulong {
        Chuck_VM_broadcast_global_event(self, name)
    }
    #[inline]
    pub unsafe fn init_global_int(&mut self, name: string) -> ::std::os::raw::c_ulong {
        Chuck_VM_init_global_int(self, name)
    }
    #[inline]
    pub unsafe fn get_global_int_value(&mut self, name: string) -> ::std::os::raw::c_long {
        Chuck_VM_get_global_int_value(self, name)
    }
    #[inline]
    pub unsafe fn get_ptr_to_global_int(&mut self, name: string) -> *mut ::std::os::raw::c_long {
        Chuck_VM_get_ptr_to_global_int(self, name)
    }
    #[inline]
    pub unsafe fn init_global_float(&mut self, name: string) -> ::std::os::raw::c_ulong {
        Chuck_VM_init_global_float(self, name)
    }
    #[inline]
    pub unsafe fn get_global_float_value(&mut self, name: string) -> f64 {
        Chuck_VM_get_global_float_value(self, name)
    }
    #[inline]
    pub unsafe fn get_ptr_to_global_float(&mut self, name: string) -> *mut f64 {
        Chuck_VM_get_ptr_to_global_float(self, name)
    }
    #[inline]
    pub unsafe fn init_global_event(
        &mut self,
        name: string,
        type_: *mut Chuck_Type,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_init_global_event(self, name, type_)
    }
    #[inline]
    pub unsafe fn get_global_event(&mut self, name: string) -> *mut Chuck_Event {
        Chuck_VM_get_global_event(self, name)
    }
    #[inline]
    pub unsafe fn get_ptr_to_global_event(&mut self, name: string) -> *mut *mut Chuck_Event {
        Chuck_VM_get_ptr_to_global_event(self, name)
    }
    #[inline]
    pub unsafe fn handle_global_queue_messages(&mut self) {
        Chuck_VM_handle_global_queue_messages(self)
    }
    #[inline]
    pub unsafe fn carrier(&self) -> *mut Chuck_Carrier {
        Chuck_VM_carrier(self)
    }
    #[inline]
    pub unsafe fn env(&self) -> *mut Chuck_Env {
        Chuck_VM_env(self)
    }
    #[inline]
    pub unsafe fn chout(&self) -> *mut Chuck_IO_Chout {
        Chuck_VM_chout(self)
    }
    #[inline]
    pub unsafe fn cherr(&self) -> *mut Chuck_IO_Cherr {
        Chuck_VM_cherr(self)
    }
    #[inline]
    pub unsafe fn input_ref(&mut self) -> *const f64 {
        Chuck_VM_input_ref(self)
    }
    #[inline]
    pub unsafe fn output_ref(&mut self) -> *mut f64 {
        Chuck_VM_output_ref(self)
    }
    #[inline]
    pub unsafe fn spork1(&mut self, shred: *mut Chuck_VM_Shred) -> *mut Chuck_VM_Shred {
        Chuck_VM_spork1(self, shred)
    }
    #[inline]
    pub unsafe fn free(
        &mut self,
        shred: *mut Chuck_VM_Shred,
        cascade: ::std::os::raw::c_ulong,
        dec: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        Chuck_VM_free(self, shred, cascade, dec)
    }
    #[inline]
    pub unsafe fn dump(&mut self, shred: *mut Chuck_VM_Shred) {
        Chuck_VM_dump(self, shred)
    }
    #[inline]
    pub unsafe fn release_dump(&mut self) {
        Chuck_VM_release_dump(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_VM_Chuck_VM(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_destructor"]
    pub fn Chuck_VM_Chuck_VM_destructor(this: *mut Chuck_VM);
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Chuck_Msg_Type {
    MSG_ADD = 1,
    MSG_REMOVE = 2,
    MSG_REMOVEALL = 3,
    MSG_REPLACE = 4,
    MSG_STATUS = 5,
    MSG_PAUSE = 6,
    MSG_KILL = 7,
    MSG_TIME = 8,
    MSG_RESET_ID = 9,
    MSG_DONE = 10,
    MSG_ABORT = 11,
    MSG_ERROR = 12,
    MSG_CLEARVM = 13,
}
pub type ck_msg_func = ::std::option::Option<unsafe extern "C" fn(msg: *const Chuck_Msg)>;
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Chuck_Msg {
    pub type_: ::std::os::raw::c_ulong,
    pub param: ::std::os::raw::c_ulong,
    pub code: *mut Chuck_VM_Code,
    pub shred: *mut Chuck_VM_Shred,
    pub when: f64,
    pub user: *mut ::std::os::raw::c_void,
    pub reply: ck_msg_func,
    pub replyA: ::std::os::raw::c_ulong,
    pub replyB: ::std::os::raw::c_ulong,
    pub replyC: *mut ::std::os::raw::c_void,
    pub args: *mut vector,
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Msg_clear(this: *mut Chuck_Msg);
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Msg_set(this: *mut Chuck_Msg, vargs: *const vector);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Msg"]
    pub fn Chuck_Msg_Chuck_Msg(this: *mut Chuck_Msg);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Msg_destructor"]
    pub fn Chuck_Msg_Chuck_Msg_destructor(this: *mut Chuck_Msg);
}
impl Default for Chuck_Msg {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Chuck_Msg {
    #[inline]
    pub unsafe fn clear(&mut self) {
        Chuck_Msg_clear(self)
    }
    #[inline]
    pub unsafe fn set(&mut self, vargs: *const vector) {
        Chuck_Msg_set(self, vargs)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Msg_Chuck_Msg(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_Msg_Chuck_Msg_destructor(self)
    }
}
#[repr(C)]
pub struct Chuck_Compiler__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
pub struct Chuck_Compiler {
    pub vtable_: *const Chuck_Compiler__bindgen_vtable,
    pub m_carrier: *mut Chuck_Carrier,
    pub emitter: *mut Chuck_Emitter,
    pub code: *mut Chuck_VM_Code,
    pub m_auto_depend: ::std::os::raw::c_ulong,
    pub m_recent: map,
    pub m_dlls: list,
    pub m_cklibs_to_preload: list,
}
extern "C" {
    #[link_name = "\u{1}env"]
    pub fn Chuck_Compiler_env(this: *const Chuck_Compiler) -> *mut Chuck_Env;
}
extern "C" {
    #[link_name = "\u{1}vm"]
    pub fn Chuck_Compiler_vm(this: *const Chuck_Compiler) -> *mut Chuck_VM;
}
extern "C" {
    #[link_name = "\u{1}carrier"]
    pub fn Chuck_Compiler_carrier(this: *const Chuck_Compiler) -> *mut Chuck_Carrier;
}
extern "C" {
    #[link_name = "\u{1}setCarrier"]
    pub fn Chuck_Compiler_setCarrier(
        this: *mut Chuck_Compiler,
        c: *mut Chuck_Carrier,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_Compiler_initialize(
        this: *mut Chuck_Compiler,
        chugin_search_paths: *mut list,
        named_dls: *mut list,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_Compiler_shutdown(this: *mut Chuck_Compiler);
}
extern "C" {
    #[link_name = "\u{1}bind"]
    pub fn Chuck_Compiler_bind(
        this: *mut Chuck_Compiler,
        query_func: f_ck_query,
        name: *const string,
        nspc: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_auto_depend"]
    pub fn Chuck_Compiler_set_auto_depend(this: *mut Chuck_Compiler, v: ::std::os::raw::c_ulong);
}
extern "C" {
    #[link_name = "\u{1}go"]
    pub fn Chuck_Compiler_go(
        this: *mut Chuck_Compiler,
        filename: *const string,
        fd: *mut FILE,
        str_src: *const ::std::os::raw::c_char,
        full_path: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}resolve"]
    pub fn Chuck_Compiler_resolve(
        this: *mut Chuck_Compiler,
        type_: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}output"]
    pub fn Chuck_Compiler_output(this: *mut Chuck_Compiler) -> *mut Chuck_VM_Code;
}
extern "C" {
    #[link_name = "\u{1}do_entire_file"]
    pub fn Chuck_Compiler_do_entire_file(
        this: *mut Chuck_Compiler,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_only_classes"]
    pub fn Chuck_Compiler_do_only_classes(
        this: *mut Chuck_Compiler,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_all_except_classes"]
    pub fn Chuck_Compiler_do_all_except_classes(
        this: *mut Chuck_Compiler,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_normal"]
    pub fn Chuck_Compiler_do_normal(
        this: *mut Chuck_Compiler,
        path: *const string,
        fd: *mut FILE,
        str_src: *const ::std::os::raw::c_char,
        full_path: *const string,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}find_recent_path"]
    pub fn Chuck_Compiler_find_recent_path(
        this: *mut Chuck_Compiler,
        path: *const string,
    ) -> *mut Chuck_Context;
}
extern "C" {
    #[link_name = "\u{1}find_recent_type"]
    pub fn Chuck_Compiler_find_recent_type(
        this: *mut Chuck_Compiler,
        type_: *const string,
    ) -> *mut Chuck_Context;
}
extern "C" {
    #[link_name = "\u{1}add_recent_path"]
    pub fn Chuck_Compiler_add_recent_path(
        this: *mut Chuck_Compiler,
        path: *const string,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Compiler"]
    pub fn Chuck_Compiler_Chuck_Compiler(this: *mut Chuck_Compiler);
}
impl Default for Chuck_Compiler {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Chuck_Compiler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "Chuck_Compiler {{ m_carrier: {:?}, emitter: {:?}, code: {:?}, m_auto_depend: {:?}, m_recent: {:?}, m_dlls: {:?}, m_cklibs_to_preload: {:?} }}" , self . m_carrier , self . emitter , self . code , self . m_auto_depend , self . m_recent , self . m_dlls , self . m_cklibs_to_preload )
    }
}
impl Chuck_Compiler {
    #[inline]
    pub unsafe fn env(&self) -> *mut Chuck_Env {
        Chuck_Compiler_env(self)
    }
    #[inline]
    pub unsafe fn vm(&self) -> *mut Chuck_VM {
        Chuck_Compiler_vm(self)
    }
    #[inline]
    pub unsafe fn carrier(&self) -> *mut Chuck_Carrier {
        Chuck_Compiler_carrier(self)
    }
    #[inline]
    pub unsafe fn setCarrier(&mut self, c: *mut Chuck_Carrier) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_setCarrier(self, c)
    }
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        chugin_search_paths: *mut list,
        named_dls: *mut list,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_initialize(self, chugin_search_paths, named_dls)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) {
        Chuck_Compiler_shutdown(self)
    }
    #[inline]
    pub unsafe fn bind(
        &mut self,
        query_func: f_ck_query,
        name: *const string,
        nspc: *const string,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_bind(self, query_func, name, nspc)
    }
    #[inline]
    pub unsafe fn set_auto_depend(&mut self, v: ::std::os::raw::c_ulong) {
        Chuck_Compiler_set_auto_depend(self, v)
    }
    #[inline]
    pub unsafe fn go(
        &mut self,
        filename: *const string,
        fd: *mut FILE,
        str_src: *const ::std::os::raw::c_char,
        full_path: *const string,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_go(self, filename, fd, str_src, full_path)
    }
    #[inline]
    pub unsafe fn resolve(&mut self, type_: *const string) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_resolve(self, type_)
    }
    #[inline]
    pub unsafe fn output(&mut self) -> *mut Chuck_VM_Code {
        Chuck_Compiler_output(self)
    }
    #[inline]
    pub unsafe fn do_entire_file(
        &mut self,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_do_entire_file(self, context)
    }
    #[inline]
    pub unsafe fn do_only_classes(
        &mut self,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_do_only_classes(self, context)
    }
    #[inline]
    pub unsafe fn do_all_except_classes(
        &mut self,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_do_all_except_classes(self, context)
    }
    #[inline]
    pub unsafe fn do_normal(
        &mut self,
        path: *const string,
        fd: *mut FILE,
        str_src: *const ::std::os::raw::c_char,
        full_path: *const string,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_do_normal(self, path, fd, str_src, full_path)
    }
    #[inline]
    pub unsafe fn find_recent_path(&mut self, path: *const string) -> *mut Chuck_Context {
        Chuck_Compiler_find_recent_path(self, path)
    }
    #[inline]
    pub unsafe fn find_recent_type(&mut self, type_: *const string) -> *mut Chuck_Context {
        Chuck_Compiler_find_recent_type(self, type_)
    }
    #[inline]
    pub unsafe fn add_recent_path(
        &mut self,
        path: *const string,
        context: *mut Chuck_Context,
    ) -> ::std::os::raw::c_ulong {
        Chuck_Compiler_add_recent_path(self, path, context)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Chuck_Compiler_Chuck_Compiler(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Compiler_destructor"]
    pub fn Chuck_Compiler_Chuck_Compiler_destructor(this: *mut Chuck_Compiler);
}
pub type float_t = f32;
pub type double_t = f64;
// pub const FP_INT_UPWARD: _bindgen_ty_57 = _bindgen_ty_57::FP_INT_UPWARD;
// pub const FP_INT_DOWNWARD: _bindgen_ty_57 = _bindgen_ty_57::FP_INT_DOWNWARD;
// pub const FP_INT_TOWARDZERO: _bindgen_ty_57 = _bindgen_ty_57::FP_INT_TOWARDZERO;
// pub const FP_INT_TONEARESTFROMZERO: _bindgen_ty_57 = _bindgen_ty_57::FP_INT_TONEARESTFROMZERO;
// pub const FP_INT_TONEAREST: _bindgen_ty_57 = _bindgen_ty_57::FP_INT_TONEAREST;
// #[repr(u32)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub enum _bindgen_ty_57 {
//     FP_INT_UPWARD = 0,
//     FP_INT_DOWNWARD = 1,
//     FP_INT_TOWARDZERO = 2,
//     FP_INT_TONEARESTFROMZERO = 3,
//     FP_INT_TONEAREST = 4,
// }
extern "C" {
    pub fn __fpclassify(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbit(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isinf(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finite(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isnan(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __iseqsig(__x: f64, __y: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __issignaling(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acos(__x: f64) -> f64;
}
extern "C" {
    pub fn __acos(__x: f64) -> f64;
}
extern "C" {
    pub fn asin(__x: f64) -> f64;
}
extern "C" {
    pub fn __asin(__x: f64) -> f64;
}
extern "C" {
    pub fn atan(__x: f64) -> f64;
}
extern "C" {
    pub fn __atan(__x: f64) -> f64;
}
extern "C" {
    pub fn atan2(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn __atan2(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn cos(__x: f64) -> f64;
}
extern "C" {
    pub fn __cos(__x: f64) -> f64;
}
extern "C" {
    pub fn sin(__x: f64) -> f64;
}
extern "C" {
    pub fn __sin(__x: f64) -> f64;
}
extern "C" {
    pub fn tan(__x: f64) -> f64;
}
extern "C" {
    pub fn __tan(__x: f64) -> f64;
}
extern "C" {
    pub fn cosh(__x: f64) -> f64;
}
extern "C" {
    pub fn __cosh(__x: f64) -> f64;
}
extern "C" {
    pub fn sinh(__x: f64) -> f64;
}
extern "C" {
    pub fn __sinh(__x: f64) -> f64;
}
extern "C" {
    pub fn tanh(__x: f64) -> f64;
}
extern "C" {
    pub fn __tanh(__x: f64) -> f64;
}
extern "C" {
    pub fn sincos(__x: f64, __sinx: *mut f64, __cosx: *mut f64);
}
extern "C" {
    pub fn __sincos(__x: f64, __sinx: *mut f64, __cosx: *mut f64);
}
extern "C" {
    pub fn acosh(__x: f64) -> f64;
}
extern "C" {
    pub fn __acosh(__x: f64) -> f64;
}
extern "C" {
    pub fn asinh(__x: f64) -> f64;
}
extern "C" {
    pub fn __asinh(__x: f64) -> f64;
}
extern "C" {
    pub fn atanh(__x: f64) -> f64;
}
extern "C" {
    pub fn __atanh(__x: f64) -> f64;
}
extern "C" {
    pub fn exp(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp(__x: f64) -> f64;
}
extern "C" {
    pub fn frexp(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __frexp(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn ldexp(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __ldexp(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn log(__x: f64) -> f64;
}
extern "C" {
    pub fn __log(__x: f64) -> f64;
}
extern "C" {
    pub fn log10(__x: f64) -> f64;
}
extern "C" {
    pub fn __log10(__x: f64) -> f64;
}
extern "C" {
    pub fn modf(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn __modf(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn exp10(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp10(__x: f64) -> f64;
}
extern "C" {
    pub fn expm1(__x: f64) -> f64;
}
extern "C" {
    pub fn __expm1(__x: f64) -> f64;
}
extern "C" {
    pub fn log1p(__x: f64) -> f64;
}
extern "C" {
    pub fn __log1p(__x: f64) -> f64;
}
extern "C" {
    pub fn logb(__x: f64) -> f64;
}
extern "C" {
    pub fn __logb(__x: f64) -> f64;
}
extern "C" {
    pub fn exp2(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp2(__x: f64) -> f64;
}
extern "C" {
    pub fn log2(__x: f64) -> f64;
}
extern "C" {
    pub fn __log2(__x: f64) -> f64;
}
extern "C" {
    pub fn pow(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __pow(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn sqrt(__x: f64) -> f64;
}
extern "C" {
    pub fn __sqrt(__x: f64) -> f64;
}
extern "C" {
    pub fn hypot(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __hypot(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn cbrt(__x: f64) -> f64;
}
extern "C" {
    pub fn __cbrt(__x: f64) -> f64;
}
extern "C" {
    pub fn ceil(__x: f64) -> f64;
}
extern "C" {
    pub fn __ceil(__x: f64) -> f64;
}
extern "C" {
    pub fn fabs(__x: f64) -> f64;
}
extern "C" {
    pub fn __fabs(__x: f64) -> f64;
}
extern "C" {
    pub fn floor(__x: f64) -> f64;
}
extern "C" {
    pub fn __floor(__x: f64) -> f64;
}
extern "C" {
    pub fn fmod(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmod(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn finite(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drem(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __drem(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn significand(__x: f64) -> f64;
}
extern "C" {
    pub fn __significand(__x: f64) -> f64;
}
extern "C" {
    pub fn copysign(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __copysign(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nan(__tagb: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn __nan(__tagb: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn j0(arg1: f64) -> f64;
}
extern "C" {
    pub fn __j0(arg1: f64) -> f64;
}
extern "C" {
    pub fn j1(arg1: f64) -> f64;
}
extern "C" {
    pub fn __j1(arg1: f64) -> f64;
}
extern "C" {
    pub fn jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn __jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn y0(arg1: f64) -> f64;
}
extern "C" {
    pub fn __y0(arg1: f64) -> f64;
}
extern "C" {
    pub fn y1(arg1: f64) -> f64;
}
extern "C" {
    pub fn __y1(arg1: f64) -> f64;
}
extern "C" {
    pub fn yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn __yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn gamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __gamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgamma_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __lgamma_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn rint(__x: f64) -> f64;
}
extern "C" {
    pub fn __rint(__x: f64) -> f64;
}
extern "C" {
    pub fn nextafter(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nextafter(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nexttoward(__x: f64, __y: u128) -> f64;
}
extern "C" {
    pub fn __nexttoward(__x: f64, __y: u128) -> f64;
}
extern "C" {
    pub fn nextdown(__x: f64) -> f64;
}
extern "C" {
    pub fn __nextdown(__x: f64) -> f64;
}
extern "C" {
    pub fn nextup(__x: f64) -> f64;
}
extern "C" {
    pub fn __nextup(__x: f64) -> f64;
}
extern "C" {
    pub fn remainder(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __remainder(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn scalbn(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __scalbn(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn ilogb(__x: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogb(__x: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn llogb(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __llogb(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn scalbln(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn __scalbln(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn nearbyint(__x: f64) -> f64;
}
extern "C" {
    pub fn __nearbyint(__x: f64) -> f64;
}
extern "C" {
    pub fn round(__x: f64) -> f64;
}
extern "C" {
    pub fn __round(__x: f64) -> f64;
}
extern "C" {
    pub fn trunc(__x: f64) -> f64;
}
extern "C" {
    pub fn __trunc(__x: f64) -> f64;
}
extern "C" {
    pub fn remquo(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __remquo(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn lrint(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrint(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrint(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrint(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lround(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lround(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llround(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llround(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdim(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fdim(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmax(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmax(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmin(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmin(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fma(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn __fma(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn roundeven(__x: f64) -> f64;
}
extern "C" {
    pub fn __roundeven(__x: f64) -> f64;
}
extern "C" {
    pub fn fromfp(
        __x: f64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfp(
        __x: f64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfp(
        __x: f64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfp(
        __x: f64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fromfpx(
        __x: f64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpx(
        __x: f64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpx(
        __x: f64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpx(
        __x: f64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fmaxmag(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmaxmag(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fminmag(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fminmag(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn totalorder(__x: f64, __y: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn totalordermag(__x: f64, __y: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalize(__cx: *mut f64, __x: *const f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpayload(__x: *const f64) -> f64;
}
extern "C" {
    pub fn __getpayload(__x: *const f64) -> f64;
}
extern "C" {
    pub fn setpayload(__x: *mut f64, __payload: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpayloadsig(__x: *mut f64, __payload: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalb(__x: f64, __n: f64) -> f64;
}
extern "C" {
    pub fn __scalb(__x: f64, __n: f64) -> f64;
}
extern "C" {
    pub fn __fpclassifyf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbitf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isinff(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finitef(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isnanf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __iseqsigf(__x: f32, __y: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __issignalingf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acosf(__x: f32) -> f32;
}
extern "C" {
    pub fn __acosf(__x: f32) -> f32;
}
extern "C" {
    pub fn asinf(__x: f32) -> f32;
}
extern "C" {
    pub fn __asinf(__x: f32) -> f32;
}
extern "C" {
    pub fn atanf(__x: f32) -> f32;
}
extern "C" {
    pub fn __atanf(__x: f32) -> f32;
}
extern "C" {
    pub fn atan2f(__y: f32, __x: f32) -> f32;
}
extern "C" {
    pub fn __atan2f(__y: f32, __x: f32) -> f32;
}
extern "C" {
    pub fn cosf(__x: f32) -> f32;
}
extern "C" {
    pub fn __cosf(__x: f32) -> f32;
}
extern "C" {
    pub fn sinf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sinf(__x: f32) -> f32;
}
extern "C" {
    pub fn tanf(__x: f32) -> f32;
}
extern "C" {
    pub fn __tanf(__x: f32) -> f32;
}
extern "C" {
    pub fn coshf(__x: f32) -> f32;
}
extern "C" {
    pub fn __coshf(__x: f32) -> f32;
}
extern "C" {
    pub fn sinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn tanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __tanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn sincosf(__x: f32, __sinx: *mut f32, __cosx: *mut f32);
}
extern "C" {
    pub fn __sincosf(__x: f32, __sinx: *mut f32, __cosx: *mut f32);
}
extern "C" {
    pub fn acoshf(__x: f32) -> f32;
}
extern "C" {
    pub fn __acoshf(__x: f32) -> f32;
}
extern "C" {
    pub fn asinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __asinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn atanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __atanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn expf(__x: f32) -> f32;
}
extern "C" {
    pub fn __expf(__x: f32) -> f32;
}
extern "C" {
    pub fn frexpf(__x: f32, __exponent: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __frexpf(__x: f32, __exponent: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn ldexpf(__x: f32, __exponent: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __ldexpf(__x: f32, __exponent: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn logf(__x: f32) -> f32;
}
extern "C" {
    pub fn __logf(__x: f32) -> f32;
}
extern "C" {
    pub fn log10f(__x: f32) -> f32;
}
extern "C" {
    pub fn __log10f(__x: f32) -> f32;
}
extern "C" {
    pub fn modff(__x: f32, __iptr: *mut f32) -> f32;
}
extern "C" {
    pub fn __modff(__x: f32, __iptr: *mut f32) -> f32;
}
extern "C" {
    pub fn exp10f(__x: f32) -> f32;
}
extern "C" {
    pub fn __exp10f(__x: f32) -> f32;
}
extern "C" {
    pub fn expm1f(__x: f32) -> f32;
}
extern "C" {
    pub fn __expm1f(__x: f32) -> f32;
}
extern "C" {
    pub fn log1pf(__x: f32) -> f32;
}
extern "C" {
    pub fn __log1pf(__x: f32) -> f32;
}
extern "C" {
    pub fn logbf(__x: f32) -> f32;
}
extern "C" {
    pub fn __logbf(__x: f32) -> f32;
}
extern "C" {
    pub fn exp2f(__x: f32) -> f32;
}
extern "C" {
    pub fn __exp2f(__x: f32) -> f32;
}
extern "C" {
    pub fn log2f(__x: f32) -> f32;
}
extern "C" {
    pub fn __log2f(__x: f32) -> f32;
}
extern "C" {
    pub fn powf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __powf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn sqrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sqrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn hypotf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __hypotf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn cbrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn __cbrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn ceilf(__x: f32) -> f32;
}
extern "C" {
    pub fn __ceilf(__x: f32) -> f32;
}
extern "C" {
    pub fn fabsf(__x: f32) -> f32;
}
extern "C" {
    pub fn __fabsf(__x: f32) -> f32;
}
extern "C" {
    pub fn floorf(__x: f32) -> f32;
}
extern "C" {
    pub fn __floorf(__x: f32) -> f32;
}
extern "C" {
    pub fn fmodf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmodf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn isinff(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn finitef(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dremf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __dremf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn significandf(__x: f32) -> f32;
}
extern "C" {
    pub fn __significandf(__x: f32) -> f32;
}
extern "C" {
    pub fn copysignf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __copysignf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn nanf(__tagb: *const ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn __nanf(__tagb: *const ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn isnanf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn j0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __j0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn j1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __j1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn jnf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn __jnf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn y0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __y0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn y1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __y1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn ynf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn __ynf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn __erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn gammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __gammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lgammaf_r(arg1: f32, __signgamp: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __lgammaf_r(arg1: f32, __signgamp: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn rintf(__x: f32) -> f32;
}
extern "C" {
    pub fn __rintf(__x: f32) -> f32;
}
extern "C" {
    pub fn nextafterf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __nextafterf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn nexttowardf(__x: f32, __y: u128) -> f32;
}
extern "C" {
    pub fn __nexttowardf(__x: f32, __y: u128) -> f32;
}
extern "C" {
    pub fn nextdownf(__x: f32) -> f32;
}
extern "C" {
    pub fn __nextdownf(__x: f32) -> f32;
}
extern "C" {
    pub fn nextupf(__x: f32) -> f32;
}
extern "C" {
    pub fn __nextupf(__x: f32) -> f32;
}
extern "C" {
    pub fn remainderf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __remainderf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn scalbnf(__x: f32, __n: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __scalbnf(__x: f32, __n: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn ilogbf(__x: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbf(__x: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn llogbf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __llogbf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn scalblnf(__x: f32, __n: ::std::os::raw::c_long) -> f32;
}
extern "C" {
    pub fn __scalblnf(__x: f32, __n: ::std::os::raw::c_long) -> f32;
}
extern "C" {
    pub fn nearbyintf(__x: f32) -> f32;
}
extern "C" {
    pub fn __nearbyintf(__x: f32) -> f32;
}
extern "C" {
    pub fn roundf(__x: f32) -> f32;
}
extern "C" {
    pub fn __roundf(__x: f32) -> f32;
}
extern "C" {
    pub fn truncf(__x: f32) -> f32;
}
extern "C" {
    pub fn __truncf(__x: f32) -> f32;
}
extern "C" {
    pub fn remquof(__x: f32, __y: f32, __quo: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __remquof(__x: f32, __y: f32, __quo: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn lrintf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdimf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fdimf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fmaxf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmaxf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fminf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fminf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
extern "C" {
    pub fn __fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
extern "C" {
    pub fn roundevenf(__x: f32) -> f32;
}
extern "C" {
    pub fn __roundevenf(__x: f32) -> f32;
}
extern "C" {
    pub fn fromfpf(
        __x: f32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpf(
        __x: f32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpf(
        __x: f32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpf(
        __x: f32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fromfpxf(
        __x: f32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpxf(
        __x: f32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpxf(
        __x: f32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpxf(
        __x: f32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fmaxmagf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmaxmagf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fminmagf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fminmagf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn totalorderf(__x: f32, __y: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn totalordermagf(__x: f32, __y: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalizef(__cx: *mut f32, __x: *const f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpayloadf(__x: *const f32) -> f32;
}
extern "C" {
    pub fn __getpayloadf(__x: *const f32) -> f32;
}
extern "C" {
    pub fn setpayloadf(__x: *mut f32, __payload: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpayloadsigf(__x: *mut f32, __payload: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalbf(__x: f32, __n: f32) -> f32;
}
extern "C" {
    pub fn __scalbf(__x: f32, __n: f32) -> f32;
}
extern "C" {
    pub fn __fpclassifyl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbitl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isinfl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finitel(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isnanl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __iseqsigl(__x: u128, __y: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __issignalingl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acosl(__x: u128) -> u128;
}
extern "C" {
    pub fn __acosl(__x: u128) -> u128;
}
extern "C" {
    pub fn asinl(__x: u128) -> u128;
}
extern "C" {
    pub fn __asinl(__x: u128) -> u128;
}
extern "C" {
    pub fn atanl(__x: u128) -> u128;
}
extern "C" {
    pub fn __atanl(__x: u128) -> u128;
}
extern "C" {
    pub fn atan2l(__y: u128, __x: u128) -> u128;
}
extern "C" {
    pub fn __atan2l(__y: u128, __x: u128) -> u128;
}
extern "C" {
    pub fn cosl(__x: u128) -> u128;
}
extern "C" {
    pub fn __cosl(__x: u128) -> u128;
}
extern "C" {
    pub fn sinl(__x: u128) -> u128;
}
extern "C" {
    pub fn __sinl(__x: u128) -> u128;
}
extern "C" {
    pub fn tanl(__x: u128) -> u128;
}
extern "C" {
    pub fn __tanl(__x: u128) -> u128;
}
extern "C" {
    pub fn coshl(__x: u128) -> u128;
}
extern "C" {
    pub fn __coshl(__x: u128) -> u128;
}
extern "C" {
    pub fn sinhl(__x: u128) -> u128;
}
extern "C" {
    pub fn __sinhl(__x: u128) -> u128;
}
extern "C" {
    pub fn tanhl(__x: u128) -> u128;
}
extern "C" {
    pub fn __tanhl(__x: u128) -> u128;
}
extern "C" {
    pub fn sincosl(__x: u128, __sinx: *mut u128, __cosx: *mut u128);
}
extern "C" {
    pub fn __sincosl(__x: u128, __sinx: *mut u128, __cosx: *mut u128);
}
extern "C" {
    pub fn acoshl(__x: u128) -> u128;
}
extern "C" {
    pub fn __acoshl(__x: u128) -> u128;
}
extern "C" {
    pub fn asinhl(__x: u128) -> u128;
}
extern "C" {
    pub fn __asinhl(__x: u128) -> u128;
}
extern "C" {
    pub fn atanhl(__x: u128) -> u128;
}
extern "C" {
    pub fn __atanhl(__x: u128) -> u128;
}
extern "C" {
    pub fn expl(__x: u128) -> u128;
}
extern "C" {
    pub fn __expl(__x: u128) -> u128;
}
extern "C" {
    pub fn frexpl(__x: u128, __exponent: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __frexpl(__x: u128, __exponent: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn ldexpl(__x: u128, __exponent: ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __ldexpl(__x: u128, __exponent: ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn logl(__x: u128) -> u128;
}
extern "C" {
    pub fn __logl(__x: u128) -> u128;
}
extern "C" {
    pub fn log10l(__x: u128) -> u128;
}
extern "C" {
    pub fn __log10l(__x: u128) -> u128;
}
extern "C" {
    pub fn modfl(__x: u128, __iptr: *mut u128) -> u128;
}
extern "C" {
    pub fn __modfl(__x: u128, __iptr: *mut u128) -> u128;
}
extern "C" {
    pub fn exp10l(__x: u128) -> u128;
}
extern "C" {
    pub fn __exp10l(__x: u128) -> u128;
}
extern "C" {
    pub fn expm1l(__x: u128) -> u128;
}
extern "C" {
    pub fn __expm1l(__x: u128) -> u128;
}
extern "C" {
    pub fn log1pl(__x: u128) -> u128;
}
extern "C" {
    pub fn __log1pl(__x: u128) -> u128;
}
extern "C" {
    pub fn logbl(__x: u128) -> u128;
}
extern "C" {
    pub fn __logbl(__x: u128) -> u128;
}
extern "C" {
    pub fn exp2l(__x: u128) -> u128;
}
extern "C" {
    pub fn __exp2l(__x: u128) -> u128;
}
extern "C" {
    pub fn log2l(__x: u128) -> u128;
}
extern "C" {
    pub fn __log2l(__x: u128) -> u128;
}
extern "C" {
    pub fn powl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __powl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn sqrtl(__x: u128) -> u128;
}
extern "C" {
    pub fn __sqrtl(__x: u128) -> u128;
}
extern "C" {
    pub fn hypotl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __hypotl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn cbrtl(__x: u128) -> u128;
}
extern "C" {
    pub fn __cbrtl(__x: u128) -> u128;
}
extern "C" {
    pub fn ceill(__x: u128) -> u128;
}
extern "C" {
    pub fn __ceill(__x: u128) -> u128;
}
extern "C" {
    pub fn fabsl(__x: u128) -> u128;
}
extern "C" {
    pub fn __fabsl(__x: u128) -> u128;
}
extern "C" {
    pub fn floorl(__x: u128) -> u128;
}
extern "C" {
    pub fn __floorl(__x: u128) -> u128;
}
extern "C" {
    pub fn fmodl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fmodl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn isinfl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn finitel(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dreml(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __dreml(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn significandl(__x: u128) -> u128;
}
extern "C" {
    pub fn __significandl(__x: u128) -> u128;
}
extern "C" {
    pub fn copysignl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __copysignl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn nanl(__tagb: *const ::std::os::raw::c_char) -> u128;
}
extern "C" {
    pub fn __nanl(__tagb: *const ::std::os::raw::c_char) -> u128;
}
extern "C" {
    pub fn isnanl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn j0l(arg1: u128) -> u128;
}
extern "C" {
    pub fn __j0l(arg1: u128) -> u128;
}
extern "C" {
    pub fn j1l(arg1: u128) -> u128;
}
extern "C" {
    pub fn __j1l(arg1: u128) -> u128;
}
extern "C" {
    pub fn jnl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
extern "C" {
    pub fn __jnl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
extern "C" {
    pub fn y0l(arg1: u128) -> u128;
}
extern "C" {
    pub fn __y0l(arg1: u128) -> u128;
}
extern "C" {
    pub fn y1l(arg1: u128) -> u128;
}
extern "C" {
    pub fn __y1l(arg1: u128) -> u128;
}
extern "C" {
    pub fn ynl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
extern "C" {
    pub fn __ynl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
extern "C" {
    pub fn erfl(arg1: u128) -> u128;
}
extern "C" {
    pub fn __erfl(arg1: u128) -> u128;
}
extern "C" {
    pub fn erfcl(arg1: u128) -> u128;
}
extern "C" {
    pub fn __erfcl(arg1: u128) -> u128;
}
extern "C" {
    pub fn lgammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn __lgammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn tgammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn __tgammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn gammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn __gammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn lgammal_r(arg1: u128, __signgamp: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __lgammal_r(arg1: u128, __signgamp: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn rintl(__x: u128) -> u128;
}
extern "C" {
    pub fn __rintl(__x: u128) -> u128;
}
extern "C" {
    pub fn nextafterl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __nextafterl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn nexttowardl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __nexttowardl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn nextdownl(__x: u128) -> u128;
}
extern "C" {
    pub fn __nextdownl(__x: u128) -> u128;
}
extern "C" {
    pub fn nextupl(__x: u128) -> u128;
}
extern "C" {
    pub fn __nextupl(__x: u128) -> u128;
}
extern "C" {
    pub fn remainderl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __remainderl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn scalbnl(__x: u128, __n: ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __scalbnl(__x: u128, __n: ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn ilogbl(__x: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbl(__x: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn llogbl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __llogbl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn scalblnl(__x: u128, __n: ::std::os::raw::c_long) -> u128;
}
extern "C" {
    pub fn __scalblnl(__x: u128, __n: ::std::os::raw::c_long) -> u128;
}
extern "C" {
    pub fn nearbyintl(__x: u128) -> u128;
}
extern "C" {
    pub fn __nearbyintl(__x: u128) -> u128;
}
extern "C" {
    pub fn roundl(__x: u128) -> u128;
}
extern "C" {
    pub fn __roundl(__x: u128) -> u128;
}
extern "C" {
    pub fn truncl(__x: u128) -> u128;
}
extern "C" {
    pub fn __truncl(__x: u128) -> u128;
}
extern "C" {
    pub fn remquol(__x: u128, __y: u128, __quo: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __remquol(__x: u128, __y: u128, __quo: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn lrintl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintl(__x: u128) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintl(__x: u128) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundl(__x: u128) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundl(__x: u128) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdiml(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fdiml(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn fmaxl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fmaxl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn fminl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fminl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn fmal(__x: u128, __y: u128, __z: u128) -> u128;
}
extern "C" {
    pub fn __fmal(__x: u128, __y: u128, __z: u128) -> u128;
}
extern "C" {
    pub fn roundevenl(__x: u128) -> u128;
}
extern "C" {
    pub fn __roundevenl(__x: u128) -> u128;
}
extern "C" {
    pub fn fromfpl(
        __x: u128,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpl(
        __x: u128,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpl(
        __x: u128,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpl(
        __x: u128,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fromfpxl(
        __x: u128,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpxl(
        __x: u128,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpxl(
        __x: u128,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpxl(
        __x: u128,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fmaxmagl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fmaxmagl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn fminmagl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fminmagl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn totalorderl(__x: u128, __y: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn totalordermagl(__x: u128, __y: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalizel(__cx: *mut u128, __x: *const u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpayloadl(__x: *const u128) -> u128;
}
extern "C" {
    pub fn __getpayloadl(__x: *const u128) -> u128;
}
extern "C" {
    pub fn setpayloadl(__x: *mut u128, __payload: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpayloadsigl(__x: *mut u128, __payload: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalbl(__x: u128, __n: u128) -> u128;
}
extern "C" {
    pub fn __scalbl(__x: u128, __n: u128) -> u128;
}
extern "C" {
    pub fn acosf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __acosf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn asinf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __asinf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn atanf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __atanf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn atan2f32(__y: _Float32, __x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __atan2f32(__y: _Float32, __x: _Float32) -> _Float32;
}
extern "C" {
    pub fn cosf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __cosf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn sinf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __sinf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn tanf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __tanf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn coshf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __coshf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn sinhf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __sinhf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn tanhf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __tanhf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn sincosf32(__x: _Float32, __sinx: *mut _Float32, __cosx: *mut _Float32);
}
extern "C" {
    pub fn __sincosf32(__x: _Float32, __sinx: *mut _Float32, __cosx: *mut _Float32);
}
extern "C" {
    pub fn acoshf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __acoshf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn asinhf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __asinhf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn atanhf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __atanhf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn expf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __expf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn frexpf32(__x: _Float32, __exponent: *mut ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn __frexpf32(__x: _Float32, __exponent: *mut ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn ldexpf32(__x: _Float32, __exponent: ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn __ldexpf32(__x: _Float32, __exponent: ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn logf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __logf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn log10f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __log10f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn modff32(__x: _Float32, __iptr: *mut _Float32) -> _Float32;
}
extern "C" {
    pub fn __modff32(__x: _Float32, __iptr: *mut _Float32) -> _Float32;
}
extern "C" {
    pub fn exp10f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __exp10f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn expm1f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __expm1f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn log1pf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __log1pf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn logbf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __logbf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn exp2f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __exp2f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn log2f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __log2f32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn powf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __powf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn sqrtf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __sqrtf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn hypotf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __hypotf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn cbrtf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __cbrtf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn ceilf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __ceilf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn fabsf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __fabsf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn floorf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __floorf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn fmodf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __fmodf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn copysignf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __copysignf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn nanf32(__tagb: *const ::std::os::raw::c_char) -> _Float32;
}
extern "C" {
    pub fn __nanf32(__tagb: *const ::std::os::raw::c_char) -> _Float32;
}
extern "C" {
    pub fn j0f32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn __j0f32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn j1f32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn __j1f32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn jnf32(arg1: ::std::os::raw::c_int, arg2: _Float32) -> _Float32;
}
extern "C" {
    pub fn __jnf32(arg1: ::std::os::raw::c_int, arg2: _Float32) -> _Float32;
}
extern "C" {
    pub fn y0f32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn __y0f32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn y1f32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn __y1f32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn ynf32(arg1: ::std::os::raw::c_int, arg2: _Float32) -> _Float32;
}
extern "C" {
    pub fn __ynf32(arg1: ::std::os::raw::c_int, arg2: _Float32) -> _Float32;
}
extern "C" {
    pub fn erff32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn __erff32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn erfcf32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn __erfcf32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn lgammaf32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn __lgammaf32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn tgammaf32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn __tgammaf32(arg1: _Float32) -> _Float32;
}
extern "C" {
    pub fn lgammaf32_r(arg1: _Float32, __signgamp: *mut ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn __lgammaf32_r(arg1: _Float32, __signgamp: *mut ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn rintf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __rintf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn nextafterf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __nextafterf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn nextdownf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __nextdownf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn nextupf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __nextupf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn remainderf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __remainderf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn scalbnf32(__x: _Float32, __n: ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn __scalbnf32(__x: _Float32, __n: ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn ilogbf32(__x: _Float32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbf32(__x: _Float32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn llogbf32(__x: _Float32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __llogbf32(__x: _Float32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn scalblnf32(__x: _Float32, __n: ::std::os::raw::c_long) -> _Float32;
}
extern "C" {
    pub fn __scalblnf32(__x: _Float32, __n: ::std::os::raw::c_long) -> _Float32;
}
extern "C" {
    pub fn nearbyintf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __nearbyintf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn roundf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __roundf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn truncf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __truncf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn remquof32(__x: _Float32, __y: _Float32, __quo: *mut ::std::os::raw::c_int) -> _Float32;
}
extern "C" {
    pub fn __remquof32(__x: _Float32, __y: _Float32, __quo: *mut ::std::os::raw::c_int)
        -> _Float32;
}
extern "C" {
    pub fn lrintf32(__x: _Float32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintf32(__x: _Float32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintf32(__x: _Float32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintf32(__x: _Float32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundf32(__x: _Float32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundf32(__x: _Float32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundf32(__x: _Float32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundf32(__x: _Float32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdimf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __fdimf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn fmaxf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __fmaxf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn fminf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __fminf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn fmaf32(__x: _Float32, __y: _Float32, __z: _Float32) -> _Float32;
}
extern "C" {
    pub fn __fmaf32(__x: _Float32, __y: _Float32, __z: _Float32) -> _Float32;
}
extern "C" {
    pub fn roundevenf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn __roundevenf32(__x: _Float32) -> _Float32;
}
extern "C" {
    pub fn fromfpf32(
        __x: _Float32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpf32(
        __x: _Float32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpf32(
        __x: _Float32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpf32(
        __x: _Float32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fromfpxf32(
        __x: _Float32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpxf32(
        __x: _Float32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpxf32(
        __x: _Float32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpxf32(
        __x: _Float32,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fmaxmagf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __fmaxmagf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn fminmagf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn __fminmagf32(__x: _Float32, __y: _Float32) -> _Float32;
}
extern "C" {
    pub fn totalorderf32(__x: _Float32, __y: _Float32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn totalordermagf32(__x: _Float32, __y: _Float32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalizef32(__cx: *mut _Float32, __x: *const _Float32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpayloadf32(__x: *const _Float32) -> _Float32;
}
extern "C" {
    pub fn __getpayloadf32(__x: *const _Float32) -> _Float32;
}
extern "C" {
    pub fn setpayloadf32(__x: *mut _Float32, __payload: _Float32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpayloadsigf32(__x: *mut _Float32, __payload: _Float32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acosf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __acosf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn asinf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __asinf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn atanf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __atanf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn atan2f64(__y: _Float64, __x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __atan2f64(__y: _Float64, __x: _Float64) -> _Float64;
}
extern "C" {
    pub fn cosf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __cosf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn sinf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __sinf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn tanf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __tanf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn coshf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __coshf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn sinhf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __sinhf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn tanhf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __tanhf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn sincosf64(__x: _Float64, __sinx: *mut _Float64, __cosx: *mut _Float64);
}
extern "C" {
    pub fn __sincosf64(__x: _Float64, __sinx: *mut _Float64, __cosx: *mut _Float64);
}
extern "C" {
    pub fn acoshf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __acoshf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn asinhf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __asinhf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn atanhf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __atanhf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn expf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __expf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn frexpf64(__x: _Float64, __exponent: *mut ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn __frexpf64(__x: _Float64, __exponent: *mut ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn ldexpf64(__x: _Float64, __exponent: ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn __ldexpf64(__x: _Float64, __exponent: ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn logf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __logf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn log10f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __log10f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn modff64(__x: _Float64, __iptr: *mut _Float64) -> _Float64;
}
extern "C" {
    pub fn __modff64(__x: _Float64, __iptr: *mut _Float64) -> _Float64;
}
extern "C" {
    pub fn exp10f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __exp10f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn expm1f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __expm1f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn log1pf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __log1pf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn logbf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __logbf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn exp2f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __exp2f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn log2f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __log2f64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn powf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __powf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn sqrtf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __sqrtf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn hypotf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __hypotf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn cbrtf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __cbrtf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn ceilf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __ceilf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn fabsf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __fabsf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn floorf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __floorf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn fmodf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __fmodf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn copysignf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __copysignf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn nanf64(__tagb: *const ::std::os::raw::c_char) -> _Float64;
}
extern "C" {
    pub fn __nanf64(__tagb: *const ::std::os::raw::c_char) -> _Float64;
}
extern "C" {
    pub fn j0f64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn __j0f64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn j1f64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn __j1f64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn jnf64(arg1: ::std::os::raw::c_int, arg2: _Float64) -> _Float64;
}
extern "C" {
    pub fn __jnf64(arg1: ::std::os::raw::c_int, arg2: _Float64) -> _Float64;
}
extern "C" {
    pub fn y0f64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn __y0f64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn y1f64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn __y1f64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn ynf64(arg1: ::std::os::raw::c_int, arg2: _Float64) -> _Float64;
}
extern "C" {
    pub fn __ynf64(arg1: ::std::os::raw::c_int, arg2: _Float64) -> _Float64;
}
extern "C" {
    pub fn erff64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn __erff64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn erfcf64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn __erfcf64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn lgammaf64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn __lgammaf64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn tgammaf64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn __tgammaf64(arg1: _Float64) -> _Float64;
}
extern "C" {
    pub fn lgammaf64_r(arg1: _Float64, __signgamp: *mut ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn __lgammaf64_r(arg1: _Float64, __signgamp: *mut ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn rintf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __rintf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn nextafterf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __nextafterf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn nextdownf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __nextdownf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn nextupf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __nextupf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn remainderf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __remainderf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn scalbnf64(__x: _Float64, __n: ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn __scalbnf64(__x: _Float64, __n: ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn ilogbf64(__x: _Float64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbf64(__x: _Float64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn llogbf64(__x: _Float64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __llogbf64(__x: _Float64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn scalblnf64(__x: _Float64, __n: ::std::os::raw::c_long) -> _Float64;
}
extern "C" {
    pub fn __scalblnf64(__x: _Float64, __n: ::std::os::raw::c_long) -> _Float64;
}
extern "C" {
    pub fn nearbyintf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __nearbyintf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn roundf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __roundf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn truncf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __truncf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn remquof64(__x: _Float64, __y: _Float64, __quo: *mut ::std::os::raw::c_int) -> _Float64;
}
extern "C" {
    pub fn __remquof64(__x: _Float64, __y: _Float64, __quo: *mut ::std::os::raw::c_int)
        -> _Float64;
}
extern "C" {
    pub fn lrintf64(__x: _Float64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintf64(__x: _Float64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintf64(__x: _Float64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintf64(__x: _Float64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundf64(__x: _Float64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundf64(__x: _Float64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundf64(__x: _Float64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundf64(__x: _Float64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdimf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __fdimf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn fmaxf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __fmaxf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn fminf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __fminf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn fmaf64(__x: _Float64, __y: _Float64, __z: _Float64) -> _Float64;
}
extern "C" {
    pub fn __fmaf64(__x: _Float64, __y: _Float64, __z: _Float64) -> _Float64;
}
extern "C" {
    pub fn roundevenf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn __roundevenf64(__x: _Float64) -> _Float64;
}
extern "C" {
    pub fn fromfpf64(
        __x: _Float64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpf64(
        __x: _Float64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpf64(
        __x: _Float64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpf64(
        __x: _Float64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fromfpxf64(
        __x: _Float64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpxf64(
        __x: _Float64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpxf64(
        __x: _Float64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpxf64(
        __x: _Float64,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fmaxmagf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __fmaxmagf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn fminmagf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn __fminmagf64(__x: _Float64, __y: _Float64) -> _Float64;
}
extern "C" {
    pub fn totalorderf64(__x: _Float64, __y: _Float64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn totalordermagf64(__x: _Float64, __y: _Float64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalizef64(__cx: *mut _Float64, __x: *const _Float64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpayloadf64(__x: *const _Float64) -> _Float64;
}
extern "C" {
    pub fn __getpayloadf64(__x: *const _Float64) -> _Float64;
}
extern "C" {
    pub fn setpayloadf64(__x: *mut _Float64, __payload: _Float64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpayloadsigf64(__x: *mut _Float64, __payload: _Float64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acosf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __acosf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn asinf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __asinf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn atanf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __atanf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn atan2f32x(__y: _Float32x, __x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __atan2f32x(__y: _Float32x, __x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn cosf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __cosf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn sinf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __sinf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn tanf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __tanf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn coshf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __coshf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn sinhf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __sinhf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn tanhf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __tanhf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn sincosf32x(__x: _Float32x, __sinx: *mut _Float32x, __cosx: *mut _Float32x);
}
extern "C" {
    pub fn __sincosf32x(__x: _Float32x, __sinx: *mut _Float32x, __cosx: *mut _Float32x);
}
extern "C" {
    pub fn acoshf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __acoshf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn asinhf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __asinhf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn atanhf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __atanhf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn expf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __expf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn frexpf32x(__x: _Float32x, __exponent: *mut ::std::os::raw::c_int) -> _Float32x;
}
extern "C" {
    pub fn __frexpf32x(__x: _Float32x, __exponent: *mut ::std::os::raw::c_int) -> _Float32x;
}
extern "C" {
    pub fn ldexpf32x(__x: _Float32x, __exponent: ::std::os::raw::c_int) -> _Float32x;
}
extern "C" {
    pub fn __ldexpf32x(__x: _Float32x, __exponent: ::std::os::raw::c_int) -> _Float32x;
}
extern "C" {
    pub fn logf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __logf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn log10f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __log10f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn modff32x(__x: _Float32x, __iptr: *mut _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __modff32x(__x: _Float32x, __iptr: *mut _Float32x) -> _Float32x;
}
extern "C" {
    pub fn exp10f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __exp10f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn expm1f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __expm1f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn log1pf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __log1pf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn logbf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __logbf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn exp2f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __exp2f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn log2f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __log2f32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn powf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __powf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn sqrtf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __sqrtf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn hypotf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __hypotf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn cbrtf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __cbrtf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn ceilf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __ceilf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn fabsf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __fabsf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn floorf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __floorf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn fmodf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __fmodf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn copysignf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __copysignf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn nanf32x(__tagb: *const ::std::os::raw::c_char) -> _Float32x;
}
extern "C" {
    pub fn __nanf32x(__tagb: *const ::std::os::raw::c_char) -> _Float32x;
}
extern "C" {
    pub fn j0f32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __j0f32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn j1f32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __j1f32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn jnf32x(arg1: ::std::os::raw::c_int, arg2: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __jnf32x(arg1: ::std::os::raw::c_int, arg2: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn y0f32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __y0f32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn y1f32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __y1f32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn ynf32x(arg1: ::std::os::raw::c_int, arg2: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __ynf32x(arg1: ::std::os::raw::c_int, arg2: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn erff32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __erff32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn erfcf32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __erfcf32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn lgammaf32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __lgammaf32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn tgammaf32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __tgammaf32x(arg1: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn lgammaf32x_r(arg1: _Float32x, __signgamp: *mut ::std::os::raw::c_int) -> _Float32x;
}
extern "C" {
    pub fn __lgammaf32x_r(arg1: _Float32x, __signgamp: *mut ::std::os::raw::c_int) -> _Float32x;
}
extern "C" {
    pub fn rintf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __rintf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn nextafterf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __nextafterf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn nextdownf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __nextdownf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn nextupf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __nextupf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn remainderf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __remainderf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn scalbnf32x(__x: _Float32x, __n: ::std::os::raw::c_int) -> _Float32x;
}
extern "C" {
    pub fn __scalbnf32x(__x: _Float32x, __n: ::std::os::raw::c_int) -> _Float32x;
}
extern "C" {
    pub fn ilogbf32x(__x: _Float32x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbf32x(__x: _Float32x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn llogbf32x(__x: _Float32x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __llogbf32x(__x: _Float32x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn scalblnf32x(__x: _Float32x, __n: ::std::os::raw::c_long) -> _Float32x;
}
extern "C" {
    pub fn __scalblnf32x(__x: _Float32x, __n: ::std::os::raw::c_long) -> _Float32x;
}
extern "C" {
    pub fn nearbyintf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __nearbyintf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn roundf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __roundf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn truncf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __truncf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn remquof32x(
        __x: _Float32x,
        __y: _Float32x,
        __quo: *mut ::std::os::raw::c_int,
    ) -> _Float32x;
}
extern "C" {
    pub fn __remquof32x(
        __x: _Float32x,
        __y: _Float32x,
        __quo: *mut ::std::os::raw::c_int,
    ) -> _Float32x;
}
extern "C" {
    pub fn lrintf32x(__x: _Float32x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintf32x(__x: _Float32x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintf32x(__x: _Float32x) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintf32x(__x: _Float32x) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundf32x(__x: _Float32x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundf32x(__x: _Float32x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundf32x(__x: _Float32x) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundf32x(__x: _Float32x) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdimf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __fdimf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn fmaxf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __fmaxf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn fminf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __fminf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn fmaf32x(__x: _Float32x, __y: _Float32x, __z: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __fmaf32x(__x: _Float32x, __y: _Float32x, __z: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn roundevenf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __roundevenf32x(__x: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn fromfpf32x(
        __x: _Float32x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpf32x(
        __x: _Float32x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpf32x(
        __x: _Float32x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpf32x(
        __x: _Float32x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fromfpxf32x(
        __x: _Float32x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpxf32x(
        __x: _Float32x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpxf32x(
        __x: _Float32x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpxf32x(
        __x: _Float32x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fmaxmagf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __fmaxmagf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn fminmagf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __fminmagf32x(__x: _Float32x, __y: _Float32x) -> _Float32x;
}
extern "C" {
    pub fn totalorderf32x(__x: _Float32x, __y: _Float32x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn totalordermagf32x(__x: _Float32x, __y: _Float32x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalizef32x(__cx: *mut _Float32x, __x: *const _Float32x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpayloadf32x(__x: *const _Float32x) -> _Float32x;
}
extern "C" {
    pub fn __getpayloadf32x(__x: *const _Float32x) -> _Float32x;
}
extern "C" {
    pub fn setpayloadf32x(__x: *mut _Float32x, __payload: _Float32x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpayloadsigf32x(__x: *mut _Float32x, __payload: _Float32x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acosf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __acosf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn asinf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __asinf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn atanf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __atanf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn atan2f64x(__y: _Float64x, __x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __atan2f64x(__y: _Float64x, __x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn cosf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __cosf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn sinf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __sinf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn tanf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __tanf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn coshf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __coshf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn sinhf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __sinhf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn tanhf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __tanhf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn sincosf64x(__x: _Float64x, __sinx: *mut _Float64x, __cosx: *mut _Float64x);
}
extern "C" {
    pub fn __sincosf64x(__x: _Float64x, __sinx: *mut _Float64x, __cosx: *mut _Float64x);
}
extern "C" {
    pub fn acoshf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __acoshf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn asinhf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __asinhf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn atanhf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __atanhf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn expf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __expf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn frexpf64x(__x: _Float64x, __exponent: *mut ::std::os::raw::c_int) -> _Float64x;
}
extern "C" {
    pub fn __frexpf64x(__x: _Float64x, __exponent: *mut ::std::os::raw::c_int) -> _Float64x;
}
extern "C" {
    pub fn ldexpf64x(__x: _Float64x, __exponent: ::std::os::raw::c_int) -> _Float64x;
}
extern "C" {
    pub fn __ldexpf64x(__x: _Float64x, __exponent: ::std::os::raw::c_int) -> _Float64x;
}
extern "C" {
    pub fn logf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __logf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn log10f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __log10f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn modff64x(__x: _Float64x, __iptr: *mut _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __modff64x(__x: _Float64x, __iptr: *mut _Float64x) -> _Float64x;
}
extern "C" {
    pub fn exp10f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __exp10f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn expm1f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __expm1f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn log1pf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __log1pf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn logbf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __logbf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn exp2f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __exp2f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn log2f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __log2f64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn powf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __powf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn sqrtf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __sqrtf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn hypotf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __hypotf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn cbrtf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __cbrtf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn ceilf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __ceilf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn fabsf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __fabsf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn floorf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __floorf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn fmodf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __fmodf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn copysignf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __copysignf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn nanf64x(__tagb: *const ::std::os::raw::c_char) -> _Float64x;
}
extern "C" {
    pub fn __nanf64x(__tagb: *const ::std::os::raw::c_char) -> _Float64x;
}
extern "C" {
    pub fn j0f64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __j0f64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn j1f64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __j1f64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn jnf64x(arg1: ::std::os::raw::c_int, arg2: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __jnf64x(arg1: ::std::os::raw::c_int, arg2: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn y0f64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __y0f64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn y1f64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __y1f64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn ynf64x(arg1: ::std::os::raw::c_int, arg2: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __ynf64x(arg1: ::std::os::raw::c_int, arg2: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn erff64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __erff64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn erfcf64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __erfcf64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn lgammaf64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __lgammaf64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn tgammaf64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __tgammaf64x(arg1: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn lgammaf64x_r(arg1: _Float64x, __signgamp: *mut ::std::os::raw::c_int) -> _Float64x;
}
extern "C" {
    pub fn __lgammaf64x_r(arg1: _Float64x, __signgamp: *mut ::std::os::raw::c_int) -> _Float64x;
}
extern "C" {
    pub fn rintf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __rintf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn nextafterf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __nextafterf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn nextdownf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __nextdownf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn nextupf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __nextupf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn remainderf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __remainderf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn scalbnf64x(__x: _Float64x, __n: ::std::os::raw::c_int) -> _Float64x;
}
extern "C" {
    pub fn __scalbnf64x(__x: _Float64x, __n: ::std::os::raw::c_int) -> _Float64x;
}
extern "C" {
    pub fn ilogbf64x(__x: _Float64x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbf64x(__x: _Float64x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn llogbf64x(__x: _Float64x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __llogbf64x(__x: _Float64x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn scalblnf64x(__x: _Float64x, __n: ::std::os::raw::c_long) -> _Float64x;
}
extern "C" {
    pub fn __scalblnf64x(__x: _Float64x, __n: ::std::os::raw::c_long) -> _Float64x;
}
extern "C" {
    pub fn nearbyintf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __nearbyintf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn roundf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __roundf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn truncf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __truncf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn remquof64x(
        __x: _Float64x,
        __y: _Float64x,
        __quo: *mut ::std::os::raw::c_int,
    ) -> _Float64x;
}
extern "C" {
    pub fn __remquof64x(
        __x: _Float64x,
        __y: _Float64x,
        __quo: *mut ::std::os::raw::c_int,
    ) -> _Float64x;
}
extern "C" {
    pub fn lrintf64x(__x: _Float64x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintf64x(__x: _Float64x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintf64x(__x: _Float64x) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintf64x(__x: _Float64x) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundf64x(__x: _Float64x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundf64x(__x: _Float64x) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundf64x(__x: _Float64x) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundf64x(__x: _Float64x) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdimf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __fdimf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn fmaxf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __fmaxf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn fminf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __fminf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn fmaf64x(__x: _Float64x, __y: _Float64x, __z: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __fmaf64x(__x: _Float64x, __y: _Float64x, __z: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn roundevenf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __roundevenf64x(__x: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn fromfpf64x(
        __x: _Float64x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpf64x(
        __x: _Float64x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpf64x(
        __x: _Float64x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpf64x(
        __x: _Float64x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fromfpxf64x(
        __x: _Float64x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn __fromfpxf64x(
        __x: _Float64x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __intmax_t;
}
extern "C" {
    pub fn ufromfpxf64x(
        __x: _Float64x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn __ufromfpxf64x(
        __x: _Float64x,
        __round: ::std::os::raw::c_int,
        __width: ::std::os::raw::c_uint,
    ) -> __uintmax_t;
}
extern "C" {
    pub fn fmaxmagf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __fmaxmagf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn fminmagf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __fminmagf64x(__x: _Float64x, __y: _Float64x) -> _Float64x;
}
extern "C" {
    pub fn totalorderf64x(__x: _Float64x, __y: _Float64x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn totalordermagf64x(__x: _Float64x, __y: _Float64x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn canonicalizef64x(__cx: *mut _Float64x, __x: *const _Float64x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpayloadf64x(__x: *const _Float64x) -> _Float64x;
}
extern "C" {
    pub fn __getpayloadf64x(__x: *const _Float64x) -> _Float64x;
}
extern "C" {
    pub fn setpayloadf64x(__x: *mut _Float64x, __payload: _Float64x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpayloadsigf64x(__x: *mut _Float64x, __payload: _Float64x) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fadd(__x: f64, __y: f64) -> f32;
}
extern "C" {
    pub fn fdiv(__x: f64, __y: f64) -> f32;
}
extern "C" {
    pub fn fmul(__x: f64, __y: f64) -> f32;
}
extern "C" {
    pub fn fsub(__x: f64, __y: f64) -> f32;
}
extern "C" {
    pub fn faddl(__x: u128, __y: u128) -> f32;
}
extern "C" {
    pub fn fdivl(__x: u128, __y: u128) -> f32;
}
extern "C" {
    pub fn fmull(__x: u128, __y: u128) -> f32;
}
extern "C" {
    pub fn fsubl(__x: u128, __y: u128) -> f32;
}
extern "C" {
    pub fn daddl(__x: u128, __y: u128) -> f64;
}
extern "C" {
    pub fn ddivl(__x: u128, __y: u128) -> f64;
}
extern "C" {
    pub fn dmull(__x: u128, __y: u128) -> f64;
}
extern "C" {
    pub fn dsubl(__x: u128, __y: u128) -> f64;
}
extern "C" {
    pub fn f32addf32x(__x: _Float32x, __y: _Float32x) -> _Float32;
}
extern "C" {
    pub fn f32divf32x(__x: _Float32x, __y: _Float32x) -> _Float32;
}
extern "C" {
    pub fn f32mulf32x(__x: _Float32x, __y: _Float32x) -> _Float32;
}
extern "C" {
    pub fn f32subf32x(__x: _Float32x, __y: _Float32x) -> _Float32;
}
extern "C" {
    pub fn f32addf64(__x: _Float64, __y: _Float64) -> _Float32;
}
extern "C" {
    pub fn f32divf64(__x: _Float64, __y: _Float64) -> _Float32;
}
extern "C" {
    pub fn f32mulf64(__x: _Float64, __y: _Float64) -> _Float32;
}
extern "C" {
    pub fn f32subf64(__x: _Float64, __y: _Float64) -> _Float32;
}
extern "C" {
    pub fn f32addf64x(__x: _Float64x, __y: _Float64x) -> _Float32;
}
extern "C" {
    pub fn f32divf64x(__x: _Float64x, __y: _Float64x) -> _Float32;
}
extern "C" {
    pub fn f32mulf64x(__x: _Float64x, __y: _Float64x) -> _Float32;
}
extern "C" {
    pub fn f32subf64x(__x: _Float64x, __y: _Float64x) -> _Float32;
}
extern "C" {
    pub fn f32xaddf64(__x: _Float64, __y: _Float64) -> _Float32x;
}
extern "C" {
    pub fn f32xdivf64(__x: _Float64, __y: _Float64) -> _Float32x;
}
extern "C" {
    pub fn f32xmulf64(__x: _Float64, __y: _Float64) -> _Float32x;
}
extern "C" {
    pub fn f32xsubf64(__x: _Float64, __y: _Float64) -> _Float32x;
}
extern "C" {
    pub fn f32xaddf64x(__x: _Float64x, __y: _Float64x) -> _Float32x;
}
extern "C" {
    pub fn f32xdivf64x(__x: _Float64x, __y: _Float64x) -> _Float32x;
}
extern "C" {
    pub fn f32xmulf64x(__x: _Float64x, __y: _Float64x) -> _Float32x;
}
extern "C" {
    pub fn f32xsubf64x(__x: _Float64x, __y: _Float64x) -> _Float32x;
}
extern "C" {
    pub fn f64addf64x(__x: _Float64x, __y: _Float64x) -> _Float64;
}
extern "C" {
    pub fn f64divf64x(__x: _Float64x, __y: _Float64x) -> _Float64;
}
extern "C" {
    pub fn f64mulf64x(__x: _Float64x, __y: _Float64x) -> _Float64;
}
extern "C" {
    pub fn f64subf64x(__x: _Float64x, __y: _Float64x) -> _Float64;
}
extern "C" {
    pub static mut signgam: ::std::os::raw::c_int;
}
// pub const FP_NAN: _bindgen_ty_58 = _bindgen_ty_58::FP_NAN;
// pub const FP_INFINITE: _bindgen_ty_58 = _bindgen_ty_58::FP_INFINITE;
// pub const FP_ZERO: _bindgen_ty_58 = _bindgen_ty_58::FP_ZERO;
// pub const FP_SUBNORMAL: _bindgen_ty_58 = _bindgen_ty_58::FP_SUBNORMAL;
// pub const FP_NORMAL: _bindgen_ty_58 = _bindgen_ty_58::FP_NORMAL;
// #[repr(u32)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub enum _bindgen_ty_58 {
//     FP_NAN = 0,
//     FP_INFINITE = 1,
//     FP_ZERO = 2,
//     FP_SUBNORMAL = 3,
//     FP_NORMAL = 4,
// }
extern "C" {
    pub fn __iscanonicall(__x: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscanonical(__val: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn issignaling(__val: f32) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __iseqsig_type {
    pub _address: u8,
}
pub const g_srate: ::std::os::raw::c_ulong = 0;
pub const osc_offset_data: ::std::os::raw::c_ulong = 0;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Osc_Data {
    pub num: f64,
    pub freq: f64,
    pub sync: ::std::os::raw::c_int,
    pub srate: ::std::os::raw::c_ulong,
    pub width: f64,
    pub phase: f64,
}
extern "C" {
    #[link_name = "\u{1}Osc_Data"]
    pub fn Osc_Data_Osc_Data(this: *mut Osc_Data);
}
impl Osc_Data {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Osc_Data_Osc_Data(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
pub const genX_offset_data: ::std::os::raw::c_ulong = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genX_Data {
    pub genX_type: ::std::os::raw::c_ulong,
    pub genX_table: [f64; 4096usize],
    pub sync: ::std::os::raw::c_long,
    pub srate: ::std::os::raw::c_ulong,
    pub xtemp: f64,
    pub coeffs: [f64; 100usize],
}
extern "C" {
    #[link_name = "\u{1}genX_Data"]
    pub fn genX_Data_genX_Data(this: *mut genX_Data);
}
impl Default for genX_Data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for genX_Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "genX_Data {{ genX_type: {:?}, genX_table: [{}], sync: {:?}, srate: {:?}, xtemp: {:?}, coeffs: [{}] }}" , self . genX_type , self . genX_table . iter ( ) . enumerate ( ) . map ( | ( i , v ) | format ! ( "{}{:?}" , if i > 0 { ", " } else { "" } , v ) ) . collect :: < String > ( ) , self . sync , self . srate , self . xtemp , self . coeffs . iter ( ) . enumerate ( ) . map ( | ( i , v ) | format ! ( "{}{:?}" , if i > 0 { ", " } else { "" } , v ) ) . collect :: < String > ( ) )
    }
}
impl ::std::cmp::PartialEq for genX_Data {
    fn eq(&self, other: &genX_Data) -> bool {
        self.genX_type == other.genX_type
            && &self.genX_table[..] == &other.genX_table[..]
            && self.sync == other.sync
            && self.srate == other.srate
            && self.xtemp == other.xtemp
            && &self.coeffs[..] == &other.coeffs[..]
    }
}
impl genX_Data {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        genX_Data_genX_Data(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __locale_data {
    pub _address: u8,
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
impl Default for __va_list_tag {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
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
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_39 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_40 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_41 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_42 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_43 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_44 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_45 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_46 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_47 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_48 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_49 {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_51 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct _bindgen_ty_52 {
    pub _address: u8,
}
pub type iterator = _Bit_iterator;
pub type char_type = ::std::os::raw::c_char;
pub type int_type = ::std::os::raw::c_int;
