//! `configuration_transaction` — apply config atomically.

/// Sentinel for `configuration_transaction`.
pub struct ConfigurationTransaction;

cast::concept! {
    name: "configuration_transaction",
    summary: "apply config atomically.",
    anchors: [cast_os_stdlib::configuration::configuration_transaction::ConfigurationTransaction],
    tags: ["cast_os_stdlib", "configuration"],
}
