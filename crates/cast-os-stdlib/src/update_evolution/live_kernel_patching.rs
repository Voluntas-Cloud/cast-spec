//! `live_kernel_patching` — patch kernel without reboot.

/// Sentinel for `live_kernel_patching`.
pub struct LiveKernelPatching;

cast::concept! {
    name: "live_kernel_patching",
    summary: "patch kernel without reboot.",
    anchors: [cast_os_stdlib::update_evolution::live_kernel_patching::LiveKernelPatching],
    tags: ["cast_os_stdlib", "update_evolution"],
}
