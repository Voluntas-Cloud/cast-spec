//! `admission_control` — reject work before overload.

/// Sentinel for `admission_control`.
pub struct AdmissionControl;

cast::concept! {
    name: "admission_control",
    summary: "Reject work before overload. The system declines \
              additional load when accepting it would degrade what is \
              already running; saying \"no\" early is cheaper than \
              cascading retries.",
    anchors: [cast_stdlib::resources::admission_control::AdmissionControl],
    tags: ["cast_stdlib", "resources"],
}
