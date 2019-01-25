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
use crate::chuck_compile_h_edited::*;
use crate::chuck_def_h_edited::*;
use crate::chuck_dl_h_edited::*;
use crate::chuck_oo_h_edited::*;
use crate::chuck_type_h_edited::*;
use crate::dts::*;
///* interface for audio processing units (called unit generators), which include
///* oscillators, filters, envelopes, noise generators, etc... this module
///* defines data structures of unit generators and how to import them into the
///* type system.
use libc::*;
pub const UGEN_OP_PASS: i32 = -1;
pub const UGEN_OP_STOP: u32 = 0;
pub const UGEN_OP_TICK: u32 = 1;
//-----------------------------------------------------------------------------
// name: struct Chuck_UGen
// dsec: ugen base
//-----------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_UGen {
    pub _base: Chuck_Object,
    // tick function
    pub tick: f_tick,
    // multichannel/vectorized tick function (added 1.3.0.0)
    pub tickf: f_tickf,
    // msg function
    pub pmsg: f_pmsg,
    // channels (if more than one is required)
    pub m_multi_chan: *mut Chuck_UGen,
    // size of m_multi_chan;
    pub m_multi_chan_size: c_ulong,
    // number of channels
    pub m_num_ins: c_ulong,
    // number of channels
    pub m_num_outs: c_ulong,
    // data
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
    // SPENCERTODO: combine with block processing (added 1.3.0.0)
    pub m_multi_in_v: *mut f32,
    pub m_multi_out_v: *mut f32,
    // SPENCERTODO: better way to handle this (added 1.3.0.0)
    pub m_is_subgraph: c_ulong,
    pub m_inlet: *mut Chuck_UGen,
    pub m_outlet: *mut Chuck_UGen,
    // block processing
    pub m_sum_v: *mut f32,
    pub m_current_v: *mut f32,
    // the shred on which the ugen is created
    pub shred: *mut Chuck_VM_Shred,
    // the vm on which the ugen is created
    pub vm: *mut Chuck_VM,
    // owner
    pub owner: *mut Chuck_UGen,
    // what a hack!
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
// alloc multi channels
extern "C" {
    #[link_name = "\u{1}alloc_multi_chan"]
    pub fn Chuck_UGen_alloc_multi_chan(this: *mut Chuck_UGen, num_ins: c_ulong, num_outs: c_ulong);
}
// sets up ugen as a subgraph type ugen (added 1.3.0.0)
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
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
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
//-----------------------------------------------------------------------------
// name: struct Chuck_UAna
// dsec: uana base
//-----------------------------------------------------------------------------
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
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
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
