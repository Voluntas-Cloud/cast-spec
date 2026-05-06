//! `race_condition_kernel` — timing-dependent bug.

/// Sentinel for `race_condition_kernel`.
pub struct RaceConditionKernel;

cast::concept! {
    name: "race_condition_kernel",
    summary: "timing-dependent bug.",
    anchors: [cast_os_stdlib::failure_modes::race_condition_kernel::RaceConditionKernel],
    tags: ["cast_os_stdlib", "failure_modes"],
}
