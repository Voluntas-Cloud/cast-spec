//! `audit_log` — durable record of security-relevant actions.

/// Sentinel for `audit_log`.
pub struct AuditLog;

cast::concept! {
    name: "audit_log",
    summary: "Durable record of security-relevant actions. Who did \
              what to which resource at what time, structured for \
              query, immune to silent rewriting.",
    anchors: [cast_stdlib::observability::audit_log::AuditLog],
    tags: ["cast_stdlib", "observability"],
}
