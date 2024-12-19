use std::{any::TypeId, collections::HashMap, sync::LazyLock};

use crate::{
    adt_def::{AdtDef, AdtRef, ConstructorDef, FieldDef},
    adt_id::{AdtId},
    raw_adt_def::{RawAdtDef, RawConstructorDef},
};
// TypeId -> AdtId

static ADT_ID_FROM_TYPE_ID: LazyLock<HashMap<TypeId, AdtId>> =
    LazyLock::new(_mk_adt_id_from_type_id);

fn _mk_adt_id_from_type_id() -> HashMap<TypeId, AdtId> {
    let mut map = HashMap::new();

    for adt_reg_wrapper in inventory::iter::<AdtRegEntryWrapper> {
        let adt_reg = (adt_reg_wrapper.0)();

        let type_id = adt_reg.type_id;
        let adt_id = AdtId::from_raw_adt_def(&adt_reg.raw_adt_def);

        if map.insert(type_id, adt_id).is_some() {
            panic!(
                "Duplicate TypeId in ADT registration. \n\n{:?} \n\n{:?} \n\n{:?}",
                type_id, adt_id, map
            );
        }
    }

    map
}

pub fn adt_id_from_type_id(type_id: TypeId) -> Option<&'static AdtId> {
    (*ADT_ID_FROM_TYPE_ID).get(&type_id)
}

// AdtId -> TypeId

static TYPE_ID_FROM_ADT_ID: LazyLock<HashMap<AdtId, TypeId>> =
    LazyLock::new(_mk_type_id_from_adt_id);

fn _mk_type_id_from_adt_id() -> HashMap<AdtId, TypeId> {
    let mut map = HashMap::new();

    for adt_reg_wrapper in inventory::iter::<AdtRegEntryWrapper> {
        let adt_reg = (adt_reg_wrapper.0)();

        let type_id = adt_reg.type_id;
        let adt_id = AdtId::from_raw_adt_def(&adt_reg.raw_adt_def);

        if map.insert(adt_id, type_id).is_some() {
            panic!(
                "Duplicate AdtId in ADT registration. \n\n{:?} \n\n{:?} \n\n{:?}",
                adt_id, type_id, map
            );
        }
    }

    map
}

pub fn type_id_from_adt_id(adt_id: AdtId) -> Option<&'static TypeId> {
    (*TYPE_ID_FROM_ADT_ID).get(&adt_id)
}

// RawAdtDef -> AdtId

// ? Is this necessary?
// static ADT_ID_FROM_RAW_ADT_DEF: LazyLock<HashMap<RawAdtDef, AdtId>> =
//     LazyLock::new(_mk_adt_id_from_raw_adt_def);

fn _mk_adt_id_from_raw_adt_def() -> HashMap<RawAdtDef, AdtId> {
    let mut map = HashMap::new();

    for adt_reg_wrapper in inventory::iter::<AdtRegEntryWrapper> {
        let adt_reg = (adt_reg_wrapper.0)();

        let adt_id = AdtId::from_raw_adt_def(&adt_reg.raw_adt_def);

        if map.insert(adt_reg.raw_adt_def.clone(), adt_id).is_some() {
            panic!(
                "Duplicate RawAdtDef in ADT registration. \n\n{:?} \n\n{:?} \n\n{:?}",
                adt_reg.raw_adt_def, adt_id, map
            );
        }
    }

    map
}

// AdtId -> AdtDef

static ADT_DEF_FROM_ADT_ID: LazyLock<HashMap<AdtId, AdtDef>> =
    LazyLock::new(_mk_adt_def_from_adt_id);

fn _mk_adt_def_from_adt_id() -> HashMap<AdtId, AdtDef> {
    let adt_id_from_raw_adt_def_map = _mk_adt_id_from_raw_adt_def();

    let adt_id_from_raw_adt_def = |raw_adt_def: &RawAdtDef| -> Option<&AdtId> {
        adt_id_from_raw_adt_def_map.get(raw_adt_def)
    };

    let mut map = HashMap::new();

    for adt_reg_wrapper in inventory::iter::<AdtRegEntryWrapper> {
        let adt_reg = (adt_reg_wrapper.0)();

        let raw_adt_def = adt_reg.raw_adt_def;
        let adt_id = AdtId::from_raw_adt_def(&raw_adt_def);

        let adt_def = match raw_adt_def {
            RawAdtDef::Primitive(primitive_type) => AdtDef::Primitive(primitive_type),
            RawAdtDef::TupleStruct(raw_adt_defs) => {
                let adt_refs = raw_adt_defs
                    .iter()
                    .map(|raw_adt_def| {
                        if let Some(adt_id) = adt_id_from_raw_adt_def(raw_adt_def) {
                            AdtRef::Composite(adt_id.clone())
                        } else {
                            panic!(
                                "Failed to find AdtId for RawAdtDef.\n\n{:?}\n\n{:?}\n\n{:?}",
                                raw_adt_def, adt_id_from_raw_adt_def_map, map
                            );
                        }
                    })
                    .collect();

                AdtDef::TupleStruct(adt_refs)
            }
            RawAdtDef::RecordStruct(raw_field_defs) => {
                let field_defs = raw_field_defs
                    .iter()
                    .map(|raw_field_def| {
                        let name = raw_field_def.name.to_string();
                        let ty = if let Some(adt_id) = adt_id_from_raw_adt_def(&raw_field_def.ty) {
                            AdtRef::Composite(adt_id.clone())
                        } else {
                            panic!(
                                "Failed to find AdtId for RawAdtDef.\n\n{:?}\n\n{:?}\n\n{:?}",
                                raw_field_def.ty, adt_id_from_raw_adt_def_map, map
                            );
                        };

                        FieldDef { name, ty }
                    })
                    .collect();

                AdtDef::RecordStruct(field_defs)
            }

            RawAdtDef::Enum(raw_constructor_defs) => {
                let constructor_defs = raw_constructor_defs
                    .iter()
                    .map(|raw_constructor_def| {
                        match raw_constructor_def {
                            RawConstructorDef::TupleLike(raw_adt_defs) => {
                                let adt_refs = raw_adt_defs
                                    .iter()
                                    .map(|raw_adt_def| {
                                        if let Some(adt_id) = adt_id_from_raw_adt_def(raw_adt_def) {
                                            AdtRef::Composite(adt_id.clone())
                                        } else {
                                            panic!(
                                                "Failed to find AdtId for RawAdtDef.\n\n{:?}\n\n{:?}\n\n{:?}",
                                                raw_adt_def, adt_id_from_raw_adt_def_map, map
                                            );
                                        }
                                    })
                                    .collect();

                                ConstructorDef::TupleLike(adt_refs)
                            }

                            RawConstructorDef::RecordLike(raw_field_defs) => {
                                let field_defs = raw_field_defs
                                    .iter()
                                    .map(|raw_field_def| {
                                        let name = raw_field_def.name.to_string();
                                        let ty = if let Some(adt_id) =
                                            adt_id_from_raw_adt_def(&raw_field_def.ty)
                                        {
                                            AdtRef::Composite(adt_id.clone())
                                        } else {
                                            panic!(
                                                "Failed to find AdtId for RawAdtDef.\n\n{:?}\n\n{:?}\n\n{:?}",
                                                raw_field_def.ty, adt_id_from_raw_adt_def_map, map
                                            );
                                        };

                                        FieldDef { name, ty }
                                    })
                                    .collect();

                                ConstructorDef::RecordLike(field_defs)
                            }

                        }
                    })
                    .collect();

                AdtDef::Enum(constructor_defs)
            }
        };

        if map.insert(adt_id, adt_def).is_some() {
            panic!("Duplicate AdtId in ADT registration. \n\n{:?}", adt_id);
        }
    }

    map
}

pub fn adt_def_from_adt_id(adt_id: AdtId) -> Option<&'static AdtDef> {
    (*ADT_DEF_FROM_ADT_ID).get(&adt_id)
}

pub struct AdtRegEntryWrapper(pub fn() -> AdtRegEntry);

inventory::collect!(AdtRegEntryWrapper);

pub struct AdtRegEntry {
    pub type_id: TypeId,
    pub raw_adt_def: RawAdtDef,
}
