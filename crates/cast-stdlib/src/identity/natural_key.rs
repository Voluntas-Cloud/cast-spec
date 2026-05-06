//! `natural_key` — identity from real-world properties.

/// Sentinel for `natural_key`.
pub struct NaturalKey;

cast::concept! {
    name: "natural_key",
    summary: "Identity based on real-world properties (SSN, ISBN, \
              email). Unstable in practice — humans rename, \
              re-marry, get reassigned numbers.",
    anchors: [cast_stdlib::identity::natural_key::NaturalKey],
    tags: ["cast_stdlib", "identity"],
}
