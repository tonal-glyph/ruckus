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
use crate::dts::*;
///* string utility functions
use libc::*;
extern "C" {
    #[link_name = "\u{1}_Z4itoaB5cxx11l"]
    pub fn itoa(val: c_long) -> crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}_Z4ftoaB5cxx11dm"]
    pub fn ftoa(val: f64, precision: c_ulong) -> crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}_Z7tolowerRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn tolower1(val: *const crate::dts::string) -> crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}_Z7toupperRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn toupper1(val: *const crate::dts::string) -> crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}_Z4trimRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn trim(val: *const crate::dts::string) -> crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}_Z5ltrimRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn ltrim(val: *const crate::dts::string) -> crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}_Z5rtrimRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn rtrim(val: *const crate::dts::string) -> crate::dts::string;
}
///* argument extraction
extern "C" {
    #[link_name = "\u{1}_Z12extract_argsRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEERS4_RSt6vectorIS4_SaIS4_EE"]
    pub fn extract_args(token: *const crate::dts::string, filename: *mut crate::dts::string, args: *mut vector) -> c_ulong;
}
///* take existing path, and attempt to dir up
extern "C" {
    #[link_name = "\u{1}_Z9dir_go_upRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEl"]
    pub fn dir_go_up(dir: *const crate::dts::string, numUp: c_long) -> crate::dts::string;
}
///* get full path to file
extern "C" {
    #[link_name = "\u{1}_Z13get_full_pathRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn get_full_path(fp: *const crate::dts::string) -> crate::dts::string;
}
///* currently just expands ~ to HOME and ~user to user's home directory
extern "C" {
    #[link_name = "\u{1}_Z15expand_filepathRNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn expand_filepath(fp: *mut crate::dts::string) -> crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}_Z20extract_filepath_dirRNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn extract_filepath_dir(filepath: *mut crate::dts::string) -> crate::dts::string;
}
///* convert \ to / (on Windows)
extern "C" {
    #[link_name = "\u{1}_Z29normalize_directory_separatorRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
    pub fn normalize_directory_separator(filepath: *const crate::dts::string) -> crate::dts::string;
}
///* determine if the last characters of str match end exactly, e.g. to test file extension
extern "C" {
    #[link_name = "\u{1}_Z10str_endsinPKcS0_"]
    pub fn str_endsin(str: *const c_char, end: *const c_char) -> c_int;
}
///*-----------------------------------------------------------------------------
///* name: parse_path_list()
///* desc: split "x:y:z"-style path list into {"x","y","z"}
///*-----------------------------------------------------------------------------
extern "C" {
    #[link_name = "\u{1}_Z15parse_path_listRNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEERNS_4listIS4_SaIS4_EEE"]
    pub fn parse_path_list(str: *mut crate::dts::string, lst: *mut crate::dts::list);
}
