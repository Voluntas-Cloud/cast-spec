//! `microvm_architecture` — minimal VM for fast isolated workloads.

/// Sentinel for `microvm_architecture`.
pub struct MicrovmArchitecture;

cast::concept! {
    name: "microvm_architecture",
    summary: "minimal VM for fast isolated workloads.",
    anchors: [cast_os_stdlib::virtualization::microvm_architecture::MicrovmArchitecture],
    tags: ["cast_os_stdlib", "virtualization"],
}
