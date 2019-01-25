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
///* header file for joystick/mouse/keyboard support
use libc::*;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HidMsg {
    pub device_type: c_long,
    pub device_num: c_long,
    pub type_: c_long,
    pub eid: c_long,
    pub idata: [c_long; 4usize],
    pub fdata: [f64; 4usize],
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_NONE"]
    pub static CK_HID_DEV_NONE: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_JOYSTICK"]
    pub static CK_HID_DEV_JOYSTICK: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_MOUSE"]
    pub static CK_HID_DEV_MOUSE: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_KEYBOARD"]
    pub static CK_HID_DEV_KEYBOARD: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_WIIREMOTE"]
    pub static CK_HID_DEV_WIIREMOTE: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_TILTSENSOR"]
    pub static CK_HID_DEV_TILTSENSOR: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_TABLET"]
    pub static CK_HID_DEV_TABLET: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_MULTITOUCH"]
    pub static CK_HID_DEV_MULTITOUCH: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEV_COUNT"]
    pub static CK_HID_DEV_COUNT: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_JOYSTICK_AXIS"]
    pub static CK_HID_JOYSTICK_AXIS: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_BUTTON_DOWN"]
    pub static CK_HID_BUTTON_DOWN: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_BUTTON_UP"]
    pub static CK_HID_BUTTON_UP: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_JOYSTICK_HAT"]
    pub static CK_HID_JOYSTICK_HAT: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_JOYSTICK_BALL"]
    pub static CK_HID_JOYSTICK_BALL: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_MOUSE_MOTION"]
    pub static CK_HID_MOUSE_MOTION: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_MOUSE_WHEEL"]
    pub static CK_HID_MOUSE_WHEEL: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEVICE_CONNECTED"]
    pub static CK_HID_DEVICE_CONNECTED: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_DEVICE_DISCONNECTED"]
    pub static CK_HID_DEVICE_DISCONNECTED: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_ACCELEROMETER"]
    pub static CK_HID_ACCELEROMETER: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_WIIREMOTE_IR"]
    pub static CK_HID_WIIREMOTE_IR: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_LED"]
    pub static CK_HID_LED: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_FORCE_FEEDBACK"]
    pub static CK_HID_FORCE_FEEDBACK: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_SPEAKER"]
    pub static CK_HID_SPEAKER: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_TABLET_PRESSURE"]
    pub static CK_HID_TABLET_PRESSURE: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_TABLET_MOTION"]
    pub static CK_HID_TABLET_MOTION: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_TABLET_ROTATION"]
    pub static CK_HID_TABLET_ROTATION: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_MULTITOUCH_TOUCH"]
    pub static CK_HID_MULTITOUCH_TOUCH: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}CK_HID_MSG_COUNT"]
    pub static CK_HID_MSG_COUNT: c_ulong;
}
pub const HidResult_HID_GENERALERROR: HidResult = -1;
pub const HidResult_HID_NOERROR: HidResult = 0;
pub type HidResult = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Chuck_Hid_Driver {
    pub init: Option<unsafe extern "C" fn()>,
    pub quit: Option<unsafe extern "C" fn()>,
    pub poll: Option<unsafe extern "C" fn()>,
    pub probe: Option<unsafe extern "C" fn()>,
    pub count: Option<unsafe extern "C" fn() -> c_int>,
    pub count_elements: Option<unsafe extern "C" fn(arg1: c_int, arg2: c_int) -> c_int>,
    pub open: Option<unsafe extern "C" fn(arg1: c_int) -> c_int>,
    pub open_async: Option<unsafe extern "C" fn(arg1: c_int) -> c_int>,
    pub close: Option<unsafe extern "C" fn(arg1: c_int) -> c_int>,
    pub send: Option<unsafe extern "C" fn(arg1: c_int, arg2: *const HidMsg) -> c_int>,
    pub read: Option<
        unsafe extern "C" fn(arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut HidMsg) -> c_int,
    >,
    pub name: Option<unsafe extern "C" fn(arg1: c_int) -> *const c_char>,
    pub driver_name: *const c_char,
}
pub type Chuck_Hid_Driver = _Chuck_Hid_Driver;
extern "C" {
    #[link_name = "\u{1}_Z8Hid_initv"]
    pub fn Hid_init();
}
extern "C" {
    #[link_name = "\u{1}_Z8Hid_pollv"]
    pub fn Hid_poll();
}
extern "C" {
    #[link_name = "\u{1}_Z8Hid_quitv"]
    pub fn Hid_quit();
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_initv"]
    pub fn Joystick_init();
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_pollv"]
    pub fn Joystick_poll();
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_quitv"]
    pub fn Joystick_quit();
}
extern "C" {
    #[link_name = "\u{1}_Z14Joystick_probev"]
    pub fn Joystick_probe();
}
extern "C" {
    #[link_name = "\u{1}_Z14Joystick_countv"]
    pub fn Joystick_count() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z23Joystick_count_elementsii"]
    pub fn Joystick_count_elements(js: c_int, type_: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_openi"]
    pub fn Joystick_open(js: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z19Joystick_open_asynci"]
    pub fn Joystick_open_async(js: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_openPKc"]
    pub fn Joystick_open1(name: *const c_char) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z14Joystick_closei"]
    pub fn Joystick_close(js: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_sendiPK6HidMsg"]
    pub fn Joystick_send(js: c_int, msg: *const HidMsg) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_namei"]
    pub fn Joystick_name(js: c_int) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_axesi"]
    pub fn Joystick_axes(js: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z16Joystick_buttonsi"]
    pub fn Joystick_buttons(js: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Joystick_hatsi"]
    pub fn Joystick_hats(js: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z10Mouse_initv"]
    pub fn Mouse_init();
}
extern "C" {
    #[link_name = "\u{1}_Z10Mouse_pollv"]
    pub fn Mouse_poll();
}
extern "C" {
    #[link_name = "\u{1}_Z10Mouse_quitv"]
    pub fn Mouse_quit();
}
extern "C" {
    #[link_name = "\u{1}_Z11Mouse_probev"]
    pub fn Mouse_probe();
}
extern "C" {
    #[link_name = "\u{1}_Z11Mouse_countv"]
    pub fn Mouse_count() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z20Mouse_count_elementsii"]
    pub fn Mouse_count_elements(js: c_int, type_: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z10Mouse_openi"]
    pub fn Mouse_open(m: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z10Mouse_openPKc"]
    pub fn Mouse_open1(name: *const c_char) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z11Mouse_closei"]
    pub fn Mouse_close(m: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z10Mouse_sendiPK6HidMsg"]
    pub fn Mouse_send(m: c_int, msg: *const HidMsg) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z10Mouse_namei"]
    pub fn Mouse_name(m: c_int) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_Z13Mouse_buttonsi"]
    pub fn Mouse_buttons(m: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z24Mouse_start_cursor_trackv"]
    pub fn Mouse_start_cursor_track() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z23Mouse_stop_cursor_trackv"]
    pub fn Mouse_stop_cursor_track() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Keyboard_initv"]
    pub fn Keyboard_init();
}
extern "C" {
    #[link_name = "\u{1}_Z13Keyboard_pollv"]
    pub fn Keyboard_poll();
}
extern "C" {
    #[link_name = "\u{1}_Z13Keyboard_quitv"]
    pub fn Keyboard_quit();
}
extern "C" {
    #[link_name = "\u{1}_Z14Keyboard_probev"]
    pub fn Keyboard_probe();
}
extern "C" {
    #[link_name = "\u{1}_Z14Keyboard_countv"]
    pub fn Keyboard_count() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z23Keyboard_count_elementsii"]
    pub fn Keyboard_count_elements(js: c_int, type_: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Keyboard_openi"]
    pub fn Keyboard_open(kb: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Keyboard_openPKc"]
    pub fn Keyboard_open1(name: *const c_char) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z14Keyboard_closei"]
    pub fn Keyboard_close(kb: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Keyboard_sendiPK6HidMsg"]
    pub fn Keyboard_send(kb: c_int, msg: *const HidMsg) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z13Keyboard_namei"]
    pub fn Keyboard_name(kb: c_int) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_Z14WiiRemote_initv"]
    pub fn WiiRemote_init();
}
extern "C" {
    #[link_name = "\u{1}_Z14WiiRemote_pollv"]
    pub fn WiiRemote_poll();
}
extern "C" {
    #[link_name = "\u{1}_Z14WiiRemote_quitv"]
    pub fn WiiRemote_quit();
}
extern "C" {
    #[link_name = "\u{1}_Z15WiiRemote_probev"]
    pub fn WiiRemote_probe();
}
extern "C" {
    #[link_name = "\u{1}_Z15WiiRemote_countv"]
    pub fn WiiRemote_count() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z14WiiRemote_openi"]
    pub fn WiiRemote_open(wr: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z14WiiRemote_openPKc"]
    pub fn WiiRemote_open1(name: *const c_char) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z15WiiRemote_closei"]
    pub fn WiiRemote_close(wr: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z14WiiRemote_sendiPK6HidMsg"]
    pub fn WiiRemote_send(wr: c_int, msg: *const HidMsg) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z14WiiRemote_namei"]
    pub fn WiiRemote_name(wr: c_int) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_Z15TiltSensor_initv"]
    pub fn TiltSensor_init();
}
extern "C" {
    #[link_name = "\u{1}_Z15TiltSensor_quitv"]
    pub fn TiltSensor_quit();
}
extern "C" {
    #[link_name = "\u{1}_Z16TiltSensor_probev"]
    pub fn TiltSensor_probe();
}
extern "C" {
    #[link_name = "\u{1}_Z16TiltSensor_countv"]
    pub fn TiltSensor_count() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z15TiltSensor_openi"]
    pub fn TiltSensor_open(ts: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z16TiltSensor_closei"]
    pub fn TiltSensor_close(ts: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z15TiltSensor_readiiiP6HidMsg"]
    pub fn TiltSensor_read(ts: c_int, type_: c_int, num: c_int, msg: *mut HidMsg) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z15TiltSensor_namei"]
    pub fn TiltSensor_name(ts: c_int) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_Z22TiltSensor_setPollRatel"]
    pub fn TiltSensor_setPollRate(usec: c_long) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}_Z22TiltSensor_getPollRatev"]
    pub fn TiltSensor_getPollRate() -> c_long;
}
extern "C" {
    #[link_name = "\u{1}_Z21MultiTouchDevice_initv"]
    pub fn MultiTouchDevice_init();
}
extern "C" {
    #[link_name = "\u{1}_Z21MultiTouchDevice_quitv"]
    pub fn MultiTouchDevice_quit();
}
extern "C" {
    #[link_name = "\u{1}_Z22MultiTouchDevice_probev"]
    pub fn MultiTouchDevice_probe();
}
extern "C" {
    #[link_name = "\u{1}_Z22MultiTouchDevice_countv"]
    pub fn MultiTouchDevice_count() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z21MultiTouchDevice_openi"]
    pub fn MultiTouchDevice_open(ts: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z22MultiTouchDevice_closei"]
    pub fn MultiTouchDevice_close(ts: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z21MultiTouchDevice_namei"]
    pub fn MultiTouchDevice_name(ts: c_int) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}_Z11Tablet_initv"]
    pub fn Tablet_init();
}
extern "C" {
    #[link_name = "\u{1}_Z11Tablet_quitv"]
    pub fn Tablet_quit();
}
extern "C" {
    #[link_name = "\u{1}_Z12Tablet_probev"]
    pub fn Tablet_probe();
}
extern "C" {
    #[link_name = "\u{1}_Z12Tablet_countv"]
    pub fn Tablet_count() -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z11Tablet_openi"]
    pub fn Tablet_open(ts: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z12Tablet_closei"]
    pub fn Tablet_close(ts: c_int) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z11Tablet_namei"]
    pub fn Tablet_name(ts: c_int) -> *const c_char;
}
