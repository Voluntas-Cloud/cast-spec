//! `encrypted_at_rest_storage` — encryption applied at the storage layer.

/// Sentinel for `encrypted_at_rest_storage`.
pub struct EncryptedAtRestStorage;

cast::concept! {
    name: "encrypted_at_rest_storage",
    summary: "Data is encrypted where it lives. The decryption key is \
              held outside the storage system — possession of the \
              media without the key reveals nothing useful.",
    anchors: [cast_stdlib::storage::encrypted_at_rest_storage::EncryptedAtRestStorage],
    tags: ["cast_stdlib", "storage"],
}
