//! `thermal_self_adaptation` — performance adjusted to thermal state.

/// Sentinel for `thermal_self_adaptation`.
pub struct ThermalSelfAdaptation;

cast::concept! {
    name: "thermal_self_adaptation",
    summary: "performance adjusted to thermal state.",
    anchors: [cast_os_stdlib::self_adaptive_os::thermal_self_adaptation::ThermalSelfAdaptation],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
