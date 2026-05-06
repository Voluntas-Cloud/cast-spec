//! `log_correlation_id` — trace related logs across systems.

/// Sentinel for `log_correlation_id`.
pub struct LogCorrelationId;

cast::concept! {
    name: "log_correlation_id",
    summary: "Trace related logs across systems. A single ID threads \
              through every log line on the request's path; lets you \
              reconstruct one operation from many systems' logs.",
    anchors: [cast_stdlib::observability::log_correlation_id::LogCorrelationId],
    tags: ["cast_stdlib", "observability"],
}
