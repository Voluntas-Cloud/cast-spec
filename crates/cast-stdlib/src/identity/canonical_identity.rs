//! `canonical_identity` — the chosen long-term handle among aliases.

/// Sentinel for `canonical_identity`.
pub struct CanonicalIdentity;

cast::concept! {
    name: "canonical_identity",
    summary: "One chosen 'real' identity among aliases. Aliases may \
              come and go; the canonical identity is the long-term \
              handle the rest of the system uses.",
    anchors: [cast_stdlib::identity::canonical_identity::CanonicalIdentity],
    tags: ["cast_stdlib", "identity"],
}
