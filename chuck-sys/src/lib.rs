//! Raw bindings to libchuck. Most stuff in this file is taken from `src/core/chuck.cpp`.
//!
//! This crate is a binding to the lib-ified version of the ChucK language.
//! https://github.com/ccrma/chuck
#![allow(
    dead_code,
    improper_ctypes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_doc_comments,
    unused_imports,
    unused_mut
)]
extern crate libc;
pub mod def;
pub mod ffi;
use def::*;
use std::collections::BTreeMap;

// chuck param defaults
/// 44100
pub const CHUCK_PARAM_SAMPLE_RATE_DEFAULT: &'static [u8; 6usize] = b"44100\0";
/// 2
pub const CHUCK_PARAM_INPUT_CHANNELS_DEFAULT: &'static [u8; 2usize] = b"2\0";
/// 2
pub const CHUCK_PARAM_OUTPUT_CHANNELS_DEFAULT: &'static [u8; 2usize] = b"2\0";
/// 0
pub const CHUCK_PARAM_VM_ADAPTIVE_DEFAULT: &'static [u8; 2usize] = b"0\0";
/// 0
pub const CHUCK_PARAM_VM_HALT_DEFAULT: &'static [u8; 2usize] = b"0\0";
/// 0
pub const CHUCK_PARAM_OTF_ENABLE_DEFAULT: &'static [u8; 2usize] = b"0\0";
/// 8888
pub const CHUCK_PARAM_OTF_PORT_DEFAULT: &'static [u8; 5usize] = b"8888\0";
/// 0
pub const CHUCK_PARAM_DUMP_INSTRUCTIONS_DEFAULT: &'static [u8; 2usize] = b"0\0";
/// 1
pub const CHUCK_PARAM_AUTO_DEPEND_DEFAULT: &'static [u8; 2usize] = b"1\0";
/// 1
pub const CHUCK_PARAM_DEPRECATE_LEVEL_DEFAULT: &'static [u8; 2usize] = b"1\0";
/// ""
pub const CHUCK_PARAM_WORKING_DIRECTORY_DEFAULT: &'static [u8; 1usize] = b"\0";
/// 1
pub const CHUCK_PARAM_CHUGIN_ENABLE_DEFAULT: &'static [u8; 2usize] = b"1\0";
/// C:\Program Files\ChucK\chugins
#[cfg(feature = "win")]
pub const CHUCK_PARAM_CHUGIN_DIRECTORY_DEFAULT: &'static [u8; 31usize] =
    b"C:\\Program Files\\ChucK\\chugins\0";
/// /usr/local/lib/chuck
#[cfg(feature = "unix")]
pub const CHUCK_PARAM_CHUGIN_DIRECTORY_DEFAULT: &'static [u8; 21usize] = b"/usr/local/lib/chuck\0";
/// Vec<String>
pub type CHUCK_PARAM_USER_CHUGINS_DEFAULT = Vec<String>;
/// Vec<String>
pub type CHUCK_PARAM_USER_CHUGIN_DIRECTORIES_DEFAULT = Vec<String>;
/// 0
pub const CHUCK_PARAM_HINT_IS_REALTIME_AUDIO_DEFAULT: &'static [u8; 2usize] = b"0\0";
pub struct ChucK {
    m_carrier: *mut ffi::root::Chuck_Carrier,
    m_params: BTreeMap<libc::c_char, libc::c_char>,
    m_listParams: BTreeMap<libc::c_char, Vec<libc::c_char>>,
    m_init: t_CKBOOL,
}
/// master ChucK implementation
impl ChucK {
    // chuck statics
    /// 1.4.0.0 (numchucks)
    pub const VERSION: &'static [u8; 20usize] = b"1.4.0.0 (numchucks)\0";
    /// The number of ChucK VMs
    pub const o_numVMs: &'static crate::def::t_CKUINT = &0;
    /// global init flag
    pub const o_isGlobalInit: &'static crate::def::t_CKBOOL = &false;
    /// enable system call w/ `--caution-to-the-wind` flag
    pub const enableSystemCall: &'static crate::def::t_CKBOOL = &false;
    // base functions
    /// prints [32-bit] to stdout
    #[cfg(target_pointer_width = "32")]
    fn intSize() {
        print!("[32-bit]");
    }
    /// prints [64-bit] to stdout
    #[cfg(target_pointer_width = "64")]
    fn intSize() {
        println!("[64-bit]");
    }
    /// returns ChucK::VERSION
    pub fn version() -> libc::c_char {
        return VERSION;
    }
    /// ChucK constructor
    pub unsafe fn ChucK() -> ChucK {
        let chuck = ChucK::new();
        chuck::m_carrier = Chuck_Carrier::new();
        chuck::m_carrier::chuck = self;
        o_numVMs += 1;
        chuck::initDefaultParams(&mut self);
        chuck::m_init = false;
        return chuck;
    }
    // ~ChucK destructor
    pub unsafe fn dropChucK() {
        shutdown();
        o_numVMs -= 1;
        drop(m_carrier);
    }
    /// init
    pub unsafe fn init() -> bool {
        // sanity check
        if (m_init) {
            ffi::root::ck_fprintf_stderr("[chuck]: VM already initialized...\n");
            return false;
        }
        // initialize VM
        if (!initVM()) {
            cleanup();
        }
        // initialize compiler
        if (!initCompiler()) {
            cleanup();
        }
        // initialize chugin system
        if (!initChugins()) {
            cleanup();
        }
        // initialize OTF programming system
        if (!initOTF()) {
            cleanup();
        }
        // did user init?
        m_init = TRUE;
        return true;
    }
    /// goto cleanup
    pub unsafe fn cleanup() -> bool {
        // shutdown, dealloc
        crate::ffi::root::ChucK::shutdown();
        return false;
    }
    /// initialize the ChucK VM
    pub unsafe fn initVM() -> bool {
        let srate: t_CKUINT = ffi::root::ChucK::getParamInt(CHUCK_PARAM_SAMPLE_RATE);
        let outs: t_CKUINT = ffi::root::ChucK::getParamInt(CHUCK_PARAM_OUTPUT_CHANNELS);
        let ins: t_CKUINT = ffi::root::ChucK::getParamInt(CHUCK_PARAM_INPUT_CHANNELS);
        let adaptiveSize: t_CKUINT = ffi::root::ChucK::getParamInt(CHUCK_PARAM_VM_ADAPTIVE);
        let halt: t_CKBOOL = ffi::root::ChucK::getParamInt(CHUCK_PARAM_VM_HALT) != 0;
        // instantiate VM
        let chuck = ffi::root::ChucK::new();
        chuck::m_carrier::vm = ffi::root::Chuck_VM::new();
        // reference back to carrier
        chuck::m_carrier::vm::setCarrier(chuck::m_carrier);
        // initialize VM
        if (!chuck::m_carrier::vm::initialize(srate, outs, ins, adaptiveSize, halt)) {
            ffi::root::ck_fprintf_stderr("[chuck]: {}\n", chuck::m_carrier::vm::last_error());
            return false;
        }
        return true;
    }
    /// initialize the ChucK compiler
    pub unsafe fn initCompiler() -> bool {
        // get compiler params
        let dump: t_CKBOOL = ffi::root::ChucK::getParamInt(CHUCK_PARAM_DUMP_INSTRUCTIONS) != 0;
        let auto_depend: t_CKBOOL = ffi::root::ChucK::getParamInt(CHUCK_PARAM_AUTO_DEPEND) != 0;
        let workingDir: libc::c_char =
            ffi::root::ChucK::getParamString(CHUCK_PARAM_WORKING_DIRECTORY);
        let chuginDir: libc::c_char =
            ffi::root::ChucK::getParamString(CHUCK_PARAM_CHUGIN_DIRECTORY);
        let deprecate: t_CKUINT = ffi::root::ChucK::getParamInt(CHUCK_PARAM_DEPRECATE_LEVEL);
        // TODO: handle chugin dir stuff
        let chuck = ffi::root::ChucK::new();
        // instantiate compiler
        chuck::m_carrier::compiler = ffi::root::Chuck_Compiler::new();
        // reference back to carrier
        chuck::m_carrier::compiler::setCarrier(chuck::m_carrier);
        // initialize compiler
        if (!chuck::m_carrier::compiler::initialize(dl_search_path, named_dls)) {
            ffi::root::ck_fprintf_stderr("[chuck]: compiler failed to initialize...\n");
            return false;
        }
        // set dump flag
        chuck::m_carrier::compiler::emitter::dump = dump;
        // set auto depend flag (for type checker) | currently must be FALSE
        chuck::m_carrier::compiler::set_auto_depend(auto_depend);
        // set deprecation level
        chuck::m_carrier::env::deprecate_level = deprecate;
        // VM + type system integration (needs to be done after compiler)
        if (!chuck::m_carrier::vm::initialize_synthesis()) {
            ffi::root::ck_fprintf_stderr("[chuck]: {}\n", chuck::m_carrier::vm::last_error());
            return false;
        }
        // TODO: handle cwd stuff
        return true;
    }
    /// initialize chugin
    pub unsafe fn initChugin() -> bool {
        let code = *ffi::root::Chuck_VM_Code;
        let shred = *ffi::root::Chuck_VM_Shred;
        if (ffi::root::ChucK::getParamInt(CHUCK_PARAM_CHUGIN_ENABLE)) {
            ffi::root::EM_log(CK_LOG_SEVERE, "pre-loading ChucK libs...");
            ffi::root::EM_pushlog();
        // TODO: Interate over list of .ck files
        /*
                // iterate over list of ck files that the compiler found
        for( Vec<ffi::root::std::__cxx11::string>::iterator j =
            compiler()->m_cklibs_to_preload.begin();
            j != compiler()->m_cklibs_to_preload.end(); j++)
        {
            // the filename
            let filename: ffi::root::std::__cxx11::string = *j;
            // log
            ffi::root::EM_log( CK_LOG_SEVERE, "preloading '%s'...", filename.c_str() );
            // push indent
            ffi::root::EM_pushlog();
            // SPENCERTODO: what to do for full path
            let full_path: ffi::root::std::__cxx11::string = filename;
            // parse, type-check, and emit
            if( compiler()->go( filename, NULL, NULL, full_path ) )
            {
                // TODO: how to compilation handle?
                //return 1;
                // get the code
                code = compiler()->output();
                // name it - TODO?
                // code->name += string(argv[i]);
                // spork it
                shred = vm()->spork( code, NULL, TRUE );
            }
            // pop indent
            ffi::root::EM_poplog();
        }
        // clear the list of chuck files to preload
        compiler()->m_cklibs_to_preload.clear();
        // pop log
        ffi::root::EM_poplog();
        return true;
        */
        } else {
            // log
            ffi::root::EM_log(CK_LOG_SYSTEM, "chugin system: OFF");
        }
        // load user namespace
        chuck::m_carrier::env::load_user_namespace();
        return true;
    }
    pub unsafe fn initOTF() -> bool {
        let chuck = ffi::root::ChucK::new();
        // server
        if (getParamInt(CHUCK_PARAM_OTF_ENABLE) != 0) {
            chuck::m_carrier::otf_port = getParamInt(CHUCK_PARAM_OTF_PORT);
            // log
            ffi::root::EM_log(
                CK_LOG_SYSTEM,
                "starting listener on port {}",
                chuck::m_carrier::otf_port,
            );
            // start tcp server
            chuck::m_carrier::otf_socket = ck_tcp_create(1);
            if (!chuck::m_carrier::otf_socket
                || !ck_bind(chuck::m_carrier::otf_socket, chuck::m_carrier::otf_port)
                || !ck_listen(chuck::m_carrier::otf_socket, 10))
            {
                ffi::root::ck_fprintf_stderr(
                    "[chuck]: cannot bind to tcp port {}...\n",
                    chuck::m_carrier::otf_port,
                );
                ck_close(chuck::m_carrier::otf_socket);
                chuck::m_carrier::otf_socket = ();
            } else {
                // #[cfg(feature = "win")]
                // let chuck::m_carrier::otf_thread = CreateThread((), 0, (LPTHREAD_START_ROUTINE)otf_cb)
                #[cfg(feature = "unix")]
                libc::pthread_create(&chuck::m_carrier::otf_thread, (), otf_cb, chuck::m_carrier);
            }
        } else {
            // log
            ffi::root::EM_log(CK_LOG_SYSTEM, "OTF server/listener: OFF");
        }
    }
    /// shutdown ChucK instance
    pub unsafe fn shutdown() -> bool {
        // let chuck = ffi::root::ChucK::new();
        // stop VM
        if (m_carrier != () && m_carrier::vm != ()) {
            m_carrier::vm::stop();
            // wait for it to stop
            while (m_carrier::vm::running()) {
                libc::usleep(1000);
            }
        }
        // free vm, compiler, and friends
        stk_detach(m_carrier);
        // cancel otf thread
        if (m_carrier::otf_thread) {
            libc::pthread_cancel(m_carrier::otf_thread);
        }
        // close otf socket
        if (m_carrier::otf_socket) {
            ck_close(m_carrier::otf_socket);
        }
        // reset
        m_carrier::otf_socket = ();
        m_carrier::otf_port = 0;
        m_carrier::otf_thread = 0;
        // TODO: a different way to unlock?
        // unlock all objects to delete chout, cherr
        Chuck_VM_Object::unlock_all();
        SAFE_RELEASE(m_carrier::chout);
        SAFE_RELEASE(m_carrier::cherr);
        // relock
        Chuck_VM_Object::lock_all();
        // clean up vm, compiler
        SAFE_DELETE(m_carrier::vm);
        SAFE_DELETE(m_carrier::compiler);
        m_carrier::env = ();
        // flag
        m_init = FALSE;
        // done
        return true;
    }
    /// compile code directly
    pub unsafe fn compileFile(
        path: &libc::c_char,
        argsTogether: &libc::c_char,
        count: f32,
    ) -> bool {
        // sanity check
        if (!m_carrier::compiler) {
            // error
            ffi::root::ck_fprintf_stderr(
                "[chuck]: compileCode() invoked before initialization ...\n",
            );
            return false;
        }
        let args = Vec::<libc::c_char>;
        let vm_code = *ffi::root::Chuck_VM_Code;
        let shred = *ffi::root::Chuck_VM_Shred;
        // log
        ffi::root::EM_log(CK_LOG_FINE, "compiling string...");
        // push indent
        ffi::root::EM_pushlog();
        // falsify filename / path for various logs
        let theThing: lib::c_char = "compiled.code:" + argsTogether;
        let fakefakeFilename: libc::c_char = "<result file name goes here>";
        // parse out command line arguments
        if (!extract_args(theThing, fakefakeFilename, args)) {
            // error
            ffi::root::ck_fprintf_stderr("[chuck]: malformed filename with argument list...\n");
            ffi::root::ck_fprintf_stderr("    -->  '{}'", theThing.to_string());
            return false;
        }
        // PARAM
        let workingDir: libc::c_char = getParamString(CHUCK_PARAM_WORKING_DIRECTORY);
        // construct full path to be associated with the file so me.sourceDir() works
        let full_path: libc::c_char = workingDir + "/compiled.code";
        // parse, type-check, and emit (full_path added 1.3.0.0)
        if (!m_carrier::compiler::go("<compiled.code>", (), code.to_string(), full_path)) {
            return false;
        }
        // get code
        vm_code = m_carrier::compiler::output();
        // name it (no path to append)
        vm_code::name += "compiled.code";
        // log
        ffi::root::EM_log(CK_LOG_FINE, "sporking {}", count);
        // spork it
        while (count -= 1) {
            // spork (for now, spork_immediate arg is always false)
            shred = m_carrier::vm::spork(vm_code, (), FALSE);
            // add args
            shred::args = args;
        }
        // pop indent
        ffi::root::EM_poplog();
        // reset the parser
        reset_parse();
        return true;
    }
    /// start chuck instance
    pub unsafe fn start(input: *mut SAMPLE, output: *mut SAMPLE, numFrames: f32) -> bool {
        // sanity check
        if (m_carrier::vm == ()) {
            ffi::root::ck_fprintf_stderr("[chuck]: cannot start, VM not initialized...\n");
            return false;
        }
        // start the VM!
        if (!m_carrier::vm::running()) {
            m_carrier::vm::start();
        }
        // return state
        return m_carrier::vm::running();
    }
    /// run engine (call from host callback)
    pub unsafe fn run() {
        // make sure we started...
        if (!m_carrier::vm::running()) {
            self::start();
        }
        // call the callback
        m_carrier::vm::run(numFrames, input, output);
    }
    /// send a message to set the value of a global int
    pub unsafe fn setGlobalInt(name: *const libc::c_char, val: t_CKINT) -> t_CKBOOL {
        if (!m_carrier::vm::running()) {
            return FALSE;
        }
        return m_carrier::vm::set_global_int(name, val);
    }
    /// send a message to get the value of a global int via callback
    pub unsafe fn getGlobalInt(name: *const libc::c_char, callback: *const t_CKINT) -> t_CKBOOL {
        if (!m_carrier::vm::running()) {
            return FALSE;
        }
        return m_carrier::vm::get_global_int(name, callback);
    }
    /// send a message to set the value of a global float
    pub unsafe fn setGlobalFloat(name: *const libc::c_char, val: t_CKFLOAT) -> t_CKBOOL {
        if (!m_carrier::vm::running()) {
            return FALSE;
        }
        return m_carrier::vm::set_global_float(name, val);
    }
    /// send a message to get the value of a global float via callback
    pub unsafe fn getGlobalFloat(name: *const libc::c_char, val: t_CKFLOAT) -> t_CKBOOL {
        if (!m_carrier::vm::running()) {
            return FALSE;
        }
        return m_carrier::vm::get_global_float(name, callback);
    }
    /// send a message to signal a global event
    pub unsafe fn signalGlobalEvent(name: *const libc::c_char) -> t_CKBOOL {
        if (!m_carrier::vm::running()) {
            return FALSE;
        }
        return m_carrier::vm::signal_global_event(name);
    }
    /// send a message to broadcast a global event
    pub unsafe fn broadcastGlobalEvent(name: *const libc::c_char) -> t_CKBOOL {
        if (!m_carrier::vm::running()) {
            return FALSE;
        }
        return m_carrier::vm::broadcast_global_event(name);
    }
    /// provide a callback where Chout print statements are routed
    pub unsafe fn setChoutCallback() -> t_CKBOOL {}
    /// provide a callback where Cherr print statements are routed
    pub unsafe fn setCherrCallback() -> t_CKBOOL {}
    /// provide a callback where stdout print statements are routed
    pub unsafe fn setStdoutCallback() -> t_CKBOOL {}
    /// provide a callback where <<< >>> and stdout print statements are routed
    pub unsafe fn setStderrCallback() -> t_CKBOOL {}
    /// init all things once (for all ChucK instances)
    pub unsafe fn globalInit() -> t_CKBOOL {}
    /// clean up all things
    pub unsafe fn globalCleanup() {}
    /// set log level - should eventually be per-VM
    pub unsafe fn setLogLevel() {}
}
/// handles parameter type
enum ck_param_type {
    ck_param_int,
    ck_param_float,
    ck_param_string,
    ck_param_string_list,
}
/*
ck_param_types[CHUCK_PARAM_SAMPLE_RATE]             = ck_param_int;
ck_param_types[CHUCK_PARAM_INPUT_CHANNELS]          = ck_param_int;
ck_param_types[CHUCK_PARAM_OUTPUT_CHANNELS]         = ck_param_int;
ck_param_types[CHUCK_PARAM_VM_ADAPTIVE]             = ck_param_int;
ck_param_types[CHUCK_PARAM_VM_HALT]                 = ck_param_int;
ck_param_types[CHUCK_PARAM_OTF_ENABLE]              = ck_param_int;
ck_param_types[CHUCK_PARAM_OTF_PORT]                = ck_param_int;
ck_param_types[CHUCK_PARAM_DUMP_INSTRUCTIONS]       = ck_param_int;
ck_param_types[CHUCK_PARAM_DEPRECATE_LEVEL]         = ck_param_int;
ck_param_types[CHUCK_PARAM_WORKING_DIRECTORY]       = ck_param_string;
ck_param_types[CHUCK_PARAM_CHUGIN_DIRECTORY]        = ck_param_string;
ck_param_types[CHUCK_PARAM_CHUGIN_ENABLE]           = ck_param_int;
ck_param_types[CHUCK_PARAM_USER_CHUGINS]            = ck_param_string_list;
ck_param_types[CHUCK_PARAM_USER_CHUGIN_DIRECTORIES] = ck_param_string_list;
ck_param_types[CHUCK_PARAM_HINT_IS_REALTIME_AUDIO]  = ck_param_int;
*/
