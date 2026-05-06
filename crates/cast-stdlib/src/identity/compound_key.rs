//! `compound_key` — identity is the tuple of fields.

/// Sentinel for `compound_key`.
pub struct CompoundKey;

cast::concept! {
    name: "compound_key",
    summary: "Identity composed from multiple fields. Often arises \
              when no single field is unique; identity is the tuple.",
    anchors: [cast_stdlib::identity::compound_key::CompoundKey],
    tags: ["cast_stdlib", "identity"],
}
