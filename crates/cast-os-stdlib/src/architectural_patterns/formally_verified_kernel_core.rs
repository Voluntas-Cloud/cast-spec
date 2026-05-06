//! `formally_verified_kernel_core` — kernel core proven against formal spec.

/// Sentinel for `formally_verified_kernel_core`.
pub struct FormallyVerifiedKernelCore;

cast::concept! {
    name: "formally_verified_kernel_core",
    summary: "kernel core proven against formal spec.",
    anchors: [cast_os_stdlib::architectural_patterns::formally_verified_kernel_core::FormallyVerifiedKernelCore],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
