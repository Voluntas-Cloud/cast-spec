//! `immutable_blob_store` — blobs replaced, never mutated.

/// Sentinel for `immutable_blob_store`.
pub struct ImmutableBlobStore;

cast::concept! {
    name: "immutable_blob_store",
    summary: "Blobs are never mutated, only replaced by new blobs. \
              Identity for a blob ties to its content or its assigned \
              ID at creation; updates produce new blobs with their \
              own identity.",
    anchors: [cast_stdlib::storage::immutable_blob_store::ImmutableBlobStore],
    tags: ["cast_stdlib", "storage"],
}
