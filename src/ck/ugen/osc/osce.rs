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
use crate::chuck_def_h_edited::*;
use crate::chuck_dl_h_edited::*;
///* oscilliator unit generators
use libc::*;
pub fn DLLQUERY() {
    osc_query(query: *mut Chuck_DL_Query);
    genX_query(query: *mut Chuck_DL_Query);
}
pub fn main() {
    DLLQUERY();
    // osc - base
    CK_DLL_CTOR(osc_ctor);
    CK_DLL_DTOR(osc_dtor);
    CK_DLL_TICK(osc_tick);
    CK_DLL_PMSG(osc_pmsg);
    CK_DLL_CTRL(osc_ctrl_freq);
    CK_DLL_CGET(osc_cget_freq);
    CK_DLL_CTRL(osc_ctrl_period);
    CK_DLL_CGET(osc_cget_period);
    CK_DLL_CTRL(osc_ctrl_phase);
    CK_DLL_CGET(osc_cget_phase);
    CK_DLL_CTRL(osc_ctrl_width);
    CK_DLL_CGET(osc_cget_width);
    CK_DLL_CTRL(osc_ctrl_sync);
    CK_DLL_CGET(osc_cget_sync);
    // sinosc
    CK_DLL_TICK(sinosc_tick);
    // pulseosc
    CK_DLL_TICK(pulseosc_tick);
    // triosc
    CK_DLL_TICK(triosc_tick);
    // sawosc
    CK_DLL_CTOR(sawosc_ctor);
    CK_DLL_CTRL(sawosc_ctrl_width);
    // sqrosc
    CK_DLL_CTOR(sqrosc_ctor);
    CK_DLL_CTRL(sqrosc_ctrl_width);
    //-----------------------------------------------------------------------------
    // file: ugen_genX
    // desc: thought it would be a good way to learn the fascinating innards of
    //       ChucK by porting some of the classic lookup table functions and adding
    //       a few new ones that might be of use.
    //       mostly ported from RTcmix (all by WarpTable)
    //
    // author: Dan Trueman (dtrueman.princeton.edu)
    // date: Winter 2007
    //-----------------------------------------------------------------------------
    // called by xxx_query
    // genX - base
    CK_DLL_CTOR(genX_ctor);
    CK_DLL_DTOR(genX_dtor);
    CK_DLL_TICK(genX_tick);
    CK_DLL_PMSG(genX_pmsg);
    CK_DLL_CTRL(genX_lookup);
    CK_DLL_CTRL(genX_coeffs);
    CK_DLL_CTRL(gen5_coeffs);
    CK_DLL_CTRL(gen7_coeffs);
    CK_DLL_CTRL(gen9_coeffs);
    CK_DLL_CTRL(gen10_coeffs);
    CK_DLL_CTRL(gen17_coeffs);
    CK_DLL_CTRL(curve_coeffs);
    CK_DLL_CTRL(warp_coeffs);
    _asymwarp(inval: t_CKDOUBLE, k: t_CKDOUBLE);
    _symwarp(inval: t_CKDOUBLE, k: t_CKDOUBLE);
}
