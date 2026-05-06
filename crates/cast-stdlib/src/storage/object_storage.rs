//! `object_storage` — get/put/delete by key, no filesystem semantics.

/// Sentinel for `object_storage`.
pub struct ObjectStorage;

cast::concept! {
    name: "object_storage",
    summary: "Blob/key storage instead of filesystem semantics. No \
              hierarchy, no partial-write, no rename — just \
              get/put/delete by key.",
    anchors: [cast_stdlib::storage::object_storage::ObjectStorage],
    tags: ["cast_stdlib", "storage"],
}
