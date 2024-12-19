use serde::{Deserialize, Serialize};

use crate::primitive_type::PrimitiveType;

// Intermediate representation of ADT values
// Field and constructor names are not included
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdtIr {
    Primitive(PrimitiveType),
    Struct(Vec<AdtIr>),
    Enum(u64, Vec<AdtIr>),
}