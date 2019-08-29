use crate::{ChucK, ck_socket_};
use crate::cereal::{Chuck_IO_Chout, Chuck_IO_Cherr};
use crate::compiler::Chuck_Compiler;
use crate::def::*;
use crate::vm::Chuck_VM;
use crate::ugen::stk::{WvOut, XWriteThread};
use std::collections::BTreeMap;

// C++ forward references
// struct Chuck_Compiler;
// struct Chuck_VM;
// struct Chuck_Env; // subclass of Chuck_VM_Object chuck_type.h
// struct Chuck_IO_Chout; // subclass of Chuck_IO chuck_oo.h
// struct Chuck_IO_Cherr;

// C forward references
// pub type ck_socket = ck_socket_;
// typedef struct ck_socket_ * ck_socket;

pub struct Chuck_Carrier {
    pub chuck: *mut ChucK,
    pub compiler: *mut Chuck_Compiler,
    pub env: *mut Chuck_Env, // subclass of Chuck_VM_Object from chuck_type.h
    pub vm: *mut Chuck_VM,
    pub chout: *mut Chuck_IO_Chout,  // subclass of Chuck_IO from chuck_oo.h
    pub cherr: *mut Chuck_IO_Cherr,  // subclass of Chuck_IO from chuck_oo.h
    pub otf_socket: ck_socket,
    pub otf_port: t_CKINT,
    pub otf_thread: CHUCK_THREAD,
    // stk specific
    pub stk_writeThread: *mut XWriteThread,
    pub stk_wavOutMap: BTreeMap::<*mut WavOut, *mut WavOut>,
}
impl Chuck_Carrier {
    use crate::def::{FALSE, NULL};
    pub fn Chuck_Carrier(chuck: ChucK, compiler: Chuck_Compiler, env: Chuck_Env, vm: Chuck_VM, chout: Chuck_IO_Chout, cherr: Chuck_IO_Cherr, otf_socket: ck_socket, otf_port: t_CKINT, otf_thread: CHUCK_THREAD, stk_writeThread: XWriteThread) -> t_CKBOOL {
        chuck(NULL);
        compiler(NULL);
        env(NULL);
        vm(NULL);
        chout(NULL);
        cherr(NULL);
        otf_socket(NULL);
        otf_port(FALSE);
        otf_thread(FALSE);
        crate::hintIsRealtimeAudio();
    }
}
