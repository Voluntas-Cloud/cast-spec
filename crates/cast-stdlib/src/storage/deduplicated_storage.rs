//! `deduplicated_storage` — store identical content once.

/// Sentinel for `deduplicated_storage`.
pub struct DeduplicatedStorage;

cast::concept! {
    name: "deduplicated_storage",
    summary: "Avoid storing identical content multiple times. Identity \
              by content hash is the natural mechanism; uniqueness \
              is per-byte, not per-name.",
    anchors: [cast_stdlib::storage::deduplicated_storage::DeduplicatedStorage],
    tags: ["cast_stdlib", "storage"],
}
