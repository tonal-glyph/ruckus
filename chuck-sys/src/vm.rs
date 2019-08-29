use bitflags::*;

// bitflags
// #define CVM_MEM_STACK_SIZE          (0x1 << 16)
// #define CVM_REG_STACK_SIZE          (0x1 << 14)
bitflags!{
    
}
// forward references
// struct Chuck_Instr;
// struct Chuck_VM;
// struct Chuck_VM_Func;
// struct Chuck_VM_FTable;
// struct Chuck_Msg;
// struct Chuck_IO_Serial;
// pub struct CBufferSimple;

// Forward references for global messages, storage
// struct Chuck_Set_Global_Int_Request;
// struct Chuck_Get_Global_Int_Request;
// struct Chuck_Set_Global_Float_Request;
// struct Chuck_Get_Global_Float_Request;
// struct Chuck_Signal_Global_Event_Request;
// struct Chuck_Global_Int_Container;
// struct Chuck_Global_Float_Container;
// struct Chuck_Global_Event_Container;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
enum Chuck_Global_Request_Type {
    set_global_int_request,
    get_global_int_request,
    set_global_float_request,
    get_global_float_request,
    signal_global_event_request,
    spork_shred_request,
}
impl Chuck_Global_Request_Type {
    pub const VARIANTS: [Chuck_Global_Request_Type; 6] = [
        Chuck_Global_Request_Type::set_global_int_request,
        Chuck_Global_Request_Type::get_global_int_request,
        Chuck_Global_Request_Type::set_global_float_request,
        Chuck_Global_Request_Type::get_global_float_request,
        Chuck_Global_Request_Type::signal_global_event_request,
        Chuck_Global_Request_Type::spork_shred_request,
    ];
}
/// chuck_vm.h
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
enum Chuck_Msg_Type {
    MSG_ADD = 1,
    MSG_REMOVE,
    MSG_REMOVEALL,
    MSG_REPLACE,
    MSG_STATUS,
    MSG_PAUSE,
    MSG_KILL,
    MSG_TIME,
    MSG_RESET_ID,
    MSG_DONE,
    MSG_ABORT,
    MSG_ERROR, // added 1.3.0.0
    MSG_CLEARVM,
}
impl Chuck_Msg_Type {
    pub const VARIANTS: [Chuck_Msg_Type; 13] = [
        Chuck_Msg_Type::MSG_ADD,
        Chuck_Msg_Type::MSG_REMOVE,
        Chuck_Msg_Type::MSG_REMOVEALL,
        Chuck_Msg_Type::MSG_REPLACE,
        Chuck_Msg_Type::MSG_STATUS,
        Chuck_Msg_Type::MSG_PAUSE,
        Chuck_Msg_Type::MSG_KILL,
        Chuck_Msg_Type::MSG_TIME,
        Chuck_Msg_Type::MSG_RESET_ID,
        Chuck_Msg_Type::MSG_DONE,
        Chuck_Msg_Type::MSG_ABORT,
        Chuck_Msg_Type::MSG_ERROR,
        Chuck_Msg_Type::MSG_CLEARVM,
    ];
}
pub struct Chuck_VM {

}
pub struct Chuck_VM_Code {

}
pub struct Chuck_VM_Object {

}
pub struct Chuck_VM_Shred {

}
