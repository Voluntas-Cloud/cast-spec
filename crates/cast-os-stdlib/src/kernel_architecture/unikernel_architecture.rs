//! `unikernel_architecture` — application and OS library linked into one specialized image.

/// Sentinel for `unikernel_architecture`.
pub struct UnikernelArchitecture;

cast::concept! {
    name: "unikernel_architecture",
    summary: "application and OS library linked into one specialized \
               image.",
    anchors: [cast_os_stdlib::kernel_architecture::unikernel_architecture::UnikernelArchitecture],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
