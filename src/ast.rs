use crate::name::*;
use crate::ty::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Module(pub Vec<Def>);

#[derive(Debug, Deserialize, Serialize)]
pub struct Def {
    pub name: Id,
    pub params: Vec<Id>,
    pub expr: Exp,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Exp {
    Atom {
        atom: Atom,
    },
    Call {
        func: Atom,
        args: Vec<Atom>,
    },
    CallDirect {
        func: Id,
        args: Vec<Atom>,
    },
    ExtCall {
        ext_func: String,
        func_type: Type,
        args: Vec<Atom>,
    },
    BinOp {
        op: Op,
        left: Atom,
        right: Atom,
    },
    Cast {
        as_type: Type,
        value: Atom,
    },
    Let {
        definitions: Vec<LocalDef>,
        expr: Box<Exp>,
    },
    Match {
        scrutinee: Box<Exp>,
        clauses: Vec<Case>,
    },
    Error {
        error_type: Type,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Atom {
    Var { variable: Id },
    Unboxed { unboxed: Unboxed },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Unboxed {
    Int32 { value: i32 },
    Int64 { value: i64 },
    Float { value: f32 },
    Double { value: f64 },
    Char { value: char },
    String { value: String },
    Bool { value: bool },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    FAdd,
    FSub,
    FMul,
    FDiv,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Obj {
    Fun {
        params: Vec<Id>,
        expr: Exp,
    },
    Pack {
        pack_to: Type,
        con: Con,
        args: Vec<Atom>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Case {
    Unpack {
        con: Con,
        bindings: Vec<Id>,
        expr: Exp,
    },
    Switch {
        value: Unboxed,
        expr: Exp,
    },
    Bind {
        binding: Id,
        expr: Exp,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct LocalDef {
    variable: Id,
    object: Obj,
}
