use libc::{
    c_char,
};
use crate::carrier::Chuck_Carrier;
use crate::def::*;
use crate::vm::{Chuck_VM_Code};
use std::collections::BTreeMap;
// submodules
/// abstract syntax
pub mod absyn;
/// dynamic linking
pub mod dl;
/// bytecode emitter
pub mod emit;
/// error messaging
pub mod errmsg;
/// scope frame
pub mod frame;
/// instruction set
// pub mod instr;  // lot of work to be done here
/// language features
pub mod lang;
/// lexer
pub mod lex; // pub during development, will eventually be private
/// object abstraction
pub mod oo;
/// on-the-fly/livecoding
pub mod otf;
/// parsing
pub mod parse;
/// lexing/scanning from flex
pub mod scan;
/// symbol syntax
pub mod symbol;
/// lookup table
pub mod table;
/// type checking
pub mod typeck;
/// parsing grammar from bison
pub mod tab; // pub during development, will eventually be private

/*
// Original Includes
#include "chuck_compile.h"
#include "chuck_lang.h"
#include "chuck_errmsg.h"
#include "chuck_otf.h"
#include "ugen_osc.h"
#include "ugen_xxx.h"
#include "ugen_filter.h"
#include "ugen_stk.h"
#include "uana_xform.h"
#include "uana_extract.h"
#include "ulib_machine.h"
#include "ulib_math.h"
#include "ulib_std.h"
#include "ulib_opsc.h"
#include "ulib_regex.h"
#include "chuck_io.h"
*/

pub struct Chuck_DLL {

}

/// Compiler class abstraction
pub struct Chuck_Compiler {
    code: *mut Chuck_VM_Code,
    emitter: *mut Chuck_Emitter,
    m_auto_depend: t_CKBOOL,
    m_carrier: *mut Chuck_Carrier,
    m_cklibs_to_preload: Vec::<c_char>,
    m_dlls: Vec::<*mut Chuck_DLL>,
    m_recent: BTreeMap<c_char, *mut Chuck_Context>,
}
/// Compiler implementation
impl Chuck_Compiler {
    pub fn Chuck_Compiler() {

    }
    pub fn dropChuck_Compiler() {

    }
    pub fn initialize(chugin_search_paths: &Vec::<c_char>, named_dls: &Vec::<c_char>) {

    }
    pub fn shutdown() {

    }
    pub fn bind(query_func: f_ck_query, name: &c_char, nspc: c_char) -> bool {

    }
    pub fn set_auto_depend(v: t_CKBOOL) {

    }
    pub fn go(filename: &c_char, fd: *mut FILE, str_src: *mut c_char, full_path: &c_char) -> t_CKBOOL {

    }
    // function prototypes
    pub fn load_internal_modules(compiler: *mut Chuck_Compiler) -> t_CKBOOL {

    }
    pub fn load_external_modules(compiler: *mut Chuck_Compiler, extension: *mut c_char, chugin_search_paths: &Vec::<c_char>, named_dls: Vec::<c_char>) -> t_CKBOOL {

    }
    pub fn load_module(compiler: *mut Chuck_Compiler, env: *mut Chuck_Env, query: f_ck_query, name: *const c_char, nspc: *const c_char) {

    }
    /// C++ class constructor
    pub fn new() -> Chuck_Compiler {
        Chuck_Compiler.emitter = NULL;
        Chuck_Compiler.code = NULL;
        Chuck_Compiler.m_carrier = NULL;
        return Chuck_Compiler;
    }
    /// C++ class destructor
    pub fn drop(this: &Self) -> t_CKBOOL {
        this::shutdown();
        return TRUE;
    }
    pub fn initialize(this: Chuck_Compiler, chugin_search_paths: &Vec::<c_char>, named_dls: &Vec::<c_char>) -> t_CKBOOL {
        EM_log(CK_LOG_SYSTEM, "initializing compiler...");
        EM_pushlog();
        let env: *mut Chuck_Env = type_engine_init(this.m_carrier);
        // allocate the type checker
        env::add_ref();
        // allocate the emitter
        this.emitter = emit_engine_init(env);
        // add reference
        this.emitter.add_ref();
        // set auto depend to 0
        this.m_auto_depend = FALSE;
        // load internal libs
        if !load_internal_modules(this) {
            this::error();
        }
        // load external libs
        if !load_external_modules(this, ".chug", chugin_search_paths, named_dls) {
            this::error();
        }
        EM_poplog();
        return TRUE;
    }
    pub fn error(this: Chuck_Compiler) {
        this::shutdown();
        EM_poplog();
        return FALSE;
    }
    pub fn shutdown(this: Chuck_Compiler) -> t_CKBOOL {
        EM_log(CK_LOG_SYSTEM, "shutting down compiler...");
        EM_pushlog();
        // TODO: free
        type_engine_shutdown(env());
        // TODO: check if emitter gets cleaned up
        // emit_engine_shutdown(this.emitter);
        this.emitter = NULL;
        this.code = NULL;
        this.m_auto_depend = FALSE;
        this.m_recent.clear();
        // TODO: figure out how to translate this
        // for(std::list<Chuck_DLL *>::iterator i = m_dlls.begin();
        //     i != m_dlls.end(); i++)
        // {
        //     delete (*i);
        // }
        this.m_dlls.clear();
        EM_poplog();
        return TRUE;
    }
    /// bind a new type system module, via query function
    pub fn bind(this: Chuck_Compiler, query_func: f_ck_query, name: &c_char, nspc: &c_char) -> t_CKBOOL {
        EM_log(CK_LOG_SYSTEM, "on-demand binding compiler module '{}'...", this.name.to_string());
        EM_pushlog();
        // get env
        let env: *mut Chuck_Env = this::env();
        // make context
        let context: *mut Chuck_Context = type_engine_make_context(NULL, concat!("@[bind]:", this.name.to_string(), "]"));
        // reset env - not needed since we just created the env
        env::reset();
        // load it
        type_engine_load_context(env, context);
        // do it
        if !load_module(this, env, query_func, this.name.to_string(), this.nspc.to_string()) {
            this::error();
        }
        // clear context
        type_engine_unload_context(env);
        // commit what is in the type checker at this point
        env::global().commit();
        EM_poplog();
        return TRUE;
    }
    /// Chuck_Compiler error
    pub fn error(this: Chuck_Compiler) -> t_CKBOOL {
        // probably dangerous: rollback
        this::env::global().rollback();
        // clear context
        type_engine_unload_context(this::env);
        EM_poplog();
        return FALSE;
    }
    /// auto dependency resolve for types
    pub fn set_auto_depend(v: t_CKBOOL) {
        if v {
            EM_log(CK_LOG_SYSTEM, "type dependency resolution: AUTO");
        } else {
            EM_log(CK_LOG_SYSTEM, "type dependency resolution: MANUAL");
        }
        this.m_auto_depend = v;
        return v;
    }
    /// parse, type-check, and emit a program
    pub fn go(this: Chuck_Compiler, filename: &c_char, fd: *mut FILE, str_src: *const c_char, full_path: c_char) -> t_CKBOOL {
        let ret: t_CKBOOL = TRUE;
        let context: Chuck_Context = NULL;
        EM_reset_msg();
        // check to see if resolve dependencies automatically
        if !this.m_auto_depend {
            // normal (note: full_path added in 1.3.0.0)
            ret = this::do_normal(filename, fd, str_src, full_path);
            return ret;
        } else { // auto
            // parse the code
            if !chuck_parse(filename.to_string(), fd, str_src) {
                return FALSE;
            }
            // make the context
            context = type_engine_make_context(g_program, filename);
            if !context {
                return FALSE;
            }
            // reset the env
            this::env().reset();
            // load the context
            if !type_engine_load_context(this::env(), context) {
                return FALSE;
            }
            // do entire file
            if code != context::code() {
                ret = FALSE;
                EM_error2(0, "internal error: context::code() NULL!");
                this::cleanup(this, ret);
            }
        }
    }
    pub fn cleanup(this: Chuck_Compiler, ret: t_CKBOOL) -> t_CKBOOL {
        if ret {
            this::env().global().commit();
        } else {
            this::env().global().rollback();
        }
        if !type_engine_unload_context(this::env()) {
            EM_error2(0, "internal error unloading context...\n");
            return FALSE;
        }
        return ret;
    }
    /// resolve type automatically - if auto_depend is off, return FALSE
    pub fn resolve(this: Chuck_Compiler, rtype: &c_char) -> t_CKBOOL {
        let ret: t_CKBOOL = TRUE;
        // check auto_depend
        if !this.m_auto_depend {
            return FALSE;
        }
        // look up if name is already parsed
        return ret;
    }
    /// parse, type-check, and emit a program
    pub fn do_entire_file(this: Chuck_Compiler, context: *mut Chuck_Context) {
        // pass 0
        if !type_engine_scan0_prog(this::env(), g_program, te_do_all) {
            return FALSE;
        }
        // pass 1
        if !type_engine_scan1_prog(this::env(), g_program, te_do_all) {
            return FALSE;
        }
        // pass 2
        if !type_engine_scan2_prog(this::env(), g_program, te_do_all) {
            return FALSE;
        }
        // check the program (pass 3)
        if !type_engine_check_context(this::env(), context, te_do_all) {
            return FALSE;
        }
        // emit (pass 4)
        if !type_engine_emit_prog(this.emitter, g_program) {
            return FALSE;
        }
        // set the state of the context to done
        context.progress = Chuck_Context::P_ALL;
        return TRUE;
    }
    /// compile only class definitions
    pub fn do_only_classes(this: Chuck_Compiler, context: *mut Chuck_Context) -> t_CKBOOL {
        // 0th-scan (pass 0)
        if !type_engine_scan0_prog(this::env(), g_program, te_do_classes_only) {
            return FALSE;
        }
        // 1st-scan (pass 1)
        if !type_engine_scan1_prog(this::env(), g_program, te_do_classes_only) {
            return FALSE;
        }
        // 2nd-scan (pass 2)
        if !type_engine_scan2_prog(this::env(), g_program, te_do_classes_only) {
            return FALSE;
        }
        // check the program (pass 3)
        if !type_engine_check_context(this::env(), context, te_do_classes_only) {
            return FALSE;
        }
        // emit (pass 4)
        if !(code = emit_engine_emit_prog(this.emitter, g_program , te_do_classes_only )) {
            return FALSE;
        }
        // set the state of the context to done
        context.progress = Chuck_Context::P_ALL;
        return TRUE;
    }
}
