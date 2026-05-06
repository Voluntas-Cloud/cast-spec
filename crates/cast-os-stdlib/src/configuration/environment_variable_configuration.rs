//! `environment_variable_configuration` — process environment as config.

/// Sentinel for `environment_variable_configuration`.
pub struct EnvironmentVariableConfiguration;

cast::concept! {
    name: "environment_variable_configuration",
    summary: "process environment as config.",
    anchors: [cast_os_stdlib::configuration::environment_variable_configuration::EnvironmentVariableConfiguration],
    tags: ["cast_os_stdlib", "configuration"],
}
