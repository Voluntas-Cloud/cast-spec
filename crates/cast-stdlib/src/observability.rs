//! Observability & diagnostics patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod alert_routing;
pub mod audit_log;
pub mod blackbox_monitoring;
pub mod core_dump;
pub mod debug_endpoint;
pub mod diagnostic_bundle;
pub mod distributed_tracing;
pub mod error_budget;
pub mod event_timeline;
pub mod heap_dump;
pub mod high_cardinality_metric_guard;
pub mod log_correlation_id;
pub mod metrics_collection;
pub mod profiling;
pub mod runbook_linking;
pub mod service_level_indicator;
pub mod service_level_objective;
pub mod structured_logging;
pub mod synthetic_probe;
pub mod whitebox_monitoring;

cast::concept! {
    name: "observability",
    summary: "Umbrella for the observability stdlib category. \
              Observability & diagnostics patterns.",
    anchors: [
        crate::observability::alert_routing,
        crate::observability::audit_log,
        crate::observability::blackbox_monitoring,
        crate::observability::core_dump,
        crate::observability::debug_endpoint,
        crate::observability::diagnostic_bundle,
        crate::observability::distributed_tracing,
        crate::observability::error_budget,
        crate::observability::event_timeline,
        crate::observability::heap_dump,
        crate::observability::high_cardinality_metric_guard,
        crate::observability::log_correlation_id,
        crate::observability::metrics_collection,
        crate::observability::profiling,
        crate::observability::runbook_linking,
        crate::observability::service_level_indicator,
        crate::observability::service_level_objective,
        crate::observability::structured_logging,
        crate::observability::synthetic_probe,
        crate::observability::whitebox_monitoring,
    ],
    tags: ["cast_stdlib", "observability"],
}

/// Sentinel for the observability stdlib group.
pub struct ObservabilityGroup;

cast::rule! {
    rule: "Emit enough state to debug without SSH séance rituals.",
    why:  "If diagnosing production requires logging into the host, \
           grepping for clues, and divining a story from process \
           memory, you do not have observability — you have an \
           incident reenactment hobby.",
    governs: [cast_stdlib::observability::ObservabilityGroup],
    tags: ["cast_stdlib", "observability"],
}
