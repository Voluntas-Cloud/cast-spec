//! `content_addressed_cache` — values keyed by content hash.

/// Sentinel for `content_addressed_cache`.
pub struct ContentAddressedCache;

cast::concept! {
    name: "content_addressed_cache",
    summary: "Values stored by hash of their content or input. Cache \
              invalidation is automatic — input change yields hash \
              change yields miss. Stable hash function across \
              versions is load-bearing.",
    anchors: [cast_stdlib::storage::content_addressed_cache::ContentAddressedCache],
    tags: ["cast_stdlib", "storage"],
}
