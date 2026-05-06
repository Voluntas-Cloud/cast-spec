//! `human_override_for_adaptation` — operator can override adaptive behavior.

/// Sentinel for `human_override_for_adaptation`.
pub struct HumanOverrideForAdaptation;

cast::concept! {
    name: "human_override_for_adaptation",
    summary: "operator can override adaptive behavior.",
    anchors: [cast_os_stdlib::self_adaptive_os::human_override_for_adaptation::HumanOverrideForAdaptation],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
