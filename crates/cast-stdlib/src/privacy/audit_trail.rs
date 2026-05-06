//! `audit_trail` — trace who did what and when.

/// Sentinel for `audit_trail`.
pub struct AuditTrail;

cast::concept! {
    name: "audit_trail",
    summary: "Trace who did what and when. Sensitive reads as well as \
              writes land in the trail; the difference between the two \
              is what auditors actually want to see.",
    anchors: [cast_stdlib::privacy::audit_trail::AuditTrail],
    tags: ["cast_stdlib", "privacy"],
}
