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
///* ChucK scanner. traverses the abstract syntax tree in several passes, and resolves types.
///* ChucK type-system / type-checker pre-scan
#![feature(libc)]
use libc::*;
use crate::ck::ast::aste::*;
use crate::ck::def::defe::*;
use crate::ck::ctype::ctypee::*;
use crate::ck::util::thread::threade::*;
extern "C" {
    pub fn type_engine_scan0_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan0_class_def(env: *mut Chuck_Env, def: a_Class_Def) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_class_def(env: *mut Chuck_Env, def: a_Class_Def) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_func_def(env: *mut Chuck_Env, def: a_Func_Def) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan1_exp_decl(env: *mut Chuck_Env, decl: a_Exp_Decl) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        val: te_HowMuch,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_class_def(env: *mut Chuck_Env, def: a_Class_Def) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_func_def(env: *mut Chuck_Env, def: a_Func_Def) -> c_ulong;
}
extern "C" {
    pub fn type_engine_scan2_exp_decl(env: *mut Chuck_Env, decl: a_Exp_Decl) -> c_ulong;
}
