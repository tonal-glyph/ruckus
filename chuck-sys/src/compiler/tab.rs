use libc;
use crate::compiler::absyn::*;

pub const YYBISON_VERSION: &'static [u8; 4usize] = b"3.2\0";
pub const YYBISON: u32 = 1;
pub const YYCOPY_NEEDED: u32 = 1;
pub const YYDEBUG: u32 = 0;
pub const YYEMPTY: i32 = -2;
pub const YYEOF: u32 = 0;
pub const YYERRCODE: u32 = 256;
pub const YYERROR_VERBOSE: u32 = 0;
pub const YYFINAL: u32 = 122;
pub const YYINITDEPTH: u32 = 200;
pub const YYLAST: u32 = 1181;
pub const YYMAXDEPTH: u32 = 10000;
pub const YYMAXUTOK: u32 = 366;
pub const YYNNTS: u32 = 58;
pub const YYNRULES: u32 = 184;
pub const YYNSTATES: u32 = 319;
pub const YYNTOKENS: u32 = 112;
pub const YYPACT_NINF: i32 = -246;
pub const YYPULL: u32 = 1;
pub const YYPURE: u32 = 0;
pub const YYPUSH: u32 = 0;
pub const YYSKELETON_NAME: &'static [u8; 7usize] = b"yacc.c\0";
pub const YYSTYPE_IS_DECLARED: u32 = 1;
pub const YYSTYPE_IS_TRIVIAL: u32 = 1;
pub const YYTABLE_NINF: i32 = -42;
pub const YYTERROR: u32 = 1;
pub const YYUNDEFTOK: u32 = 2;
pub type yytype_uint8 = libc::c_uchar;
pub type yytype_int8 = libc::c_schar;
pub type yytype_uint16 = libc::c_ushort;
pub type yytype_int16 = libc::c_short;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum yytokentype {
    ID = 258,
    STRING_LIT = 259,
    CHAR_LIT = 260,
    NUM = 261,
    FLOAT = 262,
    POUND = 263,
    COMMA = 264,
    COLON = 265,
    SEMICOLON = 266,
    LPAREN = 267,
    RPAREN = 268,
    LBRACK = 269,
    RBRACK = 270,
    LBRACE = 271,
    RBRACE = 272,
    DOT = 273,
    PLUS = 274,
    MINUS = 275,
    TIMES = 276,
    DIVIDE = 277,
    PERCENT = 278,
    EQ = 279,
    NEQ = 280,
    LT = 281,
    LE = 282,
    GT = 283,
    GE = 284,
    AND = 285,
    OR = 286,
    ASSIGN = 287,
    IF = 288,
    THEN = 289,
    ELSE = 290,
    WHILE = 291,
    FOR = 292,
    DO = 293,
    LOOP = 294,
    BREAK = 295,
    CONTINUE = 296,
    NULL_TOK = 297,
    FUNCTION = 298,
    RETURN = 299,
    QUESTION = 300,
    EXCLAMATION = 301,
    S_OR = 302,
    S_AND = 303,
    S_XOR = 304,
    PLUSPLUS = 305,
    MINUSMINUS = 306,
    DOLLAR = 307,
    POUNDPAREN = 308,
    PERCENTPAREN = 309,
    ATPAREN = 310,
    SIMULT = 311,
    PATTERN = 312,
    CODE = 313,
    TRANSPORT = 314,
    HOST = 315,
    TIME = 316,
    WHENEVER = 317,
    NEXT = 318,
    UNTIL = 319,
    EXTERNAL = 320,
    GLOBAL = 321,
    EVERY = 322,
    BEFORE = 323,
    AFTER = 324,
    AT = 325,
    AT_SYM = 326,
    ATAT_SYM = 327,
    NEW = 328,
    SIZEOF = 329,
    TYPEOF = 330,
    SAME = 331,
    PLUS_CHUCK = 332,
    MINUS_CHUCK = 333,
    TIMES_CHUCK = 334,
    DIVIDE_CHUCK = 335,
    S_AND_CHUCK = 336,
    S_OR_CHUCK = 337,
    S_XOR_CHUCK = 338,
    SHIFT_RIGHT_CHUCK = 339,
    SHIFT_LEFT_CHUCK = 340,
    PERCENT_CHUCK = 341,
    SHIFT_RIGHT = 342,
    SHIFT_LEFT = 343,
    TILDA = 344,
    CHUCK = 345,
    COLONCOLON = 346,
    S_CHUCK = 347,
    AT_CHUCK = 348,
    LEFT_S_CHUCK = 349,
    UNCHUCK = 350,
    UPCHUCK = 351,
    CLASS = 352,
    INTERFACE = 353,
    EXTENDS = 354,
    IMPLEMENTS = 355,
    PUBLIC = 356,
    PROTECTED = 357,
    PRIVATE = 358,
    STATIC = 359,
    ABSTRACT = 360,
    CONST = 361,
    SPORK = 362,
    ARROW_RIGHT = 363,
    ARROW_LEFT = 364,
    L_HACK = 365,
    R_HACK = 366,
}
impl yytokentype {
    pub const VARIANTS: [yytokentype; 109] = [
    yytokentype::ID,
    yytokentype::STRING_LIT,
    yytokentype::CHAR_LIT,
    yytokentype::NUM,
    yytokentype::FLOAT,
    yytokentype::POUND,
    yytokentype::COMMA,
    yytokentype::COLON,
    yytokentype::SEMICOLON,
    yytokentype::LPAREN,
    yytokentype::RPAREN,
    yytokentype::LBRACK,
    yytokentype::RBRACK,
    yytokentype::LBRACE,
    yytokentype::RBRACE,
    yytokentype::DOT,
    yytokentype::PLUS,
    yytokentype::MINUS,
    yytokentype::TIMES,
    yytokentype::DIVIDE,
    yytokentype::PERCENT,
    yytokentype::EQ,
    yytokentype::NEQ,
    yytokentype::LT,
    yytokentype::LE,
    yytokentype::GT,
    yytokentype::GE,
    yytokentype::AND,
    yytokentype::OR,
    yytokentype::ASSIGN,
    yytokentype::IF,
    yytokentype::THEN,
    yytokentype::ELSE,
    yytokentype::WHILE,
    yytokentype::FOR,
    yytokentype::DO,
    yytokentype::LOOP,
    yytokentype::BREAK,
    yytokentype::CONTINUE,
    yytokentype::NULL_TOK,
    yytokentype::FUNCTION,
    yytokentype::RETURN,
    yytokentype::QUESTION,
    yytokentype::EXCLAMATION,
    yytokentype::S_OR,
    yytokentype::S_AND,
    yytokentype::S_XOR,
    yytokentype::PLUSPLUS,
    yytokentype::MINUSMINUS,
    yytokentype::DOLLAR,
    yytokentype::POUNDPAREN,
    yytokentype::PERCENTPAREN,
    yytokentype::ATPAREN,
    yytokentype::SIMULT,
    yytokentype::PATTERN,
    yytokentype::CODE,
    yytokentype::TRANSPORT,
    yytokentype::HOST,
    yytokentype::TIME,
    yytokentype::WHENEVER,
    yytokentype::NEXT,
    yytokentype::UNTIL,
    yytokentype::EXTERNAL,
    yytokentype::GLOBAL,
    yytokentype::EVERY,
    yytokentype::BEFORE,
    yytokentype::AFTER,
    yytokentype::AT,
    yytokentype::AT_SYM,
    yytokentype::ATAT_SYM,
    yytokentype::NEW,
    yytokentype::SIZEOF,
    yytokentype::TYPEOF,
    yytokentype::SAME,
    yytokentype::PLUS_CHUCK,
    yytokentype::MINUS_CHUCK,
    yytokentype::TIMES_CHUCK,
    yytokentype::DIVIDE_CHUCK,
    yytokentype::S_AND_CHUCK,
    yytokentype::S_OR_CHUCK,
    yytokentype::S_XOR_CHUCK,
    yytokentype::SHIFT_RIGHT_CHUCK,
    yytokentype::SHIFT_LEFT_CHUCK,
    yytokentype::PERCENT_CHUCK,
    yytokentype::SHIFT_RIGHT,
    yytokentype::SHIFT_LEFT,
    yytokentype::TILDA,
    yytokentype::CHUCK,
    yytokentype::COLONCOLON,
    yytokentype::S_CHUCK,
    yytokentype::AT_CHUCK,
    yytokentype::LEFT_S_CHUCK,
    yytokentype::UNCHUCK,
    yytokentype::UPCHUCK,
    yytokentype::CLASS,
    yytokentype::INTERFACE,
    yytokentype::EXTENDS,
    yytokentype::IMPLEMENTS,
    yytokentype::PUBLIC,
    yytokentype::PROTECTED,
    yytokentype::PRIVATE,
    yytokentype::STATIC,
    yytokentype::ABSTRACT,
    yytokentype::CONST,
    yytokentype::SPORK,
    yytokentype::ARROW_RIGHT,
    yytokentype::ARROW_LEFT,
    yytokentype::L_HACK,
    yytokentype::R_HACK,
    ];
}

#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union YYSTYPE {
    pub pos: libc::c_int,
    pub ival: libc::c_int,
    pub fval: f64,
    pub sval: libc::c_str,
    pub program: a_Program,
    pub program_section: a_Section,
    pub stmt_list: a_Stmt_List,
    pub class_def: a_Class_Def,
    pub class_ext: a_Class_Ext,
    pub class_body: a_Class_Body,
    pub stmt: a_Stmt,
    pub exp: a_Exp,
    pub func_def: a_Func_Def,
    pub var_decl_list: a_Var_Decl_List,
    pub var_decl: a_Var_Decl,
    pub type_decl: a_Type_Decl,
    pub arg_list: a_Arg_List,
    pub id_list: a_Id_List,
    pub array_sub: a_Array_Sub,
    pub complex_exp: a_Complex,
    pub polar_exp: a_Polar,
    pub vec_exp: a_Vec,
    _bindgen_union_align: u64,
}
impl Default for YYSTYPE {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for YYSTYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "YYSTYPE {{ union }}")
    }
}

#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: YYSTYPE,
}
impl Default for yyalloc {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
impl ::std::fmt::Debug for yyalloc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "yyalloc {{ union }}")
    }
}

// external stuff
extern "C" {
    pub fn yylex() -> libc::c_int;
}

extern "C" {
    pub fn yyerror(s: *mut libc::c_char);
}

extern "C" {
    pub static mut g_program: a_Program;
}

extern "C" {
    pub static mut yylval: YYSTYPE;
}

extern "C" {
    pub fn yyparse() -> libc::c_int;
}

extern "C" {
    pub static mut yytranslate: [yytype_uint8; 367usize];
}

extern "C" {
    pub static mut yypact: [yytype_int16; 319usize];
}

extern "C" {
    pub static mut yydefact: [yytype_uint8; 319usize];
}

extern "C" {
    pub static mut yypgoto: [yytype_int16; 58usize];
}

extern "C" {
    pub static mut yydefgoto: [yytype_int16; 58usize];
}

extern "C" {
    pub static mut yytable: [yytype_int16; 1182usize];
}

extern "C" {
    pub static mut yycheck: [yytype_int16; 1182usize];
}

extern "C" {
    pub static mut yystos: [yytype_uint8; 319usize];
}

extern "C" {
    pub static mut yyr1: [yytype_uint8; 185usize];
}

extern "C" {
    pub static mut yyr2: [yytype_uint8; 185usize];
}

extern "C" {
    pub static mut yychar: libc::c_int;
}

extern "C" {
    pub static mut yynerrs: libc::c_int;
}
