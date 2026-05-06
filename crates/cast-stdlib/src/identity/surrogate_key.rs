//! `surrogate_key` — artificial system-assigned identity.

/// Sentinel for `surrogate_key`.
pub struct SurrogateKey;

cast::concept! {
    name: "surrogate_key",
    summary: "Artificial system-assigned identity. Decoupled from any \
              real-world property; immune to changes outside the \
              system's control.",
    anchors: [cast_stdlib::identity::surrogate_key::SurrogateKey],
    tags: ["cast_stdlib", "identity"],
}
