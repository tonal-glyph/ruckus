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
use crate::sys::*;
///* sockets
use libc::*;
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> Copy for __IncompleteArrayField<T> {}
#[allow(unused_imports)]
pub const _SYS_TYPES_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 28;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __USE_EXTERN_INLINES: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _SYS_SOCKET_H: u32 = 1;
pub const __iovec_defined: u32 = 1;
pub const PF_UNSPEC: u32 = 0;
pub const PF_LOCAL: u32 = 1;
pub const PF_UNIX: u32 = 1;
pub const PF_FILE: u32 = 1;
pub const PF_INET: u32 = 2;
pub const PF_AX25: u32 = 3;
pub const PF_IPX: u32 = 4;
pub const PF_APPLETALK: u32 = 5;
pub const PF_NETROM: u32 = 6;
pub const PF_BRIDGE: u32 = 7;
pub const PF_ATMPVC: u32 = 8;
pub const PF_X25: u32 = 9;
pub const PF_INET6: u32 = 10;
pub const PF_ROSE: u32 = 11;
pub const PF_DECnet: u32 = 12;
pub const PF_NETBEUI: u32 = 13;
pub const PF_SECURITY: u32 = 14;
pub const PF_KEY: u32 = 15;
pub const PF_NETLINK: u32 = 16;
pub const PF_ROUTE: u32 = 16;
pub const PF_PACKET: u32 = 17;
pub const PF_ASH: u32 = 18;
pub const PF_ECONET: u32 = 19;
pub const PF_ATMSVC: u32 = 20;
pub const PF_RDS: u32 = 21;
pub const PF_SNA: u32 = 22;
pub const PF_IRDA: u32 = 23;
pub const PF_PPPOX: u32 = 24;
pub const PF_WANPIPE: u32 = 25;
pub const PF_LLC: u32 = 26;
pub const PF_IB: u32 = 27;
pub const PF_MPLS: u32 = 28;
pub const PF_CAN: u32 = 29;
pub const PF_TIPC: u32 = 30;
pub const PF_BLUETOOTH: u32 = 31;
pub const PF_IUCV: u32 = 32;
pub const PF_RXRPC: u32 = 33;
pub const PF_ISDN: u32 = 34;
pub const PF_PHONET: u32 = 35;
pub const PF_IEEE802154: u32 = 36;
pub const PF_CAIF: u32 = 37;
pub const PF_ALG: u32 = 38;
pub const PF_NFC: u32 = 39;
pub const PF_VSOCK: u32 = 40;
pub const PF_KCM: u32 = 41;
pub const PF_QIPCRTR: u32 = 42;
pub const PF_SMC: u32 = 43;
pub const PF_MAX: u32 = 44;
pub const AF_UNSPEC: u32 = 0;
pub const AF_LOCAL: u32 = 1;
pub const AF_UNIX: u32 = 1;
pub const AF_FILE: u32 = 1;
pub const AF_INET: u32 = 2;
pub const AF_AX25: u32 = 3;
pub const AF_IPX: u32 = 4;
pub const AF_APPLETALK: u32 = 5;
pub const AF_NETROM: u32 = 6;
pub const AF_BRIDGE: u32 = 7;
pub const AF_ATMPVC: u32 = 8;
pub const AF_X25: u32 = 9;
pub const AF_INET6: u32 = 10;
pub const AF_ROSE: u32 = 11;
pub const AF_DECnet: u32 = 12;
pub const AF_NETBEUI: u32 = 13;
pub const AF_SECURITY: u32 = 14;
pub const AF_KEY: u32 = 15;
pub const AF_NETLINK: u32 = 16;
pub const AF_ROUTE: u32 = 16;
pub const AF_PACKET: u32 = 17;
pub const AF_ASH: u32 = 18;
pub const AF_ECONET: u32 = 19;
pub const AF_ATMSVC: u32 = 20;
pub const AF_RDS: u32 = 21;
pub const AF_SNA: u32 = 22;
pub const AF_IRDA: u32 = 23;
pub const AF_PPPOX: u32 = 24;
pub const AF_WANPIPE: u32 = 25;
pub const AF_LLC: u32 = 26;
pub const AF_IB: u32 = 27;
pub const AF_MPLS: u32 = 28;
pub const AF_CAN: u32 = 29;
pub const AF_TIPC: u32 = 30;
pub const AF_BLUETOOTH: u32 = 31;
pub const AF_IUCV: u32 = 32;
pub const AF_RXRPC: u32 = 33;
pub const AF_ISDN: u32 = 34;
pub const AF_PHONET: u32 = 35;
pub const AF_IEEE802154: u32 = 36;
pub const AF_CAIF: u32 = 37;
pub const AF_ALG: u32 = 38;
pub const AF_NFC: u32 = 39;
pub const AF_VSOCK: u32 = 40;
pub const AF_KCM: u32 = 41;
pub const AF_QIPCRTR: u32 = 42;
pub const AF_SMC: u32 = 43;
pub const AF_MAX: u32 = 44;
pub const SOL_RAW: u32 = 255;
pub const SOL_DECNET: u32 = 261;
pub const SOL_X25: u32 = 262;
pub const SOL_PACKET: u32 = 263;
pub const SOL_ATM: u32 = 264;
pub const SOL_AAL: u32 = 265;
pub const SOL_IRDA: u32 = 266;
pub const SOL_NETBEUI: u32 = 267;
pub const SOL_LLC: u32 = 268;
pub const SOL_DCCP: u32 = 269;
pub const SOL_NETLINK: u32 = 270;
pub const SOL_TIPC: u32 = 271;
pub const SOL_RXRPC: u32 = 272;
pub const SOL_PPPOL2TP: u32 = 273;
pub const SOL_BLUETOOTH: u32 = 274;
pub const SOL_PNPIPE: u32 = 275;
pub const SOL_RDS: u32 = 276;
pub const SOL_IUCV: u32 = 277;
pub const SOL_CAIF: u32 = 278;
pub const SOL_ALG: u32 = 279;
pub const SOL_NFC: u32 = 280;
pub const SOL_KCM: u32 = 281;
pub const SOL_TLS: u32 = 282;
pub const SOMAXCONN: u32 = 128;
pub const _BITS_SOCKADDR_H: u32 = 1;
pub const _SS_SIZE: u32 = 128;
pub const FIOSETOWN: u32 = 35073;
pub const SIOCSPGRP: u32 = 35074;
pub const FIOGETOWN: u32 = 35075;
pub const SIOCGPGRP: u32 = 35076;
pub const SIOCATMARK: u32 = 35077;
pub const SIOCGSTAMP: u32 = 35078;
pub const SIOCGSTAMPNS: u32 = 35079;
pub const SOL_SOCKET: u32 = 1;
pub const SO_DEBUG: u32 = 1;
pub const SO_REUSEADDR: u32 = 2;
pub const SO_TYPE: u32 = 3;
pub const SO_ERROR: u32 = 4;
pub const SO_DONTROUTE: u32 = 5;
pub const SO_BROADCAST: u32 = 6;
pub const SO_SNDBUF: u32 = 7;
pub const SO_RCVBUF: u32 = 8;
pub const SO_SNDBUFFORCE: u32 = 32;
pub const SO_RCVBUFFORCE: u32 = 33;
pub const SO_KEEPALIVE: u32 = 9;
pub const SO_OOBINLINE: u32 = 10;
pub const SO_NO_CHECK: u32 = 11;
pub const SO_PRIORITY: u32 = 12;
pub const SO_LINGER: u32 = 13;
pub const SO_BSDCOMPAT: u32 = 14;
pub const SO_REUSEPORT: u32 = 15;
pub const SO_PASSCRED: u32 = 16;
pub const SO_PEERCRED: u32 = 17;
pub const SO_RCVLOWAT: u32 = 18;
pub const SO_SNDLOWAT: u32 = 19;
pub const SO_RCVTIMEO: u32 = 20;
pub const SO_SNDTIMEO: u32 = 21;
pub const SO_SECURITY_AUTHENTICATION: u32 = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: u32 = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: u32 = 24;
pub const SO_BINDTODEVICE: u32 = 25;
pub const SO_ATTACH_FILTER: u32 = 26;
pub const SO_DETACH_FILTER: u32 = 27;
pub const SO_GET_FILTER: u32 = 26;
pub const SO_PEERNAME: u32 = 28;
pub const SO_TIMESTAMP: u32 = 29;
pub const SCM_TIMESTAMP: u32 = 29;
pub const SO_ACCEPTCONN: u32 = 30;
pub const SO_PEERSEC: u32 = 31;
pub const SO_PASSSEC: u32 = 34;
pub const SO_TIMESTAMPNS: u32 = 35;
pub const SCM_TIMESTAMPNS: u32 = 35;
pub const SO_MARK: u32 = 36;
pub const SO_TIMESTAMPING: u32 = 37;
pub const SCM_TIMESTAMPING: u32 = 37;
pub const SO_PROTOCOL: u32 = 38;
pub const SO_DOMAIN: u32 = 39;
pub const SO_RXQ_OVFL: u32 = 40;
pub const SO_WIFI_STATUS: u32 = 41;
pub const SCM_WIFI_STATUS: u32 = 41;
pub const SO_PEEK_OFF: u32 = 42;
pub const SO_NOFCS: u32 = 43;
pub const SO_LOCK_FILTER: u32 = 44;
pub const SO_SELECT_ERR_QUEUE: u32 = 45;
pub const SO_BUSY_POLL: u32 = 46;
pub const SO_MAX_PACING_RATE: u32 = 47;
pub const SO_BPF_EXTENSIONS: u32 = 48;
pub const SO_INCOMING_CPU: u32 = 49;
pub const SO_ATTACH_BPF: u32 = 50;
pub const SO_DETACH_BPF: u32 = 27;
pub const SO_ATTACH_REUSEPORT_CBPF: u32 = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: u32 = 52;
pub const SO_CNX_ADVICE: u32 = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: u32 = 54;
pub const SO_MEMINFO: u32 = 55;
pub const SO_INCOMING_NAPI_ID: u32 = 56;
pub const SO_COOKIE: u32 = 57;
pub const SCM_TIMESTAMPING_PKTINFO: u32 = 58;
pub const SO_PEERGROUPS: u32 = 59;
pub const SO_ZEROCOPY: u32 = 60;
pub const _NETINET_IN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const __USE_KERNEL_IPV6_DEFS: u32 = 0;
pub const IP_OPTIONS: u32 = 4;
pub const IP_HDRINCL: u32 = 3;
pub const IP_TOS: u32 = 1;
pub const IP_TTL: u32 = 2;
pub const IP_RECVOPTS: u32 = 6;
pub const IP_RETOPTS: u32 = 7;
pub const IP_MULTICAST_IF: u32 = 32;
pub const IP_MULTICAST_TTL: u32 = 33;
pub const IP_MULTICAST_LOOP: u32 = 34;
pub const IP_ADD_MEMBERSHIP: u32 = 35;
pub const IP_DROP_MEMBERSHIP: u32 = 36;
pub const IP_UNBLOCK_SOURCE: u32 = 37;
pub const IP_BLOCK_SOURCE: u32 = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 40;
pub const IP_MSFILTER: u32 = 41;
pub const IP_ROUTER_ALERT: u32 = 5;
pub const IP_PKTINFO: u32 = 8;
pub const IP_PKTOPTIONS: u32 = 9;
pub const IP_PMTUDISC: u32 = 10;
pub const IP_MTU_DISCOVER: u32 = 10;
pub const IP_RECVERR: u32 = 11;
pub const IP_RECVTTL: u32 = 12;
pub const IP_RECVTOS: u32 = 13;
pub const IP_MTU: u32 = 14;
pub const IP_FREEBIND: u32 = 15;
pub const IP_IPSEC_POLICY: u32 = 16;
pub const IP_XFRM_POLICY: u32 = 17;
pub const IP_PASSSEC: u32 = 18;
pub const IP_TRANSPARENT: u32 = 19;
pub const IP_MULTICAST_ALL: u32 = 49;
pub const IP_ORIGDSTADDR: u32 = 20;
pub const IP_RECVORIGDSTADDR: u32 = 20;
pub const IP_MINTTL: u32 = 21;
pub const IP_NODEFRAG: u32 = 22;
pub const IP_CHECKSUM: u32 = 23;
pub const IP_BIND_ADDRESS_NO_PORT: u32 = 24;
pub const IP_RECVFRAGSIZE: u32 = 25;
pub const IP_PMTUDISC_DONT: u32 = 0;
pub const IP_PMTUDISC_WANT: u32 = 1;
pub const IP_PMTUDISC_DO: u32 = 2;
pub const IP_PMTUDISC_PROBE: u32 = 3;
pub const IP_PMTUDISC_INTERFACE: u32 = 4;
pub const IP_PMTUDISC_OMIT: u32 = 5;
pub const IP_UNICAST_IF: u32 = 50;
pub const SOL_IP: u32 = 0;
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1;
pub const IP_MAX_MEMBERSHIPS: u32 = 20;
pub const IPV6_ADDRFORM: u32 = 1;
pub const IPV6_2292PKTINFO: u32 = 2;
pub const IPV6_2292HOPOPTS: u32 = 3;
pub const IPV6_2292DSTOPTS: u32 = 4;
pub const IPV6_2292RTHDR: u32 = 5;
pub const IPV6_2292PKTOPTIONS: u32 = 6;
pub const IPV6_CHECKSUM: u32 = 7;
pub const IPV6_2292HOPLIMIT: u32 = 8;
pub const IPV6_NEXTHOP: u32 = 9;
pub const IPV6_AUTHHDR: u32 = 10;
pub const IPV6_UNICAST_HOPS: u32 = 16;
pub const IPV6_MULTICAST_IF: u32 = 17;
pub const IPV6_MULTICAST_HOPS: u32 = 18;
pub const IPV6_MULTICAST_LOOP: u32 = 19;
pub const IPV6_JOIN_GROUP: u32 = 20;
pub const IPV6_LEAVE_GROUP: u32 = 21;
pub const IPV6_ROUTER_ALERT: u32 = 22;
pub const IPV6_MTU_DISCOVER: u32 = 23;
pub const IPV6_MTU: u32 = 24;
pub const IPV6_RECVERR: u32 = 25;
pub const IPV6_V6ONLY: u32 = 26;
pub const IPV6_JOIN_ANYCAST: u32 = 27;
pub const IPV6_LEAVE_ANYCAST: u32 = 28;
pub const IPV6_IPSEC_POLICY: u32 = 34;
pub const IPV6_XFRM_POLICY: u32 = 35;
pub const IPV6_HDRINCL: u32 = 36;
pub const IPV6_RECVPKTINFO: u32 = 49;
pub const IPV6_PKTINFO: u32 = 50;
pub const IPV6_RECVHOPLIMIT: u32 = 51;
pub const IPV6_HOPLIMIT: u32 = 52;
pub const IPV6_RECVHOPOPTS: u32 = 53;
pub const IPV6_HOPOPTS: u32 = 54;
pub const IPV6_RTHDRDSTOPTS: u32 = 55;
pub const IPV6_RECVRTHDR: u32 = 56;
pub const IPV6_RTHDR: u32 = 57;
pub const IPV6_RECVDSTOPTS: u32 = 58;
pub const IPV6_DSTOPTS: u32 = 59;
pub const IPV6_RECVPATHMTU: u32 = 60;
pub const IPV6_PATHMTU: u32 = 61;
pub const IPV6_DONTFRAG: u32 = 62;
pub const IPV6_RECVTCLASS: u32 = 66;
pub const IPV6_TCLASS: u32 = 67;
pub const IPV6_AUTOFLOWLABEL: u32 = 70;
pub const IPV6_ADDR_PREFERENCES: u32 = 72;
pub const IPV6_MINHOPCOUNT: u32 = 73;
pub const IPV6_ORIGDSTADDR: u32 = 74;
pub const IPV6_RECVORIGDSTADDR: u32 = 74;
pub const IPV6_TRANSPARENT: u32 = 75;
pub const IPV6_UNICAST_IF: u32 = 76;
pub const IPV6_RECVFRAGSIZE: u32 = 77;
pub const IPV6_FREEBIND: u32 = 78;
pub const IPV6_ADD_MEMBERSHIP: u32 = 20;
pub const IPV6_DROP_MEMBERSHIP: u32 = 21;
pub const IPV6_RXHOPOPTS: u32 = 54;
pub const IPV6_RXDSTOPTS: u32 = 59;
pub const IPV6_PMTUDISC_DONT: u32 = 0;
pub const IPV6_PMTUDISC_WANT: u32 = 1;
pub const IPV6_PMTUDISC_DO: u32 = 2;
pub const IPV6_PMTUDISC_PROBE: u32 = 3;
pub const IPV6_PMTUDISC_INTERFACE: u32 = 4;
pub const IPV6_PMTUDISC_OMIT: u32 = 5;
pub const SOL_IPV6: u32 = 41;
pub const SOL_ICMPV6: u32 = 58;
pub const IPV6_RTHDR_LOOSE: u32 = 0;
pub const IPV6_RTHDR_STRICT: u32 = 1;
pub const IPV6_RTHDR_TYPE_0: u32 = 0;
pub const IN_CLASSA_NET: u32 = 4278190080;
pub const IN_CLASSA_NSHIFT: u32 = 24;
pub const IN_CLASSA_HOST: u32 = 16777215;
pub const IN_CLASSA_MAX: u32 = 128;
pub const IN_CLASSB_NET: u32 = 4294901760;
pub const IN_CLASSB_NSHIFT: u32 = 16;
pub const IN_CLASSB_HOST: u32 = 65535;
pub const IN_CLASSB_MAX: u32 = 65536;
pub const IN_CLASSC_NET: u32 = 4294967040;
pub const IN_CLASSC_NSHIFT: u32 = 8;
pub const IN_CLASSC_HOST: u32 = 255;
pub const IN_LOOPBACKNET: u32 = 127;
pub const INET_ADDRSTRLEN: u32 = 16;
pub const INET6_ADDRSTRLEN: u32 = 46;
pub const _ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _STDLIB_H: u32 = 1;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _MEMORY_H: u32 = 1;
pub const _STRING_H: u32 = 1;
pub const _ASSERT_H: u32 = 1;
pub const sz_VOID: u32 = 0;
pub const sz_WORD: u32 = 4;
pub const kindof_VOID: u32 = 0;
pub const kindof_INT: u32 = 1;
pub const kindof_FLOAT: u32 = 2;
pub const kindof_COMPLEX: u32 = 3;
pub const kindof_VEC3: u32 = 4;
pub const kindof_VEC4: u32 = 5;
pub const SILENCE: f64 = 0.0;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const ONE_PI: f64 = 3.141592653589793;
pub const TWO_PI: f64 = 6.283185307179586;
pub const SQRT2: f64 = 1.4142135623730951;
pub type __u_char = c_uchar;
pub type __u_short = c_ushort;
pub type __u_int = c_uint;
pub type __u_long = c_ulong;
pub type __int8_t = c_schar;
pub type __uint8_t = c_uchar;
pub type __int16_t = c_short;
pub type __uint16_t = c_ushort;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type __int64_t = c_long;
pub type __uint64_t = c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = c_long;
pub type __u_quad_t = c_ulong;
pub type __intmax_t = c_long;
pub type __uintmax_t = c_ulong;
pub type __dev_t = c_ulong;
pub type __uid_t = c_uint;
pub type __gid_t = c_uint;
pub type __ino_t = c_ulong;
pub type __ino64_t = c_ulong;
pub type __mode_t = c_uint;
pub type __nlink_t = c_ulong;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type __pid_t = c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [c_int; 2usize],
}
pub type __clock_t = c_long;
pub type __rlim_t = c_ulong;
pub type __rlim64_t = c_ulong;
pub type __id_t = c_uint;
pub type __time_t = c_long;
pub type __useconds_t = c_uint;
pub type __suseconds_t = c_long;
pub type __daddr_t = c_int;
pub type __key_t = c_int;
pub type __clockid_t = c_int;
pub type __timer_t = *mut c_void;
pub type __blksize_t = c_long;
pub type __blkcnt_t = c_long;
pub type __blkcnt64_t = c_long;
pub type __fsblkcnt_t = c_ulong;
pub type __fsblkcnt64_t = c_ulong;
pub type __fsfilcnt_t = c_ulong;
pub type __fsfilcnt64_t = c_ulong;
pub type __fsword_t = c_long;
pub type __ssize_t = c_long;
pub type __syscall_slong_t = c_long;
pub type __syscall_ulong_t = c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut c_char;
pub type __intptr_t = c_long;
pub type __socklen_t = c_uint;
pub type __sig_atomic_t = c_int;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type u_int8_t = c_uchar;
pub type u_int16_t = c_ushort;
pub type u_int32_t = c_uint;
pub type u_int64_t = c_ulong;
pub type register_t = c_long;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}
pub type socklen_t = __socklen_t;
pub const __socket_type_SOCK_STREAM: __socket_type = 1;
pub const __socket_type_SOCK_DGRAM: __socket_type = 2;
pub const __socket_type_SOCK_RAW: __socket_type = 3;
pub const __socket_type_SOCK_RDM: __socket_type = 4;
pub const __socket_type_SOCK_SEQPACKET: __socket_type = 5;
pub const __socket_type_SOCK_DCCP: __socket_type = 6;
pub const __socket_type_SOCK_PACKET: __socket_type = 10;
pub const __socket_type_SOCK_CLOEXEC: __socket_type = 524288;
pub const __socket_type_SOCK_NONBLOCK: __socket_type = 2048;
pub type __socket_type = u32;
pub type sa_family_t = c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [c_char; 118usize],
    pub __ss_align: c_ulong,
}
pub const MSG_OOB: _bindgen_ty_1 = 1;
pub const MSG_PEEK: _bindgen_ty_1 = 2;
pub const MSG_DONTROUTE: _bindgen_ty_1 = 4;
pub const MSG_CTRUNC: _bindgen_ty_1 = 8;
pub const MSG_PROXY: _bindgen_ty_1 = 16;
pub const MSG_TRUNC: _bindgen_ty_1 = 32;
pub const MSG_DONTWAIT: _bindgen_ty_1 = 64;
pub const MSG_EOR: _bindgen_ty_1 = 128;
pub const MSG_WAITALL: _bindgen_ty_1 = 256;
pub const MSG_FIN: _bindgen_ty_1 = 512;
pub const MSG_SYN: _bindgen_ty_1 = 1024;
pub const MSG_CONFIRM: _bindgen_ty_1 = 2048;
pub const MSG_RST: _bindgen_ty_1 = 4096;
pub const MSG_ERRQUEUE: _bindgen_ty_1 = 8192;
pub const MSG_NOSIGNAL: _bindgen_ty_1 = 16384;
pub const MSG_MORE: _bindgen_ty_1 = 32768;
pub const MSG_WAITFORONE: _bindgen_ty_1 = 65536;
pub const MSG_BATCH: _bindgen_ty_1 = 262144;
pub const MSG_ZEROCOPY: _bindgen_ty_1 = 67108864;
pub const MSG_FASTOPEN: _bindgen_ty_1 = 536870912;
pub const MSG_CMSG_CLOEXEC: _bindgen_ty_1 = 1073741824;
pub type _bindgen_ty_1 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut c_void,
    pub msg_controllen: usize,
    pub msg_flags: c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
    pub __cmsg_data: __IncompleteArrayField<c_uchar>,
}
extern "C" {
    pub fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
}
pub const SCM_RIGHTS: _bindgen_ty_2 = 1;
pub type _bindgen_ty_2 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}
pub const SHUT_RD: _bindgen_ty_3 = 0;
pub const SHUT_WR: _bindgen_ty_3 = 1;
pub const SHUT_RDWR: _bindgen_ty_3 = 2;
pub type _bindgen_ty_3 = u32;
extern "C" {
    pub fn socket(__domain: c_int, __type: c_int, __protocol: c_int) -> c_int;
}
extern "C" {
    pub fn socketpair(
        __domain: c_int,
        __type: c_int,
        __protocol: c_int,
        __fds: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn bind(__fd: c_int, __addr: *const sockaddr, __len: socklen_t) -> c_int;
}
extern "C" {
    pub fn getsockname(__fd: c_int, __addr: *mut sockaddr, __len: *mut socklen_t) -> c_int;
}
extern "C" {
    pub fn connect(__fd: c_int, __addr: *const sockaddr, __len: socklen_t) -> c_int;
}
extern "C" {
    pub fn getpeername(__fd: c_int, __addr: *mut sockaddr, __len: *mut socklen_t) -> c_int;
}
extern "C" {
    pub fn send(__fd: c_int, __buf: *const c_void, __n: usize, __flags: c_int) -> isize;
}
extern "C" {
    pub fn recv(__fd: c_int, __buf: *mut c_void, __n: usize, __flags: c_int) -> isize;
}
extern "C" {
    pub fn sendto(
        __fd: c_int,
        __buf: *const c_void,
        __n: usize,
        __flags: c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> isize;
}
extern "C" {
    pub fn recvfrom(
        __fd: c_int,
        __buf: *mut c_void,
        __n: usize,
        __flags: c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> isize;
}
extern "C" {
    pub fn sendmsg(__fd: c_int, __message: *const msghdr, __flags: c_int) -> isize;
}
extern "C" {
    pub fn recvmsg(__fd: c_int, __message: *mut msghdr, __flags: c_int) -> isize;
}
extern "C" {
    pub fn getsockopt(
        __fd: c_int,
        __level: c_int,
        __optname: c_int,
        __optval: *mut c_void,
        __optlen: *mut socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn setsockopt(
        __fd: c_int,
        __level: c_int,
        __optname: c_int,
        __optval: *const c_void,
        __optlen: socklen_t,
    ) -> c_int;
}
extern "C" {
    pub fn listen(__fd: c_int, __n: c_int) -> c_int;
}
extern "C" {
    pub fn accept(__fd: c_int, __addr: *mut sockaddr, __addr_len: *mut socklen_t) -> c_int;
}
extern "C" {
    pub fn shutdown(__fd: c_int, __how: c_int) -> c_int;
}
pub type in_addr_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub const IPPROTO_IP: _bindgen_ty_4 = 0;
pub const IPPROTO_ICMP: _bindgen_ty_4 = 1;
pub const IPPROTO_IGMP: _bindgen_ty_4 = 2;
pub const IPPROTO_IPIP: _bindgen_ty_4 = 4;
pub const IPPROTO_TCP: _bindgen_ty_4 = 6;
pub const IPPROTO_EGP: _bindgen_ty_4 = 8;
pub const IPPROTO_PUP: _bindgen_ty_4 = 12;
pub const IPPROTO_UDP: _bindgen_ty_4 = 17;
pub const IPPROTO_IDP: _bindgen_ty_4 = 22;
pub const IPPROTO_TP: _bindgen_ty_4 = 29;
pub const IPPROTO_DCCP: _bindgen_ty_4 = 33;
pub const IPPROTO_IPV6: _bindgen_ty_4 = 41;
pub const IPPROTO_RSVP: _bindgen_ty_4 = 46;
pub const IPPROTO_GRE: _bindgen_ty_4 = 47;
pub const IPPROTO_ESP: _bindgen_ty_4 = 50;
pub const IPPROTO_AH: _bindgen_ty_4 = 51;
pub const IPPROTO_MTP: _bindgen_ty_4 = 92;
pub const IPPROTO_BEETPH: _bindgen_ty_4 = 94;
pub const IPPROTO_ENCAP: _bindgen_ty_4 = 98;
pub const IPPROTO_PIM: _bindgen_ty_4 = 103;
pub const IPPROTO_COMP: _bindgen_ty_4 = 108;
pub const IPPROTO_SCTP: _bindgen_ty_4 = 132;
pub const IPPROTO_UDPLITE: _bindgen_ty_4 = 136;
pub const IPPROTO_MPLS: _bindgen_ty_4 = 137;
pub const IPPROTO_RAW: _bindgen_ty_4 = 255;
pub const IPPROTO_MAX: _bindgen_ty_4 = 256;
pub type _bindgen_ty_4 = u32;
pub const IPPROTO_HOPOPTS: _bindgen_ty_5 = 0;
pub const IPPROTO_ROUTING: _bindgen_ty_5 = 43;
pub const IPPROTO_FRAGMENT: _bindgen_ty_5 = 44;
pub const IPPROTO_ICMPV6: _bindgen_ty_5 = 58;
pub const IPPROTO_NONE: _bindgen_ty_5 = 59;
pub const IPPROTO_DSTOPTS: _bindgen_ty_5 = 60;
pub const IPPROTO_MH: _bindgen_ty_5 = 135;
pub type _bindgen_ty_5 = u32;
pub type in_port_t = u16;
pub const IPPORT_ECHO: _bindgen_ty_6 = 7;
pub const IPPORT_DISCARD: _bindgen_ty_6 = 9;
pub const IPPORT_SYSTAT: _bindgen_ty_6 = 11;
pub const IPPORT_DAYTIME: _bindgen_ty_6 = 13;
pub const IPPORT_NETSTAT: _bindgen_ty_6 = 15;
pub const IPPORT_FTP: _bindgen_ty_6 = 21;
pub const IPPORT_TELNET: _bindgen_ty_6 = 23;
pub const IPPORT_SMTP: _bindgen_ty_6 = 25;
pub const IPPORT_TIMESERVER: _bindgen_ty_6 = 37;
pub const IPPORT_NAMESERVER: _bindgen_ty_6 = 42;
pub const IPPORT_WHOIS: _bindgen_ty_6 = 43;
pub const IPPORT_MTP: _bindgen_ty_6 = 57;
pub const IPPORT_TFTP: _bindgen_ty_6 = 69;
pub const IPPORT_RJE: _bindgen_ty_6 = 77;
pub const IPPORT_FINGER: _bindgen_ty_6 = 79;
pub const IPPORT_TTYLINK: _bindgen_ty_6 = 87;
pub const IPPORT_SUPDUP: _bindgen_ty_6 = 95;
pub const IPPORT_EXECSERVER: _bindgen_ty_6 = 512;
pub const IPPORT_LOGINSERVER: _bindgen_ty_6 = 513;
pub const IPPORT_CMDSERVER: _bindgen_ty_6 = 514;
pub const IPPORT_EFSSERVER: _bindgen_ty_6 = 520;
pub const IPPORT_BIFFUDP: _bindgen_ty_6 = 512;
pub const IPPORT_WHOSERVER: _bindgen_ty_6 = 513;
pub const IPPORT_ROUTESERVER: _bindgen_ty_6 = 520;
pub const IPPORT_RESERVED: _bindgen_ty_6 = 1024;
pub const IPPORT_USERRESERVED: _bindgen_ty_6 = 5000;
pub type _bindgen_ty_6 = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub __in6_u: in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr__bindgen_ty_1 {
    pub __u6_addr8: [u8; 16usize],
    pub __u6_addr16: [u16; 8usize],
    pub __u6_addr32: [u32; 4usize],
    _bindgen_union_align: [u32; 4usize],
}
extern "C" {
    #[link_name = "\u{1}in6addr_any"]
    pub static in6addr_any: in6_addr;
}
extern "C" {
    #[link_name = "\u{1}in6addr_loopback"]
    pub static in6addr_loopback: in6_addr;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [c_uchar; 8usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: c_uint,
}
extern "C" {
    pub fn ntohl(__netlong: u32) -> u32;
}
extern "C" {
    pub fn ntohs(__netshort: u16) -> u16;
}
extern "C" {
    pub fn htonl(__hostlong: u32) -> u32;
}
extern "C" {
    pub fn htons(__hostshort: u16) -> u16;
}
pub type wchar_t = c_int;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: c_int,
    pub rem: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: c_long,
    pub rem: c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: c_longlong,
    pub rem: c_longlong,
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const c_char) -> c_int;
}
extern "C" {
    pub fn atol(__nptr: *const c_char) -> c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const c_char) -> c_longlong;
}
extern "C" {
    pub fn strtod(__nptr: *const c_char, __endptr: *mut c_char) -> f64;
}
extern "C" {
    pub fn strtof(__nptr: *const c_char, __endptr: *mut c_char) -> f32;
}
extern "C" {
    pub fn strtold(__nptr: *const c_char, __endptr: *mut c_char) -> f64;
}
extern "C" {
    pub fn strtol(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_long;
}
extern "C" {
    pub fn strtoul(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_ulong;
}
extern "C" {
    pub fn strtoll(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_longlong;
}
extern "C" {
    pub fn strtoull(__nptr: *const c_char, __endptr: *mut c_char, __base: c_int) -> c_ulonglong;
}
extern "C" {
    pub fn rand() -> c_int;
}
extern "C" {
    pub fn srand(__seed: c_uint);
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn realloc(__ptr: *mut c_void, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn free(__ptr: *mut c_void);
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> c_int;
}
extern "C" {
    pub fn at_quick_exit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> c_int;
}
extern "C" {
    pub fn exit(__status: c_int);
}
extern "C" {
    pub fn quick_exit(__status: c_int);
}
extern "C" {
    pub fn _Exit(__status: c_int);
}
extern "C" {
    pub fn getenv(__name: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn system(__command: *const c_char) -> c_int;
}
pub type __compar_fn_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void) -> c_int>;
extern "C" {
    pub fn bsearch(
        __key: *const c_void,
        __base: *const c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn qsort(__base: *mut c_void, __nmemb: usize, __size: usize, __compar: __compar_fn_t);
}
extern "C" {
    pub fn abs(__x: c_int) -> c_int;
}
extern "C" {
    pub fn labs(__x: c_long) -> c_long;
}
extern "C" {
    pub fn llabs(__x: c_longlong) -> c_longlong;
}
extern "C" {
    pub fn div(__numer: c_int, __denom: c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: c_long, __denom: c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(__numer: c_longlong, __denom: c_longlong) -> lldiv_t;
}
extern "C" {
    pub fn mblen(__s: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn mbtowc(__pwc: *mut wchar_t, __s: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut c_char, __wchar: wchar_t) -> c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn memmove(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn memset(__s: *mut c_void, __c: c_int, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: usize) -> c_int;
}
extern "C" {
    pub fn memchr(__s: *const c_void, __c: c_int, __n: usize) -> *mut c_void;
}
extern "C" {
    pub fn strcpy(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strncpy(__dest: *mut c_char, __src: *const c_char, __n: usize) -> *mut c_char;
}
extern "C" {
    pub fn strcat(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strncat(__dest: *mut c_char, __src: *const c_char, __n: usize) -> *mut c_char;
}
extern "C" {
    pub fn strcmp(__s1: *const c_char, __s2: *const c_char) -> c_int;
}
extern "C" {
    pub fn strncmp(__s1: *const c_char, __s2: *const c_char, __n: usize) -> c_int;
}
extern "C" {
    pub fn strcoll(__s1: *const c_char, __s2: *const c_char) -> c_int;
}
extern "C" {
    pub fn strxfrm(__dest: *mut c_char, __src: *const c_char, __n: usize) -> c_ulong;
}
extern "C" {
    pub fn strchr(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strrchr(__s: *const c_char, __c: c_int) -> *mut c_char;
}
extern "C" {
    pub fn strcspn(__s: *const c_char, __reject: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn strspn(__s: *const c_char, __accept: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn strpbrk(__s: *const c_char, __accept: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strstr(__haystack: *const c_char, __needle: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn strtok(__s: *mut c_char, __delim: *const c_char) -> *mut c_char;
}
extern "C" {
    pub fn __strtok_r(
        __s: *mut c_char,
        __delim: *const c_char,
        __save_ptr: *mut c_char,
    ) -> *mut c_char;
}
extern "C" {
    pub fn strlen(__s: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn strerror(__errnum: c_int) -> *mut c_char;
}
extern "C" {
    pub fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    );
}
extern "C" {
    pub fn __assert_perror_fail(
        __errnum: c_int,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    );
}
extern "C" {
    pub fn __assert(__assertion: *const c_char, __file: *const c_char, __line: c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_socket_ {
    _unused: [u8; 0],
}
pub type ck_socket = *mut ck_socket_;
extern "C" {
    pub fn ck_udp_create() -> ck_socket;
}
extern "C" {
    pub fn ck_tcp_create(flags: c_int) -> ck_socket;
}
extern "C" {
    pub fn ck_connect(sock: ck_socket, hostname: *const c_char, port: c_int) -> c_ulong;
}
extern "C" {
    pub fn ck_connect2(sock: ck_socket, serv_addr: *const sockaddr, addrlen: c_int) -> c_ulong;
}
extern "C" {
    pub fn ck_bind(sock: ck_socket, port: c_int) -> c_ulong;
}
extern "C" {
    pub fn ck_listen(sock: ck_socket, backlog: c_int) -> c_ulong;
}
extern "C" {
    pub fn ck_accept(sock: ck_socket) -> ck_socket;
}
extern "C" {
    pub fn ck_send(sock: ck_socket, buffer: *const c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn ck_send2(sock: ck_socket, buffer: *const c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn ck_sendto(
        sock: ck_socket,
        buffer: *const c_char,
        len: c_int,
        to: *const sockaddr,
        tolen: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn ck_recv(sock: ck_socket, buffer: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn ck_recv2(sock: ck_socket, buffer: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn ck_recvfrom(
        sock: ck_socket,
        buffer: *mut c_char,
        len: c_int,
        from: *mut sockaddr,
        fromlen: *mut c_int,
    ) -> c_int;
}
extern "C" {
    pub fn ck_send_timeout(sock: ck_socket, sec: c_long, usec: c_long) -> c_int;
}
extern "C" {
    pub fn ck_recv_timeout(sock: ck_socket, sec: c_long, usec: c_long) -> c_int;
}
extern "C" {
    pub fn ck_close(sock: ck_socket);
}
