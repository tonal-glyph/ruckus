use libc::{FILE};
use crate::def::c_str;
use crate::compiler::absyn::*;
use crate::compiler::tab::*;
use std::mem::zeroed;
use std::fmt::{Debug, Formatter};
// syntax types (for highlighting)
enum SyntaxType
{
    COMMA = 0,
    SEMICOLON,
    DBLCOLON,
    PAREN,
    DOT,
    CHUCK_OP,
    OPERATOR,
    KEYWORD,
    DEBUG_PRINT,
    SPORK,
    INTEGER,
    FLOATING,
    STRING,
    COMMENT,
    OTHER,
    NUM_SYNTAX_TYPES
}
impl SyntaxType {
    pub const VARIANTS: [SyntaxType; 16] = [
        SyntaxType::COMMA,
        SyntaxType::SEMICOLON,
        SyntaxType::DBLCOLON,
        SyntaxType::PAREN,
        SyntaxType::DOT,
        SyntaxType::CHUCK_OP,
        SyntaxType::OPERATOR,
        SyntaxType::KEYWORD,
        SyntaxType::DEBUG_PRINT,
        SyntaxType::SPORK,
        SyntaxType::INTEGER,
        SyntaxType::FLOATING,
        SyntaxType::STRING,
        SyntaxType::COMMENT,
        SyntaxType::OTHER,
        SyntaxType::NUM_SYNTAX_TYPES,
    ];
}
#[repr(C)]
pub struct SyntaxToken {
    pub token: libc::c_char,
    pub type_: libc::c_ulong,
    pub begin: self::SyntaxToken_size_type,
    pub end: self::SyntaxToken_size_type,
}
impl Default for SyntaxToken {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for SyntaxToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "SyntaxToken {{ token: {:?}, type: {:?}, begin: {:?}, end: {:?} }}",
            self.token, self.type_, self.begin, self.end
        )
    }
}
#[repr(C)]
pub struct SyntaxTokenList {
    pub list: Vec<libc::c_char>,
    pub howmany: self::SyntaxTokenList_size_type,
}
impl Default for SyntaxTokenList {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl Debug for SyntaxTokenList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "SyntaxTokenList {{ list: {:?}, howmany: {:?} }}",
            self.list, self.howmany
        )
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct SyntaxQuery {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}parseLine"]
    pub fn SyntaxQuery_parseLine(
        this: *mut self::SyntaxQuery,
        line: *const libc::c_char,
        tokens: *mut self::SyntaxTokenList,
    ) -> libc::c_ulong;
}
impl SyntaxQuery {
    #[inline]
    pub unsafe fn parseLine(
        &mut self,
        line: *const libc::c_char,
        tokens: *mut self::SyntaxTokenList,
    ) -> libc::c_ulong {
        SyntaxQuery_parseLine(self, line, tokens)
    }
}
extern "C" {
    pub fn open_cat_ck(filename: c_str) -> *mut FILE;
}
extern "C" {
    pub fn chuck_parse(
        fname: libc::c_char,
        fd: *mut FILE,
        code: libc::c_char,
    ) -> libc::c_ulong;
}
extern "C" {
    pub fn reset_parse();
}
