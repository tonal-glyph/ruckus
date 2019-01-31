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
///* No algorithm should use these functions directly, because
///* programming with void* is too error-prone.  Instead,
///* each module should make "wrapper" functions that take
///* well-typed arguments and call the TAB_ functions.
#![feature(libc)]
use libc::*;
use crate::ck::util::utile::*;
use crate::dts::*;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TAB_table_ {
    _unused: [u8; 0],
}
pub type TAB_table = *mut TAB_table_;
pub type TAB_eq_func =
    Option<unsafe extern "C" fn(lhs: *mut c_void, rhs: *mut c_void) -> c_long>;
pub type TAB_hash_func = Option<unsafe extern "C" fn(key: *mut c_void) -> c_long>;
/* Make a new table mapping "keys" to "values". */
extern "C" {
    #[link_name = "\u{1}_Z9TAB_emptyv"]
    pub fn TAB_empty() -> TAB_table;
}
extern "C" {
    #[link_name = "\u{1}_Z10TAB_empty2m"]
    pub fn TAB_empty2(size: c_ulong) -> TAB_table;
}
extern "C" {
    #[link_name = "\u{1}_Z10TAB_empty3PFlPvS_EPFlS_Ej"]
    pub fn TAB_empty3(eq: TAB_eq_func, hash: TAB_hash_func, size: c_uint) -> TAB_table;
}
extern "C" {
    #[link_name = "\u{1}_Z10TAB_deleteP10TAB_table_"]
    pub fn TAB_delete(t: TAB_table);
}
/* Enter the mapping "key"->"value" into table "t",
 *    shadowing but not destroying any previous binding for "key". */
extern "C" {
    #[link_name = "\u{1}_Z9TAB_enterP10TAB_table_PvS1_"]
    pub fn TAB_enter(t: TAB_table, key: *mut c_void, value: *mut c_void);
}
/* Look up the most recent binding for "key" in table "t" */
extern "C" {
    #[link_name = "\u{1}_Z8TAB_lookP10TAB_table_Pv"]
    pub fn TAB_look(t: TAB_table, key: *mut c_void) -> *mut c_void;
}
/* Pop the most recent binding and return its key.
 * This may expose another binding for the same key, if there was one. */
extern "C" {
    #[link_name = "\u{1}_Z7TAB_popP10TAB_table_"]
    pub fn TAB_pop(t: TAB_table) -> *mut c_void;
}
extern "C" {
    #[link_name = "\u{1}_Z8TAB_topvP10TAB_table_"]
    pub fn TAB_topv(t: TAB_table) -> *mut c_void;
}
/* Call "show" on every "key"->"value" pair in the table,
 *  including shadowed bindings, in order from the most
 *  recent binding of any key to the oldest binding in the table */
extern "C" {
    #[link_name = "\u{1}_Z8TAB_dumpP10TAB_table_PFvPvS1_E"]
    pub fn TAB_dump(
        t: TAB_table,
        show: Option<unsafe extern "C" fn(key: *mut c_void, value: *mut c_void)>,
    );
}
/* str eq function */
extern "C" {
    #[link_name = "\u{1}_Z6str_eqPvS_"]
    pub fn str_eq(lhs: *mut c_void, rhs: *mut c_void) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}_Z8str_hashPv"]
    pub fn str_hash(key: *mut c_void) -> c_long;
}
