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
///* tre regex stack
use libc::*;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tre_parse_ctx_t {
    pub mem: tre_mem_t,
    pub stack: *mut tre_stack_t,
    pub result: *mut tre_ast_node_t,
    pub re: *const tre_char_t,
    pub re_start: *const tre_char_t,
    pub re_end: *const tre_char_t,
    pub len: c_int,
    pub submatch_id: c_int,
    pub position: c_int,
    pub max_backref: c_int,
    pub have_approx: c_int,
    pub cflags: c_int,
    pub nofirstsub: c_int,
    pub params: [c_int; 9usize],
}
extern "C" {
    pub fn tre_parse(ctx: *mut tre_parse_ctx_t) -> reg_errcode_t;
}
