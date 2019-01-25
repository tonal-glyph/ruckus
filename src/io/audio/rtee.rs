fn main() {}
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct RtError {
    pub _base: std::exception,
    pub message_: std::string,
    pub type_: RtError_Type,
}
#[doc = "< A non-critical error."]
pub const RtError_Type_WARNING: RtError_Type = 0;
#[doc = "< A non-critical error which might be useful for debugging."]
pub const RtError_Type_DEBUG_WARNING: RtError_Type = 1;
#[doc = "< The default, unspecified error type."]
pub const RtError_Type_UNSPECIFIED: RtError_Type = 2;
#[doc = "< No devices found on system."]
pub const RtError_Type_NO_DEVICES_FOUND: RtError_Type = 3;
#[doc = "< An invalid device ID was specified."]
pub const RtError_Type_INVALID_DEVICE: RtError_Type = 4;
#[doc = "< An error occured during memory allocation."]
pub const RtError_Type_MEMORY_ERROR: RtError_Type = 5;
#[doc = "< An invalid parameter was specified to a function."]
pub const RtError_Type_INVALID_PARAMETER: RtError_Type = 6;
#[doc = "< The function was called incorrectly."]
pub const RtError_Type_INVALID_USE: RtError_Type = 7;
#[doc = "< A system driver error occured."]
pub const RtError_Type_DRIVER_ERROR: RtError_Type = 8;
#[doc = "< A system error occured."]
pub const RtError_Type_SYSTEM_ERROR: RtError_Type = 9;
#[doc = "< A thread error occured."]
pub const RtError_Type_THREAD_ERROR: RtError_Type = 10;
#[doc = "! Defined RtError types."]
pub type RtError_Type = u32;
