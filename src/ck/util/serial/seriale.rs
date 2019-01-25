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
///* utility for serial I/O
use libc::*;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SerialIOManager {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN15SerialIOManager24s_availableSerialDevicesB5cxx11E"]
    pub static mut SerialIOManager_s_availableSerialDevices: vector<crate::dts::string>;
}
extern "C" {
    #[link_name = "\u{1}_ZN15SerialIOManager22availableSerialDevicesB5cxx11Ev"]
    pub fn SerialIOManager_availableSerialDevices() -> vector;
}
impl SerialIOManager {
    #[inline]
    pub unsafe fn availableSerialDevices() -> vector {
        SerialIOManager_availableSerialDevices()
    }
}
