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
use crate::chuck_absyn_h_edited::*;
use crate::chuck_def_h_edited::*;
use crate::chuck_type_h_edited::*;
use crate::util_thread_h_edited::*;
///* ChucK scanner. traverses the abstract syntax tree in several passes, and resolves types.
///* ChucK type-system / type-checker pre-scan
use libc::*;
extern "C" {
    pub fn type_engine_scan0_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan0_class_def(env: *mut Chuck_Env, def: a_Class_Def) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_class_def(env: *mut Chuck_Env, def: a_Class_Def) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_func_def(env: *mut Chuck_Env, def: a_Func_Def) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_exp_decl(env: *mut Chuck_Env, decl: a_Exp_Decl) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_class_def(env: *mut Chuck_Env, def: a_Class_Def) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_func_def(env: *mut Chuck_Env, def: a_Func_Def) -> libc::c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_exp_decl(env: *mut Chuck_Env, decl: a_Exp_Decl) -> libc::c_ulong;
}
