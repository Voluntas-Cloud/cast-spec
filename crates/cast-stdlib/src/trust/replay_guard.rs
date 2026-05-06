//! `replay_guard` — prevent reuse of a proof/request.

/// Sentinel for `replay_guard`.
pub struct ReplayGuard;

cast::concept! {
    name: "replay_guard",
    summary: "Prevents reuse of a proof/request. Implementations: \
              nonce + memory of seen nonces, monotonic sequence with \
              high-water mark, or bound-to-state.",
    anchors: [cast_stdlib::trust::replay_guard::ReplayGuard],
    tags: ["cast_stdlib", "trust"],
}
