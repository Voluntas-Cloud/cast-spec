//! `content_hash_id` — identity derived from content.

/// Sentinel for `content_hash_id`.
pub struct ContentHashId;

cast::concept! {
    name: "content_hash_id",
    summary: "Identity derived from content. Computable without an \
              authority; equality of identifiers implies equality of \
              content. Brittle to encoding changes.",
    anchors: [cast_stdlib::identity::content_hash_id::ContentHashId],
    tags: ["cast_stdlib", "identity"],
}
