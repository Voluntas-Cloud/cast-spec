//! `incident_response_system` — failures are detected, classified, routed, investigated, and mitigated.

/// Sentinel for `incident_response_system`.
pub struct IncidentResponseSystem;

cast::concept! {
    name: "incident_response_system",
    summary: "Failures are detected, classified, routed, \
              investigated, and mitigated. Composes alert_routing, \
              runbook_linking, event_timeline, error_contract, \
              health_check, synthetic_probe, diagnostic_bundle, and \
              rollback_operation. Used for production support, \
              personal-cloud self-repair, operator dashboards, \
              automated incident triage, and cluster failure \
              handling.",
    anchors: [cast_stdlib::patterns::incident_response_system::IncidentResponseSystem],
    tags: ["cast_stdlib", "patterns"],
}
