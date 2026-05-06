//! `security_policy_bypass` — enforcement gap.

/// Sentinel for `security_policy_bypass`.
pub struct SecurityPolicyBypass;

cast::concept! {
    name: "security_policy_bypass",
    summary: "enforcement gap.",
    anchors: [cast_os_stdlib::failure_modes::security_policy_bypass::SecurityPolicyBypass],
    tags: ["cast_os_stdlib", "failure_modes"],
}
