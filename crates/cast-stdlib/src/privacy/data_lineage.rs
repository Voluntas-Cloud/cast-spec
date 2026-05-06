//! `data_lineage` — trace origin and transformations.

/// Sentinel for `data_lineage`.
pub struct DataLineage;

cast::concept! {
    name: "data_lineage",
    summary: "Trace origin and transformations. Every derived dataset \
              can be walked back to its inputs and the code that \
              produced it; without this, \"can we still use that \
              column?\" becomes a research project.",
    anchors: [cast_stdlib::privacy::data_lineage::DataLineage],
    tags: ["cast_stdlib", "privacy"],
}
