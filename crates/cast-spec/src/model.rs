//! Cross-module types. Kept narrow — most semantic types come from
//! `ra_ap_hir` directly rather than being wrapped here.

use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn new(file: PathBuf, line: usize, col: usize) -> Self {
        Self { file, line, col }
    }
}

cast::concept! {
    name: "source_location",
    summary: "File + line + column. Carried on every parsed cast \
              annotation, every diagnostic, every report entry.",
    anchors: [
        crate::model::Location,
    ],
    tags: ["cast_spec_model"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "source_location",
    why: "Three required fields, all simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "source_location",
    why: "Cloneable, hashable, structurally compared, serialisable.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "source_location",
    why: lazy,
}
