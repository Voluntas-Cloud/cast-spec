//! Process, thread, and execution model concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod clone_based_process_creation;
pub mod daemon_process_model;
pub mod exec_image_replacement;
pub mod fiber_execution_model;
pub mod fork_exec_model;
pub mod green_thread;
pub mod job_object_model;
pub mod kernel_thread;
pub mod orphan_process_reparenting;
pub mod process_abstraction;
pub mod process_control_block;
pub mod process_tree;
pub mod service_process_model;
pub mod session_and_process_group;
pub mod spawn_model;
pub mod task_struct_like_process_record;
pub mod thread_abstraction;
pub mod thread_control_block;
pub mod user_thread;
pub mod zombie_process_state;

cast::concept! {
    name: "execution_model",
    summary: "Umbrella for the execution_model stdlib category. Process, \
              thread, and execution model concepts.",
    anchors: [
        crate::execution_model::clone_based_process_creation,
        crate::execution_model::daemon_process_model,
        crate::execution_model::exec_image_replacement,
        crate::execution_model::fiber_execution_model,
        crate::execution_model::fork_exec_model,
        crate::execution_model::green_thread,
        crate::execution_model::job_object_model,
        crate::execution_model::kernel_thread,
        crate::execution_model::orphan_process_reparenting,
        crate::execution_model::process_abstraction,
        crate::execution_model::process_control_block,
        crate::execution_model::process_tree,
        crate::execution_model::service_process_model,
        crate::execution_model::session_and_process_group,
        crate::execution_model::spawn_model,
        crate::execution_model::task_struct_like_process_record,
        crate::execution_model::thread_abstraction,
        crate::execution_model::thread_control_block,
        crate::execution_model::user_thread,
        crate::execution_model::zombie_process_state,
    ],
    tags: ["cast_os_stdlib", "execution_model"],
}

/// Sentinel for the execution_model stdlib group.
pub struct ExecutionModelGroup;
