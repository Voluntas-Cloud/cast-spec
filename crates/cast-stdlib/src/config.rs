//! Configuration & policy patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod allow_by_exception;
pub mod configuration_as_data;
pub mod configuration_drift_detection;
pub mod configuration_precedence;
pub mod configuration_schema;
pub mod configuration_snapshot;
pub mod default_policy;
pub mod deny_by_default;
pub mod dynamic_configuration;
pub mod environment_specific_config;
pub mod feature_flag;
pub mod immutable_configuration;
pub mod kill_switch;
pub mod override_policy;
pub mod policy_as_code;
pub mod secret_configuration_split;

cast::concept! {
    name: "config",
    summary: "Umbrella for the config stdlib category. Configuration & \
              policy patterns.",
    anchors: [
        crate::config::allow_by_exception,
        crate::config::configuration_as_data,
        crate::config::configuration_drift_detection,
        crate::config::configuration_precedence,
        crate::config::configuration_schema,
        crate::config::configuration_snapshot,
        crate::config::default_policy,
        crate::config::deny_by_default,
        crate::config::dynamic_configuration,
        crate::config::environment_specific_config,
        crate::config::feature_flag,
        crate::config::immutable_configuration,
        crate::config::kill_switch,
        crate::config::override_policy,
        crate::config::policy_as_code,
        crate::config::secret_configuration_split,
    ],
    tags: ["cast_stdlib", "config"],
}

/// Sentinel for the config stdlib group.
pub struct ConfigGroup;

cast::rule! {
    rule: "Make configuration precedence explicit.",
    why:  "Hidden config precedence is where bugs go to become \
           folklore. If a value can come from three places, every \
           reader must be able to predict which one wins without \
           reading the loader's source.",
    governs: [cast_stdlib::config::ConfigGroup],
    tags: ["cast_stdlib", "config"],
}
