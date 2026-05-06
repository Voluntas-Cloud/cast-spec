//! `configuration_snapshot` — record config used for a run/release.

/// Sentinel for `configuration_snapshot`.
pub struct ConfigurationSnapshot;

cast::concept! {
    name: "configuration_snapshot",
    summary: "Record config used for a run/release. Lets a debugger \
              or auditor know exactly what knobs the system had when \
              it produced a given outcome.",
    anchors: [cast_stdlib::config::configuration_snapshot::ConfigurationSnapshot],
    tags: ["cast_stdlib", "config"],
}
