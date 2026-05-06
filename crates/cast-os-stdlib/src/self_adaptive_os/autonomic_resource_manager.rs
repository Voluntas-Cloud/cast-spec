//! `autonomic_resource_manager` — automatic CPU/memory/I/O allocation.

/// Sentinel for `autonomic_resource_manager`.
pub struct AutonomicResourceManager;

cast::concept! {
    name: "autonomic_resource_manager",
    summary: "automatic CPU/memory/I/O allocation.",
    anchors: [cast_os_stdlib::self_adaptive_os::autonomic_resource_manager::AutonomicResourceManager],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
