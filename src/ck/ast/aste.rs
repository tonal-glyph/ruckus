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
use libc::*;
//* ChucK abstract syntax
use crate::ck::carry::carrye::*;
use crate::dts::*;
fn main() {}
pub type a_Pos = c_int;
pub const ae_Operator_ae_op_plus: ae_Operator = 0;
pub const ae_Operator_ae_op_minus: ae_Operator = 1;
pub const ae_Operator_ae_op_times: ae_Operator = 2;
pub const ae_Operator_ae_op_divide: ae_Operator = 3;
pub const ae_Operator_ae_op_eq: ae_Operator = 4;
pub const ae_Operator_ae_op_neq: ae_Operator = 5;
pub const ae_Operator_ae_op_lt: ae_Operator = 6;
pub const ae_Operator_ae_op_le: ae_Operator = 7;
pub const ae_Operator_ae_op_gt: ae_Operator = 8;
pub const ae_Operator_ae_op_ge: ae_Operator = 9;
pub const ae_Operator_ae_op_and: ae_Operator = 10;
pub const ae_Operator_ae_op_or: ae_Operator = 11;
pub const ae_Operator_ae_op_s_or: ae_Operator = 12;
pub const ae_Operator_ae_op_s_and: ae_Operator = 13;
pub const ae_Operator_ae_op_shift_left: ae_Operator = 14;
pub const ae_Operator_ae_op_shift_right: ae_Operator = 15;
pub const ae_Operator_ae_op_percent: ae_Operator = 16;
pub const ae_Operator_ae_op_s_xor: ae_Operator = 17;
pub const ae_Operator_ae_op_chuck: ae_Operator = 18;
pub const ae_Operator_ae_op_plus_chuck: ae_Operator = 19;
pub const ae_Operator_ae_op_minus_chuck: ae_Operator = 20;
pub const ae_Operator_ae_op_times_chuck: ae_Operator = 21;
pub const ae_Operator_ae_op_divide_chuck: ae_Operator = 22;
pub const ae_Operator_ae_op_s_and_chuck: ae_Operator = 23;
pub const ae_Operator_ae_op_s_or_chuck: ae_Operator = 24;
pub const ae_Operator_ae_op_s_xor_chuck: ae_Operator = 25;
pub const ae_Operator_ae_op_shift_right_chuck: ae_Operator = 26;
pub const ae_Operator_ae_op_shift_left_chuck: ae_Operator = 27;
pub const ae_Operator_ae_op_percent_chuck: ae_Operator = 28;
pub const ae_Operator_ae_op_s_chuck: ae_Operator = 29;
pub const ae_Operator_ae_op_plusplus: ae_Operator = 30;
pub const ae_Operator_ae_op_minusminus: ae_Operator = 31;
pub const ae_Operator_ae_op_tilda: ae_Operator = 32;
pub const ae_Operator_ae_op_exclamation: ae_Operator = 33;
pub const ae_Operator_ae_op_at_chuck: ae_Operator = 34;
pub const ae_Operator_ae_op_unchuck: ae_Operator = 35;
pub const ae_Operator_ae_op_upchuck: ae_Operator = 36;
pub const ae_Operator_ae_op_spork: ae_Operator = 37;
pub const ae_Operator_ae_op_typeof: ae_Operator = 38;
pub const ae_Operator_ae_op_sizeof: ae_Operator = 39;
pub const ae_Operator_ae_op_new: ae_Operator = 40;
pub const ae_Operator_ae_op_arrow_left: ae_Operator = 41;
pub const ae_Operator_ae_op_arrow_right: ae_Operator = 42;
pub type ae_Operator = u32;
extern "C" {
    pub fn op2str(op: ae_Operator) -> *const c_char;
}
pub const ae_Keyword_ae_key_this: ae_Keyword = 0;
pub const ae_Keyword_ae_key_me: ae_Keyword = 1;
pub const ae_Keyword_ae_key_func: ae_Keyword = 2;
pub const ae_Keyword_ae_key_public: ae_Keyword = 3;
pub const ae_Keyword_ae_key_protected: ae_Keyword = 4;
pub const ae_Keyword_ae_key_private: ae_Keyword = 5;
pub const ae_Keyword_ae_key_static: ae_Keyword = 6;
pub const ae_Keyword_ae_key_instance: ae_Keyword = 7;
pub const ae_Keyword_ae_key_abstract: ae_Keyword = 8;
pub type ae_Keyword = u32;
pub type a_Program = *mut a_Program_;
pub type a_Section = *mut a_Section_;
pub type a_Stmt_List = *mut a_Stmt_List_;
pub type a_Class_Def = *mut a_Class_Def_;
pub type a_Func_Def = *mut a_Func_Def_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Code_Segment_ {
    _unused: [u8; 0],
}
pub type a_Code_Segment = *mut a_Code_Segment_;
pub type a_Stmt = *mut a_Stmt_;
pub type a_Exp = *mut a_Exp_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Chuck_ {
    _unused: [u8; 0],
}
pub type a_Exp_Chuck = *mut a_Exp_Chuck_;
pub type a_Exp_Binary = *mut a_Exp_Binary_;
pub type a_Exp_Cast = *mut a_Exp_Cast_;
pub type a_Exp_Unary = *mut a_Exp_Unary_;
pub type a_Exp_Postfix = *mut a_Exp_Postfix_;
pub type a_Exp_Primary = *mut a_Exp_Primary_;
pub type a_Exp_Dur = *mut a_Exp_Dur_;
pub type a_Exp_Array = *mut a_Exp_Array_;
pub type a_Exp_Func_Call = *mut a_Exp_Func_Call_;
pub type a_Exp_Dot_Member = *mut a_Exp_Dot_Member_;
pub type a_Exp_If = *mut a_Exp_If_;
pub type a_Exp_Decl = *mut a_Exp_Decl_;
pub type a_Exp_Hack = *mut a_Exp_Hack_;
pub type a_Stmt_Code = *mut a_Stmt_Code_;
pub type a_Stmt_If = *mut a_Stmt_If_;
pub type a_Stmt_While = *mut a_Stmt_While_;
pub type a_Stmt_Until = *mut a_Stmt_Until_;
pub type a_Stmt_For = *mut a_Stmt_For_;
pub type a_Stmt_Loop = *mut a_Stmt_Loop_;
pub type a_Stmt_Switch = *mut a_Stmt_Switch_;
pub type a_Stmt_Break = *mut a_Stmt_Break_;
pub type a_Stmt_Continue = *mut a_Stmt_Continue_;
pub type a_Stmt_Return = *mut a_Stmt_Return_;
pub type a_Stmt_Case = *mut a_Stmt_Case_;
pub type a_Stmt_GotoLabel = *mut a_Stmt_GotoLabel_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Decl_ {
    _unused: [u8; 0],
}
pub type a_Decl = *mut a_Decl_;
pub type a_Var_Decl = *mut a_Var_Decl_;
pub type a_Var_Decl_List = *mut a_Var_Decl_List_;
pub type a_Type_Decl = *mut a_Type_Decl_;
pub type a_Arg_List = *mut a_Arg_List_;
pub type a_Id_List = *mut a_Id_List_;
pub type a_Class_Ext = *mut a_Class_Ext_;
pub type a_Class_Body = *mut a_Class_Body_;
pub type a_Array_Sub = *mut a_Array_Sub_;
pub type a_Complex = *mut a_Complex_;
pub type a_Polar = *mut a_Polar_;
pub type a_Vec = *mut a_Vec_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Type {
    _unused: [u8; 0],
}
pub type t_CKTYPE = *mut Chuck_Type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Value {
    _unused: [u8; 0],
}
pub type t_CKVALUE = *mut Chuck_Value;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Func {
    _unused: [u8; 0],
}
pub type t_CKFUNC = *mut Chuck_Func;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Namespace {
    _unused: [u8; 0],
}
pub type t_CKNSPC = *mut Chuck_Namespace;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_Code {
    _unused: [u8; 0],
}
pub type t_CKVMCODE = *mut Chuck_VM_Code;
extern "C" {
    pub fn new_program(section: a_Section, pos: c_int) -> a_Program;
}
extern "C" {
    pub fn prepend_program(section: a_Section, program: a_Program, pos: c_int) -> a_Program;
}
extern "C" {
    pub fn new_section_stmt(stmt_list: a_Stmt_List, pos: c_int) -> a_Section;
}
extern "C" {
    pub fn new_section_func_def(func_def: a_Func_Def, pos: c_int) -> a_Section;
}
extern "C" {
    pub fn new_section_class_def(class_def: a_Class_Def, pos: c_int) -> a_Section;
}
extern "C" {
    pub fn new_stmt_list(stmt: a_Stmt, pos: c_int) -> a_Stmt_List;
}
extern "C" {
    pub fn prepend_stmt_list(stmt: a_Stmt, stmt_list: a_Stmt_List, pos: c_int) -> a_Stmt_List;
}
extern "C" {
    pub fn new_stmt_from_expression(exp: a_Exp, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_code(code: a_Stmt_List, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_while(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_do_while(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_until(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_do_until(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_for(c1: a_Stmt, c2: a_Stmt, c3: a_Exp, body: a_Stmt, pos: c_int)
        -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_loop(cond: a_Exp, body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_if(cond: a_Exp, if_body: a_Stmt, else_body: a_Stmt, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_switch(exp: a_Exp, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_break(pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_continue(pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_return(exp: a_Exp, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_label(xid: c_str, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn new_stmt_from_case(exp: a_Exp, pos: c_int) -> a_Stmt;
}
extern "C" {
    pub fn prepend_expression(exp: a_Exp, list: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_binary(lhs: a_Exp, oper: ae_Operator, rhs: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary(oper: ae_Operator, exp: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary2(
        oper: ae_Operator,
        type_: a_Type_Decl,
        array: a_Array_Sub,
        pos: c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_unary3(oper: ae_Operator, code: a_Stmt, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_cast(type_: a_Type_Decl, exp: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_array(base: a_Exp, indices: a_Array_Sub, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_array_lit(exp_list: a_Array_Sub, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_func_call(base: a_Exp, args: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_member_dot(base: a_Exp, member: c_str, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_postfix(base: a_Exp, op: ae_Operator, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_dur(base: a_Exp, unit: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_id(xid: c_str, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_int(num: c_long, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_uint(num: c_ulong, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_float(num: f64, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_str(str: c_str, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_char(chr: c_str, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_if(cond: a_Exp, lhs: a_Exp, rhs: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_complex(arg1: a_Complex, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_polar(arg1: a_Polar, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_vec(arg1: a_Vec, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl_external(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl_global(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_decl(
        type_decl: a_Type_Decl,
        var_decl_list: a_Var_Decl_List,
        is_static: c_int,
        pos: c_int,
    ) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_hack(exp: a_Exp, pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_exp_from_nil(pos: c_int) -> a_Exp;
}
extern "C" {
    pub fn new_var_decl_list(var_decl: a_Var_Decl, pos: c_int) -> a_Var_Decl_List;
}
extern "C" {
    pub fn prepend_var_decl_list(
        var_decl: a_Var_Decl,
        list: a_Var_Decl_List,
        pos: c_int,
    ) -> a_Var_Decl_List;
}
extern "C" {
    pub fn new_var_decl(xid: c_constr, array: a_Array_Sub, pos: c_int) -> a_Var_Decl;
}
extern "C" {
    pub fn new_type_decl(xid: a_Id_List, ref_: c_int, pos: c_int) -> a_Type_Decl;
}
extern "C" {
    pub fn add_type_decl_array(
        type_decl: a_Type_Decl,
        array: a_Array_Sub,
        pos: c_int,
    ) -> a_Type_Decl;
}
extern "C" {
    pub fn new_arg_list(type_decl: a_Type_Decl, var_decl: a_Var_Decl, pos: c_int) -> a_Arg_List;
}
extern "C" {
    pub fn prepend_arg_list(
        type_decl: a_Type_Decl,
        var_decl: a_Var_Decl,
        arg_list: a_Arg_List,
        pos: c_int,
    ) -> a_Arg_List;
}
extern "C" {
    pub fn new_array_sub(exp: a_Exp, pos: c_int) -> a_Array_Sub;
}
extern "C" {
    pub fn prepend_array_sub(array: a_Array_Sub, exp: a_Exp, pos: c_int) -> a_Array_Sub;
}
extern "C" {
    pub fn new_complex(re: a_Exp, pos: c_int) -> a_Complex;
}
extern "C" {
    pub fn new_polar(mod_: a_Exp, pos: c_int) -> a_Polar;
}
extern "C" {
    pub fn new_vec(e: a_Exp, pos: c_int) -> a_Vec;
}
extern "C" {
    pub fn new_class_def(
        class_decl: ae_Keyword,
        xid: a_Id_List,
        ext: a_Class_Ext,
        body: a_Class_Body,
        pos: c_int,
    ) -> a_Class_Def;
}
extern "C" {
    pub fn new_class_body(section: a_Section, pos: c_int) -> a_Class_Body;
}
extern "C" {
    pub fn prepend_class_body(section: a_Section, body: a_Class_Body, pos: c_int) -> a_Class_Body;
}
extern "C" {
    pub fn new_class_ext(extend_id: a_Id_List, impl_list: a_Id_List, pos: c_int) -> a_Class_Ext;
}
extern "C" {
    pub fn new_iface_def(
        class_decl: ae_Keyword,
        xid: a_Id_List,
        ext: a_Class_Ext,
        body: a_Class_Body,
        pos: c_int,
    ) -> a_Class_Def;
}
extern "C" {
    pub fn new_id_list(xid: c_constr, pos: c_int) -> a_Id_List;
}
extern "C" {
    pub fn prepend_id_list(xid: c_constr, list: a_Id_List, pos: c_int) -> a_Id_List;
}
extern "C" {
    pub fn clean_exp(exp: a_Exp);
}
extern "C" {
    pub fn new_func_def(
        func_decl: ae_Keyword,
        static_decl: ae_Keyword,
        type_decl: a_Type_Decl,
        name: c_str,
        arg_list: a_Arg_List,
        code: a_Stmt,
        pos: c_int,
    ) -> a_Func_Def;
}
extern "C" {
    pub fn delete_id_list(x: a_Id_List);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Binary_ {
    pub lhs: a_Exp,
    pub op: ae_Operator,
    pub rhs: a_Exp,
    pub ck_func: t_CKFUNC,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Cast_ {
    pub type_: a_Type_Decl,
    pub exp: a_Exp,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Unary_ {
    pub op: ae_Operator,
    pub exp: a_Exp,
    pub type_: a_Type_Decl,
    pub array: a_Array_Sub,
    pub code: a_Stmt,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Postfix_ {
    pub exp: a_Exp,
    pub op: ae_Operator,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Dur_ {
    pub base: a_Exp,
    pub unit: a_Exp,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Array_ {
    pub base: a_Exp,
    pub indices: a_Array_Sub,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Func_Call_ {
    pub func: a_Exp,
    pub args: a_Exp,
    pub ret_type: t_CKTYPE,
    pub ck_func: t_CKFUNC,
    pub ck_vm_code: t_CKVMCODE,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Dot_Member_ {
    pub base: a_Exp,
    pub t_base: t_CKTYPE,
    pub xid: S_Symbol,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_If_ {
    pub cond: a_Exp,
    pub if_exp: a_Exp,
    pub else_exp: a_Exp,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Decl_ {
    pub type_: a_Type_Decl,
    pub var_decl_list: a_Var_Decl_List,
    pub num_var_decls: c_int,
    pub is_static: c_int,
    pub is_global: c_int,
    pub ck_type: t_CKTYPE,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Exp_Hack_ {
    pub exp: a_Exp,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Var_Decl_List_ {
    pub var_decl: a_Var_Decl,
    pub next: a_Var_Decl_List,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Var_Decl_ {
    pub xid: crate::chuck_absyn_h_edited::S_Symbol,
    pub var_decl: a_Var_Decl,
    pub array: a_Array_Sub,
    pub value: t_CKVALUE,
    pub addr: *mut c_void,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Type_Decl_ {
    pub xid: a_Id_List,
    pub array: a_Array_Sub,
    pub ref_: c_int,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Array_Sub_ {
    pub depth: c_ulong,
    pub exp_list: a_Exp,
    pub linepos: c_int,
    pub self_: a_Exp,
    pub err_num: c_int,
    pub err_pos: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Arg_List_ {
    pub type_decl: a_Type_Decl,
    pub var_decl: a_Var_Decl,
    pub type_: t_CKTYPE,
    pub next: a_Arg_List,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Complex_ {
    pub re: a_Exp,
    pub im: a_Exp,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Polar_ {
    pub mod_: a_Exp,
    pub phase: a_Exp,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Vec_ {
    pub args: a_Exp,
    pub numdims: c_int,
    pub linepos: c_int,
    pub self_: a_Exp,
}
pub const ae_Exp_Primary_Type_ae_primary_var: ae_Exp_Primary_Type = 0;
pub const ae_Exp_Primary_Type_ae_primary_num: ae_Exp_Primary_Type = 1;
pub const ae_Exp_Primary_Type_ae_primary_float: ae_Exp_Primary_Type = 2;
pub const ae_Exp_Primary_Type_ae_primary_str: ae_Exp_Primary_Type = 3;
pub const ae_Exp_Primary_Type_ae_primary_array: ae_Exp_Primary_Type = 4;
pub const ae_Exp_Primary_Type_ae_primary_exp: ae_Exp_Primary_Type = 5;
pub const ae_Exp_Primary_Type_ae_primary_hack: ae_Exp_Primary_Type = 6;
pub const ae_Exp_Primary_Type_ae_primary_complex: ae_Exp_Primary_Type = 7;
pub const ae_Exp_Primary_Type_ae_primary_polar: ae_Exp_Primary_Type = 8;
pub const ae_Exp_Primary_Type_ae_primary_vec: ae_Exp_Primary_Type = 9;
pub const ae_Exp_Primary_Type_ae_primary_char: ae_Exp_Primary_Type = 10;
pub const ae_Exp_Primary_Type_ae_primary_nil: ae_Exp_Primary_Type = 11;
pub type ae_Exp_Primary_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Exp_Primary_ {
    pub s_type: ae_Exp_Primary_Type,
    pub value: t_CKVALUE,
    pub __bindgen_anon_1: a_Exp_Primary___bindgen_ty_1,
    pub linepos: c_int,
    pub self_: a_Exp,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Exp_Primary___bindgen_ty_1 {
    pub var: S_Symbol,
    pub num: c_long,
    pub fnum: f64,
    pub str: c_str,
    pub chr: c_str,
    pub array: a_Array_Sub,
    pub exp: a_Exp,
    pub complex: a_Complex,
    pub polar: a_Polar,
    pub vec: a_Vec,
    _bindgen_union_align: u64,
}
pub const ae_Exp_Type_ae_exp_binary: ae_Exp_Type = 0;
pub const ae_Exp_Type_ae_exp_unary: ae_Exp_Type = 1;
pub const ae_Exp_Type_ae_exp_cast: ae_Exp_Type = 2;
pub const ae_Exp_Type_ae_exp_postfix: ae_Exp_Type = 3;
pub const ae_Exp_Type_ae_exp_dur: ae_Exp_Type = 4;
pub const ae_Exp_Type_ae_exp_primary: ae_Exp_Type = 5;
pub const ae_Exp_Type_ae_exp_array: ae_Exp_Type = 6;
pub const ae_Exp_Type_ae_exp_func_call: ae_Exp_Type = 7;
pub const ae_Exp_Type_ae_exp_dot_member: ae_Exp_Type = 8;
pub const ae_Exp_Type_ae_exp_if: ae_Exp_Type = 9;
pub const ae_Exp_Type_ae_exp_decl: ae_Exp_Type = 10;
pub type ae_Exp_Type = u32;
pub const ae_Exp_Meta_ae_meta_value: ae_Exp_Meta = 0;
pub const ae_Exp_Meta_ae_meta_var: ae_Exp_Meta = 1;
pub type ae_Exp_Meta = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Exp_ {
    pub s_type: ae_Exp_Type,
    pub s_meta: ae_Exp_Meta,
    pub type_: t_CKTYPE,
    pub owner: t_CKNSPC,
    pub next: a_Exp,
    pub group_size: c_ulong,
    pub cast_to: t_CKTYPE,
    pub emit_var: c_ulong,
    pub __bindgen_anon_1: a_Exp___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Exp___bindgen_ty_1 {
    pub binary: a_Exp_Binary_,
    pub unary: a_Exp_Unary_,
    pub cast: a_Exp_Cast_,
    pub postfix: a_Exp_Postfix_,
    pub dur: a_Exp_Dur_,
    pub primary: a_Exp_Primary_,
    pub array: a_Exp_Array_,
    pub func_call: a_Exp_Func_Call_,
    pub dot_member: a_Exp_Dot_Member_,
    pub exp_if: a_Exp_If_,
    pub decl: a_Exp_Decl_,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_While_ {
    pub is_do: c_int,
    pub cond: a_Exp,
    pub body: a_Stmt,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_Until_ {
    pub is_do: c_int,
    pub cond: a_Exp,
    pub body: a_Stmt,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_For_ {
    pub c1: a_Stmt,
    pub c2: a_Stmt,
    pub c3: a_Exp,
    pub body: a_Stmt,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_Loop_ {
    pub cond: a_Exp,
    pub body: a_Stmt,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_Code_ {
    pub stmt_list: a_Stmt_List,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_If_ {
    pub cond: a_Exp,
    pub if_body: a_Stmt,
    pub else_body: a_Stmt,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_Switch_ {
    pub val: a_Exp,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_Break_ {
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_Continue_ {
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_Return_ {
    pub val: a_Exp,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_Case_ {
    pub exp: a_Exp,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_GotoLabel_ {
    pub name: S_Symbol,
    pub linepos: c_int,
    pub self_: a_Stmt,
}
pub const ae_Stmt_Type_ae_stmt_exp: ae_Stmt_Type = 0;
pub const ae_Stmt_Type_ae_stmt_while: ae_Stmt_Type = 1;
pub const ae_Stmt_Type_ae_stmt_until: ae_Stmt_Type = 2;
pub const ae_Stmt_Type_ae_stmt_for: ae_Stmt_Type = 3;
pub const ae_Stmt_Type_ae_stmt_loop: ae_Stmt_Type = 4;
pub const ae_Stmt_Type_ae_stmt_if: ae_Stmt_Type = 5;
pub const ae_Stmt_Type_ae_stmt_code: ae_Stmt_Type = 6;
pub const ae_Stmt_Type_ae_stmt_switch: ae_Stmt_Type = 7;
pub const ae_Stmt_Type_ae_stmt_break: ae_Stmt_Type = 8;
pub const ae_Stmt_Type_ae_stmt_continue: ae_Stmt_Type = 9;
pub const ae_Stmt_Type_ae_stmt_return: ae_Stmt_Type = 10;
pub const ae_Stmt_Type_ae_stmt_case: ae_Stmt_Type = 11;
pub const ae_Stmt_Type_ae_stmt_gotolabel: ae_Stmt_Type = 12;
pub type ae_Stmt_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Stmt_ {
    pub s_type: ae_Stmt_Type,
    pub skip: c_int,
    pub __bindgen_anon_1: a_Stmt___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Stmt___bindgen_ty_1 {
    pub stmt_exp: a_Exp,
    pub stmt_code: a_Stmt_Code_,
    pub stmt_while: a_Stmt_While_,
    pub stmt_until: a_Stmt_Until_,
    pub stmt_loop: a_Stmt_Loop_,
    pub stmt_for: a_Stmt_For_,
    pub stmt_if: a_Stmt_If_,
    pub stmt_switch: a_Stmt_Switch_,
    pub stmt_break: a_Stmt_Break_,
    pub stmt_continue: a_Stmt_Continue_,
    pub stmt_return: a_Stmt_Return_,
    pub stmt_case: a_Stmt_Case_,
    pub stmt_gotolabel: a_Stmt_GotoLabel_,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Stmt_List_ {
    pub stmt: a_Stmt,
    pub next: a_Stmt_List,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Class_Def_ {
    pub decl: ae_Keyword,
    pub name: a_Id_List,
    pub ext: a_Class_Ext,
    pub body: a_Class_Body,
    pub type_: t_CKTYPE,
    pub iface: c_int,
    pub home: t_CKNSPC,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Class_Ext_ {
    pub extend_id: a_Id_List,
    pub impl_list: a_Id_List,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Class_Body_ {
    pub section: a_Section,
    pub next: a_Class_Body,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Id_List_ {
    pub xid: S_Symbol,
    pub next: a_Id_List,
    pub linepos: c_int,
}
pub const ae_Func_Type_ae_func_user: ae_Func_Type = 0;
pub const ae_Func_Type_ae_func_builtin: ae_Func_Type = 1;
pub type ae_Func_Type = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Func_Def_ {
    pub func_decl: ae_Keyword,
    pub static_decl: ae_Keyword,
    pub type_decl: a_Type_Decl,
    pub ret_type: t_CKTYPE,
    pub name: S_Symbol,
    pub arg_list: a_Arg_List,
    pub code: a_Stmt,
    pub ck_func: t_CKFUNC,
    pub global: c_uint,
    pub s_type: c_uint,
    pub stack_depth: c_uint,
    pub dl_func_ptr: *mut c_void,
    pub linepos: c_int,
}
pub const ae_Section_Type_ae_section_stmt: ae_Section_Type = 0;
pub const ae_Section_Type_ae_section_func: ae_Section_Type = 1;
pub const ae_Section_Type_ae_section_class: ae_Section_Type = 2;
pub type ae_Section_Type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_Section_ {
    pub s_type: ae_Section_Type,
    pub __bindgen_anon_1: a_Section___bindgen_ty_1,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_Section___bindgen_ty_1 {
    pub stmt_list: a_Stmt_List,
    pub class_def: a_Class_Def,
    pub func_def: a_Func_Def,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a_Program_ {
    pub section: a_Section,
    pub next: a_Program,
    pub linepos: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S_Symbol_ {
    _unused: [u8; 0],
}
pub type S_Symbol = *mut S_Symbol_;
pub struct TAB_table_ {
    _unused: [u8; 0],
}
pub type S_table = *mut TAB_table_;
extern "C" {
    pub fn insert_symbol(arg1: crate::dts::c_constr) -> S_Symbol;
}
extern "C" {
    pub fn S_name(arg1: S_Symbol) -> crate::dts::c_str;
}
extern "C" {
    pub fn S_empty() -> S_table;
}
extern "C" {
    pub fn S_empty2(size: c_uint) -> S_table;
}
extern "C" {
    pub fn S_enter(t: S_table, sym: S_Symbol, value: *mut c_void);
}
extern "C" {
    pub fn S_enter2(t: S_table, str: c_constr, value: *mut c_void);
}
extern "C" {
    pub fn S_look(t: S_table, sym: S_Symbol) -> *mut c_void;
}
extern "C" {
    pub fn S_look2(t: S_table, str: c_constr) -> *mut c_void;
}
extern "C" {
    pub fn S_beginScope(t: S_table);
}
extern "C" {
    pub fn S_endScope(t: S_table);
}
extern "C" {
    pub fn S_pop(t: S_table);
}
