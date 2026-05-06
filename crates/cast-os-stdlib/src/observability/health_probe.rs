//! `health_probe` — liveness/readiness-like OS check.

/// Sentinel for `health_probe`.
pub struct HealthProbe;

cast::concept! {
    name: "health_probe",
    summary: "liveness/readiness-like OS check.",
    anchors: [cast_os_stdlib::observability::health_probe::HealthProbe],
    tags: ["cast_os_stdlib", "observability"],
}
