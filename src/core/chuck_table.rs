/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TAB_table_ {
    _unused: [u8; 0],
}
pub type TAB_table = *mut TAB_table_;
pub type TAB_eq_func = ::std::option::Option<
    unsafe extern "C" fn(lhs: *mut ::std::os::raw::c_void, rhs: *mut ::std::os::raw::c_void)
        -> ::std::os::raw::c_long,
>;
pub type TAB_hash_func = ::std::option::Option<
    unsafe extern "C" fn(key: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long,
>;
extern "C" {
    #[link_name = "\u{1}_Z9TAB_emptyv"]
    pub fn TAB_empty() -> TAB_table;
}
extern "C" {
    #[link_name = "\u{1}_Z10TAB_empty2m"]
    pub fn TAB_empty2(size: ::std::os::raw::c_ulong) -> TAB_table;
}
extern "C" {
    #[link_name = "\u{1}_Z10TAB_empty3PFlPvS_EPFlS_Ej"]
    pub fn TAB_empty3(
        eq: TAB_eq_func,
        hash: TAB_hash_func,
        size: ::std::os::raw::c_uint,
    ) -> TAB_table;
}
extern "C" {
    #[link_name = "\u{1}_Z10TAB_deleteP10TAB_table_"]
    pub fn TAB_delete(t: TAB_table);
}
extern "C" {
    #[link_name = "\u{1}_Z9TAB_enterP10TAB_table_PvS1_"]
    pub fn TAB_enter(
        t: TAB_table,
        key: *mut ::std::os::raw::c_void,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[link_name = "\u{1}_Z8TAB_lookP10TAB_table_Pv"]
    pub fn TAB_look(t: TAB_table, key: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}_Z7TAB_popP10TAB_table_"]
    pub fn TAB_pop(t: TAB_table) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}_Z8TAB_topvP10TAB_table_"]
    pub fn TAB_topv(t: TAB_table) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}_Z8TAB_dumpP10TAB_table_PFvPvS1_E"]
    pub fn TAB_dump(
        t: TAB_table,
        show: ::std::option::Option<
            unsafe extern "C" fn(
                key: *mut ::std::os::raw::c_void,
                value: *mut ::std::os::raw::c_void,
            ),
        >,
    );
}
extern "C" {
    #[link_name = "\u{1}_Z6str_eqPvS_"]
    pub fn str_eq(
        lhs: *mut ::std::os::raw::c_void,
        rhs: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}_Z8str_hashPv"]
    pub fn str_hash(key: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_long;
}
