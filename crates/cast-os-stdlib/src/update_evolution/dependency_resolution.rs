//! `dependency_resolution` — package dependency solving.

/// Sentinel for `dependency_resolution`.
pub struct DependencyResolution;

cast::concept! {
    name: "dependency_resolution",
    summary: "package dependency solving.",
    anchors: [cast_os_stdlib::update_evolution::dependency_resolution::DependencyResolution],
    tags: ["cast_os_stdlib", "update_evolution"],
}
