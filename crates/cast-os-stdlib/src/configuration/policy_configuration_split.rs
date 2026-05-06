//! `policy_configuration_split` — policy separated from mechanism.

/// Sentinel for `policy_configuration_split`.
pub struct PolicyConfigurationSplit;

cast::concept! {
    name: "policy_configuration_split",
    summary: "policy separated from mechanism.",
    anchors: [cast_os_stdlib::configuration::policy_configuration_split::PolicyConfigurationSplit],
    tags: ["cast_os_stdlib", "configuration"],
}
