//! `append_only_audit_trail` — durable record of important actions.

/// Sentinel for `append_only_audit_trail`.
pub struct AppendOnlyAuditTrail;

cast::concept! {
    name: "append_only_audit_trail",
    summary: "A durable record of important actions, especially the \
              security-sensitive or operationally significant ones. \
              Composes append_only_log, principal_authentication, \
              signed_request, correlation_id, monotonic_sequence_id, \
              audit_log, tamper_evidence, and retention_policy. Used \
              for security audits, admin action tracking, AI agent \
              action history, compliance evidence, and \"who did \
              this?\" debugging.",
    anchors: [cast_stdlib::patterns::append_only_audit_trail::AppendOnlyAuditTrail],
    tags: ["cast_stdlib", "patterns"],
}
