//! `security` — resists unauthorized behavior.

/// Sentinel for `security`.
pub struct Security;

cast::concept! {
    name: "security",
    summary: "resists unauthorized behavior.",
    anchors: [cast_os_stdlib::design_qualities::security::Security],
    tags: ["cast_os_stdlib", "design_qualities"],
}
