//! `opaque_identifier` — token with no domain meaning.

/// Sentinel for `opaque_identifier`.
pub struct OpaqueIdentifier;

cast::concept! {
    name: "opaque_identifier",
    summary: "ID with no semantic meaning embedded. Just a token; \
              callers cannot derive properties of the thing from its \
              ID. Survives any change to the thing because it never \
              encoded anything about the thing.",
    anchors: [cast_stdlib::identity::opaque_identifier::OpaqueIdentifier],
    tags: ["cast_stdlib", "identity"],
}
