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
///* ChucK shell implementation
#![feature(libc)]
use libc::*;
use crate::ck::def::defe::*;
use crate::ck::dynl::dynle::*;
use crate::ck::dynl::dynle::Chuck_DL_Api::*;
use crate::ck::err::erre::*;
use crate::ck::vm::vme::*;
use crate::ck::util::string::stringe::*;
use crate::dts::*;
use crate::sys::*;
use std::mem::MaybeUninit;
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
pub const UGEN_OP_PASS: i32 = -1;
pub const UGEN_OP_STOP: u32 = 0;
pub const UGEN_OP_TICK: u32 = 1;
pub const CK_DEBUG_MEMORY_MGMT: u32 = 0;
pub const CVM_MEM_STACK_SIZE: u32 = 65536;
pub const CVM_REG_STACK_SIZE: u32 = 16384;
pub type c_str = *mut c_char;
pub type c_constr = *const c_char;
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
    pub fn ck_vfprintf_stdout(format: *const c_char, args: *mut __va_list_tag);
}
extern "C" {
    pub fn ck_vfprintf_stderr(format: *const c_char, args: *mut __va_list_tag);
}
extern "C" {
    pub fn ck_set_stdout_callback(callback: Option<unsafe extern "C" fn(arg1: *const c_char)>);
}
extern "C" {
    pub fn ck_set_stderr_callback(callback: Option<unsafe extern "C" fn(arg1: *const c_char)>);
}
#[repr(C)]
pub struct ChuckOutStream {
    pub m_stream: crate::ck::util::string::basic_stringstream,
    pub m_callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_isErr: bool,
}
extern "C" {
    #[link_name = "\u{1}set_callback"]
    pub fn ChuckOutStream_set_callback(
        this: *mut ChuckOutStream,
        callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
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
impl ChuckOutStream {
    #[inline]
    pub unsafe fn set_callback(
        &mut self,
        callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) {
        ChuckOutStream_set_callback(self, callback)
    }
    #[inline]
    pub unsafe fn new(isErr: bool) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub stk_wvOutMap: HashMap::new,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
pub struct Chuck_VM_Object__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_VM_Object {
    pub vtable_: *const Chuck_VM_Object__bindgen_vtable,
    pub m_ref_count: c_ulong,
    pub m_pooled: c_ulong,
    pub m_locked: c_ulong,
    pub m_v_ref: *mut Vec<f64>,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub funcs: Vec<f64>,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub m_vector: Vec<f64>,
    pub m_map: HashMap::new,
    pub m_is_obj: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr(this: *mut Chuck_Array4, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr1(this: *mut Chuck_Array4, key: *const string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get(this: *mut Chuck_Array4, i: c_long, val: *mut c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get1(
        this: *mut Chuck_Array4,
        key: *const string,
        val: *mut c_ulong,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set(this: *mut Chuck_Array4, i: c_long, val: c_ulong) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set1(this: *mut Chuck_Array4, key: *const string, val: c_ulong) -> c_long;
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
    pub unsafe fn addr1(&mut self, key: *const string) -> c_ulong {
        Chuck_Array4_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: c_long, val: *mut c_ulong) -> c_long {
        Chuck_Array4_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(&mut self, key: *const string, val: *mut c_ulong) -> c_long {
        Chuck_Array4_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: c_long, val: c_ulong) -> c_long {
        Chuck_Array4_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(&mut self, key: *const string, val: c_ulong) -> c_long {
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_Array4_find(this: *mut c_void, key: *const string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array4_erase(this: *mut c_void, key: *const string) -> c_long;
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
    pub m_vector: Vec<f64>,
    pub m_map: HashMap::new,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array8_addr(this: *mut Chuck_Array8, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array8_addr1(this: *mut Chuck_Array8, key: *const string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array8_get(this: *mut Chuck_Array8, i: c_long, val: *mut f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array8_get1(this: *mut Chuck_Array8, key: *const string, val: *mut f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array8_set(this: *mut Chuck_Array8, i: c_long, val: f64) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array8_set1(this: *mut Chuck_Array8, key: *const string, val: f64) -> c_long;
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
    pub unsafe fn addr1(&mut self, key: *const string) -> c_ulong {
        Chuck_Array8_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: c_long, val: *mut f64) -> c_long {
        Chuck_Array8_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(&mut self, key: *const string, val: *mut f64) -> c_long {
        Chuck_Array8_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: c_long, val: f64) -> c_long {
        Chuck_Array8_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(&mut self, key: *const string, val: f64) -> c_long {
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_Array8_find(this: *mut c_void, key: *const string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array8_erase(this: *mut c_void, key: *const string) -> c_long;
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
    pub m_vector: Vec<f64>,
    pub m_map: HashMap::new,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array16_addr(this: *mut Chuck_Array16, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array16_addr1(this: *mut Chuck_Array16, key: *const string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array16_get(this: *mut Chuck_Array16, i: c_long, val: *mut t_CKCOMPLEX) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array16_get1(
        this: *mut Chuck_Array16,
        key: *const string,
        val: *mut t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array16_set(
        this: *mut Chuck_Array16,
        i: c_long,
        val: *const t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array16_set1(
        this: *mut Chuck_Array16,
        key: *const string,
        val: *const t_CKCOMPLEX,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array16_push_back(this: *mut Chuck_Array16, val: *const t_CKCOMPLEX) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array16_pop_back(this: *mut Chuck_Array16) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array16_back(this: *const Chuck_Array16, val: *mut t_CKCOMPLEX) -> c_long;
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
    pub unsafe fn addr1(&mut self, key: *const string) -> c_ulong {
        Chuck_Array16_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: c_long, val: *mut t_CKCOMPLEX) -> c_long {
        Chuck_Array16_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(&mut self, key: *const string, val: *mut t_CKCOMPLEX) -> c_long {
        Chuck_Array16_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: c_long, val: *const t_CKCOMPLEX) -> c_long {
        Chuck_Array16_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(&mut self, key: *const string, val: *const t_CKCOMPLEX) -> c_long {
        Chuck_Array16_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKCOMPLEX) -> c_long {
        Chuck_Array16_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array16_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKCOMPLEX) -> c_long {
        Chuck_Array16_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array16_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_Array16_find(this: *mut c_void, key: *const string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array16_erase(this: *mut c_void, key: *const string) -> c_long;
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
    pub m_vector: Vec<f64>,
    pub m_map: HashMap::new,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array24_addr(this: *mut Chuck_Array24, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array24_addr1(this: *mut Chuck_Array24, key: *const string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array24_get(this: *mut Chuck_Array24, i: c_long, val: *mut t_CKVEC3) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array24_get1(
        this: *mut Chuck_Array24,
        key: *const string,
        val: *mut t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array24_set(this: *mut Chuck_Array24, i: c_long, val: *const t_CKVEC3) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array24_set1(
        this: *mut Chuck_Array24,
        key: *const string,
        val: *const t_CKVEC3,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array24_push_back(this: *mut Chuck_Array24, val: *const t_CKVEC3) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array24_pop_back(this: *mut Chuck_Array24) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array24_back(this: *const Chuck_Array24, val: *mut t_CKVEC3) -> c_long;
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
    pub unsafe fn addr1(&mut self, key: *const string) -> c_ulong {
        Chuck_Array24_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: c_long, val: *mut t_CKVEC3) -> c_long {
        Chuck_Array24_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(&mut self, key: *const string, val: *mut t_CKVEC3) -> c_long {
        Chuck_Array24_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: c_long, val: *const t_CKVEC3) -> c_long {
        Chuck_Array24_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(&mut self, key: *const string, val: *const t_CKVEC3) -> c_long {
        Chuck_Array24_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKVEC3) -> c_long {
        Chuck_Array24_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array24_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKVEC3) -> c_long {
        Chuck_Array24_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array24_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_Array24_find(this: *mut c_void, key: *const string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array24_erase(this: *mut c_void, key: *const string) -> c_long;
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
    pub m_vector: Vec<f64>,
    pub m_map: HashMap::new,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array32_addr(this: *mut Chuck_Array32, i: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array32_addr1(this: *mut Chuck_Array32, key: *const string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array32_get(this: *mut Chuck_Array32, i: c_long, val: *mut t_CKVEC4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array32_get1(
        this: *mut Chuck_Array32,
        key: *const string,
        val: *mut t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array32_set(this: *mut Chuck_Array32, i: c_long, val: *const t_CKVEC4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array32_set1(
        this: *mut Chuck_Array32,
        key: *const string,
        val: *const t_CKVEC4,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array32_push_back(this: *mut Chuck_Array32, val: *const t_CKVEC4) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array32_pop_back(this: *mut Chuck_Array32) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array32_back(this: *const Chuck_Array32, val: *mut t_CKVEC4) -> c_long;
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
    pub unsafe fn addr1(&mut self, key: *const string) -> c_ulong {
        Chuck_Array32_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: c_long, val: *mut t_CKVEC4) -> c_long {
        Chuck_Array32_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(&mut self, key: *const string, val: *mut t_CKVEC4) -> c_long {
        Chuck_Array32_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: c_long, val: *const t_CKVEC4) -> c_long {
        Chuck_Array32_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(&mut self, key: *const string, val: *const t_CKVEC4) -> c_long {
        Chuck_Array32_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKVEC4) -> c_long {
        Chuck_Array32_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> c_long {
        Chuck_Array32_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKVEC4) -> c_long {
        Chuck_Array32_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: c_ulong, end: c_ulong) {
        Chuck_Array32_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: c_long) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_Array32_find(this: *mut c_void, key: *const string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array32_erase(this: *mut c_void, key: *const string) -> c_long;
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
    pub m_queue: deque,
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
    pub fn Chuck_String_c_str(this: *mut Chuck_String) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_String"]
    pub fn Chuck_String_Chuck_String(this: *mut Chuck_String, s: *const string);
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
    pub unsafe fn c_str(&mut self) -> *const c_char {
        Chuck_String_c_str(self)
    }
    #[inline]
    pub unsafe fn new(s: *const string) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub stringArg: string,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub m_io: crate::dts::fstream(std::fs::Path(&str)),
    pub m_dir: *mut DIR,
    pub m_dir_start: c_long,
    pub m_path: string,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_IO_File_open(this: *mut c_void, path: *const string, flags: c_long) -> c_ulong;
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
    pub fn Chuck_IO_File_readString(this: *mut c_void, str: *mut string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_File_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write(this: *mut c_void, val: *const string);
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
    pub m_callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_buffer: crate::ck::util::string::basic_stringstream,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Chout_set_output_callback(
        this: *mut Chuck_IO_Chout,
        fp: Option<unsafe extern "C" fn(arg1: *const c_char)>,
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
        fp: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) {
        Chuck_IO_Chout_set_output_callback(self, fp)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_IO_Chout_readString(this: *mut c_void, str: *mut string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Chout_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write(this: *mut c_void, val: *const string);
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
    pub m_callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    pub m_buffer: crate::ck::util::string::basic_stringstream,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Cherr_set_output_callback(
        this: *mut Chuck_IO_Cherr,
        fp: Option<unsafe extern "C" fn(arg1: *const c_char)>,
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
        fp: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    ) {
        Chuck_IO_Cherr_set_output_callback(self, fp)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_IO_Cherr_readString(this: *mut c_void, str: *mut string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Cherr_eof(this: *mut c_void) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write(this: *mut c_void, val: *const string);
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
pub struct Chuck_UAnaBlobProxy {
    _unused: [u8; 0],
}
pub mod Chuck_DL_Api {
    
    pub type Object = *mut c_void;
    pub type Type = *mut c_void;
    pub type String = *mut c_void;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Api {
        pub vm: *mut Chuck_DL_Api::Api_VMApi,
        pub object: *mut Chuck_DL_Api::Api_ObjectApi,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Api_VMApi {
        pub get_srate:
            Option<unsafe extern "C" fn(arg1: CK_DL_API, arg2: *mut Chuck_VM_Shred) -> c_ulong>,
    }
    extern "C" {
        #[link_name = "\u{1}VMApi"]
        pub fn Api_VMApi_VMApi(this: *mut Chuck_DL_Api::Api_VMApi);
    }
    impl Api_VMApi {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = MaybeUninit::uninitialized();
            Api_VMApi_VMApi(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Api_ObjectApi {
        pub get_type: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                arg2: *mut Chuck_VM_Shred,
                name: *mut string,
            ) -> Chuck_DL_Api::Type,
        >,
        pub create: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                arg2: *mut Chuck_VM_Shred,
                type_: Chuck_DL_Api::Type,
            ) -> Chuck_DL_Api::Object,
        >,
        pub create_string: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                arg2: *mut Chuck_VM_Shred,
                value: *mut string,
            ) -> Chuck_DL_Api::String,
        >,
        pub get_mvar_int: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: Chuck_DL_Api::Object,
                name: *mut string,
                value: *mut c_long,
            ) -> c_ulong,
        >,
        pub get_mvar_float: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: Chuck_DL_Api::Object,
                name: *mut string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_dur: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: Chuck_DL_Api::Object,
                name: *mut string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_time: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: Chuck_DL_Api::Object,
                name: *mut string,
                value: *mut f64,
            ) -> c_ulong,
        >,
        pub get_mvar_string: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: Chuck_DL_Api::Object,
                name: *mut string,
                value: *mut Chuck_DL_Api::String,
            ) -> c_ulong,
        >,
        pub get_mvar_object: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                object: Chuck_DL_Api::Object,
                name: *mut string,
                value: *mut Chuck_DL_Api::Object,
            ) -> c_ulong,
        >,
        pub set_string: Option<
            unsafe extern "C" fn(
                arg1: CK_DL_API,
                string: Chuck_DL_Api::String,
                value: *mut string,
            ) -> c_ulong,
        >,
    }
    extern "C" {
        #[link_name = "\u{1}ObjectApi"]
        pub fn Api_ObjectApi_ObjectApi(this: *mut Chuck_DL_Api::Api_ObjectApi);
    }
    impl Api_ObjectApi {
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = MaybeUninit::uninitialized();
            Api_ObjectApi_ObjectApi(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
    extern "C" {
        #[link_name = "\u{1}g_api"]
        pub static mut Api_g_api: Chuck_DL_Api::Api;
    }
    extern "C" {
        #[link_name = "\u{1}instance"]
        pub fn Api_instance() -> *const Chuck_DL_Api::Api;
    }
    extern "C" {
        #[link_name = "\u{1}Api"]
        pub fn Api_Api(this: *mut Chuck_DL_Api::Api);
    }
    impl Api {
        #[inline]
        pub unsafe fn instance() -> *const Chuck_DL_Api::Api {
            Api_instance()
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = MaybeUninit::uninitialized();
            Api_Api(&mut __bindgen_tmp);
            __bindgen_tmp
        }
    }
}
pub type CK_DL_API = *const Chuck_DL_Api::Api;
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
    pub dll_name: string,
    pub curr_class: *mut Chuck_DL_Class,
    pub curr_func: *mut Chuck_DL_Func,
    pub name: string,
    pub classes: Vec<f64>,
    pub stack: Vec<f64>,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub ctors: Vec<f64>,
    pub dtor: *mut Chuck_DL_Func,
    pub mfuns: Vec<f64>,
    pub sfuns: Vec<f64>,
    pub mvars: Vec<f64>,
    pub svars: Vec<f64>,
    pub ugen_tick: f_tick,
    pub ugen_tickf: f_tickf,
    pub ugen_pmsg: f_pmsg,
    pub ugen_ctrl: Vec<f64>,
    pub uana_tock: f_tock,
    pub classes: Vec<f64>,
    pub current_mvar_offset: c_ulong,
    pub ugen_num_in: c_ulong,
    pub ugen_num_out: c_ulong,
    pub doc: string,
    pub examples: Vec<f64>,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub is_const: c_ulong,
    pub static_addr: *mut c_void,
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
        t: *const c_char,
        n: *const c_char,
        c: c_ulong,
        a: *mut c_void,
    );
}
impl Chuck_DL_Value {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        Chuck_DL_Value_Chuck_DL_Value(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(t: *const c_char, n: *const c_char, c: c_ulong, a: *mut c_void) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        Chuck_DL_Value_Chuck_DL_Value1(&mut __bindgen_tmp, t, n, c, a);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DL_Func {
    pub name: string,
    pub type_: string,
    pub __bindgen_anon_1: Chuck_DL_Func__bindgen_ty_1,
    pub args: Vec<f64>,
    pub doc: string,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        Chuck_DL_Func_Chuck_DL_Func(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(t: *const c_char, n: *const c_char, a: c_ulong) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub types: Vec<f64>,
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
impl Chuck_DL_Return {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        Chuck_DL_Return_Chuck_DL_Return(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_DLL {
    pub _base: Chuck_VM_Object,
    pub m_handle: *mut c_void,
    pub m_last_error: string,
    pub m_filename: string,
    pub m_id: string,
    pub m_func: string,
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        Chuck_DLL_Chuck_DLL(&mut __bindgen_tmp, carrier, xid);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_DLL_destructor"]
    pub fn Chuck_DLL_Chuck_DLL_destructor(this: *mut Chuck_DLL);
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
    pub fn Chuck_UGen_add(
        this: *mut Chuck_UGen,
        src: *mut Chuck_UGen,
        isUpChuck: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_UGen_remove(this: *mut Chuck_UGen, src: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_all"]
    pub fn Chuck_UGen_remove_all(this: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}set_max_src"]
    pub fn Chuck_UGen_set_max_src(this: *mut Chuck_UGen, num: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_num_src"]
    pub fn Chuck_UGen_get_num_src(this: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}is_connected_from"]
    pub fn Chuck_UGen_is_connected_from(this: *mut Chuck_UGen, src: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}disconnect"]
    pub fn Chuck_UGen_disconnect(this: *mut Chuck_UGen, recursive: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}system_tick"]
    pub fn Chuck_UGen_system_tick(this: *mut Chuck_UGen, now: f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}system_tick_v"]
    pub fn Chuck_UGen_system_tick_v(this: *mut Chuck_UGen, now: f64, numFrames: c_ulong)
        -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}alloc_v"]
    pub fn Chuck_UGen_alloc_v(this: *mut Chuck_UGen, size: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}src_chan"]
    pub fn Chuck_UGen_src_chan(this: *mut Chuck_UGen, chan: c_ulong) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}dst_for_src_chan"]
    pub fn Chuck_UGen_dst_for_src_chan(this: *mut Chuck_UGen, chan: c_ulong) -> *mut Chuck_UGen;
}
extern "C" {
    #[link_name = "\u{1}add_by"]
    pub fn Chuck_UGen_add_by(this: *mut Chuck_UGen, dest: *mut Chuck_UGen, isUpChuck: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}remove_by"]
    pub fn Chuck_UGen_remove_by(this: *mut Chuck_UGen, dest: *mut Chuck_UGen);
}
extern "C" {
    #[link_name = "\u{1}alloc_multi_chan"]
    pub fn Chuck_UGen_alloc_multi_chan(this: *mut Chuck_UGen, num_ins: c_ulong, num_outs: c_ulong);
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
impl Chuck_UGen {
    #[inline]
    pub unsafe fn add(&mut self, src: *mut Chuck_UGen, isUpChuck: c_ulong) -> c_ulong {
        Chuck_UGen_add(self, src, isUpChuck)
    }
    #[inline]
    pub unsafe fn remove(&mut self, src: *mut Chuck_UGen) -> c_ulong {
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
    pub unsafe fn is_connected_from(&mut self, src: *mut Chuck_UGen) -> c_ulong {
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
    pub unsafe fn src_chan(&mut self, chan: c_ulong) -> *mut Chuck_UGen {
        Chuck_UGen_src_chan(self, chan)
    }
    #[inline]
    pub unsafe fn dst_for_src_chan(&mut self, chan: c_ulong) -> *mut Chuck_UGen {
        Chuck_UGen_dst_for_src_chan(self, chan)
    }
    #[inline]
    pub unsafe fn add_by(&mut self, dest: *mut Chuck_UGen, isUpChuck: c_ulong) {
        Chuck_UGen_add_by(self, dest, isUpChuck)
    }
    #[inline]
    pub unsafe fn remove_by(&mut self, dest: *mut Chuck_UGen) {
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
    pub unsafe fn inlet(&mut self) -> *mut Chuck_UGen {
        Chuck_UGen_inlet(self)
    }
    #[inline]
    pub unsafe fn outlet(&mut self) -> *mut Chuck_UGen {
        Chuck_UGen_outlet(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub fn Chuck_UGen_init(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}done"]
    pub fn Chuck_UGen_done(this: *mut c_void);
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
    pub fn Chuck_UAna_system_tock(this: *mut Chuck_UAna, now: f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}is_up_connected_from"]
    pub fn Chuck_UAna_is_up_connected_from(this: *mut Chuck_UAna, src: *mut Chuck_UAna) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}numIncomingUAnae"]
    pub fn Chuck_UAna_numIncomingUAnae(this: *const Chuck_UAna) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}getIncomingUAna"]
    pub fn Chuck_UAna_getIncomingUAna(this: *const Chuck_UAna, index: c_ulong) -> *mut Chuck_UAna;
}
extern "C" {
    #[link_name = "\u{1}getIncomingBlob"]
    pub fn Chuck_UAna_getIncomingBlob(
        this: *const Chuck_UAna,
        index: c_ulong,
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
impl Chuck_UAna {
    #[inline]
    pub unsafe fn system_tock(&mut self, now: f64) -> c_ulong {
        Chuck_UAna_system_tock(self, now)
    }
    #[inline]
    pub unsafe fn is_up_connected_from(&mut self, src: *mut Chuck_UAna) -> c_ulong {
        Chuck_UAna_is_up_connected_from(self, src)
    }
    #[inline]
    pub unsafe fn numIncomingUAnae(&self) -> c_long {
        Chuck_UAna_numIncomingUAnae(self)
    }
    #[inline]
    pub unsafe fn getIncomingUAna(&self, index: c_ulong) -> *mut Chuck_UAna {
        Chuck_UAna_getIncomingUAna(self, index)
    }
    #[inline]
    pub unsafe fn getIncomingBlob(&self, index: c_ulong) -> *mut Chuck_UAnaBlobProxy {
        Chuck_UAna_getIncomingBlob(self, index)
    }
    #[inline]
    pub unsafe fn blobProxy(&self) -> *mut Chuck_UAnaBlobProxy {
        Chuck_UAna_blobProxy(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        Chuck_UAna_Chuck_UAna(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_UAna_destructor"]
    pub fn Chuck_UAna_Chuck_UAna_destructor(this: *mut Chuck_UAna);
}
extern "C" {
    pub fn ugen_generic_num_in(obj: *mut Chuck_Object, isArray: c_ulong) -> c_long;
}
extern "C" {
    pub fn ugen_generic_get_src(
        obj: *mut Chuck_Object,
        chan: c_long,
        isArray: c_ulong,
    ) -> *mut Chuck_UGen;
}
extern "C" {
    pub fn ugen_generic_get_dst(
        obj: *mut Chuck_Object,
        chan: c_long,
        isArray: c_ulong,
    ) -> *mut Chuck_UGen;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Instr {
    _unused: [u8; 0],
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
    pub stack: *mut c_uchar,
    pub sp: *mut c_uchar,
    pub sp_max: *mut c_uchar,
    pub prev: *mut Chuck_VM_Stack,
    pub next: *mut Chuck_VM_Stack,
    pub m_is_init: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_Stack_initialize(this: *mut Chuck_VM_Stack, size: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Stack_shutdown(this: *mut Chuck_VM_Stack) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Stack"]
    pub fn Chuck_VM_Stack_Chuck_VM_Stack(this: *mut Chuck_VM_Stack);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Stack_destructor"]
    pub fn Chuck_VM_Stack_Chuck_VM_Stack_destructor(this: *mut Chuck_VM_Stack);
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub instr: *mut Chuck_Instr,
    pub num_instr: c_ulong,
    pub name: string,
    pub stack_depth: c_ulong,
    pub need_this: c_ulong,
    pub native_func: c_ulong,
    pub native_func_type: c_ulong,
    pub filename: string,
}
pub const Chuck_VM_Code_NATIVE_UNKNOWN: Chuck_VM_Code__bindgen_ty_1 = 0;
pub const Chuck_VM_Code_NATIVE_CTOR: Chuck_VM_Code__bindgen_ty_1 = 1;
pub const Chuck_VM_Code_NATIVE_DTOR: Chuck_VM_Code__bindgen_ty_1 = 2;
pub const Chuck_VM_Code_NATIVE_MFUN: Chuck_VM_Code__bindgen_ty_1 = 3;
pub const Chuck_VM_Code_NATIVE_SFUN: Chuck_VM_Code__bindgen_ty_1 = 4;
pub type Chuck_VM_Code__bindgen_ty_1 = u32;
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Code"]
    pub fn Chuck_VM_Code_Chuck_VM_Code(this: *mut Chuck_VM_Code);
}
impl Chuck_VM_Code {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub instr: *mut Chuck_Instr,
    pub parent: *mut Chuck_VM_Shred,
    pub children: HashMap::new,
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
    pub m_ugen_map: HashMap::new,
    pub m_parent_objects: Vec<f64>,
    pub xid: c_ulong,
    pub name: string,
    pub args: Vec<f64>,
    pub prev: *mut Chuck_VM_Shred,
    pub next: *mut Chuck_VM_Shred,
    pub m_loopCounters: Vec<f64>,
    pub m_serials: *mut list,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn Chuck_VM_Shred_initialize(
        this: *mut Chuck_VM_Shred,
        c: *mut Chuck_VM_Code,
        mem_st_size: c_ulong,
        reg_st_size: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Shred_shutdown(this: *mut Chuck_VM_Shred) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn Chuck_VM_Shred_run(this: *mut Chuck_VM_Shred, vm: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add"]
    pub fn Chuck_VM_Shred_add(this: *mut Chuck_VM_Shred, ugen: *mut Chuck_UGen) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_VM_Shred_remove(this: *mut Chuck_VM_Shred, ugen: *mut Chuck_UGen) -> c_ulong;
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
    pub fn Chuck_VM_Shred_pushLoopCounter(this: *mut Chuck_VM_Shred) -> *mut c_ulong;
}
extern "C" {
    #[link_name = "\u{1}currentLoopCounter"]
    pub fn Chuck_VM_Shred_currentLoopCounter(this: *mut Chuck_VM_Shred) -> *mut c_ulong;
}
extern "C" {
    #[link_name = "\u{1}popLoopCounter"]
    pub fn Chuck_VM_Shred_popLoopCounter(this: *mut Chuck_VM_Shred) -> bool;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred"]
    pub fn Chuck_VM_Shred_Chuck_VM_Shred(this: *mut Chuck_VM_Shred);
}
impl Chuck_VM_Shred {
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        c: *mut Chuck_VM_Code,
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
    pub unsafe fn run(&mut self, vm: *mut Chuck_VM) -> c_ulong {
        Chuck_VM_Shred_run(self, vm)
    }
    #[inline]
    pub unsafe fn add(&mut self, ugen: *mut Chuck_UGen) -> c_ulong {
        Chuck_VM_Shred_add(self, ugen)
    }
    #[inline]
    pub unsafe fn remove(&mut self, ugen: *mut Chuck_UGen) -> c_ulong {
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub xid: c_ulong,
    pub name: string,
    pub start: f64,
    pub has_event: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shred_Status"]
    pub fn Chuck_VM_Shred_Status_Chuck_VM_Shred_Status(
        this: *mut Chuck_VM_Shred_Status,
        _id: c_ulong,
        n: *const string,
        _start: f64,
        e: c_ulong,
    );
}
impl Chuck_VM_Shred_Status {
    #[inline]
    pub unsafe fn new(_id: c_ulong, n: *const string, _start: f64, e: c_ulong) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub list: Vec<f64>,
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_VM_Status_clear(this: *mut Chuck_VM_Status);
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Status"]
    pub fn Chuck_VM_Status_Chuck_VM_Status(this: *mut Chuck_VM_Status);
}
impl Chuck_VM_Status {
    #[inline]
    pub unsafe fn clear(&mut self) {
        Chuck_VM_Status_clear(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
    pub rt_audio: c_ulong,
    pub vm_ref: *mut Chuck_VM,
    pub shred_list: *mut Chuck_VM_Shred,
    pub blocked: HashMap::new,
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
    pub fn Chuck_VM_Shreduler_initialize(this: *mut Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_Shreduler_shutdown(this: *mut Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shredule"]
    pub fn Chuck_VM_Shreduler_shredule(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shredule"]
    pub fn Chuck_VM_Shreduler_shredule1(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
        wake_time: f64,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_VM_Shreduler_get(this: *mut Chuck_VM_Shreduler) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}advance"]
    pub fn Chuck_VM_Shreduler_advance(this: *mut Chuck_VM_Shreduler, N: c_long);
}
extern "C" {
    #[link_name = "\u{1}advance_v"]
    pub fn Chuck_VM_Shreduler_advance_v(
        this: *mut Chuck_VM_Shreduler,
        num_left: *mut c_long,
        offset: *mut c_long,
    );
}
extern "C" {
    #[link_name = "\u{1}set_adaptive"]
    pub fn Chuck_VM_Shreduler_set_adaptive(this: *mut Chuck_VM_Shreduler, max_block_size: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_VM_Shreduler_remove(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}replace"]
    pub fn Chuck_VM_Shreduler_replace(
        this: *mut Chuck_VM_Shreduler,
        out: *mut Chuck_VM_Shred,
        in_: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}lookup"]
    pub fn Chuck_VM_Shreduler_lookup(
        this: *mut Chuck_VM_Shreduler,
        xid: c_ulong,
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
    pub fn Chuck_VM_Shreduler_highest(this: *mut Chuck_VM_Shreduler) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}add_blocked"]
    pub fn Chuck_VM_Shreduler_add_blocked(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_blocked"]
    pub fn Chuck_VM_Shreduler_remove_blocked(
        this: *mut Chuck_VM_Shreduler,
        shred: *mut Chuck_VM_Shred,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Shreduler"]
    pub fn Chuck_VM_Shreduler_Chuck_VM_Shreduler(this: *mut Chuck_VM_Shreduler);
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
    pub unsafe fn shredule(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_shredule(self, shred)
    }
    #[inline]
    pub unsafe fn shredule1(&mut self, shred: *mut Chuck_VM_Shred, wake_time: f64) -> c_ulong {
        Chuck_VM_Shreduler_shredule1(self, shred, wake_time)
    }
    #[inline]
    pub unsafe fn get(&mut self) -> *mut Chuck_VM_Shred {
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
    pub unsafe fn remove(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_remove(self, shred)
    }
    #[inline]
    pub unsafe fn replace(
        &mut self,
        out: *mut Chuck_VM_Shred,
        in_: *mut Chuck_VM_Shred,
    ) -> c_ulong {
        Chuck_VM_Shreduler_replace(self, out, in_)
    }
    #[inline]
    pub unsafe fn lookup(&mut self, xid: c_ulong) -> *mut Chuck_VM_Shred {
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
    pub unsafe fn highest(&mut self) -> c_ulong {
        Chuck_VM_Shreduler_highest(self)
    }
    #[inline]
    pub unsafe fn add_blocked(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_add_blocked(self, shred)
    }
    #[inline]
    pub unsafe fn remove_blocked(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
        Chuck_VM_Shreduler_remove_blocked(self, shred)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
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
pub const Chuck_Global_Request_Type_set_global_int_request: Chuck_Global_Request_Type = 0;
pub const Chuck_Global_Request_Type_get_global_int_request: Chuck_Global_Request_Type = 1;
pub const Chuck_Global_Request_Type_set_global_float_request: Chuck_Global_Request_Type = 2;
pub const Chuck_Global_Request_Type_get_global_float_request: Chuck_Global_Request_Type = 3;
pub const Chuck_Global_Request_Type_signal_global_event_request: Chuck_Global_Request_Type = 4;
pub const Chuck_Global_Request_Type_spork_shred_request: Chuck_Global_Request_Type = 5;
pub type Chuck_Global_Request_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Chuck_Global_Request {
    pub type_: Chuck_Global_Request_Type,
    pub __bindgen_anon_1: Chuck_Global_Request__bindgen_ty_1,
}
#[repr(C)]
#[repr(align(8))]
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
pub const Chuck_Msg_Type_MSG_ADD: Chuck_Msg_Type = 1;
pub const Chuck_Msg_Type_MSG_REMOVE: Chuck_Msg_Type = 2;
pub const Chuck_Msg_Type_MSG_REMOVEALL: Chuck_Msg_Type = 3;
pub const Chuck_Msg_Type_MSG_REPLACE: Chuck_Msg_Type = 4;
pub const Chuck_Msg_Type_MSG_STATUS: Chuck_Msg_Type = 5;
pub const Chuck_Msg_Type_MSG_PAUSE: Chuck_Msg_Type = 6;
pub const Chuck_Msg_Type_MSG_KILL: Chuck_Msg_Type = 7;
pub const Chuck_Msg_Type_MSG_TIME: Chuck_Msg_Type = 8;
pub const Chuck_Msg_Type_MSG_RESET_ID: Chuck_Msg_Type = 9;
pub const Chuck_Msg_Type_MSG_DONE: Chuck_Msg_Type = 10;
pub const Chuck_Msg_Type_MSG_ABORT: Chuck_Msg_Type = 11;
pub const Chuck_Msg_Type_MSG_ERROR: Chuck_Msg_Type = 12;
pub const Chuck_Msg_Type_MSG_CLEARVM: Chuck_Msg_Type = 13;
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
    pub args: *mut Vec<f64>,
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Msg_clear(this: *mut Chuck_Msg);
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Msg_set(this: *mut Chuck_Msg, vargs: *const Vec<f64>);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Msg"]
    pub fn Chuck_Msg_Chuck_Msg(this: *mut Chuck_Msg);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Msg_destructor"]
    pub fn Chuck_Msg_Chuck_Msg_destructor(this: *mut Chuck_Msg);
}
impl Chuck_Msg {
    #[inline]
    pub unsafe fn clear(&mut self) {
        Chuck_Msg_clear(self)
    }
    #[inline]
    pub unsafe fn set(&mut self, vargs: *const Vec<f64>) {
        Chuck_Msg_set(self, vargs)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        Chuck_Msg_Chuck_Msg(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_Msg_Chuck_Msg_destructor(self)
    }
}
pub type Chuck_Shell_Request = string;
pub type Chuck_Shell_Response = string;
#[repr(C)]
pub struct Chuck_Shell__bindgen_vtable(c_void);
//-----------------------------------------------------------------------------
// name: class Chuck_Shell
// desc: controller class for facilitating interaction between a shell UI and a
//      shell mode.
//-----------------------------------------------------------------------------
#[repr(C)]
pub struct Chuck_Shell {
    pub vtable_: *const Chuck_Shell__bindgen_vtable,
    pub vms: Vec<f64>,
    pub process_vm: *mut Chuck_VM,
    pub current_vm: *mut Chuck_Shell_VM,
    pub aliases: HashMap::new,
    pub variables: HashMap::new,
    pub commands: HashMap::new,
    pub allocated_commands: Vec<f64>,
    pub saved_code: HashMap::new,
    pub code: string,
    pub ui: *mut Chuck_Shell_UI,
    pub initialized: c_ulong,
    pub stop: c_ulong,
    pub prompt: string,
    pub in_: string,
    pub code_entry_active: c_ulong,
    pub scope: c_ulong,
}
#[repr(C)]
pub struct Chuck_Shell_Command__bindgen_vtable(c_void);
//-----------------------------------------------------------------------------
// name: class Chuck_Shell::Command
// desc: superclass to Chuck_Shell_Commands
//-----------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command {
    pub vtable_: *const Chuck_Shell_Command__bindgen_vtable,
    pub caller: *mut Chuck_Shell,
}
#[repr(C)]
pub struct Chuck_Shell_Command_VM {
    pub _base: Chuck_Shell_Command,
    pub commands: HashMap::new,
    pub allocated_commands: Vec<f64>,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_VMAdd {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_VMRemove {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_VMAttach {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_VMList {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_VMSwap {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_VMAttachAdd {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Add {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Remove {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Status {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Removeall {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Removelast {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Replace {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Kill {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Close {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Exit {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Ls {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Cd {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Pwd {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Alias {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Unalias {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Source {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
pub struct Chuck_Shell_Command_Code {
    pub _base: Chuck_Shell_Command,
    pub commands: HashMap::new,
    pub allocated_commands: Vec<f64>,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_CodeContext {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_CodeSave {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_CodeDelete {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_CodeAdd {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_CodeList {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_CodePrint {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_CodeWrite {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_CodeRead {
    pub _base: Chuck_Shell_Command,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_Command_Help {
    pub _base: Chuck_Shell_Command,
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn Chuck_Shell_init(
        this: *mut Chuck_Shell,
        process_vm: *mut Chuck_VM,
        arg1: *mut Chuck_Shell_UI,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn Chuck_Shell_run(this: *mut Chuck_Shell);
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_execute(
        this: *mut Chuck_Shell,
        arg1: *mut string,
        arg2: *mut string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_Shell_close(this: *mut Chuck_Shell);
}
extern "C" {
    #[link_name = "\u{1}exit"]
    pub fn Chuck_Shell_exit(this: *mut Chuck_Shell);
}
extern "C" {
    #[link_name = "\u{1}do_glob"]
    pub fn Chuck_Shell_do_glob(
        this: *mut Chuck_Shell,
        arg1: *const string,
        arg2: *mut string,
        arg3: *mut Vec<f64>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}do_code"]
    pub fn Chuck_Shell_do_code(
        this: *mut Chuck_Shell,
        arg1: *mut string,
        arg2: *mut string,
        command: string,
    );
}
extern "C" {
    #[link_name = "\u{1}do_code_context"]
    pub fn Chuck_Shell_do_code_context(this: *mut Chuck_Shell, arg1: *mut string);
}
extern "C" {
    #[link_name = "\u{1}do_aliases"]
    pub fn Chuck_Shell_do_aliases(this: *mut Chuck_Shell);
}
extern "C" {
    #[link_name = "\u{1}do_variables"]
    pub fn Chuck_Shell_do_variables(this: *mut Chuck_Shell);
}
extern "C" {
    #[link_name = "\u{1}start_code"]
    pub fn Chuck_Shell_start_code(this: *mut Chuck_Shell);
}
extern "C" {
    #[link_name = "\u{1}continue_code"]
    pub fn Chuck_Shell_continue_code(this: *mut Chuck_Shell, arg1: *mut string);
}
extern "C" {
    #[link_name = "\u{1}string_hash"]
    pub fn Chuck_Shell_string_hash(this: *mut Chuck_Shell, arg1: *mut string, arg2: *mut string);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Shell"]
    pub fn Chuck_Shell_Chuck_Shell(this: *mut Chuck_Shell);
}
impl Chuck_Shell {
    #[inline]
    pub unsafe fn init(&mut self, process_vm: *mut Chuck_VM, arg1: *mut Chuck_Shell_UI) -> c_ulong {
        Chuck_Shell_init(self, process_vm, arg1)
    }
    #[inline]
    pub unsafe fn run(&mut self) {
        Chuck_Shell_run(self)
    }
    #[inline]
    pub unsafe fn execute(&mut self, arg1: *mut string, arg2: *mut string) -> c_ulong {
        Chuck_Shell_execute(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn close(&mut self) {
        Chuck_Shell_close(self)
    }
    #[inline]
    pub unsafe fn exit(&mut self) {
        Chuck_Shell_exit(self)
    }
    ///* helper functions
    #[inline]
    pub unsafe fn do_glob(
        &mut self,
        arg1: *const string,
        arg2: *mut string,
        arg3: *mut Vec<f64>,
    ) -> c_ulong {
        Chuck_Shell_do_glob(self, arg1, arg2, arg3)
    }
    #[inline]
    pub unsafe fn do_code(&mut self, arg1: *mut string, arg2: *mut string, command: string) {
        Chuck_Shell_do_code(self, arg1, arg2, command)
    }
    #[inline]
    pub unsafe fn do_code_context(&mut self, arg1: *mut string) {
        Chuck_Shell_do_code_context(self, arg1)
    }
    #[inline]
    pub unsafe fn do_aliases(&mut self) {
        Chuck_Shell_do_aliases(self)
    }
    #[inline]
    pub unsafe fn do_variables(&mut self) {
        Chuck_Shell_do_variables(self)
    }
    #[inline]
    pub unsafe fn start_code(&mut self) {
        Chuck_Shell_start_code(self)
    }
    #[inline]
    pub unsafe fn continue_code(&mut self, arg1: *mut string) {
        Chuck_Shell_continue_code(self, arg1)
    }
    #[inline]
    pub unsafe fn string_hash(&mut self, arg1: *mut string, arg2: *mut string) {
        Chuck_Shell_string_hash(self, arg1, arg2)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        Chuck_Shell_Chuck_Shell(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Shell_destructor"]
    pub fn Chuck_Shell_Chuck_Shell_destructor(this: *mut Chuck_Shell);
}
extern "C" {
    #[link_name = "\u{1}Command_destructor"]
    pub fn Chuck_Shell_Command_Command_destructor(this: *mut Chuck_Shell_Command);
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn Chuck_Shell_Command_init(this: *mut c_void, arg1: *mut Chuck_Shell) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}long_usage"]
    pub fn Chuck_Shell_Command_long_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}Command_VM_destructor"]
    pub fn Chuck_Shell_Command_VM_Command_VM_destructor(this: *mut Chuck_Shell_Command_VM);
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn Chuck_Shell_Command_VM_init(this: *mut c_void, arg1: *mut Chuck_Shell) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_VM_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_VM_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_VMAdd_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_VMRemove_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_VMAttach_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_VMList_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_VMSwap_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_VMAttachAdd_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Add_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_Add_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Remove_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_Remove_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Status_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_Status_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Removeall_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_Removeall_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Removelast_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_Removelast_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Replace_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_Replace_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Kill_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Close_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Exit_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Ls_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Cd_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Pwd_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Alias_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Unalias_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Source_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}Command_Code_destructor"]
    pub fn Chuck_Shell_Command_Code_Command_Code_destructor(this: *mut Chuck_Shell_Command_Code);
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn Chuck_Shell_Command_Code_init(this: *mut c_void, arg1: *mut Chuck_Shell) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Code_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}usage"]
    pub fn Chuck_Shell_Command_Code_usage(this: *mut c_void) -> string;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_CodeContext_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_CodeSave_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_CodeDelete_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_CodeAdd_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_CodeList_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_CodePrint_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_CodeWrite_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_CodeRead_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Shell_Command_Help_execute(
        this: *mut c_void,
        arg1: *mut Vec<f64>,
        arg2: *mut string,
    ) -> c_long;
}
#[repr(C)]
pub struct Chuck_Shell_VM__bindgen_vtable(c_void);
//-----------------------------------------------------------------------------
// name: class Chuck_Shell_VM
// desc: encapsulates local and network VMs into a single class
//-----------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_VM {
    pub vtable_: *const Chuck_Shell_VM__bindgen_vtable,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Shell_VM_destructor"]
    pub fn Chuck_Shell_VM_Chuck_Shell_VM_destructor(this: *mut Chuck_Shell_VM);
}
//-----------------------------------------------------------------------------
// name: class Chuck_Shell_Network_VM
// desc: VM across the network
//-----------------------------------------------------------------------------
#[repr(C)]
pub struct Chuck_Shell_Network_VM {
    pub _base: Chuck_Shell_VM,
    pub hostname: string,
    pub port: c_long,
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn Chuck_Shell_Network_VM_init(
        this: *mut Chuck_Shell_Network_VM,
        arg1: *const string,
        arg2: c_long,
    ) -> c_ulong;
}
impl Chuck_Shell_Network_VM {
    #[inline]
    pub unsafe fn init(&mut self, arg1: *const string, arg2: c_long) -> c_ulong {
        Chuck_Shell_Network_VM_init(self, arg1, arg2)
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Shell_Network_VM_destructor"]
    pub fn Chuck_Shell_Network_VM_Chuck_Shell_Network_VM_destructor(
        this: *mut Chuck_Shell_Network_VM,
    );
}
extern "C" {
    #[link_name = "\u{1}copy"]
    pub fn Chuck_Shell_Network_VM_copy(this: *mut c_void) -> *mut Chuck_Shell_VM;
}
extern "C" {
    #[link_name = "\u{1}add_shred"]
    pub fn Chuck_Shell_Network_VM_add_shred(
        this: *mut c_void,
        arg1: *const Vec<f64>,
        arg2: *mut string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_shred"]
    pub fn Chuck_Shell_Network_VM_remove_shred(
        this: *mut c_void,
        arg1: *const Vec<f64>,
        arg2: *mut string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_all"]
    pub fn Chuck_Shell_Network_VM_remove_all(this: *mut c_void, arg1: *mut string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}remove_last"]
    pub fn Chuck_Shell_Network_VM_remove_last(this: *mut c_void, arg1: *mut string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}replace_shred"]
    pub fn Chuck_Shell_Network_VM_replace_shred(
        this: *mut c_void,
        arg1: *const Vec<f64>,
        arg2: *mut string,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}status"]
    pub fn Chuck_Shell_Network_VM_status(this: *mut c_void, arg1: *mut string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}kill"]
    pub fn Chuck_Shell_Network_VM_kill(this: *mut c_void, arg1: *mut string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}fullname"]
    pub fn Chuck_Shell_Network_VM_fullname(this: *mut c_void) -> string;
}
#[repr(C)]
pub struct Chuck_Shell_UI__bindgen_vtable(c_void);
//-----------------------------------------------------------------------------
// name: class Chuck_Shell_UI
// desc: superclass to various types of Chuck UIs
//-----------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Shell_UI {
    pub vtable_: *const Chuck_Shell_UI__bindgen_vtable,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Shell_UI_destructor"]
    pub fn Chuck_Shell_UI_Chuck_Shell_UI_destructor(this: *mut Chuck_Shell_UI);
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn Chuck_Shell_UI_init(this: *mut c_void) -> c_ulong;
}
