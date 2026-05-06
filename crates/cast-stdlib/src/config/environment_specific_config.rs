//! `environment_specific_config` — dev/staging/prod differences isolated.

/// Sentinel for `environment_specific_config`.
pub struct EnvironmentSpecificConfig;

cast::concept! {
    name: "environment_specific_config",
    summary: "Dev/staging/prod differences isolated. The values that \
              vary across environments are named explicitly; \
              everything else is identical across environments.",
    anchors: [cast_stdlib::config::environment_specific_config::EnvironmentSpecificConfig],
    tags: ["cast_stdlib", "config"],
}
