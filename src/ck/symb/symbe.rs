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
///* symbols in the syntax, adapted from Tiger compiler by Andrew Appel
#![feature(libc)]
use libc::*;
use crate::ck::util::utile::*;
use crate::dts::*;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S_Symbol_ {
    _unused: [u8; 0],
}
pub type S_Symbol = *mut S_Symbol_;
/* Make a unique symbol from a given c_str.
 *  Different calls to make_symbol("foo") will yield the same S_Symbol
 *  value, even if the "foo" c_str are at different locations. */
extern "C" {
    pub fn insert_symbol(arg1: c_constr) -> S_Symbol;
}
/* Extract the underlying c_str from a symbol */
extern "C" {
    pub fn S_name(arg1: S_Symbol) -> c_str;
}
/* S_table is a mapping from S_Symbol->any, where "any" is represented
 *     here by void*  */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TAB_table_ {
    _unused: [u8; 0],
}
pub type S_table = *mut TAB_table_;
/* Make a new table */
extern "C" {
    pub fn S_empty() -> S_table;
}
extern "C" {
    pub fn S_empty2(size: c_uint) -> S_table;
}
/* Enter a binding "sym->value" into "t", shadowing but not deleting
 *    any previous binding of "sym". */
extern "C" {
    pub fn S_enter(t: S_table, sym: S_Symbol, value: *mut c_void);
}
extern "C" {
    pub fn S_enter2(t: S_table, str: c_constr, value: *mut c_void);
}
/* Look up the most recent binding of "sym" in "t", or return NULL
 *    if sym is unbound. */
extern "C" {
    pub fn S_look(t: S_table, sym: S_Symbol) -> *mut c_void;
}
extern "C" {
    pub fn S_look2(t: S_table, str: c_constr) -> *mut c_void;
}
/* Start a new "scope" in "t".  Scopes are nested. */
extern "C" {
    pub fn S_beginScope(t: S_table);
}
/* Remove any bindings entered since the current scope began,
and end the current scope. */
extern "C" {
    pub fn S_endScope(t: S_table);
}
extern "C" {
    pub fn S_pop(t: S_table);
}
