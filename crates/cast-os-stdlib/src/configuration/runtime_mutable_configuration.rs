//! `runtime_mutable_configuration` — change config while running.

/// Sentinel for `runtime_mutable_configuration`.
pub struct RuntimeMutableConfiguration;

cast::concept! {
    name: "runtime_mutable_configuration",
    summary: "change config while running.",
    anchors: [cast_os_stdlib::configuration::runtime_mutable_configuration::RuntimeMutableConfiguration],
    tags: ["cast_os_stdlib", "configuration"],
}
