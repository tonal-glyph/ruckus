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
use crate::rtmidi_h_edited::*;
use crate::util_buffers_h_edited::*;
///* RtMidi IO header
use libc::*;
pub const CHUCK_ARRAY4_DATAKIND: u32 = 1;
pub const CHUCK_ARRAY8_DATAKIND: u32 = 2;
pub const CHUCK_ARRAY16_DATAKIND: u32 = 3;
pub const CHUCK_ARRAY24_DATAKIND: u32 = 4;
pub const CHUCK_ARRAY32_DATAKIND: u32 = 5;
pub const MIDI_NOTEON: u32 = 0x90;
pub const MIDI_NOTEOFF: u32 = 0x80;
pub const MIDI_POLYPRESS: u32 = 0xa0;
pub const MIDI_CTRLCHANGE: u32 = 0xb0;
pub const MIDI_PROGCHANGE: u32 = 0xc0;
pub const MIDI_CHANPRESS: u32 = 0xd0;
pub const MIDI_PITCHBEND: u32 = 0xe0;
pub const MIDI_ALLNOTESOFF: u32 = 0x7b;
#[repr(C)]
#[repr(align(1))]
#[derive(Copy, Clone)]
pub union MidiMsg {
    pub data: [c_uchar; 4usize],
    _bindgen_union_align: [u8; 4usize],
}
#[repr(C)]
pub struct MidiOut {
    pub mout: *mut RtMidiOut,
    pub m_msg: vector,
    pub m_device_num: c_ulong,
    pub m_valid: c_ulong,
    pub m_suppress_output: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiOut_open(this: *mut MidiOut, device_num: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiOut_open1(this: *mut MidiOut, name: *const string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn MidiOut_close(this: *mut MidiOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn MidiOut_good(this: *mut MidiOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}num"]
    pub fn MidiOut_num(this: *mut MidiOut) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_suppress"]
    pub fn MidiOut_set_suppress(this: *mut MidiOut, print_or_not: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}get_suppress"]
    pub fn MidiOut_get_suppress(this: *mut MidiOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}send"]
    pub fn MidiOut_send(this: *mut MidiOut, status: c_uchar) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}send"]
    pub fn MidiOut_send1(this: *mut MidiOut, status: c_uchar, data1: c_uchar) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}send"]
    pub fn MidiOut_send2(
        this: *mut MidiOut,
        status: c_uchar,
        data1: c_uchar,
        data2: c_uchar,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}send"]
    pub fn MidiOut_send3(this: *mut MidiOut, msg: *const MidiMsg) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}noteon"]
    pub fn MidiOut_noteon(
        this: *mut MidiOut,
        channel: c_ulong,
        note: c_ulong,
        velocity: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}noteoff"]
    pub fn MidiOut_noteoff(
        this: *mut MidiOut,
        channel: c_ulong,
        note: c_ulong,
        velocity: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}polypress"]
    pub fn MidiOut_polypress(
        this: *mut MidiOut,
        channel: c_ulong,
        note: c_ulong,
        pressure: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}ctrlchange"]
    pub fn MidiOut_ctrlchange(
        this: *mut MidiOut,
        channel: c_ulong,
        ctrl_num: c_ulong,
        ctrl_val: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}progchange"]
    pub fn MidiOut_progchange(this: *mut MidiOut, channel: c_ulong, patch: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}chanpress"]
    pub fn MidiOut_chanpress(this: *mut MidiOut, channel: c_ulong, pressure: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}pitchbend"]
    pub fn MidiOut_pitchbend(this: *mut MidiOut, channel: c_ulong, bend_val: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}allnotesoff"]
    pub fn MidiOut_allnotesoff(this: *mut MidiOut, channel: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}MidiOut"]
    pub fn MidiOut_MidiOut(this: *mut MidiOut);
}
extern "C" {
    #[link_name = "\u{1}MidiOut_destructor"]
    pub fn MidiOut_MidiOut_destructor(this: *mut MidiOut);
}
impl MidiOut {
    #[inline]
    pub unsafe fn open(&mut self, device_num: c_ulong) -> c_ulong {
        MidiOut_open(self, device_num)
    }
    #[inline]
    pub unsafe fn open1(&mut self, name: *const string) -> c_ulong {
        MidiOut_open1(self, name)
    }
    #[inline]
    pub unsafe fn close(&mut self) -> c_ulong {
        MidiOut_close(self)
    }
    #[inline]
    pub unsafe fn good(&mut self) -> c_ulong {
        MidiOut_good(self)
    }
    #[inline]
    pub unsafe fn num(&mut self) -> c_long {
        MidiOut_num(self)
    }
    #[inline]
    pub unsafe fn set_suppress(&mut self, print_or_not: c_ulong) {
        MidiOut_set_suppress(self, print_or_not)
    }
    #[inline]
    pub unsafe fn get_suppress(&mut self) -> c_ulong {
        MidiOut_get_suppress(self)
    }
    #[inline]
    pub unsafe fn send(&mut self, status: c_uchar) -> c_ulong {
        MidiOut_send(self, status)
    }
    #[inline]
    pub unsafe fn send1(&mut self, status: c_uchar, data1: c_uchar) -> c_ulong {
        MidiOut_send1(self, status, data1)
    }
    #[inline]
    pub unsafe fn send2(&mut self, status: c_uchar, data1: c_uchar, data2: c_uchar) -> c_ulong {
        MidiOut_send2(self, status, data1, data2)
    }
    #[inline]
    pub unsafe fn send3(&mut self, msg: *const MidiMsg) -> c_ulong {
        MidiOut_send3(self, msg)
    }
    #[inline]
    pub unsafe fn noteon(&mut self, channel: c_ulong, note: c_ulong, velocity: c_ulong) -> c_ulong {
        MidiOut_noteon(self, channel, note, velocity)
    }
    #[inline]
    pub unsafe fn noteoff(
        &mut self,
        channel: c_ulong,
        note: c_ulong,
        velocity: c_ulong,
    ) -> c_ulong {
        MidiOut_noteoff(self, channel, note, velocity)
    }
    #[inline]
    pub unsafe fn polypress(
        &mut self,
        channel: c_ulong,
        note: c_ulong,
        pressure: c_ulong,
    ) -> c_ulong {
        MidiOut_polypress(self, channel, note, pressure)
    }
    #[inline]
    pub unsafe fn ctrlchange(
        &mut self,
        channel: c_ulong,
        ctrl_num: c_ulong,
        ctrl_val: c_ulong,
    ) -> c_ulong {
        MidiOut_ctrlchange(self, channel, ctrl_num, ctrl_val)
    }
    #[inline]
    pub unsafe fn progchange(&mut self, channel: c_ulong, patch: c_ulong) -> c_ulong {
        MidiOut_progchange(self, channel, patch)
    }
    #[inline]
    pub unsafe fn chanpress(&mut self, channel: c_ulong, pressure: c_ulong) -> c_ulong {
        MidiOut_chanpress(self, channel, pressure)
    }
    #[inline]
    pub unsafe fn pitchbend(&mut self, channel: c_ulong, bend_val: c_ulong) -> c_ulong {
        MidiOut_pitchbend(self, channel, bend_val)
    }
    #[inline]
    pub unsafe fn allnotesoff(&mut self, channel: c_ulong) -> c_ulong {
        MidiOut_allnotesoff(self, channel)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        MidiOut_MidiOut(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        MidiOut_MidiOut_destructor(self)
    }
}
#[repr(C)]
pub struct MidiIn {
    pub _base: Chuck_Event,
    pub m_buffer: *mut CBufferAdvance,
    pub m_read_index: c_ulong,
    pub min: *mut RtMidiIn,
    pub m_valid: c_ulong,
    pub m_device_num: c_ulong,
    pub SELF: *mut Chuck_Object,
    pub m_suppress_output: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiIn_open(this: *mut MidiIn, vm: *mut Chuck_VM, device_num: c_ulong) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiIn_open1(this: *mut MidiIn, vm: *mut Chuck_VM, name: *const string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn MidiIn_close(this: *mut MidiIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn MidiIn_good(this: *mut MidiIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}num"]
    pub fn MidiIn_num(this: *mut MidiIn) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}set_suppress"]
    pub fn MidiIn_set_suppress(this: *mut MidiIn, print_or_not: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}get_suppress"]
    pub fn MidiIn_get_suppress(this: *mut MidiIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}empty"]
    pub fn MidiIn_empty(this: *mut MidiIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}recv"]
    pub fn MidiIn_recv(this: *mut MidiIn, msg: *mut MidiMsg) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}MidiIn"]
    pub fn MidiIn_MidiIn(this: *mut MidiIn);
}
impl MidiIn {
    #[inline]
    pub unsafe fn open(&mut self, vm: *mut Chuck_VM, device_num: c_ulong) -> c_ulong {
        MidiIn_open(self, vm, device_num)
    }
    #[inline]
    pub unsafe fn open1(&mut self, vm: *mut Chuck_VM, name: *const string) -> c_ulong {
        MidiIn_open1(self, vm, name)
    }
    #[inline]
    pub unsafe fn close(&mut self) -> c_ulong {
        MidiIn_close(self)
    }
    #[inline]
    pub unsafe fn good(&mut self) -> c_ulong {
        MidiIn_good(self)
    }
    #[inline]
    pub unsafe fn num(&mut self) -> c_long {
        MidiIn_num(self)
    }
    #[inline]
    pub unsafe fn set_suppress(&mut self, print_or_not: c_ulong) {
        MidiIn_set_suppress(self, print_or_not)
    }
    #[inline]
    pub unsafe fn get_suppress(&mut self) -> c_ulong {
        MidiIn_get_suppress(self)
    }
    #[inline]
    pub unsafe fn empty(&mut self) -> c_ulong {
        MidiIn_empty(self)
    }
    #[inline]
    pub unsafe fn recv(&mut self, msg: *mut MidiMsg) -> c_ulong {
        MidiIn_recv(self, msg)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        MidiIn_MidiIn(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}MidiIn_destructor"]
    pub fn MidiIn_MidiIn_destructor(this: *mut MidiIn);
}
extern "C" {
    pub fn probeMidiIn();
}
extern "C" {
    pub fn probeMidiOut();
}
#[repr(C)]
#[derive(Debug)]
pub struct MidiInManager {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}the_mins"]
    pub static mut MidiInManager_the_mins: vector;
}
extern "C" {
    #[link_name = "\u{1}the_bufs"]
    pub static mut MidiInManager_the_bufs: vector;
}
extern "C" {
    #[link_name = "\u{1}m_event_buffers"]
    pub static mut MidiInManager_m_event_buffers: map;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiInManager_open(min: *mut MidiIn, vm: *mut Chuck_VM, device_num: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiInManager_open1(min: *mut MidiIn, vm: *mut Chuck_VM, name: *const string)
        -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn MidiInManager_close(min: *mut MidiIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}cleanup_buffer"]
    pub fn MidiInManager_cleanup_buffer(vm: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}cb_midi_input"]
    pub fn MidiInManager_cb_midi_input(deltatime: f64, msg: *mut vector, userData: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}MidiInManager"]
    pub fn MidiInManager_MidiInManager(this: *mut MidiInManager);
}
extern "C" {
    #[link_name = "\u{1}MidiInManager_destructor"]
    pub fn MidiInManager_MidiInManager_destructor(this: *mut MidiInManager);
}
impl MidiInManager {
    #[inline]
    pub unsafe fn open(min: *mut MidiIn, vm: *mut Chuck_VM, device_num: c_long) -> c_ulong {
        MidiInManager_open(min, vm, device_num)
    }
    #[inline]
    pub unsafe fn open1(min: *mut MidiIn, vm: *mut Chuck_VM, name: *const string) -> c_ulong {
        MidiInManager_open1(min, vm, name)
    }
    #[inline]
    pub unsafe fn close(min: *mut MidiIn) -> c_ulong {
        MidiInManager_close(min)
    }
    #[inline]
    pub unsafe fn cleanup_buffer(vm: *mut Chuck_VM) {
        MidiInManager_cleanup_buffer(vm)
    }
    #[inline]
    pub unsafe fn cb_midi_input(deltatime: f64, msg: *mut vector, userData: *mut c_void) {
        MidiInManager_cb_midi_input(deltatime, msg, userData)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        MidiInManager_MidiInManager(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        MidiInManager_MidiInManager_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct MidiOutManager {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}the_mouts"]
    pub static mut MidiOutManager_the_mouts: vector;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiOutManager_open(mout: *mut MidiOut, device_num: c_long) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiOutManager_open1(mout: *mut MidiOut, name: *const string) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn MidiOutManager_close(mout: *mut MidiOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}MidiOutManager"]
    pub fn MidiOutManager_MidiOutManager(this: *mut MidiOutManager);
}
extern "C" {
    #[link_name = "\u{1}MidiOutManager_destructor"]
    pub fn MidiOutManager_MidiOutManager_destructor(this: *mut MidiOutManager);
}
impl MidiOutManager {
    #[inline]
    pub unsafe fn open(mout: *mut MidiOut, device_num: c_long) -> c_ulong {
        MidiOutManager_open(mout, device_num)
    }
    #[inline]
    pub unsafe fn open1(mout: *mut MidiOut, name: *const string) -> c_ulong {
        MidiOutManager_open1(mout, name)
    }
    #[inline]
    pub unsafe fn close(mout: *mut MidiOut) -> c_ulong {
        MidiOutManager_close(mout)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        MidiOutManager_MidiOutManager(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        MidiOutManager_MidiOutManager_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct MidiRW {
    pub file: *mut FILE,
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiRW_open(this: *mut MidiRW, filename: *const c_char) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn MidiRW_close(this: *mut MidiRW) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}read"]
    pub fn MidiRW_read(this: *mut MidiRW, msg: *mut MidiMsg, time: *mut f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn MidiRW_write(this: *mut MidiRW, msg: *mut MidiMsg, time: *mut f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}MidiRW"]
    pub fn MidiRW_MidiRW(this: *mut MidiRW);
}
extern "C" {
    #[link_name = "\u{1}MidiRW_destructor"]
    pub fn MidiRW_MidiRW_destructor(this: *mut MidiRW);
}
impl MidiRW {
    #[inline]
    pub unsafe fn open(&mut self, filename: *const c_char) -> c_ulong {
        MidiRW_open(self, filename)
    }
    #[inline]
    pub unsafe fn close(&mut self) -> c_ulong {
        MidiRW_close(self)
    }
    #[inline]
    pub unsafe fn read(&mut self, msg: *mut MidiMsg, time: *mut f64) -> c_ulong {
        MidiRW_read(self, msg, time)
    }
    #[inline]
    pub unsafe fn write(&mut self, msg: *mut MidiMsg, time: *mut f64) -> c_ulong {
        MidiRW_write(self, msg, time)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        MidiRW_MidiRW(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        MidiRW_MidiRW_destructor(self)
    }
}
extern "C" {
    pub fn midirw_detach() -> c_ulong;
}
#[repr(C)]
#[derive(Debug)]
pub struct MidiMsgOut {
    pub file: *mut FILE,
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiMsgOut_open(this: *mut MidiMsgOut, filename: *const c_char) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn MidiMsgOut_close(this: *mut MidiMsgOut) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn MidiMsgOut_write(this: *mut MidiMsgOut, msg: *mut MidiMsg, time: *mut f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}MidiMsgOut"]
    pub fn MidiMsgOut_MidiMsgOut(this: *mut MidiMsgOut);
}
extern "C" {
    #[link_name = "\u{1}MidiMsgOut_destructor"]
    pub fn MidiMsgOut_MidiMsgOut_destructor(this: *mut MidiMsgOut);
}
impl MidiMsgOut {
    #[inline]
    pub unsafe fn open(&mut self, filename: *const c_char) -> c_ulong {
        MidiMsgOut_open(self, filename)
    }
    #[inline]
    pub unsafe fn close(&mut self) -> c_ulong {
        MidiMsgOut_close(self)
    }
    #[inline]
    pub unsafe fn write(&mut self, msg: *mut MidiMsg, time: *mut f64) -> c_ulong {
        MidiMsgOut_write(self, msg, time)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        MidiMsgOut_MidiMsgOut(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        MidiMsgOut_MidiMsgOut_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct MidiMsgIn {
    pub file: *mut FILE,
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn MidiMsgIn_open(this: *mut MidiMsgIn, filename: *const c_char) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn MidiMsgIn_close(this: *mut MidiMsgIn) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}read"]
    pub fn MidiMsgIn_read(this: *mut MidiMsgIn, msg: *mut MidiMsg, time: *mut f64) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}MidiMsgIn"]
    pub fn MidiMsgIn_MidiMsgIn(this: *mut MidiMsgIn);
}
extern "C" {
    #[link_name = "\u{1}MidiMsgIn_destructor"]
    pub fn MidiMsgIn_MidiMsgIn_destructor(this: *mut MidiMsgIn);
}
impl MidiMsgIn {
    #[inline]
    pub unsafe fn open(&mut self, filename: *const c_char) -> c_ulong {
        MidiMsgIn_open(self, filename)
    }
    #[inline]
    pub unsafe fn close(&mut self) -> c_ulong {
        MidiMsgIn_close(self)
    }
    #[inline]
    pub unsafe fn read(&mut self, msg: *mut MidiMsg, time: *mut f64) -> c_ulong {
        MidiMsgIn_read(self, msg, time)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        MidiMsgIn_MidiMsgIn(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        MidiMsgIn_MidiMsgIn_destructor(self)
    }
}
