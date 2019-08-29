use sdl2;
/// util_hid.h
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
enum HidResult {
    HID_GENERALERROR = -1,
    HID_NOERROR = 0,
}
pub struct HidInManager {

}
