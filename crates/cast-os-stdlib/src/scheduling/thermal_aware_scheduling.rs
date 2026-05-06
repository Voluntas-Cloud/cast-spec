//! `thermal_aware_scheduling` — adjust scheduling due to heat.

/// Sentinel for `thermal_aware_scheduling`.
pub struct ThermalAwareScheduling;

cast::concept! {
    name: "thermal_aware_scheduling",
    summary: "adjust scheduling due to heat.",
    anchors: [cast_os_stdlib::scheduling::thermal_aware_scheduling::ThermalAwareScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
