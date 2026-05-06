//! `oops_recoverable_kernel_fault` — Linux non-fatal kernel fault.

/// Sentinel for `oops_recoverable_kernel_fault`.
pub struct OopsRecoverableKernelFault;

cast::concept! {
    name: "oops_recoverable_kernel_fault",
    summary: "Linux non-fatal kernel fault.",
    anchors: [cast_os_stdlib::fault_recovery::oops_recoverable_kernel_fault::OopsRecoverableKernelFault],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
