/// util_opsc.h
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
enum osc_datatype {
    OSC_UNTYPED,
    OSC_NOARGS,
    OSC_INT,
    OSC_FLOAT,
    OSC_STRING,
    OSC_BLOB,
    OSC_NTYPE
}
/// util_opsc.cpp
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
enum udp_stat {
    UDP_NOINIT,
    UDP_UNBOUND,
    UDP_BOUND,
    UDP_READY,
    UDP_ERROR,
    UDP_NUM
}
