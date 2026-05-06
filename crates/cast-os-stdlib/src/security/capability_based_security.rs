//! `capability_based_security` — authority by unforgeable token/handle.

/// Sentinel for `capability_based_security`.
pub struct CapabilityBasedSecurity;

cast::concept! {
    name: "capability_based_security",
    summary: "authority by unforgeable token/handle.",
    anchors: [cast_os_stdlib::security::capability_based_security::CapabilityBasedSecurity],
    tags: ["cast_os_stdlib", "security"],
}
