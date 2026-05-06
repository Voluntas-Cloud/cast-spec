//! `job_object_model` — Windows-style grouping and resource control.

/// Sentinel for `job_object_model`.
pub struct JobObjectModel;

cast::concept! {
    name: "job_object_model",
    summary: "Windows-style grouping and resource control.",
    anchors: [cast_os_stdlib::execution_model::job_object_model::JobObjectModel],
    tags: ["cast_os_stdlib", "execution_model"],
}
