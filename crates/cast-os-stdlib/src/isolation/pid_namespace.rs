//! `pid_namespace` — isolate process ID view.

/// Sentinel for `pid_namespace`.
pub struct PidNamespace;

cast::concept! {
    name: "pid_namespace",
    summary: "isolate process ID view.",
    anchors: [cast_os_stdlib::isolation::pid_namespace::PidNamespace],
    tags: ["cast_os_stdlib", "isolation"],
}
