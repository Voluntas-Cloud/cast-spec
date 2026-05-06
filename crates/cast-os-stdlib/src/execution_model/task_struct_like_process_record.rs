//! `task_struct_like_process_record` — kernel structure representing a task/process.

/// Sentinel for `task_struct_like_process_record`.
pub struct TaskStructLikeProcessRecord;

cast::concept! {
    name: "task_struct_like_process_record",
    summary: "kernel structure representing a task/process.",
    anchors: [cast_os_stdlib::execution_model::task_struct_like_process_record::TaskStructLikeProcessRecord],
    tags: ["cast_os_stdlib", "execution_model"],
}
