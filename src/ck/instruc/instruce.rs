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
//* ChucK VM instruction set. this contains the virtual instructions for the VM.
//* arithmetic, logical, memory, timing, and audio-specific instructions are all
//* part of the instruction set.
extern crate libc;
use crate::chuck_def_h_edited::*;
use crate::chuck_type_h_edited::*;
use crate::dts::*;
use crate::chuck_vm_h_edited::Chuck_VM;
use crate::chuck_compile_h_edited::Chuck_VM_Shred;
use crate::rtmidi_h_edited::*;
use libc::*;
use std::mem::*;
use std::option::Option;
#[repr(C)]
pub struct Chuck_Instr__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr {
    pub vtable_: *const Chuck_Instr__bindgen_vtable,
    pub m_linepos: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}set_linepos"]
    pub fn Chuck_Instr_set_linepos(this: *mut Chuck_Instr, linepos: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr"]
    pub fn Chuck_Instr_Chuck_Instr(this: *mut Chuck_Instr);
}
impl Chuck_Instr {
    #[inline]
    pub unsafe fn set_linepos(&mut self, linepos: c_ulong) {
        Chuck_Instr_set_linepos(self, linepos)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Chuck_Instr(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_destructor"]
    pub fn Chuck_Instr_Chuck_Instr_destructor(this: *mut Chuck_Instr);
}
extern "C" {
    #[link_name = "\u{1}name"]
    pub fn Chuck_Instr_name(this: *mut c_void) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Op {
    pub _base: Chuck_Instr,
    pub m_jmp: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Instr_Branch_Op_set(this: *mut Chuck_Instr_Branch_Op, jmp: c_ulong);
}
impl Chuck_Instr_Branch_Op {
    #[inline]
    pub unsafe fn set(&mut self, jmp: c_ulong) {
        Chuck_Instr_Branch_Op_set(self, jmp)
    }
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Branch_Op_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Unary_Op {
    pub _base: Chuck_Instr,
    pub m_val: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Instr_Unary_Op_set(this: *mut Chuck_Instr_Unary_Op, val: c_ulong);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Instr_Unary_Op_get(this: *mut Chuck_Instr_Unary_Op) -> c_ulong;
}
impl Chuck_Instr_Unary_Op {
    #[inline]
    pub unsafe fn set(&mut self, val: c_ulong) {
        Chuck_Instr_Unary_Op_set(self, val)
    }
    #[inline]
    pub unsafe fn get(&mut self) -> c_ulong {
        Chuck_Instr_Unary_Op_get(self)
    }
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Unary_Op_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Unary_Op2 {
    pub _base: Chuck_Instr,
    pub m_val: f64,
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Instr_Unary_Op2_set(this: *mut Chuck_Instr_Unary_Op2, val: f64);
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Instr_Unary_Op2_get(this: *mut Chuck_Instr_Unary_Op2) -> f64;
}
impl Chuck_Instr_Unary_Op2 {
    #[inline]
    pub unsafe fn set(&mut self, val: f64) {
        Chuck_Instr_Unary_Op2_set(self, val)
    }
    #[inline]
    pub unsafe fn get(&mut self) -> f64 {
        Chuck_Instr_Unary_Op2_get(self)
    }
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Unary_Op2_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Op {
    pub _base: Chuck_Instr,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_PreInc_int {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_PreInc_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_PostInc_int {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_PostInc_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_PreDec_int {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_PreDec_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_PostDec_int {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_PostDec_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dec_int_Addr {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dec_int_Addr"]
    pub fn Chuck_Instr_Dec_int_Addr_Chuck_Instr_Dec_int_Addr(
        this: *mut Chuck_Instr_Dec_int_Addr,
        src: c_ulong,
    );
}
impl Chuck_Instr_Dec_int_Addr {
    #[inline]
    pub unsafe fn new(src: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dec_int_Addr_Chuck_Instr_Dec_int_Addr(&mut __bindgen_tmp, src);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dec_int_Addr_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Complement_int {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Complement_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mod_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mod_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mod_int_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mod_int_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_int_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_int_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Times_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Times_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_int_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_int_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_double_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_double_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Times_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Times_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_double_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_double_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mod_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mod_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mod_double_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mod_double_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_complex {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_complex {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_complex_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_complex_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Times_complex {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Times_complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_complex {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_complex_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_complex_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_polar {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_polar_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_polar {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_polar_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_polar_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_polar_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Times_polar {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Times_polar_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_polar {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_polar_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_polar_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_polar_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_vec3 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_vec3 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_XProduct_vec3 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_XProduct_vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_vec4 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_vec4 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_XProduct_vec4 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_XProduct_vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_float_Times_vec3 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_float_Times_vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_vec3_Times_float {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_vec3_Times_float_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_vec3_Divide_float {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_vec3_Divide_float_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_float_Times_vec4 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_float_Times_vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_vec4_Times_float {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_vec4_Times_float_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_vec4_Divide_float {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_vec4_Divide_float_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_int_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_int_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mod_int_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mod_int_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_int_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_int_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Times_int_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Times_int_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_int_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_int_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_double_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_double_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_double_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_double_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Times_double_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Times_double_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_double_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_double_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mod_double_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mod_double_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_complex_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_complex_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_complex_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_complex_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Times_complex_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Times_complex_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_complex_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_complex_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_polar_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_polar_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_polar_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_polar_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Times_polar_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Times_polar_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Divide_polar_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Divide_polar_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_vec3_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_vec3_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_vec3_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_vec3_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_vec4_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_vec4_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Minus_vec4_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Minus_vec4_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_float_Times_vec3_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_float_Times_vec3_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_float_Times_vec4_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_float_Times_vec4_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_vec3_Divide_float_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_vec3_Divide_float_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_vec4_Divide_float_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_vec4_Divide_float_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_string {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_string_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_string_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_string_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_string_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_string_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_string_float {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_string_float_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_int_string {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_int_string_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_float_string {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_float_string_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_int_string_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_int_string_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Add_float_string_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Add_float_string_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Lt_int {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Lt_int"]
    pub fn Chuck_Instr_Branch_Lt_int_Chuck_Instr_Branch_Lt_int(
        this: *mut Chuck_Instr_Branch_Lt_int,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Lt_int {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Lt_int_Chuck_Instr_Branch_Lt_int(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Lt_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Gt_int {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Gt_int"]
    pub fn Chuck_Instr_Branch_Gt_int_Chuck_Instr_Branch_Gt_int(
        this: *mut Chuck_Instr_Branch_Gt_int,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Gt_int {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Gt_int_Chuck_Instr_Branch_Gt_int(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Gt_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Le_int {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Le_int"]
    pub fn Chuck_Instr_Branch_Le_int_Chuck_Instr_Branch_Le_int(
        this: *mut Chuck_Instr_Branch_Le_int,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Le_int {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Le_int_Chuck_Instr_Branch_Le_int(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Le_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Ge_int {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Ge_int"]
    pub fn Chuck_Instr_Branch_Ge_int_Chuck_Instr_Branch_Ge_int(
        this: *mut Chuck_Instr_Branch_Ge_int,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Ge_int {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Ge_int_Chuck_Instr_Branch_Ge_int(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Ge_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Eq_int {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Eq_int"]
    pub fn Chuck_Instr_Branch_Eq_int_Chuck_Instr_Branch_Eq_int(
        this: *mut Chuck_Instr_Branch_Eq_int,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Eq_int {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Eq_int_Chuck_Instr_Branch_Eq_int(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Eq_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Neq_int {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Neq_int"]
    pub fn Chuck_Instr_Branch_Neq_int_Chuck_Instr_Branch_Neq_int(
        this: *mut Chuck_Instr_Branch_Neq_int,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Neq_int {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Neq_int_Chuck_Instr_Branch_Neq_int(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Neq_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Lt_double {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Lt_double"]
    pub fn Chuck_Instr_Branch_Lt_double_Chuck_Instr_Branch_Lt_double(
        this: *mut Chuck_Instr_Branch_Lt_double,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Lt_double {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Lt_double_Chuck_Instr_Branch_Lt_double(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Lt_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Gt_double {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Gt_double"]
    pub fn Chuck_Instr_Branch_Gt_double_Chuck_Instr_Branch_Gt_double(
        this: *mut Chuck_Instr_Branch_Gt_double,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Gt_double {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Gt_double_Chuck_Instr_Branch_Gt_double(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Gt_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Le_double {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Le_double"]
    pub fn Chuck_Instr_Branch_Le_double_Chuck_Instr_Branch_Le_double(
        this: *mut Chuck_Instr_Branch_Le_double,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Le_double {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Le_double_Chuck_Instr_Branch_Le_double(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Le_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Ge_double {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Ge_double"]
    pub fn Chuck_Instr_Branch_Ge_double_Chuck_Instr_Branch_Ge_double(
        this: *mut Chuck_Instr_Branch_Ge_double,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Ge_double {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Ge_double_Chuck_Instr_Branch_Ge_double(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Ge_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Eq_double {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Eq_double"]
    pub fn Chuck_Instr_Branch_Eq_double_Chuck_Instr_Branch_Eq_double(
        this: *mut Chuck_Instr_Branch_Eq_double,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Eq_double {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Eq_double_Chuck_Instr_Branch_Eq_double(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Eq_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Neq_double {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Neq_double"]
    pub fn Chuck_Instr_Branch_Neq_double_Chuck_Instr_Branch_Neq_double(
        this: *mut Chuck_Instr_Branch_Neq_double,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Neq_double {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Neq_double_Chuck_Instr_Branch_Neq_double(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Neq_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Eq_int_IO_good {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Eq_int_IO_good"]
    pub fn Chuck_Instr_Branch_Eq_int_IO_good_Chuck_Instr_Branch_Eq_int_IO_good(
        this: *mut Chuck_Instr_Branch_Eq_int_IO_good,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Eq_int_IO_good {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Eq_int_IO_good_Chuck_Instr_Branch_Eq_int_IO_good(
            &mut __bindgen_tmp,
            jmp,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Eq_int_IO_good_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Branch_Neq_int_IO_good {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Branch_Neq_int_IO_good"]
    pub fn Chuck_Instr_Branch_Neq_int_IO_good_Chuck_Instr_Branch_Neq_int_IO_good(
        this: *mut Chuck_Instr_Branch_Neq_int_IO_good,
        jmp: c_ulong,
    );
}
impl Chuck_Instr_Branch_Neq_int_IO_good {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Branch_Neq_int_IO_good_Chuck_Instr_Branch_Neq_int_IO_good(
            &mut __bindgen_tmp,
            jmp,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Branch_Neq_int_IO_good_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Lt_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Lt_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Gt_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Gt_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Le_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Le_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Ge_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Ge_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Eq_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Eq_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Neq_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Neq_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Not_int {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Not_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Negate_int {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Negate_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Negate_double {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Negate_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Lt_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Lt_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Gt_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Gt_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Le_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Le_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Ge_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Ge_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Eq_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Eq_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Neq_double {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Neq_double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Eq_complex {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Eq_complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Neq_complex {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Neq_complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Eq_vec3 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Eq_vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Neq_vec3 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Neq_vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Eq_vec4 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Eq_vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Neq_vec4 {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Neq_vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_And {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_And_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Or {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Or_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Xor {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Xor_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Shift_Right {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Shift_Right_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Shift_Right_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Shift_Right_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Shift_Left {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Shift_Left_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Shift_Left_Reverse {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Shift_Left_Reverse_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_And_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_And_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Or_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Or_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Xor_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Xor_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Shift_Right_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Shift_Right_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Binary_Shift_Left_Assign {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Binary_Shift_Left_Assign_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_And {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_And_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Or {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Or_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Goto {
    pub _base: Chuck_Instr_Branch_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Goto"]
    pub fn Chuck_Instr_Goto_Chuck_Instr_Goto(this: *mut Chuck_Instr_Goto, jmp: c_ulong);
}
impl Chuck_Instr_Goto {
    #[inline]
    pub unsafe fn new(jmp: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Goto_Chuck_Instr_Goto(&mut __bindgen_tmp, jmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Goto_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Pop_Word {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Pop_Word_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Pop_Word2 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Pop_Word2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Pop_Word3 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Pop_Word3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Pop_Word4 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Pop_Word4"]
    pub fn Chuck_Instr_Reg_Pop_Word4_Chuck_Instr_Reg_Pop_Word4(
        this: *mut Chuck_Instr_Reg_Pop_Word4,
        num: c_ulong,
    );
}
impl Chuck_Instr_Reg_Pop_Word4 {
    #[inline]
    pub unsafe fn new(num: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Pop_Word4_Chuck_Instr_Reg_Pop_Word4(&mut __bindgen_tmp, num);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Pop_Word4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Pop_Mem {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Pop_Mem_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Pop_Mem2 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Pop_Mem2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Imm {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Imm"]
    pub fn Chuck_Instr_Reg_Push_Imm_Chuck_Instr_Reg_Push_Imm(
        this: *mut Chuck_Instr_Reg_Push_Imm,
        val: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Imm {
    #[inline]
    pub unsafe fn new(val: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Imm_Chuck_Instr_Reg_Push_Imm(&mut __bindgen_tmp, val);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Imm_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Imm2 {
    pub _base: Chuck_Instr_Unary_Op2,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Imm2"]
    pub fn Chuck_Instr_Reg_Push_Imm2_Chuck_Instr_Reg_Push_Imm2(
        this: *mut Chuck_Instr_Reg_Push_Imm2,
        val: f64,
    );
}
impl Chuck_Instr_Reg_Push_Imm2 {
    #[inline]
    pub unsafe fn new(val: f64) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Imm2_Chuck_Instr_Reg_Push_Imm2(&mut __bindgen_tmp, val);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Imm2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Imm4 {
    pub _base: Chuck_Instr_Unary_Op2,
    pub m_val2: f64,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Imm4"]
    pub fn Chuck_Instr_Reg_Push_Imm4_Chuck_Instr_Reg_Push_Imm4(
        this: *mut Chuck_Instr_Reg_Push_Imm4,
        x: f64,
        y: f64,
    );
}
impl Chuck_Instr_Reg_Push_Imm4 {
    #[inline]
    pub unsafe fn new(x: f64, y: f64) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Imm4_Chuck_Instr_Reg_Push_Imm4(&mut __bindgen_tmp, x, y);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Imm4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Dup_Last {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Dup_Last_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Dup_Last2 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Dup_Last2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Dup_Last_As_Pointer {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Dup_Last_As_Pointer"]
    pub fn Chuck_Instr_Reg_Dup_Last_As_Pointer_Chuck_Instr_Reg_Dup_Last_As_Pointer(
        this: *mut Chuck_Instr_Reg_Dup_Last_As_Pointer,
        sizeInWords: c_ulong,
    );
}
impl Chuck_Instr_Reg_Dup_Last_As_Pointer {
    #[inline]
    pub unsafe fn new(sizeInWords: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Dup_Last_As_Pointer_Chuck_Instr_Reg_Dup_Last_As_Pointer(
            &mut __bindgen_tmp,
            sizeInWords,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Dup_Last_As_Pointer_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Now {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Now_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Me {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Me_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_This {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_This_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Start {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Start_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Maybe {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Maybe_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Mem {
    pub _base: Chuck_Instr_Unary_Op,
    pub base: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Mem"]
    pub fn Chuck_Instr_Reg_Push_Mem_Chuck_Instr_Reg_Push_Mem(
        this: *mut Chuck_Instr_Reg_Push_Mem,
        src: c_ulong,
        use_base: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Mem {
    #[inline]
    pub unsafe fn new(src: c_ulong, use_base: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Mem_Chuck_Instr_Reg_Push_Mem(&mut __bindgen_tmp, src, use_base);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Mem_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Reg_Push_Mem_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Mem2 {
    pub _base: Chuck_Instr_Unary_Op,
    pub base: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Mem2"]
    pub fn Chuck_Instr_Reg_Push_Mem2_Chuck_Instr_Reg_Push_Mem2(
        this: *mut Chuck_Instr_Reg_Push_Mem2,
        src: c_ulong,
        use_base: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Mem2 {
    #[inline]
    pub unsafe fn new(src: c_ulong, use_base: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Mem2_Chuck_Instr_Reg_Push_Mem2(&mut __bindgen_tmp, src, use_base);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Mem2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Reg_Push_Mem2_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Mem4 {
    pub _base: Chuck_Instr_Unary_Op,
    pub base: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Mem4"]
    pub fn Chuck_Instr_Reg_Push_Mem4_Chuck_Instr_Reg_Push_Mem4(
        this: *mut Chuck_Instr_Reg_Push_Mem4,
        src: c_ulong,
        use_base: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Mem4 {
    #[inline]
    pub unsafe fn new(src: c_ulong, use_base: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Mem4_Chuck_Instr_Reg_Push_Mem4(&mut __bindgen_tmp, src, use_base);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Mem4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Reg_Push_Mem4_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Mem_Vec3 {
    pub _base: Chuck_Instr_Unary_Op,
    pub base: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Mem_Vec3"]
    pub fn Chuck_Instr_Reg_Push_Mem_Vec3_Chuck_Instr_Reg_Push_Mem_Vec3(
        this: *mut Chuck_Instr_Reg_Push_Mem_Vec3,
        src: c_ulong,
        use_base: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Mem_Vec3 {
    #[inline]
    pub unsafe fn new(src: c_ulong, use_base: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Mem_Vec3_Chuck_Instr_Reg_Push_Mem_Vec3(
            &mut __bindgen_tmp,
            src,
            use_base,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Mem_Vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Reg_Push_Mem_Vec3_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Mem_Vec4 {
    pub _base: Chuck_Instr_Unary_Op,
    pub base: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Mem_Vec4"]
    pub fn Chuck_Instr_Reg_Push_Mem_Vec4_Chuck_Instr_Reg_Push_Mem_Vec4(
        this: *mut Chuck_Instr_Reg_Push_Mem_Vec4,
        src: c_ulong,
        use_base: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Mem_Vec4 {
    #[inline]
    pub unsafe fn new(src: c_ulong, use_base: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Mem_Vec4_Chuck_Instr_Reg_Push_Mem_Vec4(
            &mut __bindgen_tmp,
            src,
            use_base,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Mem_Vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Reg_Push_Mem_Vec4_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
pub struct Chuck_Instr_Reg_Push_Global {
    pub _base: Chuck_Instr_Unary_Op,
    pub m_name: crate::rtmidi_h_edited::string(&mut c_char),
    pub m_type: te_GlobalType,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Global"]
    pub fn Chuck_Instr_Reg_Push_Global_Chuck_Instr_Reg_Push_Global(
        this: *mut Chuck_Instr_Reg_Push_Global,
        name: crate::rtmidi_h_edited::string(&mut c_char),
        type_: te_GlobalType,
    );
}
impl Chuck_Instr_Reg_Push_Global {
    #[inline]
    pub unsafe fn new(name: crate::rtmidi_h_edited::string(&mut c_char), type_: te_GlobalType) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Global_Chuck_Instr_Reg_Push_Global(&mut __bindgen_tmp, name, type_);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Global_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Reg_Push_Global_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Mem_Addr {
    pub _base: Chuck_Instr_Unary_Op,
    pub base: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Mem_Addr"]
    pub fn Chuck_Instr_Reg_Push_Mem_Addr_Chuck_Instr_Reg_Push_Mem_Addr(
        this: *mut Chuck_Instr_Reg_Push_Mem_Addr,
        src: c_ulong,
        use_base: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Mem_Addr {
    #[inline]
    pub unsafe fn new(src: c_ulong, use_base: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Mem_Addr_Chuck_Instr_Reg_Push_Mem_Addr(
            &mut __bindgen_tmp,
            src,
            use_base,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Mem_Addr_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Reg_Push_Mem_Addr_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
pub struct Chuck_Instr_Reg_Push_Global_Addr {
    pub _base: Chuck_Instr_Unary_Op,
    pub m_name: crate::rtmidi_h_edited::string(&mut c_char),
    pub m_type: te_GlobalType,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Global_Addr"]
    pub fn Chuck_Instr_Reg_Push_Global_Addr_Chuck_Instr_Reg_Push_Global_Addr(
        this: *mut Chuck_Instr_Reg_Push_Global_Addr,
        name: crate::rtmidi_h_edited::string(&mut c_char),
        type_: te_GlobalType,
    );
}
impl Chuck_Instr_Reg_Push_Global_Addr {
    #[inline]
    pub unsafe fn new(name: crate::rtmidi_h_edited::string(&mut c_char), type_: te_GlobalType) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Global_Addr_Chuck_Instr_Reg_Push_Global_Addr(
            &mut __bindgen_tmp,
            name,
            type_,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Global_Addr_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Reg_Push_Global_Addr_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Deref {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Deref"]
    pub fn Chuck_Instr_Reg_Push_Deref_Chuck_Instr_Reg_Push_Deref(
        this: *mut Chuck_Instr_Reg_Push_Deref,
        src: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Deref {
    #[inline]
    pub unsafe fn new(src: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Deref_Chuck_Instr_Reg_Push_Deref(&mut __bindgen_tmp, src);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Deref_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Deref2 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Reg_Push_Deref2"]
    pub fn Chuck_Instr_Reg_Push_Deref2_Chuck_Instr_Reg_Push_Deref2(
        this: *mut Chuck_Instr_Reg_Push_Deref2,
        src: c_ulong,
    );
}
impl Chuck_Instr_Reg_Push_Deref2 {
    #[inline]
    pub unsafe fn new(src: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Reg_Push_Deref2_Chuck_Instr_Reg_Push_Deref2(&mut __bindgen_tmp, src);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Deref2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mem_Set_Imm {
    pub _base: Chuck_Instr,
    pub m_offset: c_ulong,
    pub m_val: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Mem_Set_Imm"]
    pub fn Chuck_Instr_Mem_Set_Imm_Chuck_Instr_Mem_Set_Imm(
        this: *mut Chuck_Instr_Mem_Set_Imm,
        offset: c_ulong,
        val: c_ulong,
    );
}
impl Chuck_Instr_Mem_Set_Imm {
    #[inline]
    pub unsafe fn new(offset: c_ulong, val: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Mem_Set_Imm_Chuck_Instr_Mem_Set_Imm(&mut __bindgen_tmp, offset, val);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mem_Set_Imm_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Mem_Set_Imm_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mem_Set_Imm2 {
    pub _base: Chuck_Instr_Unary_Op,
    pub m_offset: c_ulong,
    pub m_val: f64,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Mem_Set_Imm2"]
    pub fn Chuck_Instr_Mem_Set_Imm2_Chuck_Instr_Mem_Set_Imm2(
        this: *mut Chuck_Instr_Mem_Set_Imm2,
        offset: c_ulong,
        val: f64,
    );
}
impl Chuck_Instr_Mem_Set_Imm2 {
    #[inline]
    pub unsafe fn new(offset: c_ulong, val: f64) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Mem_Set_Imm2_Chuck_Instr_Mem_Set_Imm2(&mut __bindgen_tmp, offset, val);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mem_Set_Imm2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Mem_Set_Imm2_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mem_Push_Imm {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Mem_Push_Imm"]
    pub fn Chuck_Instr_Mem_Push_Imm_Chuck_Instr_Mem_Push_Imm(
        this: *mut Chuck_Instr_Mem_Push_Imm,
        src: c_ulong,
    );
}
impl Chuck_Instr_Mem_Push_Imm {
    #[inline]
    pub unsafe fn new(src: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Mem_Push_Imm_Chuck_Instr_Mem_Push_Imm(&mut __bindgen_tmp, src);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mem_Push_Imm_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mem_Push_Imm2 {
    pub _base: Chuck_Instr_Unary_Op2,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Mem_Push_Imm2"]
    pub fn Chuck_Instr_Mem_Push_Imm2_Chuck_Instr_Mem_Push_Imm2(
        this: *mut Chuck_Instr_Mem_Push_Imm2,
        src: f64,
    );
}
impl Chuck_Instr_Mem_Push_Imm2 {
    #[inline]
    pub unsafe fn new(src: f64) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Mem_Push_Imm2_Chuck_Instr_Mem_Push_Imm2(&mut __bindgen_tmp, src);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mem_Push_Imm2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mem_Pop_Word {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mem_Pop_Word_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mem_Pop_Word2 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mem_Pop_Word2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Mem_Pop_Word3 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Mem_Pop_Word3"]
    pub fn Chuck_Instr_Mem_Pop_Word3_Chuck_Instr_Mem_Pop_Word3(
        this: *mut Chuck_Instr_Mem_Pop_Word3,
        num: c_ulong,
    );
}
impl Chuck_Instr_Mem_Pop_Word3 {
    #[inline]
    pub unsafe fn new(num: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Mem_Pop_Word3_Chuck_Instr_Mem_Pop_Word3(&mut __bindgen_tmp, num);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Mem_Pop_Word3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Nop {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Nop_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_EOC {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_EOC_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Word {
    pub _base: Chuck_Instr_Unary_Op,
    pub m_is_object: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Word"]
    pub fn Chuck_Instr_Alloc_Word_Chuck_Instr_Alloc_Word(
        this: *mut Chuck_Instr_Alloc_Word,
        offset: c_ulong,
        is_object: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Word {
    #[inline]
    pub unsafe fn new(offset: c_ulong, is_object: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Word_Chuck_Instr_Alloc_Word(&mut __bindgen_tmp, offset, is_object);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Word_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Word2 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Word2"]
    pub fn Chuck_Instr_Alloc_Word2_Chuck_Instr_Alloc_Word2(
        this: *mut Chuck_Instr_Alloc_Word2,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Word2 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Word2_Chuck_Instr_Alloc_Word2(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Word2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Word4 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Word4"]
    pub fn Chuck_Instr_Alloc_Word4_Chuck_Instr_Alloc_Word4(
        this: *mut Chuck_Instr_Alloc_Word4,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Word4 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Word4_Chuck_Instr_Alloc_Word4(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Word4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Vec3 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Vec3"]
    pub fn Chuck_Instr_Alloc_Vec3_Chuck_Instr_Alloc_Vec3(
        this: *mut Chuck_Instr_Alloc_Vec3,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Vec3 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Vec3_Chuck_Instr_Alloc_Vec3(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Vec4 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Vec4"]
    pub fn Chuck_Instr_Alloc_Vec4_Chuck_Instr_Alloc_Vec4(
        this: *mut Chuck_Instr_Alloc_Vec4,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Vec4 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Vec4_Chuck_Instr_Alloc_Vec4(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Member_Word {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Member_Word"]
    pub fn Chuck_Instr_Alloc_Member_Word_Chuck_Instr_Alloc_Member_Word(
        this: *mut Chuck_Instr_Alloc_Member_Word,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Member_Word {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Member_Word_Chuck_Instr_Alloc_Member_Word(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Member_Word_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Member_Word2 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Member_Word2"]
    pub fn Chuck_Instr_Alloc_Member_Word2_Chuck_Instr_Alloc_Member_Word2(
        this: *mut Chuck_Instr_Alloc_Member_Word2,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Member_Word2 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Member_Word2_Chuck_Instr_Alloc_Member_Word2(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Member_Word2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Member_Word4 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Member_Word4"]
    pub fn Chuck_Instr_Alloc_Member_Word4_Chuck_Instr_Alloc_Member_Word4(
        this: *mut Chuck_Instr_Alloc_Member_Word4,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Member_Word4 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Member_Word4_Chuck_Instr_Alloc_Member_Word4(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Member_Word4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Member_Vec3 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Member_Vec3"]
    pub fn Chuck_Instr_Alloc_Member_Vec3_Chuck_Instr_Alloc_Member_Vec3(
        this: *mut Chuck_Instr_Alloc_Member_Vec3,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Member_Vec3 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Member_Vec3_Chuck_Instr_Alloc_Member_Vec3(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Member_Vec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Alloc_Member_Vec4 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Member_Vec4"]
    pub fn Chuck_Instr_Alloc_Member_Vec4_Chuck_Instr_Alloc_Member_Vec4(
        this: *mut Chuck_Instr_Alloc_Member_Vec4,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Alloc_Member_Vec4 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Member_Vec4_Chuck_Instr_Alloc_Member_Vec4(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Member_Vec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
pub struct Chuck_Instr_Alloc_Word_Global {
    pub _base: Chuck_Instr_Unary_Op,
    pub m_name: crate::rtmidi_h_edited::string(&mut c_char),
    pub m_type: te_GlobalType,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Alloc_Word_Global"]
    pub fn Chuck_Instr_Alloc_Word_Global_Chuck_Instr_Alloc_Word_Global(
        this: *mut Chuck_Instr_Alloc_Word_Global,
    );
}
impl Chuck_Instr_Alloc_Word_Global {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Alloc_Word_Global_Chuck_Instr_Alloc_Word_Global(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Alloc_Word_Global_params(this: *mut c_void) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Alloc_Word_Global_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Instantiate_Object {
    pub _base: Chuck_Instr,
    pub type_: *mut Chuck_Type,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Instantiate_Object"]
    pub fn Chuck_Instr_Instantiate_Object_Chuck_Instr_Instantiate_Object(
        this: *mut Chuck_Instr_Instantiate_Object,
        t: *mut Chuck_Type,
    );
}
impl Chuck_Instr_Instantiate_Object {
    #[inline]
    pub unsafe fn new(t: *mut Chuck_Type) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Instantiate_Object_Chuck_Instr_Instantiate_Object(&mut __bindgen_tmp, t);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Instantiate_Object_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Instantiate_Object_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Pre_Constructor {
    pub _base: Chuck_Instr,
    pub pre_ctor: *mut Chuck_VM_Code,
    pub stack_offset: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Pre_Constructor"]
    pub fn Chuck_Instr_Pre_Constructor_Chuck_Instr_Pre_Constructor(
        this: *mut Chuck_Instr_Pre_Constructor,
        pre: *mut Chuck_VM_Code,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Pre_Constructor {
    #[inline]
    pub unsafe fn new(pre: *mut Chuck_VM_Code, offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Pre_Constructor_Chuck_Instr_Pre_Constructor(&mut __bindgen_tmp, pre, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Pre_Constructor_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Pre_Ctor_Array_Top {
    pub _base: Chuck_Instr_Unary_Op,
    pub type_: *mut Chuck_Type,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Pre_Ctor_Array_Top"]
    pub fn Chuck_Instr_Pre_Ctor_Array_Top_Chuck_Instr_Pre_Ctor_Array_Top(
        this: *mut Chuck_Instr_Pre_Ctor_Array_Top,
        t: *mut Chuck_Type,
    );
}
impl Chuck_Instr_Pre_Ctor_Array_Top {
    #[inline]
    pub unsafe fn new(t: *mut Chuck_Type) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Pre_Ctor_Array_Top_Chuck_Instr_Pre_Ctor_Array_Top(&mut __bindgen_tmp, t);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Pre_Ctor_Array_Top_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Pre_Ctor_Array_Top_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Pre_Ctor_Array_Bottom {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Pre_Ctor_Array_Bottom_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Pre_Ctor_Array_Post {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Pre_Ctor_Array_Post_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Array_Prepend {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Prepend"]
    pub fn Chuck_Instr_Array_Prepend_Chuck_Instr_Array_Prepend(
        this: *mut Chuck_Instr_Array_Prepend,
        size: c_ulong,
    );
}
impl Chuck_Instr_Array_Prepend {
    #[inline]
    pub unsafe fn new(size: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Array_Prepend_Chuck_Instr_Array_Prepend(&mut __bindgen_tmp, size);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Array_Prepend_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Array_Append {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Append"]
    pub fn Chuck_Instr_Array_Append_Chuck_Instr_Array_Append(
        this: *mut Chuck_Instr_Array_Append,
        size: c_ulong,
    );
}
impl Chuck_Instr_Array_Append {
    #[inline]
    pub unsafe fn new(size: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Array_Append_Chuck_Instr_Array_Append(&mut __bindgen_tmp, size);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Array_Append_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Assign_String {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Assign_String_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Assign_Primitive {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Assign_Primitive_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Assign_Primitive2 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Assign_Primitive2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Assign_Primitive4 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Assign_Primitive4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Assign_PrimitiveVec3 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Assign_PrimitiveVec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Assign_PrimitiveVec4 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Assign_PrimitiveVec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Assign_Object {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Assign_Object_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Assign_Object_To_Map {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Assign_Object_To_Map"]
    pub fn Chuck_Instr_Assign_Object_To_Map_Chuck_Instr_Assign_Object_To_Map(
        this: *mut Chuck_Instr_Assign_Object_To_Map,
        size: c_ulong,
    );
}
impl Chuck_Instr_Assign_Object_To_Map {
    #[inline]
    pub unsafe fn new(size: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Assign_Object_To_Map_Chuck_Instr_Assign_Object_To_Map(&mut __bindgen_tmp, size);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Assign_Object_To_Map_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_AddRef_Object {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_AddRef_Object_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_AddRef_Object2 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_AddRef_Object2"]
    pub fn Chuck_Instr_AddRef_Object2_Chuck_Instr_AddRef_Object2(
        this: *mut Chuck_Instr_AddRef_Object2,
        offset: c_ulong,
    );
}
impl Chuck_Instr_AddRef_Object2 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_AddRef_Object2_Chuck_Instr_AddRef_Object2(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_AddRef_Object2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_AddRef_Object3 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_AddRef_Object3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Release_Object {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Release_Object_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Release_Object2 {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Release_Object2"]
    pub fn Chuck_Instr_Release_Object2_Chuck_Instr_Release_Object2(
        this: *mut Chuck_Instr_Release_Object2,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Release_Object2 {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Release_Object2_Chuck_Instr_Release_Object2(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Release_Object2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Func_To_Code {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Func_To_Code_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Func_Call {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Func_Call_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Func_Call_Member {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Func_Call_Member"]
    pub fn Chuck_Instr_Func_Call_Member_Chuck_Instr_Func_Call_Member(
        this: *mut Chuck_Instr_Func_Call_Member,
        ret_size: c_ulong,
    );
}
impl Chuck_Instr_Func_Call_Member {
    #[inline]
    pub unsafe fn new(ret_size: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Func_Call_Member_Chuck_Instr_Func_Call_Member(&mut __bindgen_tmp, ret_size);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Func_Call_Member_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Func_Call_Static {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Func_Call_Static"]
    pub fn Chuck_Instr_Func_Call_Static_Chuck_Instr_Func_Call_Static(
        this: *mut Chuck_Instr_Func_Call_Static,
        ret_size: c_ulong,
    );
}
impl Chuck_Instr_Func_Call_Static {
    #[inline]
    pub unsafe fn new(ret_size: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Func_Call_Static_Chuck_Instr_Func_Call_Static(&mut __bindgen_tmp, ret_size);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Func_Call_Static_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Func_Return {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Func_Return_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Spork {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Spork"]
    pub fn Chuck_Instr_Spork_Chuck_Instr_Spork(this: *mut Chuck_Instr_Spork, v: c_ulong);
}
impl Chuck_Instr_Spork {
    #[inline]
    pub unsafe fn new(v: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Spork_Chuck_Instr_Spork(&mut __bindgen_tmp, v);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Spork_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Time_Advance {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Time_Advance_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Event_Wait {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Event_Wait_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Array_Init {
    pub _base: Chuck_Instr,
    pub m_type_ref: *mut Chuck_Type,
    pub m_length: c_long,
    pub m_is_obj: c_ulong,
    pub m_param_str: *mut c_char,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Init"]
    pub fn Chuck_Instr_Array_Init_Chuck_Instr_Array_Init(
        this: *mut Chuck_Instr_Array_Init,
        env: *mut Chuck_Env,
        the_type: *mut Chuck_Type,
        length: c_long,
    );
}
impl Chuck_Instr_Array_Init {
    #[inline]
    pub unsafe fn new(
        env: *mut Chuck_Env,
        the_type: *mut Chuck_Type,
        length: c_long,
    ) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Array_Init_Chuck_Instr_Array_Init(&mut __bindgen_tmp, env, the_type, length);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Init_destructor"]
    pub fn Chuck_Instr_Array_Init_Chuck_Instr_Array_Init_destructor(
        this: *mut Chuck_Instr_Array_Init,
    );
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Array_Init_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Array_Init_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Array_Alloc {
    pub _base: Chuck_Instr,
    pub m_depth: c_ulong,
    pub m_type_ref: *mut Chuck_Type,
    pub m_is_obj: c_ulong,
    pub m_param_str: *mut c_char,
    pub m_stack_offset: c_ulong,
    pub m_is_ref: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Alloc"]
    pub fn Chuck_Instr_Array_Alloc_Chuck_Instr_Array_Alloc(
        this: *mut Chuck_Instr_Array_Alloc,
        env: *mut Chuck_Env,
        depth: c_ulong,
        the_type: *mut Chuck_Type,
        offset: c_ulong,
        ref_: c_ulong,
    );
}
impl Chuck_Instr_Array_Alloc {
    #[inline]
    pub unsafe fn new(
        env: *mut Chuck_Env,
        depth: c_ulong,
        the_type: *mut Chuck_Type,
        offset: c_ulong,
        ref_: c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Array_Alloc_Chuck_Instr_Array_Alloc(
            &mut __bindgen_tmp,
            env,
            depth,
            the_type,
            offset,
            ref_,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Alloc_destructor"]
    pub fn Chuck_Instr_Array_Alloc_Chuck_Instr_Array_Alloc_destructor(
        this: *mut Chuck_Instr_Array_Alloc,
    );
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Array_Alloc_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Array_Alloc_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Array_Access {
    pub _base: Chuck_Instr,
    pub m_kind: c_ulong,
    pub m_emit_addr: c_ulong,
    pub m_istr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Access"]
    pub fn Chuck_Instr_Array_Access_Chuck_Instr_Array_Access(
        this: *mut Chuck_Instr_Array_Access,
        kind: c_ulong,
        emit_addr: c_ulong,
        istr: c_ulong,
    );
}
impl Chuck_Instr_Array_Access {
    #[inline]
    pub unsafe fn new(kind: c_ulong, emit_addr: c_ulong, istr: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Array_Access_Chuck_Instr_Array_Access(
            &mut __bindgen_tmp,
            kind,
            emit_addr,
            istr,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Array_Access_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Array_Access_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Array_Map_Access {
    pub _base: Chuck_Instr,
    pub m_kind: c_ulong,
    pub m_emit_addr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Map_Access"]
    pub fn Chuck_Instr_Array_Map_Access_Chuck_Instr_Array_Map_Access(
        this: *mut Chuck_Instr_Array_Map_Access,
        kind: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Array_Map_Access {
    #[inline]
    pub unsafe fn new(kind: c_ulong, emit_addr: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Array_Map_Access_Chuck_Instr_Array_Map_Access(
            &mut __bindgen_tmp,
            kind,
            emit_addr,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Array_Map_Access_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Array_Map_Access_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
pub struct Chuck_Instr_Array_Access_Multi {
    pub _base: Chuck_Instr,
    pub m_depth: c_ulong,
    pub m_kind: c_ulong,
    pub m_emit_addr: c_ulong,
    pub m_indexIsAssociative: crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}indexIsAssociative"]
    pub fn Chuck_Instr_Array_Access_Multi_indexIsAssociative(
        this: *mut Chuck_Instr_Array_Access_Multi,
    ) -> *mut crate::dts::vector;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Array_Access_Multi"]
    pub fn Chuck_Instr_Array_Access_Multi_Chuck_Instr_Array_Access_Multi(
        this: *mut Chuck_Instr_Array_Access_Multi,
        depth: c_ulong,
        kind: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Array_Access_Multi {
    #[inline]
    pub unsafe fn indexIsAssociative(&mut self) -> *mut crate::dts::vector {
        Chuck_Instr_Array_Access_Multi_indexIsAssociative(self)
    }
    #[inline]
    pub unsafe fn new(depth: c_ulong, kind: c_ulong, emit_addr: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Array_Access_Multi_Chuck_Instr_Array_Access_Multi(
            &mut __bindgen_tmp,
            depth,
            kind,
            emit_addr,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Array_Access_Multi_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Array_Access_Multi_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Member_Data {
    pub _base: Chuck_Instr,
    pub m_offset: c_ulong,
    pub m_kind: c_ulong,
    pub m_emit_addr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Member_Data"]
    pub fn Chuck_Instr_Dot_Member_Data_Chuck_Instr_Dot_Member_Data(
        this: *mut Chuck_Instr_Dot_Member_Data,
        offset: c_ulong,
        kind: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Dot_Member_Data {
    #[inline]
    pub unsafe fn new(
        offset: c_ulong,
        kind: c_ulong,
        emit_addr: c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Member_Data_Chuck_Instr_Dot_Member_Data(
            &mut __bindgen_tmp,
            offset,
            kind,
            emit_addr,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Member_Data_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Member_Data_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Member_Func {
    pub _base: Chuck_Instr,
    pub m_offset: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Member_Func"]
    pub fn Chuck_Instr_Dot_Member_Func_Chuck_Instr_Dot_Member_Func(
        this: *mut Chuck_Instr_Dot_Member_Func,
        offset: c_ulong,
    );
}
impl Chuck_Instr_Dot_Member_Func {
    #[inline]
    pub unsafe fn new(offset: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Member_Func_Chuck_Instr_Dot_Member_Func(&mut __bindgen_tmp, offset);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Member_Func_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Member_Func_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Primitive_Func {
    pub _base: Chuck_Instr,
    pub m_native_func: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Primitive_Func"]
    pub fn Chuck_Instr_Dot_Primitive_Func_Chuck_Instr_Dot_Primitive_Func(
        this: *mut Chuck_Instr_Dot_Primitive_Func,
        native_func: c_ulong,
    );
}
impl Chuck_Instr_Dot_Primitive_Func {
    #[inline]
    pub unsafe fn new(native_func: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Primitive_Func_Chuck_Instr_Dot_Primitive_Func(
            &mut __bindgen_tmp,
            native_func,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Primitive_Func_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Primitive_Func_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Static_Data {
    pub _base: Chuck_Instr,
    pub m_offset: c_ulong,
    pub m_size: c_ulong,
    pub m_kind: c_ulong,
    pub m_emit_addr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Static_Data"]
    pub fn Chuck_Instr_Dot_Static_Data_Chuck_Instr_Dot_Static_Data(
        this: *mut Chuck_Instr_Dot_Static_Data,
        offset: c_ulong,
        size: c_ulong,
        kind: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Dot_Static_Data {
    #[inline]
    pub unsafe fn new(
        offset: c_ulong,
        size: c_ulong,
        kind: c_ulong,
        emit_addr: c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Static_Data_Chuck_Instr_Dot_Static_Data(
            &mut __bindgen_tmp,
            offset,
            size,
            kind,
            emit_addr,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Static_Data_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Static_Data_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Static_Import_Data {
    pub _base: Chuck_Instr,
    pub m_addr: *mut c_void,
    pub m_kind: c_ulong,
    pub m_emit_addr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Static_Import_Data"]
    pub fn Chuck_Instr_Dot_Static_Import_Data_Chuck_Instr_Dot_Static_Import_Data(
        this: *mut Chuck_Instr_Dot_Static_Import_Data,
        addr: *mut c_void,
        kind: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Dot_Static_Import_Data {
    #[inline]
    pub unsafe fn new(
        addr: *mut c_void,
        kind: c_ulong,
        emit_addr: c_ulong,
    ) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Static_Import_Data_Chuck_Instr_Dot_Static_Import_Data(
            &mut __bindgen_tmp,
            addr,
            kind,
            emit_addr,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Static_Import_Data_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Static_Import_Data_params(
        this: *mut c_void,
    ) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Static_Func {
    pub _base: Chuck_Instr,
    pub m_func: *mut Chuck_Func,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Static_Func"]
    pub fn Chuck_Instr_Dot_Static_Func_Chuck_Instr_Dot_Static_Func(
        this: *mut Chuck_Instr_Dot_Static_Func,
        func: *mut Chuck_Func,
    );
}
impl Chuck_Instr_Dot_Static_Func {
    #[inline]
    pub unsafe fn new(func: *mut Chuck_Func) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Static_Func_Chuck_Instr_Dot_Static_Func(&mut __bindgen_tmp, func);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Static_Func_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Static_Func_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Cmp_First {
    pub _base: Chuck_Instr,
    pub m_is_mem: c_ulong,
    pub m_emit_addr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Cmp_First"]
    pub fn Chuck_Instr_Dot_Cmp_First_Chuck_Instr_Dot_Cmp_First(
        this: *mut Chuck_Instr_Dot_Cmp_First,
        is_mem: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Dot_Cmp_First {
    #[inline]
    pub unsafe fn new(is_mem: c_ulong, emit_addr: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Cmp_First_Chuck_Instr_Dot_Cmp_First(&mut __bindgen_tmp, is_mem, emit_addr);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Cmp_First_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Cmp_First_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Cmp_Second {
    pub _base: Chuck_Instr,
    pub m_is_mem: c_ulong,
    pub m_emit_addr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Cmp_Second"]
    pub fn Chuck_Instr_Dot_Cmp_Second_Chuck_Instr_Dot_Cmp_Second(
        this: *mut Chuck_Instr_Dot_Cmp_Second,
        is_mem: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Dot_Cmp_Second {
    #[inline]
    pub unsafe fn new(is_mem: c_ulong, emit_addr: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Cmp_Second_Chuck_Instr_Dot_Cmp_Second(
            &mut __bindgen_tmp,
            is_mem,
            emit_addr,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Cmp_Second_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Cmp_Second_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Cmp_Third {
    pub _base: Chuck_Instr,
    pub m_is_mem: c_ulong,
    pub m_emit_addr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Cmp_Third"]
    pub fn Chuck_Instr_Dot_Cmp_Third_Chuck_Instr_Dot_Cmp_Third(
        this: *mut Chuck_Instr_Dot_Cmp_Third,
        is_mem: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Dot_Cmp_Third {
    #[inline]
    pub unsafe fn new(is_mem: c_ulong, emit_addr: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Cmp_Third_Chuck_Instr_Dot_Cmp_Third(&mut __bindgen_tmp, is_mem, emit_addr);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Cmp_Third_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Cmp_Third_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dot_Cmp_Fourth {
    pub _base: Chuck_Instr,
    pub m_is_mem: c_ulong,
    pub m_emit_addr: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Dot_Cmp_Fourth"]
    pub fn Chuck_Instr_Dot_Cmp_Fourth_Chuck_Instr_Dot_Cmp_Fourth(
        this: *mut Chuck_Instr_Dot_Cmp_Fourth,
        is_mem: c_ulong,
        emit_addr: c_ulong,
    );
}
impl Chuck_Instr_Dot_Cmp_Fourth {
    #[inline]
    pub unsafe fn new(is_mem: c_ulong, emit_addr: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Dot_Cmp_Fourth_Chuck_Instr_Dot_Cmp_Fourth(
            &mut __bindgen_tmp,
            is_mem,
            emit_addr,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dot_Cmp_Fourth_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Dot_Cmp_Fourth_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_ADC {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_ADC_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_DAC {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_DAC_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Bunghole {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Bunghole_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Chout {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Chout_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cherr {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cherr_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_UGen_Link {
    pub _base: Chuck_Instr,
    pub m_isUpChuck: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_UGen_Link"]
    pub fn Chuck_Instr_UGen_Link_Chuck_Instr_UGen_Link(
        this: *mut Chuck_Instr_UGen_Link,
        isUpChuck: c_ulong,
    );
}
impl Chuck_Instr_UGen_Link {
    #[inline]
    pub unsafe fn new(isUpChuck: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_UGen_Link_Chuck_Instr_UGen_Link(&mut __bindgen_tmp, isUpChuck);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_UGen_Link_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_UGen_Array_Link {
    pub _base: Chuck_Instr,
    pub m_srcIsArray: c_ulong,
    pub m_dstIsArray: c_ulong,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_UGen_Array_Link"]
    pub fn Chuck_Instr_UGen_Array_Link_Chuck_Instr_UGen_Array_Link(
        this: *mut Chuck_Instr_UGen_Array_Link,
        srcIsArray: c_ulong,
        dstIsArray: c_ulong,
    );
}
impl Chuck_Instr_UGen_Array_Link {
    #[inline]
    pub unsafe fn new(srcIsArray: c_ulong, dstIsArray: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_UGen_Array_Link_Chuck_Instr_UGen_Array_Link(
            &mut __bindgen_tmp,
            srcIsArray,
            dstIsArray,
        );
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_UGen_Array_Link_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_UGen_UnLink {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_UGen_UnLink_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_UGen_Ctrl {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_UGen_Ctrl_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_UGen_CGet {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_UGen_CGet_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_UGen_Ctrl2 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_UGen_Ctrl2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_UGen_CGet2 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_UGen_CGet2_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_UGen_PMsg {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_UGen_PMsg_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_double2int {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_double2int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_int2double {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_int2double_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_int2complex {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_int2complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_int2polar {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_int2polar_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_double2complex {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_double2complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_double2polar {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_double2polar_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_complex2polar {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_complex2polar_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_polar2complex {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_polar2complex_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_vec3tovec4 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_vec3tovec4_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_vec4tovec3 {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_vec4tovec3_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Cast_object2string {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Cast_object2string_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Op_string {
    pub _base: Chuck_Instr_Unary_Op,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Op_string"]
    pub fn Chuck_Instr_Op_string_Chuck_Instr_Op_string(
        this: *mut Chuck_Instr_Op_string,
        v: c_ulong,
    );
}
impl Chuck_Instr_Op_string {
    #[inline]
    pub unsafe fn new(v: c_ulong) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Op_string_Chuck_Instr_Op_string(&mut __bindgen_tmp, v);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Op_string_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Init_Loop_Counter {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Init_Loop_Counter_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Reg_Push_Loop_Counter_Deref {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Reg_Push_Loop_Counter_Deref_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Dec_Loop_Counter {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Dec_Loop_Counter_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Pop_Loop_Counter {
    pub _base: Chuck_Instr,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Pop_Loop_Counter_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_IO_in_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_IO_in_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_IO_in_float {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_IO_in_float_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_IO_in_string {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_IO_in_string_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_IO_out_int {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_IO_out_int_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_IO_out_float {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_IO_out_float_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_IO_out_string {
    pub _base: Chuck_Instr_Binary_Op,
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_IO_out_string_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Instr_Hack {
    pub _base: Chuck_Instr,
    pub m_type_ref: *mut Chuck_Type,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Hack"]
    pub fn Chuck_Instr_Hack_Chuck_Instr_Hack(this: *mut Chuck_Instr_Hack, type_: *mut Chuck_Type);
}
impl Chuck_Instr_Hack {
    #[inline]
    pub unsafe fn new(type_: *mut Chuck_Type) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Hack_Chuck_Instr_Hack(&mut __bindgen_tmp, type_);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Hack_destructor"]
    pub fn Chuck_Instr_Hack_Chuck_Instr_Hack_destructor(this: *mut Chuck_Instr_Hack);
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Hack_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Hack_params(this: *mut c_void) -> *const c_char;
}
#[repr(C)]
pub struct Chuck_Instr_Gack {
    pub _base: Chuck_Instr,
    pub m_type_refs: crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Gack"]
    pub fn Chuck_Instr_Gack_Chuck_Instr_Gack(
        this: *mut Chuck_Instr_Gack,
        types: *const crate::dts::vector,
    );
}
impl Chuck_Instr_Gack {
    #[inline]
    pub unsafe fn new(types: *const crate::dts::vector) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Instr_Gack_Chuck_Instr_Gack(&mut __bindgen_tmp, types);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Instr_Gack_destructor"]
    pub fn Chuck_Instr_Gack_Chuck_Instr_Gack_destructor(this: *mut Chuck_Instr_Gack);
}
extern "C" {
    #[link_name = "\u{1}execute"]
    pub fn Chuck_Instr_Gack_execute(
        this: *mut c_void,
        vm: *mut Chuck_VM,
        shred: *mut Chuck_VM_Shred,
    );
}
extern "C" {
    #[link_name = "\u{1}params"]
    pub fn Chuck_Instr_Gack_params(this: *mut c_void) -> *const c_char;
}
extern "C" {
    pub fn instantiate_and_initialize_object(
        type_: *mut Chuck_Type,
        shred: *mut Chuck_VM_Shred,
    ) -> *mut Chuck_Object;
}
extern "C" {
    pub fn initialize_object(obj: *mut Chuck_Object, type_: *mut Chuck_Type) -> c_ulong;
}
extern "C" {
    pub fn throw_exception(shred: *mut Chuck_VM_Shred, name: *const c_char);
}
