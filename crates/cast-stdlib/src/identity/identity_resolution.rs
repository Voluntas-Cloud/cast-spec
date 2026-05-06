//! `identity_resolution` — translate external identities to internal entities.

/// Sentinel for `identity_resolution`.
pub struct IdentityResolution;

cast::concept! {
    name: "identity_resolution",
    summary: "Mapping external identities to internal entities. The \
              outside world has its own ID schemes; this layer is \
              where those get translated to the internal canonical ID.",
    anchors: [cast_stdlib::identity::identity_resolution::IdentityResolution],
    tags: ["cast_stdlib", "identity"],
}
