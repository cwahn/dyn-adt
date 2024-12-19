use serde::Serialize;

use crate::raw_adt_def::RawAdtDef;

pub trait AdtTrait {
    fn raw_adt_def() -> RawAdtDef;
}
