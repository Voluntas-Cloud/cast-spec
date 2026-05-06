//! `mach_port_ipc` — capability-like IPC endpoint model.

/// Sentinel for `mach_port_ipc`.
pub struct MachPortIpc;

cast::concept! {
    name: "mach_port_ipc",
    summary: "capability-like IPC endpoint model.",
    anchors: [cast_os_stdlib::ipc::mach_port_ipc::MachPortIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
