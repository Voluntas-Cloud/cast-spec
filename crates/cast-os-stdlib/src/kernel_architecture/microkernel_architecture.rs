//! `microkernel_architecture` — minimal kernel; services run in user space.

/// Sentinel for `microkernel_architecture`.
pub struct MicrokernelArchitecture;

cast::concept! {
    name: "microkernel_architecture",
    summary: "minimal kernel; services run in user space.",
    anchors: [cast_os_stdlib::kernel_architecture::microkernel_architecture::MicrokernelArchitecture],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
