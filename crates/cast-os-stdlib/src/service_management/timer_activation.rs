//! `timer_activation` — start service on schedule.

/// Sentinel for `timer_activation`.
pub struct TimerActivation;

cast::concept! {
    name: "timer_activation",
    summary: "start service on schedule.",
    anchors: [cast_os_stdlib::service_management::timer_activation::TimerActivation],
    tags: ["cast_os_stdlib", "service_management"],
}
