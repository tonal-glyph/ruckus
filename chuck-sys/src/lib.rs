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



// submodules
// pub mod audio;
// pub mod carrier;
// pub mod cereal;
// pub mod compiler;
// pub mod def;
// pub mod ffi; // pub during development, will eventually be private
// pub mod midi;
// pub mod ugen;
// pub mod util;
// pub mod vm;

// // imports
// use cfg_if;
// use std::collections::BTreeMap;
// use std::mem::zeroed;
// use carrier::Chuck_Carrier;
// use def::*;
// use libc::{
//     c_char,
//     c_float,
//     pthread_cancel,
//     pthread_create,
//     usleep
// };

// // C++ macro definitions
// pub const HAVE_CONFIG_H: u32 = 1;
// pub const HAVE_POLL: u32 = 1;
// pub const HAVE_LIBPTHREAD: u32 = 1;
// pub const ENABLE_THREADS: u32 = 1;
// pub const __PLATFORM_LINUX__: u32 = 1;
// pub const __LINUX_ALSA__: u32 = 1;
// pub const __UNIX_JACK__: u32 = 1;
// pub const __LINUX_PULSE__: u32 = 1;

// // parameter section
// // chuck param names
// pub const CHUCK_PARAM_SAMPLE_RATE: &'static [u8; 12usize] = b"SAMPLE_RATE\0";
// pub const CHUCK_PARAM_INPUT_CHANNELS: &'static [u8; 15usize] = b"INPUT_CHANNELS\0";
// pub const CHUCK_PARAM_OUTPUT_CHANNELS: &'static [u8; 16usize] = b"OUTPUT_CHANNELS\0";
// pub const CHUCK_PARAM_VM_ADAPTIVE: &'static [u8; 12usize] = b"VM_ADAPTIVE\0";
// pub const CHUCK_PARAM_VM_HALT: &'static [u8; 8usize] = b"VM_HALT\0";
// pub const CHUCK_PARAM_OTF_ENABLE: &'static [u8; 11usize] = b"OTF_ENABLE\0";
// pub const CHUCK_PARAM_OTF_PORT: &'static [u8; 9usize] = b"OFT_PORT\0";
// pub const CHUCK_PARAM_DUMP_INSTRUCTIONS: &'static [u8; 18usize] = b"DUMP_INSTRUCTIONS\0";
// pub const CHUCK_PARAM_AUTO_DEPEND: &'static [u8; 12usize] = b"AUTO_DEPEND\0";
// pub const CHUCK_PARAM_DEPRECATE_LEVEL: &'static [u8; 16usize] = b"DEPRECATE_LEVEL\0";
// pub const CHUCK_PARAM_WORKING_DIRECTORY: &'static [u8; 18usize] = b"WORKING_DIRECTORY\0";
// pub const CHUCK_PARAM_CHUGIN_ENABLE: &'static [u8; 14usize] = b"CHUGIN_ENABLE\0";
// pub const CHUCK_PARAM_CHUGIN_DIRECTORY: &'static [u8; 17usize] = b"CHUGIN_DIRECTORY\0";
// pub const CHUCK_PARAM_USER_CHUGINS: &'static [u8; 13usize] = b"USER_CHUGINS\0";
// pub const CHUCK_PARAM_USER_CHUGIN_DIRECTORIES: &'static [u8; 24usize] = b"USER_CHUGIN_DIRECTORIES\0";
// pub const CHUCK_PARAM_HINT_IS_REALTIME_AUDIO: &'static [u8; 23usize] = b"HINT_IS_REALTIME_AUDIO\0";

// // chuck param defaults
// /// 44100
// pub const CHUCK_PARAM_SAMPLE_RATE_DEFAULT: &'static [u8; 6usize] = b"44100\0";
// /// 2
// pub const CHUCK_PARAM_INPUT_CHANNELS_DEFAULT: &'static [u8; 2usize] = b"2\0";
// /// 2
// pub const CHUCK_PARAM_OUTPUT_CHANNELS_DEFAULT: &'static [u8; 2usize] = b"2\0";
// /// 0
// pub const CHUCK_PARAM_VM_ADAPTIVE_DEFAULT: &'static [u8; 2usize] = b"0\0";
// /// 0
// pub const CHUCK_PARAM_VM_HALT_DEFAULT: &'static [u8; 2usize] = b"0\0";
// /// 0
// pub const CHUCK_PARAM_OTF_ENABLE_DEFAULT: &'static [u8; 2usize] = b"0\0";
// /// 8888
// pub const CHUCK_PARAM_OTF_PORT_DEFAULT: &'static [u8; 5usize] = b"8888\0";
// /// 0
// pub const CHUCK_PARAM_DUMP_INSTRUCTIONS_DEFAULT: &'static [u8; 2usize] = b"0\0";
// /// 1
// pub const CHUCK_PARAM_AUTO_DEPEND_DEFAULT: &'static [u8; 2usize] = b"1\0";
// /// 1
// pub const CHUCK_PARAM_DEPRECATE_LEVEL_DEFAULT: &'static [u8; 2usize] = b"1\0";
// /// ""
// pub const CHUCK_PARAM_WORKING_DIRECTORY_DEFAULT: &'static [u8; 1usize] = b"\0";
// /// 1
// pub const CHUCK_PARAM_CHUGIN_ENABLE_DEFAULT: &'static [u8; 2usize] = b"1\0";
// /// C:\Program Files\ChucK\chugins
// #[cfg(target_os = "windows")]
// pub const CHUCK_PARAM_CHUGIN_DIRECTORY_DEFAULT: &'static [u8; 31usize] =
//     b"C:\\Program Files\\ChucK\\chugins\0";
// /// /usr/local/lib/chuck
// #[cfg(target_os = "linux")]
// pub const CHUCK_PARAM_CHUGIN_DIRECTORY_DEFAULT: &'static [u8; 21usize] = b"/usr/local/lib/chuck\0";
// /// Vec<c_char>
// pub type CHUCK_PARAM_USER_CHUGINS_DEFAULT = Vec<c_char>;
// /// Vec<c_char>
// pub type CHUCK_PARAM_USER_CHUGIN_DIRECTORIES_DEFAULT = Vec<c_char>;
// /// 0
// pub const CHUCK_PARAM_HINT_IS_REALTIME_AUDIO_DEFAULT: &'static [u8; 2usize] = b"0\0";

// // chuck statics
// /// 1.4.0.0 (numchucks)
// pub const VERSION: &'static [u8; 20usize] = b"1.4.0.0 (numchucks)\0";
// /// The number of ChucK VMs
// pub const o_numVMs: &'static t_CKUINT = &0;
// /// global init flag
// pub const o_isGlobalInit: &'static t_CKBOOL = &false;
// /// enable system call w/ `--caution-to-the-wind` flag
// pub const enableSystemCall: &'static t_CKBOOL = &false;
// /// handles parameter type
// enum ck_param_type {
//     ck_param_int = u32,
//     ck_param_float = f32,
//     ck_param_string = String,
//     ck_param_string_list = Vec::<String>,
// }

// /// abstraction of master ChucK class, entry into library
// pub struct ChucK {
//     // fields
//     m_carrier: *mut crate::carrier::Chuck_Carrier,
//     m_params: BTreeMap<c_char, c_char>, //std::map equivalent
//     m_listParams: BTreeMap<c_char, Vec<c_char>>, //std::map equivalent
//     m_init: t_CKBOOL,
// }

// /// master ChucK implementation
// impl ChucK {
//     // base functions
//     /// prints [32-bit] to stdout
//     #[cfg(target_pointer_width = "32")]
//     pub fn intSize() {
//         print!("[32-bit]");
//     }
//     /// prints [64-bit] to stdout
//     #[cfg(target_pointer_width = "64")]
//     pub fn intSize() {
//         print!("[64-bit]");
//     }
//     /// returns ChucK::VERSION
//     pub fn version() {
//         VERSION;
//     }
//     /// ChucK constructor
//     pub fn ChucK(self: &Self) {
//         let chuck = ffi::root::ChucK.new();
//         (*chuck.m_carrier) = Chuck_Carrier::new();
//         (*chuck.m_carrier).chuck = &self;
//         chuck.o_numVMs += 1;
//         chuck.initDefaultParams(&mut self);
//         chuck.m_init = false;
//         chuck;
//     }
//     // ~ChucK destructor
//     pub fn destroyChucK(chuck: ChucK) {
//         chuck.shutdown();
//         o_numVMs -= 1;
//         drop((*chuck.m_carrier));
//         true;
//     }
//     /// init
//     pub fn init(chuck: ChucK) -> bool {
//         // sanity check
//         if chuck.m_init {
//             // ck_fprintf_stderr("[chuck]: VM already initialized...\n");
//             return false;
//         }
//         // initialize VM
//         // if !initVM(chuck) {
//         //     chuck.cleanup();
//         // }
//         // initialize compiler
//         // if !initCompiler(chuck) {
//         //     chuck.cleanup();
//         // }
//         // initialize chugin system
//         // if !initChugins(chuck) {
//         //     chuck.cleanup();
//         // }
//         // initialize OTF programming system
//         // if !initOTF(chuck) {
//         //     chuck.cleanup();
//         // }
//         // did user init?
//         chuck.m_init = TRUE;
//         true;
//     }
//     /// goto cleanup
//     pub fn cleanup(chuck: ffi::root::ChucK) -> bool {
//         // shutdown, dealloc
//         chuck.shutdown();
//         false;
//     }
//     // /// initialize the ChucK VM
//     // pub fn initVM(chuck: ChucK) -> bool {
//     //     let srate: t_CKUINT = ChucK.getParamInt(CHUCK_PARAM_SAMPLE_RATE);
//     //     let outs: t_CKUINT = ChucK.getParamInt(CHUCK_PARAM_OUTPUT_CHANNELS);
//     //     let ins: t_CKUINT = ChucK.getParamInt(CHUCK_PARAM_INPUT_CHANNELS);
//     //     let adaptiveSize: t_CKUINT = ChucK.getParamInt(CHUCK_PARAM_VM_ADAPTIVE);
//     //     let halt: t_CKBOOL = ChucK.getParamInt(CHUCK_PARAM_VM_HALT) != 0;
//     //     // instantiate VM
//     //     let chuck = ChucK::new();
//     //     (*chuck.m_carrier).vm = Chuck_VM::new();
//     //     // reference back to carrier
//     //     (*chuck.m_carrier).vm.setCarrier((*chuck.m_carrier));
//     //     // initialize VM
//     //     if (!(*chuck.m_carrier).vm.initialize(srate, outs, ins, adaptiveSize, halt)) {
//     //         ck_fprintf_stderr("[chuck]: {}\n", (*chuck.m_carrier).vm.last_error());
//     //         return false;
//     //     }
//     //     return true;
//     // }
//     /// initialize chugin
//     pub fn initChugin(chuck: ChucK) -> bool {
//         let code = *ffi::root::Chuck_VM_Code;
//         let shred = *ffi::root::Chuck_VM_Shred;
//         if chuck.getParamInt(CHUCK_PARAM_CHUGIN_ENABLE) {
//             ffi::root::EM_log(CK_LOG_SEVERE, "pre-loading ChucK libs...");
//             ffi::root::EM_pushlog();
//             // iterate over list of ck files that the compiler found

//             // clear the list of chuck files to preload
//             (*ChucK::compiler()).m_cklibs_to_preload.clear();
//             // pop log
//             ffi::root::EM_poplog();
//             return true;
//         } else {
//             // log
//             ffi::root::EM_log(CK_LOG_SYSTEM, "chugin system: OFF");
//             return false;
//         }
//         // load user namespace
//         (*chuck.m_carrier)::env.load_user_namespace();
//         return true;
//     }
//     pub fn initOTF(chuck: ChucK) -> bool {
//         // server
//         if getParamInt(CHUCK_PARAM_OTF_ENABLE) != 0 {
//             otf_port = getParamInt(CHUCK_PARAM_OTF_PORT);
//             // log
//             ffi::root::EM_log(CK_LOG_SYSTEM, "starting listener on port {}", (*chuck.m_carrier).otf_port);
//             // start tcp server
//             otf_socket = ck_tcp_create(1);
//             if (!(*chuck.m_carrier).otf_socket || !ck_bind((*chuck.m_carrier).otf_socket, (*chuck.m_carrier).otf_port) || !ck_listen((*chuck.m_carrier).otf_socket, 10)) {
//                 // ck_fprintf_stderr("[chuck]: cannot bind to tcp port {}...\n", (*chuck.m_carrier).otf_port);
//                 ck_close(otf_socket);
//                 // otf_socket set to NULL NULL
//                 otf_socket = NULL;
//             } else {
//                 // #[cfg(target_os = "windows")]
//                 // let (*chuck.m_carrier).otf_thread = CreateThread((), 0, (LPTHREAD_START_ROUTINE)otf_cb)
//                 #[cfg(target_os = "linux")]
//                 pthread_create(&otf_thread, NULL, otf_cb, (*chuck.m_carrier));
//             }
//         } else {
//             // log
//             ffi::root::EM_log(CK_LOG_SYSTEM, "OTF server/listener: OFF");
//         }
//         return true;
//     }
//     /// shutdown ChucK instance
//     pub fn shutdown(chuck: ChucK) -> bool {
//         // stop VM
//         if ((*chuck.m_carrier) != NULL && (*chuck.m_carrier).vm != NULL) {
//             (*(*chuck.m_carrier)).vm.stop();
//             // wait for it to stop
//             while (*chuck.m_carrier).vm.running() {
//                 usleep(1000);
//             }
//         }
//         // free vm, compiler, and friends
//         // stk_detach((*chuck.m_carrier));
//         // cancel otf thread
//         if otf_thread {
//             // pthread_cancel((*chuck.m_carrier).otf_thread);
//         }
//         // close otf socket
//         if ((*chuck.m_carrier).otf_socket) {
//             ck_close(chuck.m_carrier.otf_socket);
//         }
//         // reset
//         (*chuck.m_carrier).otf_socket = NULL;
//         (*chuck.m_carrier).otf_port = 0;
//         (*chuck.m_carrier).otf_thread = 0;
//         // TODO: a different way to unlock?
//         // unlock all objects to delete chout, cherr
//         Chuck_VM_Object.unlock_all();
//         SAFE_RELEASE((*chuck.m_carrier).chout);
//         SAFE_RELEASE((*chuck.m_carrier).cherr);
//         // relock
//         Chuck_VM_Object.lock_all();
//         // clean up vm, compiler
//         SAFE_DELETE((*chuck.m_carrier).vm);
//         SAFE_DELETE((*chuck.m_carrier).compiler);
//         (*chuck.m_carrier).env = NULL;
//         // flag
//         chuck.m_init = FALSE;
//         // done
//         return true;
//     }
//     /// compile code directly
//     pub fn compileFile(chuck: ChucK, path: &c_char, argsTogether: &c_char, count: f32) -> bool {
//         // sanity check
//         if !(*chuck.m_carrier).compiler {
//             // error
//             // ck_fprintf_stderr("[chuck]: compileCode() invoked before initialization ...\n");
//             return false;
//         }
//         let args = Vec::<c_char>;
//         let vm_code = *Chuck_VM_Code;
//         let shred = *Chuck_VM_Shred;
//         // log
//         ffi::root::EM_log(CK_LOG_FINE, "compiling string...");
//         // push indent
//         ffi::root::EM_pushlog();
//         // falsify filename / path for various logs
//         let theThing: c_char = argsTogether.to_string();
//         let fakefakeFilename: c_char = "<result file name goes here>";
//         // parse out command line arguments
//         if !extract_args(theThing, fakefakeFilename, args) {
//             // error
//             // ck_fprintf_stderr("[chuck]: malformed filename with argument list...\n");
//             // ck_fprintf_stderr("    -->  '{}'", theThing.to_string());
//             return false;
//         }
//         // PARAM
//         let workingDir: c_char = chuck.getParamString(CHUCK_PARAM_WORKING_DIRECTORY);
//         // construct full path to be associated with the file so me.sourceDir() works
//         let full_path: c_char = workingDir.to_string();
//         // parse, type-check, and emit (full_path added 1.3.0.0)
//         if !(*chuck.m_carrier).compiler.go("<compiled.code>", NULL, code.to_string(), full_path) {
//             return false;
//         }
//         // get code
//         vm_code = (*chuck.m_carrier).compiler.output();
//         // name it (no path to append)
//         vm_code.name += "compiled.code";
//         // log
//         ffi::root::EM_log(CK_LOG_FINE, "sporking {}", count);
//         // spork it
//         while count -= 1 {
//             // spork (for now, spork_immediate arg is always false)
//             shred = (*chuck.m_carrier).vm.spork(vm_code, NULL, FALSE);
//             // add args
//             shred.args = args;
//         }
//         // pop indent
//         ffi::root::EM_poplog();
//         // reset the parser
//         reset_parse();
//         return true;
//     }
//     /// start chuck instance
//     pub fn start(chuck: ChucK, input: *mut SAMPLE, output: *mut SAMPLE, numFrames: c_float) -> bool {
//         // sanity check
//         if (*chuck.m_carrier).vm == NULL {
//             // ck_fprintf_stderr("[chuck]: cannot start, VM not initialized...\n");
//             return false;
//         }
//         // start the VM!
//         if !(*chuck.m_carrier).vm.running() {
//             (*chuck.m_carrier).vm.start();
//         }
//         // return state
//         return (*chuck.m_carrier).vm.running();
//     }
//     /// run engine (call from host callback)
//     pub fn run(chuck: ChucK) {
//         // make sure we started...
//         if !(*chuck.m_carrier).vm.running() {
//             chuck.start();
//         }
//         // call the callback
//         (*chuck.m_carrier).vm.run(numFrames, input, output);
//     }
//     /// is initialized
//     pub fn running (chuck: ChucK) -> bool {
//         return chuck.m_init;
//     }
//     /// additional native chuck bindings/types (use with extra caution)
//     // pub fn bind(queryFunc: f_ck_query, name: c_char) -> bool {

//     // }
//     /// get VM (dangerous)
//     pub unsafe fn vm(chuck: ChucK) -> *mut ffi::root::Chuck_VM {
//         return *(*chuck.m_carrier).vm;
//     }
//     pub unsafe fn compiler(chuck: ChucK) -> *mut ffi::root::Chuck_Compiler {
//         return (*chuck.m_carrier).compiler;
//     }
//     /// send a message to set the value of a global int
//     pub fn setGlobalInt(chuck: ChucK, name: *const c_char, val: t_CKINT) -> t_CKBOOL {
//         if (!(*chuck.m_carrier).vm.running()) {
//             return FALSE;
//         }
//         return (*chuck.m_carrier).vm.set_global_int(name, val);
//     }
//     /// send a message to get the value of a global int via callback
//     pub fn getGlobalInt(chuck: ChucK, name: *const c_char, callback: *const t_CKINT) -> t_CKBOOL {
//         if (!(*chuck.m_carrier).vm.running()) {
//             return FALSE;
//         }
//         return (*chuck.m_carrier).vm.get_global_int(name, callback);
//     }
//     /// send a message to set the value of a global float
//     pub fn setGlobalFloat(chuck: ChucK, name: *const c_char, val: t_CKFLOAT) -> t_CKBOOL {
//         if (!(*chuck.m_carrier).vm.running()) {
//             return FALSE;
//         }
//         return (*chuck.m_carrier).vm.set_global_float(name, val);
//     }
//     /// send a message to get the value of a global float via callback
//     pub fn getGlobalFloat(chuck: ChucK, name: *const c_char, val: t_CKFLOAT) -> t_CKBOOL {
//         if (!(*chuck.m_carrier).vm.running()) {
//             return FALSE;
//         }
//         return ChucK.m_carrier.vm.get_global_float(name, callback);
//     }
//     /// send a message to signal a global event
//     pub fn signalGlobalEvent(chuck: ChucK, name: *const c_char) -> t_CKBOOL {
//         if (!(*chuck.m_carrier).vm.running()) {
//             return FALSE;
//         }
//         return (*chuck.m_carrier).vm.signal_global_event(name);
//     }
//     /// send a message to broadcast a global event
//     pub fn broadcastGlobalEvent(chuck: ChucK, name: *const c_char) -> t_CKBOOL {
//         if (!(*chuck.m_carrier).vm.running()) {
//             return FALSE;
//         }
//         return (*chuck.m_carrier).vm.broadcast_global_event(name);
//     }
//     /// provide a callback where Chout print statements are routed
//     pub fn setChoutCallback(chuck: ChucK, callback: *const c_char) -> t_CKBOOL {
//         if !chuck.m_init || (*chuck.m_carrier).chout == NULL {
//             return FALSE;
//         }
//         (*chuck.m_carrier).chout.set_output_callback(callback);
//         return TRUE;
//     }
//     /// provide a callback where Cherr print statements are routed
//     pub fn setCherrCallback(chuck: ChucK, callback: *const c_char) -> t_CKBOOL {
//         if !chuck.m_init || (*chuck.m_carrier).cherr == NULL {
//             return FALSE;
//         }
//         (*chuck.m_carrier).cherr.set_output_callback(callback);
//         return TRUE;
//     }
//     /// provide a callback where stdout print statements are routed
//     pub fn setStdoutCallback(callback: *const c_char) -> t_CKBOOL {
//         ck_set_stdout_callback(callback);
//         return TRUE;
//     }
//     /// provide a callback where <<< >>> and stdout print statements are routed
//     pub fn setStderrCallback(callback: *const c_char) -> t_CKBOOL {
//         ck_set_stderr_callback(callback);
//         return TRUE;
//     }
//     /// init all things once (for all ChucK instances)
//     pub fn globalInit(chuck: ChucK) -> t_CKBOOL {
//         // sanity check
//         if chuck.o_isGlobalInit {
//             return FALSE;
//         }
//         // nothing to do, for now
//         // set flag
//         chuck.o_isGlobalInit = TRUE;
//         // done
//         return TRUE;
//     }
//     /// clean up all things
//     pub fn globalCleanup(chuck: ChucK) {
//         // sanity check
//         if !chuck.o_isGlobalInit {
//             return;
//         }
//         // set flag
//         chuck.o_isGlobalInit = FALSE;
//         // log
//         ffi::root::EM_log(CK_LOG_INFO, "detaching all resources...");
//         // push
//         ffi::root::EM_pushlog();
//         // cfg_if! {
//         //     if !__DISABLE_MIDI__ {
//         //         midirw_detach();
//         //     }
//         // }
//         // cfg_if! {
//         //     if !__ALTER_HID__ {
//         //         // shutdown HID
//         //         HidInManager.cleanup();
//         //     }
//         // }
//         // shutdown serial
//         Chuck_IO_Serial.shutdown();
//         // cfg_if! {
//         //     if !__DISABLE_KBHIT__ {
//         //         // shutdown kb loop
//         //         KBHitManager.shutdown();
//         //     }
//         // }
//         // pop
//         ffi::root::EM_poplog();
//     }
//     /// set log level - should eventually be per-VM
//     pub fn setLogLevel(level: t_CKINT) {
//         EM_setlog(level);
//     }
//     /// get log level -- also per-VM?
//     pub fn getLogLevel() {
//         return g_loglevel;
//     }
//     /// --poop flag compatibility
//     pub fn poop() {
//         // uh();
//         return;
//     }
//     /// number of ChucKs
//     pub fn numVMs(chuck: ChucK) {
//         return chuck.o_numVMs;
//     }
//     pub fn initDefaultParams(chuck: ChucK) {
//         // chuck.m_params[CHUCK_PARAM_SAMPLE_RATE] = CHUCK_PARAM_SAMPLE_RATE_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_INPUT_CHANNELS] = CHUCK_PARAM_INPUT_CHANNELS_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_OUTPUT_CHANNELS] = CHUCK_PARAM_OUTPUT_CHANNELS_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_VM_ADAPTIVE] = CHUCK_PARAM_VM_ADAPTIVE_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_VM_HALT] = CHUCK_PARAM_VM_HALT_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_OTF_ENABLE] = CHUCK_PARAM_OTF_ENABLE;
//         // chuck.m_params[CHUCK_PARAM_OTF_PORT] = CHUCK_PARAM_OTF_PORT_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_DUMP_INSTRUCTIONS] = CHUCK_PARAM_DUMP_INSTRUCTIONS_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_DEPRECATE_LEVEL] = CHUCK_PARAM_DEPRECATE_LEVEL_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_WORKING_DIRECTORY] = CHUCK_PARAM_WORKING_DIRECTORY_DEFAULT;
//         // chuck.m_params[CHUCK_PARAM_CHUGIN_DIRECTORY] = CHUCK_PARAM_CHUGIN_DIRECTORY;
//         // chuck.m_params[CHUCK_PARAM_CHUGIN_ENABLE] = CHUCK_PARAM_CHUGIN_ENABLE_DEFAULT;
//         // chuck.m_listParams[CHUCK_PARAM_USER_CHUGINS] = CHUCK_PARAM_USER_CHUGINS_DEFAULT;
//         // chuck.m_listParams[CHUCK_PARAM_USER_CHUGIN_DIRECTORIES] = CHUCK_PARAM_USER_CHUGINS;
//         // chuck.m_params[CHUCK_PARAM_HINT_IS_REALTIME_AUDIO] = CHUCK_PARAM_HINT_IS_REALTIME_AUDIO_DEFAULT;
//         // ck_param_types[CHUCK_PARAM_SAMPLE_RATE] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_INPUT_CHANNELS] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_OUTPUT_CHANNELS] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_VM_ADAPTIVE] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_VM_HALT] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_OTF_ENABLE] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_OTF_PORT] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_DUMP_INSTRUCTIONS] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_DEPRECATE_LEVEL] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_WORKING_DIRECTORY] = ck_param_string;
//         // ck_param_types[CHUCK_PARAM_CHUGIN_DIRECTORY] = ck_param_string;
//         // ck_param_types[CHUCK_PARAM_CHUGIN_ENABLE] = ck_param_int;
//         // ck_param_types[CHUCK_PARAM_USER_CHUGINS] = ck_param_string_list;
//         // ck_param_types[CHUCK_PARAM_USER_CHUGIN_DIRECTORIES] = ck_param_string_list;
//         // ck_param_types[CHUCK_PARAM_HINT_IS_REALTIME_AUDIO] = ck_param_int;
//     }
//     /// initialize VM
//     pub fn initVM(chuck: ChucK) -> bool {
//         // get VM params
//         let srate: t_CKUINT = chuck.getParamInt(CHUCK_PARAM_SAMPLE_RATE);
//         let outs: t_CKUINT = chuck.getParamInt(CHUCK_PARAM_OUTPUT_CHANNELS);
//         let ins: t_CKUINT = chuck.getParamInt(CHUCK_PARAM_INPUT_CHANNELS);
//         let adaptiveSize: t_CKUINT = chuck.getParamInt(CHUCK_PARAM_VM_ADAPTIVE);
//         let halt: t_CKBOOL = chuck.getParamInt(CHUCK_PARAM_VM_HALT);
//         // instantiate VM (Chuck_VM constructor)
//         (*chuck.m_carrier).vm = Chuck_VM();
//         // reference back to carrier
//         (*chuck.m_carrier).vm.setCarrier((*chuck.m_carrier));
//         // initialize VM
//         if !(*chuck.m_carrier).vm.initialize(srate, outs, ins, adaptiveSize, halt) {
//             ck_fprintf_stderr("[chuck[ {}\n", (*chuck.m_carrier).vm.last_error().to_string());
//             return false;
//         }
//         return true;
//     }
//     /// initialize compiler
//     pub fn initCompiler(chuck: ChucK) -> bool {
//         // get compiler params
//         let dump: t_CKBOOL = getParamInt(CHUCK_PARAM_DUMP_INSTRUCTIONS) != 0;
//         let auto_depend: t_CKBOOL = getParamInt(CHUCK_PARAM_AUTO_DEPEND) != 0;
//         let workingDir: c_char = getParamString(CHUCK_PARAM_WORKING_DIRECTORY);
//         let chuginDir: c_char = getParamString(CHUCK_PARAM_CHUGIN_DIRECTORY);
//         let deprecate: t_CKUINT = getParamInt(CHUCK_PARAM_DEPRECATE_LEVEL);
//         // list of search paths (added 1.3.0.0)
//         let dl_search_path: Vec::<c_char> = getParamStringList(CHUCK_PARAM_USER_CHUGIN_DIRECTORIES);
//         if chuginDir != c_char {
//             dl_search_path.push(chuginDir);
//         }
//         // list of individually named chug-ins (added 1.3.0.0)
//         let named_dls: Vec::<c_char> = getParamStringList(CHUCK_PARAM_USER_CHUGINS);
//         // if chugin load is off, then clear the lists (added 1.3.0.0 -- TODO: refactor)
//         if getParamInt(CHUCK_PARAM_CHUGIN_ENABLE) == 0 {
//             // turn off chugin load
//             dl_search_path.clear();
//             named_dls.clear();
//         }
//         // instantiate compiler (Chuck_Compiler constructor)
//         (*chuck.m_carrier).compiler = Chuck_Compiler();
//         // reference back to carrier
//         (*chuck.m_carrier).compiler.setCarrier((*chuck.m_carrier));
//         // initialize compiler
//         if !(*chuck.m_carrier).compiler.initialize(dl_search_path, named_dls) {
//             ck_fprintf_stderr("[chuck]: compiler failed to initialize...\n");
//             return false;
//         }
//         // set dump flag
//         (*chuck.m_carrier).compiler.emitter.dump = dump;
//         // set auto depend flag (for type checker) | currently must be FALSE
//         *chuck.m_carrier.compiler.set_auto_depend(auto_depend);
//         // set deprecation level
//         *chuck.m_carrier.env.deprecate_level = deprecate;
//         // VM + type system integration (needs to be done after compiler)
//         if !*chuck.m_carrier.vm.initialize_synthesis() {
//             ck_fprintf_stderr("[chuck]: {}\n", (*chuck.m_carrier)vm.last_error().to_string);
//             return false;
//         }
//         // let cwd = *c_char;
//         // let cstr_cwd[MAXPATHLEN; c_char] = c_char;
//         // figure out current working directory (added 1.3.0.0)
//         // is this needed for current path to work correctly?!
//         // if getcwd(cstr_cwd, MAXPATHLEN) == NULL {
//         //     // uh...
//         //     ffi::root::EM_log(CK_LOG_SEVERE, "error: unable to determine current working directory!");
//         // } else {
//         //     cwd = cstr_cwd;
//         //     cwd = concat!(normalize_directory_separator(cwd), "/");
//         // }
//         return true;
//     }
// }
