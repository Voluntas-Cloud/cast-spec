//! `process_isolation` — processes separated by address spaces and permissions.

/// Sentinel for `process_isolation`.
pub struct ProcessIsolation;

cast::concept! {
    name: "process_isolation",
    summary: "processes separated by address spaces and permissions.",
    anchors: [cast_os_stdlib::isolation::process_isolation::ProcessIsolation],
    tags: ["cast_os_stdlib", "isolation"],
}
