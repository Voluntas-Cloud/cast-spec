//! `ephemeral_storage` — temporary, reconstructible from durable sources.

/// Sentinel for `ephemeral_storage`.
pub struct EphemeralStorage;

cast::concept! {
    name: "ephemeral_storage",
    summary: "Temporary data with no durability guarantee. Survives \
              process for performance reasons but not for correctness \
              — the system must be able to rebuild it from durable \
              sources.",
    anchors: [cast_stdlib::storage::ephemeral_storage::EphemeralStorage],
    tags: ["cast_stdlib", "storage"],
}
