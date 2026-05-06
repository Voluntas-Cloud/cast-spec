//! `indexing_strategy` — optimize lookup paths.

/// Sentinel for `indexing_strategy`.
pub struct IndexingStrategy;

cast::concept! {
    name: "indexing_strategy",
    summary: "Optimize lookup paths. Each index trades write cost for \
              read cost; the right indexes are determined by the \
              actual query mix, not by guessing.",
    anchors: [cast_stdlib::performance::indexing_strategy::IndexingStrategy],
    tags: ["cast_stdlib", "performance"],
}
