//! `parallel_service_startup` — start independent services concurrently.

/// Sentinel for `parallel_service_startup`.
pub struct ParallelServiceStartup;

cast::concept! {
    name: "parallel_service_startup",
    summary: "start independent services concurrently.",
    anchors: [cast_os_stdlib::boot_init::parallel_service_startup::ParallelServiceStartup],
    tags: ["cast_os_stdlib", "boot_init"],
}
