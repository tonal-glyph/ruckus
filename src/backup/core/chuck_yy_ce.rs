use std::fmt::*;
use std::mem::*;
use std::option::*;
use std::os::raw::*;
use std::ptr::null;
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
pub const __OBJC_BOOL_IS_BOOL: u32 = 0;
pub const __CONSTANT_CFSTRINGS__: u32 = 1;
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
pub const __GNUC_STDC_INLINE__: u32 = 1;
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
pub const __FLOAT128__: u32 = 1;
pub const __STDC__: u32 = 1;
pub const __STDC_HOSTED__: u32 = 1;
pub const __STDC_VERSION__: u32 = 201112;
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
pub const YY_FLEX_MAJOR_VERSION: u32 = 2;
pub const YY_FLEX_MINOR_VERSION: u32 = 6;
pub const YY_FLEX_SUBMINOR_VERSION: u32 = 4;
pub const _STDIO_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
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
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __USE_EXTERN_INLINES: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _____fpos_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const FOPEN_MAX: u32 = 16;
pub const _BITS_STDIO_H: u32 = 1;
pub const _STRING_H: u32 = 1;
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
pub const _STDLIB_H: u32 = 1;
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
pub const __STDC_LIMIT_MACROS: u32 = 1;
pub const _INTTYPES_H: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const __WCHAR_MAX: u32 = 2147483647;
pub const __WCHAR_MIN: i32 = -2147483648;
pub const _BITS_STDINT_INTN_H: u32 = 1;
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
pub const WCHAR_MIN: i32 = -2147483648;
pub const WCHAR_MAX: u32 = 2147483647;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const ____gwchar_t_defined: u32 = 1;
pub const __PRI64_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const PRId8: &'static [u8; 2usize] = b"d\0";
pub const PRId16: &'static [u8; 2usize] = b"d\0";
pub const PRId32: &'static [u8; 2usize] = b"d\0";
pub const PRId64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIi8: &'static [u8; 2usize] = b"i\0";
pub const PRIi16: &'static [u8; 2usize] = b"i\0";
pub const PRIi32: &'static [u8; 2usize] = b"i\0";
pub const PRIi64: &'static [u8; 3usize] = b"li\0";
pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiFAST16: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST32: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIo8: &'static [u8; 2usize] = b"o\0";
pub const PRIo16: &'static [u8; 2usize] = b"o\0";
pub const PRIo32: &'static [u8; 2usize] = b"o\0";
pub const PRIo64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIu8: &'static [u8; 2usize] = b"u\0";
pub const PRIu16: &'static [u8; 2usize] = b"u\0";
pub const PRIu32: &'static [u8; 2usize] = b"u\0";
pub const PRIu64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIx8: &'static [u8; 2usize] = b"x\0";
pub const PRIx16: &'static [u8; 2usize] = b"x\0";
pub const PRIx32: &'static [u8; 2usize] = b"x\0";
pub const PRIx64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIX8: &'static [u8; 2usize] = b"X\0";
pub const PRIX16: &'static [u8; 2usize] = b"X\0";
pub const PRIX32: &'static [u8; 2usize] = b"X\0";
pub const PRIX64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXFAST16: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST32: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIdMAX: &'static [u8; 3usize] = b"ld\0";
pub const PRIiMAX: &'static [u8; 3usize] = b"li\0";
pub const PRIoMAX: &'static [u8; 3usize] = b"lo\0";
pub const PRIuMAX: &'static [u8; 3usize] = b"lu\0";
pub const PRIxMAX: &'static [u8; 3usize] = b"lx\0";
pub const PRIXMAX: &'static [u8; 3usize] = b"lX\0";
pub const PRIdPTR: &'static [u8; 3usize] = b"ld\0";
pub const PRIiPTR: &'static [u8; 3usize] = b"li\0";
pub const PRIoPTR: &'static [u8; 3usize] = b"lo\0";
pub const PRIuPTR: &'static [u8; 3usize] = b"lu\0";
pub const PRIxPTR: &'static [u8; 3usize] = b"lx\0";
pub const PRIXPTR: &'static [u8; 3usize] = b"lX\0";
pub const SCNd8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNd16: &'static [u8; 3usize] = b"hd\0";
pub const SCNd32: &'static [u8; 2usize] = b"d\0";
pub const SCNd64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\0";
pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const SCNdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNi8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNi16: &'static [u8; 3usize] = b"hi\0";
pub const SCNi32: &'static [u8; 2usize] = b"i\0";
pub const SCNi64: &'static [u8; 3usize] = b"li\0";
pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\0";
pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const SCNiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiFAST16: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST32: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNu8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNu16: &'static [u8; 3usize] = b"hu\0";
pub const SCNu32: &'static [u8; 2usize] = b"u\0";
pub const SCNu64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\0";
pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const SCNuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNo8: &'static [u8; 4usize] = b"hho\0";
pub const SCNo16: &'static [u8; 3usize] = b"ho\0";
pub const SCNo32: &'static [u8; 2usize] = b"o\0";
pub const SCNo64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\0";
pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const SCNoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNx8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNx16: &'static [u8; 3usize] = b"hx\0";
pub const SCNx32: &'static [u8; 2usize] = b"x\0";
pub const SCNx64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\0";
pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const SCNxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNdMAX: &'static [u8; 3usize] = b"ld\0";
pub const SCNiMAX: &'static [u8; 3usize] = b"li\0";
pub const SCNoMAX: &'static [u8; 3usize] = b"lo\0";
pub const SCNuMAX: &'static [u8; 3usize] = b"lu\0";
pub const SCNxMAX: &'static [u8; 3usize] = b"lx\0";
pub const SCNdPTR: &'static [u8; 3usize] = b"ld\0";
pub const SCNiPTR: &'static [u8; 3usize] = b"li\0";
pub const SCNoPTR: &'static [u8; 3usize] = b"lo\0";
pub const SCNuPTR: &'static [u8; 3usize] = b"lu\0";
pub const SCNxPTR: &'static [u8; 3usize] = b"lx\0";
pub const YY_NULL: u32 = 0;
pub const YY_END_OF_BUFFER_CHAR: u32 = 0;
pub const YY_BUF_SIZE: u32 = 16384;
pub const EOB_ACT_CONTINUE_SCAN: u32 = 0;
pub const EOB_ACT_END_OF_FILE: u32 = 1;
pub const EOB_ACT_LAST_MATCH: u32 = 2;
pub const YY_BUFFER_NEW: u32 = 0;
pub const YY_BUFFER_NORMAL: u32 = 1;
pub const YY_BUFFER_EOF_PENDING: u32 = 2;
pub const YY_NUM_RULES: u32 = 105;
pub const YY_END_OF_BUFFER: u32 = 106;
pub const YY_MORE_ADJ: u32 = 0;
pub const _MEMORY_H: u32 = 1;
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
pub const YYDEBUG: u32 = 0;
pub const YYSTYPE_IS_TRIVIAL: u32 = 1;
pub const YYSTYPE_IS_DECLARED: u32 = 1;
pub const INITIAL: u32 = 0;
pub const _UNISTD_H: u32 = 1;
pub const _POSIX_VERSION: u32 = 199009;
pub const __POSIX2_THIS_VERSION: u32 = 199209;
pub const _POSIX2_VERSION: u32 = 199209;
pub const _POSIX2_C_VERSION: u32 = 199209;
pub const _POSIX2_C_BIND: u32 = 199209;
pub const _POSIX2_C_DEV: u32 = 199209;
pub const _POSIX2_SW_DEV: u32 = 199209;
pub const _POSIX2_LOCALEDEF: u32 = 199209;
pub const _XOPEN_VERSION: u32 = 4;
pub const _XOPEN_XCU_VERSION: u32 = 4;
pub const _XOPEN_XPG2: u32 = 1;
pub const _XOPEN_XPG3: u32 = 1;
pub const _XOPEN_XPG4: u32 = 1;
pub const _XOPEN_UNIX: u32 = 1;
pub const _XOPEN_ENH_I18N: u32 = 1;
pub const _XOPEN_LEGACY: u32 = 1;
pub const _BITS_POSIX_OPT_H: u32 = 1;
pub const _POSIX_JOB_CONTROL: u32 = 1;
pub const _POSIX_SAVED_IDS: u32 = 1;
pub const _POSIX_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_SYNCHRONIZED_IO: u32 = 200809;
pub const _POSIX_FSYNC: u32 = 200809;
pub const _POSIX_MAPPED_FILES: u32 = 200809;
pub const _POSIX_MEMLOCK: u32 = 200809;
pub const _POSIX_MEMLOCK_RANGE: u32 = 200809;
pub const _POSIX_MEMORY_PROTECTION: u32 = 200809;
pub const _POSIX_CHOWN_RESTRICTED: u32 = 0;
pub const _POSIX_VDISABLE: u8 = 0u8;
pub const _POSIX_NO_TRUNC: u32 = 1;
pub const _XOPEN_REALTIME: u32 = 1;
pub const _XOPEN_REALTIME_THREADS: u32 = 1;
pub const _XOPEN_SHM: u32 = 1;
pub const _POSIX_THREADS: u32 = 200809;
pub const _POSIX_REENTRANT_FUNCTIONS: u32 = 1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: u32 = 200809;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKSIZE: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKADDR: u32 = 200809;
pub const _POSIX_THREAD_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_PRIO_PROTECT: u32 = 200809;
pub const _POSIX_SEMAPHORES: u32 = 200809;
pub const _POSIX_REALTIME_SIGNALS: u32 = 200809;
pub const _POSIX_ASYNCHRONOUS_IO: u32 = 200809;
pub const _POSIX_ASYNC_IO: u32 = 1;
pub const _LFS_ASYNCHRONOUS_IO: u32 = 1;
pub const _POSIX_PRIORITIZED_IO: u32 = 200809;
pub const _LFS64_ASYNCHRONOUS_IO: u32 = 1;
pub const _LFS_LARGEFILE: u32 = 1;
pub const _LFS64_LARGEFILE: u32 = 1;
pub const _LFS64_STDIO: u32 = 1;
pub const _POSIX_SHARED_MEMORY_OBJECTS: u32 = 200809;
pub const _POSIX_CPUTIME: u32 = 0;
pub const _POSIX_THREAD_CPUTIME: u32 = 0;
pub const _POSIX_REGEXP: u32 = 1;
pub const _POSIX_READER_WRITER_LOCKS: u32 = 200809;
pub const _POSIX_SHELL: u32 = 1;
pub const _POSIX_TIMEOUTS: u32 = 200809;
pub const _POSIX_SPIN_LOCKS: u32 = 200809;
pub const _POSIX_SPAWN: u32 = 200809;
pub const _POSIX_TIMERS: u32 = 200809;
pub const _POSIX_BARRIERS: u32 = 200809;
pub const _POSIX_MESSAGE_PASSING: u32 = 200809;
pub const _POSIX_THREAD_PROCESS_SHARED: u32 = 200809;
pub const _POSIX_MONOTONIC_CLOCK: u32 = 0;
pub const _POSIX_CLOCK_SELECTION: u32 = 200809;
pub const _POSIX_ADVISORY_INFO: u32 = 200809;
pub const _POSIX_IPV6: u32 = 200809;
pub const _POSIX_RAW_SOCKETS: u32 = 200809;
pub const _POSIX2_CHAR_TERM: u32 = 200809;
pub const _POSIX_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_THREAD_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_TRACE: i32 = -1;
pub const _POSIX_TRACE_EVENT_FILTER: i32 = -1;
pub const _POSIX_TRACE_INHERIT: i32 = -1;
pub const _POSIX_TRACE_LOG: i32 = -1;
pub const _POSIX_TYPED_MEMORY_OBJECTS: i32 = -1;
pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;
pub const R_OK: u32 = 4;
pub const W_OK: u32 = 2;
pub const X_OK: u32 = 1;
pub const F_OK: u32 = 0;
pub const YY_READ_BUF_SIZE: u32 = 8192;
pub const YY_START_STACK_INCR: u32 = 25;
pub const YY_DECL_IS_OURS: u32 = 1;
pub const YY_EXIT_FAILURE: u32 = 2;
pub const YYTABLES_NAME: &'static [u8; 9usize] = b"yytables\0";
pub type __gnuc_va_list = self::__builtin_va_list;
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
pub type fpos_t = self::__fpos_t;
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
    pub fn tmpfile() -> *mut self::FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut self::FILE) -> c_int;
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
    pub fn fputc(__c: c_int, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn putc(__c: c_int, __stream: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn putchar(__c: c_int) -> c_int;
}
extern "C" {
    pub fn fgets(__s: *mut c_char, __n: c_int, __stream: *mut self::FILE) -> *mut c_char;
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
    pub fn fread(
        __ptr: *mut c_void,
        __size: usize,
        __n: usize,
        __stream: *mut self::FILE,
    ) -> c_ulong;
}
extern "C" {
    pub fn fwrite(__ptr: *const c_void, __size: usize, __n: usize, __s: *mut self::FILE)
        -> c_ulong;
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
    pub fn fgetpos(__stream: *mut self::FILE, __pos: *mut self::fpos_t) -> c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut self::FILE, __pos: *const self::fpos_t) -> c_int;
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
    pub fn perror(__s: *const c_char);
}
extern "C" {
    pub fn __uflow(arg1: *mut self::FILE) -> c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut self::FILE, arg2: c_int) -> c_int;
}
extern "C" {
    pub fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn memmove(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
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
    pub fn strxfrm(__dest: *mut c_char, __src: *const c_char, __n: usize) -> c_ulong;
}
extern "C" {
    pub fn strchr(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strrchr(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strcspn(__s: *const c_char, __reject: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn strspn(__s: *const c_char, __accept: *const c_char) -> c_ulong;
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
    pub fn strlen(__s: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn strerror(__errnum: c_int) -> *mut c_char;
}
extern "C" {
    pub fn __errno_location() -> *mut c_int;
}
pub type wchar_t = c_int;
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
    pub fn strtol(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int) -> c_long;
}
extern "C" {
    pub fn strtoul(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int) -> c_ulong;
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
    pub fn rand() -> c_int;
}
extern "C" {
    pub fn srand(__seed: c_uint);
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
    pub fn free(__ptr: *mut c_void);
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
    pub fn system(__command: *const c_char) -> c_int;
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void) -> c_int>;
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
    pub fn mblen(__s: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn mbtowc(__pwc: *mut self::wchar_t, __s: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut c_char, __wchar: self::wchar_t) -> c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut self::wchar_t, __s: *const c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut c_char, __pwcs: *const self::wchar_t, __n: usize) -> usize;
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
pub type __gwchar_t = c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct imaxdiv_t {
    pub quot: c_long,
    pub rem: c_long,
}
extern "C" {
    pub fn imaxabs(__n: self::intmax_t) -> self::intmax_t;
}
extern "C" {
    pub fn imaxdiv(__numer: self::intmax_t, __denom: self::intmax_t) -> self::imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
    ) -> self::intmax_t;
}
extern "C" {
    pub fn strtoumax(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
    ) -> self::uintmax_t;
}
extern "C" {
    pub fn wcstoimax(
        __nptr: *const self::__gwchar_t,
        __endptr: *mut *mut self::__gwchar_t,
        __base: c_int,
    ) -> self::intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        __nptr: *const self::__gwchar_t,
        __endptr: *mut *mut self::__gwchar_t,
        __base: c_int,
    ) -> self::uintmax_t;
}
extern "C" {
    pub fn __strtol_internal(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
        __group: c_int,
    ) -> c_long;
}
extern "C" {
    pub fn __strtoul_internal(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
        __group: c_int,
    ) -> c_ulong;
}
extern "C" {
    pub fn __wcstol_internal(
        __nptr: *const self::__gwchar_t,
        __endptr: *mut *mut self::__gwchar_t,
        __base: c_int,
        __group: c_int,
    ) -> c_long;
}
extern "C" {
    pub fn __wcstoul_internal(
        __nptr: *const self::__gwchar_t,
        __endptr: *mut *mut self::__gwchar_t,
        __base: c_int,
        __group: c_int,
    ) -> c_ulong;
}
pub type flex_int8_t = i8;
pub type flex_uint8_t = u8;
pub type flex_int16_t = i16;
pub type flex_uint16_t = u16;
pub type flex_int32_t = i32;
pub type flex_uint32_t = u32;
pub type YY_BUFFER_STATE = *mut self::yy_buffer_state;
pub type yy_size_t = usize;
extern "C" {
    pub static mut yyleng: c_int;
}
extern "C" {
    pub static mut yyin: *mut self::FILE;
}
extern "C" {
    pub static mut yyout: *mut self::FILE;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut self::FILE,
    pub yy_ch_buf: *mut c_char,
    pub yy_buf_pos: *mut c_char,
    pub yy_buf_size: c_int,
    pub yy_n_chars: c_int,
    pub yy_is_our_buffer: c_int,
    pub yy_is_interactive: c_int,
    pub yy_at_bol: c_int,
    #[doc = "< The line count."]
    pub yy_bs_lineno: c_int,
    #[doc = "< The column count."]
    pub yy_bs_column: c_int,
    pub yy_fill_buffer: c_int,
    pub yy_buffer_status: c_int,
}
impl Default for yy_buffer_state {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
pub const yy_buffer_stack_top: usize = 0;
pub const yy_buffer_stack_max: usize = 0;
extern "C" {
    pub static mut yy_buffer_stack: *mut self::YY_BUFFER_STATE;
}
extern "C" {
    pub static mut yy_hold_char: c_char;
}
extern "C" {
    pub static mut yy_n_chars: c_int;
}
extern "C" {
    pub static mut yy_c_buf_p: *mut c_char;
}
pub const yy_init: c_int = 0;
pub const yy_start: c_int = 0;
extern "C" {
    pub static mut yy_did_buffer_switch_on_eof: c_int;
}
extern "C" {
    #[doc = " Immediately switch to a different input stream."]
    #[doc = " @param input_file A readable stream."]
    #[doc = ""]
    #[doc = " @note This function does not reset the start condition to @c INITIAL ."]
    pub fn yyrestart(input_file: *mut self::FILE);
}
extern "C" {
    #[doc = " Switch to a different input buffer."]
    #[doc = " @param new_buffer The new input buffer."]
    #[doc = ""]
    pub fn yy_switch_to_buffer(new_buffer: self::YY_BUFFER_STATE);
}
extern "C" {
    #[doc = " Allocate and initialize an input buffer state."]
    #[doc = " @param file A readable stream."]
    #[doc = " @param size The character buffer size in bytes. When in doubt, use @c YY_BUF_SIZE."]
    #[doc = ""]
    #[doc = " @return the allocated buffer state."]
    pub fn yy_create_buffer(file: *mut self::FILE, size: c_int) -> self::YY_BUFFER_STATE;
}
extern "C" {
    #[doc = " Destroy the buffer."]
    #[doc = " @param b a buffer created with yy_create_buffer()"]
    #[doc = ""]
    pub fn yy_delete_buffer(b: self::YY_BUFFER_STATE);
}
extern "C" {
    #[doc = " Discard all buffered characters. On the next scan, YY_INPUT will be called."]
    #[doc = " @param b the buffer state to be flushed, usually @c YY_CURRENT_BUFFER."]
    #[doc = ""]
    pub fn yy_flush_buffer(b: self::YY_BUFFER_STATE);
}
extern "C" {
    #[doc = " Pushes the new state onto the stack. The new state becomes"]
    #[doc = "  the current state. This function will allocate the stack"]
    #[doc = "  if necessary."]
    #[doc = "  @param new_buffer The new state."]
    #[doc = ""]
    pub fn yypush_buffer_state(new_buffer: self::YY_BUFFER_STATE);
}
extern "C" {
    #[doc = " Removes and deletes the top of the stack, if present."]
    #[doc = "  The next element becomes the new top."]
    #[doc = ""]
    pub fn yypop_buffer_state();
}
extern "C" {
    #[doc = " Setup the input buffer state to scan directly from a user-specified character buffer."]
    #[doc = " @param base the character buffer"]
    #[doc = " @param size the size in bytes of the character buffer"]
    #[doc = ""]
    #[doc = " @return the newly allocated buffer state object."]
    pub fn yy_scan_buffer(base: *mut c_char, size: self::yy_size_t) -> self::YY_BUFFER_STATE;
}
extern "C" {
    #[doc = " Setup the input buffer state to scan a string. The next call to yylex() will"]
    #[doc = " scan from a @e copy of @a str."]
    #[doc = " @param yystr a NUL-terminated string to scan"]
    #[doc = ""]
    #[doc = " @return the newly allocated buffer state object."]
    #[doc = " @note If you want to scan bytes that may contain NUL values, then use"]
    #[doc = "       yy_scan_bytes() instead."]
    pub fn yy_scan_string(yy_str: *const c_char) -> self::YY_BUFFER_STATE;
}
extern "C" {
    #[doc = " Setup the input buffer state to scan the given bytes. The next call to yylex() will"]
    #[doc = " scan from a @e copy of @a bytes."]
    #[doc = " @param yybytes the byte buffer to scan"]
    #[doc = " @param _yybytes_len the number of bytes in the buffer pointed to by @a bytes."]
    #[doc = ""]
    #[doc = " @return the newly allocated buffer state object."]
    pub fn yy_scan_bytes(bytes: *const c_char, len: c_int) -> self::YY_BUFFER_STATE;
}
extern "C" {
    pub fn yyalloc(arg1: self::yy_size_t) -> *mut c_void;
}
extern "C" {
    pub fn yyrealloc(arg1: *mut c_void, arg2: self::yy_size_t) -> *mut c_void;
}
extern "C" {
    pub fn yyfree(arg1: *mut c_void);
}
pub type YY_CHAR = self::flex_uint8_t;
pub type yy_state_type = c_int;
extern "C" {
    pub static mut yylineno: c_int;
}
extern "C" {
    pub static mut yytext: *mut c_char;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct yy_trans_info {
    pub yy_verify: self::flex_int32_t,
    pub yy_nxt: self::flex_int32_t,
}
extern "C" {
    pub static mut yy_accept: [self::flex_int16_t; 243usize];
}
extern "C" {
    pub static mut yy_ec: [self::YY_CHAR; 256usize];
}
extern "C" {
    pub static mut yy_meta: [self::YY_CHAR; 69usize];
}
extern "C" {
    pub static mut yy_base: [self::flex_int16_t; 248usize];
}
extern "C" {
    pub static mut yy_def: [self::flex_int16_t; 248usize];
}
extern "C" {
    pub static mut yy_nxt: [self::flex_int16_t; 396usize];
}
extern "C" {
    pub static mut yy_chk: [self::flex_int16_t; 396usize];
}
extern "C" {
    pub static mut yy_last_accepting_state: self::yy_state_type;
}
extern "C" {
    pub static mut yy_last_accepting_cpos: *mut c_char;
}
extern "C" {
    pub static mut yy_flex_debug: c_int;
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
extern "C" {}
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
pub type va_list = self::__gnuc_va_list;
extern "C" {
    pub static mut EM_anyErrors: c_ulong;
}
extern "C" {
    pub static mut EM_tokPos: c_int;
}
extern "C" {
    pub static mut EM_lineNum: c_int;
}
extern "C" {
    pub static mut EM_extLineNum: c_int;
}
extern "C" {
    pub fn EM_newline();
}
extern "C" {
    pub fn ck_fprintf_stdout(format: *const c_char, ...);
}
extern "C" {
    pub fn ck_fprintf_stderr(format: *const c_char, ...);
}
extern "C" {
    pub fn ck_fflush_stdout();
}
extern "C" {
    pub fn ck_fflush_stderr();
}
extern "C" {
    pub fn ck_vfprintf_stdout(format: *const c_char, args: *mut self::__va_list_tag);
}
extern "C" {
    pub fn ck_vfprintf_stderr(format: *const c_char, args: *mut self::__va_list_tag);
}
extern "C" {
    pub fn ck_set_stdout_callback(callback: Option<unsafe extern "C" fn(arg1: *const c_char)>);
}
extern "C" {
    pub fn ck_set_stderr_callback(callback: Option<unsafe extern "C" fn(arg1: *const c_char)>);
}
extern "C" {
    pub fn EM_log(arg1: c_long, arg2: self::c_constr, ...);
}
extern "C" {
    pub fn EM_setlog(arg1: c_long);
}
extern "C" {
    pub fn EM_pushlog();
}
extern "C" {
    pub fn EM_poplog();
}
extern "C" {
    pub static mut g_loglevel: c_long;
}
extern "C" {
    pub fn EM_error(arg1: c_int, arg2: self::c_constr, ...);
}
extern "C" {
    pub fn EM_error2(arg1: c_int, arg2: self::c_constr, ...);
}
extern "C" {
    pub fn EM_error2b(arg1: c_int, arg2: self::c_constr, ...);
}
extern "C" {
    pub fn EM_error3(arg1: self::c_constr, ...);
}
extern "C" {
    pub fn EM_impossible(arg1: self::c_constr, ...);
}
extern "C" {
    pub fn EM_reset(filename: self::c_constr, fd: *mut self::FILE) -> c_ulong;
}
extern "C" {
    pub fn EM_change_file(filename: self::c_constr);
}
extern "C" {
    pub fn EM_lasterror() -> *const c_char;
}
extern "C" {
    pub fn EM_reset_msg();
}
extern "C" {
    pub fn mini(str: *const c_char) -> *const c_char;
}
extern "C" {
    pub fn mini_type(str: *const c_char) -> *const c_char;
}
pub const yytokentype_ID: self::yytokentype = 258;
pub const yytokentype_STRING_LIT: self::yytokentype = 259;
pub const yytokentype_CHAR_LIT: self::yytokentype = 260;
pub const yytokentype_NUM: self::yytokentype = 261;
pub const yytokentype_FLOAT: self::yytokentype = 262;
pub const yytokentype_POUND: self::yytokentype = 263;
pub const yytokentype_COMMA: self::yytokentype = 264;
pub const yytokentype_COLON: self::yytokentype = 265;
pub const yytokentype_SEMICOLON: self::yytokentype = 266;
pub const yytokentype_LPAREN: self::yytokentype = 267;
pub const yytokentype_RPAREN: self::yytokentype = 268;
pub const yytokentype_LBRACK: self::yytokentype = 269;
pub const yytokentype_RBRACK: self::yytokentype = 270;
pub const yytokentype_LBRACE: self::yytokentype = 271;
pub const yytokentype_RBRACE: self::yytokentype = 272;
pub const yytokentype_DOT: self::yytokentype = 273;
pub const yytokentype_PLUS: self::yytokentype = 274;
pub const yytokentype_MINUS: self::yytokentype = 275;
pub const yytokentype_TIMES: self::yytokentype = 276;
pub const yytokentype_DIVIDE: self::yytokentype = 277;
pub const yytokentype_PERCENT: self::yytokentype = 278;
pub const yytokentype_EQ: self::yytokentype = 279;
pub const yytokentype_NEQ: self::yytokentype = 280;
pub const yytokentype_LT: self::yytokentype = 281;
pub const yytokentype_LE: self::yytokentype = 282;
pub const yytokentype_GT: self::yytokentype = 283;
pub const yytokentype_GE: self::yytokentype = 284;
pub const yytokentype_AND: self::yytokentype = 285;
pub const yytokentype_OR: self::yytokentype = 286;
pub const yytokentype_ASSIGN: self::yytokentype = 287;
pub const yytokentype_IF: self::yytokentype = 288;
pub const yytokentype_THEN: self::yytokentype = 289;
pub const yytokentype_ELSE: self::yytokentype = 290;
pub const yytokentype_WHILE: self::yytokentype = 291;
pub const yytokentype_FOR: self::yytokentype = 292;
pub const yytokentype_DO: self::yytokentype = 293;
pub const yytokentype_LOOP: self::yytokentype = 294;
pub const yytokentype_BREAK: self::yytokentype = 295;
pub const yytokentype_CONTINUE: self::yytokentype = 296;
pub const yytokentype_NULL_TOK: self::yytokentype = 297;
pub const yytokentype_FUNCTION: self::yytokentype = 298;
pub const yytokentype_RETURN: self::yytokentype = 299;
pub const yytokentype_QUESTION: self::yytokentype = 300;
pub const yytokentype_EXCLAMATION: self::yytokentype = 301;
pub const yytokentype_S_OR: self::yytokentype = 302;
pub const yytokentype_S_AND: self::yytokentype = 303;
pub const yytokentype_S_XOR: self::yytokentype = 304;
pub const yytokentype_PLUSPLUS: self::yytokentype = 305;
pub const yytokentype_MINUSMINUS: self::yytokentype = 306;
pub const yytokentype_DOLLAR: self::yytokentype = 307;
pub const yytokentype_POUNDPAREN: self::yytokentype = 308;
pub const yytokentype_PERCENTPAREN: self::yytokentype = 309;
pub const yytokentype_ATPAREN: self::yytokentype = 310;
pub const yytokentype_SIMULT: self::yytokentype = 311;
pub const yytokentype_PATTERN: self::yytokentype = 312;
pub const yytokentype_CODE: self::yytokentype = 313;
pub const yytokentype_TRANSPORT: self::yytokentype = 314;
pub const yytokentype_HOST: self::yytokentype = 315;
pub const yytokentype_TIME: self::yytokentype = 316;
pub const yytokentype_WHENEVER: self::yytokentype = 317;
pub const yytokentype_NEXT: self::yytokentype = 318;
pub const yytokentype_UNTIL: self::yytokentype = 319;
pub const yytokentype_EXTERNAL: self::yytokentype = 320;
pub const yytokentype_GLOBAL: self::yytokentype = 321;
pub const yytokentype_EVERY: self::yytokentype = 322;
pub const yytokentype_BEFORE: self::yytokentype = 323;
pub const yytokentype_AFTER: self::yytokentype = 324;
pub const yytokentype_AT: self::yytokentype = 325;
pub const yytokentype_AT_SYM: self::yytokentype = 326;
pub const yytokentype_ATAT_SYM: self::yytokentype = 327;
pub const yytokentype_NEW: self::yytokentype = 328;
pub const yytokentype_SIZEOF: self::yytokentype = 329;
pub const yytokentype_TYPEOF: self::yytokentype = 330;
pub const yytokentype_SAME: self::yytokentype = 331;
pub const yytokentype_PLUS_CHUCK: self::yytokentype = 332;
pub const yytokentype_MINUS_CHUCK: self::yytokentype = 333;
pub const yytokentype_TIMES_CHUCK: self::yytokentype = 334;
pub const yytokentype_DIVIDE_CHUCK: self::yytokentype = 335;
pub const yytokentype_S_AND_CHUCK: self::yytokentype = 336;
pub const yytokentype_S_OR_CHUCK: self::yytokentype = 337;
pub const yytokentype_S_XOR_CHUCK: self::yytokentype = 338;
pub const yytokentype_SHIFT_RIGHT_CHUCK: self::yytokentype = 339;
pub const yytokentype_SHIFT_LEFT_CHUCK: self::yytokentype = 340;
pub const yytokentype_PERCENT_CHUCK: self::yytokentype = 341;
pub const yytokentype_SHIFT_RIGHT: self::yytokentype = 342;
pub const yytokentype_SHIFT_LEFT: self::yytokentype = 343;
pub const yytokentype_TILDA: self::yytokentype = 344;
pub const yytokentype_CHUCK: self::yytokentype = 345;
pub const yytokentype_COLONCOLON: self::yytokentype = 346;
pub const yytokentype_S_CHUCK: self::yytokentype = 347;
pub const yytokentype_AT_CHUCK: self::yytokentype = 348;
pub const yytokentype_LEFT_S_CHUCK: self::yytokentype = 349;
pub const yytokentype_UNCHUCK: self::yytokentype = 350;
pub const yytokentype_UPCHUCK: self::yytokentype = 351;
pub const yytokentype_CLASS: self::yytokentype = 352;
pub const yytokentype_INTERFACE: self::yytokentype = 353;
pub const yytokentype_EXTENDS: self::yytokentype = 354;
pub const yytokentype_IMPLEMENTS: self::yytokentype = 355;
pub const yytokentype_PUBLIC: self::yytokentype = 356;
pub const yytokentype_PROTECTED: self::yytokentype = 357;
pub const yytokentype_PRIVATE: self::yytokentype = 358;
pub const yytokentype_STATIC: self::yytokentype = 359;
pub const yytokentype_ABSTRACT: self::yytokentype = 360;
pub const yytokentype_CONST: self::yytokentype = 361;
pub const yytokentype_SPORK: self::yytokentype = 362;
pub const yytokentype_ARROW_RIGHT: self::yytokentype = 363;
pub const yytokentype_ARROW_LEFT: self::yytokentype = 364;
pub const yytokentype_L_HACK: self::yytokentype = 365;
pub const yytokentype_R_HACK: self::yytokentype = 366;
pub type yytokentype = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub union YYSTYPE {
    pub pos: c_int,
    pub ival: c_int,
    pub fval: f64,
    pub sval: self::c_str,
    pub program: self::a_Program,
    pub program_section: self::a_Section,
    pub stmt_list: self::a_Stmt_List,
    pub class_def: self::a_Class_Def,
    pub class_ext: self::a_Class_Ext,
    pub class_body: self::a_Class_Body,
    pub stmt: self::a_Stmt,
    pub exp: self::a_Exp,
    pub func_def: self::a_Func_Def,
    pub var_decl_list: self::a_Var_Decl_List,
    pub var_decl: self::a_Var_Decl,
    pub type_decl: self::a_Type_Decl,
    pub arg_list: self::a_Arg_List,
    pub id_list: self::a_Id_List,
    pub array_sub: self::a_Array_Sub,
    pub complex_exp: self::a_Complex,
    pub polar_exp: self::a_Polar,
    pub vec_exp: self::a_Vec,
    _bindgen_union_align: u64,
}
impl Default for YYSTYPE {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for YYSTYPE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "YYSTYPE {{ union }}")
    }
}
extern "C" {
    pub static mut yylval: self::YYSTYPE;
}
extern "C" {
    pub fn yyparse() -> c_int;
}
pub const char_pos: c_int = 1;
extern "C" {
    pub fn yywrap() -> c_int;
}
extern "C" {
    pub fn adjust();
}
extern "C" {
    pub fn strip_lit(str: self::c_str) -> self::c_str;
}
extern "C" {
    pub fn alloc_str(str: self::c_str) -> self::c_str;
}
extern "C" {
    pub fn htol(str: self::c_str) -> c_long;
}
extern "C" {
    pub fn comment() -> c_int;
}
extern "C" {
    pub fn block_comment() -> c_int;
}
extern "C" {
    pub fn access(__name: *const c_char, __type: c_int) -> c_int;
}
extern "C" {
    pub fn lseek(__fd: c_int, __offset: self::__off_t, __whence: c_int) -> self::__off_t;
}
extern "C" {
    pub fn close(__fd: c_int) -> c_int;
}
extern "C" {
    pub fn read(__fd: c_int, __buf: *mut c_void, __nbytes: usize) -> isize;
}
extern "C" {
    pub fn write(__fd: c_int, __buf: *const c_void, __n: usize) -> isize;
}
extern "C" {
    pub fn pipe(__pipedes: *mut c_int) -> c_int;
}
extern "C" {
    pub fn alarm(__seconds: c_uint) -> c_uint;
}
extern "C" {
    pub fn sleep(__seconds: c_uint) -> c_uint;
}
extern "C" {
    pub fn pause() -> c_int;
}
extern "C" {
    pub fn chown(__file: *const c_char, __owner: self::__uid_t, __group: self::__gid_t) -> c_int;
}
extern "C" {
    pub fn chdir(__path: *const c_char) -> c_int;
}
extern "C" {
    pub fn getcwd(__buf: *mut c_char, __size: usize) -> *mut c_char;
}
extern "C" {
    pub fn dup(__fd: c_int) -> c_int;
}
extern "C" {
    pub fn dup2(__fd: c_int, __fd2: c_int) -> c_int;
}
extern "C" {
    pub static mut __environ: *mut *mut c_char;
}
extern "C" {
    pub fn execve(
        __path: *const c_char,
        __argv: *const *mut c_char,
        __envp: *const *mut c_char,
    ) -> c_int;
}
extern "C" {
    pub fn execv(__path: *const c_char, __argv: *const *mut c_char) -> c_int;
}
extern "C" {
    pub fn execle(__path: *const c_char, __arg: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn execl(__path: *const c_char, __arg: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn execvp(__file: *const c_char, __argv: *const *mut c_char) -> c_int;
}
extern "C" {
    pub fn execlp(__file: *const c_char, __arg: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn _exit(__status: c_int);
}
pub const _PC_LINK_MAX: self::_bindgen_ty_1 = 0;
pub const _PC_MAX_CANON: self::_bindgen_ty_1 = 1;
pub const _PC_MAX_INPUT: self::_bindgen_ty_1 = 2;
pub const _PC_NAME_MAX: self::_bindgen_ty_1 = 3;
pub const _PC_PATH_MAX: self::_bindgen_ty_1 = 4;
pub const _PC_PIPE_BUF: self::_bindgen_ty_1 = 5;
pub const _PC_CHOWN_RESTRICTED: self::_bindgen_ty_1 = 6;
pub const _PC_NO_TRUNC: self::_bindgen_ty_1 = 7;
pub const _PC_VDISABLE: self::_bindgen_ty_1 = 8;
pub const _PC_SYNC_IO: self::_bindgen_ty_1 = 9;
pub const _PC_ASYNC_IO: self::_bindgen_ty_1 = 10;
pub const _PC_PRIO_IO: self::_bindgen_ty_1 = 11;
pub const _PC_SOCK_MAXBUF: self::_bindgen_ty_1 = 12;
pub const _PC_FILESIZEBITS: self::_bindgen_ty_1 = 13;
pub const _PC_REC_INCR_XFER_SIZE: self::_bindgen_ty_1 = 14;
pub const _PC_REC_MAX_XFER_SIZE: self::_bindgen_ty_1 = 15;
pub const _PC_REC_MIN_XFER_SIZE: self::_bindgen_ty_1 = 16;
pub const _PC_REC_XFER_ALIGN: self::_bindgen_ty_1 = 17;
pub const _PC_ALLOC_SIZE_MIN: self::_bindgen_ty_1 = 18;
pub const _PC_SYMLINK_MAX: self::_bindgen_ty_1 = 19;
pub const _PC_2_SYMLINKS: self::_bindgen_ty_1 = 20;
pub type _bindgen_ty_1 = u32;
pub const _SC_ARG_MAX: self::_bindgen_ty_2 = 0;
pub const _SC_CHILD_MAX: self::_bindgen_ty_2 = 1;
pub const _SC_CLK_TCK: self::_bindgen_ty_2 = 2;
pub const _SC_NGROUPS_MAX: self::_bindgen_ty_2 = 3;
pub const _SC_OPEN_MAX: self::_bindgen_ty_2 = 4;
pub const _SC_STREAM_MAX: self::_bindgen_ty_2 = 5;
pub const _SC_TZNAME_MAX: self::_bindgen_ty_2 = 6;
pub const _SC_JOB_CONTROL: self::_bindgen_ty_2 = 7;
pub const _SC_SAVED_IDS: self::_bindgen_ty_2 = 8;
pub const _SC_REALTIME_SIGNALS: self::_bindgen_ty_2 = 9;
pub const _SC_PRIORITY_SCHEDULING: self::_bindgen_ty_2 = 10;
pub const _SC_TIMERS: self::_bindgen_ty_2 = 11;
pub const _SC_ASYNCHRONOUS_IO: self::_bindgen_ty_2 = 12;
pub const _SC_PRIORITIZED_IO: self::_bindgen_ty_2 = 13;
pub const _SC_SYNCHRONIZED_IO: self::_bindgen_ty_2 = 14;
pub const _SC_FSYNC: self::_bindgen_ty_2 = 15;
pub const _SC_MAPPED_FILES: self::_bindgen_ty_2 = 16;
pub const _SC_MEMLOCK: self::_bindgen_ty_2 = 17;
pub const _SC_MEMLOCK_RANGE: self::_bindgen_ty_2 = 18;
pub const _SC_MEMORY_PROTECTION: self::_bindgen_ty_2 = 19;
pub const _SC_MESSAGE_PASSING: self::_bindgen_ty_2 = 20;
pub const _SC_SEMAPHORES: self::_bindgen_ty_2 = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: self::_bindgen_ty_2 = 22;
pub const _SC_AIO_LISTIO_MAX: self::_bindgen_ty_2 = 23;
pub const _SC_AIO_MAX: self::_bindgen_ty_2 = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: self::_bindgen_ty_2 = 25;
pub const _SC_DELAYTIMER_MAX: self::_bindgen_ty_2 = 26;
pub const _SC_MQ_OPEN_MAX: self::_bindgen_ty_2 = 27;
pub const _SC_MQ_PRIO_MAX: self::_bindgen_ty_2 = 28;
pub const _SC_VERSION: self::_bindgen_ty_2 = 29;
pub const _SC_PAGESIZE: self::_bindgen_ty_2 = 30;
pub const _SC_RTSIG_MAX: self::_bindgen_ty_2 = 31;
pub const _SC_SEM_NSEMS_MAX: self::_bindgen_ty_2 = 32;
pub const _SC_SEM_VALUE_MAX: self::_bindgen_ty_2 = 33;
pub const _SC_SIGQUEUE_MAX: self::_bindgen_ty_2 = 34;
pub const _SC_TIMER_MAX: self::_bindgen_ty_2 = 35;
pub const _SC_BC_BASE_MAX: self::_bindgen_ty_2 = 36;
pub const _SC_BC_DIM_MAX: self::_bindgen_ty_2 = 37;
pub const _SC_BC_SCALE_MAX: self::_bindgen_ty_2 = 38;
pub const _SC_BC_STRING_MAX: self::_bindgen_ty_2 = 39;
pub const _SC_COLL_WEIGHTS_MAX: self::_bindgen_ty_2 = 40;
pub const _SC_EQUIV_CLASS_MAX: self::_bindgen_ty_2 = 41;
pub const _SC_EXPR_NEST_MAX: self::_bindgen_ty_2 = 42;
pub const _SC_LINE_MAX: self::_bindgen_ty_2 = 43;
pub const _SC_RE_DUP_MAX: self::_bindgen_ty_2 = 44;
pub const _SC_CHARCLASS_NAME_MAX: self::_bindgen_ty_2 = 45;
pub const _SC_2_VERSION: self::_bindgen_ty_2 = 46;
pub const _SC_2_C_BIND: self::_bindgen_ty_2 = 47;
pub const _SC_2_C_DEV: self::_bindgen_ty_2 = 48;
pub const _SC_2_FORT_DEV: self::_bindgen_ty_2 = 49;
pub const _SC_2_FORT_RUN: self::_bindgen_ty_2 = 50;
pub const _SC_2_SW_DEV: self::_bindgen_ty_2 = 51;
pub const _SC_2_LOCALEDEF: self::_bindgen_ty_2 = 52;
pub const _SC_PII: self::_bindgen_ty_2 = 53;
pub const _SC_PII_XTI: self::_bindgen_ty_2 = 54;
pub const _SC_PII_SOCKET: self::_bindgen_ty_2 = 55;
pub const _SC_PII_INTERNET: self::_bindgen_ty_2 = 56;
pub const _SC_PII_OSI: self::_bindgen_ty_2 = 57;
pub const _SC_POLL: self::_bindgen_ty_2 = 58;
pub const _SC_SELECT: self::_bindgen_ty_2 = 59;
pub const _SC_UIO_MAXIOV: self::_bindgen_ty_2 = 60;
pub const _SC_IOV_MAX: self::_bindgen_ty_2 = 60;
pub const _SC_PII_INTERNET_STREAM: self::_bindgen_ty_2 = 61;
pub const _SC_PII_INTERNET_DGRAM: self::_bindgen_ty_2 = 62;
pub const _SC_PII_OSI_COTS: self::_bindgen_ty_2 = 63;
pub const _SC_PII_OSI_CLTS: self::_bindgen_ty_2 = 64;
pub const _SC_PII_OSI_M: self::_bindgen_ty_2 = 65;
pub const _SC_T_IOV_MAX: self::_bindgen_ty_2 = 66;
pub const _SC_THREADS: self::_bindgen_ty_2 = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: self::_bindgen_ty_2 = 68;
pub const _SC_GETGR_R_SIZE_MAX: self::_bindgen_ty_2 = 69;
pub const _SC_GETPW_R_SIZE_MAX: self::_bindgen_ty_2 = 70;
pub const _SC_LOGIN_NAME_MAX: self::_bindgen_ty_2 = 71;
pub const _SC_TTY_NAME_MAX: self::_bindgen_ty_2 = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: self::_bindgen_ty_2 = 73;
pub const _SC_THREAD_KEYS_MAX: self::_bindgen_ty_2 = 74;
pub const _SC_THREAD_STACK_MIN: self::_bindgen_ty_2 = 75;
pub const _SC_THREAD_THREADS_MAX: self::_bindgen_ty_2 = 76;
pub const _SC_THREAD_ATTR_STACKADDR: self::_bindgen_ty_2 = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: self::_bindgen_ty_2 = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: self::_bindgen_ty_2 = 79;
pub const _SC_THREAD_PRIO_INHERIT: self::_bindgen_ty_2 = 80;
pub const _SC_THREAD_PRIO_PROTECT: self::_bindgen_ty_2 = 81;
pub const _SC_THREAD_PROCESS_SHARED: self::_bindgen_ty_2 = 82;
pub const _SC_NPROCESSORS_CONF: self::_bindgen_ty_2 = 83;
pub const _SC_NPROCESSORS_ONLN: self::_bindgen_ty_2 = 84;
pub const _SC_PHYS_PAGES: self::_bindgen_ty_2 = 85;
pub const _SC_AVPHYS_PAGES: self::_bindgen_ty_2 = 86;
pub const _SC_ATEXIT_MAX: self::_bindgen_ty_2 = 87;
pub const _SC_PASS_MAX: self::_bindgen_ty_2 = 88;
pub const _SC_XOPEN_VERSION: self::_bindgen_ty_2 = 89;
pub const _SC_XOPEN_XCU_VERSION: self::_bindgen_ty_2 = 90;
pub const _SC_XOPEN_UNIX: self::_bindgen_ty_2 = 91;
pub const _SC_XOPEN_CRYPT: self::_bindgen_ty_2 = 92;
pub const _SC_XOPEN_ENH_I18N: self::_bindgen_ty_2 = 93;
pub const _SC_XOPEN_SHM: self::_bindgen_ty_2 = 94;
pub const _SC_2_CHAR_TERM: self::_bindgen_ty_2 = 95;
pub const _SC_2_C_VERSION: self::_bindgen_ty_2 = 96;
pub const _SC_2_UPE: self::_bindgen_ty_2 = 97;
pub const _SC_XOPEN_XPG2: self::_bindgen_ty_2 = 98;
pub const _SC_XOPEN_XPG3: self::_bindgen_ty_2 = 99;
pub const _SC_XOPEN_XPG4: self::_bindgen_ty_2 = 100;
pub const _SC_CHAR_BIT: self::_bindgen_ty_2 = 101;
pub const _SC_CHAR_MAX: self::_bindgen_ty_2 = 102;
pub const _SC_CHAR_MIN: self::_bindgen_ty_2 = 103;
pub const _SC_INT_MAX: self::_bindgen_ty_2 = 104;
pub const _SC_INT_MIN: self::_bindgen_ty_2 = 105;
pub const _SC_LONG_BIT: self::_bindgen_ty_2 = 106;
pub const _SC_WORD_BIT: self::_bindgen_ty_2 = 107;
pub const _SC_MB_LEN_MAX: self::_bindgen_ty_2 = 108;
pub const _SC_NZERO: self::_bindgen_ty_2 = 109;
pub const _SC_SSIZE_MAX: self::_bindgen_ty_2 = 110;
pub const _SC_SCHAR_MAX: self::_bindgen_ty_2 = 111;
pub const _SC_SCHAR_MIN: self::_bindgen_ty_2 = 112;
pub const _SC_SHRT_MAX: self::_bindgen_ty_2 = 113;
pub const _SC_SHRT_MIN: self::_bindgen_ty_2 = 114;
pub const _SC_UCHAR_MAX: self::_bindgen_ty_2 = 115;
pub const _SC_UINT_MAX: self::_bindgen_ty_2 = 116;
pub const _SC_ULONG_MAX: self::_bindgen_ty_2 = 117;
pub const _SC_USHRT_MAX: self::_bindgen_ty_2 = 118;
pub const _SC_NL_ARGMAX: self::_bindgen_ty_2 = 119;
pub const _SC_NL_LANGMAX: self::_bindgen_ty_2 = 120;
pub const _SC_NL_MSGMAX: self::_bindgen_ty_2 = 121;
pub const _SC_NL_NMAX: self::_bindgen_ty_2 = 122;
pub const _SC_NL_SETMAX: self::_bindgen_ty_2 = 123;
pub const _SC_NL_TEXTMAX: self::_bindgen_ty_2 = 124;
pub const _SC_XBS5_ILP32_OFF32: self::_bindgen_ty_2 = 125;
pub const _SC_XBS5_ILP32_OFFBIG: self::_bindgen_ty_2 = 126;
pub const _SC_XBS5_LP64_OFF64: self::_bindgen_ty_2 = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: self::_bindgen_ty_2 = 128;
pub const _SC_XOPEN_LEGACY: self::_bindgen_ty_2 = 129;
pub const _SC_XOPEN_REALTIME: self::_bindgen_ty_2 = 130;
pub const _SC_XOPEN_REALTIME_THREADS: self::_bindgen_ty_2 = 131;
pub const _SC_ADVISORY_INFO: self::_bindgen_ty_2 = 132;
pub const _SC_BARRIERS: self::_bindgen_ty_2 = 133;
pub const _SC_BASE: self::_bindgen_ty_2 = 134;
pub const _SC_C_LANG_SUPPORT: self::_bindgen_ty_2 = 135;
pub const _SC_C_LANG_SUPPORT_R: self::_bindgen_ty_2 = 136;
pub const _SC_CLOCK_SELECTION: self::_bindgen_ty_2 = 137;
pub const _SC_CPUTIME: self::_bindgen_ty_2 = 138;
pub const _SC_THREAD_CPUTIME: self::_bindgen_ty_2 = 139;
pub const _SC_DEVICE_IO: self::_bindgen_ty_2 = 140;
pub const _SC_DEVICE_SPECIFIC: self::_bindgen_ty_2 = 141;
pub const _SC_DEVICE_SPECIFIC_R: self::_bindgen_ty_2 = 142;
pub const _SC_FD_MGMT: self::_bindgen_ty_2 = 143;
pub const _SC_FIFO: self::_bindgen_ty_2 = 144;
pub const _SC_PIPE: self::_bindgen_ty_2 = 145;
pub const _SC_FILE_ATTRIBUTES: self::_bindgen_ty_2 = 146;
pub const _SC_FILE_LOCKING: self::_bindgen_ty_2 = 147;
pub const _SC_FILE_SYSTEM: self::_bindgen_ty_2 = 148;
pub const _SC_MONOTONIC_CLOCK: self::_bindgen_ty_2 = 149;
pub const _SC_MULTI_PROCESS: self::_bindgen_ty_2 = 150;
pub const _SC_SINGLE_PROCESS: self::_bindgen_ty_2 = 151;
pub const _SC_NETWORKING: self::_bindgen_ty_2 = 152;
pub const _SC_READER_WRITER_LOCKS: self::_bindgen_ty_2 = 153;
pub const _SC_SPIN_LOCKS: self::_bindgen_ty_2 = 154;
pub const _SC_REGEXP: self::_bindgen_ty_2 = 155;
pub const _SC_REGEX_VERSION: self::_bindgen_ty_2 = 156;
pub const _SC_SHELL: self::_bindgen_ty_2 = 157;
pub const _SC_SIGNALS: self::_bindgen_ty_2 = 158;
pub const _SC_SPAWN: self::_bindgen_ty_2 = 159;
pub const _SC_SPORADIC_SERVER: self::_bindgen_ty_2 = 160;
pub const _SC_THREAD_SPORADIC_SERVER: self::_bindgen_ty_2 = 161;
pub const _SC_SYSTEM_DATABASE: self::_bindgen_ty_2 = 162;
pub const _SC_SYSTEM_DATABASE_R: self::_bindgen_ty_2 = 163;
pub const _SC_TIMEOUTS: self::_bindgen_ty_2 = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: self::_bindgen_ty_2 = 165;
pub const _SC_USER_GROUPS: self::_bindgen_ty_2 = 166;
pub const _SC_USER_GROUPS_R: self::_bindgen_ty_2 = 167;
pub const _SC_2_PBS: self::_bindgen_ty_2 = 168;
pub const _SC_2_PBS_ACCOUNTING: self::_bindgen_ty_2 = 169;
pub const _SC_2_PBS_LOCATE: self::_bindgen_ty_2 = 170;
pub const _SC_2_PBS_MESSAGE: self::_bindgen_ty_2 = 171;
pub const _SC_2_PBS_TRACK: self::_bindgen_ty_2 = 172;
pub const _SC_SYMLOOP_MAX: self::_bindgen_ty_2 = 173;
pub const _SC_STREAMS: self::_bindgen_ty_2 = 174;
pub const _SC_2_PBS_CHECKPOINT: self::_bindgen_ty_2 = 175;
pub const _SC_V6_ILP32_OFF32: self::_bindgen_ty_2 = 176;
pub const _SC_V6_ILP32_OFFBIG: self::_bindgen_ty_2 = 177;
pub const _SC_V6_LP64_OFF64: self::_bindgen_ty_2 = 178;
pub const _SC_V6_LPBIG_OFFBIG: self::_bindgen_ty_2 = 179;
pub const _SC_HOST_NAME_MAX: self::_bindgen_ty_2 = 180;
pub const _SC_TRACE: self::_bindgen_ty_2 = 181;
pub const _SC_TRACE_EVENT_FILTER: self::_bindgen_ty_2 = 182;
pub const _SC_TRACE_INHERIT: self::_bindgen_ty_2 = 183;
pub const _SC_TRACE_LOG: self::_bindgen_ty_2 = 184;
pub const _SC_LEVEL1_ICACHE_SIZE: self::_bindgen_ty_2 = 185;
pub const _SC_LEVEL1_ICACHE_ASSOC: self::_bindgen_ty_2 = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: self::_bindgen_ty_2 = 187;
pub const _SC_LEVEL1_DCACHE_SIZE: self::_bindgen_ty_2 = 188;
pub const _SC_LEVEL1_DCACHE_ASSOC: self::_bindgen_ty_2 = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: self::_bindgen_ty_2 = 190;
pub const _SC_LEVEL2_CACHE_SIZE: self::_bindgen_ty_2 = 191;
pub const _SC_LEVEL2_CACHE_ASSOC: self::_bindgen_ty_2 = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: self::_bindgen_ty_2 = 193;
pub const _SC_LEVEL3_CACHE_SIZE: self::_bindgen_ty_2 = 194;
pub const _SC_LEVEL3_CACHE_ASSOC: self::_bindgen_ty_2 = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: self::_bindgen_ty_2 = 196;
pub const _SC_LEVEL4_CACHE_SIZE: self::_bindgen_ty_2 = 197;
pub const _SC_LEVEL4_CACHE_ASSOC: self::_bindgen_ty_2 = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: self::_bindgen_ty_2 = 199;
pub const _SC_IPV6: self::_bindgen_ty_2 = 235;
pub const _SC_RAW_SOCKETS: self::_bindgen_ty_2 = 236;
pub const _SC_V7_ILP32_OFF32: self::_bindgen_ty_2 = 237;
pub const _SC_V7_ILP32_OFFBIG: self::_bindgen_ty_2 = 238;
pub const _SC_V7_LP64_OFF64: self::_bindgen_ty_2 = 239;
pub const _SC_V7_LPBIG_OFFBIG: self::_bindgen_ty_2 = 240;
pub const _SC_SS_REPL_MAX: self::_bindgen_ty_2 = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: self::_bindgen_ty_2 = 242;
pub const _SC_TRACE_NAME_MAX: self::_bindgen_ty_2 = 243;
pub const _SC_TRACE_SYS_MAX: self::_bindgen_ty_2 = 244;
pub const _SC_TRACE_USER_EVENT_MAX: self::_bindgen_ty_2 = 245;
pub const _SC_XOPEN_STREAMS: self::_bindgen_ty_2 = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: self::_bindgen_ty_2 = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: self::_bindgen_ty_2 = 248;
pub type _bindgen_ty_2 = u32;
pub const _CS_PATH: self::_bindgen_ty_3 = 0;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: self::_bindgen_ty_3 = 1;
pub const _CS_GNU_LIBC_VERSION: self::_bindgen_ty_3 = 2;
pub const _CS_GNU_LIBPTHREAD_VERSION: self::_bindgen_ty_3 = 3;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: self::_bindgen_ty_3 = 4;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: self::_bindgen_ty_3 = 5;
pub const _CS_LFS_CFLAGS: self::_bindgen_ty_3 = 1000;
pub const _CS_LFS_LDFLAGS: self::_bindgen_ty_3 = 1001;
pub const _CS_LFS_LIBS: self::_bindgen_ty_3 = 1002;
pub const _CS_LFS_LINTFLAGS: self::_bindgen_ty_3 = 1003;
pub const _CS_LFS64_CFLAGS: self::_bindgen_ty_3 = 1004;
pub const _CS_LFS64_LDFLAGS: self::_bindgen_ty_3 = 1005;
pub const _CS_LFS64_LIBS: self::_bindgen_ty_3 = 1006;
pub const _CS_LFS64_LINTFLAGS: self::_bindgen_ty_3 = 1007;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: self::_bindgen_ty_3 = 1100;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: self::_bindgen_ty_3 = 1101;
pub const _CS_XBS5_ILP32_OFF32_LIBS: self::_bindgen_ty_3 = 1102;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: self::_bindgen_ty_3 = 1103;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: self::_bindgen_ty_3 = 1104;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: self::_bindgen_ty_3 = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: self::_bindgen_ty_3 = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: self::_bindgen_ty_3 = 1107;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: self::_bindgen_ty_3 = 1108;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: self::_bindgen_ty_3 = 1109;
pub const _CS_XBS5_LP64_OFF64_LIBS: self::_bindgen_ty_3 = 1110;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: self::_bindgen_ty_3 = 1111;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: self::_bindgen_ty_3 = 1112;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: self::_bindgen_ty_3 = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: self::_bindgen_ty_3 = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: self::_bindgen_ty_3 = 1115;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: self::_bindgen_ty_3 = 1116;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: self::_bindgen_ty_3 = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: self::_bindgen_ty_3 = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: self::_bindgen_ty_3 = 1119;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: self::_bindgen_ty_3 = 1120;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: self::_bindgen_ty_3 = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: self::_bindgen_ty_3 = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: self::_bindgen_ty_3 = 1123;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: self::_bindgen_ty_3 = 1124;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: self::_bindgen_ty_3 = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: self::_bindgen_ty_3 = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: self::_bindgen_ty_3 = 1127;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: self::_bindgen_ty_3 = 1128;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: self::_bindgen_ty_3 = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: self::_bindgen_ty_3 = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: self::_bindgen_ty_3 = 1131;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: self::_bindgen_ty_3 = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: self::_bindgen_ty_3 = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: self::_bindgen_ty_3 = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: self::_bindgen_ty_3 = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: self::_bindgen_ty_3 = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: self::_bindgen_ty_3 = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: self::_bindgen_ty_3 = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: self::_bindgen_ty_3 = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: self::_bindgen_ty_3 = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: self::_bindgen_ty_3 = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: self::_bindgen_ty_3 = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: self::_bindgen_ty_3 = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: self::_bindgen_ty_3 = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: self::_bindgen_ty_3 = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: self::_bindgen_ty_3 = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: self::_bindgen_ty_3 = 1147;
pub const _CS_V6_ENV: self::_bindgen_ty_3 = 1148;
pub const _CS_V7_ENV: self::_bindgen_ty_3 = 1149;
pub type _bindgen_ty_3 = u32;
extern "C" {
    pub fn pathconf(__path: *const c_char, __name: c_int) -> c_long;
}
extern "C" {
    pub fn fpathconf(__fd: c_int, __name: c_int) -> c_long;
}
extern "C" {
    pub fn sysconf(__name: c_int) -> c_long;
}
extern "C" {
    pub fn getpid() -> self::__pid_t;
}
extern "C" {
    pub fn getppid() -> self::__pid_t;
}
extern "C" {
    pub fn getpgrp() -> self::__pid_t;
}
extern "C" {
    pub fn __getpgid(__pid: self::__pid_t) -> self::__pid_t;
}
extern "C" {
    pub fn setpgid(__pid: self::__pid_t, __pgid: self::__pid_t) -> c_int;
}
extern "C" {
    pub fn setsid() -> self::__pid_t;
}
extern "C" {
    pub fn getuid() -> self::__uid_t;
}
extern "C" {
    pub fn geteuid() -> self::__uid_t;
}
extern "C" {
    pub fn getgid() -> self::__gid_t;
}
extern "C" {
    pub fn getegid() -> self::__gid_t;
}
extern "C" {
    pub fn getgroups(__size: c_int, __list: *mut self::__gid_t) -> c_int;
}
extern "C" {
    pub fn setuid(__uid: self::__uid_t) -> c_int;
}
extern "C" {
    pub fn setgid(__gid: self::__gid_t) -> c_int;
}
extern "C" {
    pub fn fork() -> self::__pid_t;
}
extern "C" {
    pub fn ttyname(__fd: c_int) -> *mut c_char;
}
extern "C" {
    pub fn ttyname_r(__fd: c_int, __buf: *mut c_char, __buflen: usize) -> c_int;
}
extern "C" {
    pub fn isatty(__fd: c_int) -> c_int;
}
extern "C" {
    pub fn link(__from: *const c_char, __to: *const c_char) -> c_int;
}
extern "C" {
    pub fn unlink(__name: *const c_char) -> c_int;
}
extern "C" {
    pub fn rmdir(__path: *const c_char) -> c_int;
}
extern "C" {
    pub fn tcgetpgrp(__fd: c_int) -> self::__pid_t;
}
extern "C" {
    pub fn tcsetpgrp(__fd: c_int, __pgrp_id: self::__pid_t) -> c_int;
}
extern "C" {
    pub fn getlogin() -> *mut c_char;
}
extern "C" {
    pub fn fsync(__fd: c_int) -> c_int;
}
extern "C" {
    pub fn yylex_destroy() -> c_int;
}
extern "C" {
    pub fn yyget_debug() -> c_int;
}
extern "C" {
    pub fn yyset_debug(debug_flag: c_int);
}
extern "C" {
    pub fn yyget_extra() -> *mut c_void;
}
extern "C" {
    pub fn yyset_extra(user_defined: *mut c_void);
}
extern "C" {
    #[doc = " Get the input stream."]
    #[doc = ""]
    pub fn yyget_in() -> *mut self::FILE;
}
extern "C" {
    #[doc = " Set the input stream. This does not discard the current"]
    #[doc = " input buffer."]
    #[doc = " @param _in_str A readable stream."]
    #[doc = ""]
    #[doc = " @see yy_switch_to_buffer"]
    pub fn yyset_in(_in_str: *mut self::FILE);
}
extern "C" {
    #[doc = " Get the output stream."]
    #[doc = ""]
    pub fn yyget_out() -> *mut self::FILE;
}
extern "C" {
    pub fn yyset_out(_out_str: *mut self::FILE);
}
extern "C" {
    #[doc = " Get the length of the current token."]
    #[doc = ""]
    pub fn yyget_leng() -> c_int;
}
extern "C" {
    #[doc = " Get the current token."]
    #[doc = ""]
    pub fn yyget_text() -> *mut c_char;
}
extern "C" {
    #[doc = " Get the current line number."]
    #[doc = ""]
    pub fn yyget_lineno() -> c_int;
}
extern "C" {
    #[doc = " Set the current line number."]
    #[doc = " @param _line_number line number"]
    #[doc = ""]
    pub fn yyset_lineno(_line_number: c_int);
}
extern "C" {
    pub fn yylex() -> c_int;
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
