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
//* ChucK byte-code emitter. if a program has made it this far, then it is type-checked.
//**/ the emitter traverses the type-checked syntax tree, and emits virtual ChucK VM
//**/ instructions for each file into a shred (ChucK thread). The instruction set is
//**/ defined in chuck_instr.
//! This file was largely hand-written, as bindgen didn't really work.
//! TODO: Fix this file re: function syntax w/ return
use crate::ck::carry::carrye::*;
use crate::ck::def::defe::*;
use crate::ck::frame::framee::*;
use crate::ck::oo::ooe::*;
use crate::ck::ctype::ctypee::*;
use crate::ck::vm::vme::*;
use crate::ck::util::thread::threade::*;
use crate::dts::*;
pub const NULL: u64 = 0;
pub struct Chuck_Code {
    pub name: String,
    pub stack_depth: t_CKUINT,
    pub need_this: t_CKBOOL,
    pub frame: Chuck_Frame,
    pub code: Vec::from(&'a [Chuck_Instr]),
    pub stack_cont: Vec::from(&'a [Chuck_Instr_Goto]),
    pub stack_break: Vec::from(&'a [Chuck_Instr_Goto]),
    pub stack_return: Vec::from(&'a [Chuck_Instr_Goto]),
    pub filename: crate::dts::string,
}
// impl Chuck_Code {
//     fn new() Chuck_Code<stack_depth: 0, need_this: false, frame: Chuck_Frame> {};
// }
// pub struct Chuck_Emitter: Chuck_VM_Object {
//     pub env: Chuck_Env,
//     pub code: Chuck_Code,
//     pub context: Chuck_Context,
//     pub nspc: Chuck_Namespace,
//     pub func: Chuck_Func,
//     pub stack: crate::dts::vector<Chuck_Code>,
//     pub locals: crate::dts::vector<Chuck_Local>,
//     pub dump: t_CKBOOL,
// }
// impl Chuck_Emitter {
//     pub fn new() Chuck_Emitter<env: NULL, code: NULL, context: NULL, nspc: NULL, func: NULL, dump: false> {};
//     pub fn next_index() {} -> code::size();
//     pub fn append(instr: Chuck_Instr) {
//         assert!(code != NULL);
//     } -> code::push_back(instr);
//     pub fn push_scope() {
//         assert!(code != NULL);
//     } -> code::frame::push_scope();
//     // addref_on_scope(); /// This fn implemented in chuck_emit.cpp
//     // pop_scope(); /// This fn implemented in chuck_emit.cpp
//     /// This is the raw C++ code, not even sure how to translate this
//     // t_CKBOOL find_dur( const std::string & name, t_CKDUR * out );
// }
fn main() {
    crate::chuck_compile_h_edited::emit_engine_init(env: Chuck_Env);
    crate::chuck_compile_h_edited::emit_engine_shutdown(emit: Chuck_Emitter);
    crate::chuck_compile_h_edited::emit_engine_emit_prog(
        emit: Chuck_Emitter,
        prog: a_Program,
        how_much: te_HowMuch = te_do_all,
    );
    // crate::chuck_compile_h_edited::emit_to_code(in: Chuck_Code, out: Chuck_VM_Code = NULL, dump: t_CKBOOL = false);
    // The following aren't used
    // crate::chuck_compile_h_edited::emit_engine_addr_map(emit: Chuck_Emitter, shred: Chuck_VM_Shred);
    // crate::chuck_compile_h_edited::emit_engine_resolve();
}
