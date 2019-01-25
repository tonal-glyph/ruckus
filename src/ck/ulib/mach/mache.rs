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
use crate::chuck_compile_h_edited::*;
use crate::chuck_def_h_edited::*;
use crate::chuck_dl_h_edited::*;
use crate::chuck_otf_h_edited::*;
use crate::chuck_vm_h_edited::*;
///* class library for `Machine`
use libc::*;
pub fn DLLQUERY() {
    machine_query(QUERY: *mut Chuck_DL_Query);
    init();
}
pub fn proc_msg_func() {
    proc_msg_func(*Chuck_VM, *Chuck_Compiler, *Net_Msg, t_CKBOOL, cvoid);
}
pub fn init() {
    machine_init(compiler: Chuck_Compiler, func: *mut proc_msg_func());
}
pub fn intsize() {
    machine_intsize();
}
pub fn main() {
    DLLQUERY();
    // exports
    CK_DLL_SFUN(machine_crash_impl);
    CK_DLL_SFUN(machine_add_impl);
    CK_DLL_SFUN(machine_spork_impl);
    CK_DLL_SFUN(machine_remove_impl);
    CK_DLL_SFUN(machine_replace_impl);
    CK_DLL_SFUN(machine_status_impl);
    CK_DLL_SFUN(machine_intsize_impl);
    CK_DLL_SFUN(machine_shreds_impl);
}
