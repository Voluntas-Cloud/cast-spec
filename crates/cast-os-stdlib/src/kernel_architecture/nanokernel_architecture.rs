//! `nanokernel_architecture` — extremely small hardware abstraction core.

/// Sentinel for `nanokernel_architecture`.
pub struct NanokernelArchitecture;

cast::concept! {
    name: "nanokernel_architecture",
    summary: "extremely small hardware abstraction core.",
    anchors: [cast_os_stdlib::kernel_architecture::nanokernel_architecture::NanokernelArchitecture],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
