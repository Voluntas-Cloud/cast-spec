//! `health_check` — determine if a service can run.

/// Sentinel for `health_check`.
pub struct HealthCheck;

cast::concept! {
    name: "health_check",
    summary: "Determine if service can run. The umbrella signal; \
              specialized into liveness (should I be restarted?) and \
              readiness (should I receive traffic?).",
    anchors: [cast_stdlib::reliability::health_check::HealthCheck],
    tags: ["cast_stdlib", "reliability"],
}
