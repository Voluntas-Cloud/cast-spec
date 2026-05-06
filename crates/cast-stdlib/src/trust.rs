//! Trust, authentication & authorization patterns.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only the
//! group sentinel and the category rule.

pub mod audience_bound_token;
pub mod authorization_policy;
pub mod capability_token;
pub mod certificate_authority_trust_root;
pub mod delegated_authority;
pub mod hardware_backed_key;
pub mod impersonation_guard;
pub mod key_rotation;
pub mod least_privilege;
pub mod mutual_tls_identity;
pub mod nonce_challenge;
pub mod principal_authentication;
pub mod replay_guard;
pub mod revocable_credential;
pub mod scope_limited_token;
pub mod secret_scoping;
pub mod signed_request;
pub mod time_bound_credential;
pub mod trust_on_first_use;
pub mod zero_trust_boundary;

cast::concept! {
    name: "trust",
    summary: "Umbrella for the trust stdlib category. Trust, \
              authentication & authorization patterns.",
    anchors: [
        crate::trust::audience_bound_token,
        crate::trust::authorization_policy,
        crate::trust::capability_token,
        crate::trust::certificate_authority_trust_root,
        crate::trust::delegated_authority,
        crate::trust::hardware_backed_key,
        crate::trust::impersonation_guard,
        crate::trust::key_rotation,
        crate::trust::least_privilege,
        crate::trust::mutual_tls_identity,
        crate::trust::nonce_challenge,
        crate::trust::principal_authentication,
        crate::trust::replay_guard,
        crate::trust::revocable_credential,
        crate::trust::scope_limited_token,
        crate::trust::secret_scoping,
        crate::trust::signed_request,
        crate::trust::time_bound_credential,
        crate::trust::trust_on_first_use,
        crate::trust::zero_trust_boundary,
    ],
    tags: ["cast_stdlib", "trust"],
}

/// Sentinel for the trust stdlib group.
pub struct TrustGroup;

cast::rule! {
    rule: "Never treat 'logged in' as enough — keep authentication, authorization, and auditing separate.",
    why:  "Authentication says who; authorization says may do what; \
           auditing says what disaster did they cause. Conflating any \
           two of those collapses the security model into the weakest \
           one.",
    governs: [cast_stdlib::trust::TrustGroup],
    tags: ["cast_stdlib", "trust"],
}
