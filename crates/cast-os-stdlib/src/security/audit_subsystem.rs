//! `audit_subsystem` — security-relevant event recording.

/// Sentinel for `audit_subsystem`.
pub struct AuditSubsystem;

cast::concept! {
    name: "audit_subsystem",
    summary: "security-relevant event recording.",
    anchors: [cast_os_stdlib::security::audit_subsystem::AuditSubsystem],
    tags: ["cast_os_stdlib", "security"],
}
