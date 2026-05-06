//! `energy_efficiency` — conserves power.

/// Sentinel for `energy_efficiency`.
pub struct EnergyEfficiency;

cast::concept! {
    name: "energy_efficiency",
    summary: "conserves power.",
    anchors: [cast_os_stdlib::design_qualities::energy_efficiency::EnergyEfficiency],
    tags: ["cast_os_stdlib", "design_qualities"],
}
