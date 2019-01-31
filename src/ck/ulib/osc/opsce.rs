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
///* class library for open sound control
#![feature(libc)]
use libc::*;
use crate::ck::def::defe::*;
use crate::ck::dynl::dynle::*;
use crate::ck::oo::ooe::*;
pub fn DLLQUERY() {
    opensoundcontrol_query(QUERY: *mut Chuck_DL_Query);
}
pub fn main() {
    DLLQUERY();
    CK_DLL_CTOR(osc_send_ctor);
    CK_DLL_DTOR(osc_send_dtor);
    CK_DLL_MFUN(osc_send_setHost);
    CK_DLL_MFUN(osc_send_startMesg);
    CK_DLL_MFUN(osc_send_startMesg_spec);
    CK_DLL_MFUN(osc_send_addInt);
    CK_DLL_MFUN(osc_send_addFloat);
    CK_DLL_MFUN(osc_send_addString);
    CK_DLL_MFUN(osc_send_openBundle);
    CK_DLL_MFUN(osc_send_closeBundle);
    CK_DLL_MFUN(osc_send_holdMesg);
    CK_DLL_MFUN(osc_send_kickMesg);
    CK_DLL_CTOR(osc_address_ctor);
    CK_DLL_DTOR(osc_address_dtor);
    CK_DLL_MFUN(osc_address_set);
    CK_DLL_MFUN(osc_address_can_wait);
    CK_DLL_MFUN(osc_address_has_mesg);
    CK_DLL_MFUN(osc_address_next_mesg);
    CK_DLL_MFUN(osc_address_next_int);
    CK_DLL_MFUN(osc_address_next_float);
    CK_DLL_MFUN(osc_address_next_string);
    CK_DLL_CTOR(osc_recv_ctor);
    CK_DLL_DTOR(osc_recv_dtor);
    CK_DLL_MFUN(osc_recv_port);
    CK_DLL_MFUN(osc_recv_new_address);
    CK_DLL_MFUN(osc_recv_new_address_type);
    CK_DLL_MFUN(osc_recv_add_address);
    CK_DLL_MFUN(osc_recv_remove_address);
    CK_DLL_MFUN(osc_recv_listen);
    CK_DLL_MFUN(osc_recv_listen_port);
    CK_DLL_MFUN(osc_recv_listen_stop);
}
