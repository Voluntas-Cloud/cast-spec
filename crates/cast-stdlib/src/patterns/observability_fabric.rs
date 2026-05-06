//! `observability_fabric` — logs, metrics, traces, audits, and diagnostics form one operational view.

/// Sentinel for `observability_fabric`.
pub struct ObservabilityFabric;

cast::concept! {
    name: "observability_fabric",
    summary: "Logs, metrics, traces, audits, and diagnostics form \
              one operational view. Composes structured_logging, \
              distributed_tracing, metrics_collection, \
              correlation_id, audit_log, diagnostic_bundle, \
              synthetic_probe, and service_level_indicator. Used for \
              cluster debugging, production operations, incident \
              response, AI action tracing, and workflow visibility.",
    anchors: [cast_stdlib::patterns::observability_fabric::ObservabilityFabric],
    tags: ["cast_stdlib", "patterns"],
}
