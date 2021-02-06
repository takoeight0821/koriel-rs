use std::collections::HashSet;

#[derive(Debug)]
pub struct Con(String, Vec<Type>);

#[derive(Debug)]
pub struct Type {
    pub kind: TypeKind,
}

#[derive(Debug)]
pub enum TypeKind {
    FunT(Vec<Type>, Box<Type>),
    Int32T,
    Int64T,
    FloatT,
    DoubleT,
    CharT,
    StringT,
    BoolT,
    SumT(HashSet<Con>),
    PtrT(Box<Type>),
    AnyT,
    VoidT,
}

