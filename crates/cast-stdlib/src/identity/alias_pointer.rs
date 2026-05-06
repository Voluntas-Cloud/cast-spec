//! `alias_pointer` — name pointing to a canonical identity.

/// Sentinel for `alias_pointer`.
pub struct AliasPointer;

cast::concept! {
    name: "alias_pointer",
    summary: "Name that points to another canonical identity. Handles \
              renames, redirects, and 'this is the new name for that \
              thing' migrations without breaking old references.",
    anchors: [cast_stdlib::identity::alias_pointer::AliasPointer],
    tags: ["cast_stdlib", "identity"],
}
