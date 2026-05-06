//! `static_analysis` — inspect code without running it.

/// Sentinel for `static_analysis`.
pub struct StaticAnalysis;

cast::concept! {
    name: "static_analysis",
    summary: "Inspect code without running it. Linters, type checkers, \
              taint analyzers — catch bugs at compile time that runtime \
              tests would only catch by accident.",
    anchors: [cast_stdlib::testing::static_analysis::StaticAnalysis],
    tags: ["cast_stdlib", "testing"],
}
