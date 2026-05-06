//! `auditability` — security actions can be reviewed.

/// Sentinel for `auditability`.
pub struct Auditability;

cast::concept! {
    name: "auditability",
    summary: "Security actions can be reviewed. Every grant, revocation, \
              and elevation lands in a tamper-evident log that survives \
              the compromise of the system being audited.",
    anchors: [cast_stdlib::security::auditability::Auditability],
    tags: ["cast_stdlib", "security"],
}
