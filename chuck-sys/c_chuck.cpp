#include "c_chuck.h" // C wrapper
#include "../chuck/src/core/chuck.h" // ChucK single header magic
/*
core/chuck.h pulls in chuck_compile.h,chuck_dl.h,chuck_vm.h,chuck_shell.h,chuck_carrier.h,ulib_machine,util_math.h,util_string.h,
    hidio_sdl.h,midiio_rtmidi.h
core/chuck_absyn.h pulls in chuck_symbol.h
core/chuck_carrier.h pulls in chuck_def.h,util_thread.h
core/chuck_compile.h pulls in chuck_def.h,chuck_parse.h,chuck_scan.h,chuck_type.h,chuck_emit.h,chuck_vm.h
core/chuck_dl.h pulls in chuck_def.h,chuck_oo.h,chuck_carrier.h
core/chuck_emit.h pulls in chuck_def.h,chuck_oo.h,chuck_type.h,chuck_frame.h
core/chuck_errmsg.h pulls in chuck_def.h
core/chuck_frame.h pulls in chuck_def.h
core/chuck_oo.h pulls in chuck_def.h,chuck_carrier.h,util_thread.h
core/chuck_otf.h pulls in chuck_def.h,util_network.h
core/chuck_parse.h pulls in chuck_def.h,chuck_absyn.h
core/chuck_scan.h pulls in chuck_type.h
core/chuck_shell.h pulls in chuck_def.h,chuck_errmsg.h,chuck_vm.h
core/chuck_symbol.h pulls in chuck_utils.h
core/chuck_type.h pulls in chuck_def.h,chuck_absyn.h,chuck_oo.h,chuck_dl.h,chuck_errmsg.h
core/chuck_ugen.h pulls in chuck_def.h,chuck_oo.h,chuck_dl.h
core/chuck_vm.h pulls in chuck_oo.h,chuck_ugen.h,chuck_carrier.h,util_buffers.h
core/hidio_sdl.h pulls in chuck_def.h,util_buffers.h,util_thread.h,util_hid.h
core/midiio_rtmidi.h pulls in chuck_def.h,rtmidi.h,util_buffers.h
core/ulib_machine.h pulls in chuck_dl.h,chuck_otf.h
core/util_buffers.h pulls in chuck_oo.h,util_thread.h
core/util_string.h pulls in chuck_def.h
core/util_thread.h pulls in chuck_def.h
*/
extern "C" {
    ChucK* newChucK() {
        return new ChucK();
    }
    void destroyChucK(ChucK* v) {
        delete v;
    }
    Chuck_Carrier* newChuck_Carrier() {
        return new Chuck_Carrier();
    }
    void destroyChuck_Carrier(Chuck_Carrier* v) {
        delete v;
    }
    Chuck_VM* newChuck_VM() {
        return new Chuck_VM();
    }
    void destroyChuck_VM(Chuck_VM* v) {
        delete v;
    }
    Chuck_VM_Code* newChuck_VM_Code() {
        return new Chuck_VM_Code();
    }
    void destroyChuck_VM_Code(Chuck_VM_Code* v) {
        delete v;
    }
    Chuck_VM_Shred* newChuck_VM_Shred() {
        return new Chuck_VM_Shred();
    }
    void destroyChuck_VM_Shred(Chuck_VM_Shred* v) {
        delete v;
    }
    Chuck_VM_Object* newChuck_VM_Object() {
        return new Chuck_VM_Object();
    }
    void destroyChuck_VM_Object(Chuck_VM_Object* v) {
        delete v;
    }
    // Chuck_IO_Serial* newChuck_IO_Serial() {
    //     return new Chuck_IO_Serial();
    // }
    // void destroyChuck_IO_Serial(Chuck_IO_Serial* v) {
    //     delete v;
    // }
    Chuck_Compiler* newChuck_Compiler() {
        return new Chuck_Compiler();
    }
    void destroyChuck_Compiler(Chuck_Compiler* v) {
        delete v;
    }
    HidInManager* newHidInManager() {
        return new HidInManager();
    }
    void destroyHidInManager(HidInManager* v) {
        delete v;
    }
}
/* WHITELIST
SAMPLE type
t_CK.* type
ChucK
Chuck_Carrier type
Chuck_VM.* type
Chuck_VM_Code type
Chuck_VM_Shred type
Chuck_VM_Object type
Chuck_Compiler type
f_ck_query
m_query_func
ck_param_type type
CHUCK_PARAM_.* type
CK_FPRINTF_STDERR function
MAXPATHLEN type
CK_LOG_.* type
EM_.* function
HidInManager type
Chuck_IO_Serial type
stk_detach function
*/
