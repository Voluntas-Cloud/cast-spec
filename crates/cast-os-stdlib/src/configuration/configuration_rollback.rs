//! `configuration_rollback` — revert config to known-good state.

/// Sentinel for `configuration_rollback`.
pub struct ConfigurationRollback;

cast::concept! {
    name: "configuration_rollback",
    summary: "revert config to known-good state.",
    anchors: [cast_os_stdlib::configuration::configuration_rollback::ConfigurationRollback],
    tags: ["cast_os_stdlib", "configuration"],
}
