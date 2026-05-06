//! `kernel_mode_driver` — driver runs in kernel privilege.

/// Sentinel for `kernel_mode_driver`.
pub struct KernelModeDriver;

cast::concept! {
    name: "kernel_mode_driver",
    summary: "driver runs in kernel privilege.",
    anchors: [cast_os_stdlib::driver_model::kernel_mode_driver::KernelModeDriver],
    tags: ["cast_os_stdlib", "driver_model"],
}
