//! `replay_protected_pairing_flow` — secure enrollment of a new device, client, or trusted participant.

/// Sentinel for `replay_protected_pairing_flow`.
pub struct ReplayProtectedPairingFlow;

cast::concept! {
    name: "replay_protected_pairing_flow",
    summary: "A secure process for enrolling a new device, client, \
              or trusted participant. Composes time_bound_credential, \
              replay_guard, nonce_challenge, principal_authentication, \
              trust_on_first_use, certificate_authority_trust_root, \
              capability_token, and hardware_backed_key. Used for \
              phone-to-cluster enrollment, IoT onboarding, admin \
              device registration, client certificate issuance, and \
              QR-code pairing flows.",
    anchors: [cast_stdlib::patterns::replay_protected_pairing_flow::ReplayProtectedPairingFlow],
    tags: ["cast_stdlib", "patterns"],
}
