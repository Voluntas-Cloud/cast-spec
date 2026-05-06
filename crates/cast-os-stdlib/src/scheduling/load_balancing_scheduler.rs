//! `load_balancing_scheduler` — scheduler spreads work across CPUs.

/// Sentinel for `load_balancing_scheduler`.
pub struct LoadBalancingScheduler;

cast::concept! {
    name: "load_balancing_scheduler",
    summary: "scheduler spreads work across CPUs.",
    anchors: [cast_os_stdlib::scheduling::load_balancing_scheduler::LoadBalancingScheduler],
    tags: ["cast_os_stdlib", "scheduling"],
}
