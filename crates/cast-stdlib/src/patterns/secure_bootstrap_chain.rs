//! `secure_bootstrap_chain` — initial setup establishes root trust, identity, keys, config, and update paths safely.

/// Sentinel for `secure_bootstrap_chain`.
pub struct SecureBootstrapChain;

cast::concept! {
    name: "secure_bootstrap_chain",
    summary: "Initial setup establishes root trust, identity, keys, \
              config, and update paths safely. Composes \
              secure_bootstrap, certificate_authority_trust_root, \
              principal_authentication, signed_artifact, \
              configuration_snapshot, trust_on_first_use, \
              key_derivation, and audit_log. Used for cluster \
              installation, device provisioning, OS/image bootstrap, \
              Voluntas first-run setup, and secure service \
              enrollment.",
    anchors: [cast_stdlib::patterns::secure_bootstrap_chain::SecureBootstrapChain],
    tags: ["cast_stdlib", "patterns"],
}
