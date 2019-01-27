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
///* tre regex mem
use libc::*;
pub const TRE_MEM_H: u32 = 1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_list {
    pub data: *mut c_void,
    pub next: *mut tre_list,
}
pub type tre_list_t = tre_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_mem_struct {
    pub blocks: *mut tre_list_t,
    pub current: *mut tre_list_t,
    pub ptr: *mut c_char,
    pub n: usize,
    pub failed: c_int,
    pub provided: *mut *mut c_void,
}
pub type tre_mem_t = *mut tre_mem_struct;
extern "C" {
    pub fn tre_mem_new_impl(provided: c_int, provided_block: *mut c_void) -> tre_mem_t;
}
extern "C" {
    pub fn tre_mem_alloc_impl(
        mem: tre_mem_t,
        provided: c_int,
        provided_block: *mut c_void,
        zero: c_int,
        size: usize,
    ) -> *mut c_void;
}
extern "C" {
    pub fn tre_mem_destroy(mem: tre_mem_t);
}
