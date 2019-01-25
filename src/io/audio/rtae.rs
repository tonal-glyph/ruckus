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
use crate::sys::*;
use crate::RtError_h_edited::*;
///* Gary Scavone's RtAudio header. Need a way to configure RtApi at runtime,
///* maybe a config.toml?
use libc::*;
#[doc = " \\typedef typedef unsigned long RtAudioFormat;"]
#[doc = "\\brief RtAudio data format type."]
#[doc = ""]
#[doc = "Support for signed integers and floats.  Audio data fed to/from an"]
#[doc = "RtAudio stream is assumed to ALWAYS be in host byte order.  The"]
#[doc = "internal routines will automatically take care of any necessary"]
#[doc = "byte-swapping between the host format and the soundcard.  Thus,"]
#[doc = "endian-ness is not a concern in the following format definitions."]
#[doc = ""]
#[doc = "- \\e RTAUDIO_SINT8:   8-bit signed integer."]
#[doc = "- \\e RTAUDIO_SINT16:  16-bit signed integer."]
#[doc = "- \\e RTAUDIO_SINT24:  24-bit signed integer."]
#[doc = "- \\e RTAUDIO_SINT32:  32-bit signed integer."]
#[doc = "- \\e RTAUDIO_FLOAT32: Normalized between plus/minus 1.0."]
#[doc = "- \\e RTAUDIO_FLOAT64: Normalized between plus/minus 1.0."]
pub type RtAudioFormat = c_ulong;
pub const RTAUDIO_SINT8: RtAudioFormat = 0x1;
pub const RTAUDIO_SINT16: RtAudioFormat = 0x2;
pub const RTAUDIO_SINT24: RtAudioFormat = 0x4;
pub const RTAUDIO_SINT32: RtAudioFormat = 0x8;
pub const RTAUDIO_FLOAT32: RtAudioFormat = 0x16;
pub const RTAUDIO_FLOAT64: RtAudioFormat = 0x32;
#[doc = " \\typedef typedef unsigned long RtAudioStreamFlags;"]
#[doc = "\\brief RtAudio stream option flags."]
#[doc = ""]
#[doc = "The following flags can be OR\'ed together to allow a client to"]
#[doc = "make changes to the default stream behavior:"]
#[doc = ""]
#[doc = "- \\e RTAUDIO_NONINTERLEAVED:   Use non-interleaved buffers (default = interleaved)."]
#[doc = "- \\e RTAUDIO_MINIMIZE_LATENCY: Attempt to set stream parameters for lowest possible latency."]
#[doc = "- \\e RTAUDIO_HOG_DEVICE:       Attempt grab device for exclusive use."]
#[doc = "- \\e RTAUDIO_ALSA_USE_DEFAULT: Use the \"default\" PCM device (ALSA only)."]
#[doc = ""]
#[doc = "By default, RtAudio streams pass and receive audio data from the"]
#[doc = "client in an interleaved format.  By passing the"]
#[doc = "RTAUDIO_NONINTERLEAVED flag to the openStream() function, audio"]
#[doc = "data will instead be presented in non-interleaved buffers.  In"]
#[doc = "this case, each buffer argument in the RtAudioCallback function"]
#[doc = "will point to a single array of data, with \\c nFrames samples for"]
#[doc = "each channel concatenated back-to-back.  For example, the first"]
#[doc = "sample of data for the second channel would be located at index \\c"]
#[doc = "nFrames (assuming the \\c buffer pointer was recast to the correct"]
#[doc = "data type for the stream)."]
#[doc = ""]
#[doc = "Certain audio APIs offer a number of parameters that influence the"]
#[doc = "I/O latency of a stream.  By default, RtAudio will attempt to set"]
#[doc = "these parameters internally for robust (glitch-free) performance"]
#[doc = "(though some APIs, like Windows Direct Sound, make this difficult)."]
#[doc = "By passing the RTAUDIO_MINIMIZE_LATENCY flag to the openStream()"]
#[doc = "function, internal stream settings will be influenced in an attempt"]
#[doc = "to minimize stream latency, though possibly at the expense of stream"]
#[doc = "performance."]
#[doc = ""]
#[doc = "If the RTAUDIO_HOG_DEVICE flag is set, RtAudio will attempt to"]
#[doc = "open the input and/or output stream device(s) for exclusive use."]
#[doc = "Note that this is not possible with all supported audio APIs."]
#[doc = ""]
#[doc = "If the RTAUDIO_SCHEDULE_REALTIME flag is set, RtAudio will attempt"]
#[doc = "to select realtime scheduling (round-robin) for the callback thread."]
#[doc = ""]
#[doc = "If the RTAUDIO_ALSA_USE_DEFAULT flag is set, RtAudio will attempt to"]
#[doc = "open the \"default\" PCM device when using the ALSA API. Note that this"]
#[doc = "will override any specified input or output device id."]
pub type RtAudioStreamFlags = c_uint;
pub const RTAUDIO_NONINTERLEAVED: RtAudioStreamFlags = 0x1;
pub const RTAUDIO_MINIMIZE_LATENCY: RtAudioStreamFlags = 0x2;
pub const RTAUDIO_HOG_DEVICE: RtAudioStreamFlags = 0x4;
pub const RTAUDIO_SCHEDULE_REALTIME: RtAudioStreamFlags = 0x8;
pub const RTAUDIO_ALSA_USE_DEFAULT: RtAudioStreamFlags = 0x10;
#[doc = " \\typedef typedef unsigned long RtAudioStreamStatus;"]
#[doc = "\\brief RtAudio stream status (over- or underflow) flags."]
#[doc = ""]
#[doc = "Notification of a stream over- or underflow is indicated by a"]
#[doc = "non-zero stream \\c status argument in the RtAudioCallback function."]
#[doc = "The stream status can be one of the following two options,"]
#[doc = "depending on whether the stream is open for output and/or input:"]
#[doc = ""]
#[doc = "- \\e RTAUDIO_INPUT_OVERFLOW:   Input data was discarded because of an overflow condition at the driver."]
#[doc = "- \\e RTAUDIO_OUTPUT_UNDERFLOW: The output buffer ran low, likely producing a break in the output sound."]
pub type RtAudioStreamStatus = c_uint;
pub const RTAUDIO_INPUT_OVERFLOW: RtAudioStreamStatus = 0x1;
pub const RTAUDIO_OUTPUT_UNDERFLOW: RtAudioStreamStatus = 0x2;
#[doc = "! RtAudio callback function prototype."]
#[doc = "*!"]
#[doc = "All RtAudio clients must create a function of type RtAudioCallback"]
#[doc = "to read and/or write data from/to the audio stream.  When the"]
#[doc = "underlying audio system is ready for new input or output data, this"]
#[doc = "function will be invoked."]
#[doc = ""]
#[doc = "\\param outputBuffer For output (or duplex) streams, the client"]
#[doc = "should write \\c nFrames of audio sample frames into this"]
#[doc = "buffer.  This argument should be recast to the datatype"]
#[doc = "specified when the stream was opened.  For input-only"]
#[doc = "streams, this argument will be NULL."]
#[doc = ""]
#[doc = "\\param inputBuffer For input (or duplex) streams, this buffer will"]
#[doc = "hold \\c nFrames of input audio sample frames.  This"]
#[doc = "argument should be recast to the datatype specified when the"]
#[doc = "stream was opened.  For output-only streams, this argument"]
#[doc = "will be NULL."]
#[doc = ""]
#[doc = "\\param nFrames The number of sample frames of input or output"]
#[doc = "data in the buffers.  The actual buffer size in bytes is"]
#[doc = "dependent on the data type and number of channels in use."]
#[doc = ""]
#[doc = "\\param streamTime The number of seconds that have elapsed since the"]
#[doc = "stream was started."]
#[doc = ""]
#[doc = "\\param status If non-zero, this argument indicates a data overflow"]
#[doc = "or underflow condition for the stream.  The particular"]
#[doc = "condition can be determined by comparison with the"]
#[doc = "RtAudioStreamStatus flags."]
#[doc = ""]
#[doc = "\\param userData A pointer to optional data provided by the client"]
#[doc = "when opening the stream (default = NULL)."]
#[doc = ""]
#[doc = "To continue normal stream operation, the RtAudioCallback function"]
#[doc = "should return a value of zero.  To stop the stream and drain the"]
#[doc = "output buffer, the function should return a value of one.  To abort"]
#[doc = "the stream immediately, the client should return a value of two."]
#[doc = "*/"]
pub type RtAudioCallback = Option<
    unsafe extern "C" fn(
        outputBuffer: *mut c_void,
        inputBuffer: *mut c_void,
        nFrames: c_uint,
        streamTime: f64,
        status: RtAudioStreamStatus,
        userData: *mut c_void,
    ) -> c_int,
>;
#[doc = "! RtAudio error callback function prototype."]
#[doc = "*!"]
#[doc = "\\param type Type of error."]
#[doc = "\\param errorText Error description."]
#[doc = "*/"]
///! RtAudio error callback function prototype.
pub type RtAudioErrorCallback =
    Option<unsafe extern "C" fn(type_: RtError_Type, errorText: *const string)>;
#[repr(C)]
#[derive(Debug)]
pub struct RtAudio {
    pub rtapi_: *mut RtApi,
}
#[doc = "< Search for a working compiled API."]
pub const RtAudio_Api_UNSPECIFIED: RtAudio_Api = 0;
#[doc = "< The Advanced Linux Sound Architecture API."]
pub const RtAudio_Api_LINUX_ALSA: RtAudio_Api = 1;
#[doc = "< The Linux PulseAudio API."]
pub const RtAudio_Api_LINUX_PULSE: RtAudio_Api = 2;
#[doc = "< The Linux Open Sound System API."]
pub const RtAudio_Api_LINUX_OSS: RtAudio_Api = 3;
#[doc = "< The Jack Low-Latency Audio Server API."]
pub const RtAudio_Api_UNIX_JACK: RtAudio_Api = 4;
#[doc = "< Macintosh OS-X Core Audio API."]
pub const RtAudio_Api_MACOSX_CORE: RtAudio_Api = 5;
#[doc = "< The Steinberg Audio Stream I/O API."]
pub const RtAudio_Api_WINDOWS_ASIO: RtAudio_Api = 6;
#[doc = "< The Microsoft Direct Sound API."]
pub const RtAudio_Api_WINDOWS_DS: RtAudio_Api = 7;
#[doc = "< A compilable but non-functional API."]
pub const RtAudio_Api_RTAUDIO_DUMMY: RtAudio_Api = 8;
#[doc = "! Audio API specifier arguments."]
pub type RtAudio_Api = u32;
#[doc = "! The public device information structure for returning queried values."]
#[repr(C)]
pub struct RtAudio_DeviceInfo {
    #[doc = "< true if the device capabilities were successfully probed."]
    pub probed: bool,
    #[doc = "< Character string device identifier."]
    pub name: string,
    #[doc = "< Maximum output channels supported by device."]
    pub outputChannels: c_uint,
    #[doc = "< Maximum input channels supported by device."]
    pub inputChannels: c_uint,
    #[doc = "< Maximum simultaneous input/output channels supported by device."]
    pub duplexChannels: c_uint,
    #[doc = "< true if this is the default output device."]
    pub isDefaultOutput: bool,
    #[doc = "< true if this is the default input device."]
    pub isDefaultInput: bool,
    #[doc = "< Supported sample rates (queried from list of standard rates)."]
    pub sampleRates: vector,
    #[doc = "< Bit mask of supported data formats."]
    pub nativeFormats: RtAudioFormat,
}
#[doc = "! The structure for specifying input or ouput stream parameters."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RtAudio_StreamParameters {
    #[doc = "< Device index (0 to getDeviceCount() - 1)."]
    pub deviceId: c_uint,
    #[doc = "< Number of channels."]
    pub nChannels: c_uint,
    #[doc = "< First channel index on device (default = 0)."]
    pub firstChannel: c_uint,
}
#[doc = "! The structure for specifying stream options."]
#[doc = "*!"]
#[doc = "The following flags can be OR\'ed together to allow a client to"]
#[doc = "make changes to the default stream behavior:"]
#[doc = ""]
#[doc = "- \\e RTAUDIO_NONINTERLEAVED:    Use non-interleaved buffers (default = interleaved)."]
#[doc = "- \\e RTAUDIO_MINIMIZE_LATENCY:  Attempt to set stream parameters for lowest possible latency."]
#[doc = "- \\e RTAUDIO_HOG_DEVICE:        Attempt grab device for exclusive use."]
#[doc = "- \\e RTAUDIO_SCHEDULE_REALTIME: Attempt to select realtime scheduling for callback thread."]
#[doc = "- \\e RTAUDIO_ALSA_USE_DEFAULT:  Use the \"default\" PCM device (ALSA only)."]
#[doc = ""]
#[doc = "By default, RtAudio streams pass and receive audio data from the"]
#[doc = "client in an interleaved format.  By passing the"]
#[doc = "RTAUDIO_NONINTERLEAVED flag to the openStream() function, audio"]
#[doc = "data will instead be presented in non-interleaved buffers.  In"]
#[doc = "this case, each buffer argument in the RtAudioCallback function"]
#[doc = "will point to a single array of data, with \\c nFrames samples for"]
#[doc = "each channel concatenated back-to-back.  For example, the first"]
#[doc = "sample of data for the second channel would be located at index \\c"]
#[doc = "nFrames (assuming the \\c buffer pointer was recast to the correct"]
#[doc = "data type for the stream)."]
#[doc = ""]
#[doc = "Certain audio APIs offer a number of parameters that influence the"]
#[doc = "I/O latency of a stream.  By default, RtAudio will attempt to set"]
#[doc = "these parameters internally for robust (glitch-free) performance"]
#[doc = "(though some APIs, like Windows Direct Sound, make this difficult)."]
#[doc = "By passing the RTAUDIO_MINIMIZE_LATENCY flag to the openStream()"]
#[doc = "function, internal stream settings will be influenced in an attempt"]
#[doc = "to minimize stream latency, though possibly at the expense of stream"]
#[doc = "performance."]
#[doc = ""]
#[doc = "If the RTAUDIO_HOG_DEVICE flag is set, RtAudio will attempt to"]
#[doc = "open the input and/or output stream device(s) for exclusive use."]
#[doc = "Note that this is not possible with all supported audio APIs."]
#[doc = ""]
#[doc = "If the RTAUDIO_SCHEDULE_REALTIME flag is set, RtAudio will attempt"]
#[doc = "to select realtime scheduling (round-robin) for the callback thread."]
#[doc = "The \\c priority parameter will only be used if the RTAUDIO_SCHEDULE_REALTIME"]
#[doc = "flag is set. It defines the thread\'s realtime priority."]
#[doc = ""]
#[doc = "If the RTAUDIO_ALSA_USE_DEFAULT flag is set, RtAudio will attempt to"]
#[doc = "open the \"default\" PCM device when using the ALSA API. Note that this"]
#[doc = "will override any specified input or output device id."]
#[doc = ""]
#[doc = "The \\c numberOfBuffers parameter can be used to control stream"]
#[doc = "latency in the Windows DirectSound, Linux OSS, and Linux Alsa APIs"]
#[doc = "only.  A value of two is usually the smallest allowed.  Larger"]
#[doc = "numbers can potentially result in more robust stream performance,"]
#[doc = "though likely at the cost of stream latency.  The value set by the"]
#[doc = "user is replaced during execution of the RtAudio::openStream()"]
#[doc = "function by the value actually used by the system."]
#[doc = ""]
#[doc = "The \\c streamName parameter can be used to set the client name"]
#[doc = "when using the Jack API.  By default, the client name is set to"]
#[doc = "RtApiJack.  However, if you wish to create multiple instances of"]
#[doc = "RtAudio with Jack, each instance must have a unique client name."]
#[doc = "*/"]
#[repr(C)]
pub struct RtAudio_StreamOptions {
    #[doc = "< A bit-mask of stream flags (RTAUDIO_NONINTERLEAVED, RTAUDIO_MINIMIZE_LATENCY, RTAUDIO_HOG_DEVICE, RTAUDIO_ALSA_USE_DEFAULT)."]
    pub flags: RtAudioStreamFlags,
    #[doc = "< Number of stream buffers."]
    pub numberOfBuffers: c_uint,
    #[doc = "< A stream name (currently used only in Jack)."]
    pub streamName: string,
    #[doc = "< Scheduling priority of callback thread (only used with flag RTAUDIO_SCHEDULE_REALTIME)."]
    pub priority: c_int,
}
extern "C" {
    #[doc = "! A static function to determine the available compiled audio APIs."]
    #[doc = "*!"]
    #[doc = "The values returned in the vector can be compared against"]
    #[doc = "the enumerated list values.  Note that there can be more than one"]
    #[doc = "API compiled for certain operating systems."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio14getCompiledApiERSt6vectorINS_3ApiESaIS1_EE"]
    pub fn RtAudio_getCompiledApi(apis: *mut vector);
}
extern "C" {
    #[doc = "! Returns the audio API specifier for the current instance of RtAudio."]
    #[link_name = "\u{1}_ZN7RtAudio13getCurrentApiEv"]
    pub fn RtAudio_getCurrentApi(this: *mut RtAudio) -> RtAudio_Api;
}
extern "C" {
    #[doc = "! A public function that queries for the number of audio devices available."]
    #[doc = "*!"]
    #[doc = "This function performs a system query of available devices each time it"]
    #[doc = "is called, thus supporting devices connected \\e after instantiation. If"]
    #[doc = "a system error occurs during processing, a warning will be issued."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio14getDeviceCountEv"]
    pub fn RtAudio_getDeviceCount(this: *mut RtAudio) -> c_uint;
}
extern "C" {
    #[doc = "! Return an RtAudio::DeviceInfo structure for a specified device number."]
    #[doc = "*!"]
    #[doc = ""]
    #[doc = "Any device integer between 0 and getDeviceCount() - 1 is valid."]
    #[doc = "If an invalid argument is provided, an RtError (type = INVALID_USE)"]
    #[doc = "will be thrown.  If a device is busy or otherwise unavailable, the"]
    #[doc = "structure member \"probed\" will have a value of \"false\" and all"]
    #[doc = "other members are undefined.  If the specified device is the"]
    #[doc = "current default input or output device, the corresponding"]
    #[doc = "\"isDefault\" member will have a value of \"true\"."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio13getDeviceInfoEj"]
    pub fn RtAudio_getDeviceInfo(this: *mut RtAudio, device: c_uint) -> RtAudio_DeviceInfo;
}
extern "C" {
    #[doc = "! A function that returns the index of the default output device."]
    #[doc = "*!"]
    #[doc = "If the underlying audio API does not provide a \"default"]
    #[doc = "device\", or if no devices are available, the return value will be"]
    #[doc = "0.  Note that this is a valid device identifier and it is the"]
    #[doc = "client\'s responsibility to verify that a device is available"]
    #[doc = "before attempting to open a stream."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio22getDefaultOutputDeviceEv"]
    pub fn RtAudio_getDefaultOutputDevice(this: *mut RtAudio) -> c_uint;
}
extern "C" {
    #[doc = "! A function that returns the index of the default input device."]
    #[doc = "*!"]
    #[doc = "If the underlying audio API does not provide a \"default"]
    #[doc = "device\", or if no devices are available, the return value will be"]
    #[doc = "0.  Note that this is a valid device identifier and it is the"]
    #[doc = "client\'s responsibility to verify that a device is available"]
    #[doc = "before attempting to open a stream."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio21getDefaultInputDeviceEv"]
    pub fn RtAudio_getDefaultInputDevice(this: *mut RtAudio) -> c_uint;
}
extern "C" {
    #[doc = "! A public function for opening a stream with the specified parameters."]
    #[doc = "*!"]
    #[doc = "An RtError (type = SYSTEM_ERROR) is thrown if a stream cannot be"]
    #[doc = "opened with the specified parameters or an error occurs during"]
    #[doc = "processing.  An RtError (type = INVALID_USE) is thrown if any"]
    #[doc = "invalid device ID or channel number parameters are specified."]
    #[doc = ""]
    #[doc = "\\param outputParameters Specifies output stream parameters to use"]
    #[doc = "when opening a stream, including a device ID, number of channels,"]
    #[doc = "and starting channel number.  For input-only streams, this"]
    #[doc = "argument should be NULL.  The device ID is an index value between"]
    #[doc = "0 and getDeviceCount() - 1."]
    #[doc = "\\param inputParameters Specifies input stream parameters to use"]
    #[doc = "when opening a stream, including a device ID, number of channels,"]
    #[doc = "and starting channel number.  For output-only streams, this"]
    #[doc = "argument should be NULL.  The device ID is an index value between"]
    #[doc = "0 and getDeviceCount() - 1."]
    #[doc = "\\param format An RtAudioFormat specifying the desired sample data format."]
    #[doc = "\\param sampleRate The desired sample rate (sample frames per second)."]
    #[doc = "\\param *bufferFrames A pointer to a value indicating the desired"]
    #[doc = "internal buffer size in sample frames.  The actual value"]
    #[doc = "used by the device is returned via the same pointer.  A"]
    #[doc = "value of zero can be specified, in which case the lowest"]
    #[doc = "allowable value is determined."]
    #[doc = "\\param callback A client-defined function that will be invoked"]
    #[doc = "when input data is available and/or output data is needed."]
    #[doc = "\\param userData An optional pointer to data that can be accessed"]
    #[doc = "from within the callback function."]
    #[doc = "\\param options An optional pointer to a structure containing various"]
    #[doc = "global stream options, including a list of OR\'ed RtAudioStreamFlags"]
    #[doc = "and a suggested number of stream buffers that can be used to"]
    #[doc = "control stream latency.  More buffers typically result in more"]
    #[doc = "robust performance, though at a cost of greater latency.  If a"]
    #[doc = "value of zero is specified, a system-specific median value is"]
    #[doc = "chosen.  If the RTAUDIO_MINIMIZE_LATENCY flag bit is set, the"]
    #[doc = "lowest allowable value is used.  The actual value used is"]
    #[doc = "returned via the structure argument.  The parameter is API dependent."]
    #[doc = "\\param errorCallback A client-defined function that will be invoked"]
    #[doc = "when an error has occured."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio10openStreamEPNS_16StreamParametersES1_mjPjPFiPvS3_jdjS3_ES3_PNS_13StreamOptionsEPFvN7RtError4TypeERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEE"]
    pub fn RtAudio_openStream(
        this: *mut RtAudio,
        outputParameters: *mut RtAudio_StreamParameters,
        inputParameters: *mut RtAudio_StreamParameters,
        format: RtAudioFormat,
        sampleRate: c_uint,
        bufferFrames: *mut c_uint,
        callback: RtAudioCallback,
        userData: *mut c_void,
        options: *mut RtAudio_StreamOptions,
        errorCallback: RtAudioErrorCallback,
    );
}
extern "C" {
    #[doc = "! A function that closes a stream and frees any associated stream memory."]
    #[doc = "*!"]
    #[doc = "If a stream is not open, this function issues a warning and"]
    #[doc = "returns (no exception is thrown)."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio11closeStreamEv"]
    pub fn RtAudio_closeStream(this: *mut RtAudio);
}
extern "C" {
    #[doc = "! A function that starts a stream."]
    #[doc = "*!"]
    #[doc = "An RtError (type = SYSTEM_ERROR) is thrown if an error occurs"]
    #[doc = "during processing.  An RtError (type = INVALID_USE) is thrown if a"]
    #[doc = "stream is not open.  A warning is issued if the stream is already"]
    #[doc = "running."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio11startStreamEv"]
    pub fn RtAudio_startStream(this: *mut RtAudio);
}
extern "C" {
    #[doc = "! Stop a stream, allowing any samples remaining in the output queue to be played."]
    #[doc = "*!"]
    #[doc = "An RtError (type = SYSTEM_ERROR) is thrown if an error occurs"]
    #[doc = "during processing.  An RtError (type = INVALID_USE) is thrown if a"]
    #[doc = "stream is not open.  A warning is issued if the stream is already"]
    #[doc = "stopped."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio10stopStreamEv"]
    pub fn RtAudio_stopStream(this: *mut RtAudio);
}
extern "C" {
    #[doc = "! Stop a stream, discarding any samples remaining in the input/output queue."]
    #[doc = "*!"]
    #[doc = "An RtError (type = SYSTEM_ERROR) is thrown if an error occurs"]
    #[doc = "during processing.  An RtError (type = INVALID_USE) is thrown if a"]
    #[doc = "stream is not open.  A warning is issued if the stream is already"]
    #[doc = "stopped."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio11abortStreamEv"]
    pub fn RtAudio_abortStream(this: *mut RtAudio);
}
extern "C" {
    #[doc = "! Returns true if a stream is open and false if not."]
    #[link_name = "\u{1}_ZNK7RtAudio12isStreamOpenEv"]
    pub fn RtAudio_isStreamOpen(this: *const RtAudio) -> bool;
}
extern "C" {
    #[doc = "! Returns true if the stream is running and false if it is stopped or not open."]
    #[link_name = "\u{1}_ZNK7RtAudio15isStreamRunningEv"]
    pub fn RtAudio_isStreamRunning(this: *const RtAudio) -> bool;
}
extern "C" {
    #[doc = "! Returns the number of elapsed seconds since the stream was started."]
    #[doc = "*!"]
    #[doc = "If a stream is not open, an RtError (type = INVALID_USE) will be thrown."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio13getStreamTimeEv"]
    pub fn RtAudio_getStreamTime(this: *mut RtAudio) -> f64;
}
extern "C" {
    #[doc = "! Returns the internal stream latency in sample frames."]
    #[doc = "*!"]
    #[doc = "The stream latency refers to delay in audio input and/or output"]
    #[doc = "caused by internal buffering by the audio system and/or hardware."]
    #[doc = "For duplex streams, the returned value will represent the sum of"]
    #[doc = "the input and output latencies.  If a stream is not open, an"]
    #[doc = "RtError (type = INVALID_USE) will be thrown.  If the API does not"]
    #[doc = "report latency, the return value will be zero."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio16getStreamLatencyEv"]
    pub fn RtAudio_getStreamLatency(this: *mut RtAudio) -> c_long;
}
extern "C" {
    #[doc = "! Returns actual sample rate in use by the stream."]
    #[doc = "*!"]
    #[doc = "On some systems, the sample rate used may be slightly different"]
    #[doc = "than that specified in the stream parameters.  If a stream is not"]
    #[doc = "open, an RtError (type = INVALID_USE) will be thrown."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudio19getStreamSampleRateEv"]
    pub fn RtAudio_getStreamSampleRate(this: *mut RtAudio) -> c_uint;
}
extern "C" {
    #[doc = "! Specify whether warning messages should be printed to stderr."]
    #[link_name = "\u{1}_ZN7RtAudio12showWarningsEb"]
    pub fn RtAudio_showWarnings(this: *mut RtAudio, value: bool);
}
extern "C" {
    #[link_name = "\u{1}_ZN7RtAudio9openRtApiENS_3ApiE"]
    pub fn RtAudio_openRtApi(this: *mut RtAudio, api: RtAudio_Api);
}
extern "C" {
    #[doc = "! The class constructor."]
    #[doc = "*!"]
    #[doc = "The constructor performs minor initialization tasks.  No exceptions"]
    #[doc = "can be thrown."]
    #[doc = ""]
    #[doc = "If no API argument is specified and multiple API support has been"]
    #[doc = "compiled, the default order of use is JACK, ALSA, OSS (Linux"]
    #[doc = "systems) and ASIO, DS (Windows systems)."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudioC1ENS_3ApiE"]
    pub fn RtAudio_RtAudio(this: *mut RtAudio, api: RtAudio_Api);
}
extern "C" {
    #[doc = "! The destructor."]
    #[doc = "*!"]
    #[doc = "If a stream is running or open, it will be stopped and closed"]
    #[doc = "automatically."]
    #[doc = "*/"]
    #[link_name = "\u{1}_ZN7RtAudioD1Ev"]
    pub fn RtAudio_RtAudio_destructor(this: *mut RtAudio);
}
impl RtAudio {
    #[inline]
    pub unsafe fn getCompiledApi(apis: *mut vector) {
        RtAudio_getCompiledApi(apis)
    }
    #[inline]
    pub unsafe fn getCurrentApi(&mut self) -> RtAudio_Api {
        RtAudio_getCurrentApi(self)
    }
    #[inline]
    pub unsafe fn getDeviceCount(&mut self) -> c_uint {
        RtAudio_getDeviceCount(self)
    }
    #[inline]
    pub unsafe fn getDeviceInfo(&mut self, device: c_uint) -> RtAudio_DeviceInfo {
        RtAudio_getDeviceInfo(self, device)
    }
    #[inline]
    pub unsafe fn getDefaultOutputDevice(&mut self) -> c_uint {
        RtAudio_getDefaultOutputDevice(self)
    }
    #[inline]
    pub unsafe fn getDefaultInputDevice(&mut self) -> c_uint {
        RtAudio_getDefaultInputDevice(self)
    }
    #[inline]
    pub unsafe fn openStream(
        &mut self,
        outputParameters: *mut RtAudio_StreamParameters,
        inputParameters: *mut RtAudio_StreamParameters,
        format: RtAudioFormat,
        sampleRate: c_uint,
        bufferFrames: *mut c_uint,
        callback: RtAudioCallback,
        userData: *mut c_void,
        options: *mut RtAudio_StreamOptions,
        errorCallback: RtAudioErrorCallback,
    ) {
        RtAudio_openStream(
            self,
            outputParameters,
            inputParameters,
            format,
            sampleRate,
            bufferFrames,
            callback,
            userData,
            options,
            errorCallback,
        )
    }
    #[inline]
    pub unsafe fn closeStream(&mut self) {
        RtAudio_closeStream(self)
    }
    #[inline]
    pub unsafe fn startStream(&mut self) {
        RtAudio_startStream(self)
    }
    #[inline]
    pub unsafe fn stopStream(&mut self) {
        RtAudio_stopStream(self)
    }
    #[inline]
    pub unsafe fn abortStream(&mut self) {
        RtAudio_abortStream(self)
    }
    #[inline]
    pub unsafe fn isStreamOpen(&self) -> bool {
        RtAudio_isStreamOpen(self)
    }
    #[inline]
    pub unsafe fn isStreamRunning(&self) -> bool {
        RtAudio_isStreamRunning(self)
    }
    #[inline]
    pub unsafe fn getStreamTime(&mut self) -> f64 {
        RtAudio_getStreamTime(self)
    }
    #[inline]
    pub unsafe fn getStreamLatency(&mut self) -> c_long {
        RtAudio_getStreamLatency(self)
    }
    #[inline]
    pub unsafe fn getStreamSampleRate(&mut self) -> c_uint {
        RtAudio_getStreamSampleRate(self)
    }
    #[inline]
    pub unsafe fn showWarnings(&mut self, value: bool) {
        RtAudio_showWarnings(self, value)
    }
    #[inline]
    pub unsafe fn openRtApi(&mut self, api: RtAudio_Api) {
        RtAudio_openRtApi(self, api)
    }
    #[inline]
    pub unsafe fn new(api: RtAudio_Api) -> Self {
        let mut __bindgen_tmp = uninitialized();
        RtAudio_RtAudio(&mut __bindgen_tmp, api);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        RtAudio_RtAudio_destructor(self)
    }
}
pub type ThreadHandle = pthread_t;
pub type StreamMutex = pthread_mutex_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CallbackInfo {
    pub object: *mut c_void,
    pub thread: ThreadHandle,
    pub callback: *mut c_void,
    pub userData: *mut c_void,
    pub errorCallback: *mut c_void,
    pub apiInfo: *mut c_void,
    pub isRunning: bool,
    pub doRealtime: bool,
    pub priority: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S24 {
    pub c3: [c_uchar; 3usize],
}
#[repr(C)]
pub struct RtApi__bindgen_vtable(c_void);
#[repr(C)]
pub struct RtApi {
    pub vtable_: *const RtApi__bindgen_vtable,
    pub errorStream_: uwl::StringStream,
    pub errorText_: string,
    pub showWarnings_: bool,
    pub stream_: RtApi_RtApiStream,
}
pub const RtApi_FAILURE: RtApi__bindgen_ty_1 = 0;
pub const RtApi_SUCCESS: RtApi__bindgen_ty_1 = 1;
pub type RtApi__bindgen_ty_1 = u32;
pub const RtApi_StreamState_STREAM_STOPPED: RtApi_StreamState = 0;
pub const RtApi_StreamState_STREAM_STOPPING: RtApi_StreamState = 1;
pub const RtApi_StreamState_STREAM_RUNNING: RtApi_StreamState = 2;
pub const RtApi_StreamState_STREAM_CLOSED: RtApi_StreamState = -50;
pub type RtApi_StreamState = i32;
pub const RtApi_StreamMode_OUTPUT: RtApi_StreamMode = 0;
pub const RtApi_StreamMode_INPUT: RtApi_StreamMode = 1;
pub const RtApi_StreamMode_DUPLEX: RtApi_StreamMode = 2;
pub const RtApi_StreamMode_UNINITIALIZED: RtApi_StreamMode = -75;
pub type RtApi_StreamMode = i32;
#[repr(C)]
pub struct RtApi_ConvertInfo {
    pub channels: c_int,
    pub inJump: c_int,
    pub outJump: c_int,
    pub inFormat: RtAudioFormat,
    pub outFormat: RtAudioFormat,
    pub inOffset: vector,
    pub outOffset: vector,
}
#[repr(C)]
pub struct RtApi_RtApiStream {
    pub device: [c_uint; 2usize],
    pub apiHandle: *mut c_void,
    pub mode: RtApi_StreamMode,
    pub state: RtApi_StreamState,
    pub userBuffer: [*mut c_char; 2usize],
    pub deviceBuffer: *mut c_char,
    pub doConvertBuffer: [bool; 2usize],
    pub userInterleaved: bool,
    pub deviceInterleaved: [bool; 2usize],
    pub doByteSwap: [bool; 2usize],
    pub sampleRate: c_uint,
    pub bufferSize: c_uint,
    pub nBuffers: c_uint,
    pub nUserChannels: [c_uint; 2usize],
    pub nDeviceChannels: [c_uint; 2usize],
    pub channelOffset: [c_uint; 2usize],
    pub latency: [c_ulong; 2usize],
    pub userFormat: RtAudioFormat,
    pub deviceFormat: [RtAudioFormat; 2usize],
    pub mutex: StreamMutex,
    pub callbackInfo: CallbackInfo,
    pub convertInfo: [RtApi_ConvertInfo; 2usize],
    pub streamTime: f64,
}
pub type RtApi_Int24 = S24;
pub type RtApi_Int16 = c_short;
pub type RtApi_Int32 = c_int;
pub type RtApi_Float32 = f32;
pub type RtApi_Float64 = f64;
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi16MAX_SAMPLE_RATESE"]
    pub static RtApi_MAX_SAMPLE_RATES: c_uint;
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi12SAMPLE_RATESE"]
    pub static mut RtApi_SAMPLE_RATES: [c_uint; 0usize];
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi10openStreamEPN7RtAudio16StreamParametersES2_mjPjPFiPvS4_jdjS4_ES4_PNS0_13StreamOptionsEPFvN7RtError4TypeERKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEE"]
    pub fn RtApi_openStream(
        this: *mut RtApi,
        outputParameters: *mut RtAudio_StreamParameters,
        inputParameters: *mut RtAudio_StreamParameters,
        format: RtAudioFormat,
        sampleRate: c_uint,
        bufferFrames: *mut c_uint,
        callback: RtAudioCallback,
        userData: *mut c_void,
        options: *mut RtAudio_StreamOptions,
        errorCallback: RtAudioErrorCallback,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi16getStreamLatencyEv"]
    pub fn RtApi_getStreamLatency(this: *mut RtApi) -> c_long;
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi19getStreamSampleRateEv"]
    pub fn RtApi_getStreamSampleRate(this: *mut RtApi) -> c_uint;
}
extern "C" {
    #[doc = "! A protected function used to increment the stream time."]
    #[link_name = "\u{1}_ZN5RtApi14tickStreamTimeEv"]
    pub fn RtApi_tickStreamTime(this: *mut RtApi);
}
extern "C" {
    #[doc = "! Protected common method to clear an RtApiStream structure."]
    #[link_name = "\u{1}_ZN5RtApi15clearStreamInfoEv"]
    pub fn RtApi_clearStreamInfo(this: *mut RtApi);
}
extern "C" {
    #[doc = "Protected common method that throws an RtError (type ="]
    #[doc = "INVALID_USE) if a stream is not open."]
    #[link_name = "\u{1}_ZN5RtApi12verifyStreamEv"]
    pub fn RtApi_verifyStream(this: *mut RtApi);
}
extern "C" {
    #[doc = "! Protected common error method to allow global control over error handling."]
    #[link_name = "\u{1}_ZN5RtApi5errorEN7RtError4TypeE"]
    pub fn RtApi_error(this: *mut RtApi, type_: crate::RtError_Type);
}
extern "C" {
    #[doc = "Protected method used to perform format, channel number, and/or interleaving"]
    #[doc = "conversions between the user and device buffers."]
    #[link_name = "\u{1}_ZN5RtApi13convertBufferEPcS0_RNS_11ConvertInfoE"]
    pub fn RtApi_convertBuffer(
        this: *mut RtApi,
        outBuffer: *mut c_char,
        inBuffer: *mut c_char,
        info: *mut RtApi_ConvertInfo,
    );
}
extern "C" {
    #[doc = "! Protected common method used to perform byte-swapping on buffers."]
    #[link_name = "\u{1}_ZN5RtApi14byteSwapBufferEPcjm"]
    pub fn RtApi_byteSwapBuffer(
        this: *mut RtApi,
        buffer: *mut c_char,
        samples: c_uint,
        format: RtAudioFormat,
    );
}
extern "C" {
    #[doc = "! Protected common method that returns the number of bytes for a given format."]
    #[link_name = "\u{1}_ZN5RtApi11formatBytesEm"]
    pub fn RtApi_formatBytes(this: *mut RtApi, format: RtAudioFormat) -> c_uint;
}
extern "C" {
    #[doc = "! Protected common method that sets up the parameters for buffer conversion."]
    #[link_name = "\u{1}_ZN5RtApi14setConvertInfoENS_10StreamModeEj"]
    pub fn RtApi_setConvertInfo(this: *mut RtApi, mode: RtApi_StreamMode, firstChannel: c_uint);
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApiC2Ev"]
    pub fn RtApi_RtApi(this: *mut RtApi);
}
impl RtApi {
    #[inline]
    pub unsafe fn openStream(
        &mut self,
        outputParameters: *mut RtAudio_StreamParameters,
        inputParameters: *mut RtAudio_StreamParameters,
        format: RtAudioFormat,
        sampleRate: c_uint,
        bufferFrames: *mut c_uint,
        callback: RtAudioCallback,
        userData: *mut c_void,
        options: *mut RtAudio_StreamOptions,
        errorCallback: RtAudioErrorCallback,
    ) {
        RtApi_openStream(
            self,
            outputParameters,
            inputParameters,
            format,
            sampleRate,
            bufferFrames,
            callback,
            userData,
            options,
            errorCallback,
        )
    }
    #[inline]
    pub unsafe fn getStreamLatency(&mut self) -> c_long {
        RtApi_getStreamLatency(self)
    }
    #[inline]
    pub unsafe fn getStreamSampleRate(&mut self) -> c_uint {
        RtApi_getStreamSampleRate(self)
    }
    #[inline]
    pub unsafe fn tickStreamTime(&mut self) {
        RtApi_tickStreamTime(self)
    }
    #[inline]
    pub unsafe fn clearStreamInfo(&mut self) {
        RtApi_clearStreamInfo(self)
    }
    #[inline]
    pub unsafe fn verifyStream(&mut self) {
        RtApi_verifyStream(self)
    }
    #[inline]
    pub unsafe fn error(&mut self, type_: crate::RtError_Type) {
        RtApi_error(self, type_)
    }
    #[inline]
    pub unsafe fn convertBuffer(
        &mut self,
        outBuffer: *mut c_char,
        inBuffer: *mut c_char,
        info: *mut RtApi_ConvertInfo,
    ) {
        RtApi_convertBuffer(self, outBuffer, inBuffer, info)
    }
    #[inline]
    pub unsafe fn byteSwapBuffer(
        &mut self,
        buffer: *mut c_char,
        samples: c_uint,
        format: RtAudioFormat,
    ) {
        RtApi_byteSwapBuffer(self, buffer, samples, format)
    }
    #[inline]
    pub unsafe fn formatBytes(&mut self, format: RtAudioFormat) -> c_uint {
        RtApi_formatBytes(self, format)
    }
    #[inline]
    pub unsafe fn setConvertInfo(&mut self, mode: RtApi_StreamMode, firstChannel: c_uint) {
        RtApi_setConvertInfo(self, mode, firstChannel)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        RtApi_RtApi(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApiD1Ev"]
    pub fn RtApi_RtApi_destructor(this: *mut RtApi);
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi21getDefaultInputDeviceEv"]
    pub fn RtApi_getDefaultInputDevice(this: *mut c_void) -> c_uint;
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi22getDefaultOutputDeviceEv"]
    pub fn RtApi_getDefaultOutputDevice(this: *mut c_void) -> c_uint;
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi11closeStreamEv"]
    pub fn RtApi_closeStream(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}_ZN5RtApi13getStreamTimeEv"]
    pub fn RtApi_getStreamTime(this: *mut c_void) -> f64;
}
extern "C" {
    #[doc = "Protected, api-specific method that attempts to open a device"]
    #[doc = "with the given parameters.  This function MUST be implemented by"]
    #[doc = "all subclasses.  If an error is encountered during the probe, a"]
    #[doc = "\"warning\" message is reported and FAILURE is returned. A"]
    #[doc = "successful probe is indicated by a return value of SUCCESS."]
    #[link_name = "\u{1}_ZN5RtApi15probeDeviceOpenEjNS_10StreamModeEjjjmPjPN7RtAudio13StreamOptionsE"]
    pub fn RtApi_probeDeviceOpen(
        this: *mut c_void,
        device: c_uint,
        mode: RtApi_StreamMode,
        channels: c_uint,
        firstChannel: c_uint,
        sampleRate: c_uint,
        format: RtAudioFormat,
        bufferSize: *mut c_uint,
        options: *mut RtAudio_StreamOptions,
    ) -> bool;
}
#[repr(C)]
pub struct RtApiJack {
    pub _base: RtApi,
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJack16getStreamLatencyEv"]
    pub fn RtApiJack_getStreamLatency(this: *mut RtApiJack) -> c_long;
}
///! This function is intended for internal use only.  It must be
///! public because it is called by the internal callback handler,
///! which is not a member of RtAudio.  External use of this function
///! will most likely produce highly undesireable results!
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJack13callbackEventEm"]
    pub fn RtApiJack_callbackEvent(this: *mut RtApiJack, nframes: c_ulong) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJackC1Ev"]
    pub fn RtApiJack_RtApiJack(this: *mut RtApiJack);
}
impl RtApiJack {
    #[inline]
    pub unsafe fn getStreamLatency(&mut self) -> c_long {
        RtApiJack_getStreamLatency(self)
    }
    ///! This function is intended for internal use only.  It must be
    ///! public because it is called by the internal callback handler,
    ///! which is not a member of RtAudio.  External use of this function
    ///! will most likely produce highly undesireable results!
    #[inline]
    pub unsafe fn callbackEvent(&mut self, nframes: c_ulong) -> bool {
        RtApiJack_callbackEvent(self, nframes)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        RtApiJack_RtApiJack(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJackD1Ev"]
    pub fn RtApiJack_RtApiJack_destructor(this: *mut RtApiJack);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJack14getDeviceCountEv"]
    pub fn RtApiJack_getDeviceCount(this: *mut c_void) -> c_uint;
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJack13getDeviceInfoEj"]
    pub fn RtApiJack_getDeviceInfo(this: *mut c_void, device: c_uint) -> RtAudio_DeviceInfo;
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJack11closeStreamEv"]
    pub fn RtApiJack_closeStream(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJack11startStreamEv"]
    pub fn RtApiJack_startStream(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJack10stopStreamEv"]
    pub fn RtApiJack_stopStream(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiJack11abortStreamEv"]
    pub fn RtApiJack_abortStream(this: *mut c_void);
}
#[repr(C)]
pub struct RtApiAlsa {
    pub _base: RtApi,
    pub devices_: vector,
}
///! This function is intended for internal use only.  It must be
///! public because it is called by the internal callback handler,
///! which is not a member of RtAudio.  External use of this function
///! will most likely produce highly undesireable results!
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsa13callbackEventEv"]
    pub fn RtApiAlsa_callbackEvent(this: *mut RtApiAlsa);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsaC1Ev"]
    pub fn RtApiAlsa_RtApiAlsa(this: *mut RtApiAlsa);
}
impl RtApiAlsa {
    #[inline]
    ///! This function is intended for internal use only.  It must be
    ///! public because it is called by the internal callback handler,
    ///! which is not a member of RtAudio.  External use of this function
    ///! will most likely produce highly undesireable results!
    pub unsafe fn callbackEvent(&mut self) {
        RtApiAlsa_callbackEvent(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        RtApiAlsa_RtApiAlsa(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsaD1Ev"]
    pub fn RtApiAlsa_RtApiAlsa_destructor(this: *mut RtApiAlsa);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsa14getDeviceCountEv"]
    pub fn RtApiAlsa_getDeviceCount(this: *mut c_void) -> c_uint;
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsa13getDeviceInfoEj"]
    pub fn RtApiAlsa_getDeviceInfo(this: *mut c_void, device: c_uint) -> RtAudio_DeviceInfo;
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsa11closeStreamEv"]
    pub fn RtApiAlsa_closeStream(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsa11startStreamEv"]
    pub fn RtApiAlsa_startStream(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsa10stopStreamEv"]
    pub fn RtApiAlsa_stopStream(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}_ZN9RtApiAlsa11abortStreamEv"]
    pub fn RtApiAlsa_abortStream(this: *mut c_void);
}
