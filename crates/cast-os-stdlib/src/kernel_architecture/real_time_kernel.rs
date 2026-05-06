//! `real_time_kernel` — kernel designed for deterministic timing.

/// Sentinel for `real_time_kernel`.
pub struct RealTimeKernel;

cast::concept! {
    name: "real_time_kernel",
    summary: "kernel designed for deterministic timing.",
    anchors: [cast_os_stdlib::kernel_architecture::real_time_kernel::RealTimeKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
