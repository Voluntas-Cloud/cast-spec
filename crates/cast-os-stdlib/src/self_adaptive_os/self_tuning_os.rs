//! `self_tuning_os` — OS adjusts parameters automatically.

/// Sentinel for `self_tuning_os`.
pub struct SelfTuningOs;

cast::concept! {
    name: "self_tuning_os",
    summary: "OS adjusts parameters automatically.",
    anchors: [cast_os_stdlib::self_adaptive_os::self_tuning_os::SelfTuningOs],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
