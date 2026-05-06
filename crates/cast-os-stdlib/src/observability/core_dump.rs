//! `core_dump` — process memory dump after crash.

/// Sentinel for `core_dump`.
pub struct CoreDump;

cast::concept! {
    name: "core_dump",
    summary: "process memory dump after crash.",
    anchors: [cast_os_stdlib::observability::core_dump::CoreDump],
    tags: ["cast_os_stdlib", "observability"],
}
