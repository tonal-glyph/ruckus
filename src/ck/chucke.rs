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
extern crate libc;
use libc::*;
use crate::chuck_carrier_h_edited::*;
use crate::chuck_compile_h_edited::*;
use crate::chuck_dl_h_edited::*;
use crate::chuck_shell_h_edited::*;
use crate::chuck_vm_h_edited::*;
use crate::hidio_sdl_h_edited::*;
use crate::midiio_rtmidi_h_edited::*;
use crate::ulib_machine_h_edited::*;
use crate::util_math_h_edited::*;
use crate::util_string_h_edited::*;
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
pub const CK_DEBUG_MEMORY_MGMT: u32 = 0;
pub const CVM_MEM_STACK_SIZE: u32 = 65536;
pub const CVM_REG_STACK_SIZE: u32 = 16384;
pub const CK_MAX_HID_DEVICES: u32 = 1024;
pub const MIDI_NOTEON: u32 = 144;
pub const MIDI_NOTEOFF: u32 = 128;
pub const MIDI_POLYPRESS: u32 = 160;
pub const MIDI_CTRLCHANGE: u32 = 176;
pub const MIDI_PROGCHANGE: u32 = 192;
pub const MIDI_CHANPRESS: u32 = 208;
pub const MIDI_PITCHBEND: u32 = 224;
pub const MIDI_ALLNOTESOFF: u32 = 123;
pub const CHUCK_PARAM_SAMPLE_RATE: &'static [u8; 12usize] = b"SAMPLE_RATE\0";
pub const CHUCK_PARAM_INPUT_CHANNELS: &'static [u8; 15usize] = b"INPUT_CHANNELS\0";
pub const CHUCK_PARAM_OUTPUT_CHANNELS: &'static [u8; 16usize] = b"OUTPUT_CHANNELS\0";
pub const CHUCK_PARAM_VM_ADAPTIVE: &'static [u8; 12usize] = b"VM_ADAPTIVE\0";
pub const CHUCK_PARAM_VM_HALT: &'static [u8; 8usize] = b"VM_HALT\0";
pub const CHUCK_PARAM_OTF_ENABLE: &'static [u8; 11usize] = b"OTF_ENABLE\0";
pub const CHUCK_PARAM_OTF_PORT: &'static [u8; 9usize] = b"OTF_PORT\0";
pub const CHUCK_PARAM_DUMP_INSTRUCTIONS: &'static [u8; 18usize] = b"DUMP_INSTRUCTIONS\0";
pub const CHUCK_PARAM_AUTO_DEPEND: &'static [u8; 12usize] = b"AUTO_DEPEND\0";
pub const CHUCK_PARAM_DEPRECATE_LEVEL: &'static [u8; 16usize] = b"DEPRECATE_LEVEL\0";
pub const CHUCK_PARAM_WORKING_DIRECTORY: &'static [u8; 18usize] = b"WORKING_DIRECTORY\0";
pub const CHUCK_PARAM_CHUGIN_ENABLE: &'static [u8; 14usize] = b"CHUGIN_ENABLE\0";
pub const CHUCK_PARAM_CHUGIN_DIRECTORY: &'static [u8; 17usize] = b"CHUGIN_DIRECTORY\0";
pub const CHUCK_PARAM_USER_CHUGINS: &'static [u8; 13usize] = b"USER_CHUGINS\0";
pub const CHUCK_PARAM_USER_CHUGIN_DIRECTORIES: &'static [u8; 24usize] =
    b"USER_CHUGIN_DIRECTORIES\0";
pub const CHUCK_PARAM_HINT_IS_REALTIME_AUDIO: &'static [u8; 23usize] = b"HINT_IS_REALTIME_AUDIO\0";
#[repr(C)]
pub struct ChucK__bindgen_vtable(libc::c_void);
#[repr(C)]
pub struct ChucK {
    pub vtable_: *const ChucK__bindgen_vtable,
    pub m_carrier: *mut Chuck_Carrier,
    pub m_params: crate::dts::map,
    pub m_listParams: crate::dts::map,
    pub m_init: libc::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}o_isGlobalInit"]
    pub static mut ChucK_o_isGlobalInit: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}enableSystemCall"]
    pub static mut ChucK_enableSystemCall: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}VERSION"]
    pub static mut ChucK_VERSION: [libc::c_char; 0usize];
}
extern "C" {
    #[link_name = "\u{1}o_numVMs"]
    pub static mut ChucK_o_numVMs: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setParam"]
    pub fn ChucK_setParam(
        this: *mut ChucK,
        name: *const crate::chuck_parse_h_edited::std::string,
        value: libc::c_long,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}setParamFloat"]
    pub fn ChucK_setParamFloat(
        this: *mut ChucK,
        name: *const crate::chuck_parse_h_edited::std::string,
        value: f64,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}setParam"]
    pub fn ChucK_setParam1(
        this: *mut ChucK,
        name: *const crate::chuck_parse_h_edited::std::string,
        value: *const crate::chuck_parse_h_edited::std::string,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}setParam"]
    pub fn ChucK_setParam2(
        this: *mut ChucK,
        name: *const crate::chuck_parse_h_edited::std::string,
        value: *const crate::dts::list,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}getParamInt"]
    pub fn ChucK_getParamInt(
        this: *mut ChucK,
        key: *const crate::chuck_parse_h_edited::std::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}getParamFloat"]
    pub fn ChucK_getParamFloat(
        this: *mut ChucK,
        key: *const crate::chuck_parse_h_edited::std::string,
    ) -> f64;
}
extern "C" {
    #[link_name = "\u{1}getParamString"]
    pub fn ChucK_getParamString(
        this: *mut ChucK,
        key: *const crate::chuck_parse_h_edited::std::string,
    ) -> crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}getParamStringList"]
    pub fn ChucK_getParamStringList(
        this: *mut ChucK,
        key: *const crate::chuck_parse_h_edited::std::string,
    ) -> crate::dts::list;
}
extern "C" {
    #[link_name = "\u{1}compileFile"]
    pub fn ChucK_compileFile(
        this: *mut ChucK,
        path: *const crate::chuck_parse_h_edited::std::string,
        argsTogether: *const crate::chuck_parse_h_edited::std::string,
        count: libc::c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}compileCode"]
    pub fn ChucK_compileCode(
        this: *mut ChucK,
        code: *const crate::chuck_parse_h_edited::std::string,
        argsTogether: *const crate::chuck_parse_h_edited::std::string,
        count: libc::c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn ChucK_init(this: *mut ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}start"]
    pub fn ChucK_start(this: *mut ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn ChucK_run(this: *mut ChucK, input: *mut f32, output: *mut f32, numFrames: libc::c_int);
}
extern "C" {
    #[link_name = "\u{1}running"]
    pub fn ChucK_running(this: *mut ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}bind"]
    pub fn ChucK_bind(
        this: *mut ChucK,
        queryFunc: f_ck_query,
        name: *const crate::chuck_parse_h_edited::std::string,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}vm"]
    pub fn ChucK_vm(this: *mut ChucK) -> *mut Chuck_VM;
}
extern "C" {
    #[link_name = "\u{1}compiler"]
    pub fn ChucK_compiler(this: *mut ChucK) -> *mut Chuck_Compiler;
}
extern "C" {
    #[link_name = "\u{1}setGlobalInt"]
    pub fn ChucK_setGlobalInt(
        this: *mut ChucK,
        name: *const libc::c_char,
        val: libc::c_long,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}getGlobalInt"]
    pub fn ChucK_getGlobalInt(
        this: *mut ChucK,
        name: *const libc::c_char,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: libc::c_long)>,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setGlobalFloat"]
    pub fn ChucK_setGlobalFloat(
        this: *mut ChucK,
        name: *const libc::c_char,
        val: f64,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}getGlobalFloat"]
    pub fn ChucK_getGlobalFloat(
        this: *mut ChucK,
        name: *const libc::c_char,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signalGlobalEvent"]
    pub fn ChucK_signalGlobalEvent(this: *mut ChucK, name: *const libc::c_char) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}broadcastGlobalEvent"]
    pub fn ChucK_broadcastGlobalEvent(this: *mut ChucK, name: *const libc::c_char)
        -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setChoutCallback"]
    pub fn ChucK_setChoutCallback(
        this: *mut ChucK,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setCherrCallback"]
    pub fn ChucK_setCherrCallback(
        this: *mut ChucK,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setStdoutCallback"]
    pub fn ChucK_setStdoutCallback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setStderrCallback"]
    pub fn ChucK_setStderrCallback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}globalInit"]
    pub fn ChucK_globalInit() -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}globalCleanup"]
    pub fn ChucK_globalCleanup();
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn ChucK_shutdown(this: *mut ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}version"]
    pub fn ChucK_version() -> *const libc::c_char;
}
extern "C" {
    #[link_name = "\u{1}intSize"]
    pub fn ChucK_intSize() -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}numVMs"]
    pub fn ChucK_numVMs() -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}poop"]
    pub fn ChucK_poop();
}
extern "C" {
    #[link_name = "\u{1}setLogLevel"]
    pub fn ChucK_setLogLevel(level: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}getLogLevel"]
    pub fn ChucK_getLogLevel() -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}initDefaultParams"]
    pub fn ChucK_initDefaultParams(this: *mut ChucK);
}
extern "C" {
    #[link_name = "\u{1}initVM"]
    pub fn ChucK_initVM(this: *mut ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}initCompiler"]
    pub fn ChucK_initCompiler(this: *mut ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}initChugins"]
    pub fn ChucK_initChugins(this: *mut ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}initOTF"]
    pub fn ChucK_initOTF(this: *mut ChucK) -> bool;
}
extern "C" {
    #[link_name = "\u{1}ChucK"]
    pub fn ChucK_ChucK(this: *mut ChucK);
}
impl ChucK {
    #[inline]
    pub unsafe fn setParam(
        &mut self,
        name: *const crate::chuck_parse_h_edited::std::string,
        value: libc::c_long,
    ) -> bool {
        ChucK_setParam(self, name, value)
    }
    #[inline]
    pub unsafe fn setParamFloat(
        &mut self,
        name: *const crate::chuck_parse_h_edited::std::string,
        value: f64,
    ) -> bool {
        ChucK_setParamFloat(self, name, value)
    }
    #[inline]
    pub unsafe fn setParam1(
        &mut self,
        name: *const crate::chuck_parse_h_edited::std::string,
        value: *const crate::chuck_parse_h_edited::std::string,
    ) -> bool {
        ChucK_setParam1(self, name, value)
    }
    #[inline]
    pub unsafe fn setParam2(
        &mut self,
        name: *const crate::chuck_parse_h_edited::std::string,
        value: *const crate::dts::list,
    ) -> bool {
        ChucK_setParam2(self, name, value)
    }
    #[inline]
    pub unsafe fn getParamInt(
        &mut self,
        key: *const crate::chuck_parse_h_edited::std::string,
    ) -> libc::c_long {
        ChucK_getParamInt(self, key)
    }
    #[inline]
    pub unsafe fn getParamFloat(&mut self, key: *const crate::chuck_parse_h_edited::std::string) -> f64 {
        ChucK_getParamFloat(self, key)
    }
    #[inline]
    pub unsafe fn getParamString(
        &mut self,
        key: *const crate::chuck_parse_h_edited::std::string,
    ) -> crate::dts::string {
        ChucK_getParamString(self, key)
    }
    #[inline]
    pub unsafe fn getParamStringList(
        &mut self,
        key: *const crate::chuck_parse_h_edited::std::string,
    ) -> crate::dts::list {
        ChucK_getParamStringList(self, key)
    }
    #[inline]
    pub unsafe fn compileFile(
        &mut self,
        path: *const crate::chuck_parse_h_edited::std::string,
        argsTogether: *const crate::chuck_parse_h_edited::std::string,
        count: libc::c_int,
    ) -> bool {
        ChucK_compileFile(self, path, argsTogether, count)
    }
    #[inline]
    pub unsafe fn compileCode(
        &mut self,
        code: *const crate::chuck_parse_h_edited::std::string,
        argsTogether: *const crate::chuck_parse_h_edited::std::string,
        count: libc::c_int,
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
    pub unsafe fn run(&mut self, input: *mut f32, output: *mut f32, numFrames: libc::c_int) {
        ChucK_run(self, input, output, numFrames)
    }
    #[inline]
    pub unsafe fn running(&mut self) -> bool {
        ChucK_running(self)
    }
    #[inline]
    pub unsafe fn bind(
        &mut self,
        queryFunc: f_ck_query,
        name: *const crate::chuck_parse_h_edited::std::string,
    ) -> bool {
        ChucK_bind(self, queryFunc, name)
    }
    #[inline]
    pub unsafe fn vm(&mut self) -> *mut Chuck_VM {
        ChucK_vm(self)
    }
    #[inline]
    pub unsafe fn compiler(&mut self) -> *mut Chuck_Compiler {
        ChucK_compiler(self)
    }
    #[inline]
    pub unsafe fn setGlobalInt(
        &mut self,
        name: *const libc::c_char,
        val: libc::c_long,
    ) -> libc::c_ulong {
        ChucK_setGlobalInt(self, name, val)
    }
    #[inline]
    pub unsafe fn getGlobalInt(
        &mut self,
        name: *const libc::c_char,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: libc::c_long)>,
    ) -> libc::c_ulong {
        ChucK_getGlobalInt(self, name, callback)
    }
    #[inline]
    pub unsafe fn setGlobalFloat(&mut self, name: *const libc::c_char, val: f64) -> libc::c_ulong {
        ChucK_setGlobalFloat(self, name, val)
    }
    #[inline]
    pub unsafe fn getGlobalFloat(
        &mut self,
        name: *const libc::c_char,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> libc::c_ulong {
        ChucK_getGlobalFloat(self, name, callback)
    }
    #[inline]
    pub unsafe fn signalGlobalEvent(&mut self, name: *const libc::c_char) -> libc::c_ulong {
        ChucK_signalGlobalEvent(self, name)
    }
    #[inline]
    pub unsafe fn broadcastGlobalEvent(&mut self, name: *const libc::c_char) -> libc::c_ulong {
        ChucK_broadcastGlobalEvent(self, name)
    }
    #[inline]
    pub unsafe fn setChoutCallback(
        &mut self,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) -> libc::c_ulong {
        ChucK_setChoutCallback(self, callback)
    }
    #[inline]
    pub unsafe fn setCherrCallback(
        &mut self,
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) -> libc::c_ulong {
        ChucK_setCherrCallback(self, callback)
    }
    #[inline]
    pub unsafe fn setStdoutCallback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) -> libc::c_ulong {
        ChucK_setStdoutCallback(callback)
    }
    #[inline]
    pub unsafe fn setStderrCallback(
        callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) -> libc::c_ulong {
        ChucK_setStderrCallback(callback)
    }
    #[inline]
    pub unsafe fn globalInit() -> libc::c_ulong {
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
    pub unsafe fn version() -> *const libc::c_char {
        ChucK_version()
    }
    #[inline]
    pub unsafe fn intSize() -> libc::c_ulong {
        ChucK_intSize()
    }
    #[inline]
    pub unsafe fn numVMs() -> libc::c_ulong {
        ChucK_numVMs()
    }
    #[inline]
    pub unsafe fn poop() {
        ChucK_poop()
    }
    #[inline]
    pub unsafe fn setLogLevel(level: libc::c_long) {
        ChucK_setLogLevel(level)
    }
    #[inline]
    pub unsafe fn getLogLevel() -> libc::c_long {
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
    pub fn ChucK_ChucK_destructor(this: *mut ChucK);
}
