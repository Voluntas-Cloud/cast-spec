//! `data_classification` — label data by sensitivity.

/// Sentinel for `data_classification`.
pub struct DataClassification;

cast::concept! {
    name: "data_classification",
    summary: "Label data by sensitivity. The class (public, internal, \
              confidential, regulated) drives storage, transport, and \
              access controls; classification drift is how data ends \
              up in the wrong system.",
    anchors: [cast_stdlib::privacy::data_classification::DataClassification],
    tags: ["cast_stdlib", "privacy"],
}
