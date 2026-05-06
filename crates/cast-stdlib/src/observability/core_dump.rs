//! `core_dump` — inspect crashed process state.

/// Sentinel for `core_dump`.
pub struct CoreDump;

cast::concept! {
    name: "core_dump",
    summary: "Inspect crashed process. Captured at termination so a \
              debugger can reconstruct what the program was doing in \
              its final moments.",
    anchors: [cast_stdlib::observability::core_dump::CoreDump],
    tags: ["cast_stdlib", "observability"],
}
