use crate::ty::*;
use crate::name::*;

#[derive(Debug)]
pub struct Exp {
    pub kind: ExpKind,
}

#[derive(Debug)]
pub enum ExpKind {
    Atom(Atom),
    Call(Atom, Vec<Atom>),
    CallDirect(Name, Vec<Atom>),
    ExtCall(String, Type, Vec<Atom>),
    BinOp(Op, Atom, Atom),
    Cast(Type, Atom),
    Let(Vec<(Name, Obj)>, Box<Exp>),
    Match(Box<Exp>, Vec<Case>),
    Error(Type),
}

#[derive(Debug)]
pub struct Atom {
    pub kind: AtomKind,
}

#[derive(Debug)]
pub enum AtomKind {
    Var(Name),
    Unboxed(Unboxed),
}

#[derive(Debug)]
pub struct Unboxed {
    pub kind: UnboxedKind,
}

#[derive(Debug)]
pub enum UnboxedKind {
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Char(char),
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Obj {
    pub kind: ObjKind,
}

#[derive(Debug)]
pub enum ObjKind {
    Fun(Vec<Name>, Exp),
    Pack(Type, Con, Vec<Atom>),
}

#[derive(Debug)]
pub struct Case {
    pub kind: CaseKind,
}

#[derive(Debug)]
pub enum CaseKind {
    Unpack(Con, Vec<Name>, Exp),
    Switch(Unboxed, Exp),
    Bind(Name, Exp),
}

