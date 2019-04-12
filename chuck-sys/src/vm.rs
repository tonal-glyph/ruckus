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
