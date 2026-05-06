//! `synthetic_probe` — fake user/request to test service.

/// Sentinel for `synthetic_probe`.
pub struct SyntheticProbe;

cast::concept! {
    name: "synthetic_probe",
    summary: "Fake user/request to test service. A predictable, \
              continuously-running request whose failure indicates \
              the service is broken before real users notice.",
    anchors: [cast_stdlib::observability::synthetic_probe::SyntheticProbe],
    tags: ["cast_stdlib", "observability"],
}
