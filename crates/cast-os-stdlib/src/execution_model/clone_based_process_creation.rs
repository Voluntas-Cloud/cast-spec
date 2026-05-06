//! `clone_based_process_creation` — process/thread creation via shared-resource flags.

/// Sentinel for `clone_based_process_creation`.
pub struct CloneBasedProcessCreation;

cast::concept! {
    name: "clone_based_process_creation",
    summary: "process/thread creation via shared-resource flags.",
    anchors: [cast_os_stdlib::execution_model::clone_based_process_creation::CloneBasedProcessCreation],
    tags: ["cast_os_stdlib", "execution_model"],
}
