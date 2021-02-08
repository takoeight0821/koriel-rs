use crate::ty::Type;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Id {
    name: String,
    uniq: i64,
    meta: Type,
    is_toplevel: bool,
    is_external: bool,
}
