//! `fallback_kernel` — boot alternate known-good kernel.

/// Sentinel for `fallback_kernel`.
pub struct FallbackKernel;

cast::concept! {
    name: "fallback_kernel",
    summary: "boot alternate known-good kernel.",
    anchors: [cast_os_stdlib::fault_recovery::fallback_kernel::FallbackKernel],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
