//! `configuration_drift_detection` — detect unexpected config changes.

/// Sentinel for `configuration_drift_detection`.
pub struct ConfigurationDriftDetection;

cast::concept! {
    name: "configuration_drift_detection",
    summary: "Detect unexpected config changes. Compares running \
              config against the declared/desired config; flags any \
              differences for reconciliation or alerting.",
    anchors: [cast_stdlib::config::configuration_drift_detection::ConfigurationDriftDetection],
    tags: ["cast_stdlib", "config"],
}
