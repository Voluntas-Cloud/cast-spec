//! `profiling` — measure resource use inside code.

/// Sentinel for `profiling`.
pub struct Profiling;

cast::concept! {
    name: "profiling",
    summary: "Measure resource use inside code. CPU, allocations, \
              I/O wait — broken down by call site so optimization \
              targets the actual bottleneck.",
    anchors: [cast_stdlib::observability::profiling::Profiling],
    tags: ["cast_stdlib", "observability"],
}
