//! `envelope_encryption` — encrypt data keys with master keys.

/// Sentinel for `envelope_encryption`.
pub struct EnvelopeEncryption;

cast::concept! {
    name: "envelope_encryption",
    summary: "Encrypt data keys with master keys. Bulk data is \
              encrypted by short-lived data keys; the master key only \
              ever wraps and unwraps those, so it can live in an HSM \
              and never touch the payload directly.",
    anchors: [cast_stdlib::security::envelope_encryption::EnvelopeEncryption],
    tags: ["cast_stdlib", "security"],
}
