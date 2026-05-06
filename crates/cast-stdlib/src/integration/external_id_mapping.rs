//! `external_id_mapping` — associate internal and external IDs.

/// Sentinel for `external_id_mapping`.
pub struct ExternalIdMapping;

cast::concept! {
    name: "external_id_mapping",
    summary: "Associate internal IDs with external ones, and store \
              the mapping. Without it, every integration has to \
              re-derive identity from heuristics, and the heuristics \
              eventually disagree about which records are the same.",
    anchors: [cast_stdlib::integration::external_id_mapping::ExternalIdMapping],
    tags: ["cast_stdlib", "integration"],
}
