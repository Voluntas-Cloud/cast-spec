//! `key_rotation` — replacing keys regularly or after risk.

/// Sentinel for `key_rotation`.
pub struct KeyRotation;

cast::concept! {
    name: "key_rotation",
    summary: "Replacing cryptographic keys regularly or after risk. \
              Limits the time-window of damage from undetected key \
              compromise.",
    anchors: [cast_stdlib::trust::key_rotation::KeyRotation],
    tags: ["cast_stdlib", "trust"],
}
