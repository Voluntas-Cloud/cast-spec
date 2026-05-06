//! `early_kernel_init` — architecture and memory setup.

/// Sentinel for `early_kernel_init`.
pub struct EarlyKernelInit;

cast::concept! {
    name: "early_kernel_init",
    summary: "architecture and memory setup.",
    anchors: [cast_os_stdlib::boot_init::early_kernel_init::EarlyKernelInit],
    tags: ["cast_os_stdlib", "boot_init"],
}
