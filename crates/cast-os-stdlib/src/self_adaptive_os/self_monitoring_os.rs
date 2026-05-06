//! `self_monitoring_os` — OS observes its own state.

/// Sentinel for `self_monitoring_os`.
pub struct SelfMonitoringOs;

cast::concept! {
    name: "self_monitoring_os",
    summary: "OS observes its own state.",
    anchors: [cast_os_stdlib::self_adaptive_os::self_monitoring_os::SelfMonitoringOs],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
