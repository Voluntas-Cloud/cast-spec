//! `hypervisor_host_os` — OS/control plane around VMs.

/// Sentinel for `hypervisor_host_os`.
pub struct HypervisorHostOs;

cast::concept! {
    name: "hypervisor_host_os",
    summary: "OS/control plane around VMs.",
    anchors: [cast_os_stdlib::os_use_cases::hypervisor_host_os::HypervisorHostOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
