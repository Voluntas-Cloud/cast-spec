//! `procfs_configuration_view` — process/kernel info pseudo-filesystem.

/// Sentinel for `procfs_configuration_view`.
pub struct ProcfsConfigurationView;

cast::concept! {
    name: "procfs_configuration_view",
    summary: "process/kernel info pseudo-filesystem.",
    anchors: [cast_os_stdlib::configuration::procfs_configuration_view::ProcfsConfigurationView],
    tags: ["cast_os_stdlib", "configuration"],
}
