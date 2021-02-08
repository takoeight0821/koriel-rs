use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Con {
    pub tag: String,
    pub params: Vec<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Type {
    FunT {
        params: Vec<Type>,
        result: Box<Type>,
    },
    Int32T,
    Int64T,
    FloatT,
    DoubleT,
    CharT,
    StringT,
    BoolT,
    SumT {
        con_set: Vec<Con>,
    },
    PtrT {
        ptr_to: Box<Type>,
    },
    AnyT,
    VoidT,
}
