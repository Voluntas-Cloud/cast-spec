//! `reproducible_system_build` — rebuild OS image from known inputs.

/// Sentinel for `reproducible_system_build`.
pub struct ReproducibleSystemBuild;

cast::concept! {
    name: "reproducible_system_build",
    summary: "rebuild OS image from known inputs.",
    anchors: [cast_os_stdlib::update_evolution::reproducible_system_build::ReproducibleSystemBuild],
    tags: ["cast_os_stdlib", "update_evolution"],
}
