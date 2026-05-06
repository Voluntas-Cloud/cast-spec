//! `object_based_kernel` — kernel resources represented as objects.

/// Sentinel for `object_based_kernel`.
pub struct ObjectBasedKernel;

cast::concept! {
    name: "object_based_kernel",
    summary: "kernel resources represented as objects.",
    anchors: [cast_os_stdlib::kernel_architecture::object_based_kernel::ObjectBasedKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
