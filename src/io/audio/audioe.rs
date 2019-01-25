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
//* ChucK audio interface w/ RtAudio
use crate::dts::*;
use crate::RtAudio_h_edited::*;
use libc::*;
fn main() {}
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(unused_imports)]
pub const NUM_CHANNELS_DEFAULT: u32 = 2;
pub const NUM_BUFFERS_DEFAULT: u32 = 8;
pub const DEVICE_NUM_OUT_DEFAULT: u32 = 0;
pub const DEVICE_NUM_IN_DEFAULT: u32 = 0;
pub const SAMPLE_RATE_DEFAULT: u32 = 48000;
pub const BUFFER_SIZE_DEFAULT: u32 = 256;
pub type f_audio_cb = Option<
    unsafe extern "C" fn(
        input: *mut f64,
        output: *mut f64,
        numFrames: c_ulong,
        numInChans: c_ulong,
        numOutChans: c_ulong,
        userData: *mut c_void,
    ),
>;
extern "C" {
    #[link_name = "\u{1}g_do_watchdog"]
    pub static mut g_do_watchdog: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}g_watchdog_countermeasure_priority"]
    pub static mut g_watchdog_countermeasure_priority: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}g_watchdog_timeout"]
    pub static mut g_watchdog_timeout: f64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ChuckAudio {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio6m_initE"]
    pub static mut ChuckAudio_m_init: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio7m_startE"]
    pub static mut ChuckAudio_m_start: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio4m_goE"]
    pub static mut ChuckAudio_m_go: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio8m_silentE"]
    pub static mut ChuckAudio_m_silent: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio18m_num_channels_outE"]
    pub static mut ChuckAudio_m_num_channels_out: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio17m_num_channels_inE"]
    pub static mut ChuckAudio_m_num_channels_in: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio13m_sample_rateE"]
    pub static mut ChuckAudio_m_sample_rate: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio5m_bpsE"]
    pub static mut ChuckAudio_m_bps: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio13m_buffer_sizeE"]
    pub static mut ChuckAudio_m_buffer_size: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio13m_num_buffersE"]
    pub static mut ChuckAudio_m_num_buffers: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio12m_buffer_outE"]
    pub static mut ChuckAudio_m_buffer_out: *mut f64;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio11m_buffer_inE"]
    pub static mut ChuckAudio_m_buffer_in: *mut f64;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio11m_extern_inE"]
    pub static mut ChuckAudio_m_extern_in: *mut f64;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio12m_extern_outE"]
    pub static mut ChuckAudio_m_extern_out: *mut f64;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio9m_rtaudioE"]
    pub static mut ChuckAudio_m_rtaudio: *mut RtAudio;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio7m_dac_nE"]
    pub static mut ChuckAudio_m_dac_n: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio7m_adc_nE"]
    pub static mut ChuckAudio_m_adc_n: c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio10m_audio_cbE"]
    pub static mut ChuckAudio_m_audio_cb: f_audio_cb;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio14m_cb_user_dataE"]
    pub static mut ChuckAudio_m_cb_user_data: *mut c_void;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio10initializeEmmmmmPFvPdS0_mmmPvES1_m"]
    pub fn ChuckAudio_initialize(
        num_dac_channels: c_ulong,
        num_adc_channels: c_ulong,
        sample_rate: c_ulong,
        buffer_size: c_ulong,
        num_buffers: c_ulong,
        callback: f_audio_cb,
        data: *mut c_void,
        force_srate: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio8shutdownEv"]
    pub fn ChuckAudio_shutdown();
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio5startEv"]
    pub fn ChuckAudio_start() -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio4stopEv"]
    pub fn ChuckAudio_stop() -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio14watchdog_startEv"]
    pub fn ChuckAudio_watchdog_start() -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio13watchdog_stopEv"]
    pub fn ChuckAudio_watchdog_stop() -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio5probeEv"]
    pub fn ChuckAudio_probe();
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio12device_namedERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEmm"]
    pub fn ChuckAudio_device_named(
        name: *const crate::rtmidi_h_edited::string,
        needs_dac: c_ulong,
        needs_adc: c_ulong,
    ) -> c_ulong;
}
extern "C" {
    #[link_name = "\u{1}_ZN10ChuckAudio2cbEPvS0_jdjS0_"]
    pub fn ChuckAudio_cb(
        output_buffer: *mut c_void,
        input_buffer: *mut c_void,
        buffer_size: c_uint,
        streamTime: f64,
        status: RtAudioStreamStatus,
        user_data: *mut c_void,
    ) -> c_int;
}
impl ChuckAudio {
    #[inline]
    pub unsafe fn initialize(
        num_dac_channels: c_ulong,
        num_adc_channels: c_ulong,
        sample_rate: c_ulong,
        buffer_size: c_ulong,
        num_buffers: c_ulong,
        callback: f_audio_cb,
        data: *mut c_void,
        force_srate: c_ulong,
    ) -> c_ulong {
        ChuckAudio_initialize(
            num_dac_channels,
            num_adc_channels,
            sample_rate,
            buffer_size,
            num_buffers,
            callback,
            data,
            force_srate,
        )
    }
    #[inline]
    pub unsafe fn shutdown() {
        ChuckAudio_shutdown()
    }
    #[inline]
    pub unsafe fn start() -> c_ulong {
        ChuckAudio_start()
    }
    #[inline]
    pub unsafe fn stop() -> c_ulong {
        ChuckAudio_stop()
    }
    #[inline]
    pub unsafe fn watchdog_start() -> c_ulong {
        ChuckAudio_watchdog_start()
    }
    #[inline]
    pub unsafe fn watchdog_stop() -> c_ulong {
        ChuckAudio_watchdog_stop()
    }
    #[inline]
    pub unsafe fn probe() {
        ChuckAudio_probe()
    }
    #[inline]
    pub unsafe fn device_named(
        name: *const crate::rtmidi_h_edited::string,
        needs_dac: c_ulong,
        needs_adc: c_ulong,
    ) -> c_ulong {
        ChuckAudio_device_named(name, needs_dac, needs_adc)
    }
    #[inline]
    pub unsafe fn cb(
        output_buffer: *mut c_void,
        input_buffer: *mut c_void,
        buffer_size: c_uint,
        streamTime: f64,
        status: RtAudioStreamStatus,
        user_data: *mut c_void,
    ) -> c_int {
        ChuckAudio_cb(
            output_buffer,
            input_buffer,
            buffer_size,
            streamTime,
            status,
            user_data,
        )
    }
}
