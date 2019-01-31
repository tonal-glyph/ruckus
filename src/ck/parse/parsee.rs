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
///* chuck parser interface (using flex and bison)
///* I'm leaving most of the system-level artifacts in this file to try to make
///* the parser work. I might move it to a separate file eventually
///* chuck.tab.c, chuck.tab.h, and chuck.yy.c implement the lexer/parser, maybe pull
///* these to a separate lib?
#![feature(libc)]
use libc::*;
use crate::ck::ast::aste::*;
use crate::ck::def::defe::*;
use crate::ck::util::string::stringe::*
use crate::dts::*;
use crate::sys::*;
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
pub const __cpp_lib_is_null_pointer: u32 = 201309;
pub const __cpp_lib_is_final: u32 = 201402;
pub const __cpp_lib_transformation_trait_aliases: u32 = 201304;
pub const __cpp_lib_result_of_sfinae: u32 = 201210;
pub const _STL_ITERATOR_BASE_TYPES_H: u32 = 1;
pub const _STL_ITERATOR_BASE_FUNCS_H: u32 = 1;
pub const _GLIBCXX_DEBUG_ASSERTIONS_H: u32 = 1;
pub const _STL_ITERATOR_H: u32 = 1;
pub const _PTR_TRAITS_H: u32 = 1;
pub const __cpp_lib_make_reverse_iterator: u32 = 201402;
pub const _GLIBCXX_DEBUG_MACRO_SWITCH_H: u32 = 1;
pub const _GLIBCXX_PREDEFINED_OPS_H: u32 = 1;
pub const __cpp_lib_robust_nonmodifying_seq_ops: u32 = 201304;
pub const _GLIBCXX_POSTYPES_H: u32 = 1;
pub const _WCHAR_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const __wint_t_defined: u32 = 1;
pub const _WINT_T: u32 = 1;
pub const __mbstate_t_defined: u32 = 1;
pub const WEOF: u32 = 4294967295;
pub const _GLIBCXX_CWCHAR: u32 = 1;
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
pub const _CXXABI_INIT_EXCEPTION_H: u32 = 1;
pub const _GLIBCXX_HAVE_CDTOR_CALLABI: u32 = 0;
pub const _HASH_BYTES_H: u32 = 1;
pub const __GXX_MERGED_TYPEINFO_NAMES: u32 = 0;
pub const __GXX_TYPEINFO_EQUALITY_INLINE: u32 = 1;
pub const _GLIBCXX_NESTED_EXCEPTION_H: u32 = 1;
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
pub const _FUNCTIONAL_HASH_H: u32 = 1;
pub const __cpp_lib_string_udls: u32 = 201304;
pub const _BASIC_STRING_TCC: u32 = 1;
pub const _GLIBCXX_VECTOR: u32 = 1;
pub const _STL_CONSTRUCT_H: u32 = 1;
pub const _STL_UNINITIALIZED_H: u32 = 1;
pub const _STL_VECTOR_H: u32 = 1;
pub const _STL_BVECTOR_H: u32 = 1;
pub const _VECTOR_TCC: u32 = 1;
pub mod std {
    
    pub type nullptr_t = *const c_void;
    pub type string = crate::chuck_parse_h_edited::std::basic_string<c_char>;
    pub type wstring = crate::chuck_parse_h_edited::std::basic_string<u32>;
    pub type u16string = crate::chuck_parse_h_edited::std::basic_string<u16>;
    pub type u32string = crate::chuck_parse_h_edited::std::basic_string<u32>;
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
    pub struct numpunct {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct numpunct_byname {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct collate {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct collate_byname {
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
        pub _M_dataplus: crate::chuck_parse_h_edited::std::basic_string__Alloc_hider,
        pub _M_string_length: crate::chuck_parse_h_edited::std::basic_string_size_type,
        pub __bindgen_anon_1: crate::chuck_parse_h_edited::std::basic_string__bindgen_ty_2<_CharT>,
        pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
    }
    pub type basic_string__Char_alloc_type = [u8; 0usize];
    pub type basic_string__Alloc_traits = __alloc_traits;
    pub type basic_string_traits_type<_Traits> = _Traits;
    pub type basic_string_value_type = [u8; 0usize];
    pub type basic_string_allocator_type =
        crate::chuck_parse_h_edited::std::basic_string__Char_alloc_type;
    pub type basic_string_size_type = [u8; 0usize];
    pub type basic_string_difference_type = [u8; 0usize];
    pub type basic_string_reference = [u8; 0usize];
    pub type basic_string_const_reference = [u8; 0usize];
    pub type basic_string_pointer = [u8; 0usize];
    pub type basic_string_const_pointer = [u8; 0usize];
    pub type basic_string_iterator =
        __normal_iterator<crate::chuck_parse_h_edited::std::basic_string_pointer>;
    pub type basic_string_const_iterator =
        __normal_iterator<crate::chuck_parse_h_edited::std::basic_string_const_pointer>;
    pub type basic_string_const_reverse_iterator =
        crate::chuck_parse_h_edited::std::reverse_iterator<
            crate::chuck_parse_h_edited::std::basic_string_const_iterator,
        >;
    pub type basic_string_reverse_iterator = crate::chuck_parse_h_edited::std::reverse_iterator<
        crate::chuck_parse_h_edited::std::basic_string_iterator,
    >;
    pub type basic_string___const_iterator =
        crate::chuck_parse_h_edited::std::basic_string_const_iterator;
    #[repr(C)]
    pub struct basic_string__Alloc_hider {
        pub _M_p: crate::chuck_parse_h_edited::std::basic_string_pointer,
    }
    pub const basic_string__S_local_capacity:
        crate::chuck_parse_h_edited::std::basic_string__bindgen_ty_1 = 0;
    pub type basic_string__bindgen_ty_1 = i32;
    #[repr(C)]
    pub struct basic_string__bindgen_ty_2<_CharT> {
        pub _M_local_buf: __BindgenUnionField<*mut _CharT>,
        pub _M_allocated_capacity:
            __BindgenUnionField<crate::chuck_parse_h_edited::std::basic_string_size_type>,
        pub bindgen_union_field: u64,
        pub _phantom_0: PhantomData<UnsafeCell<_CharT>>,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct uses_allocator {
        pub _address: u8,
    }
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
    pub type __truth_type___type = crate::chuck_parse_h_edited::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __traitor {
        pub _address: u8,
    }
    pub const __traitor___value: crate::chuck_parse_h_edited::std::__traitor__bindgen_ty_1 = 0;
    pub type __traitor__bindgen_ty_1 = i32;
    pub type __traitor___type = u8;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __are_same {
        pub _address: u8,
    }
    pub const __are_same___value: crate::chuck_parse_h_edited::std::__are_same__bindgen_ty_1 = 0;
    pub type __are_same__bindgen_ty_1 = i32;
    pub type __are_same___type = crate::chuck_parse_h_edited::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_void {
        pub _address: u8,
    }
    pub const __is_void___value: crate::chuck_parse_h_edited::std::__is_void__bindgen_ty_1 = 0;
    pub type __is_void__bindgen_ty_1 = i32;
    pub type __is_void___type = crate::chuck_parse_h_edited::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_integer {
        pub _address: u8,
    }
    pub const __is_integer___value: crate::chuck_parse_h_edited::std::__is_integer__bindgen_ty_1 =
        0;
    pub type __is_integer__bindgen_ty_1 = i32;
    pub type __is_integer___type = crate::chuck_parse_h_edited::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_floating {
        pub _address: u8,
    }
    pub const __is_floating___value: crate::chuck_parse_h_edited::std::__is_floating__bindgen_ty_1 =
        0;
    pub type __is_floating__bindgen_ty_1 = i32;
    pub type __is_floating___type = crate::chuck_parse_h_edited::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_pointer {
        pub _address: u8,
    }
    pub const __is_pointer___value: crate::chuck_parse_h_edited::std::__is_pointer__bindgen_ty_1 =
        0;
    pub type __is_pointer__bindgen_ty_1 = i32;
    pub type __is_pointer___type = crate::chuck_parse_h_edited::std::__false_type;
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
    pub const __is_char___value: crate::chuck_parse_h_edited::std::__is_char__bindgen_ty_1 = 0;
    pub type __is_char__bindgen_ty_1 = i32;
    pub type __is_char___type = crate::chuck_parse_h_edited::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_byte {
        pub _address: u8,
    }
    pub const __is_byte___value: crate::chuck_parse_h_edited::std::__is_byte__bindgen_ty_1 = 0;
    pub type __is_byte__bindgen_ty_1 = i32;
    pub type __is_byte___type = crate::chuck_parse_h_edited::std::__false_type;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_move_iterator {
        pub _address: u8,
    }
    pub const __is_move_iterator___value:
        crate::chuck_parse_h_edited::std::__is_move_iterator__bindgen_ty_1 = 0;
    pub type __is_move_iterator__bindgen_ty_1 = i32;
    pub type __is_move_iterator___type = crate::chuck_parse_h_edited::std::__false_type;
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
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_void {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_integral_helper {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_integral {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_floating_point_helper {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_floating_point {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_array {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_pointer_helper {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_lvalue_reference {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_rvalue_reference {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_member_object_pointer_helper {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_member_object_pointer {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_member_function_pointer_helper {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
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
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_null_pointer_helper {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
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
        pub _base: crate::chuck_parse_h_edited::std::false_type,
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
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_volatile {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
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
        pub fn __do_is_implicitly_default_constructible_impl___test(
        ) -> crate::chuck_parse_h_edited::std::false_type;
    }
    impl __do_is_implicitly_default_constructible_impl {
        #[inline]
        pub unsafe fn __test() -> crate::chuck_parse_h_edited::std::false_type {
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
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct is_base_of {
        pub _address: u8,
    }
    pub type __is_convertible_helper_type = crate::chuck_parse_h_edited::std::is_void;
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
    pub type remove_cv_type = crate::chuck_parse_h_edited::std::remove_const;
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
    pub type add_cv_type = crate::chuck_parse_h_edited::std::add_const;
    pub type remove_const_t = crate::chuck_parse_h_edited::std::remove_const;
    pub type remove_volatile_t = crate::chuck_parse_h_edited::std::remove_volatile;
    pub type remove_cv_t = crate::chuck_parse_h_edited::std::remove_cv;
    pub type add_const_t = crate::chuck_parse_h_edited::std::add_const;
    pub type add_volatile_t = crate::chuck_parse_h_edited::std::add_volatile;
    pub type add_cv_t = crate::chuck_parse_h_edited::std::add_cv;
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
    pub type remove_reference_t = crate::chuck_parse_h_edited::std::remove_reference;
    pub type add_lvalue_reference_t = crate::chuck_parse_h_edited::std::add_lvalue_reference;
    pub type add_rvalue_reference_t = crate::chuck_parse_h_edited::std::add_rvalue_reference;
    pub type __match_cv_qualifiers___match = u8;
    pub type __match_cv_qualifiers___type =
        crate::chuck_parse_h_edited::std::__match_cv_qualifiers___match;
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
    pub type make_signed_t = crate::chuck_parse_h_edited::std::make_signed;
    pub type make_unsigned_t = crate::chuck_parse_h_edited::std::make_unsigned;
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
    pub type remove_extent_t = crate::chuck_parse_h_edited::std::remove_extent;
    pub type remove_all_extents_t = crate::chuck_parse_h_edited::std::remove_all_extents;
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
    pub type remove_pointer_t = crate::chuck_parse_h_edited::std::remove_pointer;
    pub type add_pointer_t = crate::chuck_parse_h_edited::std::add_pointer;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union __aligned_storage_msa___type {
        pub __data: *mut c_uchar,
        pub __align: crate::chuck_parse_h_edited::std::__aligned_storage_msa___type__bindgen_ty_1,
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
        pub __align: crate::chuck_parse_h_edited::std::aligned_storage_type__bindgen_ty_1,
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
    pub type aligned_union___strictest = crate::chuck_parse_h_edited::std::__strictest_alignment;
    pub type aligned_union_type = u8;
    extern "C" {
        pub static alignment_value: usize;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct decay {
        pub _address: u8,
    }
    pub type decay___remove_type = crate::chuck_parse_h_edited::std::remove_reference;
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
    pub type __decay_and_strip___type = crate::chuck_parse_h_edited::std::__strip_reference_wrapper;
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
    pub type __expanded_common_type_wrapper_type = crate::chuck_parse_h_edited::std::common_type;
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
    pub type __result_of_impl_type = crate::chuck_parse_h_edited::std::__failure_type;
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
    pub type decay_t = crate::chuck_parse_h_edited::std::decay;
    pub type enable_if_t = u8;
    pub type conditional_t = u8;
    pub type common_type_t = crate::chuck_parse_h_edited::std::common_type;
    pub type underlying_type_t = crate::chuck_parse_h_edited::std::underlying_type;
    pub type result_of_t = crate::chuck_parse_h_edited::std::result_of;
    pub type __enable_if_t = u8;
    pub type __void_t = c_void;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __detector {
        pub _address: u8,
    }
    pub type __detector_value_t = crate::chuck_parse_h_edited::std::false_type;
    pub type __detector_type<_Default> = _Default;
    pub type __detected_or = crate::chuck_parse_h_edited::std::__detector;
    pub type __detected_or_t = crate::chuck_parse_h_edited::std::__detected_or;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct tuple {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_tuple_like_impl {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_tuple_like {
        pub _address: u8,
    }
    pub mod __swappable_details {
        
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
        pub _base: crate::chuck_parse_h_edited::std::false_type,
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
    pub type __call_is_nothrow_ = crate::chuck_parse_h_edited::std::__call_is_nothrow;
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
    #[derive(Debug, Copy, Clone)]
    pub struct piecewise_construct_t {
        pub _address: u8,
    }
    extern "C" {
        #[link_name = "\u{1}_ZStL19piecewise_construct"]
        pub static piecewise_construct: crate::chuck_parse_h_edited::std::piecewise_construct_t;
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
    pub type __get_first_arg_type = crate::chuck_parse_h_edited::std::__undefined;
    pub type __get_first_arg_t = crate::chuck_parse_h_edited::std::__get_first_arg;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __replace_first_arg {
        pub _address: u8,
    }
    pub type __replace_first_arg_t = crate::chuck_parse_h_edited::std::__replace_first_arg;
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
    pub type pointer_traits_element_type = crate::chuck_parse_h_edited::std::__detected_or_t;
    pub type pointer_traits_difference_type = crate::chuck_parse_h_edited::std::__detected_or_t;
    pub type pointer_traits_rebind = crate::chuck_parse_h_edited::std::pointer_traits___rebind;
    pub type __ptr_rebind = crate::chuck_parse_h_edited::std::pointer_traits;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct reverse_iterator<_Iterator> {
        pub current: _Iterator,
        pub _phantom_0: PhantomData<UnsafeCell<_Iterator>>,
    }
    pub type reverse_iterator___traits_type = crate::chuck_parse_h_edited::std::iterator_traits;
    pub type reverse_iterator_iterator_type<_Iterator> = _Iterator;
    pub type reverse_iterator_difference_type =
        crate::chuck_parse_h_edited::std::reverse_iterator___traits_type;
    pub type reverse_iterator_pointer =
        crate::chuck_parse_h_edited::std::reverse_iterator___traits_type;
    pub type reverse_iterator_reference =
        crate::chuck_parse_h_edited::std::reverse_iterator___traits_type;
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
    pub type move_iterator___traits_type = crate::chuck_parse_h_edited::std::iterator_traits;
    pub type move_iterator___base_ref =
        crate::chuck_parse_h_edited::std::move_iterator___traits_type;
    pub type move_iterator_iterator_type<_Iterator> = _Iterator;
    pub type move_iterator_iterator_category =
        crate::chuck_parse_h_edited::std::move_iterator___traits_type;
    pub type move_iterator_value_type =
        crate::chuck_parse_h_edited::std::move_iterator___traits_type;
    pub type move_iterator_difference_type =
        crate::chuck_parse_h_edited::std::move_iterator___traits_type;
    pub type move_iterator_pointer<_Iterator> = _Iterator;
    pub type move_iterator_reference = u8;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct istreambuf_iterator {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ostreambuf_iterator {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __lc_rai {
        pub _address: u8,
    }
    pub type streamoff = c_long;
    pub type streamsize = isize;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct fpos<_StateT> {
        pub _M_off: crate::chuck_parse_h_edited::std::streamoff,
        pub _M_state: _StateT,
        pub _phantom_0: PhantomData<UnsafeCell<_StateT>>,
    }
    pub type streampos = crate::chuck_parse_h_edited::std::fpos<mbstate_t>;
    pub type wstreampos = crate::chuck_parse_h_edited::std::fpos<mbstate_t>;
    pub type u16streampos = crate::chuck_parse_h_edited::std::fpos<mbstate_t>;
    pub type u32streampos = crate::chuck_parse_h_edited::std::fpos<mbstate_t>;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct char_traits {
        pub _address: u8,
    }
    #[repr(C)]
    pub struct exception__bindgen_vtable(c_void);
    #[repr(C)]
    #[derive(Debug)]
    pub struct exception {
        pub vtable_: *const exception__bindgen_vtable,
    }
    extern "C" {
        #[link_name = "\u{1}_ZNSt9exceptionD1Ev"]
        pub fn exception_exception_destructor(
            this: *mut crate::chuck_parse_h_edited::std::exception,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZNKSt9exception4whatEv"]
        pub fn exception_what(this: *mut c_void) -> *const c_char;
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct bad_exception {
        pub _base: crate::chuck_parse_h_edited::std::exception,
    }
    extern "C" {
        #[link_name = "\u{1}_ZNSt13bad_exceptionD1Ev"]
        pub fn bad_exception_bad_exception_destructor(
            this: *mut crate::chuck_parse_h_edited::std::bad_exception,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZNKSt13bad_exception4whatEv"]
        pub fn bad_exception_what(this: *mut c_void) -> *const c_char;
    }
    pub type terminate_handler = Option<unsafe extern "C" fn()>;
    pub type unexpected_handler = Option<unsafe extern "C" fn()>;
    extern "C" {
        #[link_name = "\u{1}_ZSt13set_terminatePFvvE"]
        pub fn set_terminate(
            arg1: crate::chuck_parse_h_edited::std::terminate_handler,
        ) -> crate::chuck_parse_h_edited::std::terminate_handler;
    }
    extern "C" {
        #[link_name = "\u{1}_ZSt13get_terminatev"]
        pub fn get_terminate() -> crate::chuck_parse_h_edited::std::terminate_handler;
    }
    extern "C" {
        #[link_name = "\u{1}_ZSt9terminatev"]
        pub fn terminate();
    }
    extern "C" {
        #[link_name = "\u{1}_ZSt14set_unexpectedPFvvE"]
        pub fn set_unexpected(
            arg1: crate::chuck_parse_h_edited::std::unexpected_handler,
        ) -> crate::chuck_parse_h_edited::std::unexpected_handler;
    }
    extern "C" {
        #[link_name = "\u{1}_ZSt14get_unexpectedv"]
        pub fn get_unexpected() -> crate::chuck_parse_h_edited::std::unexpected_handler;
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
        pub fn type_info_type_info_destructor(
            this: *mut crate::chuck_parse_h_edited::std::type_info,
        );
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
            __thr_type: *const crate::chuck_parse_h_edited::std::type_info,
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
        pub _base: crate::chuck_parse_h_edited::std::exception,
    }
    extern "C" {
        #[link_name = "\u{1}_ZNSt8bad_castD1Ev"]
        pub fn bad_cast_bad_cast_destructor(this: *mut crate::chuck_parse_h_edited::std::bad_cast);
    }
    extern "C" {
        #[link_name = "\u{1}_ZNKSt8bad_cast4whatEv"]
        pub fn bad_cast_what(this: *mut c_void) -> *const c_char;
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct bad_typeid {
        pub _base: crate::chuck_parse_h_edited::std::exception,
    }
    extern "C" {
        #[link_name = "\u{1}_ZNSt10bad_typeidD1Ev"]
        pub fn bad_typeid_bad_typeid_destructor(
            this: *mut crate::chuck_parse_h_edited::std::bad_typeid,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZNKSt10bad_typeid4whatEv"]
        pub fn bad_typeid_what(this: *mut c_void) -> *const c_char;
    }
    pub mod __exception_ptr {
        
        #[repr(C)]
        #[derive(Debug)]
        pub struct exception_ptr {
            pub _M_exception_object: *mut c_void,
        }
        extern "C" {
            #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptr4swapERS0_"]
            pub fn exception_ptr_swap(
                this: *mut crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
                arg1: *mut crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_ZNKSt15__exception_ptr13exception_ptr20__cxa_exception_typeEv"]
            pub fn exception_ptr___cxa_exception_type(
                this: *const crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
            ) -> *const crate::chuck_parse_h_edited::std::type_info;
        }
        extern "C" {
            #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrC1Ev"]
            pub fn exception_ptr_exception_ptr(
                this: *mut crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrC1ERKS0_"]
            pub fn exception_ptr_exception_ptr1(
                this: *mut crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
                arg1: *const crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
            );
        }
        extern "C" {
            #[link_name = "\u{1}_ZNSt15__exception_ptr13exception_ptrD1Ev"]
            pub fn exception_ptr_exception_ptr_destructor(
                this: *mut crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
            );
        }
        impl exception_ptr {
            #[inline]
            pub unsafe fn swap(
                &mut self,
                arg1: *mut crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
            ) {
                exception_ptr_swap(self, arg1)
            }
            #[inline]
            pub unsafe fn __cxa_exception_type(
                &self,
            ) -> *const crate::chuck_parse_h_edited::std::type_info {
                exception_ptr___cxa_exception_type(self)
            }
            #[inline]
            pub unsafe fn new() -> Self {
                let mut __bindgen_tmp = uninitialized();
                exception_ptr_exception_ptr(&mut __bindgen_tmp);
                __bindgen_tmp
            }
            #[inline]
            pub unsafe fn new1(
                arg1: *const crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
            ) -> Self {
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
        #[link_name = "\u{1}_ZSt17current_exceptionv"]
        pub fn current_exception(
        ) -> crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr;
    }
    extern "C" {
        #[link_name = "\u{1}_ZSt17rethrow_exceptionNSt15__exception_ptr13exception_ptrE"]
        pub fn rethrow_exception(
            arg1: crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
        );
    }
    #[repr(C)]
    pub struct nested_exception__bindgen_vtable(c_void);
    #[repr(C)]
    #[derive(Debug)]
    pub struct nested_exception {
        pub vtable_: *const nested_exception__bindgen_vtable,
        pub _M_ptr: crate::chuck_parse_h_edited::std::__exception_ptr::exception_ptr,
    }
    extern "C" {
        #[link_name = "\u{1}_ZNSt16nested_exceptionD1Ev"]
        pub fn nested_exception_nested_exception_destructor(
            this: *mut crate::chuck_parse_h_edited::std::nested_exception,
        );
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct _Nested_exception<_Except> {
        pub _base: _Except,
        pub _base_1: crate::chuck_parse_h_edited::std::nested_exception,
        pub _phantom_0: PhantomData<UnsafeCell<_Except>>,
    }
    pub type __rethrow_if_nested_cond = u8;
    #[repr(C)]
    #[derive(Debug)]
    pub struct bad_alloc {
        pub _base: crate::chuck_parse_h_edited::std::exception,
    }
    extern "C" {
        #[link_name = "\u{1}_ZNSt9bad_allocD1Ev"]
        pub fn bad_alloc_bad_alloc_destructor(
            this: *mut crate::chuck_parse_h_edited::std::bad_alloc,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZNKSt9bad_alloc4whatEv"]
        pub fn bad_alloc_what(this: *mut c_void) -> *const c_char;
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct bad_array_new_length {
        pub _base: crate::chuck_parse_h_edited::std::bad_alloc,
    }
    extern "C" {
        #[link_name = "\u{1}_ZNSt20bad_array_new_lengthD1Ev"]
        pub fn bad_array_new_length_bad_array_new_length_destructor(
            this: *mut crate::chuck_parse_h_edited::std::bad_array_new_length,
        );
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
        pub static nothrow: crate::chuck_parse_h_edited::std::nothrow_t;
    }
    pub type new_handler = Option<unsafe extern "C" fn()>;
    extern "C" {
        #[link_name = "\u{1}_ZSt15set_new_handlerPFvvE"]
        pub fn set_new_handler(
            arg1: crate::chuck_parse_h_edited::std::new_handler,
        ) -> crate::chuck_parse_h_edited::std::new_handler;
    }
    extern "C" {
        #[link_name = "\u{1}_ZSt15get_new_handlerv"]
        pub fn get_new_handler() -> crate::chuck_parse_h_edited::std::new_handler;
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
    pub type allocator_rebind_other = crate::chuck_parse_h_edited::std::allocator;
    pub type allocator_propagate_on_container_move_assignment =
        crate::chuck_parse_h_edited::std::true_type;
    pub type allocator_is_always_equal = crate::chuck_parse_h_edited::std::true_type;
    pub type __c_locale = __locale_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ios_base {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct basic_ios {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct basic_streambuf {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct basic_istream {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct basic_ostream {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct basic_iostream {
        pub _address: u8,
    }
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
    pub type ios = crate::chuck_parse_h_edited::std::basic_ios;
    pub type streambuf = crate::chuck_parse_h_edited::std::basic_streambuf;
    pub type istream = crate::chuck_parse_h_edited::std::basic_istream;
    pub type ostream = crate::chuck_parse_h_edited::std::basic_ostream;
    pub type iostream = crate::chuck_parse_h_edited::std::basic_iostream;
    pub type stringbuf = crate::chuck_parse_h_edited::std::basic_stringbuf;
    pub type istringstream = crate::chuck_parse_h_edited::std::basic_istringstream;
    pub type ostringstream = crate::chuck_parse_h_edited::std::basic_ostringstream;
    pub type stringstream = crate::chuck_parse_h_edited::std::basic_stringstream;
    pub type filebuf = crate::chuck_parse_h_edited::std::basic_filebuf;
    pub type ifstream = crate::chuck_parse_h_edited::std::basic_ifstream;
    pub type ofstream = crate::chuck_parse_h_edited::std::basic_ofstream;
    pub type fstream = crate::chuck_parse_h_edited::std::basic_fstream;
    pub type wios = crate::chuck_parse_h_edited::std::basic_ios;
    pub type wstreambuf = crate::chuck_parse_h_edited::std::basic_streambuf;
    pub type wistream = crate::chuck_parse_h_edited::std::basic_istream;
    pub type wostream = crate::chuck_parse_h_edited::std::basic_ostream;
    pub type wiostream = crate::chuck_parse_h_edited::std::basic_iostream;
    pub type wstringbuf = crate::chuck_parse_h_edited::std::basic_stringbuf;
    pub type wistringstream = crate::chuck_parse_h_edited::std::basic_istringstream;
    pub type wostringstream = crate::chuck_parse_h_edited::std::basic_ostringstream;
    pub type wstringstream = crate::chuck_parse_h_edited::std::basic_stringstream;
    pub type wfilebuf = crate::chuck_parse_h_edited::std::basic_filebuf;
    pub type wifstream = crate::chuck_parse_h_edited::std::basic_ifstream;
    pub type wofstream = crate::chuck_parse_h_edited::std::basic_ofstream;
    pub type wfstream = crate::chuck_parse_h_edited::std::basic_fstream;
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
    #[derive(Debug, Copy, Clone)]
    pub struct ctype {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ctype_byname {
        pub _address: u8,
    }
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
    pub struct num_get {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct num_put {
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
        pub _M_array: crate::chuck_parse_h_edited::std::initializer_list_iterator<_E>,
        pub _M_len: crate::chuck_parse_h_edited::std::initializer_list_size_type,
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
    pub type __alloc_rebind = crate::chuck_parse_h_edited::std::__allocator_traits_base;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct allocator_traits {
        pub _address: u8,
    }
    pub type allocator_traits_allocator_type<_Alloc> = _Alloc;
    pub type allocator_traits_value_type = [u8; 0usize];
    pub type allocator_traits_pointer = crate::chuck_parse_h_edited::std::__detected_or_t;
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
    pub type allocator_traits__Diff_type = crate::chuck_parse_h_edited::std::pointer_traits;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct allocator_traits__Size {
        pub _address: u8,
    }
    pub type allocator_traits_const_pointer = [u8; 0usize];
    pub type allocator_traits_void_pointer =
        crate::chuck_parse_h_edited::std::allocator_traits__Ptr;
    pub type allocator_traits_const_void_pointer =
        crate::chuck_parse_h_edited::std::allocator_traits__Ptr;
    pub type allocator_traits_difference_type = [u8; 0usize];
    pub type allocator_traits_size_type = [u8; 0usize];
    pub type allocator_traits_propagate_on_container_copy_assignment =
        crate::chuck_parse_h_edited::std::__detected_or_t;
    pub type allocator_traits_propagate_on_container_move_assignment =
        crate::chuck_parse_h_edited::std::__detected_or_t;
    pub type allocator_traits_propagate_on_container_swap =
        crate::chuck_parse_h_edited::std::__detected_or_t;
    pub type allocator_traits_is_always_equal = crate::chuck_parse_h_edited::std::__detected_or_t;
    pub type allocator_traits_rebind_alloc = crate::chuck_parse_h_edited::std::__alloc_rebind;
    pub type allocator_traits_rebind_traits = crate::chuck_parse_h_edited::std::allocator_traits;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct allocator_traits___construct_helper {
        pub _address: u8,
    }
    pub type allocator_traits___construct_helper_type<_Alloc> = _Alloc;
    pub type allocator_traits___has_construct =
        crate::chuck_parse_h_edited::std::allocator_traits___construct_helper;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_copy_insertable_impl {
        pub _address: u8,
    }
    pub type __is_copy_insertable_impl__Traits = crate::chuck_parse_h_edited::std::allocator_traits;
    pub type __is_copy_insertable_impl_type<_Alloc> = _Alloc;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_copy_insertable {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __is_allocator {
        pub _base: crate::chuck_parse_h_edited::std::false_type,
    }
    pub type _RequireAllocator = u8;
    extern "C" {
        #[link_name = "\u{1}_ZSt7getlineIcSt11char_traitsIcESaIcEERSt13basic_istreamIT_T0_ES7_RNSt7__cxx1112basic_stringIS4_S5_T1_EES4_"]
        pub fn getline(
            __in: *mut crate::chuck_parse_h_edited::std::basic_istream,
            __str: *mut crate::chuck_parse_h_edited::std::basic_string<c_char>,
            __delim: c_char,
        ) -> *mut crate::chuck_parse_h_edited::std::basic_istream;
    }
    extern "C" {
        #[link_name = "\u{1}_ZSt7getlineIwSt11char_traitsIwESaIwEERSt13basic_istreamIT_T0_ES7_RNSt7__cxx1112basic_stringIS4_S5_T1_EES4_"]
        pub fn getline1(
            __in: *mut crate::chuck_parse_h_edited::std::basic_istream,
            __str: *mut crate::chuck_parse_h_edited::std::basic_string<u32>,
            __delim: u32,
        ) -> *mut crate::chuck_parse_h_edited::std::basic_istream;
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
        pub _base: crate::chuck_parse_h_edited::std::true_type,
    }
    extern "C" {
        pub static npos: crate::chuck_parse_h_edited::std::basic_string_size_type;
    }
    #[repr(C)]
    pub struct _Vector_base {
        pub _M_impl: crate::chuck_parse_h_edited::std::_Vector_base__Vector_impl,
    }
    pub type _Vector_base__Tp_alloc_type = [u8; 0usize];
    pub type _Vector_base_pointer = [u8; 0usize];
    #[repr(C)]
    pub struct _Vector_base__Vector_impl {
        pub _M_start: crate::chuck_parse_h_edited::std::_Vector_base_pointer,
        pub _M_finish: crate::chuck_parse_h_edited::std::_Vector_base_pointer,
        pub _M_end_of_storage: crate::chuck_parse_h_edited::std::_Vector_base_pointer,
    }
    pub type _Vector_base_allocator_type<_Alloc> = _Alloc;
    #[repr(C)]
    pub struct vector {
        pub _base: crate::chuck_parse_h_edited::std::_Vector_base,
    }
    pub type vector__Base = crate::chuck_parse_h_edited::std::_Vector_base;
    pub type vector__Tp_alloc_type = Vec<f64>__Base;
    pub type vector__Alloc_traits = __alloc_traits;
    pub type vector_value_type<_Tp> = _Tp;
    pub type vector_pointer = Vec<f64>__Base;
    pub type vector_const_pointer = Vec<f64>__Alloc_traits;
    pub type vector_reference = Vec<f64>__Alloc_traits;
    pub type vector_const_reference = Vec<f64>__Alloc_traits;
    pub type vector_iterator = __normal_iterator<Vec<f64>_pointer>;
    pub type vector_const_iterator = __normal_iterator<Vec<f64>_const_pointer>;
    pub type vector_const_reverse_iterator =
        crate::chuck_parse_h_edited::std::reverse_iterator<Vec<f64>_const_iterator>;
    pub type vector_reverse_iterator =
        crate::chuck_parse_h_edited::std::reverse_iterator<Vec<f64>_iterator>;
    pub type vector_size_type = usize;
    pub type vector_difference_type = isize;
    pub type vector_allocator_type<_Alloc> = _Alloc;
    #[repr(C)]
    #[derive(Debug)]
    pub struct vector__Temporary_value {
        pub _M_this: *mut Vec<f64>,
        pub __buf: u8,
    }
    pub type _Bit_type = c_ulong;
    pub const std__S_word_bit: crate::chuck_parse_h_edited::std::_bindgen_ty_1 = 64;
    pub type _bindgen_ty_1 = u32;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _Bit_reference {
        pub _M_p: *mut crate::chuck_parse_h_edited::std::_Bit_type,
        pub _M_mask: crate::chuck_parse_h_edited::std::_Bit_type,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _Bit_iterator_base {
        pub _M_p: *mut crate::chuck_parse_h_edited::std::_Bit_type,
        pub _M_offset: c_uint,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _Bit_iterator {
        pub _base: crate::chuck_parse_h_edited::std::_Bit_iterator_base,
    }
    pub type _Bit_iterator_reference = crate::chuck_parse_h_edited::std::_Bit_reference;
    pub type _Bit_iterator_pointer = *mut crate::chuck_parse_h_edited::std::_Bit_reference;
    pub type _Bit_iterator_iterator = crate::chuck_parse_h_edited::std::_Bit_iterator;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _Bit_const_iterator {
        pub _base: crate::chuck_parse_h_edited::std::_Bit_iterator_base,
    }
    pub type _Bit_const_iterator_reference = bool;
    pub type _Bit_const_iterator_const_reference = bool;
    pub type _Bit_const_iterator_pointer = *const bool;
    pub type _Bit_const_iterator_const_iterator =
        crate::chuck_parse_h_edited::std::_Bit_const_iterator;
    #[repr(C)]
    pub struct _Bvector_base {
        pub _M_impl: crate::chuck_parse_h_edited::std::_Bvector_base__Bvector_impl,
    }
    pub type _Bvector_base__Bit_alloc_type = [u8; 0usize];
    pub type _Bvector_base__Bit_alloc_traits = __alloc_traits;
    pub type _Bvector_base__Bit_pointer = [u8; 0usize];
    #[repr(C)]
    pub struct _Bvector_base__Bvector_impl_data {
        pub _M_start: crate::chuck_parse_h_edited::std::_Bit_iterator,
        pub _M_finish: crate::chuck_parse_h_edited::std::_Bit_iterator,
        pub _M_end_of_storage: crate::chuck_parse_h_edited::std::_Bvector_base__Bit_pointer,
    }
    #[repr(C)]
    pub struct _Bvector_base__Bvector_impl {
        pub _base_1: crate::chuck_parse_h_edited::std::_Bvector_base__Bvector_impl_data,
    }
    pub type _Bvector_base_allocator_type<_Alloc> = _Alloc;
}
pub mod __gnu_cxx {
    
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
    pub type __normal_iterator___traits_type = crate::chuck_parse_h_edited::std::iterator_traits;
    pub type __normal_iterator_iterator_type<_Iterator> = _Iterator;
    pub type __normal_iterator_iterator_category = __normal_iterator___traits_type;
    pub type __normal_iterator_value_type = __normal_iterator___traits_type;
    pub type __normal_iterator_difference_type = __normal_iterator___traits_type;
    pub type __normal_iterator_reference = __normal_iterator___traits_type;
    pub type __normal_iterator_pointer = __normal_iterator___traits_type;
    pub mod __ops {
        
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
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _Char_types {
        pub _address: u8,
    }
    pub type _Char_types_int_type = c_ulong;
    pub type _Char_types_pos_type = crate::chuck_parse_h_edited::std::streampos;
    pub type _Char_types_off_type = crate::chuck_parse_h_edited::std::streamoff;
    pub type _Char_types_state_type = mbstate_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct char_traits {
        pub _address: u8,
    }
    pub type char_traits_char_type<_CharT> = _CharT;
    pub type char_traits_int_type = _Char_types;
    pub type char_traits_pos_type = _Char_types;
    pub type char_traits_off_type = _Char_types;
    pub type char_traits_state_type = _Char_types;
    extern "C" {
        #[link_name = "\u{1}_ZN9__gnu_cxx27__verbose_terminate_handlerEv"]
        pub fn __verbose_terminate_handler();
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
    pub type new_allocator_propagate_on_container_move_assignment =
        crate::chuck_parse_h_edited::std::true_type;
    extern "C" {
        pub fn __uselocale(arg1: locale_t) -> locale_t;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __alloc_traits {
        pub _address: u8,
    }
    pub type __alloc_traits_allocator_type<_Alloc> = _Alloc;
    pub type __alloc_traits__Base_type = crate::chuck_parse_h_edited::std::allocator_traits;
    pub type __alloc_traits_value_type = __alloc_traits__Base_type;
    pub type __alloc_traits_pointer = __alloc_traits__Base_type;
    pub type __alloc_traits_const_pointer = __alloc_traits__Base_type;
    pub type __alloc_traits_size_type = __alloc_traits__Base_type;
    pub type __alloc_traits_difference_type = __alloc_traits__Base_type;
    pub type __alloc_traits_reference = *mut __alloc_traits_value_type;
    pub type __alloc_traits_const_reference = *const __alloc_traits_value_type;
    pub type __alloc_traits___is_custom_pointer = crate::chuck_parse_h_edited::std::__and_;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __alloc_traits_rebind {
        pub _address: u8,
    }
    pub type __alloc_traits_rebind_other = __alloc_traits__Base_type;
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: c_int,
    pub rem: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: c_long,
    pub rem: c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    pub fn strtod(__nptr: *const c_char, __endptr: *mut c_char) -> f64;
}
extern "C" {
    pub fn strtof(__nptr: *const c_char, __endptr: *mut c_char) -> f32;
}
extern "C" {
    pub fn strtold(__nptr: *const c_char, __endptr: *mut c_char) -> f64;
}
extern "C" {
    pub fn strtof32(__nptr: *const c_char, __endptr: *mut c_char) -> _Float32;
}
extern "C" {
    pub fn strtof64(__nptr: *const c_char, __endptr: *mut c_char) -> _Float64;
}
extern "C" {
    pub fn strtof32x(__nptr: *const c_char, __endptr: *mut c_char) -> _Float32x;
}
extern "C" {
    pub fn strtof64x(__nptr: *const c_char, __endptr: *mut c_char) -> _Float64x;
}
extern "C" {
    pub fn strtol(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_long;
}
extern "C" {
    pub fn strtoul(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_ulong;
}
extern "C" {
    pub fn strtoq(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn strtouq(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int)
        -> c_ulonglong;
}
extern "C" {
    pub fn strtoll(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const c_char,
        __endptr: *mut c_char,
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
        __f: _Float32,
    ) -> c_int;
}
extern "C" {
    pub fn strfromf64(
        __dest: *mut c_char,
        __size: usize,
        __format: *const c_char,
        __f: _Float64,
    ) -> c_int;
}
extern "C" {
    pub fn strfromf32x(
        __dest: *mut c_char,
        __size: usize,
        __format: *const c_char,
        __f: _Float32x,
    ) -> c_int;
}
extern "C" {
    pub fn strfromf64x(
        __dest: *mut c_char,
        __size: usize,
        __format: *const c_char,
        __f: _Float64x,
    ) -> c_int;
}
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
extern "C" {
    pub fn strtol_l(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __base: c_int,
        __loc: locale_t,
    ) -> c_long;
}
extern "C" {
    pub fn strtoul_l(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __base: c_int,
        __loc: locale_t,
    ) -> c_ulong;
}
extern "C" {
    pub fn strtoll_l(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __base: c_int,
        __loc: locale_t,
    ) -> c_longlong;
}
extern "C" {
    pub fn strtoull_l(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __base: c_int,
        __loc: locale_t,
    ) -> c_ulonglong;
}
extern "C" {
    pub fn strtod_l(__nptr: *const c_char, __endptr: *mut c_char, __loc: locale_t) -> f64;
}
extern "C" {
    pub fn strtof_l(__nptr: *const c_char, __endptr: *mut c_char, __loc: locale_t) -> f32;
}
extern "C" {
    pub fn strtold_l(__nptr: *const c_char, __endptr: *mut c_char, __loc: locale_t) -> f64;
}
extern "C" {
    pub fn strtof32_l(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __loc: locale_t,
    ) -> _Float32;
}
extern "C" {
    pub fn strtof64_l(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __loc: locale_t,
    ) -> _Float64;
}
extern "C" {
    pub fn strtof32x_l(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __loc: locale_t,
    ) -> _Float32x;
}
extern "C" {
    pub fn strtof64x_l(
        __nptr: *const c_char,
        __endptr: *mut c_char,
        __loc: locale_t,
    ) -> _Float64x;
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
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
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
#[derive(Debug, Copy, Clone)]
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
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut c_char;
pub type __intptr_t = c_long;
pub type __socklen_t = c_uint;
pub type __sig_atomic_t = c_int;
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
pub type ulong = c_ulong;
pub type ushort = c_ushort;
pub type uint = c_uint;
pub type u_int8_t = c_uchar;
pub type u_int16_t = c_ushort;
pub type u_int32_t = c_uint;
pub type u_int64_t = c_ulong;
pub type register_t = c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16usize],
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16usize],
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type blkcnt64_t = __blkcnt64_t;
pub type fsblkcnt64_t = __fsblkcnt64_t;
pub type fsfilcnt64_t = __fsfilcnt64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: c_int,
    pub __count: c_uint,
    pub __owner: c_int,
    pub __nusers: c_uint,
    pub __kind: c_int,
    pub __spins: c_short,
    pub __elision: c_short,
    pub __list: __pthread_list_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
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
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: c_uint,
    pub __high: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: c_uint,
    pub __high: c_uint,
}
pub type pthread_t = c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
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
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [c_char; 48usize],
    pub __align: c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [c_char; 56usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [c_char; 8usize],
    pub __align: c_long,
    _bindgen_union_align: u64,
}
pub type pthread_spinlock_t = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [c_char; 32usize],
    pub __align: c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [c_char; 4usize],
    pub __align: c_int,
    _bindgen_union_align: u32,
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
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: c_int,
    pub rand_deg: c_int,
    pub rand_sep: c_int,
    pub end_ptr: *mut i32,
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> c_int;
}
extern "C" {
    pub fn srandom_r(__seed: c_uint, __buf: *mut random_data) -> c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: c_uint,
        __statebuf: *mut c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> c_int;
}
extern "C" {
    pub fn setstate_r(__statebuf: *mut c_char, __buf: *mut random_data) -> c_int;
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
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [c_ushort; 3usize],
    pub __old_x: [c_ushort; 3usize],
    pub __c: c_ushort,
    pub __init: c_ushort,
    pub __a: c_ulonglong,
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> c_int;
}
extern "C" {
    pub fn lrand48_r(__buffer: *mut drand48_data, __result: *mut c_long) -> c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut c_long,
    ) -> c_int;
}
extern "C" {
    pub fn mrand48_r(__buffer: *mut drand48_data, __result: *mut c_long) -> c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut c_long,
    ) -> c_int;
}
extern "C" {
    pub fn srand48_r(__seedval: c_long, __buffer: *mut drand48_data) -> c_int;
}
extern "C" {
    pub fn seed48_r(__seed16v: *mut c_ushort, __buffer: *mut drand48_data) -> c_int;
}
extern "C" {
    pub fn lcong48_r(__param: *mut c_ushort, __buffer: *mut drand48_data) -> c_int;
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
    pub fn posix_memalign(__memptr: *mut c_void, __alignment: usize, __size: usize) -> c_int;
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
pub type comparison_fn_t = __compar_fn_t;
pub type __compar_d_fn_t = Option<
    unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void, arg3: *mut c_void) -> c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const c_void,
        __base: *const c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn qsort(__base: *mut c_void, __nmemb: usize, __size: usize, __compar: __compar_fn_t);
}
extern "C" {
    pub fn qsort_r(
        __base: *mut c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_d_fn_t,
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
    pub fn div(__numer: c_int, __denom: c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: c_long, __denom: c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(__numer: c_longlong, __denom: c_longlong) -> lldiv_t;
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
        __optionp: *mut c_char,
        __tokens: *const *mut c_char,
        __valuep: *mut c_char,
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
    pub fn strcoll_l(__s1: *const c_char, __s2: *const c_char, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn strxfrm_l(__dest: *mut c_char, __src: *const c_char, __n: usize, __l: locale_t)
        -> usize;
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
        __save_ptr: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strtok_r(
        __s: *mut c_char,
        __delim: *const c_char,
        __save_ptr: *mut c_char,
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
    pub fn strerror_l(__errnum: c_int, __l: locale_t) -> *mut c_char;
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
    pub fn strcasecmp_l(__s1: *const c_char, __s2: *const c_char, __loc: locale_t) -> c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        __s1: *const c_char,
        __s2: *const c_char,
        __n: usize,
        __loc: locale_t,
    ) -> c_int;
}
extern "C" {
    pub fn explicit_bzero(__s: *mut c_void, __n: usize);
}
extern "C" {
    pub fn strsep(__stringp: *mut c_char, __delim: *const c_char) -> *mut c_char;
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
pub type U_boolList = *mut U_boolList_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct U_boolList_ {
    pub head: c_ulong,
    pub tail: U_boolList,
}
extern "C" {
    pub fn checked_malloc(size: c_int) -> *mut c_void;
}
extern "C" {
    pub fn cc_str(arg1: *mut c_char) -> c_str;
}
extern "C" {
    pub fn U_BoolList(head: c_ulong, tail: U_boolList) -> U_boolList;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Type {
    _unused: [u8; 0],
}
pub type t_CKTYPE = *mut Chuck_Type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Value {
    _unused: [u8; 0],
}
pub type t_CKVALUE = *mut Chuck_Value;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Func {
    _unused: [u8; 0],
}
pub type t_CKFUNC = *mut Chuck_Func;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Namespace {
    _unused: [u8; 0],
}
pub type t_CKNSPC = *mut Chuck_Namespace;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_Code {
    _unused: [u8; 0],
}
pub type t_CKVMCODE = *mut Chuck_VM_Code;
extern "C" {
    pub fn new_program(section: a_Section, pos: c_int) -> a_Program;
}
extern "C" {
    pub fn prepend_program(section: a_Section, program: a_Program, pos: c_int) -> a_Program;
}
extern "C" {
    pub fn new_section_stmt(stmt_list: a_Stmt_List, pos: c_int) -> a_Section;
}
extern "C" {
    pub fn new_section_func_def(func_def: a_Func_Def, pos: c_int) -> a_Section;
}
extern "C" {
    pub fn new_section_class_def(class_def: a_Class_Def, pos: c_int) -> a_Section;
}
extern "C" {
    pub fn new_stmt_list(stmt: a_Stmt, pos: c_int) -> a_Stmt_List;
}
extern "C" {
    pub fn prepend_stmt_list(stmt: a_Stmt, stmt_list: a_Stmt_List, pos: c_int) -> a_Stmt_List;
}
extern "C" {
    pub fn new_stmt_from_expression(exp: a_Exp, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_code(code: a_Stmt_List, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_while(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_do_while(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_until(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_do_until(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_for(c1: a_Stmt, c2: a_Stmt, c3: a_Exp, body: a_Stmt, pos: c_int)
        -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_loop(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_if(cond: a_Exp, if_body: a_Stmt, else_body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_switch(exp: a_Exp, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_break(pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_continue(pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_return(exp: a_Exp, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_label(xid: c_str, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_case(exp: a_Exp, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn prepend_expression(exp: a_Exp, list: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_binary(lhs: a_Exp, oper: ae_Operator, rhs: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary(oper: ae_Operator, exp: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary2(
        oper: ae_Operator,
        type_: a_Type_Decl,
        array: a_Array_Sub,
        pos: c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary3(oper: ae_Operator, code: a_Stmt, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_cast(type_: a_Type_Decl, exp: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_array(base: a_Exp, indices: a_Array_Sub, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_array_lit(exp_list: a_Array_Sub, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_func_call(base: a_Exp, args: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_member_dot(base: a_Exp, member: c_str, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_postfix(base: a_Exp, op: ae_Operator, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_dur(base: a_Exp, unit: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_id(xid: c_str, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_int(num: c_long, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_uint(num: c_ulong, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_float(num: f64, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_str(str: c_str, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_char(chr: c_str, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_if(cond: a_Exp, lhs: a_Exp, rhs: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_complex(arg1: a_Complex, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_polar(arg1: a_Polar, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_vec(arg1: a_Vec, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl_external(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl_global(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_hack(exp: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_nil(pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_var_decl_list(var_decl: a_Var_Decl, pos: c_int) -> a_Var_Decl_List;
}
extern "C" {
    pub fn prepend_var_decl_list(
        var_decl: a_Var_Decl,
        list: a_Var_Decl_List,
        pos: c_int,
    ) -> a_Var_Decl_List;
}
extern "C" {
    pub fn new_var_decl(xid: c_constr, array: a_Array_Sub, pos: c_int) -> a_Var_Decl;
}
extern "C" {
    pub fn new_type_decl(xid: a_Id_List, ref_: c_int, pos: c_int) -> a_Type_Decl;
}
extern "C" {
    pub fn add_type_decl_array(
        type_decl: a_Type_Decl,
        array: a_Array_Sub,
        pos: c_int,
    ) -> a_Type_Decl;
}
extern "C" {
    pub fn new_arg_list(type_decl: a_Type_Decl, var_decl: a_Var_Decl, pos: c_int) -> a_Arg_List;
}
extern "C" {
    pub fn prepend_arg_list(
        type_decl: a_Type_Decl,
        var_decl: a_Var_Decl,
        arg_list: a_Arg_List,
        pos: c_int,
    ) -> a_Arg_List;
}
extern "C" {
    pub fn new_array_sub(exp: a_Exp, pos: c_int) -> a_Array_Sub;
}
extern "C" {
    pub fn prepend_array_sub(array: a_Array_Sub, exp: a_Exp, pos: c_int) -> a_Array_Sub;
}
extern "C" {
    pub fn new_complex(re: a_Exp, pos: c_int) -> a_Complex;
}
extern "C" {
    pub fn new_polar(mod_: a_Exp, pos: c_int) -> a_Polar;
}
extern "C" {
    pub fn new_vec(e: a_Exp, pos: c_int) -> a_Vec;
}
extern "C" {
    pub fn new_class_def(
        class_decl: ae_Keyword,
        xid: a_Id_List,
        ext: a_Class_Ext,
        body: a_Class_Body,
        pos: c_int,
    ) -> a_Class_Def;
}
extern "C" {
    pub fn new_class_body(section: a_Section, pos: c_int) -> a_Class_Body;
}
extern "C" {
    pub fn prepend_class_body(section: a_Section, body: a_Class_Body, pos: c_int) -> a_Class_Body;
}
extern "C" {
    pub fn new_class_ext(extend_id: a_Id_List, impl_list: a_Id_List, pos: c_int) -> a_Class_Ext;
}
extern "C" {
    pub fn new_iface_def(
        class_decl: ae_Keyword,
        xid: a_Id_List,
        ext: a_Class_Ext,
        body: a_Class_Body,
        pos: c_int,
    ) -> a_Class_Def;
}
extern "C" {
    pub fn new_id_list(xid: c_constr, pos: c_int) -> a_Id_List;
}
extern "C" {
    pub fn prepend_id_list(xid: c_constr, list: a_Id_List, pos: c_int) -> a_Id_List;
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
        pos: c_int,
    ) -> a_Func_Def;
}
extern "C" {
    pub fn delete_id_list(x: a_Id_List);
}
pub type __gnuc_va_list = __builtin_va_list;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
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
pub type _IO_lock_t = c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub __pad5: usize,
    pub _mode: c_int,
    pub _unused2: [c_char; 20usize],
}
pub type cookie_read_function_t = Option<
    unsafe extern "C" fn(__cookie: *mut c_void, __buf: *mut c_char, __nbytes: usize) -> __ssize_t,
>;
pub type cookie_write_function_t = Option<
    unsafe extern "C" fn(__cookie: *mut c_void, __buf: *const c_char, __nbytes: usize) -> __ssize_t,
>;
pub type cookie_seek_function_t =
    Option<unsafe extern "C" fn(__cookie: *mut c_void, __pos: *mut __off64_t, __w: c_int) -> c_int>;
pub type cookie_close_function_t = Option<unsafe extern "C" fn(__cookie: *mut c_void) -> c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_cookie_io_functions_t {
    pub read: cookie_read_function_t,
    pub write: cookie_write_function_t,
    pub seek: cookie_seek_function_t,
    pub close: cookie_close_function_t,
}
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
pub type va_list = __gnuc_va_list;
pub type fpos_t = __fpos_t;
pub type fpos64_t = __fpos64_t;
extern "C" {
    #[link_name = "\u{1}stdin"]
    pub static mut stdin: *mut FILE;
}
extern "C" {
    #[link_name = "\u{1}stdout"]
    pub static mut stdout: *mut FILE;
}
extern "C" {
    #[link_name = "\u{1}stderr"]
    pub static mut stderr: *mut FILE;
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
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpfile64() -> *mut FILE;
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
    pub fn fclose(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fcloseall() -> c_int;
}
extern "C" {
    pub fn fopen(__filename: *const c_char, __modes: *const c_char) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const c_char,
        __modes: *const c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fopen64(__filename: *const c_char, __modes: *const c_char) -> *mut FILE;
}
extern "C" {
    pub fn freopen64(
        __filename: *const c_char,
        __modes: *const c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: c_int, __modes: *const c_char) -> *mut FILE;
}
extern "C" {
    pub fn fopencookie(
        __magic_cookie: *mut c_void,
        __modes: *const c_char,
        __io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(__s: *mut c_void, __len: usize, __modes: *const c_char) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(__bufloc: *mut c_char, __sizeloc: *mut usize) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut c_char);
}
extern "C" {
    pub fn setvbuf(__stream: *mut FILE, __buf: *mut c_char, __modes: c_int, __n: usize) -> c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(__stream: *mut FILE, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn printf(__format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn sprintf(__s: *mut c_char, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vfprintf(__s: *mut FILE, __format: *const c_char, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn vprintf(__format: *const c_char, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn vsprintf(__s: *mut c_char, __format: *const c_char, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn snprintf(__s: *mut c_char, __maxlen: usize, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut c_char,
        __maxlen: usize,
        __format: *const c_char,
        __arg: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn vasprintf(
        __ptr: *mut c_char,
        __f: *const c_char,
        __arg: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn __asprintf(__ptr: *mut c_char, __fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn asprintf(__ptr: *mut c_char, __fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vdprintf(__fd: c_int, __fmt: *const c_char, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn dprintf(__fd: c_int, __fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn fscanf(__stream: *mut FILE, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn scanf(__format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn sscanf(__s: *const c_char, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn vfscanf(__s: *mut FILE, __format: *const c_char, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn vscanf(__format: *const c_char, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn vsscanf(__s: *const c_char, __format: *const c_char, __arg: *mut __va_list_tag)
        -> c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn getchar() -> c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fputc(__c: c_int, __stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn putc(__c: c_int, __stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn putchar(__c: c_int) -> c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: c_int, __stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: c_int, __stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: c_int) -> c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn putw(__w: c_int, __stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fgets(__s: *mut c_char, __n: c_int, __stream: *mut FILE) -> *mut c_char;
}
extern "C" {
    pub fn fgets_unlocked(__s: *mut c_char, __n: c_int, __stream: *mut FILE) -> *mut c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut c_char,
        __n: *mut usize,
        __delimiter: c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut c_char,
        __n: *mut usize,
        __delimiter: c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(__lineptr: *mut c_char, __n: *mut usize, __stream: *mut FILE) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const c_char, __stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn puts(__s: *const c_char) -> c_int;
}
extern "C" {
    pub fn ungetc(__c: c_int, __stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fread(__ptr: *mut c_void, __size: usize, __n: usize, __stream: *mut FILE) -> usize;
}
extern "C" {
    pub fn fwrite(__ptr: *const c_void, __size: usize, __n: usize, __s: *mut FILE) -> usize;
}
extern "C" {
    pub fn fputs_unlocked(__s: *const c_char, __stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(__stream: *mut FILE, __off: c_long, __whence: c_int) -> c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: c_int) -> c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> c_int;
}
extern "C" {
    pub fn fseeko64(__stream: *mut FILE, __off: __off64_t, __whence: c_int) -> c_int;
}
extern "C" {
    pub fn ftello64(__stream: *mut FILE) -> __off64_t;
}
extern "C" {
    pub fn fgetpos64(__stream: *mut FILE, __pos: *mut fpos64_t) -> c_int;
}
extern "C" {
    pub fn fsetpos64(__stream: *mut FILE, __pos: *const fpos64_t) -> c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn perror(__s: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}sys_nerr"]
    pub static mut sys_nerr: c_int;
}
extern "C" {
    #[link_name = "\u{1}sys_errlist"]
    pub static mut sys_errlist: [*const c_char; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_sys_nerr"]
    pub static mut _sys_nerr: c_int;
}
extern "C" {
    #[link_name = "\u{1}_sys_errlist"]
    pub static mut _sys_errlist: [*const c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn popen(__command: *const c_char, __modes: *const c_char) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> c_int;
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
    pub fn obstack_printf(__obstack: *mut obstack, __format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const c_char,
        __args: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: c_int) -> c_int;
}
pub type wint_t = c_uint;
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
    pub fn wcscasecmp_l(__s1: *const u32, __s2: *const u32, __loc: locale_t) -> c_int;
}
extern "C" {
    pub fn wcsncasecmp_l(__s1: *const u32, __s2: *const u32, __n: usize, __loc: locale_t) -> c_int;
}
extern "C" {
    pub fn wcscoll(__s1: *const u32, __s2: *const u32) -> c_int;
}
extern "C" {
    pub fn wcsxfrm(__s1: *mut u32, __s2: *const u32, __n: usize) -> usize;
}
extern "C" {
    pub fn wcscoll_l(__s1: *const u32, __s2: *const u32, __loc: locale_t) -> c_int;
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
    pub fn wcstok(__s: *mut u32, __delim: *const u32, __ptr: *mut u32) -> *mut u32;
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
    pub fn btowc(__c: c_int) -> wint_t;
}
extern "C" {
    pub fn wctob(__c: wint_t) -> c_int;
}
extern "C" {
    pub fn mbsinit(__ps: *const mbstate_t) -> c_int;
}
extern "C" {
    pub fn mbrtowc(__pwc: *mut u32, __s: *const c_char, __n: usize, __p: *mut mbstate_t) -> usize;
}
extern "C" {
    pub fn wcrtomb(__s: *mut c_char, __wc: u32, __ps: *mut mbstate_t) -> usize;
}
extern "C" {
    pub fn __mbrlen(__s: *const c_char, __n: usize, __ps: *mut mbstate_t) -> usize;
}
extern "C" {
    pub fn mbrlen(__s: *const c_char, __n: usize, __ps: *mut mbstate_t) -> usize;
}
extern "C" {
    pub fn mbsrtowcs(
        __dst: *mut u32,
        __src: *mut *const c_char,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsrtombs(
        __dst: *mut c_char,
        __src: *mut *const u32,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn mbsnrtowcs(
        __dst: *mut u32,
        __src: *mut *const c_char,
        __nmc: usize,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcsnrtombs(
        __dst: *mut c_char,
        __src: *mut *const u32,
        __nwc: usize,
        __len: usize,
        __ps: *mut mbstate_t,
    ) -> usize;
}
extern "C" {
    pub fn wcwidth(__c: u32) -> c_int;
}
extern "C" {
    pub fn wcswidth(__s: *const u32, __n: usize) -> c_int;
}
extern "C" {
    pub fn wcstod(__nptr: *const u32, __endptr: *mut u32) -> f64;
}
extern "C" {
    pub fn wcstof(__nptr: *const u32, __endptr: *mut u32) -> f32;
}
extern "C" {
    pub fn wcstold(__nptr: *const u32, __endptr: *mut u32) -> f64;
}
extern "C" {
    pub fn wcstof32(__nptr: *const u32, __endptr: *mut u32) -> _Float32;
}
extern "C" {
    pub fn wcstof64(__nptr: *const u32, __endptr: *mut u32) -> _Float64;
}
extern "C" {
    pub fn wcstof32x(__nptr: *const u32, __endptr: *mut u32) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x(__nptr: *const u32, __endptr: *mut u32) -> _Float64x;
}
extern "C" {
    pub fn wcstol(__nptr: *const u32, __endptr: *mut u32, __base: c_int) -> c_long;
}
extern "C" {
    pub fn wcstoul(__nptr: *const u32, __endptr: *mut u32, __base: c_int) -> c_ulong;
}
extern "C" {
    pub fn wcstoll(__nptr: *const u32, __endptr: *mut u32, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn wcstoull(__nptr: *const u32, __endptr: *mut u32, __base: c_int) -> c_ulonglong;
}
extern "C" {
    pub fn wcstoq(__nptr: *const u32, __endptr: *mut u32, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn wcstouq(__nptr: *const u32, __endptr: *mut u32, __base: c_int) -> c_ulonglong;
}
extern "C" {
    pub fn wcstol_l(
        __nptr: *const u32,
        __endptr: *mut u32,
        __base: c_int,
        __loc: locale_t,
    ) -> c_long;
}
extern "C" {
    pub fn wcstoul_l(
        __nptr: *const u32,
        __endptr: *mut u32,
        __base: c_int,
        __loc: locale_t,
    ) -> c_ulong;
}
extern "C" {
    pub fn wcstoll_l(
        __nptr: *const u32,
        __endptr: *mut u32,
        __base: c_int,
        __loc: locale_t,
    ) -> c_longlong;
}
extern "C" {
    pub fn wcstoull_l(
        __nptr: *const u32,
        __endptr: *mut u32,
        __base: c_int,
        __loc: locale_t,
    ) -> c_ulonglong;
}
extern "C" {
    pub fn wcstod_l(__nptr: *const u32, __endptr: *mut u32, __loc: locale_t) -> f64;
}
extern "C" {
    pub fn wcstof_l(__nptr: *const u32, __endptr: *mut u32, __loc: locale_t) -> f32;
}
extern "C" {
    pub fn wcstold_l(__nptr: *const u32, __endptr: *mut u32, __loc: locale_t) -> f64;
}
extern "C" {
    pub fn wcstof32_l(__nptr: *const u32, __endptr: *mut u32, __loc: locale_t) -> _Float32;
}
extern "C" {
    pub fn wcstof64_l(__nptr: *const u32, __endptr: *mut u32, __loc: locale_t) -> _Float64;
}
extern "C" {
    pub fn wcstof32x_l(__nptr: *const u32, __endptr: *mut u32, __loc: locale_t) -> _Float32x;
}
extern "C" {
    pub fn wcstof64x_l(__nptr: *const u32, __endptr: *mut u32, __loc: locale_t) -> _Float64x;
}
extern "C" {
    pub fn wcpcpy(__dest: *mut u32, __src: *const u32) -> *mut u32;
}
extern "C" {
    pub fn wcpncpy(__dest: *mut u32, __src: *const u32, __n: usize) -> *mut u32;
}
extern "C" {
    pub fn open_wmemstream(__bufloc: *mut u32, __sizeloc: *mut usize) -> *mut __FILE;
}
extern "C" {
    pub fn fwide(__fp: *mut __FILE, __mode: c_int) -> c_int;
}
extern "C" {
    pub fn fwprintf(__stream: *mut __FILE, __format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn wprintf(__format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn swprintf(__s: *mut u32, __n: usize, __format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn vfwprintf(__s: *mut __FILE, __format: *const u32, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn vwprintf(__format: *const u32, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn vswprintf(
        __s: *mut u32,
        __n: usize,
        __format: *const u32,
        __arg: *mut __va_list_tag,
    ) -> c_int;
}
extern "C" {
    pub fn fwscanf(__stream: *mut __FILE, __format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn wscanf(__format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn swscanf(__s: *const u32, __format: *const u32, ...) -> c_int;
}
extern "C" {
    pub fn vfwscanf(__s: *mut __FILE, __format: *const u32, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn vwscanf(__format: *const u32, __arg: *mut __va_list_tag) -> c_int;
}
extern "C" {
    pub fn vswscanf(__s: *const u32, __format: *const u32, __arg: *mut __va_list_tag) -> c_int;
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
    pub fn fgetws(__ws: *mut u32, __n: c_int, __stream: *mut __FILE) -> *mut u32;
}
extern "C" {
    pub fn fputws(__ws: *const u32, __stream: *mut __FILE) -> c_int;
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
    pub fn fgetws_unlocked(__ws: *mut u32, __n: c_int, __stream: *mut __FILE) -> *mut u32;
}
extern "C" {
    pub fn fputws_unlocked(__ws: *const u32, __stream: *mut __FILE) -> c_int;
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
pub type int_fast8_t = c_schar;
pub type int_fast16_t = c_long;
pub type int_fast32_t = c_long;
pub type int_fast64_t = c_long;
pub type uint_fast8_t = c_uchar;
pub type uint_fast16_t = c_ulong;
pub type uint_fast32_t = c_ulong;
pub type uint_fast64_t = c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
            tinfo: *mut crate::chuck_parse_h_edited::std::type_info,
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
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
extern "C" {
    pub fn setlocale(__category: c_int, __locale: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn localeconv() -> *mut lconv;
}
extern "C" {
    pub fn newlocale(__category_mask: c_int, __locale: *const c_char, __base: locale_t)
        -> locale_t;
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
pub const _ISupper: _bindgen_ty_31 = 256;
pub const _ISlower: _bindgen_ty_31 = 512;
pub const _ISalpha: _bindgen_ty_31 = 1024;
pub const _ISdigit: _bindgen_ty_31 = 2048;
pub const _ISxdigit: _bindgen_ty_31 = 4096;
pub const _ISspace: _bindgen_ty_31 = 8192;
pub const _ISprint: _bindgen_ty_31 = 16384;
pub const _ISgraph: _bindgen_ty_31 = 32768;
pub const _ISblank: _bindgen_ty_31 = 1;
pub const _IScntrl: _bindgen_ty_31 = 2;
pub const _ISpunct: _bindgen_ty_31 = 4;
pub const _ISalnum: _bindgen_ty_31 = 8;
pub type _bindgen_ty_31 = u32;
extern "C" {
    pub fn __ctype_b_loc() -> *mut *const c_ushort;
}
extern "C" {
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
    pub fn isalnum_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isalpha_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn iscntrl_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isdigit_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn islower_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isgraph_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isprint_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn ispunct_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isspace_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isupper_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isxdigit_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn isblank_l(arg1: c_int, arg2: locale_t) -> c_int;
}
extern "C" {
    pub fn __tolower_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn tolower_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn __toupper_l(__c: c_int, __l: locale_t) -> c_int;
}
extern "C" {
    pub fn toupper_l(__c: c_int, __l: locale_t) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16usize],
}
extern "C" {
    pub fn __sched_cpucount(__setsize: usize, __setp: *const cpu_set_t) -> c_int;
}
extern "C" {
    pub fn __sched_cpualloc(__count: usize) -> *mut cpu_set_t;
}
extern "C" {
    pub fn __sched_cpufree(__set: *mut cpu_set_t);
}
extern "C" {
    pub fn sched_setparam(__pid: __pid_t, __param: *const sched_param) -> c_int;
}
extern "C" {
    pub fn sched_getparam(__pid: __pid_t, __param: *mut sched_param) -> c_int;
}
extern "C" {
    pub fn sched_setscheduler(
        __pid: __pid_t,
        __policy: c_int,
        __param: *const sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn sched_getscheduler(__pid: __pid_t) -> c_int;
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
    pub fn sched_rr_get_interval(__pid: __pid_t, __t: *mut timespec) -> c_int;
}
extern "C" {
    pub fn sched_setaffinity(
        __pid: __pid_t,
        __cpusetsize: usize,
        __cpuset: *const cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn sched_getaffinity(
        __pid: __pid_t,
        __cpusetsize: usize,
        __cpuset: *mut cpu_set_t,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timex {
    pub modes: c_uint,
    pub offset: __syscall_slong_t,
    pub freq: __syscall_slong_t,
    pub maxerror: __syscall_slong_t,
    pub esterror: __syscall_slong_t,
    pub status: c_int,
    pub constant: __syscall_slong_t,
    pub precision: __syscall_slong_t,
    pub tolerance: __syscall_slong_t,
    pub time: timeval,
    pub tick: __syscall_slong_t,
    pub ppsfreq: __syscall_slong_t,
    pub jitter: __syscall_slong_t,
    pub shift: c_int,
    pub stabil: __syscall_slong_t,
    pub jitcnt: __syscall_slong_t,
    pub calcnt: __syscall_slong_t,
    pub errcnt: __syscall_slong_t,
    pub stbcnt: __syscall_slong_t,
    pub tai: c_int,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 44usize], u8>,
}
impl timex {
    #[inline]
    pub fn new_bitfield_1() -> __BindgenBitfieldUnit<[u8; 44usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 44usize], u8> =
            Default::default();
        __bindgen_bitfield_unit
    }
}
extern "C" {
    pub fn clock_adjtime(__clock_id: __clockid_t, __utx: *mut timex) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
        __s: *mut c_char,
        __maxsize: usize,
        __format: *const c_char,
        __tp: *const tm,
    ) -> usize;
}
extern "C" {
    pub fn strptime(__s: *const c_char, __fmt: *const c_char, __tp: *mut tm) -> *mut c_char;
}
extern "C" {
    pub fn strftime_l(
        __s: *mut c_char,
        __maxsize: usize,
        __format: *const c_char,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strptime_l(
        __s: *const c_char,
        __fmt: *const c_char,
        __tp: *mut tm,
        __loc: locale_t,
    ) -> *mut c_char;
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
    pub fn asctime(__tp: *const tm) -> *mut c_char;
}
extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut c_char;
}
extern "C" {
    pub fn asctime_r(__tp: *const tm, __buf: *mut c_char) -> *mut c_char;
}
extern "C" {
    pub fn ctime_r(__timer: *const time_t, __buf: *mut c_char) -> *mut c_char;
}
extern "C" {
    #[link_name = "\u{1}__tzname"]
    pub static mut __tzname: [*mut c_char; 2usize];
}
extern "C" {
    #[link_name = "\u{1}__daylight"]
    pub static mut __daylight: c_int;
}
extern "C" {
    #[link_name = "\u{1}__timezone"]
    pub static mut __timezone: c_long;
}
extern "C" {
    #[link_name = "\u{1}tzname"]
    pub static mut tzname: [*mut c_char; 2usize];
}
extern "C" {
    pub fn tzset();
}
extern "C" {
    #[link_name = "\u{1}daylight"]
    pub static mut daylight: c_int;
}
extern "C" {
    #[link_name = "\u{1}timezone"]
    pub static mut timezone: c_long;
}
extern "C" {
    pub fn stime(__when: *const time_t) -> c_int;
}
extern "C" {
    pub fn timegm(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn timelocal(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn dysize(__year: c_int) -> c_int;
}
extern "C" {
    pub fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> c_int;
}
extern "C" {
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> c_int;
}
extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> c_int;
}
extern "C" {
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> c_int;
}
extern "C" {
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> c_int;
}
extern "C" {
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> c_int;
}
extern "C" {
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> c_int;
}
extern "C" {
    pub fn timer_delete(__timerid: timer_t) -> c_int;
}
extern "C" {
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> c_int;
}
extern "C" {
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> c_int;
}
extern "C" {
    pub fn timer_getoverrun(__timerid: timer_t) -> c_int;
}
extern "C" {
    pub fn timespec_get(__ts: *mut timespec, __base: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}getdate_err"]
    pub static mut getdate_err: c_int;
}
extern "C" {
    pub fn getdate(__string: *const c_char) -> *mut tm;
}
extern "C" {
    pub fn getdate_r(__string: *const c_char, __resbufp: *mut tm) -> c_int;
}
pub type __jmp_buf = [c_long; 8usize];
pub const PTHREAD_CREATE_JOINABLE: _bindgen_ty_32 = 0;
pub const PTHREAD_CREATE_DETACHED: _bindgen_ty_32 = 1;
pub type _bindgen_ty_32 = u32;
pub const PTHREAD_MUTEX_TIMED_NP: _bindgen_ty_33 = 0;
pub const PTHREAD_MUTEX_RECURSIVE_NP: _bindgen_ty_33 = 1;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: _bindgen_ty_33 = 2;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: _bindgen_ty_33 = 3;
pub const PTHREAD_MUTEX_NORMAL: _bindgen_ty_33 = 0;
pub const PTHREAD_MUTEX_RECURSIVE: _bindgen_ty_33 = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: _bindgen_ty_33 = 2;
pub const PTHREAD_MUTEX_DEFAULT: _bindgen_ty_33 = 0;
pub const PTHREAD_MUTEX_FAST_NP: _bindgen_ty_33 = 0;
pub type _bindgen_ty_33 = u32;
pub const PTHREAD_MUTEX_STALLED: _bindgen_ty_34 = 0;
pub const PTHREAD_MUTEX_STALLED_NP: _bindgen_ty_34 = 0;
pub const PTHREAD_MUTEX_ROBUST: _bindgen_ty_34 = 1;
pub const PTHREAD_MUTEX_ROBUST_NP: _bindgen_ty_34 = 1;
pub type _bindgen_ty_34 = u32;
pub const PTHREAD_PRIO_NONE: _bindgen_ty_35 = 0;
pub const PTHREAD_PRIO_INHERIT: _bindgen_ty_35 = 1;
pub const PTHREAD_PRIO_PROTECT: _bindgen_ty_35 = 2;
pub type _bindgen_ty_35 = u32;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: _bindgen_ty_36 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: _bindgen_ty_36 = 1;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: _bindgen_ty_36 = 2;
pub const PTHREAD_RWLOCK_DEFAULT_NP: _bindgen_ty_36 = 0;
pub type _bindgen_ty_36 = u32;
pub const PTHREAD_INHERIT_SCHED: _bindgen_ty_37 = 0;
pub const PTHREAD_EXPLICIT_SCHED: _bindgen_ty_37 = 1;
pub type _bindgen_ty_37 = u32;
pub const PTHREAD_SCOPE_SYSTEM: _bindgen_ty_38 = 0;
pub const PTHREAD_SCOPE_PROCESS: _bindgen_ty_38 = 1;
pub type _bindgen_ty_38 = u32;
pub const PTHREAD_PROCESS_PRIVATE: _bindgen_ty_39 = 0;
pub const PTHREAD_PROCESS_SHARED: _bindgen_ty_39 = 1;
pub type _bindgen_ty_39 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _pthread_cleanup_buffer {
    pub __routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __arg: *mut c_void,
    pub __canceltype: c_int,
    pub __prev: *mut _pthread_cleanup_buffer,
}
pub const PTHREAD_CANCEL_ENABLE: _bindgen_ty_40 = 0;
pub const PTHREAD_CANCEL_DISABLE: _bindgen_ty_40 = 1;
pub type _bindgen_ty_40 = u32;
pub const PTHREAD_CANCEL_DEFERRED: _bindgen_ty_41 = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: _bindgen_ty_41 = 1;
pub type _bindgen_ty_41 = u32;
extern "C" {
    pub fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>,
        __arg: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_exit(__retval: *mut c_void);
}
extern "C" {
    pub fn pthread_join(__th: pthread_t, __thread_return: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pthread_tryjoin_np(__th: pthread_t, __thread_return: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pthread_timedjoin_np(
        __th: pthread_t,
        __thread_return: *mut c_void,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_detach(__th: pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_self() -> pthread_t;
}
extern "C" {
    pub fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_init(__attr: *mut pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getdetachstate(
        __attr: *const pthread_attr_t,
        __detachstate: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setdetachstate(__attr: *mut pthread_attr_t, __detachstate: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getguardsize(
        __attr: *const pthread_attr_t,
        __guardsize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setguardsize(__attr: *mut pthread_attr_t, __guardsize: usize) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getschedparam(
        __attr: *const pthread_attr_t,
        __param: *mut sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setschedparam(
        __attr: *mut pthread_attr_t,
        __param: *const sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getschedpolicy(
        __attr: *const pthread_attr_t,
        __policy: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setschedpolicy(__attr: *mut pthread_attr_t, __policy: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getinheritsched(
        __attr: *const pthread_attr_t,
        __inherit: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setinheritsched(__attr: *mut pthread_attr_t, __inherit: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getscope(__attr: *const pthread_attr_t, __scope: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setscope(__attr: *mut pthread_attr_t, __scope: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstackaddr(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstackaddr(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut c_void,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstacksize(__attr: *mut pthread_attr_t, __stacksize: usize) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getstack(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut c_void,
        __stacksize: *mut usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setstack(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut c_void,
        __stacksize: usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_setaffinity_np(
        __attr: *mut pthread_attr_t,
        __cpusetsize: usize,
        __cpuset: *const cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_attr_getaffinity_np(
        __attr: *const pthread_attr_t,
        __cpusetsize: usize,
        __cpuset: *mut cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getattr_default_np(__attr: *mut pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_setattr_default_np(__attr: *const pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_getattr_np(__th: pthread_t, __attr: *mut pthread_attr_t) -> c_int;
}
extern "C" {
    pub fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: c_int,
        __param: *const sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut c_int,
        __param: *mut sched_param,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_setschedprio(__target_thread: pthread_t, __prio: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_getname_np(
        __target_thread: pthread_t,
        __buf: *mut c_char,
        __buflen: usize,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_setname_np(__target_thread: pthread_t, __name: *const c_char) -> c_int;
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
        __th: pthread_t,
        __cpusetsize: usize,
        __cpuset: *const cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_getaffinity_np(
        __th: pthread_t,
        __cpusetsize: usize,
        __cpuset: *mut cpu_set_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_once(
        __once_control: *mut pthread_once_t,
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
    pub fn pthread_cancel(__th: pthread_t) -> c_int;
}
extern "C" {
    pub fn pthread_testcancel();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__pthread_unwind_buf_t__bindgen_ty_1; 1usize],
    pub __pad: [*mut c_void; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_unwind_buf_t__bindgen_ty_1 {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __cancel_arg: *mut c_void,
    pub __do_it: c_int,
    pub __cancel_type: c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct __pthread_cleanup_class {
    pub __cancel_routine: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub __cancel_arg: *mut c_void,
    pub __do_it: c_int,
    pub __cancel_type: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
    _unused: [u8; 0],
}
extern "C" {
    pub fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_getprioceiling(
        __mutex: *const pthread_mutex_t,
        __prioceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_setprioceiling(
        __mutex: *mut pthread_mutex_t,
        __prioceiling: c_int,
        __old_ceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutex_consistent_np(__mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getpshared(
        __attr: *const pthread_mutexattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_settype(__attr: *mut pthread_mutexattr_t, __kind: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprotocol(
        __attr: *const pthread_mutexattr_t,
        __protocol: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprotocol(
        __attr: *mut pthread_mutexattr_t,
        __protocol: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getprioceiling(
        __attr: *const pthread_mutexattr_t,
        __prioceiling: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setprioceiling(
        __attr: *mut pthread_mutexattr_t,
        __prioceiling: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_getrobust_np(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_mutexattr_setrobust_np(
        __attr: *mut pthread_mutexattr_t,
        __robustness: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_tryrdlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_trywrlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_timedwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getpshared(
        __attr: *const pthread_rwlockattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setpshared(
        __attr: *mut pthread_rwlockattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_getkind_np(
        __attr: *const pthread_rwlockattr_t,
        __pref: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_rwlockattr_setkind_np(__attr: *mut pthread_rwlockattr_t, __pref: c_int)
        -> c_int;
}
extern "C" {
    pub fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t) -> c_int;
}
extern "C" {
    pub fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_getpshared(
        __attr: *const pthread_condattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_setpshared(__attr: *mut pthread_condattr_t, __pshared: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut __clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_spin_init(__lock: *mut pthread_spinlock_t, __pshared: c_int) -> c_int;
}
extern "C" {
    pub fn pthread_spin_destroy(__lock: *mut pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_init(
        __barrier: *mut pthread_barrier_t,
        __attr: *const pthread_barrierattr_t,
        __count: c_uint,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_destroy(__barrier: *mut pthread_barrier_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrier_wait(__barrier: *mut pthread_barrier_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_init(__attr: *mut pthread_barrierattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_destroy(__attr: *mut pthread_barrierattr_t) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_getpshared(
        __attr: *const pthread_barrierattr_t,
        __pshared: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_barrierattr_setpshared(
        __attr: *mut pthread_barrierattr_t,
        __pshared: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> c_int;
}
extern "C" {
    pub fn pthread_key_delete(__key: pthread_key_t) -> c_int;
}
extern "C" {
    pub fn pthread_getspecific(__key: pthread_key_t) -> *mut c_void;
}
extern "C" {
    pub fn pthread_setspecific(__key: pthread_key_t, __pointer: *const c_void) -> c_int;
}
extern "C" {
    pub fn pthread_getcpuclockid(__thread_id: pthread_t, __clock_id: *mut __clockid_t) -> c_int;
}
extern "C" {
    pub fn pthread_atfork(
        __prepare: Option<unsafe extern "C" fn()>,
        __parent: Option<unsafe extern "C" fn()>,
        __child: Option<unsafe extern "C" fn()>,
    ) -> c_int;
}
pub type __gthread_t = pthread_t;
pub type __gthread_key_t = pthread_key_t;
pub type __gthread_once_t = pthread_once_t;
pub type __gthread_mutex_t = pthread_mutex_t;
pub type __gthread_recursive_mutex_t = pthread_mutex_t;
pub type __gthread_cond_t = pthread_cond_t;
pub type __gthread_time_t = timespec;
pub type _Atomic_word = c_int;
extern "C" {
    pub fn __errno_location() -> *mut c_int;
}
extern "C" {
    #[link_name = "\u{1}program_invocation_name"]
    pub static mut program_invocation_name: *mut c_char;
}
extern "C" {
    #[link_name = "\u{1}program_invocation_short_name"]
    pub static mut program_invocation_short_name: *mut c_char;
}
pub type error_t = c_int;
extern "C" {
    #[link_name = "\u{1}g_program"]
    pub static mut g_program: a_Program;
}
extern "C" {
    pub fn yyparse() -> c_int;
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
    pub fn yy_scan_string(arg1: *const c_char) -> YY_BUFFER_STATE;
}
extern "C" {
    pub fn yy_delete_buffer(arg1: YY_BUFFER_STATE);
}
extern "C" {
    #[link_name = "\u{1}_Z11open_cat_ckPc"]
    pub fn open_cat_ck(filename: c_str) -> *mut FILE;
}
extern "C" {
    #[link_name = "\u{1}_Z11chuck_parsePKcP8_IO_FILES0_"]
    pub fn chuck_parse(fname: c_constr, fd: *mut FILE, code: c_constr) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_Z11reset_parsev"]
    pub fn reset_parse();
}
pub const SyntaxType_COMMA: SyntaxType = 0;
pub const SyntaxType_SEMICOLON: SyntaxType = 1;
pub const SyntaxType_DBLCOLON: SyntaxType = 2;
pub const SyntaxType_PAREN: SyntaxType = 3;
pub const SyntaxType_DOT: SyntaxType = 4;
pub const SyntaxType_CHUCK_OP: SyntaxType = 5;
pub const SyntaxType_OPERATOR: SyntaxType = 6;
pub const SyntaxType_KEYWORD: SyntaxType = 7;
pub const SyntaxType_DEBUG_PRINT: SyntaxType = 8;
pub const SyntaxType_SPORK: SyntaxType = 9;
pub const SyntaxType_INTEGER: SyntaxType = 10;
pub const SyntaxType_FLOATING: SyntaxType = 11;
pub const SyntaxType_STRING: SyntaxType = 12;
pub const SyntaxType_COMMENT: SyntaxType = 13;
pub const SyntaxType_OTHER: SyntaxType = 14;
pub const SyntaxType_NUM_SYNTAX_TYPES: SyntaxType = 15;
pub type SyntaxType = u32;
#[repr(C)]
pub struct SyntaxToken {
    pub token: string,
    pub type_: c_ulong,
    pub begin: SyntaxToken_size_type,
    pub end: SyntaxToken_size_type,
}
#[repr(C)]
pub struct SyntaxTokenList {
    pub list: Vec<f64>,
    pub howmany: SyntaxTokenList_size_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SyntaxQuery {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN11SyntaxQuery9parseLineERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEER15SyntaxTokenList"]
    pub fn SyntaxQuery_parseLine(
        this: *mut SyntaxQuery,
        line: *const string,
        tokens: *mut SyntaxTokenList,
    ) -> c_ulong;
}
impl SyntaxQuery {
    #[inline]
    pub unsafe fn parseLine(
        &mut self,
        line: *const string,
        tokens: *mut SyntaxTokenList,
    ) -> c_ulong {
        SyntaxQuery_parseLine(self, line, tokens)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_42 {
    pub _address: u8,
}
pub type iterator = crate::chuck_parse_h_edited::std::_Bit_iterator;
pub type size_type = usize;
pub type SyntaxTokenList_size_type = usize;
