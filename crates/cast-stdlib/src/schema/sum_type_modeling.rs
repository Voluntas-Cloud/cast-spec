//! `sum_type_modeling` — variants enumerated explicitly.

/// Sentinel for `sum_type_modeling`.
pub struct SumTypeModeling;

cast::concept! {
    name: "sum_type_modeling",
    summary: "Model variants explicitly. The set of possible shapes is \
              enumerated; readers handle each variant by name rather \
              than guessing from sentinel field values.",
    anchors: [cast_stdlib::schema::sum_type_modeling::SumTypeModeling],
    tags: ["cast_stdlib", "schema"],
}
