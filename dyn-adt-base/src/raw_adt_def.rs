
use crate::primitive_type::PrimitiveType;

// Used for univeral type comparison and diffing etc.
// Raw ADT definitions are not for saving but for in-memory representation of ADT definitions.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawAdtDef {
    Primitive(PrimitiveType),
    TupleStruct(Vec<RawAdtDef>),
    RecordStruct(Vec<RawFieldDef>),
    Enum(Vec<RawConstructorDef>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawFieldDef {
    pub name: &'static str,
    pub ty: RawAdtDef,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawConstructorDef {
    TupleLike(Vec<RawAdtDef>),
    RecordLike(Vec<RawFieldDef>),
}
