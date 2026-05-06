//! Security engineering patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod attack_surface_reduction;
pub mod auditability;
pub mod credential_revocation;
pub mod data_minimization;
pub mod defense_in_depth;
pub mod envelope_encryption;
pub mod input_validation;
pub mod key_derivation;
pub mod output_encoding;
pub mod privilege_separation;
pub mod sandboxing;
pub mod secret_injection;
pub mod secret_rotation;
pub mod secure_bootstrap;
pub mod secure_defaults;
pub mod secure_deletion;
pub mod supply_chain_integrity;
pub mod tamper_evidence;
pub mod threat_model;

cast::concept! {
    name: "security",
    summary: "Umbrella for the security stdlib category. Security \
              engineering patterns.",
    anchors: [
        crate::security::attack_surface_reduction,
        crate::security::auditability,
        crate::security::credential_revocation,
        crate::security::data_minimization,
        crate::security::defense_in_depth,
        crate::security::envelope_encryption,
        crate::security::input_validation,
        crate::security::key_derivation,
        crate::security::output_encoding,
        crate::security::privilege_separation,
        crate::security::sandboxing,
        crate::security::secret_injection,
        crate::security::secret_rotation,
        crate::security::secure_bootstrap,
        crate::security::secure_defaults,
        crate::security::secure_deletion,
        crate::security::supply_chain_integrity,
        crate::security::tamper_evidence,
        crate::security::threat_model,
    ],
    tags: ["cast_stdlib", "security"],
}

/// Sentinel for the security stdlib group.
pub struct SecurityGroup;

cast::rule! {
    rule: "Bootstrap is the most dangerous part.",
    why:  "If first trust is wrong, everything after it is just a \
           beautifully encrypted mistake. Audit how the first key, the \
           first cert, and the first identity arrive — that is your \
           real threat model.",
    governs: [cast_stdlib::security::SecurityGroup],
    tags: ["cast_stdlib", "security"],
}
