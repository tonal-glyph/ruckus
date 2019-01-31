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
use crate::chuck_def_h_edited::*;
use crate::chuck_oo_h_edited::*;
use crate::chuck_type_h_edited::*;
use crate::chuck_vm_h_edited::*;
use crate::util_buffers_h_edited::*;
use crate::util_thread_h_edited::*;
///* utility for open sound control
use libc::*;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UDP_Transmitter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UDP_Receiver {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OSC_Transmitter__bindgen_vtable(c_void);
#[repr(C)]
pub struct OSC_Transmitter {
    pub vtable_: *const OSC_Transmitter__bindgen_vtable,
    pub _buffer: [c_char; 2048usize],
    pub _osc: OSCbuf,
    pub _out: *mut UDP_Transmitter,
    pub _holdMessage: bool,
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn OSC_Transmitter_init(this: *mut OSC_Transmitter);
}
extern "C" {
    #[link_name = "\u{1}setHost"]
    pub fn OSC_Transmitter_setHost(this: *mut OSC_Transmitter, host: *mut c_char, port: c_int);
}
extern "C" {
    #[link_name = "\u{1}addMessage"]
    pub fn OSC_Transmitter_addMessage(
        this: *mut OSC_Transmitter,
        address: *mut c_char,
        args: *mut c_char,
        ...
    );
}
extern "C" {
    #[link_name = "\u{1}openBundle"]
    pub fn OSC_Transmitter_openBundle(this: *mut OSC_Transmitter, t: OSCTimeTag);
}
extern "C" {
    #[link_name = "\u{1}closeBundle"]
    pub fn OSC_Transmitter_closeBundle(this: *mut OSC_Transmitter);
}
extern "C" {
    #[link_name = "\u{1}startMessage"]
    pub fn OSC_Transmitter_startMessage(this: *mut OSC_Transmitter, spec: *mut c_char);
}
extern "C" {
    #[link_name = "\u{1}startMessage"]
    pub fn OSC_Transmitter_startMessage1(
        this: *mut OSC_Transmitter,
        address: *mut c_char,
        args: *mut c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}addInt"]
    pub fn OSC_Transmitter_addInt(this: *mut OSC_Transmitter, i: int4byte);
}
extern "C" {
    #[link_name = "\u{1}addFloat"]
    pub fn OSC_Transmitter_addFloat(this: *mut OSC_Transmitter, f: f32);
}
extern "C" {
    #[link_name = "\u{1}addString"]
    pub fn OSC_Transmitter_addString(this: *mut OSC_Transmitter, s: *mut c_char);
}
extern "C" {
    #[link_name = "\u{1}packetReady"]
    pub fn OSC_Transmitter_packetReady(this: *mut OSC_Transmitter) -> bool;
}
extern "C" {
    #[link_name = "\u{1}holdMessage"]
    pub fn OSC_Transmitter_holdMessage(this: *mut OSC_Transmitter, arg: bool);
}
extern "C" {
    #[link_name = "\u{1}tryMessage"]
    pub fn OSC_Transmitter_tryMessage(this: *mut OSC_Transmitter);
}
extern "C" {
    #[link_name = "\u{1}kickMessage"]
    pub fn OSC_Transmitter_kickMessage(this: *mut OSC_Transmitter);
}
extern "C" {
    #[link_name = "\u{1}presend"]
    pub fn OSC_Transmitter_presend(this: *mut OSC_Transmitter, buffer: *mut c_char, size: c_int);
}
extern "C" {
    #[link_name = "\u{1}OSC_Transmitter"]
    pub fn OSC_Transmitter_OSC_Transmitter(this: *mut OSC_Transmitter);
}
extern "C" {
    #[link_name = "\u{1}OSC_Transmitter"]
    pub fn OSC_Transmitter_OSC_Transmitter1(this: *mut OSC_Transmitter, out: *mut UDP_Transmitter);
}
impl OSC_Transmitter {
    #[inline]
    pub unsafe fn init(&mut self) {
        OSC_Transmitter_init(self)
    }
    #[inline]
    pub unsafe fn setHost(&mut self, host: *mut c_char, port: c_int) {
        OSC_Transmitter_setHost(self, host, port)
    }
    #[inline]
    pub unsafe fn openBundle(&mut self, t: OSCTimeTag) {
        OSC_Transmitter_openBundle(self, t)
    }
    #[inline]
    pub unsafe fn closeBundle(&mut self) {
        OSC_Transmitter_closeBundle(self)
    }
    #[inline]
    pub unsafe fn startMessage(&mut self, spec: *mut c_char) {
        OSC_Transmitter_startMessage(self, spec)
    }
    #[inline]
    pub unsafe fn startMessage1(&mut self, address: *mut c_char, args: *mut c_char) {
        OSC_Transmitter_startMessage1(self, address, args)
    }
    #[inline]
    pub unsafe fn addInt(&mut self, i: int4byte) {
        OSC_Transmitter_addInt(self, i)
    }
    #[inline]
    pub unsafe fn addFloat(&mut self, f: f32) {
        OSC_Transmitter_addFloat(self, f)
    }
    #[inline]
    pub unsafe fn addString(&mut self, s: *mut c_char) {
        OSC_Transmitter_addString(self, s)
    }
    #[inline]
    pub unsafe fn packetReady(&mut self) -> bool {
        OSC_Transmitter_packetReady(self)
    }
    #[inline]
    pub unsafe fn holdMessage(&mut self, arg: bool) {
        OSC_Transmitter_holdMessage(self, arg)
    }
    #[inline]
    pub unsafe fn tryMessage(&mut self) {
        OSC_Transmitter_tryMessage(self)
    }
    #[inline]
    pub unsafe fn kickMessage(&mut self) {
        OSC_Transmitter_kickMessage(self)
    }
    #[inline]
    pub unsafe fn presend(&mut self, buffer: *mut c_char, size: c_int) {
        OSC_Transmitter_presend(self, buffer, size)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        OSC_Transmitter_OSC_Transmitter(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(out: *mut UDP_Transmitter) -> Self {
        let mut __bindgen_tmp = uninitialized();
        OSC_Transmitter_OSC_Transmitter1(&mut __bindgen_tmp, out);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}OSC_Transmitter_destructor"]
    pub fn OSC_Transmitter_OSC_Transmitter_destructor(this: *mut OSC_Transmitter);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OSCMesg {
    pub address: *mut c_char,
    pub types: *mut c_char,
    pub data: *mut c_char,
    pub payload: *mut c_char,
    pub len: c_int,
    pub recvtime: f64,
    pub timetag: c_long,
}
#[repr(C)]
pub struct UDP_Subscriber__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct UDP_Subscriber {
    pub vtable_: *const UDP_Subscriber__bindgen_vtable,
}
extern "C" {
    #[link_name = "\u{1}UDP_Subscriber_destructor"]
    pub fn UDP_Subscriber_UDP_Subscriber_destructor(this: *mut UDP_Subscriber);
}
extern "C" {
    #[link_name = "\u{1}subscribe"]
    pub fn UDP_Subscriber_subscribe(this: *mut c_void, port: c_int) -> bool;
}
extern "C" {
    #[link_name = "\u{1}unsubscribe"]
    pub fn UDP_Subscriber_unsubscribe(this: *mut c_void) -> bool;
}
#[repr(C)]
#[derive(Debug)]
pub struct OSC_Receiver {
    pub _base: UDP_Subscriber,
    pub _port: c_int,
    pub _tmp_port: c_int,
    pub _io_mutex: *mut XMutex,
    pub _address_mutex: *mut XMutex,
    pub _meep: OSCMesg,
    pub _inbox: *mut OSCMesg,
    pub _inbox_size: c_int,
    pub _started: bool,
    pub _in_read: c_int,
    pub _in_write: c_int,
    pub _address_space: *mut OSC_Address_Space,
    pub _address_size: c_int,
    pub _address_num: c_int,
    pub m_event_buffer: *mut CBufferSimple,
    pub m_vmRef: *mut Chuck_VM,
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn OSC_Receiver_init(this: *mut OSC_Receiver);
}
extern "C" {
    #[link_name = "\u{1}bind_to_port"]
    pub fn OSC_Receiver_bind_to_port(this: *mut OSC_Receiver, port: c_int);
}
extern "C" {
    #[link_name = "\u{1}close_sock"]
    pub fn OSC_Receiver_close_sock(this: *mut OSC_Receiver);
}
extern "C" {
    #[link_name = "\u{1}recv_mesg"]
    pub fn OSC_Receiver_recv_mesg(this: *mut OSC_Receiver);
}
extern "C" {
    #[link_name = "\u{1}listen"]
    pub fn OSC_Receiver_listen(this: *mut OSC_Receiver, port: c_int) -> bool;
}
extern "C" {
    #[link_name = "\u{1}listen"]
    pub fn OSC_Receiver_listen1(this: *mut OSC_Receiver) -> bool;
}
extern "C" {
    #[link_name = "\u{1}stopListening"]
    pub fn OSC_Receiver_stopListening(this: *mut OSC_Receiver);
}
extern "C" {
    #[link_name = "\u{1}parse"]
    pub fn OSC_Receiver_parse(this: *mut OSC_Receiver, arg1: *mut c_char, len: c_int);
}
extern "C" {
    #[link_name = "\u{1}handle_mesg"]
    pub fn OSC_Receiver_handle_mesg(this: *mut OSC_Receiver, arg1: *mut c_char, len: c_int);
}
extern "C" {
    #[link_name = "\u{1}handle_bundle"]
    pub fn OSC_Receiver_handle_bundle(this: *mut OSC_Receiver, arg1: *mut c_char, len: c_int);
}
extern "C" {
    #[link_name = "\u{1}set_mesg"]
    pub fn OSC_Receiver_set_mesg(
        this: *mut OSC_Receiver,
        m: *mut OSCMesg,
        buf: *mut c_char,
        len: c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn OSC_Receiver_write(this: *mut OSC_Receiver) -> *mut OSCMesg;
}
extern "C" {
    #[link_name = "\u{1}next_write"]
    pub fn OSC_Receiver_next_write(this: *mut OSC_Receiver);
}
extern "C" {
    #[link_name = "\u{1}read"]
    pub fn OSC_Receiver_read(this: *mut OSC_Receiver) -> *mut OSCMesg;
}
extern "C" {
    #[link_name = "\u{1}next_read"]
    pub fn OSC_Receiver_next_read(this: *mut OSC_Receiver) -> *mut OSCMesg;
}
extern "C" {
    #[link_name = "\u{1}has_mesg"]
    pub fn OSC_Receiver_has_mesg(this: *mut OSC_Receiver) -> bool;
}
extern "C" {
    #[link_name = "\u{1}get_mesg"]
    pub fn OSC_Receiver_get_mesg(this: *mut OSC_Receiver, bucket: *mut OSCMesg) -> bool;
}
extern "C" {
    #[link_name = "\u{1}add_address"]
    pub fn OSC_Receiver_add_address(this: *mut OSC_Receiver, o: *mut OSC_Address_Space);
}
extern "C" {
    #[link_name = "\u{1}remove_address"]
    pub fn OSC_Receiver_remove_address(this: *mut OSC_Receiver, o: *mut OSC_Address_Space);
}
extern "C" {
    #[link_name = "\u{1}new_event"]
    pub fn OSC_Receiver_new_event(
        this: *mut OSC_Receiver,
        spec: *mut c_char,
    ) -> *mut OSC_Address_Space;
}
extern "C" {
    #[link_name = "\u{1}new_event"]
    pub fn OSC_Receiver_new_event1(
        this: *mut OSC_Receiver,
        addr: *mut c_char,
        type_: *mut c_char,
    ) -> *mut OSC_Address_Space;
}
extern "C" {
    #[link_name = "\u{1}distribute_message"]
    pub fn OSC_Receiver_distribute_message(this: *mut OSC_Receiver, msg: *mut OSCMesg);
}
extern "C" {
    #[link_name = "\u{1}OSC_Receiver"]
    pub fn OSC_Receiver_OSC_Receiver(this: *mut OSC_Receiver, vm: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}OSC_Receiver"]
    pub fn OSC_Receiver_OSC_Receiver1(
        this: *mut OSC_Receiver,
        vm: *mut Chuck_VM,
        in_: *mut UDP_Receiver,
    );
}
impl OSC_Receiver {
    #[inline]
    pub unsafe fn init(&mut self) {
        OSC_Receiver_init(self)
    }
    #[inline]
    pub unsafe fn bind_to_port(&mut self, port: c_int) {
        OSC_Receiver_bind_to_port(self, port)
    }
    #[inline]
    pub unsafe fn close_sock(&mut self) {
        OSC_Receiver_close_sock(self)
    }
    #[inline]
    pub unsafe fn recv_mesg(&mut self) {
        OSC_Receiver_recv_mesg(self)
    }
    #[inline]
    pub unsafe fn listen(&mut self, port: c_int) -> bool {
        OSC_Receiver_listen(self, port)
    }
    #[inline]
    pub unsafe fn listen1(&mut self) -> bool {
        OSC_Receiver_listen1(self)
    }
    #[inline]
    pub unsafe fn stopListening(&mut self) {
        OSC_Receiver_stopListening(self)
    }
    #[inline]
    pub unsafe fn parse(&mut self, arg1: *mut c_char, len: c_int) {
        OSC_Receiver_parse(self, arg1, len)
    }
    #[inline]
    pub unsafe fn handle_mesg(&mut self, arg1: *mut c_char, len: c_int) {
        OSC_Receiver_handle_mesg(self, arg1, len)
    }
    #[inline]
    pub unsafe fn handle_bundle(&mut self, arg1: *mut c_char, len: c_int) {
        OSC_Receiver_handle_bundle(self, arg1, len)
    }
    #[inline]
    pub unsafe fn set_mesg(&mut self, m: *mut OSCMesg, buf: *mut c_char, len: c_int) {
        OSC_Receiver_set_mesg(self, m, buf, len)
    }
    #[inline]
    pub unsafe fn write(&mut self) -> *mut OSCMesg {
        OSC_Receiver_write(self)
    }
    #[inline]
    pub unsafe fn next_write(&mut self) {
        OSC_Receiver_next_write(self)
    }
    #[inline]
    pub unsafe fn read(&mut self) -> *mut OSCMesg {
        OSC_Receiver_read(self)
    }
    #[inline]
    pub unsafe fn next_read(&mut self) -> *mut OSCMesg {
        OSC_Receiver_next_read(self)
    }
    #[inline]
    pub unsafe fn has_mesg(&mut self) -> bool {
        OSC_Receiver_has_mesg(self)
    }
    #[inline]
    pub unsafe fn get_mesg(&mut self, bucket: *mut OSCMesg) -> bool {
        OSC_Receiver_get_mesg(self, bucket)
    }
    #[inline]
    pub unsafe fn add_address(&mut self, o: *mut OSC_Address_Space) {
        OSC_Receiver_add_address(self, o)
    }
    #[inline]
    pub unsafe fn remove_address(&mut self, o: *mut OSC_Address_Space) {
        OSC_Receiver_remove_address(self, o)
    }
    #[inline]
    pub unsafe fn new_event(&mut self, spec: *mut c_char) -> *mut OSC_Address_Space {
        OSC_Receiver_new_event(self, spec)
    }
    #[inline]
    pub unsafe fn new_event1(
        &mut self,
        addr: *mut c_char,
        type_: *mut c_char,
    ) -> *mut OSC_Address_Space {
        OSC_Receiver_new_event1(self, addr, type_)
    }
    #[inline]
    pub unsafe fn distribute_message(&mut self, msg: *mut OSCMesg) {
        OSC_Receiver_distribute_message(self, msg)
    }
    #[inline]
    pub unsafe fn new(vm: *mut Chuck_VM) -> Self {
        let mut __bindgen_tmp = uninitialized();
        OSC_Receiver_OSC_Receiver(&mut __bindgen_tmp, vm);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(vm: *mut Chuck_VM, in_: *mut UDP_Receiver) -> Self {
        let mut __bindgen_tmp = uninitialized();
        OSC_Receiver_OSC_Receiver1(&mut __bindgen_tmp, vm, in_);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}onReceive"]
    pub fn OSC_Receiver_onReceive(this: *mut c_void, mesg: *mut c_char, mesgLen: c_int);
}
extern "C" {
    #[link_name = "\u{1}port"]
    pub fn OSC_Receiver_port(this: *mut c_void) -> *mut c_int;
}
extern "C" {
    #[link_name = "\u{1}OSC_Receiver_destructor"]
    pub fn OSC_Receiver_OSC_Receiver_destructor(this: *mut OSC_Receiver);
}
pub const osc_datatype_OSC_UNTYPED: osc_datatype = 0;
pub const osc_datatype_OSC_NOARGS: osc_datatype = 1;
pub const osc_datatype_OSC_INT: osc_datatype = 2;
pub const osc_datatype_OSC_FLOAT: osc_datatype = 3;
pub const osc_datatype_OSC_STRING: osc_datatype = 4;
pub const osc_datatype_OSC_BLOB: osc_datatype = 5;
pub const osc_datatype_OSC_NTYPE: osc_datatype = 6;
pub type osc_datatype = u32;
#[repr(C)]
pub struct opsc_data {
    pub t: osc_datatype,
    pub timetag: OSCTimeTag,
    pub __bindgen_anon_1: opsc_data__bindgen_ty_1,
    pub s: *mut c_char,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union opsc_data__bindgen_ty_1 {
    pub i: int4byte,
    pub u: c_ulong,
    pub f: f32,
    _bindgen_union_align: u64,
}
extern "C" {
    #[link_name = "\u{1}opsc_data"]
    pub fn opsc_data_opsc_data(this: *mut opsc_data);
}
extern "C" {
    #[link_name = "\u{1}opsc_data_destructor"]
    pub fn opsc_data_opsc_data_destructor(this: *mut opsc_data);
}
impl opsc_data {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        opsc_data_opsc_data(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        opsc_data_opsc_data_destructor(self)
    }
}
pub type OSCSrc = OSC_Address_Space;
#[repr(C)]
pub struct OSC_Address_Space {
    pub _base: Chuck_Event,
    pub _receiver: *mut OSC_Receiver,
    pub _buffer_mutex: *mut XMutex,
    pub _spec: [c_char; 512usize],
    pub _needparse: bool,
    pub _address: [c_char; 512usize],
    pub _type: [c_char; 512usize],
    pub _queue: *mut opsc_data,
    pub _current_data: *mut opsc_data,
    pub _qread: c_int,
    pub _qwrite: c_int,
    pub _queueSize: c_int,
    pub _cur_value: c_int,
    pub _cur_mesg: *mut opsc_data,
    pub _vals: *mut opsc_data,
    pub _dataSize: c_int,
    pub _noArgs: bool,
    pub SELF: *mut Chuck_Object,
    pub p_str: Chuck_String,
}
extern "C" {
    #[link_name = "\u{1}resizeData"]
    pub fn OSC_Address_Space_resizeData(this: *mut OSC_Address_Space, n: c_int);
}
extern "C" {
    #[link_name = "\u{1}resizeQueue"]
    pub fn OSC_Address_Space_resizeQueue(this: *mut OSC_Address_Space, n: c_int);
}
extern "C" {
    #[link_name = "\u{1}parseSpec"]
    pub fn OSC_Address_Space_parseSpec(this: *mut OSC_Address_Space);
}
extern "C" {
    #[link_name = "\u{1}scanSpec"]
    pub fn OSC_Address_Space_scanSpec(this: *mut OSC_Address_Space);
}
extern "C" {
    #[link_name = "\u{1}init"]
    pub fn OSC_Address_Space_init(this: *mut OSC_Address_Space);
}
extern "C" {
    #[link_name = "\u{1}setSpec"]
    pub fn OSC_Address_Space_setSpec(this: *mut OSC_Address_Space, c: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}setSpec"]
    pub fn OSC_Address_Space_setSpec1(
        this: *mut OSC_Address_Space,
        addr: *const c_char,
        type_: *const c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}setReceiver"]
    pub fn OSC_Address_Space_setReceiver(this: *mut OSC_Address_Space, recv: *mut OSC_Receiver);
}
extern "C" {
    #[link_name = "\u{1}try_queue_mesg"]
    pub fn OSC_Address_Space_try_queue_mesg(this: *mut OSC_Address_Space, o: *mut OSCMesg) -> bool;
}
extern "C" {
    #[link_name = "\u{1}message_matches"]
    pub fn OSC_Address_Space_message_matches(this: *mut OSC_Address_Space, o: *mut OSCMesg)
        -> bool;
}
extern "C" {
    #[link_name = "\u{1}queue_mesg"]
    pub fn OSC_Address_Space_queue_mesg(this: *mut OSC_Address_Space, o: *mut OSCMesg);
}
extern "C" {
    #[link_name = "\u{1}has_mesg"]
    pub fn OSC_Address_Space_has_mesg(this: *mut OSC_Address_Space) -> bool;
}
extern "C" {
    #[link_name = "\u{1}next_mesg"]
    pub fn OSC_Address_Space_next_mesg(this: *mut OSC_Address_Space) -> bool;
}
extern "C" {
    #[link_name = "\u{1}wait"]
    pub fn OSC_Address_Space_wait(
        this: *mut OSC_Address_Space,
        shred: *mut Chuck_VM_Shred,
        vm: *mut Chuck_VM,
    );
}
extern "C" {
    #[link_name = "\u{1}vcheck"]
    pub fn OSC_Address_Space_vcheck(this: *mut OSC_Address_Space, tt: osc_datatype) -> bool;
}
extern "C" {
    #[link_name = "\u{1}next_int"]
    pub fn OSC_Address_Space_next_int(this: *mut OSC_Address_Space) -> int4byte;
}
extern "C" {
    #[link_name = "\u{1}next_float"]
    pub fn OSC_Address_Space_next_float(this: *mut OSC_Address_Space) -> f32;
}
extern "C" {
    #[link_name = "\u{1}next_string"]
    pub fn OSC_Address_Space_next_string(this: *mut OSC_Address_Space) -> *mut c_char;
}
extern "C" {
    #[link_name = "\u{1}next_string_dup"]
    pub fn OSC_Address_Space_next_string_dup(this: *mut OSC_Address_Space) -> *mut c_char;
}
extern "C" {
    #[link_name = "\u{1}next_blob"]
    pub fn OSC_Address_Space_next_blob(this: *mut OSC_Address_Space) -> *mut c_char;
}
extern "C" {
    #[link_name = "\u{1}OSC_Address_Space"]
    pub fn OSC_Address_Space_OSC_Address_Space(this: *mut OSC_Address_Space);
}
extern "C" {
    #[link_name = "\u{1}OSC_Address_Space"]
    pub fn OSC_Address_Space_OSC_Address_Space1(this: *mut OSC_Address_Space, spec: *const c_char);
}
extern "C" {
    #[link_name = "\u{1}OSC_Address_Space"]
    pub fn OSC_Address_Space_OSC_Address_Space2(
        this: *mut OSC_Address_Space,
        addr: *const c_char,
        type_: *const c_char,
    );
}
impl OSC_Address_Space {
    #[inline]
    pub unsafe fn resizeData(&mut self, n: c_int) {
        OSC_Address_Space_resizeData(self, n)
    }
    #[inline]
    pub unsafe fn resizeQueue(&mut self, n: c_int) {
        OSC_Address_Space_resizeQueue(self, n)
    }
    #[inline]
    pub unsafe fn parseSpec(&mut self) {
        OSC_Address_Space_parseSpec(self)
    }
    #[inline]
    pub unsafe fn scanSpec(&mut self) {
        OSC_Address_Space_scanSpec(self)
    }
    #[inline]
    pub unsafe fn init(&mut self) {
        OSC_Address_Space_init(self)
    }
    #[inline]
    pub unsafe fn setSpec(&mut self, c: *const c_char) {
        OSC_Address_Space_setSpec(self, c)
    }
    #[inline]
    pub unsafe fn setSpec1(&mut self, addr: *const c_char, type_: *const c_char) {
        OSC_Address_Space_setSpec1(self, addr, type_)
    }
    #[inline]
    pub unsafe fn setReceiver(&mut self, recv: *mut OSC_Receiver) {
        OSC_Address_Space_setReceiver(self, recv)
    }
    #[inline]
    pub unsafe fn try_queue_mesg(&mut self, o: *mut OSCMesg) -> bool {
        OSC_Address_Space_try_queue_mesg(self, o)
    }
    #[inline]
    pub unsafe fn message_matches(&mut self, o: *mut OSCMesg) -> bool {
        OSC_Address_Space_message_matches(self, o)
    }
    #[inline]
    pub unsafe fn queue_mesg(&mut self, o: *mut OSCMesg) {
        OSC_Address_Space_queue_mesg(self, o)
    }
    #[inline]
    pub unsafe fn has_mesg(&mut self) -> bool {
        OSC_Address_Space_has_mesg(self)
    }
    #[inline]
    pub unsafe fn next_mesg(&mut self) -> bool {
        OSC_Address_Space_next_mesg(self)
    }
    #[inline]
    pub unsafe fn wait(&mut self, shred: *mut Chuck_VM_Shred, vm: *mut Chuck_VM) {
        OSC_Address_Space_wait(self, shred, vm)
    }
    #[inline]
    pub unsafe fn vcheck(&mut self, tt: osc_datatype) -> bool {
        OSC_Address_Space_vcheck(self, tt)
    }
    #[inline]
    pub unsafe fn next_int(&mut self) -> int4byte {
        OSC_Address_Space_next_int(self)
    }
    #[inline]
    pub unsafe fn next_float(&mut self) -> f32 {
        OSC_Address_Space_next_float(self)
    }
    #[inline]
    pub unsafe fn next_string(&mut self) -> *mut c_char {
        OSC_Address_Space_next_string(self)
    }
    #[inline]
    pub unsafe fn next_string_dup(&mut self) -> *mut c_char {
        OSC_Address_Space_next_string_dup(self)
    }
    #[inline]
    pub unsafe fn next_blob(&mut self) -> *mut c_char {
        OSC_Address_Space_next_blob(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        OSC_Address_Space_OSC_Address_Space(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(spec: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        OSC_Address_Space_OSC_Address_Space1(&mut __bindgen_tmp, spec);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new2(addr: *const c_char, type_: *const c_char) -> Self {
        let mut __bindgen_tmp = uninitialized();
        OSC_Address_Space_OSC_Address_Space2(&mut __bindgen_tmp, addr, type_);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}OSC_Address_Space_destructor"]
    pub fn OSC_Address_Space_OSC_Address_Space_destructor(this: *mut OSC_Address_Space);
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
pub struct _bindgen_ty_31 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_32 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_33 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_34 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_35 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_36 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_37 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_38 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_39 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_40 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_41 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_43 {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_44 {
    pub _address: u8,
}
