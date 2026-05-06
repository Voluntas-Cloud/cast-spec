//! `rate_monotonic_scheduling` — static priority by task period.

/// Sentinel for `rate_monotonic_scheduling`.
pub struct RateMonotonicScheduling;

cast::concept! {
    name: "rate_monotonic_scheduling",
    summary: "static priority by task period.",
    anchors: [cast_os_stdlib::scheduling::rate_monotonic_scheduling::RateMonotonicScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
