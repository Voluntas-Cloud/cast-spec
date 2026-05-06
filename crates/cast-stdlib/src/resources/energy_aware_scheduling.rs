//! `energy_aware_scheduling` — optimize for power use.

/// Sentinel for `energy_aware_scheduling`.
pub struct EnergyAwareScheduling;

cast::concept! {
    name: "energy_aware_scheduling",
    summary: "Optimize for power use. Workloads are placed and paced \
              with carbon, electricity cost, or thermal headroom in \
              mind; capacity is treated as a function of when, not just \
              where.",
    anchors: [cast_stdlib::resources::energy_aware_scheduling::EnergyAwareScheduling],
    tags: ["cast_stdlib", "resources"],
}
