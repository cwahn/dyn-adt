use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{adt_trait::AdtTrait, raw_adt_def::RawAdtDef};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdtId(u64);

impl AdtId {
    pub fn from_raw_adt_def(raw_adt_def: &RawAdtDef) -> AdtId {
        let mut hasher = DefaultHasher::new();
        raw_adt_def.hash(&mut hasher);
        AdtId(hasher.finish())
    }

    pub fn of<A: AdtTrait>() -> AdtId {
        // ! Should take value from generate map TypeId -> AdtId
        // This requires all ADTs to be registered in a map
        let raw_adt_def = A::raw_adt_def();
        AdtId::from_raw_adt_def(&raw_adt_def)
    }
}
