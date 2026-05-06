//! `durable_workflow_engine` — long-running workflows survive crashes and retries.

/// Sentinel for `durable_workflow_engine`.
pub struct DurableWorkflowEngine;

cast::concept! {
    name: "durable_workflow_engine",
    summary: "A system that executes long-running workflows safely \
              across crashes and retries. Composes stateful_workflow, \
              durable_execution, checkpoint_resume, retry_policy, \
              compensation_workflow, human_approval_step, \
              idempotent_operation, and event_stream. Used for order \
              processing, cluster bootstrap, user onboarding, AI task \
              execution, backup/restore workflows, and multi-step \
              deployments.",
    anchors: [cast_stdlib::patterns::durable_workflow_engine::DurableWorkflowEngine],
    tags: ["cast_stdlib", "patterns"],
}
