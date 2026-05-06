//! Workflow & orchestration patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod checkpointed_pipeline;
pub mod compensation_workflow;
pub mod cron_semantics;
pub mod dag_workflow;
pub mod durable_execution;
pub mod event_triggered_job;
pub mod fan_out_fan_in;
pub mod human_approval_step;
pub mod manual_intervention_hook;
pub mod pipeline_backpressure;
pub mod retry_policy;
pub mod scheduled_job;
pub mod stateful_workflow;
pub mod task_graph;
pub mod work_queue;
pub mod workflow_versioning;

cast::concept! {
    name: "workflow",
    summary: "Umbrella for the workflow stdlib category. Workflow & \
              orchestration patterns.",
    anchors: [
        crate::workflow::checkpointed_pipeline,
        crate::workflow::compensation_workflow,
        crate::workflow::cron_semantics,
        crate::workflow::dag_workflow,
        crate::workflow::durable_execution,
        crate::workflow::event_triggered_job,
        crate::workflow::fan_out_fan_in,
        crate::workflow::human_approval_step,
        crate::workflow::manual_intervention_hook,
        crate::workflow::pipeline_backpressure,
        crate::workflow::retry_policy,
        crate::workflow::scheduled_job,
        crate::workflow::stateful_workflow,
        crate::workflow::task_graph,
        crate::workflow::work_queue,
        crate::workflow::workflow_versioning,
    ],
    tags: ["cast_stdlib", "workflow"],
}

/// Sentinel for the workflow stdlib group.
pub struct WorkflowGroup;

cast::rule! {
    rule: "Make long-running workflows durable and inspectable.",
    why:  "Invisible background magic is just failure with better \
           branding. A workflow you can't introspect at hour 3 is a \
           workflow you can only debug by re-running.",
    governs: [cast_stdlib::workflow::WorkflowGroup],
    tags: ["cast_stdlib", "workflow"],
}
