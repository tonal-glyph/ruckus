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
use crate::chuck_carrier_h_edited::*;
use crate::chuck_oo_h_edited::*;
use crate::chuck_stats_h_edited::*;
use crate::chuck_ugen_h_edited::*;
use crate::util_buffers_h_edited::*;
///* ChucK VM implementation, including the ChucK 'shreduler', and data/functions
///* for manipulating ChucK 'shreds'. The VM is a strict interpreter over the ChucK
///* instructions emitted in earlier stages. It also cooperates with the ChucK
///* shreduler for shreduling the concurrent shreds, which are user-level constructs.
///* Additionally, the VM also coordinates with a real-time native audio engine and
///* a native graphics engine (currently, this is a wrapper for a subset of the OpenGL
///* API, available in ChucK, with all the timing benefits of the language).
use libc::*;
pub const CK_DEBUG_MEMORY_MGMT: u32 = 0;
//-----------------------------------------------------------------------------
// vm defines
//-----------------------------------------------------------------------------
pub const CVM_MEM_STACK_SIZE: u32 = 65536;
pub const CVM_REG_STACK_SIZE: u32 = 16384;
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
        let mut __bindgen_tmp = uninitialized();
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
    pub children: map,
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
    pub m_ugen_map: map,
    pub m_parent_objects: vector,
    pub xid: c_ulong,
    pub name: string,
    pub args: vector,
    pub prev: *mut Chuck_VM_Shred,
    pub next: *mut Chuck_VM_Shred,
    pub m_loopCounters: vector,
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
        let mut __bindgen_tmp = uninitialized();
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
        let mut __bindgen_tmp = uninitialized();
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
    pub fn Chuck_VM_Status_Chuck_VM_Status_destructor(this: *mut Chuck_VM_Status);
}
#[repr(C)]
pub struct Chuck_VM_Shreduler {
    pub _base: Chuck_Object,
    pub now_system: f64,
    pub rt_audio: c_ulong,
    pub vm_ref: *mut Chuck_VM,
    pub shred_list: *mut Chuck_VM_Shred,
    pub blocked: map,
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
        let mut __bindgen_tmp = uninitialized();
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
#[repr(C)]
pub struct Chuck_VM {
    pub _base: Chuck_Object,
    pub m_carrier: *mut Chuck_Carrier,
    pub m_adc: *mut Chuck_UGen,
    pub m_dac: *mut Chuck_UGen,
    pub m_bunghole: *mut Chuck_UGen,
    pub m_srate: c_ulong,
    pub m_num_adc_channels: c_ulong,
    pub m_num_dac_channels: c_ulong,
    pub m_halt: c_ulong,
    pub m_is_running: c_ulong,
    pub m_input_ref: *const f32,
    pub m_output_ref: *mut f32,
    pub m_init: c_ulong,
    pub m_last_error: string,
    pub m_shreds: *mut Chuck_VM_Shred,
    pub m_num_shreds: c_ulong,
    pub m_shred_id: c_ulong,
    pub m_shreduler: *mut Chuck_VM_Shreduler,
    pub m_shred_dump: vector,
    pub m_num_dumped_shreds: c_ulong,
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
        srate: c_ulong,
        dac_chan: c_ulong,
        adc_chan: c_ulong,
        adaptive: c_ulong,
        halt: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}initialize_synthesis"]
    pub fn Chuck_VM_initialize_synthesis(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}setCarrier"]
    pub fn Chuck_VM_setCarrier(this: *mut Chuck_VM, c: *mut Chuck_Carrier) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_VM_shutdown(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}has_init"]
    pub fn Chuck_VM_has_init(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}start"]
    pub fn Chuck_VM_start(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}running"]
    pub fn Chuck_VM_running(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}stop"]
    pub fn Chuck_VM_stop(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}runningState"]
    pub fn Chuck_VM_runningState(this: *mut Chuck_VM) -> *mut c_ulong;
}
extern "C" {
    #[link_name = "\u{1}spork"]
    pub fn Chuck_VM_spork(
        this: *mut Chuck_VM,
        code: *mut Chuck_VM_Code,
        parent: *mut Chuck_VM_Shred,
        immediate: c_ulong,
    ) -> *mut Chuck_VM_Shred;
}
extern "C" {
    #[link_name = "\u{1}shreduler"]
    pub fn Chuck_VM_shreduler(this: *const Chuck_VM) -> *mut Chuck_VM_Shreduler;
}
extern "C" {
    #[link_name = "\u{1}next_id"]
    pub fn Chuck_VM_next_id(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}srate"]
    pub fn Chuck_VM_srate(this: *const Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}run"]
    pub fn Chuck_VM_run(
        this: *mut Chuck_VM,
        numFrames: c_long,
        input: *const f32,
        output: *mut f32,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}compute"]
    pub fn Chuck_VM_compute(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}abort_current_shred"]
    pub fn Chuck_VM_abort_current_shred(this: *mut Chuck_VM) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}invoke_static"]
    pub fn Chuck_VM_invoke_static(this: *mut Chuck_VM, shred: *mut Chuck_VM_Shred) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}gc"]
    pub fn Chuck_VM_gc(this: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}gc"]
    pub fn Chuck_VM_gc1(this: *mut Chuck_VM, amount: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}queue_msg"]
    pub fn Chuck_VM_queue_msg(this: *mut Chuck_VM, msg: *mut Chuck_Msg, num_msg: c_int) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_event"]
    pub fn Chuck_VM_queue_event(
        this: *mut Chuck_VM,
        event: *mut Chuck_Event,
        num_msg: c_int,
        buffer: *mut CBufferSimple,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}process_msg"]
    pub fn Chuck_VM_process_msg(this: *mut Chuck_VM, msg: *mut Chuck_Msg) -> c_ulong;
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
    pub fn Chuck_VM_last_error(this: *const Chuck_VM) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}get_global_int"]
    pub fn Chuck_VM_get_global_int(
        this: *mut Chuck_VM,
        name: string,
        callback: Option<unsafe extern "C" fn(arg1: c_long)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_global_int"]
    pub fn Chuck_VM_set_global_int(this: *mut Chuck_VM, name: string, val: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_float"]
    pub fn Chuck_VM_get_global_float(
        this: *mut Chuck_VM,
        name: string,
        callback: Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}set_global_float"]
    pub fn Chuck_VM_set_global_float(this: *mut Chuck_VM, name: string, val: f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signal_global_event"]
    pub fn Chuck_VM_signal_global_event(this: *mut Chuck_VM, name: string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}broadcast_global_event"]
    pub fn Chuck_VM_broadcast_global_event(this: *mut Chuck_VM, name: string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}init_global_int"]
    pub fn Chuck_VM_init_global_int(this: *mut Chuck_VM, name: string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_int_value"]
    pub fn Chuck_VM_get_global_int_value(this: *mut Chuck_VM, name: string) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_int"]
    pub fn Chuck_VM_get_ptr_to_global_int(this: *mut Chuck_VM, name: string) -> *mut c_long;
}
extern "C" {
    #[link_name = "\u{1}init_global_float"]
    pub fn Chuck_VM_init_global_float(this: *mut Chuck_VM, name: string) -> c_ulong;
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
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get_global_event"]
    pub fn Chuck_VM_get_global_event(this: *mut Chuck_VM, name: string) -> *mut Chuck_Event;
}
extern "C" {
    #[link_name = "\u{1}get_ptr_to_global_event"]
    pub fn Chuck_VM_get_ptr_to_global_event(this: *mut Chuck_VM, name: string) -> *mut Chuck_Event;
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
    pub fn Chuck_VM_input_ref(this: *mut Chuck_VM) -> *const f32;
}
extern "C" {
    #[link_name = "\u{1}output_ref"]
    pub fn Chuck_VM_output_ref(this: *mut Chuck_VM) -> *mut f32;
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
        cascade: c_ulong,
        dec: c_ulong,
    ) -> c_ulong;
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
    pub unsafe fn setCarrier(&mut self, c: *mut Chuck_Carrier) -> c_ulong {
        Chuck_VM_setCarrier(self, c)
    }
    #[inline]
    pub unsafe fn shutdown(&mut self) -> c_ulong {
        Chuck_VM_shutdown(self)
    }
    #[inline]
    pub unsafe fn has_init(&mut self) -> c_ulong {
        Chuck_VM_has_init(self)
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
    pub unsafe fn runningState(&mut self) -> *mut c_ulong {
        Chuck_VM_runningState(self)
    }
    #[inline]
    pub unsafe fn spork(
        &mut self,
        code: *mut Chuck_VM_Code,
        parent: *mut Chuck_VM_Shred,
        immediate: c_ulong,
    ) -> *mut Chuck_VM_Shred {
        Chuck_VM_spork(self, code, parent, immediate)
    }
    #[inline]
    pub unsafe fn shreduler(&self) -> *mut Chuck_VM_Shreduler {
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
        input: *const f32,
        output: *mut f32,
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
    pub unsafe fn invoke_static(&mut self, shred: *mut Chuck_VM_Shred) -> c_ulong {
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
    pub unsafe fn queue_msg(&mut self, msg: *mut Chuck_Msg, num_msg: c_int) -> c_ulong {
        Chuck_VM_queue_msg(self, msg, num_msg)
    }
    #[inline]
    pub unsafe fn queue_event(
        &mut self,
        event: *mut Chuck_Event,
        num_msg: c_int,
        buffer: *mut CBufferSimple,
    ) -> c_ulong {
        Chuck_VM_queue_event(self, event, num_msg, buffer)
    }
    #[inline]
    pub unsafe fn process_msg(&mut self, msg: *mut Chuck_Msg) -> c_ulong {
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
    pub unsafe fn last_error(&self) -> *const c_char {
        Chuck_VM_last_error(self)
    }
    #[inline]
    pub unsafe fn get_global_int(
        &mut self,
        name: string,
        callback: Option<unsafe extern "C" fn(arg1: c_long)>,
    ) -> c_ulong {
        Chuck_VM_get_global_int(self, name, callback)
    }
    #[inline]
    pub unsafe fn set_global_int(&mut self, name: string, val: c_long) -> c_ulong {
        Chuck_VM_set_global_int(self, name, val)
    }
    #[inline]
    pub unsafe fn get_global_float(
        &mut self,
        name: string,
        callback: Option<unsafe extern "C" fn(arg1: f64)>,
    ) -> c_ulong {
        Chuck_VM_get_global_float(self, name, callback)
    }
    #[inline]
    pub unsafe fn set_global_float(&mut self, name: string, val: f64) -> c_ulong {
        Chuck_VM_set_global_float(self, name, val)
    }
    #[inline]
    pub unsafe fn signal_global_event(&mut self, name: string) -> c_ulong {
        Chuck_VM_signal_global_event(self, name)
    }
    #[inline]
    pub unsafe fn broadcast_global_event(&mut self, name: string) -> c_ulong {
        Chuck_VM_broadcast_global_event(self, name)
    }
    #[inline]
    pub unsafe fn init_global_int(&mut self, name: string) -> c_ulong {
        Chuck_VM_init_global_int(self, name)
    }
    #[inline]
    pub unsafe fn get_global_int_value(&mut self, name: string) -> c_long {
        Chuck_VM_get_global_int_value(self, name)
    }
    #[inline]
    pub unsafe fn get_ptr_to_global_int(&mut self, name: string) -> *mut c_long {
        Chuck_VM_get_ptr_to_global_int(self, name)
    }
    #[inline]
    pub unsafe fn init_global_float(&mut self, name: string) -> c_ulong {
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
    pub unsafe fn init_global_event(&mut self, name: string, type_: *mut Chuck_Type) -> c_ulong {
        Chuck_VM_init_global_event(self, name, type_)
    }
    #[inline]
    pub unsafe fn get_global_event(&mut self, name: string) -> *mut Chuck_Event {
        Chuck_VM_get_global_event(self, name)
    }
    #[inline]
    pub unsafe fn get_ptr_to_global_event(&mut self, name: string) -> *mut Chuck_Event {
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
    pub unsafe fn input_ref(&mut self) -> *const f32 {
        Chuck_VM_input_ref(self)
    }
    #[inline]
    pub unsafe fn output_ref(&mut self) -> *mut f32 {
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
        cascade: c_ulong,
        dec: c_ulong,
    ) -> c_ulong {
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
        let mut __bindgen_tmp = uninitialized();
        Chuck_VM_Chuck_VM(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_destructor"]
    pub fn Chuck_VM_Chuck_VM_destructor(this: *mut Chuck_VM);
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
        let mut __bindgen_tmp = uninitialized();
        Chuck_Msg_Chuck_Msg(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Chuck_Msg_Chuck_Msg_destructor(self)
    }
}
