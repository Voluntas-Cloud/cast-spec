//! `identity_migration` — change identifier without breaking references.

/// Sentinel for `identity_migration`.
pub struct IdentityMigration;

cast::concept! {
    name: "identity_migration",
    summary: "Changing identifiers without breaking references. \
              Typically uses alias pointers as a bridge phase: old \
              ID redirects to new ID until all callers update.",
    anchors: [cast_stdlib::identity::identity_migration::IdentityMigration],
    tags: ["cast_stdlib", "identity"],
}
