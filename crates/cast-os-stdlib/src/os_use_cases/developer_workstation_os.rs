//! `developer_workstation_os` — OS optimized for build/debug/tooling workflows.

/// Sentinel for `developer_workstation_os`.
pub struct DeveloperWorkstationOs;

cast::concept! {
    name: "developer_workstation_os",
    summary: "OS optimized for build/debug/tooling workflows.",
    anchors: [cast_os_stdlib::os_use_cases::developer_workstation_os::DeveloperWorkstationOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
