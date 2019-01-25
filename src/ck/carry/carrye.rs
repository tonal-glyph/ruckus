#![feature(libc)]
///* desc: carrier of things associated with each ChucK instance
extern crate libc;
use libc::*;
use crate::util_thread_h_edited::*;
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes
)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ChucK {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Compiler {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Env {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_IO_Chout {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_IO_Cherr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_socket_ {
    _unused: [u8; 0],
}
pub type ck_socket = *mut ck_socket_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WvOut {
    _unused: [u8; 0],
}
//! From SE: https://codereview.stackexchange.com/questions/146339/btreemap-as-a-multimap-in-rust
use std::collections::BTreeMap;
use std::io::{self, BufRead};
pub fn insert_dup<V>(map: &mut BTreeMap<i32, Vec<V>>, k: i32, v: V) {
    map.entry(k).or_insert_with(Vec::new).push(v)
}

pub fn remove_dup_internal<V>(map: &mut BTreeMap<i32, Vec<V>>, k: i32) -> Option<usize> {
    map.get_mut(&k).map(|vec| {
        vec.pop();
        vec.len()
    })
}
//! From SE: https://codereview.stackexchange.com/questions/127350/idiomatic-word-counting-in-rust/127381#127381
// pub fn wordcount() {
//     let mut counts = BTreeMap::new();
//     let stdin = io::stdin();
//     for line in stdin.lock().lines() {
//         let line = line.expect("Error parsing stdin");
//         for word in line.split_whitespace().map(str::to_lowercase) {
//             *counts.entry(word).or_insert(0usize) += 1;
//         }
//     }
//     for (word, count) in counts.iter() {
//         println!("{} {}", word, count);
//     }
// }
#[repr(C)]
pub struct Chuck_Carrier {
    chuck: *mut ChucK,
    compiler: *mut Chuck_Compiler,
    env: *mut Chuck_Env,
    vm: *mut Chuck_VM,
    chout: *mut Chuck_IO_Chout,
    cherr: *mut Chuck_IO_Cherr,
    otf_socket: ck_socket,
    otf_port: c_long,
    otf_thread: crate::util_thread_h_edited::pthread_t,
    stk_writeThread: *mut crate::util_thread_h_edited::XWriteThread,
    stk_wvOutMap: HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}hintIsRealtimeAudio"]
    fn Chuck_Carrier_hintIsRealtimeAudio(this: *mut Chuck_Carrier) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Carrier"]
    fn Chuck_Carrier_Chuck_Carrier(this: *mut Chuck_Carrier);
}
impl Chuck_Carrier {
    #[inline]
    unsafe fn hintIsRealtimeAudio(&mut self) -> c_ulong {
        Chuck_Carrier_hintIsRealtimeAudio(self)
    }
    #[inline]
    unsafe fn new() -> Self {
        let mut __bindgen_tmp = std::mem::MaybeUninit::uninitialized();
        Chuck_Carrier_Chuck_Carrier(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
