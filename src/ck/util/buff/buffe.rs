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
///* buffer utilities
#![feature(libc)]
use libc::*;
use crate::ck::compile::compilee::*;
use crate::ck::oo::ooe::*;
use crate::ck::util::osc::utilopsce::*;
use crate::ck::util::thread::threade::*;
use crate::dts::*;
use std::mem::MaybeUninit;
use std::marker::PhantomData;
use std::cell::UnsafeCell;
///* These types are sorta duplicated from chuck_def_h
pub type DWORD__ = u64;
pub type SINT__ = i64;
pub type UINT__ = DWORD__;
pub type BOOL__ = DWORD__;
pub type BYTE__ = u8;
pub const TRUE: u64 = 1;
pub const FALSE: u64 = 0;
#[repr(C)]
pub struct CBufferAdvance {
    pub m_data: *mut BYTE__,
    pub m_data_width: UINT__,
    pub m_read_offsets: Vec<f64>,
    pub m_free: deque,
    pub m_write_offset: SINT__,
    pub m_max_elem: SINT__,
    pub m_mutex: XMutex,
    pub m_event_buffer: *mut CBufferSimple,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CBufferAdvance_ReadOffset {
    pub read_offset: SINT__,
    pub event: *mut Chuck_Event,
}
extern "C" {
    #[link_name = "\u{1}ReadOffset"]
    pub fn CBufferAdvance_ReadOffset_ReadOffset(
        this: *mut CBufferAdvance_ReadOffset,
        ro: SINT__,
        e: *mut Chuck_Event,
    );
}
impl CBufferAdvance_ReadOffset {
    #[inline]
    pub unsafe fn new(ro: SINT__, e: *mut Chuck_Event) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        CBufferAdvance_ReadOffset_ReadOffset(&mut __bindgen_tmp, ro, e);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn CBufferAdvance_initialize(
        this: *mut CBufferAdvance,
        num_elem: UINT__,
        width: UINT__,
        event_buffer: *mut CBufferSimple,
    ) -> UINT__;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn CBufferAdvance_cleanup(this: *mut CBufferAdvance);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn CBufferAdvance_get(
        this: *mut CBufferAdvance,
        data: *mut c_void,
        num_elem: UINT__,
        read_offset_index: UINT__,
    ) -> UINT__;
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn CBufferAdvance_put(this: *mut CBufferAdvance, data: *mut c_void, num_elem: UINT__);
}
extern "C" {
    #[link_name = "\u{1}empty"]
    pub fn CBufferAdvance_empty(this: *mut CBufferAdvance, read_offset_index: UINT__) -> UINT__;
}
extern "C" {
    #[link_name = "\u{1}join"]
    pub fn CBufferAdvance_join(this: *mut CBufferAdvance, event: *mut Chuck_Event) -> UINT__;
}
extern "C" {
    #[link_name = "\u{1}resign"]
    pub fn CBufferAdvance_resign(this: *mut CBufferAdvance, read_offset_index: UINT__);
}
extern "C" {
    #[link_name = "\u{1}CBufferAdvance"]
    pub fn CBufferAdvance_CBufferAdvance(this: *mut CBufferAdvance);
}
extern "C" {
    #[link_name = "\u{1}CBufferAdvance_destructor"]
    pub fn CBufferAdvance_CBufferAdvance_destructor(this: *mut CBufferAdvance);
}
impl CBufferAdvance {
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        num_elem: UINT__,
        width: UINT__,
        event_buffer: *mut CBufferSimple,
    ) -> UINT__ {
        CBufferAdvance_initialize(self, num_elem, width, event_buffer)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        CBufferAdvance_cleanup(self)
    }
    #[inline]
    pub unsafe fn get(
        &mut self,
        data: *mut c_void,
        num_elem: UINT__,
        read_offset_index: UINT__,
    ) -> UINT__ {
        CBufferAdvance_get(self, data, num_elem, read_offset_index)
    }
    #[inline]
    pub unsafe fn put(&mut self, data: *mut c_void, num_elem: UINT__) {
        CBufferAdvance_put(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn empty(&mut self, read_offset_index: UINT__) -> UINT__ {
        CBufferAdvance_empty(self, read_offset_index)
    }
    #[inline]
    pub unsafe fn join(&mut self, event: *mut Chuck_Event) -> UINT__ {
        CBufferAdvance_join(self, event)
    }
    #[inline]
    pub unsafe fn resign(&mut self, read_offset_index: UINT__) {
        CBufferAdvance_resign(self, read_offset_index)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        CBufferAdvance_CBufferAdvance(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        CBufferAdvance_CBufferAdvance_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct CBufferSimple {
    pub m_data: *mut BYTE__,
    pub m_data_width: UINT__,
    pub m_read_offset: UINT__,
    pub m_write_offset: UINT__,
    pub m_max_elem: UINT__,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn CBufferSimple_initialize(
        this: *mut CBufferSimple,
        num_elem: UINT__,
        width: UINT__,
    ) -> UINT__;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn CBufferSimple_cleanup(this: *mut CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn CBufferSimple_get(
        this: *mut CBufferSimple,
        data: *mut c_void,
        num_elem: UINT__,
    ) -> UINT__;
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn CBufferSimple_put(this: *mut CBufferSimple, data: *mut c_void, num_elem: UINT__);
}
extern "C" {
    #[link_name = "\u{1}CBufferSimple"]
    pub fn CBufferSimple_CBufferSimple(this: *mut CBufferSimple);
}
extern "C" {
    #[link_name = "\u{1}CBufferSimple_destructor"]
    pub fn CBufferSimple_CBufferSimple_destructor(this: *mut CBufferSimple);
}
impl CBufferSimple {
    #[inline]
    pub unsafe fn initialize(&mut self, num_elem: UINT__, width: UINT__) -> UINT__ {
        CBufferSimple_initialize(self, num_elem, width)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        CBufferSimple_cleanup(self)
    }
    #[inline]
    pub unsafe fn get(&mut self, data: *mut c_void, num_elem: UINT__) -> UINT__ {
        CBufferSimple_get(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn put(&mut self, data: *mut c_void, num_elem: UINT__) {
        CBufferSimple_put(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        CBufferSimple_CBufferSimple(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        CBufferSimple_CBufferSimple_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct AccumBuffer {
    pub m_data: *mut f32,
    pub m_write_offset: UINT__,
    pub m_max_elem: UINT__,
}
extern "C" {
    #[link_name = "\u{1}resize"]
    pub fn AccumBuffer_resize(this: *mut AccumBuffer, new_size: SINT__) -> SINT__;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn AccumBuffer_cleanup(this: *mut AccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn AccumBuffer_put(this: *mut AccumBuffer, next: f32);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn AccumBuffer_get(this: *mut AccumBuffer, buffer: *mut f32, num_elem: SINT__);
}
extern "C" {
    #[link_name = "\u{1}AccumBuffer"]
    pub fn AccumBuffer_AccumBuffer(this: *mut AccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}AccumBuffer_destructor"]
    pub fn AccumBuffer_AccumBuffer_destructor(this: *mut AccumBuffer);
}
impl AccumBuffer {
    #[inline]
    pub unsafe fn resize(&mut self, new_size: SINT__) -> SINT__ {
        AccumBuffer_resize(self, new_size)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        AccumBuffer_cleanup(self)
    }
    #[inline]
    pub unsafe fn put(&mut self, next: f32) {
        AccumBuffer_put(self, next)
    }
    #[inline]
    pub unsafe fn get(&mut self, buffer: *mut f32, num_elem: SINT__) {
        AccumBuffer_get(self, buffer, num_elem)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        AccumBuffer_AccumBuffer(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        AccumBuffer_AccumBuffer_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct DeccumBuffer {
    pub m_data: *mut f32,
    pub m_read_offset: UINT__,
    pub m_max_elem: UINT__,
}
extern "C" {
    #[link_name = "\u{1}resize"]
    pub fn DeccumBuffer_resize(this: *mut DeccumBuffer, new_size: SINT__) -> SINT__;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn DeccumBuffer_cleanup(this: *mut DeccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn DeccumBuffer_put(this: *mut DeccumBuffer, next: *mut f32, num_elem: SINT__);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn DeccumBuffer_get(this: *mut DeccumBuffer, out: *mut f32);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn DeccumBuffer_get1(this: *mut DeccumBuffer, buffer: *mut f32, num_elem: SINT__);
}
extern "C" {
    #[link_name = "\u{1}DeccumBuffer"]
    pub fn DeccumBuffer_DeccumBuffer(this: *mut DeccumBuffer);
}
extern "C" {
    #[link_name = "\u{1}DeccumBuffer_destructor"]
    pub fn DeccumBuffer_DeccumBuffer_destructor(this: *mut DeccumBuffer);
}
impl DeccumBuffer {
    #[inline]
    pub unsafe fn resize(&mut self, new_size: SINT__) -> SINT__ {
        DeccumBuffer_resize(self, new_size)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        DeccumBuffer_cleanup(self)
    }
    #[inline]
    pub unsafe fn put(&mut self, next: *mut f32, num_elem: SINT__) {
        DeccumBuffer_put(self, next, num_elem)
    }
    #[inline]
    pub unsafe fn get(&mut self, out: *mut f32) {
        DeccumBuffer_get(self, out)
    }
    #[inline]
    pub unsafe fn get1(&mut self, buffer: *mut f32, num_elem: SINT__) {
        DeccumBuffer_get1(self, buffer, num_elem)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        DeccumBuffer_DeccumBuffer(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        DeccumBuffer_DeccumBuffer_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct CircularBuffer<T> {
    pub m_elements: *mut T,
    pub m_read: usize,
    pub m_write: usize,
    pub m_numElements: usize,
    pub _phantom_0: PhantomData<UnsafeCell<T>>,
}
#[repr(C)]
#[derive(Debug)]
pub struct FastCircularBuffer {
    pub m_data: *mut BYTE__,
    pub m_data_width: UINT__,
    pub m_read_offset: UINT__,
    pub m_write_offset: UINT__,
    pub m_max_elem: UINT__,
}
extern "C" {
    #[link_name = "\u{1}initialize"]
    pub fn FastCircularBuffer_initialize(
        this: *mut FastCircularBuffer,
        num_elem: UINT__,
        width: UINT__,
    ) -> UINT__;
}
extern "C" {
    #[link_name = "\u{1}cleanup"]
    pub fn FastCircularBuffer_cleanup(this: *mut FastCircularBuffer);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn FastCircularBuffer_get(
        this: *mut FastCircularBuffer,
        data: *mut c_void,
        num_elem: UINT__,
    ) -> UINT__;
}
extern "C" {
    #[link_name = "\u{1}put"]
    pub fn FastCircularBuffer_put(
        this: *mut FastCircularBuffer,
        data: *mut c_void,
        num_elem: UINT__,
    ) -> UINT__;
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
impl FastCircularBuffer {
    #[inline]
    pub unsafe fn initialize(&mut self, num_elem: UINT__, width: UINT__) -> UINT__ {
        FastCircularBuffer_initialize(self, num_elem, width)
    }
    #[inline]
    pub unsafe fn cleanup(&mut self) {
        FastCircularBuffer_cleanup(self)
    }
    #[inline]
    pub unsafe fn get(&mut self, data: *mut c_void, num_elem: UINT__) -> UINT__ {
        FastCircularBuffer_get(self, data, num_elem)
    }
    #[inline]
    pub unsafe fn put(&mut self, data: *mut c_void, num_elem: UINT__) -> UINT__ {
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
        let mut __bindgen_tmp = MaybeUninit::uninitialized();
        FastCircularBuffer_FastCircularBuffer(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        FastCircularBuffer_FastCircularBuffer_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct XCircleBuffer<T> {
    pub m_buffer: *mut T,
    pub m_length: SINT__,
    pub m_writeIndex: SINT__,
    pub m_readIndex: SINT__,
    pub m_numElements: SINT__,
    pub _phantom_0: PhantomData<UnsafeCell<T>>,
}
