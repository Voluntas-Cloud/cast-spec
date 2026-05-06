//! `exokernel_architecture` — kernel securely multiplexes hardware while apps build abstractions.

/// Sentinel for `exokernel_architecture`.
pub struct ExokernelArchitecture;

cast::concept! {
    name: "exokernel_architecture",
    summary: "kernel securely multiplexes hardware while apps build \
               abstractions.",
    anchors: [cast_os_stdlib::kernel_architecture::exokernel_architecture::ExokernelArchitecture],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
