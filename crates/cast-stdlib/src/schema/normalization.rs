//! `normalization` — reduce duplication and update anomalies.

/// Sentinel for `normalization`.
pub struct Normalization;

cast::concept! {
    name: "normalization",
    summary: "Reduce duplication and update anomalies. Each fact lives \
              in one place; updates touch one row.",
    anchors: [cast_stdlib::schema::normalization::Normalization],
    tags: ["cast_stdlib", "schema"],
}
