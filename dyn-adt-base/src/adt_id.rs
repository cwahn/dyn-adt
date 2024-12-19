use const_fnv1a_hash::fnv1a_hash_64;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use crate::raw_adt_def::RawAdtDef;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdtId(u64);

impl AdtId {
    //
    pub const fn from_raw_adt_def(raw_adt_def: &RawAdtDef) -> AdtId {
        // let mut hasher = std::collections::hash_map::DefaultHasher::new();
        // raw_adt_def.hash(&mut hasher);
        // AdtId(hasher.finish())

        let hash = fnv1a_hash_64();
        AdtId(hash)
    }
}