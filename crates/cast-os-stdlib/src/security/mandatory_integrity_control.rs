//! `mandatory_integrity_control` — Windows integrity-level isolation.

/// Sentinel for `mandatory_integrity_control`.
pub struct MandatoryIntegrityControl;

cast::concept! {
    name: "mandatory_integrity_control",
    summary: "Windows integrity-level isolation.",
    anchors: [cast_os_stdlib::security::mandatory_integrity_control::MandatoryIntegrityControl],
    tags: ["cast_os_stdlib", "security"],
}
