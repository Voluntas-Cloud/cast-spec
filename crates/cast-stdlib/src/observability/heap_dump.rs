//! `heap_dump` — inspect memory state.

/// Sentinel for `heap_dump`.
pub struct HeapDump;

cast::concept! {
    name: "heap_dump",
    summary: "Inspect memory state. Captures the live heap so leaks \
              and bloat can be analyzed offline; the only practical \
              way to chase certain memory pathologies.",
    anchors: [cast_stdlib::observability::heap_dump::HeapDump],
    tags: ["cast_stdlib", "observability"],
}
