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
use crate::chuck_vm_h_edited::*;
use crate::dts::*;
use crate::util_thread_h_edited::*;
///* Statistics for audicle. This file was largely written by hand, as bindgen didn't work.
use libc::*;
pub struct Shred_Activation {
    when: t_CKTIME,
    cycles: t_CKUINT,
}
// impl Shred_Activation {
//     pub fn proc(a: t_CKTIME, b: t_CKUINT) {} -> Shred_Activation<when: a, cycles: b>;
// }
pub struct Shred_Stat {
    cycles: t_CKUINT,
    xid: t_CKUINT,
    parent: t_CKUINT,
    state: t_CKUINT,
    ///* reference (could be pointing to garbage, see state)
    shred_ref: *mut Chuck_VM_Shred,
    activations: t_CKUINT,
    average_ctrl: t_CKFLOAT,
    average_cycles: t_CKFLOAT,
    spork_time: t_CKTIME,
    active_time: t_CKTIME,
    wake_time: t_CKTIME,
    free_time: t_CKTIME,
    name: string,
    owner: string,
    source: string,
    diffs: std::collections::queue<t_CKDUR>,
    num_diffs: t_CKUINT,
    diff_total: t_CKDUR,
    act_cycles: std::collections::queue<t_CKUINT>,
    act_cycles_total: t_CKUINT,
    last_cycles: t_CKUINT,
    children: std::collections::Vec<Shred_Stat>,
    activationss: std::collections::Vec<Shred_Activation>,
    mutex: XMutex,
    data: Shred_Data,
    time: Shred_Time,
}
impl Shred_Stat {
    pub fn get_sporked(s: Shred_Stat::children) {
        println!("chuck sporks: {}", s)
    }
    pub fn get_activations(s: Shred_Stat::activationss) {
        println!("chuck sporks: {}", s)
    }
    pub fn clear() {
        Shred_Stat::xid = 0;
        Shred_Stat::parent = 0;
        Shred_Stat::state = 0;
        Shred_Stat::cycles = 0;
        Shred_Stat::activations = 0;
        Shred_Stat::average_ctrl = 0.0;
        Shred_Stat::spork_time = 0.0;
        Shred_Stat::active_time = 0.0;
        Shred_Stat::wake_time = 0.0;
        Shred_Stat::free_time = 0.0;
        Shred_Stat::name = "no name";
        Shred_Stat::owner = "none";
        Shred_Stat::source = "nowhere";
        while self.diffs.len() > 0 {
            self.diffs.pop();
            Shred_Stat::diff_total = 0.0;
        }
        while self.act_cycles.len() > 0 {
            self.act_cycles.pop();
            Shred_Stat::act_cycles_total = 0;
        }
        Shred_Stat::last_cycles = 0;
        Shred_Stat::children.clear();
    }
}
pub struct Chuck_Stats {
    vm: Chuck_VM,
    shreds: std::collections::HashMap<t_CKUINT, *mut Shred_Stat>,
    done: std::collections::Vec<*mut Shred_Stat>,
    mutex: XMutex,
    our_instance: *mut Chuck_Stats,
    activations_yes: t_CKBOOL,
}
impl Chuck_Stats {
    pub fn get_shred(xid: t_CKUINT) {
        Chuck_Stats::mutex.acquire();
        let s: Shred_Stat = self.shreds[xid];
        Chuck_Stats::mutex.release();
        return s;
    }
}
// pub fn get_shreds(xid: vector<*mut Shred_Stat>, shreds: map<(*mut Shred_Stat, *mut Shred_Stat)>);
// pub fn set_vm_ref(_vm: Chuck_Stats::vm);
// pub fn add_shred(shred: *mut Chuck_VM_Shred);
// pub fn activate_shred(shred: *mut Chuck_VM_Shred);
// pub fn advance_time(shred: *mut Chuck_VM_Shred, to: t_CKTIME);
// pub fn deactivate_shred(shred: *mut Chuck_VM_Shred);
// pub fn remove_shred(shred: *mut Chuck_VM_Shred);
