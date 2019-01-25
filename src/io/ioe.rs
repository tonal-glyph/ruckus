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
use crate::chuck_oo_h_edited::*;
use crate::dts::*;
use crate::util_buffers_h_edited::*;
use crate::util_thread_h_edited::*;
use std::mem::*;
use std::option::Option;
use std::os::raw::*;
use std::prelude::v1::*;
#[repr(C)]
pub struct Chuck_IO_Serial {
    pub _base: Chuck_IO,
    pub m_read_thread: *mut XThread,
    pub m_write_thread: *mut XThread,
    pub m_do_read_thread: bool,
    pub m_do_write_thread: bool,
    pub m_asyncRequests: CircularBuffer<Chuck_IO_Serial_Request>,
    pub m_asyncResponses: CircularBuffer<Chuck_IO_Serial_Request>,
    pub m_event_buffer: *mut CBufferSimple,
    pub m_fd: libc::c_int,
    pub m_cfd: *mut FILE,
    pub m_path: crate::dts::string,
    pub m_io_buf: *mut libc::c_uchar,
    pub m_io_buf_max: libc::c_ulong,
    pub m_io_buf_available: libc::c_ulong,
    pub m_io_buf_pos: libc::c_ulong,
    pub m_tmp_buf: *mut libc::c_uchar,
    pub m_tmp_buf_max: libc::c_ulong,
    pub m_asyncWriteRequests: CircularBuffer<Chuck_IO_Serial_Request>,
    pub m_writeBuffer: CircularBuffer<libc::c_char>,
    pub m_flags: libc::c_long,
    pub m_iomode: libc::c_long,
    pub m_eof: libc::c_ulong,
    pub m_do_exit: libc::c_ulong,
    pub m_vmRef: *mut Chuck_VM,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_IO_Serial_Request {
    pub m_type: libc::c_ulong,
    pub m_num: libc::c_ulong,
    pub m_status: libc::c_ulong,
    pub m_val: libc::c_ulong,
}
pub const Chuck_IO_Serial_Request_Status_RQ_STATUS_PENDING: Chuck_IO_Serial_Request_Status = 0;
pub const Chuck_IO_Serial_Request_Status_RQ_STATUS_SUCCESS: Chuck_IO_Serial_Request_Status = 1;
pub const Chuck_IO_Serial_Request_Status_RQ_STATUS_FAILURE: Chuck_IO_Serial_Request_Status = 2;
pub const Chuck_IO_Serial_Request_Status_RQ_STATUS_INVALID: Chuck_IO_Serial_Request_Status = 3;
pub const Chuck_IO_Serial_Request_Status_RQ_STATUS_EOF: Chuck_IO_Serial_Request_Status = 4;
pub type Chuck_IO_Serial_Request_Status = u32;
extern "C" {
    #[link_name = "\u{1}TYPE_NONE"]
    pub static Chuck_IO_Serial_TYPE_NONE: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}TYPE_BYTE"]
    pub static Chuck_IO_Serial_TYPE_BYTE: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}TYPE_WORD"]
    pub static Chuck_IO_Serial_TYPE_WORD: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}TYPE_INT"]
    pub static Chuck_IO_Serial_TYPE_INT: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}TYPE_FLOAT"]
    pub static Chuck_IO_Serial_TYPE_FLOAT: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}TYPE_STRING"]
    pub static Chuck_IO_Serial_TYPE_STRING: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}TYPE_LINE"]
    pub static Chuck_IO_Serial_TYPE_LINE: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}TYPE_WRITE"]
    pub static Chuck_IO_Serial_TYPE_WRITE: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_2400"]
    pub static Chuck_IO_Serial_CK_BAUD_2400: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_4800"]
    pub static Chuck_IO_Serial_CK_BAUD_4800: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_9600"]
    pub static Chuck_IO_Serial_CK_BAUD_9600: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_19200"]
    pub static Chuck_IO_Serial_CK_BAUD_19200: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_38400"]
    pub static Chuck_IO_Serial_CK_BAUD_38400: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_7200"]
    pub static Chuck_IO_Serial_CK_BAUD_7200: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_14400"]
    pub static Chuck_IO_Serial_CK_BAUD_14400: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_28800"]
    pub static Chuck_IO_Serial_CK_BAUD_28800: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_57600"]
    pub static Chuck_IO_Serial_CK_BAUD_57600: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_76800"]
    pub static Chuck_IO_Serial_CK_BAUD_76800: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_115200"]
    pub static Chuck_IO_Serial_CK_BAUD_115200: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_BAUD_230400"]
    pub static Chuck_IO_Serial_CK_BAUD_230400: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}s_serials"]
    pub static mut Chuck_IO_Serial_s_serials: crate::dts::list;
}
extern "C" {
    #[link_name = "\u{1}shutdown"]
    pub fn Chuck_IO_Serial_shutdown();
}
extern "C" {
    #[link_name = "\u{1}start_read_thread"]
    pub fn Chuck_IO_Serial_start_read_thread(this: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}shell_read_cb"]
    pub fn Chuck_IO_Serial_shell_read_cb(arg1: *mut libc::c_void) -> *mut libc::c_void;
}
extern "C" {
    #[link_name = "\u{1}read_cb"]
    pub fn Chuck_IO_Serial_read_cb(this: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}shell_write_cb"]
    pub fn Chuck_IO_Serial_shell_write_cb(arg1: *mut libc::c_void) -> *mut libc::c_void;
}
extern "C" {
    #[link_name = "\u{1}write_cb"]
    pub fn Chuck_IO_Serial_write_cb(this: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}close_int"]
    pub fn Chuck_IO_Serial_close_int(this: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}get_buffer"]
    pub fn Chuck_IO_Serial_get_buffer(
        this: *mut Chuck_IO_Serial,
        timeout_ms: libc::c_long,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}peek_buffer"]
    pub fn Chuck_IO_Serial_peek_buffer(this: *mut Chuck_IO_Serial) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}pull_buffer"]
    pub fn Chuck_IO_Serial_pull_buffer(this: *mut Chuck_IO_Serial) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}buffer_bytes_to_tmp"]
    pub fn Chuck_IO_Serial_buffer_bytes_to_tmp(
        this: *mut Chuck_IO_Serial,
        num_bytes: libc::c_long,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}handle_line"]
    pub fn Chuck_IO_Serial_handle_line(
        this: *mut Chuck_IO_Serial,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}handle_string"]
    pub fn Chuck_IO_Serial_handle_string(
        this: *mut Chuck_IO_Serial,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}handle_float_ascii"]
    pub fn Chuck_IO_Serial_handle_float_ascii(
        this: *mut Chuck_IO_Serial,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}handle_int_ascii"]
    pub fn Chuck_IO_Serial_handle_int_ascii(
        this: *mut Chuck_IO_Serial,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}handle_byte"]
    pub fn Chuck_IO_Serial_handle_byte(
        this: *mut Chuck_IO_Serial,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}handle_float_binary"]
    pub fn Chuck_IO_Serial_handle_float_binary(
        this: *mut Chuck_IO_Serial,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}handle_int_binary"]
    pub fn Chuck_IO_Serial_handle_int_binary(
        this: *mut Chuck_IO_Serial,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Serial"]
    pub fn Chuck_IO_Serial_Chuck_IO_Serial(this: *mut Chuck_IO_Serial, vm: *mut Chuck_VM);
}
impl Chuck_IO_Serial {
    #[inline]
    pub unsafe fn shutdown() {
        Chuck_IO_Serial_shutdown()
    }
    #[inline]
    pub unsafe fn start_read_thread(&mut self) {
        Chuck_IO_Serial_start_read_thread(self)
    }
    #[inline]
    pub unsafe fn shell_read_cb(arg1: *mut libc::c_void) -> *mut libc::c_void {
        Chuck_IO_Serial_shell_read_cb(arg1)
    }
    #[inline]
    pub unsafe fn read_cb(&mut self) {
        Chuck_IO_Serial_read_cb(self)
    }
    #[inline]
    pub unsafe fn shell_write_cb(arg1: *mut libc::c_void) -> *mut libc::c_void {
        Chuck_IO_Serial_shell_write_cb(arg1)
    }
    #[inline]
    pub unsafe fn write_cb(&mut self) {
        Chuck_IO_Serial_write_cb(self)
    }
    #[inline]
    pub unsafe fn close_int(&mut self) {
        Chuck_IO_Serial_close_int(self)
    }
    #[inline]
    pub unsafe fn get_buffer(&mut self, timeout_ms: libc::c_long) -> libc::c_ulong {
        Chuck_IO_Serial_get_buffer(self, timeout_ms)
    }
    #[inline]
    pub unsafe fn peek_buffer(&mut self) -> libc::c_long {
        Chuck_IO_Serial_peek_buffer(self)
    }
    #[inline]
    pub unsafe fn pull_buffer(&mut self) -> libc::c_long {
        Chuck_IO_Serial_pull_buffer(self)
    }
    #[inline]
    pub unsafe fn buffer_bytes_to_tmp(&mut self, num_bytes: libc::c_long) -> libc::c_long {
        Chuck_IO_Serial_buffer_bytes_to_tmp(self, num_bytes)
    }
    #[inline]
    pub unsafe fn handle_line(&mut self, arg1: *mut Chuck_IO_Serial_Request) -> libc::c_ulong {
        Chuck_IO_Serial_handle_line(self, arg1)
    }
    #[inline]
    pub unsafe fn handle_string(&mut self, arg1: *mut Chuck_IO_Serial_Request) -> libc::c_ulong {
        Chuck_IO_Serial_handle_string(self, arg1)
    }
    #[inline]
    pub unsafe fn handle_float_ascii(
        &mut self,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong {
        Chuck_IO_Serial_handle_float_ascii(self, arg1)
    }
    #[inline]
    pub unsafe fn handle_int_ascii(&mut self, arg1: *mut Chuck_IO_Serial_Request) -> libc::c_ulong {
        Chuck_IO_Serial_handle_int_ascii(self, arg1)
    }
    #[inline]
    pub unsafe fn handle_byte(&mut self, arg1: *mut Chuck_IO_Serial_Request) -> libc::c_ulong {
        Chuck_IO_Serial_handle_byte(self, arg1)
    }
    #[inline]
    pub unsafe fn handle_float_binary(
        &mut self,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong {
        Chuck_IO_Serial_handle_float_binary(self, arg1)
    }
    #[inline]
    pub unsafe fn handle_int_binary(
        &mut self,
        arg1: *mut Chuck_IO_Serial_Request,
    ) -> libc::c_ulong {
        Chuck_IO_Serial_handle_int_binary(self, arg1)
    }
    #[inline]
    pub unsafe fn new(vm: *mut Chuck_VM) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_Serial_Chuck_IO_Serial(&mut __bindgen_tmp, vm);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Serial_destructor"]
    pub fn Chuck_IO_Serial_Chuck_IO_Serial_destructor(this: *mut Chuck_IO_Serial);
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn Chuck_IO_Serial_open(
        this: *mut libc::c_void,
        i: libc::c_ulong,
        flags: libc::c_long,
        baud: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn Chuck_IO_Serial_open1(
        this: *mut libc::c_void,
        path: *const crate::dts::string,
        flags: libc::c_long,
        baud: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}can_wait"]
    pub fn Chuck_IO_Serial_can_wait(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_IO_Serial_good(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_Serial_close(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_Serial_flush(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Serial_mode(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Serial_mode1(this: *mut libc::c_void, flag: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_Serial_readLine(this: *mut libc::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_Serial_readInt(this: *mut libc::c_void, flags: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_Serial_readFloat(this: *mut libc::c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_Serial_readString(
        this: *mut libc::c_void,
        str: *mut crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Serial_eof(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Serial_write(
        this: *mut libc::c_void,
        val: *const crate::dts::string,
    );
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Serial_write1(this: *mut libc::c_void, val: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Serial_write2(this: *mut libc::c_void, val: libc::c_long, size: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}writeBytes"]
    pub fn Chuck_IO_Serial_writeBytes(this: *mut libc::c_void, arr: *mut Chuck_Array4);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Serial_write3(this: *mut libc::c_void, val: f64);
}
extern "C" {
    #[link_name = "\u{1}setBaudRate"]
    pub fn Chuck_IO_Serial_setBaudRate(
        this: *mut libc::c_void,
        rate: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}getBaudRate"]
    pub fn Chuck_IO_Serial_getBaudRate(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}readAsync"]
    pub fn Chuck_IO_Serial_readAsync(
        this: *mut libc::c_void,
        type_: libc::c_ulong,
        num: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}getLine"]
    pub fn Chuck_IO_Serial_getLine(this: *mut libc::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}getByte"]
    pub fn Chuck_IO_Serial_getByte(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}getBytes"]
    pub fn Chuck_IO_Serial_getBytes(this: *mut libc::c_void) -> *mut Chuck_Array;
}
extern "C" {
    #[link_name = "\u{1}getInts"]
    pub fn Chuck_IO_Serial_getInts(this: *mut libc::c_void) -> *mut Chuck_Array;
}
extern "C" {
    #[link_name = "\u{1}getFloats"]
    pub fn Chuck_IO_Serial_getFloats(this: *mut libc::c_void) -> *mut Chuck_Array;
}
extern "C" {
    #[link_name = "\u{1}getString"]
    pub fn Chuck_IO_Serial_getString(this: *mut libc::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}ready"]
    pub fn Chuck_IO_Serial_ready(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    pub fn init_class_serialio(env: *mut Chuck_Env) -> libc::c_ulong;
}
