//! `versioned_identifier` — identity carries or resolves to a version.

/// Sentinel for `versioned_identifier`.
pub struct VersionedIdentifier;

cast::concept! {
    name: "versioned_identifier",
    summary: "Identity includes or resolves to a version. Distinguishes \
              the same logical thing across versions; readers ask \
              for a specific version or 'latest'.",
    anchors: [cast_stdlib::identity::versioned_identifier::VersionedIdentifier],
    tags: ["cast_stdlib", "identity"],
}
