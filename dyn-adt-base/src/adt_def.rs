use serde::{Deserialize, Serialize};

use crate::{adt_id::AdtId, primitive_type::PrimitiveType};

// Ser, De supporting savable ADT definitions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdtDef {
    Primitive(PrimitiveType),
    TupleStruct(Vec<AdtRef>),
    RecordStruct(Vec<FieldDef>),
    Enum(Vec<ConstructorDef>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub ty: AdtRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructorDef {
    TupleLike(Vec<AdtRef>),
    RecordLike(Vec<FieldDef>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdtRef {
    Primitive(PrimitiveType),
    Composite(AdtId),
}
