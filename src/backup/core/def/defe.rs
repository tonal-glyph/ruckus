#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX {
    pub re: f64,
    pub im: f64,
}
#[test]
fn bindgen_test_layout_t_CKCOMPLEX() {
    assert_eq!(
        ::std::mem::size_of::<t_CKCOMPLEX>(),
        16usize,
        concat!("Size of: ", stringify!(t_CKCOMPLEX))
    );
    assert_eq!(
        ::std::mem::align_of::<t_CKCOMPLEX>(),
        8usize,
        concat!("Alignment of ", stringify!(t_CKCOMPLEX))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX>())).re as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKCOMPLEX),
            "::",
            stringify!(re)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX>())).im as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKCOMPLEX),
            "::",
            stringify!(im)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKPOLAR {
    pub modulus: f64,
    pub phase: f64,
}
#[test]
fn bindgen_test_layout_t_CKPOLAR() {
    assert_eq!(
        ::std::mem::size_of::<t_CKPOLAR>(),
        16usize,
        concat!("Size of: ", stringify!(t_CKPOLAR))
    );
    assert_eq!(
        ::std::mem::align_of::<t_CKPOLAR>(),
        8usize,
        concat!("Alignment of ", stringify!(t_CKPOLAR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKPOLAR>())).modulus as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKPOLAR),
            "::",
            stringify!(modulus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKPOLAR>())).phase as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKPOLAR),
            "::",
            stringify!(phase)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[test]
fn bindgen_test_layout_t_CKVEC3() {
    assert_eq!(
        ::std::mem::size_of::<t_CKVEC3>(),
        24usize,
        concat!("Size of: ", stringify!(t_CKVEC3))
    );
    assert_eq!(
        ::std::mem::align_of::<t_CKVEC3>(),
        8usize,
        concat!("Alignment of ", stringify!(t_CKVEC3))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVEC3),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).y as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVEC3),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVEC3>())).z as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVEC3),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKVEC4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[test]
fn bindgen_test_layout_t_CKVEC4() {
    assert_eq!(
        ::std::mem::size_of::<t_CKVEC4>(),
        32usize,
        concat!("Size of: ", stringify!(t_CKVEC4))
    );
    assert_eq!(
        ::std::mem::align_of::<t_CKVEC4>(),
        8usize,
        concat!("Alignment of ", stringify!(t_CKVEC4))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVEC4),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).y as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVEC4),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).z as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVEC4),
            "::",
            stringify!(z)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVEC4>())).w as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVEC4),
            "::",
            stringify!(w)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct t_CKVECTOR {
    pub N: ::std::os::raw::c_ulong,
    pub values: *mut f64,
}
#[test]
fn bindgen_test_layout_t_CKVECTOR() {
    assert_eq!(
        ::std::mem::size_of::<t_CKVECTOR>(),
        16usize,
        concat!("Size of: ", stringify!(t_CKVECTOR))
    );
    assert_eq!(
        ::std::mem::align_of::<t_CKVECTOR>(),
        8usize,
        concat!("Alignment of ", stringify!(t_CKVECTOR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVECTOR>())).N as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVECTOR),
            "::",
            stringify!(N)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKVECTOR>())).values as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKVECTOR),
            "::",
            stringify!(values)
        )
    );
}
impl Default for t_CKVECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type c_str = *mut ::std::os::raw::c_char;
pub type c_constr = *const ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct t_CKCOMPLEX_SAMPLE {
    pub re: f64,
    pub im: f64,
}
#[test]
fn bindgen_test_layout_t_CKCOMPLEX_SAMPLE() {
    assert_eq!(
        ::std::mem::size_of::<t_CKCOMPLEX_SAMPLE>(),
        16usize,
        concat!("Size of: ", stringify!(t_CKCOMPLEX_SAMPLE))
    );
    assert_eq!(
        ::std::mem::align_of::<t_CKCOMPLEX_SAMPLE>(),
        8usize,
        concat!("Alignment of ", stringify!(t_CKCOMPLEX_SAMPLE))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX_SAMPLE>())).re as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKCOMPLEX_SAMPLE),
            "::",
            stringify!(re)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<t_CKCOMPLEX_SAMPLE>())).im as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(t_CKCOMPLEX_SAMPLE),
            "::",
            stringify!(im)
        )
    );
}
