//! `priority_inversion_failure` — high-priority work blocked by low-priority work.

/// Sentinel for `priority_inversion_failure`.
pub struct PriorityInversionFailure;

cast::concept! {
    name: "priority_inversion_failure",
    summary: "high-priority work blocked by low-priority work.",
    anchors: [cast_os_stdlib::failure_modes::priority_inversion_failure::PriorityInversionFailure],
    tags: ["cast_os_stdlib", "failure_modes"],
}
