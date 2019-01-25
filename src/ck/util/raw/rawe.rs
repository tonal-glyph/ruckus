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
///* raw waves from STK
use libc::*;
extern "C" {
    #[link_name = "\u{1}ahh_data"]
    pub static mut ahh_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}britestk_data"]
    pub static mut britestk_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}dope_data"]
    pub static mut dope_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}eee_data"]
    pub static mut eee_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}fwavblnk_data"]
    pub static mut fwavblnk_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}halfwave_data"]
    pub static mut halfwave_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}impuls10_data"]
    pub static mut impuls10_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}impuls20_data"]
    pub static mut impuls20_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}impuls40_data"]
    pub static mut impuls40_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}mand1_data"]
    pub static mut mand1_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}mandpluk_data"]
    pub static mut mandpluk_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}marmstk1_data"]
    pub static mut marmstk1_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}ooo_data"]
    pub static mut ooo_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}peksblnk_data"]
    pub static mut peksblnk_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}ppksblnk_data"]
    pub static mut ppksblnk_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}silence_data"]
    pub static mut silence_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}sineblnk_data"]
    pub static mut sineblnk_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}sinewave_data"]
    pub static mut sinewave_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}snglpeak_data"]
    pub static mut snglpeak_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}twopeaks_data"]
    pub static mut twopeaks_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}glot_ahh_data"]
    pub static mut glot_ahh_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}glot_eee_data"]
    pub static mut glot_eee_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}glot_ooo_data"]
    pub static mut glot_ooo_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}glot_pop_data"]
    pub static mut glot_pop_data: [f64; 0usize];
}
extern "C" {
    #[link_name = "\u{1}ahh_size"]
    pub static mut ahh_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}britestk_size"]
    pub static mut britestk_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}dope_size"]
    pub static mut dope_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}eee_size"]
    pub static mut eee_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}fwavblnk_size"]
    pub static mut fwavblnk_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}halfwave_size"]
    pub static mut halfwave_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}impuls10_size"]
    pub static mut impuls10_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}impuls20_size"]
    pub static mut impuls20_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}impuls40_size"]
    pub static mut impuls40_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}mand1_size"]
    pub static mut mand1_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}mandpluk_size"]
    pub static mut mandpluk_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}marmstk1_size"]
    pub static mut marmstk1_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}ooo_size"]
    pub static mut ooo_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}peksblnk_size"]
    pub static mut peksblnk_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}ppksblnk_size"]
    pub static mut ppksblnk_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}silence_size"]
    pub static mut silence_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}sineblnk_size"]
    pub static mut sineblnk_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}sinewave_size"]
    pub static mut sinewave_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}snglpeak_size"]
    pub static mut snglpeak_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}twopeaks_size"]
    pub static mut twopeaks_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}glot_ahh_size"]
    pub static mut glot_ahh_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}glot_eee_size"]
    pub static mut glot_eee_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}glot_ooo_size"]
    pub static mut glot_ooo_size: t_CKUINT;
}
extern "C" {
    #[link_name = "\u{1}glot_pop_size"]
    pub static mut glot_pop_size: t_CKUINT;
}
