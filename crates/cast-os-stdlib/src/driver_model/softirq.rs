//! `softirq` — Linux deferred interrupt mechanism.

/// Sentinel for `softirq`.
pub struct Softirq;

cast::concept! {
    name: "softirq",
    summary: "Linux deferred interrupt mechanism.",
    anchors: [cast_os_stdlib::driver_model::softirq::Softirq],
    tags: ["cast_os_stdlib", "driver_model"],
}
