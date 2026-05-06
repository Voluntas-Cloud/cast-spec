//! `audit_event_stream` — security event trace.

/// Sentinel for `audit_event_stream`.
pub struct AuditEventStream;

cast::concept! {
    name: "audit_event_stream",
    summary: "security event trace.",
    anchors: [cast_os_stdlib::observability::audit_event_stream::AuditEventStream],
    tags: ["cast_os_stdlib", "observability"],
}
