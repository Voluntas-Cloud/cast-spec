//! `workload_classification` — detect workload type automatically.

/// Sentinel for `workload_classification`.
pub struct WorkloadClassification;

cast::concept! {
    name: "workload_classification",
    summary: "detect workload type automatically.",
    anchors: [cast_os_stdlib::self_adaptive_os::workload_classification::WorkloadClassification],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
