//! `least_privilege_process_model` — process runs with minimum rights.

/// Sentinel for `least_privilege_process_model`.
pub struct LeastPrivilegeProcessModel;

cast::concept! {
    name: "least_privilege_process_model",
    summary: "process runs with minimum rights.",
    anchors: [cast_os_stdlib::security::least_privilege_process_model::LeastPrivilegeProcessModel],
    tags: ["cast_os_stdlib", "security"],
}
