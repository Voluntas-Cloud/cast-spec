//! `thermal_throttling` — reduce performance to avoid overheating.

/// Sentinel for `thermal_throttling`.
pub struct ThermalThrottling;

cast::concept! {
    name: "thermal_throttling",
    summary: "reduce performance to avoid overheating.",
    anchors: [cast_os_stdlib::power_thermal::thermal_throttling::ThermalThrottling],
    tags: ["cast_os_stdlib", "power_thermal"],
}
