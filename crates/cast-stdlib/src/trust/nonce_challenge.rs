//! `nonce_challenge` — one-time random value proves freshness.

/// Sentinel for `nonce_challenge`.
pub struct NonceChallenge;

cast::concept! {
    name: "nonce_challenge",
    summary: "Random value used once to prove freshness. Server \
              issues nonce; signed response includes it; same nonce \
              is rejected on second use.",
    anchors: [cast_stdlib::trust::nonce_challenge::NonceChallenge],
    tags: ["cast_stdlib", "trust"],
}
