//! `audit_history` — preserve who/what/when.

/// Sentinel for `audit_history`.
pub struct AuditHistory;

cast::concept! {
    name: "audit_history",
    summary: "Preserve who changed what, and when. Different from \
              audit_trail in scope: audit_history is the immutable \
              record of state evolution, queryable as a sequence of \
              changes rather than as discrete actions.",
    anchors: [cast_stdlib::time_ordering::audit_history::AuditHistory],
    tags: ["cast_stdlib", "time_ordering"],
}
