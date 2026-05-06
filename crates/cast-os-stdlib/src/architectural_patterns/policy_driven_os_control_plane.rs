//! `policy_driven_os_control_plane` — system behavior governed by policy layer.

/// Sentinel for `policy_driven_os_control_plane`.
pub struct PolicyDrivenOsControlPlane;

cast::concept! {
    name: "policy_driven_os_control_plane",
    summary: "system behavior governed by policy layer.",
    anchors: [cast_os_stdlib::architectural_patterns::policy_driven_os_control_plane::PolicyDrivenOsControlPlane],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
