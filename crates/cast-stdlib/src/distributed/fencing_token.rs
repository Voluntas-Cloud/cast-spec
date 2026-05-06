//! `fencing_token` — prevents old leader from acting after lease loss.

/// Sentinel for `fencing_token`.
pub struct FencingToken;

cast::concept! {
    name: "fencing_token",
    summary: "Prevents old leader from acting after lease loss. Each \
              new leader gets a higher token; downstream services \
              reject writes from a token lower than the highest seen.",
    anchors: [cast_stdlib::distributed::fencing_token::FencingToken],
    tags: ["cast_stdlib", "distributed"],
}
