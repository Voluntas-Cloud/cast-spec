//! `working_set_model` — memory needed by active computation.

/// Sentinel for `working_set_model`.
pub struct WorkingSetModel;

cast::concept! {
    name: "working_set_model",
    summary: "memory needed by active computation.",
    anchors: [cast_os_stdlib::memory_management::working_set_model::WorkingSetModel],
    tags: ["cast_os_stdlib", "memory_management"],
}
