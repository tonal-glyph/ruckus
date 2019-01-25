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
///* utility for serial I/O
use libc::*;
pub const SF_FORMAT_WAV: _bindgen_ty_1 = 65536;
pub const SF_FORMAT_AIFF: _bindgen_ty_1 = 131072;
pub const SF_FORMAT_AU: _bindgen_ty_1 = 196608;
pub const SF_FORMAT_RAW: _bindgen_ty_1 = 262144;
pub const SF_FORMAT_PAF: _bindgen_ty_1 = 327680;
pub const SF_FORMAT_SVX: _bindgen_ty_1 = 393216;
pub const SF_FORMAT_NIST: _bindgen_ty_1 = 458752;
pub const SF_FORMAT_VOC: _bindgen_ty_1 = 524288;
pub const SF_FORMAT_IRCAM: _bindgen_ty_1 = 655360;
pub const SF_FORMAT_W64: _bindgen_ty_1 = 720896;
pub const SF_FORMAT_MAT4: _bindgen_ty_1 = 786432;
pub const SF_FORMAT_MAT5: _bindgen_ty_1 = 851968;
pub const SF_FORMAT_PVF: _bindgen_ty_1 = 917504;
pub const SF_FORMAT_XI: _bindgen_ty_1 = 983040;
pub const SF_FORMAT_HTK: _bindgen_ty_1 = 1048576;
pub const SF_FORMAT_SDS: _bindgen_ty_1 = 1114112;
pub const SF_FORMAT_AVR: _bindgen_ty_1 = 1179648;
pub const SF_FORMAT_WAVEX: _bindgen_ty_1 = 1245184;
pub const SF_FORMAT_PCM_S8: _bindgen_ty_1 = 1;
pub const SF_FORMAT_PCM_16: _bindgen_ty_1 = 2;
pub const SF_FORMAT_PCM_24: _bindgen_ty_1 = 3;
pub const SF_FORMAT_PCM_32: _bindgen_ty_1 = 4;
pub const SF_FORMAT_PCM_U8: _bindgen_ty_1 = 5;
pub const SF_FORMAT_FLOAT: _bindgen_ty_1 = 6;
pub const SF_FORMAT_DOUBLE: _bindgen_ty_1 = 7;
pub const SF_FORMAT_ULAW: _bindgen_ty_1 = 16;
pub const SF_FORMAT_ALAW: _bindgen_ty_1 = 17;
pub const SF_FORMAT_IMA_ADPCM: _bindgen_ty_1 = 18;
pub const SF_FORMAT_MS_ADPCM: _bindgen_ty_1 = 19;
pub const SF_FORMAT_GSM610: _bindgen_ty_1 = 32;
pub const SF_FORMAT_VOX_ADPCM: _bindgen_ty_1 = 33;
pub const SF_FORMAT_G721_32: _bindgen_ty_1 = 48;
pub const SF_FORMAT_G723_24: _bindgen_ty_1 = 49;
pub const SF_FORMAT_G723_40: _bindgen_ty_1 = 50;
pub const SF_FORMAT_DWVW_12: _bindgen_ty_1 = 64;
pub const SF_FORMAT_DWVW_16: _bindgen_ty_1 = 65;
pub const SF_FORMAT_DWVW_24: _bindgen_ty_1 = 66;
pub const SF_FORMAT_DWVW_N: _bindgen_ty_1 = 67;
pub const SF_FORMAT_DPCM_8: _bindgen_ty_1 = 80;
pub const SF_FORMAT_DPCM_16: _bindgen_ty_1 = 81;
pub const SF_ENDIAN_FILE: _bindgen_ty_1 = 0;
pub const SF_ENDIAN_LITTLE: _bindgen_ty_1 = 268435456;
pub const SF_ENDIAN_BIG: _bindgen_ty_1 = 536870912;
pub const SF_ENDIAN_CPU: _bindgen_ty_1 = 805306368;
pub const SF_FORMAT_SUBMASK: _bindgen_ty_1 = 65535;
pub const SF_FORMAT_TYPEMASK: _bindgen_ty_1 = 268369920;
pub const SF_FORMAT_ENDMASK: _bindgen_ty_1 = 805306368;
pub type _bindgen_ty_1 = u32;
pub const SFC_GET_LIB_VERSION: _bindgen_ty_2 = 4096;
pub const SFC_GET_LOG_INFO: _bindgen_ty_2 = 4097;
pub const SFC_GET_NORM_DOUBLE: _bindgen_ty_2 = 4112;
pub const SFC_GET_NORM_FLOAT: _bindgen_ty_2 = 4113;
pub const SFC_SET_NORM_DOUBLE: _bindgen_ty_2 = 4114;
pub const SFC_SET_NORM_FLOAT: _bindgen_ty_2 = 4115;
pub const SFC_GET_SIMPLE_FORMAT_COUNT: _bindgen_ty_2 = 4128;
pub const SFC_GET_SIMPLE_FORMAT: _bindgen_ty_2 = 4129;
pub const SFC_GET_FORMAT_INFO: _bindgen_ty_2 = 4136;
pub const SFC_GET_FORMAT_MAJOR_COUNT: _bindgen_ty_2 = 4144;
pub const SFC_GET_FORMAT_MAJOR: _bindgen_ty_2 = 4145;
pub const SFC_GET_FORMAT_SUBTYPE_COUNT: _bindgen_ty_2 = 4146;
pub const SFC_GET_FORMAT_SUBTYPE: _bindgen_ty_2 = 4147;
pub const SFC_CALC_SIGNAL_MAX: _bindgen_ty_2 = 4160;
pub const SFC_CALC_NORM_SIGNAL_MAX: _bindgen_ty_2 = 4161;
pub const SFC_CALC_MAX_ALL_CHANNELS: _bindgen_ty_2 = 4162;
pub const SFC_CALC_NORM_MAX_ALL_CHANNELS: _bindgen_ty_2 = 4163;
pub const SFC_SET_ADD_PEAK_CHUNK: _bindgen_ty_2 = 4176;
pub const SFC_UPDATE_HEADER_NOW: _bindgen_ty_2 = 4192;
pub const SFC_SET_UPDATE_HEADER_AUTO: _bindgen_ty_2 = 4193;
pub const SFC_FILE_TRUNCATE: _bindgen_ty_2 = 4224;
pub const SFC_SET_RAW_START_OFFSET: _bindgen_ty_2 = 4240;
pub const SFC_SET_DITHER_ON_WRITE: _bindgen_ty_2 = 4256;
pub const SFC_SET_DITHER_ON_READ: _bindgen_ty_2 = 4257;
pub const SFC_GET_DITHER_INFO_COUNT: _bindgen_ty_2 = 4258;
pub const SFC_GET_DITHER_INFO: _bindgen_ty_2 = 4259;
pub const SFC_GET_EMBED_FILE_INFO: _bindgen_ty_2 = 4272;
pub const SFC_SET_CLIPPING: _bindgen_ty_2 = 4288;
pub const SFC_GET_CLIPPING: _bindgen_ty_2 = 4289;
pub const SFC_GET_INSTRUMENT: _bindgen_ty_2 = 4304;
pub const SFC_SET_INSTRUMENT: _bindgen_ty_2 = 4305;
pub const SFC_TEST_IEEE_FLOAT_REPLACE: _bindgen_ty_2 = 24577;
pub const SFC_SET_ADD_DITHER_ON_WRITE: _bindgen_ty_2 = 4208;
pub const SFC_SET_ADD_DITHER_ON_READ: _bindgen_ty_2 = 4209;
pub type _bindgen_ty_2 = u32;
pub const SF_STR_TITLE: _bindgen_ty_3 = 1;
pub const SF_STR_COPYRIGHT: _bindgen_ty_3 = 2;
pub const SF_STR_SOFTWARE: _bindgen_ty_3 = 3;
pub const SF_STR_ARTIST: _bindgen_ty_3 = 4;
pub const SF_STR_COMMENT: _bindgen_ty_3 = 5;
pub const SF_STR_DATE: _bindgen_ty_3 = 6;
pub type _bindgen_ty_3 = u32;
pub const SF_FALSE: _bindgen_ty_4 = 0;
pub const SF_TRUE: _bindgen_ty_4 = 1;
pub const SFM_READ: _bindgen_ty_4 = 16;
pub const SFM_WRITE: _bindgen_ty_4 = 32;
pub const SFM_RDWR: _bindgen_ty_4 = 48;
pub type _bindgen_ty_4 = u32;
pub const SF_ERR_NO_ERROR: _bindgen_ty_5 = 0;
pub const SF_ERR_UNRECOGNISED_FORMAT: _bindgen_ty_5 = 1;
pub const SF_ERR_SYSTEM: _bindgen_ty_5 = 2;
pub type _bindgen_ty_5 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SNDFILE_tag {
    _unused: [u8; 0],
}
pub type SNDFILE = SNDFILE_tag;
pub type sf_count_t = loff_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SF_INFO {
    pub frames: sf_count_t,
    pub samplerate: c_int,
    pub channels: c_int,
    pub format: c_int,
    pub sections: c_int,
    pub seekable: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SF_FORMAT_INFO {
    pub format: c_int,
    pub name: *const c_char,
    pub extension: *const c_char,
}
pub const SFD_DEFAULT_LEVEL: _bindgen_ty_6 = 0;
pub const SFD_CUSTOM_LEVEL: _bindgen_ty_6 = 1073741824;
pub const SFD_NO_DITHER: _bindgen_ty_6 = 500;
pub const SFD_WHITE: _bindgen_ty_6 = 501;
pub const SFD_TRIANGULAR_PDF: _bindgen_ty_6 = 502;
pub type _bindgen_ty_6 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SF_DITHER_INFO {
    pub type_: c_int,
    pub level: f64,
    pub name: *const c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SF_EMBED_FILE_INFO {
    pub offset: sf_count_t,
    pub length: sf_count_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SF_INSTRUMENT {
    pub basenote: c_int,
    pub gain: c_int,
    pub sustain_mode: c_int,
    pub sustain_start: c_int,
    pub sustain_end: c_int,
    pub release_mode: c_int,
    pub release_start: c_int,
    pub reslease_end: c_int,
}
pub const SF_LOOP_NONE: _bindgen_ty_7 = 800;
pub const SF_LOOP_FORWARD: _bindgen_ty_7 = 801;
pub const SF_LOOP_BACKWARD: _bindgen_ty_7 = 802;
pub type _bindgen_ty_7 = u32;
extern "C" {
    pub fn sf_open(path: *const c_char, mode: c_int, sfinfo: *mut SF_INFO) -> *mut SNDFILE;
}
extern "C" {
    pub fn sf_open_fd(
        fd: c_int,
        mode: c_int,
        sfinfo: *mut SF_INFO,
        close_desc: c_int,
    ) -> *mut SNDFILE;
}
extern "C" {
    pub fn sf_error(sndfile: *mut SNDFILE) -> c_int;
}
extern "C" {
    pub fn sf_strerror(sndfile: *mut SNDFILE) -> *const c_char;
}
extern "C" {
    pub fn sf_error_number(errnum: c_int) -> *const c_char;
}
extern "C" {
    pub fn sf_perror(sndfile: *mut SNDFILE) -> c_int;
}
extern "C" {
    pub fn sf_error_str(sndfile: *mut SNDFILE, str: *mut c_char, len: usize) -> c_int;
}
extern "C" {
    pub fn sf_command(
        sndfile: *mut SNDFILE,
        command: c_int,
        data: *mut c_void,
        datasize: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn sf_format_check(info: *const SF_INFO) -> c_int;
}
extern "C" {
    pub fn sf_seek(sndfile: *mut SNDFILE, frames: sf_count_t, whence: c_int) -> sf_count_t;
}
extern "C" {
    pub fn sf_set_string(sndfile: *mut SNDFILE, str_type: c_int, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn sf_get_string(sndfile: *mut SNDFILE, str_type: c_int) -> *const c_char;
}
extern "C" {
    pub fn sf_read_raw(sndfile: *mut SNDFILE, ptr: *mut c_void, bytes: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_write_raw(sndfile: *mut SNDFILE, ptr: *mut c_void, bytes: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_readf_short(
        sndfile: *mut SNDFILE,
        ptr: *mut c_short,
        frames: sf_count_t,
    ) -> sf_count_t;
}
extern "C" {
    pub fn sf_writef_short(
        sndfile: *mut SNDFILE,
        ptr: *mut c_short,
        frames: sf_count_t,
    ) -> sf_count_t;
}
extern "C" {
    pub fn sf_readf_int(sndfile: *mut SNDFILE, ptr: *mut c_int, frames: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_writef_int(sndfile: *mut SNDFILE, ptr: *mut c_int, frames: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_readf_float(sndfile: *mut SNDFILE, ptr: *mut f32, frames: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_writef_float(sndfile: *mut SNDFILE, ptr: *mut f32, frames: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_readf_double(sndfile: *mut SNDFILE, ptr: *mut f64, frames: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_writef_double(sndfile: *mut SNDFILE, ptr: *mut f64, frames: sf_count_t)
        -> sf_count_t;
}
extern "C" {
    pub fn sf_read_short(sndfile: *mut SNDFILE, ptr: *mut c_short, items: sf_count_t)
        -> sf_count_t;
}
extern "C" {
    pub fn sf_write_short(
        sndfile: *mut SNDFILE,
        ptr: *mut c_short,
        items: sf_count_t,
    ) -> sf_count_t;
}
extern "C" {
    pub fn sf_read_int(sndfile: *mut SNDFILE, ptr: *mut c_int, items: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_write_int(sndfile: *mut SNDFILE, ptr: *mut c_int, items: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_read_float(sndfile: *mut SNDFILE, ptr: *mut f32, items: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_write_float(sndfile: *mut SNDFILE, ptr: *mut f32, items: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_read_double(sndfile: *mut SNDFILE, ptr: *mut f64, items: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_write_double(sndfile: *mut SNDFILE, ptr: *mut f64, items: sf_count_t) -> sf_count_t;
}
extern "C" {
    pub fn sf_close(sndfile: *mut SNDFILE) -> c_int;
}
pub const SF_PEAK_START: _bindgen_ty_8 = 42;
pub const SF_PEAK_END: _bindgen_ty_8 = 43;
pub const SF_SCALE_MAX: _bindgen_ty_8 = 52;
pub const SF_SCALE_MIN: _bindgen_ty_8 = 53;
pub const SF_STR_ALLOW_START: _bindgen_ty_8 = 256;
pub const SF_STR_ALLOW_END: _bindgen_ty_8 = 512;
pub const SF_STR_LOCATE_START: _bindgen_ty_8 = 1024;
pub const SF_STR_LOCATE_END: _bindgen_ty_8 = 2048;
pub const SFD_TYPEMASK: _bindgen_ty_8 = 268435455;
pub type _bindgen_ty_8 = u32;
pub const SF_FORMAT_WVE: _bindgen_ty_9 = 67239936;
pub const SF_FORMAT_TXW: _bindgen_ty_9 = 67305472;
pub const SF_FORMAT_DWD: _bindgen_ty_9 = 67371008;
pub const SF_FORMAT_OGG: _bindgen_ty_9 = 67698688;
pub const SF_FORMAT_REX: _bindgen_ty_9 = 67764224;
pub const SF_FORMAT_SD2: _bindgen_ty_9 = 67895296;
pub const SF_FORMAT_REX2: _bindgen_ty_9 = 67960832;
pub const SF_FORMAT_KRZ: _bindgen_ty_9 = 68026368;
pub const SF_FORMAT_WMA: _bindgen_ty_9 = 68157440;
pub const SF_FORMAT_SHN: _bindgen_ty_9 = 68222976;
pub const SF_FORMAT_FLAC: _bindgen_ty_9 = 68288512;
pub const SF_FORMAT_VORBIS: _bindgen_ty_9 = 4097;
pub const SF_FORMAT_SVX_FIB: _bindgen_ty_9 = 4128;
pub const SF_FORMAT_SVX_EXP: _bindgen_ty_9 = 4129;
pub const SF_FORMAT_PCM_N: _bindgen_ty_9 = 4144;
pub type _bindgen_ty_9 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PEAK_POS {
    pub value: f32,
    pub position: c_uint,
}
#[repr(C)]
#[derive(Debug)]
pub struct PEAK_CHUNK {
    pub version: c_uint,
    pub timestamp: c_uint,
    pub peaks: __IncompleteArrayField<PEAK_POS>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STR_DATA {
    pub type_: c_int,
    pub flags: c_int,
    pub str: *mut c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sf_private_tag {
    pub buffer: [f64; 2048usize],
    pub filename: [c_char; 256usize],
    pub syserr: [c_char; 256usize],
    pub logbuffer: [c_char; 16384usize],
    pub header: [c_uchar; 4096usize],
    pub rwf_endian: c_int,
    pub strings: [STR_DATA; 16usize],
    pub str_storage: [c_char; 16384usize],
    pub str_end: *mut c_char,
    pub str_flags: c_int,
    pub Magick: c_int,
    pub logindex: c_int,
    pub headindex: c_int,
    pub headend: c_int,
    pub has_text: c_int,
    pub do_not_close_descriptor: c_int,
    pub filedes: c_int,
    pub end_of_file: c_int,
    pub error: c_int,
    pub mode: c_int,
    pub endian: c_int,
    pub float_endswap: c_int,
    pub is_pipe: c_int,
    pub pipeoffset: sf_count_t,
    pub add_clipping: c_int,
    pub sf: SF_INFO,
    pub have_written: c_int,
    pub has_peak: c_int,
    pub peak_loc: c_int,
    pub pchunk: *mut PEAK_CHUNK,
    pub filelength: sf_count_t,
    pub fileoffset: sf_count_t,
    pub dataoffset: sf_count_t,
    pub datalength: sf_count_t,
    pub dataend: sf_count_t,
    pub blockwidth: c_int,
    pub bytewidth: c_int,
    pub dither: *mut c_void,
    pub interleave: *mut c_void,
    pub last_op: c_int,
    pub read_current: sf_count_t,
    pub write_current: sf_count_t,
    pub fdata: *mut c_void,
    pub write_dither: SF_DITHER_INFO,
    pub read_dither: SF_DITHER_INFO,
    pub norm_double: c_int,
    pub norm_float: c_int,
    pub auto_header: c_int,
    pub ieee_replace: c_int,
    pub read_short: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            ptr: *mut c_short,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub read_int: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            ptr: *mut c_int,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub read_float: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            ptr: *mut f32,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub read_double: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            ptr: *mut f64,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_short: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            ptr: *mut c_short,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_int: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            ptr: *mut c_int,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_float: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            ptr: *mut f32,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_double: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            ptr: *mut f64,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub seek: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            mode: c_int,
            samples_from_start: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_header:
        Option<unsafe extern "C" fn(arg1: *mut sf_private_tag, calc_length: c_int) -> c_int>,
    pub command: Option<
        unsafe extern "C" fn(
            arg1: *mut sf_private_tag,
            command: c_int,
            data: *mut c_void,
            datasize: c_int,
        ) -> c_int,
    >,
    pub close: Option<unsafe extern "C" fn(arg1: *mut sf_private_tag) -> c_int>,
    pub format_desc: *mut c_char,
}
pub type SF_PRIVATE = sf_private_tag;
pub const SFE_NO_ERROR: _bindgen_ty_10 = 0;
pub const SFE_BAD_OPEN_FORMAT: _bindgen_ty_10 = 1;
pub const SFE_SYSTEM: _bindgen_ty_10 = 2;
pub const SFE_BAD_FILE: _bindgen_ty_10 = 3;
pub const SFE_BAD_FILE_READ: _bindgen_ty_10 = 4;
pub const SFE_OPEN_FAILED: _bindgen_ty_10 = 5;
pub const SFE_BAD_SNDFILE_PTR: _bindgen_ty_10 = 6;
pub const SFE_BAD_SF_INFO_PTR: _bindgen_ty_10 = 7;
pub const SFE_BAD_SF_INCOMPLETE: _bindgen_ty_10 = 8;
pub const SFE_BAD_FILE_PTR: _bindgen_ty_10 = 9;
pub const SFE_BAD_INT_PTR: _bindgen_ty_10 = 10;
pub const SFE_BAD_STAT_SIZE: _bindgen_ty_10 = 11;
pub const SFE_MALLOC_FAILED: _bindgen_ty_10 = 12;
pub const SFE_UNIMPLEMENTED: _bindgen_ty_10 = 13;
pub const SFE_BAD_READ_ALIGN: _bindgen_ty_10 = 14;
pub const SFE_BAD_WRITE_ALIGN: _bindgen_ty_10 = 15;
pub const SFE_UNKNOWN_FORMAT: _bindgen_ty_10 = 16;
pub const SFE_NOT_READMODE: _bindgen_ty_10 = 17;
pub const SFE_NOT_WRITEMODE: _bindgen_ty_10 = 18;
pub const SFE_BAD_MODE_RW: _bindgen_ty_10 = 19;
pub const SFE_BAD_SF_INFO: _bindgen_ty_10 = 20;
pub const SFE_BAD_OFFSET: _bindgen_ty_10 = 21;
pub const SFE_NO_EMBED_SUPPORT: _bindgen_ty_10 = 22;
pub const SFE_NO_EMBEDDED_RDWR: _bindgen_ty_10 = 23;
pub const SFE_NO_PIPE_WRITE: _bindgen_ty_10 = 24;
pub const SFE_INTERNAL: _bindgen_ty_10 = 25;
pub const SFE_LOG_OVERRUN: _bindgen_ty_10 = 26;
pub const SFE_BAD_CONTROL_CMD: _bindgen_ty_10 = 27;
pub const SFE_BAD_ENDIAN: _bindgen_ty_10 = 28;
pub const SFE_CHANNEL_COUNT: _bindgen_ty_10 = 29;
pub const SFE_BAD_RDWR_FORMAT: _bindgen_ty_10 = 30;
pub const SFE_INTERLEAVE_MODE: _bindgen_ty_10 = 31;
pub const SFE_INTERLEAVE_SEEK: _bindgen_ty_10 = 32;
pub const SFE_INTERLEAVE_READ: _bindgen_ty_10 = 33;
pub const SFE_BAD_SEEK: _bindgen_ty_10 = 34;
pub const SFE_NOT_SEEKABLE: _bindgen_ty_10 = 35;
pub const SFE_AMBIGUOUS_SEEK: _bindgen_ty_10 = 36;
pub const SFE_WRONG_SEEK: _bindgen_ty_10 = 37;
pub const SFE_SEEK_FAILED: _bindgen_ty_10 = 38;
pub const SFE_BAD_OPEN_MODE: _bindgen_ty_10 = 39;
pub const SFE_OPEN_PIPE_RDWR: _bindgen_ty_10 = 40;
pub const SFE_RDWR_POSITION: _bindgen_ty_10 = 41;
pub const SFE_STR_NO_SUPPORT: _bindgen_ty_10 = 42;
pub const SFE_STR_MAX_DATA: _bindgen_ty_10 = 43;
pub const SFE_STR_MAX_COUNT: _bindgen_ty_10 = 44;
pub const SFE_STR_BAD_TYPE: _bindgen_ty_10 = 45;
pub const SFE_STR_NO_ADD_END: _bindgen_ty_10 = 46;
pub const SFE_STR_BAD_STRING: _bindgen_ty_10 = 47;
pub const SFE_STR_WEIRD: _bindgen_ty_10 = 48;
pub const SFE_RDWR_BAD_HEADER: _bindgen_ty_10 = 49;
pub const SFE_WAV_NO_RIFF: _bindgen_ty_10 = 50;
pub const SFE_WAV_NO_WAVE: _bindgen_ty_10 = 51;
pub const SFE_WAV_NO_FMT: _bindgen_ty_10 = 52;
pub const SFE_WAV_FMT_SHORT: _bindgen_ty_10 = 53;
pub const SFE_WAV_FMT_TOO_BIG: _bindgen_ty_10 = 54;
pub const SFE_WAV_BAD_FACT: _bindgen_ty_10 = 55;
pub const SFE_WAV_BAD_PEAK: _bindgen_ty_10 = 56;
pub const SFE_WAV_PEAK_B4_FMT: _bindgen_ty_10 = 57;
pub const SFE_WAV_BAD_FORMAT: _bindgen_ty_10 = 58;
pub const SFE_WAV_BAD_BLOCKALIGN: _bindgen_ty_10 = 59;
pub const SFE_WAV_NO_DATA: _bindgen_ty_10 = 60;
pub const SFE_WAV_ADPCM_NOT4BIT: _bindgen_ty_10 = 61;
pub const SFE_WAV_ADPCM_CHANNELS: _bindgen_ty_10 = 62;
pub const SFE_WAV_GSM610_FORMAT: _bindgen_ty_10 = 63;
pub const SFE_WAV_UNKNOWN_CHUNK: _bindgen_ty_10 = 64;
pub const SFE_WAV_WVPK_DATA: _bindgen_ty_10 = 65;
pub const SFE_AIFF_NO_FORM: _bindgen_ty_10 = 66;
pub const SFE_AIFF_AIFF_NO_FORM: _bindgen_ty_10 = 67;
pub const SFE_AIFF_COMM_NO_FORM: _bindgen_ty_10 = 68;
pub const SFE_AIFF_SSND_NO_COMM: _bindgen_ty_10 = 69;
pub const SFE_AIFF_UNKNOWN_CHUNK: _bindgen_ty_10 = 70;
pub const SFE_AIFF_COMM_CHUNK_SIZE: _bindgen_ty_10 = 71;
pub const SFE_AIFF_BAD_COMM_CHUNK: _bindgen_ty_10 = 72;
pub const SFE_AIFF_PEAK_B4_COMM: _bindgen_ty_10 = 73;
pub const SFE_AIFF_BAD_PEAK: _bindgen_ty_10 = 74;
pub const SFE_AIFF_NO_SSND: _bindgen_ty_10 = 75;
pub const SFE_AIFF_NO_DATA: _bindgen_ty_10 = 76;
pub const SFE_AIFF_RW_SSND_NOT_LAST: _bindgen_ty_10 = 77;
pub const SFE_AU_UNKNOWN_FORMAT: _bindgen_ty_10 = 78;
pub const SFE_AU_NO_DOTSND: _bindgen_ty_10 = 79;
pub const SFE_AU_EMBED_BAD_LEN: _bindgen_ty_10 = 80;
pub const SFE_RAW_READ_BAD_SPEC: _bindgen_ty_10 = 81;
pub const SFE_RAW_BAD_BITWIDTH: _bindgen_ty_10 = 82;
pub const SFE_RAW_BAD_FORMAT: _bindgen_ty_10 = 83;
pub const SFE_PAF_NO_MARKER: _bindgen_ty_10 = 84;
pub const SFE_PAF_VERSION: _bindgen_ty_10 = 85;
pub const SFE_PAF_UNKNOWN_FORMAT: _bindgen_ty_10 = 86;
pub const SFE_PAF_SHORT_HEADER: _bindgen_ty_10 = 87;
pub const SFE_SVX_NO_FORM: _bindgen_ty_10 = 88;
pub const SFE_SVX_NO_BODY: _bindgen_ty_10 = 89;
pub const SFE_SVX_NO_DATA: _bindgen_ty_10 = 90;
pub const SFE_SVX_BAD_COMP: _bindgen_ty_10 = 91;
pub const SFE_SVX_BAD_NAME_LENGTH: _bindgen_ty_10 = 92;
pub const SFE_NIST_BAD_HEADER: _bindgen_ty_10 = 93;
pub const SFE_NIST_CRLF_CONVERISON: _bindgen_ty_10 = 94;
pub const SFE_NIST_BAD_ENCODING: _bindgen_ty_10 = 95;
pub const SFE_VOC_NO_CREATIVE: _bindgen_ty_10 = 96;
pub const SFE_VOC_BAD_FORMAT: _bindgen_ty_10 = 97;
pub const SFE_VOC_BAD_VERSION: _bindgen_ty_10 = 98;
pub const SFE_VOC_BAD_MARKER: _bindgen_ty_10 = 99;
pub const SFE_VOC_BAD_SECTIONS: _bindgen_ty_10 = 100;
pub const SFE_VOC_MULTI_SAMPLERATE: _bindgen_ty_10 = 101;
pub const SFE_VOC_MULTI_SECTION: _bindgen_ty_10 = 102;
pub const SFE_VOC_MULTI_PARAM: _bindgen_ty_10 = 103;
pub const SFE_VOC_SECTION_COUNT: _bindgen_ty_10 = 104;
pub const SFE_VOC_NO_PIPE: _bindgen_ty_10 = 105;
pub const SFE_IRCAM_NO_MARKER: _bindgen_ty_10 = 106;
pub const SFE_IRCAM_BAD_CHANNELS: _bindgen_ty_10 = 107;
pub const SFE_IRCAM_UNKNOWN_FORMAT: _bindgen_ty_10 = 108;
pub const SFE_W64_64_BIT: _bindgen_ty_10 = 109;
pub const SFE_W64_NO_RIFF: _bindgen_ty_10 = 110;
pub const SFE_W64_NO_WAVE: _bindgen_ty_10 = 111;
pub const SFE_W64_NO_FMT: _bindgen_ty_10 = 112;
pub const SFE_W64_NO_DATA: _bindgen_ty_10 = 113;
pub const SFE_W64_FMT_SHORT: _bindgen_ty_10 = 114;
pub const SFE_W64_FMT_TOO_BIG: _bindgen_ty_10 = 115;
pub const SFE_W64_ADPCM_NOT4BIT: _bindgen_ty_10 = 116;
pub const SFE_W64_ADPCM_CHANNELS: _bindgen_ty_10 = 117;
pub const SFE_W64_GSM610_FORMAT: _bindgen_ty_10 = 118;
pub const SFE_MAT4_BAD_NAME: _bindgen_ty_10 = 119;
pub const SFE_MAT4_NO_SAMPLERATE: _bindgen_ty_10 = 120;
pub const SFE_MAT4_ZERO_CHANNELS: _bindgen_ty_10 = 121;
pub const SFE_MAT5_BAD_ENDIAN: _bindgen_ty_10 = 122;
pub const SFE_MAT5_NO_BLOCK: _bindgen_ty_10 = 123;
pub const SFE_MAT5_SAMPLE_RATE: _bindgen_ty_10 = 124;
pub const SFE_MAT5_ZERO_CHANNELS: _bindgen_ty_10 = 125;
pub const SFE_PVF_NO_PVF1: _bindgen_ty_10 = 126;
pub const SFE_PVF_BAD_HEADER: _bindgen_ty_10 = 127;
pub const SFE_PVF_BAD_BITWIDTH: _bindgen_ty_10 = 128;
pub const SFE_DWVW_BAD_BITWIDTH: _bindgen_ty_10 = 129;
pub const SFE_G72X_NOT_MONO: _bindgen_ty_10 = 130;
pub const SFE_XI_BAD_HEADER: _bindgen_ty_10 = 131;
pub const SFE_XI_EXCESS_SAMPLES: _bindgen_ty_10 = 132;
pub const SFE_XI_NO_PIPE: _bindgen_ty_10 = 133;
pub const SFE_HTK_NO_PIPE: _bindgen_ty_10 = 134;
pub const SFE_SDS_NOT_SDS: _bindgen_ty_10 = 135;
pub const SFE_SDS_BAD_BIT_WIDTH: _bindgen_ty_10 = 136;
pub const SFE_MAX_ERROR: _bindgen_ty_10 = 137;
pub type _bindgen_ty_10 = u32;
extern "C" {
    pub fn subformat_to_bytewidth(format: c_int) -> c_int;
}
extern "C" {
    pub fn s_bitwidth_to_subformat(bits: c_int) -> c_int;
}
extern "C" {
    pub fn u_bitwidth_to_subformat(bits: c_int) -> c_int;
}
extern "C" {
    pub fn float32_be_read(cptr: *mut c_uchar) -> f32;
}
extern "C" {
    pub fn float32_le_read(cptr: *mut c_uchar) -> f32;
}
extern "C" {
    pub fn float32_be_write(in_: f32, out: *mut c_uchar);
}
extern "C" {
    pub fn float32_le_write(in_: f32, out: *mut c_uchar);
}
extern "C" {
    pub fn double64_be_read(cptr: *mut c_uchar) -> f64;
}
extern "C" {
    pub fn double64_le_read(cptr: *mut c_uchar) -> f64;
}
extern "C" {
    pub fn double64_be_write(in_: f64, out: *mut c_uchar);
}
extern "C" {
    pub fn double64_le_write(in_: f64, out: *mut c_uchar);
}
extern "C" {
    pub fn psf_log_printf(psf: *mut SF_PRIVATE, format: *const c_char, ...);
}
extern "C" {
    pub fn psf_log_SF_INFO(psf: *mut SF_PRIVATE);
}
extern "C" {
    pub fn psf_hexdump(ptr: *mut c_void, len: c_int);
}
extern "C" {
    pub fn psf_binheader_writef(psf: *mut SF_PRIVATE, format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn psf_asciiheader_printf(psf: *mut SF_PRIVATE, format: *const c_char, ...);
}
extern "C" {
    pub fn psf_binheader_readf(psf: *mut SF_PRIVATE, format: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn peak_update_short(psf: *mut SF_PRIVATE, ptr: *mut c_short, items: usize);
}
extern "C" {
    pub fn peak_update_int(psf: *mut SF_PRIVATE, ptr: *mut c_int, items: usize);
}
extern "C" {
    pub fn peak_update_double(psf: *mut SF_PRIVATE, ptr: *mut f64, items: usize);
}
extern "C" {
    pub fn psf_get_format_simple_count() -> c_int;
}
extern "C" {
    pub fn psf_get_format_simple(data: *mut SF_FORMAT_INFO) -> c_int;
}
extern "C" {
    pub fn psf_get_format_info(data: *mut SF_FORMAT_INFO) -> c_int;
}
extern "C" {
    pub fn psf_get_format_major_count() -> c_int;
}
extern "C" {
    pub fn psf_get_format_major(data: *mut SF_FORMAT_INFO) -> c_int;
}
extern "C" {
    pub fn psf_get_format_subtype_count() -> c_int;
}
extern "C" {
    pub fn psf_get_format_subtype(data: *mut SF_FORMAT_INFO) -> c_int;
}
extern "C" {
    pub fn psf_generate_format_desc(psf: *mut SF_PRIVATE);
}
extern "C" {
    pub fn psf_calc_signal_max(psf: *mut SF_PRIVATE, normalize: c_int) -> f64;
}
extern "C" {
    pub fn psf_calc_max_all_channels(
        psf: *mut SF_PRIVATE,
        peaks: *mut f64,
        normalize: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn psf_get_string(psf: *mut SF_PRIVATE, str_type: c_int) -> *const c_char;
}
extern "C" {
    pub fn psf_store_string(psf: *mut SF_PRIVATE, str_type: c_int, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn psf_default_seek(
        psf: *mut SF_PRIVATE,
        mode: c_int,
        samples_from_start: sf_count_t,
    ) -> sf_count_t;
}
extern "C" {
    pub fn psf_get_date_str(str: *mut c_char, maxlen: c_int);
}
extern "C" {
    pub fn macos_guess_file_type(psf: *mut SF_PRIVATE, filename: *const c_char) -> c_int;
}
extern "C" {
    pub fn psf_fopen(psf: *mut SF_PRIVATE, pathname: *const c_char, flags: c_int) -> c_int;
}
extern "C" {
    pub fn psf_set_stdio(psf: *mut SF_PRIVATE, mode: c_int) -> c_int;
}
extern "C" {
    pub fn psf_filedes_valid(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn psf_set_file(psf: *mut SF_PRIVATE, fd: c_int);
}
extern "C" {
    pub fn psf_fseek(psf: *mut SF_PRIVATE, offset: sf_count_t, whence: c_int) -> sf_count_t;
}
extern "C" {
    pub fn psf_fread(
        ptr: *mut c_void,
        bytes: sf_count_t,
        count: sf_count_t,
        psf: *mut SF_PRIVATE,
    ) -> sf_count_t;
}
extern "C" {
    pub fn psf_fwrite(
        ptr: *mut c_void,
        bytes: sf_count_t,
        count: sf_count_t,
        psf: *mut SF_PRIVATE,
    ) -> sf_count_t;
}
extern "C" {
    pub fn psf_fgets(buffer: *mut c_char, bufsize: sf_count_t, psf: *mut SF_PRIVATE) -> sf_count_t;
}
extern "C" {
    pub fn psf_ftell(psf: *mut SF_PRIVATE) -> sf_count_t;
}
extern "C" {
    pub fn psf_get_filelen(psf: *mut SF_PRIVATE) -> sf_count_t;
}
extern "C" {
    pub fn psf_is_pipe(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn psf_ftruncate(psf: *mut SF_PRIVATE, len: sf_count_t) -> c_int;
}
extern "C" {
    pub fn psf_fclose(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn aiff_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn au_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn au_nh_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn avr_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn htk_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn ircam_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn mat4_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn mat5_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn nist_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn paf_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn pvf_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn raw_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn sds_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn svx_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn voc_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn w64_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn wav_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn xi_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn ogg_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn rx2_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn sd2_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn txw_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn wve_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn dwd_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn macbinary3_open(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn pcm_init(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn ulaw_init(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn alaw_init(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn float32_init(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn double64_init(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn dwvw_init(psf: *mut SF_PRIVATE, bitwidth: c_int) -> c_int;
}
extern "C" {
    pub fn gsm610_init(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn vox_adpcm_init(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn dither_init(psf: *mut SF_PRIVATE, mode: c_int) -> c_int;
}
extern "C" {
    pub fn wav_w64_ima_init(
        psf: *mut SF_PRIVATE,
        blockalign: c_int,
        samplesperblock: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn wav_w64_msadpcm_init(
        psf: *mut SF_PRIVATE,
        blockalign: c_int,
        samplesperblock: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn aiff_ima_init(psf: *mut SF_PRIVATE, blockalign: c_int, samplesperblock: c_int) -> c_int;
}
extern "C" {
    pub fn interleave_init(psf: *mut SF_PRIVATE) -> c_int;
}
extern "C" {
    pub fn psf_memset(s: *mut c_void, c: c_int, n: sf_count_t) -> *mut c_void;
}
pub const SFC_TEST_AIFF_ADD_INST_CHUNK: _bindgen_ty_11 = 8192;
pub const SFC_TEST_WAV_ADD_INFO_CHUNK: _bindgen_ty_11 = 8208;
pub type _bindgen_ty_11 = u32;
pub const G723_16_BITS_PER_SAMPLE: _bindgen_ty_12 = 2;
pub const G723_24_BITS_PER_SAMPLE: _bindgen_ty_12 = 3;
pub const G723_40_BITS_PER_SAMPLE: _bindgen_ty_12 = 5;
pub const G721_32_BITS_PER_SAMPLE: _bindgen_ty_12 = 4;
pub const G721_40_BITS_PER_SAMPLE: _bindgen_ty_12 = 5;
pub const G723_16_SAMPLES_PER_BLOCK: _bindgen_ty_12 = 120;
pub const G723_24_SAMPLES_PER_BLOCK: _bindgen_ty_12 = 120;
pub const G723_40_SAMPLES_PER_BLOCK: _bindgen_ty_12 = 120;
pub const G721_32_SAMPLES_PER_BLOCK: _bindgen_ty_12 = 120;
pub const G721_40_SAMPLES_PER_BLOCK: _bindgen_ty_12 = 120;
pub const G723_16_BYTES_PER_BLOCK: _bindgen_ty_12 = 30;
pub const G723_24_BYTES_PER_BLOCK: _bindgen_ty_12 = 45;
pub const G723_40_BYTES_PER_BLOCK: _bindgen_ty_12 = 75;
pub const G721_32_BYTES_PER_BLOCK: _bindgen_ty_12 = 60;
pub const G721_40_BYTES_PER_BLOCK: _bindgen_ty_12 = 75;
pub type _bindgen_ty_12 = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct G72x_DATA {
    pub sprivateo: [c_ulong; 32usize],
    pub blocksize: c_int,
    pub max_bytes: c_int,
    pub samplesperblock: c_int,
    pub bytesperblock: c_int,
    pub blocks: c_int,
    pub blockcount: c_int,
    pub samplecount: c_int,
    pub block: [c_uchar; 120usize],
    pub samples: [c_short; 120usize],
}
extern "C" {
    pub fn g72x_reader_init(data: *mut G72x_DATA, codec: c_int) -> c_int;
}
extern "C" {
    pub fn g72x_writer_init(data: *mut G72x_DATA, codec: c_int) -> c_int;
}
extern "C" {
    pub fn g72x_decode_block(data: *mut G72x_DATA) -> c_int;
}
extern "C" {
    pub fn g72x_encode_block(data: *mut G72x_DATA) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct private_g72x {
    pub yl: c_long,
    pub yu: c_short,
    pub dms: c_short,
    pub dml: c_short,
    pub ap: c_short,
    pub a: [c_short; 2usize],
    pub b: [c_short; 6usize],
    pub pk: [c_short; 2usize],
    pub dq: [c_short; 6usize],
    pub sr: [c_short; 2usize],
    pub td: c_char,
    pub encoder: Option<unsafe extern "C" fn(arg1: c_int, state: *mut private_g72x) -> c_int>,
    pub decoder: Option<unsafe extern "C" fn(arg1: c_int, state: *mut private_g72x) -> c_int>,
    pub codec_bits: c_int,
    pub byte_index: c_int,
    pub sample_index: c_int,
}
pub type G72x_STATE = private_g72x;
extern "C" {
    pub fn predictor_zero(state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn predictor_pole(state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn step_size(state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn quantize(d: c_int, y: c_int, table: *mut c_short, size: c_int) -> c_int;
}
extern "C" {
    pub fn reconstruct(sign: c_int, dqln: c_int, y: c_int) -> c_int;
}
extern "C" {
    pub fn update(
        code_size: c_int,
        y: c_int,
        wi: c_int,
        fi: c_int,
        dq: c_int,
        sr: c_int,
        dqsez: c_int,
        state_ptr: *mut G72x_STATE,
    );
}
extern "C" {
    pub fn g721_encoder(sample: c_int, state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn g721_decoder(code: c_int, state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn g723_16_encoder(sample: c_int, state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn g723_16_decoder(code: c_int, state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn g723_24_encoder(sample: c_int, state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn g723_24_decoder(code: c_int, state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn g723_40_encoder(sample: c_int, state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn g723_40_decoder(code: c_int, state_ptr: *mut G72x_STATE) -> c_int;
}
extern "C" {
    pub fn unpack_bytes(data: *mut G72x_DATA, bits: c_int) -> c_int;
}
extern "C" {
    pub fn pack_bytes(data: *mut G72x_DATA, bits: c_int) -> c_int;
}
extern "C" {
    pub fn private_init_state(state_ptr: *mut G72x_STATE);
}
extern "C" {
    pub fn endswap_short_array(ptr: *mut c_short, len: c_int);
}
extern "C" {
    pub fn endswap_int_array(ptr: *mut c_int, len: c_int);
}
extern "C" {
    pub fn endswap_long_array(ptr: *mut c_long, len: c_int);
}
extern "C" {
    pub fn endswap_short_copy(dest: *mut c_short, src: *mut c_short, len: c_int);
}
extern "C" {
    pub fn endswap_int_copy(dest: *mut c_int, src: *mut c_int, len: c_int);
}
extern "C" {
    pub fn endswap_long_copy(dest: *mut c_long, src: *mut c_long, len: c_int);
}
pub type gsm = *mut gsm_state;
pub type gsm_signal = c_short;
pub type gsm_byte = c_uchar;
pub type gsm_frame = [gsm_byte; 33usize];
extern "C" {
    pub fn gsm_create() -> gsm;
}
extern "C" {
    pub fn gsm_init(arg1: gsm);
}
extern "C" {
    pub fn gsm_destroy(arg1: gsm);
}
extern "C" {
    pub fn gsm_print(arg1: *mut FILE, arg2: gsm, arg3: *mut gsm_byte) -> c_int;
}
extern "C" {
    pub fn gsm_option(arg1: gsm, arg2: c_int, arg3: *mut c_int) -> c_int;
}
extern "C" {
    pub fn gsm_encode(arg1: gsm, arg2: *mut gsm_signal, arg3: *mut gsm_byte);
}
extern "C" {
    pub fn gsm_decode(arg1: gsm, arg2: *mut gsm_byte, arg3: *mut gsm_signal) -> c_int;
}
extern "C" {
    pub fn gsm_explode(arg1: gsm, arg2: *mut gsm_byte, arg3: *mut gsm_signal) -> c_int;
}
extern "C" {
    pub fn gsm_implode(arg1: gsm, arg2: *mut gsm_signal, arg3: *mut gsm_byte);
}
pub const AU_H_G721_32: _bindgen_ty_13 = 200;
pub const AU_H_G723_24: _bindgen_ty_13 = 201;
pub const AU_H_G723_40: _bindgen_ty_13 = 202;
pub type _bindgen_ty_13 = u32;
extern "C" {
    pub fn au_g72x_reader_init(psf: *mut SF_PRIVATE, codec: c_int) -> c_int;
}
extern "C" {
    pub fn au_g72x_writer_init(psf: *mut SF_PRIVATE, codec: c_int) -> c_int;
}
pub const WAVE_FORMAT_UNKNOWN: _bindgen_ty_14 = 0;
pub const WAVE_FORMAT_PCM: _bindgen_ty_14 = 1;
pub const WAVE_FORMAT_MS_ADPCM: _bindgen_ty_14 = 2;
pub const WAVE_FORMAT_IEEE_FLOAT: _bindgen_ty_14 = 3;
pub const WAVE_FORMAT_VSELP: _bindgen_ty_14 = 4;
pub const WAVE_FORMAT_IBM_CVSD: _bindgen_ty_14 = 5;
pub const WAVE_FORMAT_ALAW: _bindgen_ty_14 = 6;
pub const WAVE_FORMAT_MULAW: _bindgen_ty_14 = 7;
pub const WAVE_FORMAT_OKI_ADPCM: _bindgen_ty_14 = 16;
pub const WAVE_FORMAT_IMA_ADPCM: _bindgen_ty_14 = 17;
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: _bindgen_ty_14 = 18;
pub const WAVE_FORMAT_SIERRA_ADPCM: _bindgen_ty_14 = 19;
pub const WAVE_FORMAT_G723_ADPCM: _bindgen_ty_14 = 20;
pub const WAVE_FORMAT_DIGISTD: _bindgen_ty_14 = 21;
pub const WAVE_FORMAT_DIGIFIX: _bindgen_ty_14 = 22;
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: _bindgen_ty_14 = 23;
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: _bindgen_ty_14 = 24;
pub const WAVE_FORMAT_CU_CODEC: _bindgen_ty_14 = 25;
pub const WAVE_FORMAT_YAMAHA_ADPCM: _bindgen_ty_14 = 32;
pub const WAVE_FORMAT_SONARC: _bindgen_ty_14 = 33;
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: _bindgen_ty_14 = 34;
pub const WAVE_FORMAT_ECHOSC1: _bindgen_ty_14 = 35;
pub const WAVE_FORMAT_AUDIOFILE_AF36: _bindgen_ty_14 = 36;
pub const WAVE_FORMAT_APTX: _bindgen_ty_14 = 37;
pub const WAVE_FORMAT_AUDIOFILE_AF10: _bindgen_ty_14 = 38;
pub const WAVE_FORMAT_PROSODY_1612: _bindgen_ty_14 = 39;
pub const WAVE_FORMAT_LRC: _bindgen_ty_14 = 40;
pub const WAVE_FORMAT_DOLBY_AC2: _bindgen_ty_14 = 48;
pub const WAVE_FORMAT_GSM610: _bindgen_ty_14 = 49;
pub const WAVE_FORMAT_MSNAUDIO: _bindgen_ty_14 = 50;
pub const WAVE_FORMAT_ANTEX_ADPCME: _bindgen_ty_14 = 51;
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: _bindgen_ty_14 = 52;
pub const WAVE_FORMAT_DIGIREAL: _bindgen_ty_14 = 53;
pub const WAVE_FORMAT_DIGIADPCM: _bindgen_ty_14 = 54;
pub const WAVE_FORMAT_CONTROL_RES_CR10: _bindgen_ty_14 = 55;
pub const WAVE_FORMAT_NMS_VBXADPCM: _bindgen_ty_14 = 56;
pub const WAVE_FORMAT_ROLAND_RDAC: _bindgen_ty_14 = 57;
pub const WAVE_FORMAT_ECHOSC3: _bindgen_ty_14 = 58;
pub const WAVE_FORMAT_ROCKWELL_ADPCM: _bindgen_ty_14 = 59;
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: _bindgen_ty_14 = 60;
pub const WAVE_FORMAT_XEBEC: _bindgen_ty_14 = 61;
pub const WAVE_FORMAT_G721_ADPCM: _bindgen_ty_14 = 64;
pub const WAVE_FORMAT_G728_CELP: _bindgen_ty_14 = 65;
pub const WAVE_FORMAT_MSG723: _bindgen_ty_14 = 66;
pub const WAVE_FORMAT_MPEG: _bindgen_ty_14 = 80;
pub const WAVE_FORMAT_RT24: _bindgen_ty_14 = 82;
pub const WAVE_FORMAT_PAC: _bindgen_ty_14 = 83;
pub const WAVE_FORMAT_MPEGLAYER3: _bindgen_ty_14 = 85;
pub const WAVE_FORMAT_LUCENT_G723: _bindgen_ty_14 = 89;
pub const WAVE_FORMAT_CIRRUS: _bindgen_ty_14 = 96;
pub const WAVE_FORMAT_ESPCM: _bindgen_ty_14 = 97;
pub const WAVE_FORMAT_VOXWARE: _bindgen_ty_14 = 98;
pub const WAVE_FORMAT_CANOPUS_ATRAC: _bindgen_ty_14 = 99;
pub const WAVE_FORMAT_G726_ADPCM: _bindgen_ty_14 = 100;
pub const WAVE_FORMAT_G722_ADPCM: _bindgen_ty_14 = 101;
pub const WAVE_FORMAT_DSAT: _bindgen_ty_14 = 102;
pub const WAVE_FORMAT_DSAT_DISPLAY: _bindgen_ty_14 = 103;
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: _bindgen_ty_14 = 105;
pub const WAVE_FORMAT_VOXWARE_AC8: _bindgen_ty_14 = 112;
pub const WAVE_FORMAT_VOXWARE_AC10: _bindgen_ty_14 = 113;
pub const WAVE_FORMAT_VOXWARE_AC16: _bindgen_ty_14 = 114;
pub const WAVE_FORMAT_VOXWARE_AC20: _bindgen_ty_14 = 115;
pub const WAVE_FORMAT_VOXWARE_RT24: _bindgen_ty_14 = 116;
pub const WAVE_FORMAT_VOXWARE_RT29: _bindgen_ty_14 = 117;
pub const WAVE_FORMAT_VOXWARE_RT29HW: _bindgen_ty_14 = 118;
pub const WAVE_FORMAT_VOXWARE_VR12: _bindgen_ty_14 = 119;
pub const WAVE_FORMAT_VOXWARE_VR18: _bindgen_ty_14 = 120;
pub const WAVE_FORMAT_VOXWARE_TQ40: _bindgen_ty_14 = 121;
pub const WAVE_FORMAT_SOFTSOUND: _bindgen_ty_14 = 128;
pub const WAVE_FORMAT_VOXARE_TQ60: _bindgen_ty_14 = 129;
pub const WAVE_FORMAT_MSRT24: _bindgen_ty_14 = 130;
pub const WAVE_FORMAT_G729A: _bindgen_ty_14 = 131;
pub const WAVE_FORMAT_MVI_MV12: _bindgen_ty_14 = 132;
pub const WAVE_FORMAT_DF_G726: _bindgen_ty_14 = 133;
pub const WAVE_FORMAT_DF_GSM610: _bindgen_ty_14 = 134;
pub const WAVE_FORMAT_ONLIVE: _bindgen_ty_14 = 137;
pub const WAVE_FORMAT_SBC24: _bindgen_ty_14 = 145;
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: _bindgen_ty_14 = 146;
pub const WAVE_FORMAT_ZYXEL_ADPCM: _bindgen_ty_14 = 151;
pub const WAVE_FORMAT_PHILIPS_LPCBB: _bindgen_ty_14 = 152;
pub const WAVE_FORMAT_PACKED: _bindgen_ty_14 = 153;
pub const WAVE_FORMAT_RHETOREX_ADPCM: _bindgen_ty_14 = 256;
pub const IBM_FORMAT_MULAW: _bindgen_ty_14 = 257;
pub const IBM_FORMAT_ALAW: _bindgen_ty_14 = 258;
pub const IBM_FORMAT_ADPCM: _bindgen_ty_14 = 259;
pub const WAVE_FORMAT_VIVO_G723: _bindgen_ty_14 = 273;
pub const WAVE_FORMAT_VIVO_SIREN: _bindgen_ty_14 = 274;
pub const WAVE_FORMAT_DIGITAL_G723: _bindgen_ty_14 = 291;
pub const WAVE_FORMAT_CREATIVE_ADPCM: _bindgen_ty_14 = 512;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: _bindgen_ty_14 = 514;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: _bindgen_ty_14 = 515;
pub const WAVE_FORMAT_QUARTERDECK: _bindgen_ty_14 = 544;
pub const WAVE_FORMAT_FM_TOWNS_SND: _bindgen_ty_14 = 768;
pub const WAVE_FORMAT_BZV_DIGITAL: _bindgen_ty_14 = 1024;
pub const WAVE_FORMAT_VME_VMPCM: _bindgen_ty_14 = 1664;
pub const WAVE_FORMAT_OLIGSM: _bindgen_ty_14 = 4096;
pub const WAVE_FORMAT_OLIADPCM: _bindgen_ty_14 = 4097;
pub const WAVE_FORMAT_OLICELP: _bindgen_ty_14 = 4098;
pub const WAVE_FORMAT_OLISBC: _bindgen_ty_14 = 4099;
pub const WAVE_FORMAT_OLIOPR: _bindgen_ty_14 = 4100;
pub const WAVE_FORMAT_LH_CODEC: _bindgen_ty_14 = 4352;
pub const WAVE_FORMAT_NORRIS: _bindgen_ty_14 = 5120;
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: _bindgen_ty_14 = 5376;
pub const WAVE_FORMAT_DVM: _bindgen_ty_14 = 8192;
pub const WAVE_FORMAT_INTERWAV_VSC112: _bindgen_ty_14 = 29008;
pub const WAVE_FORMAT_EXTENSIBLE: _bindgen_ty_14 = 65534;
pub type _bindgen_ty_14 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIN_WAV_FMT {
    pub format: c_ushort,
    pub channels: c_ushort,
    pub samplerate: c_uint,
    pub bytespersec: c_uint,
    pub blockalign: c_ushort,
    pub bitwidth: c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WAV_FMT_SIZE20 {
    pub format: c_ushort,
    pub channels: c_ushort,
    pub samplerate: c_uint,
    pub bytespersec: c_uint,
    pub blockalign: c_ushort,
    pub bitwidth: c_ushort,
    pub extrabytes: c_ushort,
    pub dummy: c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MS_ADPCM_WAV_FMT {
    pub format: c_ushort,
    pub channels: c_ushort,
    pub samplerate: c_uint,
    pub bytespersec: c_uint,
    pub blockalign: c_ushort,
    pub bitwidth: c_ushort,
    pub extrabytes: c_ushort,
    pub samplesperblock: c_ushort,
    pub numcoeffs: c_ushort,
    pub coeffs: [MS_ADPCM_WAV_FMT__bindgen_ty_1; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MS_ADPCM_WAV_FMT__bindgen_ty_1 {
    pub coeff1: c_short,
    pub coeff2: c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMA_ADPCM_WAV_FMT {
    pub format: c_ushort,
    pub channels: c_ushort,
    pub samplerate: c_uint,
    pub bytespersec: c_uint,
    pub blockalign: c_ushort,
    pub bitwidth: c_ushort,
    pub extrabytes: c_ushort,
    pub samplesperblock: c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct G72x_ADPCM_WAV_FMT {
    pub format: c_ushort,
    pub channels: c_ushort,
    pub samplerate: c_uint,
    pub bytespersec: c_uint,
    pub blockalign: c_ushort,
    pub bitwidth: c_ushort,
    pub extrabytes: c_ushort,
    pub auxblocksize: c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GSM610_WAV_FMT {
    pub format: c_ushort,
    pub channels: c_ushort,
    pub samplerate: c_uint,
    pub bytespersec: c_uint,
    pub blockalign: c_ushort,
    pub bitwidth: c_ushort,
    pub extrabytes: c_ushort,
    pub samplesperblock: c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EXT_SUBFORMAT {
    pub esf_field1: c_uint,
    pub esf_field2: c_ushort,
    pub esf_field3: c_ushort,
    pub esf_field4: [c_char; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EXTENSIBLE_WAV_FMT {
    pub format: c_ushort,
    pub channels: c_ushort,
    pub samplerate: c_uint,
    pub bytespersec: c_uint,
    pub blockalign: c_ushort,
    pub bitwidth: c_ushort,
    pub extrabytes: c_ushort,
    pub validbits: c_ushort,
    pub channelmask: c_uint,
    pub esf: EXT_SUBFORMAT,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union WAV_FMT {
    pub format: c_ushort,
    pub min: MIN_WAV_FMT,
    pub ima: IMA_ADPCM_WAV_FMT,
    pub msadpcm: MS_ADPCM_WAV_FMT,
    pub g72x: G72x_ADPCM_WAV_FMT,
    pub ext: EXTENSIBLE_WAV_FMT,
    pub gsm610: GSM610_WAV_FMT,
    pub size20: WAV_FMT_SIZE20,
    pub padding: [c_char; 512usize],
    _bindgen_union_align: [u32; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FACT_CHUNK {
    pub frames: c_int,
}
extern "C" {
    pub fn msadpcm_write_adapt_coeffs(psf: *mut SF_PRIVATE);
}
extern "C" {
    pub fn wav_w64_srate2blocksize(srate_chan_product: c_int) -> c_int;
}
extern "C" {
    pub fn wav_w64_format_str(k: c_int) -> *const c_char;
}
extern "C" {
    pub fn wav_w64_read_fmt_chunk(
        psf: *mut SF_PRIVATE,
        wav_fmt: *mut WAV_FMT,
        structsize: c_int,
    ) -> c_int;
}
pub type word = c_short;
pub type longword = c_int;
pub type uword = c_ushort;
pub type ulongword = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_state {
    pub dp0: [word; 280usize],
    pub z1: word,
    pub L_z2: longword,
    pub mp: c_int,
    pub u: [word; 8usize],
    pub LARpp: [[word; 8usize]; 2usize],
    pub j: word,
    pub ltp_cut: word,
    pub nrp: word,
    pub v: [word; 9usize],
    pub msr: word,
    pub verbose: c_char,
    pub fast: c_char,
    pub wav_fmt: c_char,
    pub frame_index: c_uchar,
    pub frame_chain: c_uchar,
    pub e: [word; 50usize],
}
pub type GSM_STATE = gsm_state;
extern "C" {
    pub fn gsm_mult(a: word, b: word) -> word;
}
extern "C" {
    pub fn gsm_L_mult(a: word, b: word) -> longword;
}
extern "C" {
    pub fn gsm_mult_r(a: word, b: word) -> word;
}
extern "C" {
    pub fn gsm_div(num: word, denum: word) -> word;
}
extern "C" {
    pub fn gsm_add(a: word, b: word) -> word;
}
extern "C" {
    pub fn gsm_L_add(a: longword, b: longword) -> longword;
}
extern "C" {
    pub fn gsm_sub(a: word, b: word) -> word;
}
extern "C" {
    pub fn gsm_L_sub(a: longword, b: longword) -> longword;
}
extern "C" {
    pub fn gsm_abs(a: word) -> word;
}
extern "C" {
    pub fn gsm_norm(a: longword) -> word;
}
extern "C" {
    pub fn gsm_L_asl(a: longword, n: c_int) -> longword;
}
extern "C" {
    pub fn gsm_asl(a: word, n: c_int) -> word;
}
extern "C" {
    pub fn gsm_L_asr(a: longword, n: c_int) -> longword;
}
extern "C" {
    pub fn gsm_asr(a: word, n: c_int) -> word;
}
extern "C" {
    pub fn Gsm_Coder(
        S: *mut gsm_state,
        s: *mut word,
        LARc: *mut word,
        Nc: *mut word,
        bc: *mut word,
        Mc: *mut word,
        xmaxc: *mut word,
        xMc: *mut word,
    );
}
extern "C" {
    pub fn Gsm_Long_Term_Predictor(
        S: *mut gsm_state,
        d: *mut word,
        dp: *mut word,
        e: *mut word,
        dpp: *mut word,
        Nc: *mut word,
        bc: *mut word,
    );
}
extern "C" {
    pub fn Gsm_LPC_Analysis(S: *mut gsm_state, s: *mut word, LARc: *mut word);
}
extern "C" {
    pub fn Gsm_Preprocess(S: *mut gsm_state, s: *mut word, so: *mut word);
}
extern "C" {
    pub fn Gsm_Encoding(
        S: *mut gsm_state,
        e: *mut word,
        ep: *mut word,
        xmaxc: *mut word,
        Mc: *mut word,
        xMc: *mut word,
    );
}
extern "C" {
    pub fn Gsm_Short_Term_Analysis_Filter(S: *mut gsm_state, LARc: *mut word, d: *mut word);
}
extern "C" {
    pub fn Gsm_Decoder(
        S: *mut gsm_state,
        LARcr: *mut word,
        Ncr: *mut word,
        bcr: *mut word,
        Mcr: *mut word,
        xmaxcr: *mut word,
        xMcr: *mut word,
        s: *mut word,
    );
}
extern "C" {
    pub fn Gsm_Decoding(
        S: *mut gsm_state,
        xmaxcr: word,
        Mcr: word,
        xMcr: *mut word,
        erp: *mut word,
    );
}
extern "C" {
    pub fn Gsm_Long_Term_Synthesis_Filtering(
        S: *mut gsm_state,
        Ncr: word,
        bcr: word,
        erp: *mut word,
        drp: *mut word,
    );
}
extern "C" {
    pub fn Gsm_RPE_Decoding(xmaxcr: word, Mcr: word, xMcr: *mut word, erp: *mut word);
}
extern "C" {
    pub fn Gsm_RPE_Encoding(e: *mut word, xmaxc: *mut word, Mc: *mut word, xMc: *mut word);
}
extern "C" {
    pub fn Gsm_Short_Term_Synthesis_Filter(
        S: *mut gsm_state,
        LARcr: *mut word,
        drp: *mut word,
        s: *mut word,
    );
}
extern "C" {
    pub fn Gsm_Update_of_reconstructed_short_time_residual_signal(
        dpp: *mut word,
        ep: *mut word,
        dp: *mut word,
    );
}
extern "C" {
    pub static mut gsm_A: [word; 8usize];
}
extern "C" {
    pub static mut gsm_B: [word; 8usize];
}
extern "C" {
    pub static mut gsm_MIC: [word; 8usize];
}
extern "C" {
    pub static mut gsm_MAC: [word; 8usize];
}
extern "C" {
    pub static mut gsm_INVA: [word; 8usize];
}
extern "C" {
    pub static mut gsm_DLB: [word; 4usize];
}
extern "C" {
    pub static mut gsm_QLB: [word; 4usize];
}
extern "C" {
    pub static mut gsm_H: [word; 11usize];
}
extern "C" {
    pub static mut gsm_NRFAC: [word; 8usize];
}
extern "C" {
    pub static mut gsm_FAC: [word; 8usize];
}
extern "C" {
    pub fn gsm_debug_words(name: *mut c_char, arg1: c_int, arg2: c_int, arg3: *mut word);
}
extern "C" {
    pub fn gsm_debug_longwords(name: *mut c_char, arg1: c_int, arg2: c_int, arg3: *mut longword);
}
extern "C" {
    pub fn gsm_debug_longword(name: *mut c_char, arg1: longword);
}
extern "C" {
    pub fn gsm_debug_word(name: *mut c_char, arg1: word);
}
