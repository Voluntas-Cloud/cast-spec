//! `selinux_policy_model` — Linux MAC policy system.

/// Sentinel for `selinux_policy_model`.
pub struct SelinuxPolicyModel;

cast::concept! {
    name: "selinux_policy_model",
    summary: "Linux MAC policy system.",
    anchors: [cast_os_stdlib::security::selinux_policy_model::SelinuxPolicyModel],
    tags: ["cast_os_stdlib", "security"],
}
