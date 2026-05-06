//! `service_oriented_os` — OS services structured as separate cooperating services.

/// Sentinel for `service_oriented_os`.
pub struct ServiceOrientedOs;

cast::concept! {
    name: "service_oriented_os",
    summary: "OS services structured as separate cooperating services.",
    anchors: [cast_os_stdlib::kernel_architecture::service_oriented_os::ServiceOrientedOs],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
