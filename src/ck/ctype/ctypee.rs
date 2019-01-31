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
use libc::*;
use crate::ck::ast::aste::*;
use crate::ck::def::defe::*;
use crate::ck::dynl::dynle::*;
use crate::ck::err::erre::*;
use crate::ck::oo::ooe::*;
use crate::ck::util::string::stringe::*;
use crate::dts::*;
use std::collections::HashMap;
///* ChucK type-checker. traverses the abstract syntax tree, and assigns and validates
///* types for each part of the tree. If a semantic error is encountered, a message is
///* generated and returned. if a program type-checks, the output is returned to be
///* passed to the code emitter.
///* I handwrote the parts that bindgen didn't see.
//-----------------------------------------------------------------------------
// name: enum te_Type
// desc: basic, default ChucK types
//-----------------------------------------------------------------------------
pub const te_Type_te_none: te_Type = 0;
pub const te_Type_te_int: te_Type = 1;
pub const te_Type_te_uint: te_Type = 2;
pub const te_Type_te_single: te_Type = 3;
pub const te_Type_te_float: te_Type = 4;
pub const te_Type_te_double: te_Type = 5;
pub const te_Type_te_time: te_Type = 6;
pub const te_Type_te_dur: te_Type = 7;
pub const te_Type_te_complex: te_Type = 8;
pub const te_Type_te_polar: te_Type = 9;
pub const te_Type_te_string: te_Type = 10;
pub const te_Type_te_thread: te_Type = 11;
pub const te_Type_te_shred: te_Type = 12;
pub const te_Type_te_class: te_Type = 13;
pub const te_Type_te_function: te_Type = 14;
pub const te_Type_te_object: te_Type = 15;
pub const te_Type_te_user: te_Type = 16;
pub const te_Type_te_array: te_Type = 17;
pub const te_Type_te_null: te_Type = 18;
pub const te_Type_te_ugen: te_Type = 19;
pub const te_Type_te_uana: te_Type = 20;
pub const te_Type_te_event: te_Type = 21;
pub const te_Type_te_void: te_Type = 22;
pub const te_Type_te_stdout: te_Type = 23;
pub const te_Type_te_stderr: te_Type = 24;
pub const te_Type_te_adc: te_Type = 25;
pub const te_Type_te_dac: te_Type = 26;
pub const te_Type_te_bunghole: te_Type = 27;
pub const te_Type_te_uanablob: te_Type = 28;
pub const te_Type_te_io: te_Type = 29;
pub const te_Type_te_fileio: te_Type = 30;
pub const te_Type_te_chout: te_Type = 31;
pub const te_Type_te_cherr: te_Type = 32;
pub const te_Type_te_multi: te_Type = 33;
pub const te_Type_te_vec3: te_Type = 34;
pub const te_Type_te_vec4: te_Type = 35;
pub const te_Type_te_vector: te_Type = 36;
pub type te_Type = u32;
//-----------------------------------------------------------------------------
// name: enum te_GlobalType
// desc: ChucK types for global vars: int, float, (subclass of) Event
//       (REFACTOR-2017)
//-----------------------------------------------------------------------------
pub const te_GlobalType_te_globalInt: te_GlobalType = 0;
pub const te_GlobalType_te_globalFloat: te_GlobalType = 1;
pub const te_GlobalType_te_globalEvent: te_GlobalType = 2;
pub type te_GlobalType = u32;
//-----------------------------------------------------------------------------
// name: enum te_HowMuch
// desc: how much to scan/type check
//-----------------------------------------------------------------------------
pub const te_HowMuch_te_do_all: te_HowMuch = 0;
pub const te_HowMuch_te_do_classes_only: te_HowMuch = 1;
pub const te_HowMuch_te_do_no_classes: te_HowMuch = 2;
pub type te_HowMuch = u32;
//-----------------------------------------------------------------------------
// name: struct Chuck_Scope
// desc: scoping structure
//-----------------------------------------------------------------------------
#[repr(C)]
pub struct Chuck_Scope {
    pub scope: Vec<f64>,
    pub commit_map: HashMap::new,
}
impl Chuck_Scope {
    // pub fn push() {
    //     scope.push_back(map<S_Symbol, *Chuck_VM_Object>);
    // }
    pub fn pop() {
        assert!(self.scope.len() != 0);
        self.scope.back();
        self.scope.pop_back();
    }
    pub fn reset() {
        self.scope.clear();
        Chuck_Scope::push(self);
    }
    pub fn commit() {
        assert!(self.scope.len() != 0);
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Multi {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_Namespace {
    pub _base: Chuck_VM_Object,
    pub type_: Chuck_Scope,
    pub value: Chuck_Scope,
    pub func: Chuck_Scope,
    pub obj_v_table: Chuck_VTable,
    pub class_data: *mut c_uchar,
    pub class_data_size: c_ulong,
    pub name: string,
    pub pre_ctor: *mut Chuck_VM_Code,
    pub dtor: *mut Chuck_VM_Code,
    pub parent: *mut Chuck_Namespace,
    pub offset: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}lookup_type"]
    pub fn Chuck_Namespace_lookup_type(
        this: *mut Chuck_Namespace,
        name: *const string,
        climb: c_long,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}lookup_type"]
    pub fn Chuck_Namespace_lookup_type1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: c_long,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}lookup_value"]
    pub fn Chuck_Namespace_lookup_value(
        this: *mut Chuck_Namespace,
        name: *const string,
        climb: c_long,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}lookup_value"]
    pub fn Chuck_Namespace_lookup_value1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: c_long,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}lookup_func"]
    pub fn Chuck_Namespace_lookup_func(
        this: *mut Chuck_Namespace,
        name: *const string,
        climb: c_long,
    ) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}lookup_func"]
    pub fn Chuck_Namespace_lookup_func1(
        this: *mut Chuck_Namespace,
        name: S_Symbol,
        climb: c_long,
    ) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}commit"]
    pub fn Chuck_Namespace_commit(this: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}rollback"]
    pub fn Chuck_Namespace_rollback(this: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}get_types"]
    pub fn Chuck_Namespace_get_types(this: *mut Chuck_Namespace, out: *mut Vec<f64>);
}
extern "C" {
    #[link_name = "\u{1}get_values"]
    pub fn Chuck_Namespace_get_values(this: *mut Chuck_Namespace, out: *mut Vec<f64>);
}
extern "C" {
    #[link_name = "\u{1}get_funcs"]
    pub fn Chuck_Namespace_get_funcs(this: *mut Chuck_Namespace, out: *mut Vec<f64>);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Namespace"]
    pub fn Chuck_Namespace_Chuck_Namespace(this: *mut Chuck_Namespace);
}
impl Chuck_Namespace {
    #[inline]
    pub unsafe fn lookup_type(&mut self, name: *const string, climb: c_long) -> *mut Chuck_Type {
        Chuck_Namespace_lookup_type(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_type1(&mut self, name: S_Symbol, climb: c_long) -> *mut Chuck_Type {
        Chuck_Namespace_lookup_type1(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_value(&mut self, name: *const string, climb: c_long) -> *mut Chuck_Value {
        Chuck_Namespace_lookup_value(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_value1(&mut self, name: S_Symbol, climb: c_long) -> *mut Chuck_Value {
        Chuck_Namespace_lookup_value1(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_func(&mut self, name: *const string, climb: c_long) -> *mut Chuck_Func {
        Chuck_Namespace_lookup_func(self, name, climb)
    }
    #[inline]
    pub unsafe fn lookup_func1(&mut self, name: S_Symbol, climb: c_long) -> *mut Chuck_Func {
        Chuck_Namespace_lookup_func1(self, name, climb)
    }
    #[inline]
    pub unsafe fn commit(&mut self) {
        Chuck_Namespace_commit(self)
    }
    #[inline]
    pub unsafe fn rollback(&mut self) {
        Chuck_Namespace_rollback(self)
    }
    #[inline]
    pub unsafe fn get_types(&mut self, out: *mut Vec<f64>) {
        Chuck_Namespace_get_types(self, out)
    }
    #[inline]
    pub unsafe fn get_values(&mut self, out: *mut Vec<f64>) {
        Chuck_Namespace_get_values(self, out)
    }
    #[inline]
    pub unsafe fn get_funcs(&mut self, out: *mut Vec<f64>) {
        Chuck_Namespace_get_funcs(self, out)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Namespace_Chuck_Namespace(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Namespace_destructor"]
    pub fn Chuck_Namespace_Chuck_Namespace_destructor(this: *mut Chuck_Namespace);
}
#[repr(C)]
pub struct Chuck_Context {
    pub _base: Chuck_VM_Object,
    pub filename: string,
    pub full_path: string,
    pub parse_tree: a_Program,
    pub nspc: *mut Chuck_Namespace,
    pub public_class_def: a_Class_Def,
    pub has_error: c_ulong,
    pub progress: c_ulong,
    pub new_types: Vec<f64>,
    pub new_values: Vec<f64>,
    pub new_funcs: Vec<f64>,
    pub new_nspc: Vec<f64>,
    pub commit_map: HashMap::new,
}
pub const Chuck_Context_P_NONE: Chuck_Context__bindgen_ty_1 = 0;
pub const Chuck_Context_P_CLASSES_ONLY: Chuck_Context__bindgen_ty_1 = 1;
pub const Chuck_Context_P_ALL: Chuck_Context__bindgen_ty_1 = 2;
pub type Chuck_Context__bindgen_ty_1 = u32;
extern "C" {
    #[link_name = "\u{1}add_commit_candidate"]
    pub fn Chuck_Context_add_commit_candidate(this: *mut Chuck_Context, nspc: *mut Chuck_Namespace);
}
extern "C" {
    #[link_name = "\u{1}commit"]
    pub fn Chuck_Context_commit(this: *mut Chuck_Context);
}
extern "C" {
    #[link_name = "\u{1}rollback"]
    pub fn Chuck_Context_rollback(this: *mut Chuck_Context);
}
extern "C" {
    #[link_name = "\u{1}code"]
    pub fn Chuck_Context_code(this: *mut Chuck_Context) -> *mut Chuck_VM_Code;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Type"]
    pub fn Chuck_Context_new_Chuck_Type(
        this: *mut Chuck_Context,
        env: *mut Chuck_Env,
    ) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Value"]
    pub fn Chuck_Context_new_Chuck_Value(
        this: *mut Chuck_Context,
        t: *mut Chuck_Type,
        name: *const string,
    ) -> *mut Chuck_Value;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Func"]
    pub fn Chuck_Context_new_Chuck_Func(this: *mut Chuck_Context) -> *mut Chuck_Func;
}
extern "C" {
    #[link_name = "\u{1}new_Chuck_Namespace"]
    pub fn Chuck_Context_new_Chuck_Namespace(this: *mut Chuck_Context) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Context"]
    pub fn Chuck_Context_Chuck_Context(this: *mut Chuck_Context);
}
impl Chuck_Context {
    #[inline]
    pub unsafe fn add_commit_candidate(&mut self, nspc: *mut Chuck_Namespace) {
        Chuck_Context_add_commit_candidate(self, nspc)
    }
    #[inline]
    pub unsafe fn commit(&mut self) {
        Chuck_Context_commit(self)
    }
    #[inline]
    pub unsafe fn rollback(&mut self) {
        Chuck_Context_rollback(self)
    }
    #[inline]
    pub unsafe fn code(&mut self) -> *mut Chuck_VM_Code {
        Chuck_Context_code(self)
    }
    #[inline]
    pub unsafe fn new_Chuck_Type(&mut self, env: *mut Chuck_Env) -> *mut Chuck_Type {
        Chuck_Context_new_Chuck_Type(self, env)
    }
    #[inline]
    pub unsafe fn new_Chuck_Value(
        &mut self,
        t: *mut Chuck_Type,
        name: *const string,
    ) -> *mut Chuck_Value {
        Chuck_Context_new_Chuck_Value(self, t, name)
    }
    #[inline]
    pub unsafe fn new_Chuck_Func(&mut self) -> *mut Chuck_Func {
        Chuck_Context_new_Chuck_Func(self)
    }
    #[inline]
    pub unsafe fn new_Chuck_Namespace(&mut self) -> *mut Chuck_Namespace {
        Chuck_Context_new_Chuck_Namespace(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Context_Chuck_Context(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Context_destructor"]
    pub fn Chuck_Context_Chuck_Context_destructor(this: *mut Chuck_Context);
}
#[repr(C)]
pub struct Chuck_Env {
    pub _base: Chuck_VM_Object,
    pub m_carrier: *mut Chuck_Carrier,
    pub global_nspc: *mut Chuck_Namespace,
    pub global_context: Chuck_Context,
    pub user_nspc: *mut Chuck_Namespace,
    pub nspc_stack: Vec<f64>,
    pub curr: *mut Chuck_Namespace,
    pub class_stack: Vec<f64>,
    pub class_def: *mut Chuck_Type,
    pub func: *mut Chuck_Func,
    pub class_scope: c_ulong,
    pub contexts: Vec<f64>,
    pub context: *mut Chuck_Context,
    pub breaks: Vec<f64>,
    pub key_words: HashMap::new,
    pub key_types: HashMap::new,
    pub key_values: HashMap::new,
    pub deprecated: HashMap::new,
    pub deprecate_level: c_long,
    pub t_void: *mut Chuck_Type,
    pub t_int: *mut Chuck_Type,
    pub t_float: *mut Chuck_Type,
    pub t_time: *mut Chuck_Type,
    pub t_dur: *mut Chuck_Type,
    pub t_complex: *mut Chuck_Type,
    pub t_polar: *mut Chuck_Type,
    pub t_vec3: *mut Chuck_Type,
    pub t_vec4: *mut Chuck_Type,
    pub t_null: *mut Chuck_Type,
    pub t_function: *mut Chuck_Type,
    pub t_object: *mut Chuck_Type,
    pub t_array: *mut Chuck_Type,
    pub t_string: *mut Chuck_Type,
    pub t_event: *mut Chuck_Type,
    pub t_ugen: *mut Chuck_Type,
    pub t_uana: *mut Chuck_Type,
    pub t_uanablob: *mut Chuck_Type,
    pub t_shred: *mut Chuck_Type,
    pub t_io: *mut Chuck_Type,
    pub t_fileio: *mut Chuck_Type,
    pub t_chout: *mut Chuck_Type,
    pub t_cherr: *mut Chuck_Type,
    pub t_thread: *mut Chuck_Type,
    pub t_class: *mut Chuck_Type,
    pub t_dac: *mut Chuck_Type,
    pub t_adc: *mut Chuck_Type,
}
extern "C" {
    #[link_name = "\u{1}set_carrier"]
    pub fn Chuck_Env_set_carrier(this: *mut Chuck_Env, carrier: *mut Chuck_Carrier);
}
extern "C" {
    #[link_name = "\u{1}vm"]
    pub fn Chuck_Env_vm(this: *mut Chuck_Env) -> *mut Chuck_VM;
}
extern "C" {
    #[link_name = "\u{1}global"]
    pub fn Chuck_Env_global(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}user"]
    pub fn Chuck_Env_user(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}reset"]
    pub fn Chuck_Env_reset(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}load_user_namespace"]
    pub fn Chuck_Env_load_user_namespace(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}clear_user_namespace"]
    pub fn Chuck_Env_clear_user_namespace(this: *mut Chuck_Env);
}
extern "C" {
    #[link_name = "\u{1}nspc_top"]
    pub fn Chuck_Env_nspc_top(this: *mut Chuck_Env) -> *mut Chuck_Namespace;
}
extern "C" {
    #[link_name = "\u{1}class_top"]
    pub fn Chuck_Env_class_top(this: *mut Chuck_Env) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}is_global"]
    pub fn Chuck_Env_is_global(this: *mut Chuck_Env) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Env"]
    pub fn Chuck_Env_Chuck_Env(this: *mut Chuck_Env);
}
impl Chuck_Env {
    #[inline]
    pub unsafe fn set_carrier(&mut self, carrier: *mut Chuck_Carrier) {
        Chuck_Env_set_carrier(self, carrier)
    }
    #[inline]
    pub unsafe fn vm(&mut self) -> *mut Chuck_VM {
        Chuck_Env_vm(self)
    }
    #[inline]
    pub unsafe fn global(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_global(self)
    }
    #[inline]
    pub unsafe fn user(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_user(self)
    }
    #[inline]
    pub unsafe fn reset(&mut self) {
        Chuck_Env_reset(self)
    }
    #[inline]
    pub unsafe fn load_user_namespace(&mut self) {
        Chuck_Env_load_user_namespace(self)
    }
    #[inline]
    pub unsafe fn clear_user_namespace(&mut self) {
        Chuck_Env_clear_user_namespace(self)
    }
    #[inline]
    pub unsafe fn nspc_top(&mut self) -> *mut Chuck_Namespace {
        Chuck_Env_nspc_top(self)
    }
    #[inline]
    pub unsafe fn class_top(&mut self) -> *mut Chuck_Type {
        Chuck_Env_class_top(self)
    }
    #[inline]
    pub unsafe fn is_global(&mut self) -> c_ulong {
        Chuck_Env_is_global(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Env_Chuck_Env(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Env_destructor"]
    pub fn Chuck_Env_Chuck_Env_destructor(this: *mut Chuck_Env);
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_UGen_Info {
    pub _base: Chuck_VM_Object,
    pub tick: f_tick,
    pub tickf: f_tickf,
    pub pmsg: f_pmsg,
    pub num_ins: c_ulong,
    pub num_outs: c_ulong,
    pub tock: f_tock,
    pub num_ins_ana: c_ulong,
    pub num_outs_ana: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_UGen_Info"]
    pub fn Chuck_UGen_Info_Chuck_UGen_Info(this: *mut Chuck_UGen_Info);
}
impl Chuck_UGen_Info {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_UGen_Info_Chuck_UGen_Info(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Chuck_Type {
    pub _base: Chuck_VM_Object,
    pub xid: te_Type,
    pub name: string,
    pub parent: *mut Chuck_Type,
    pub size: c_ulong,
    pub owner: *mut Chuck_Namespace,
    pub __bindgen_anon_1: Chuck_Type__bindgen_ty_1,
    pub array_depth: c_ulong,
    pub obj_size: c_ulong,
    pub info: *mut Chuck_Namespace,
    pub func: *mut Chuck_Func,
    pub def: a_Class_Def,
    pub ugen_info: *mut Chuck_UGen_Info,
    pub is_copy: c_ulong,
    pub is_complete: c_ulong,
    pub has_constructor: c_ulong,
    pub has_destructor: c_ulong,
    pub allocator: f_alloc,
    pub doc: string,
    pub examples: Vec<f64>,
    pub ret: string,
    pub m_env: *mut Chuck_Env,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union Chuck_Type__bindgen_ty_1 {
    pub array_type: *mut Chuck_Type,
    pub actual_type: *mut Chuck_Type,
    _bindgen_union_align: u64,
}
extern "C" {
    #[link_name = "\u{1}reset"]
    pub fn Chuck_Type_reset(this: *mut Chuck_Type);
}
extern "C" {
    #[link_name = "\u{1}copy"]
    pub fn Chuck_Type_copy(this: *const Chuck_Type, env: *mut Chuck_Env) -> *mut Chuck_Type;
}
extern "C" {
    #[link_name = "\u{1}str"]
    pub fn Chuck_Type_str(this: *mut Chuck_Type) -> *const string;
}
extern "C" {
    #[link_name = "\u{1}c_name"]
    pub fn Chuck_Type_c_name(this: *mut Chuck_Type) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Type"]
    pub fn Chuck_Type_Chuck_Type(
        this: *mut Chuck_Type,
        env: *mut Chuck_Env,
        _id: te_Type,
        _n: *const string,
        _p: *mut Chuck_Type,
        _s: c_ulong,
    );
}
impl Chuck_Type {
    #[inline]
    pub unsafe fn reset(&mut self) {
        Chuck_Type_reset(self)
    }
    #[inline]
    pub unsafe fn copy(&self, env: *mut Chuck_Env) -> *mut Chuck_Type {
        Chuck_Type_copy(self, env)
    }
    #[inline]
    pub unsafe fn str(&mut self) -> *const string {
        Chuck_Type_str(self)
    }
    #[inline]
    pub unsafe fn c_name(&mut self) -> *const c_char {
        Chuck_Type_c_name(self)
    }
    #[inline]
    pub unsafe fn new(
        env: *mut Chuck_Env,
        _id: te_Type,
        _n: *const string,
        _p: *mut Chuck_Type,
        _s: c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Type_Chuck_Type(&mut __bindgen_tmp, env, _id, _n, _p, _s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Type_destructor"]
    pub fn Chuck_Type_Chuck_Type_destructor(this: *mut Chuck_Type);
}
#[repr(C)]
pub struct Chuck_Value {
    pub _base: Chuck_VM_Object,
    pub type_: *mut Chuck_Type,
    pub name: string,
    pub offset: c_ulong,
    pub addr: *mut c_void,
    pub is_const: c_ulong,
    pub is_member: c_ulong,
    pub is_static: c_ulong,
    pub is_context_global: c_ulong,
    pub is_decl_checked: c_ulong,
    pub is_global: c_ulong,
    pub access: c_ulong,
    pub owner: *mut Chuck_Namespace,
    pub owner_class: *mut Chuck_Type,
    pub func_ref: *mut Chuck_Func,
    pub func_num_overloads: c_long,
    pub doc: string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Value"]
    pub fn Chuck_Value_Chuck_Value(
        this: *mut Chuck_Value,
        t: *mut Chuck_Type,
        n: *const string,
        a: *mut c_void,
        c: c_ulong,
        acc: c_ulong,
        o: *mut Chuck_Namespace,
        oc: *mut Chuck_Type,
        s: c_ulong,
    );
}
impl Chuck_Value {
    #[inline]
    pub unsafe fn new(
        t: *mut Chuck_Type,
        n: *const string,
        a: *mut c_void,
        c: c_ulong,
        acc: c_ulong,
        o: *mut Chuck_Namespace,
        oc: *mut Chuck_Type,
        s: c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Value_Chuck_Value(&mut __bindgen_tmp, t, n, a, c, acc, o, oc, s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Value_destructor"]
    pub fn Chuck_Value_Chuck_Value_destructor(this: *mut Chuck_Value);
}
#[repr(C)]
pub struct Chuck_Func {
    pub _base: Chuck_VM_Object,
    pub name: string,
    pub def: a_Func_Def,
    pub code: *mut Chuck_VM_Code,
    pub is_member: c_ulong,
    pub vt_index: c_ulong,
    pub value_ref: *mut Chuck_Value,
    pub next: *mut Chuck_Func,
    pub up: *mut Chuck_Value,
    pub doc: string,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Func"]
    pub fn Chuck_Func_Chuck_Func(this: *mut Chuck_Func);
}
impl Chuck_Func {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Func_Chuck_Func(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Func_destructor"]
    pub fn Chuck_Func_Chuck_Func_destructor(this: *mut Chuck_Func);
}
extern "C" {
    pub fn type_engine_init(carrier: *mut Chuck_Carrier) -> *mut Chuck_Env;
}
extern "C" {
    pub fn type_engine_shutdown(env: *mut Chuck_Env);
}
extern "C" {
    pub fn type_engine_load_context(env: *mut Chuck_Env, context: *mut Chuck_Context) -> c_ulong;
}
extern "C" {
    pub fn type_engine_unload_context(env: *mut Chuck_Env) -> c_ulong;
}
extern "C" {
    pub fn type_engine_check_prog(
        env: *mut Chuck_Env,
        prog: a_Program,
        filename: *const string,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_make_context(prog: a_Program, filename: *const string)
        -> *mut Chuck_Context;
}
extern "C" {
    pub fn type_engine_check_context(
        env: *mut Chuck_Env,
        context: *mut Chuck_Context,
        how_much: te_HowMuch,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_check_stmt(env: *mut Chuck_Env, stmt: a_Stmt) -> c_ulong;
}
extern "C" {
    pub fn type_engine_check_exp(env: *mut Chuck_Env, exp: a_Exp) -> t_CKTYPE;
}
extern "C" {
    pub fn type_engine_add_dll(
        env: *mut Chuck_Env,
        dll: *mut Chuck_DLL,
        nspc: *const string,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_add_dll2(
        env: *mut Chuck_Env,
        dll: *mut Chuck_DLL,
        dest: *const string,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_add_class_from_dl(env: *mut Chuck_Env, c: *mut Chuck_DL_Class) -> c_ulong;
}
extern "C" {
    pub fn equals(lhs: *mut Chuck_Type, rhs: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn isa(lhs: *mut Chuck_Type, rhs: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn isprim(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn isobj(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn isfunc(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn iskindofint(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn getkindof(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn type_engine_import_class_begin(
        env: *mut Chuck_Env,
        type_: *mut Chuck_Type,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        doc: *const c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_import_ugen_begin(
        env: *mut Chuck_Env,
        name: *const c_char,
        parent: *const c_char,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        tick: f_tick,
        tickf: f_tickf,
        pmsg: f_pmsg,
        num_ins: c_ulong,
        num_outs: c_ulong,
        doc: *const c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_import_uana_begin(
        env: *mut Chuck_Env,
        name: *const c_char,
        parent: *const c_char,
        where_: *mut Chuck_Namespace,
        pre_ctor: f_ctor,
        dtor: f_dtor,
        tick: f_tick,
        tock: f_tock,
        pmsg: f_pmsg,
        num_ins: c_ulong,
        num_outs: c_ulong,
        num_ins_ana: c_ulong,
        num_outs_ana: c_ulong,
        doc: *const c_char,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_import_mfun(env: *mut Chuck_Env, mfun: *mut Chuck_DL_Func) -> c_ulong;
}
extern "C" {
    pub fn type_engine_import_sfun(env: *mut Chuck_Env, sfun: *mut Chuck_DL_Func) -> c_ulong;
}
extern "C" {
    pub fn type_engine_import_mvar(
        env: *mut Chuck_Env,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
        doc: *const c_char,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_import_svar(
        env: *mut Chuck_Env,
        type_: *const c_char,
        name: *const c_char,
        is_const: c_ulong,
        addr: c_ulong,
        doc: *const c_char,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_import_ugen_ctrl(
        env: *mut Chuck_Env,
        type_: *const c_char,
        name: *const c_char,
        ctrl: f_ctrl,
        write: c_ulong,
        read: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_import_add_ex(env: *mut Chuck_Env, ex: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn type_engine_import_class_end(env: *mut Chuck_Env) -> c_ulong;
}
extern "C" {
    pub fn type_engine_register_deprecate(
        env: *mut Chuck_Env,
        former: *const string,
        latter: *const string,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_check_reserved(
        env: *mut Chuck_Env,
        xid: *const string,
        pos: c_int,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_check_primitive(env: *mut Chuck_Env, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn type_engine_compat_func(
        lhs: a_Func_Def,
        rhs: a_Func_Def,
        pos: c_int,
        err: *mut string,
        print: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_get_deprecate(
        env: *mut Chuck_Env,
        from: *const string,
        to: *mut string,
    ) -> c_ulong;
}
extern "C" {
    pub fn type_engine_find_common_anc(
        lhs: *mut Chuck_Type,
        rhs: *mut Chuck_Type,
    ) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_find_type(env: *mut Chuck_Env, path: a_Id_List) -> *mut Chuck_Type;
}
extern "C" {
    pub fn type_engine_find_value(type_: *mut Chuck_Type, xid: *const string) -> *mut Chuck_Value;
}
extern "C" {
    pub fn type_engine_find_nspc(env: *mut Chuck_Env, path: a_Id_List) -> *mut Chuck_Namespace;
}
extern "C" {
    /// spencer: added this into function to provide the same logic path
    /// for type_engine_check_exp_decl() and ck_add_mvar() when they determine
    /// offsets for mvars -- added 1.3.0.0
    pub fn type_engine_next_offset(current_offset: c_ulong, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn verify_array(array: a_Array_Sub) -> c_ulong;
}
extern "C" {
    pub fn new_array_type(
        env: *mut Chuck_Env,
        array_parent: *mut Chuck_Type,
        depth: c_ulong,
        base_type: *mut Chuck_Type,
        owner_nspc: *mut Chuck_Namespace,
    ) -> *mut Chuck_Type;
}
