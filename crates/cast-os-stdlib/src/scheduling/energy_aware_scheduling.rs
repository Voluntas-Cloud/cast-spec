//! `energy_aware_scheduling` — schedule with power efficiency in mind.

/// Sentinel for `energy_aware_scheduling`.
pub struct EnergyAwareScheduling;

cast::concept! {
    name: "energy_aware_scheduling",
    summary: "schedule with power efficiency in mind.",
    anchors: [cast_os_stdlib::scheduling::energy_aware_scheduling::EnergyAwareScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
