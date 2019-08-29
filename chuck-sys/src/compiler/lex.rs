use libc::{c_char, c_int, c_void};
use std::mem::zeroed;

pub const YY_BUF_SIZE: u32 = 16384;
pub const YY_BUFFER_EOF_PENDING: u32 = 2;
pub const YY_BUFFER_NEW: u32 = 0;
pub const YY_BUFFER_NORMAL: u32 = 1;
pub const YY_DECL_IS_OURS: u32 = 1;
pub const YY_END_OF_BUFFER_CHAR: u32 = 0;
pub const YY_END_OF_BUFFER: u32 = 106;
pub const YY_EXIT_FAILURE: u32 = 2;
pub const YY_FLEX_MAJOR_VERSION: u32 = 2;
pub const YY_FLEX_MINOR_VERSION: u32 = 6;
pub const YY_FLEX_SUBMINOR_VERSION: u32 = 4;
pub const YY_MORE_ADJ: u32 = 0;
pub const YY_NULL: u32 = 0;
pub const YY_NUM_RULES: u32 = 105;
pub const YY_READ_BUF_SIZE: u32 = 8192;
pub const YY_START_STACK_INCR: u32 = 25;
pub const YYDEBUG: u32 = 0;
pub const YYSTYPE_IS_DECLARED: u32 = 1;
pub const YYSTYPE_IS_TRIVIAL: u32 = 1;
pub const YYTABLES_NAME: &'static [u8; 9usize] = b"yytables\0";
pub type YY_BUFFER_STATE = *mut self::yy_buffer_state;
pub type yy_size_t = usize;
pub const yy_buffer_stack_top: usize = 0;
pub const yy_buffer_stack_max: usize = 0;
pub const yy_init: c_int = 0;
pub const yy_start: c_int = 0;
pub type YY_CHAR = self::flex_uint8_t;
pub type yy_state_type = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut self::FILE,
    pub yy_ch_buf: *mut c_char,
    pub yy_buf_pos: *mut c_char,
    pub yy_buf_size: c_int,
    pub yy_n_chars: c_int,
    pub yy_is_our_buffer: c_int,
    pub yy_is_interactive: c_int,
    pub yy_at_bol: c_int,
    #[doc = "< The line count."]
    pub yy_bs_lineno: c_int,
    #[doc = "< The column count."]
    pub yy_bs_column: c_int,
    pub yy_fill_buffer: c_int,
    pub yy_buffer_status: c_int,
}
impl Default for yy_buffer_state {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct yy_trans_info {
    pub yy_verify: self::flex_int32_t,
    pub yy_nxt: self::flex_int32_t,
}

// external stuff
extern "C" {
    pub static mut yyleng: c_int;
}

extern "C" {
    pub static mut yyin: *mut self::FILE;
}

extern "C" {
    pub static mut yyout: *mut self::FILE;
}

extern "C" {
    pub static mut yy_buffer_stack: *mut self::YY_BUFFER_STATE;
}

extern "C" {
    pub static mut yy_hold_char: c_char;
}

extern "C" {
    pub static mut yy_n_chars: c_int;
}

extern "C" {
    pub static mut yy_c_buf_p: *mut c_char;
}

extern "C" {
    pub static mut yy_did_buffer_switch_on_eof: c_int;
}

extern "C" {
    /// Immediately switch to a different input stream.
    /// @param input_file A readable stream
    ///
    /// @note This function does not reset the start condition to @c INITIAL .
    pub fn yyrestart(input_file: *mut self::FILE);
}

extern "C" {
    /// Switch to a different input buffer.
    /// @param new_buffer The new input buffer.
    pub fn yy_switch_to_buffer(new_buffer: self::YY_BUFFER_STATE);
}

extern "C" {
    /// Allocate and initialize an input buffer state.
    /// @param file A readable stream.
    /// @param size The character buffer size in bytes. When in doubt, use @c YY_BUF_SIZE.
    ///
    /// @return the allocated buffer state.
    pub fn yy_create_buffer(file: *mut self::FILE, size: c_int) -> self::YY_BUFFER_STATE;
}

extern "C" {
    /// Destroy the buffer.
    /// @param b A buffer created with yy_create_buffer()
    pub fn yy_delete_buffer(b: self::YY_BUFFER_STATE);
}

extern "C" {
    /// Discard all buffered characters. On the next scan, YY_INPUT will be called.
    /// @param b The buffer state to be flushed, usually @c YY_CURRENT_BUFFER.
    pub fn yy_flush_buffer(b: self::YY_BUFFER_STATE);
}

extern "C" {
    /// Pushes the new state onto the stack. The new state becomes
    /// the current state. This function will allocate the stack
    /// if necessary.
    /// @param new_buffer The new state.
    pub fn yypush_buffer_state(new_buffer: self::YY_BUFFER_STATE);
}

extern "C" {
    /// Removes and deletes the top of the stack, if present.
    /// The next element becomes the new top.
    pub fn yypop_buffer_state();
}

extern "C" {
    /// Setup the input buffer state to scan directly from a user-specified character buffer.
    /// @param base The character buffer
    /// @param size The size in bytes of the character buffer
    ///
    /// @return The newly allocated buffer state object.
    pub fn yy_scan_buffer(base: *mut c_char, size: self::yy_size_t) -> self::YY_BUFFER_STATE;
}

extern "C" {
    /// Setup the input buffer state to scan a string. The next call to yylex() will
    /// scan from a @e copy of @a str.
    /// @param yystr a NUL-terminated string to scan
    ///
    /// @return the newly allocated buffer state object.
    /// @note If you want to scan bytes that may contain NUL values, then use
    ///       yy_scan_bytes() instead.
    pub fn yy_scan_string(yy_str: *const c_char) -> self::YY_BUFFER_STATE;
}

extern "C" {
    /// Setup the input buffer state to scan the given bytes. The next call to yylex() will
    /// scan from a @e copy of @a bytes.
    /// @param yybytes the byte buffer to scan
    /// @param _yybytes_len the number of bytes in the buffer pointed to by @a bytes.
    ///
    /// @return the newly allocated buffer state object.
    pub fn yy_scan_bytes(bytes: *const c_char, len: c_int) -> self::YY_BUFFER_STATE;
}

extern "C" {
    pub fn yyalloc(arg1: self::yy_size_t) -> *mut c_void;
}

extern "C" {
    pub fn yyrealloc(arg1: *mut c_void, arg2: self::yy_size_t) -> *mut c_void;
}

extern "C" {
    pub fn yyfree(arg1: *mut c_void);
}

extern "C" {
    pub static mut yylineno: c_int;
}

extern "C" {
    pub static mut yytext: *mut c_char;
}

extern "C" {
    pub static mut yy_accept: [self::flex_int16_t; 243usize];
}

extern "C" {
    pub static mut yy_ec: [self::YY_CHAR; 256usize];
}

extern "C" {
    pub static mut yy_meta: [self::YY_CHAR; 69usize];
}

extern "C" {
    pub static mut yy_base: [self::flex_int16_t; 248usize];
}

extern "C" {
    pub static mut yy_def: [self::flex_int16_t; 248usize];
}

extern "C" {
    pub static mut yy_nxt: [self::flex_int16_t; 396usize];
}

extern "C" {
    pub static mut yy_chk: [self::flex_int16_t; 396usize];
}

extern "C" {
    pub static mut yy_last_accepting_state: self::yy_state_type;
}

extern "C" {
    pub static mut yy_last_accepting_cpos: *mut c_char;
}

extern "C" {
    pub static mut yy_flex_debug: c_int;
}
