//! `configuration_drift_detection` — detect divergence from desired config.

/// Sentinel for `configuration_drift_detection`.
pub struct ConfigurationDriftDetection;

cast::concept! {
    name: "configuration_drift_detection",
    summary: "detect divergence from desired config.",
    anchors: [cast_os_stdlib::configuration::configuration_drift_detection::ConfigurationDriftDetection],
    tags: ["cast_os_stdlib", "configuration"],
}
