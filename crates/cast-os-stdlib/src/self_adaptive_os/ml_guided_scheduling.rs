//! `ml_guided_scheduling` — scheduler informed by learned model.

/// Sentinel for `ml_guided_scheduling`.
pub struct MlGuidedScheduling;

cast::concept! {
    name: "ml_guided_scheduling",
    summary: "scheduler informed by learned model.",
    anchors: [cast_os_stdlib::self_adaptive_os::ml_guided_scheduling::MlGuidedScheduling],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
