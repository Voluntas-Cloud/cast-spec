//! Identity & naming patterns.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only the
//! group sentinel, cross-concept comparisons, and the category rule.

pub mod alias_pointer;
pub mod canonical_identity;
pub mod compound_key;
pub mod content_hash_id;
pub mod identity_migration;
pub mod identity_resolution;
pub mod monotonic_sequence_id;
pub mod namespace_scoped_id;
pub mod natural_key;
pub mod opaque_identifier;
pub mod semantic_identifier;
pub mod stable_logical_name;
pub mod surrogate_key;
pub mod temporary_identifier;
pub mod uuid_identity;
pub mod versioned_identifier;

cast::concept! {
    name: "identity",
    summary: "Umbrella for the identity stdlib category. Identity & naming \
              patterns.",
    anchors: [
        crate::identity::alias_pointer,
        crate::identity::canonical_identity,
        crate::identity::compound_key,
        crate::identity::content_hash_id,
        crate::identity::identity_migration,
        crate::identity::identity_resolution,
        crate::identity::monotonic_sequence_id,
        crate::identity::namespace_scoped_id,
        crate::identity::natural_key,
        crate::identity::opaque_identifier,
        crate::identity::semantic_identifier,
        crate::identity::stable_logical_name,
        crate::identity::surrogate_key,
        crate::identity::temporary_identifier,
        crate::identity::uuid_identity,
        crate::identity::versioned_identifier,
    ],
    tags: ["cast_stdlib", "identity"],
}

/// Sentinel for the identity stdlib group.
pub struct IdentityGroup;

cast::compare! {
    content_hash       @ cast_stdlib::identity::content_hash_id::ContentHashId:
        "computable from content; no authority needed; large opaque values; equality implies content equality; brittle to encoding changes",
    monotonic_sequence @ cast_stdlib::identity::monotonic_sequence_id::MonotonicSequenceId:
        "issued by an authority; reveals issuance order; small dense values; equality reveals nothing about content; needs centralization",
    note: "Two opposing ways to mint identifiers. Many real systems carry both — content hash for identity, sequence ID for ordering.",
    tags: ["cast_stdlib", "identity"],
}

cast::rule! {
    rule: "Use opaque stable IDs internally; layer human-readable names on top.",
    why:  "Humans love semantic IDs until someone renames a company, \
           marries, transitions, moves country, or otherwise rudely \
           has a life. The internal ID must outlive every renaming \
           event; semantic IDs cannot.",
    governs: [cast_stdlib::identity::IdentityGroup],
    tags: ["cast_stdlib", "identity"],
}
