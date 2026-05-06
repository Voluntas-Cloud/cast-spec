//! `cost_aware_cloud_bursting` — local infra temporarily uses external resources under policy.

/// Sentinel for `cost_aware_cloud_bursting`.
pub struct CostAwareCloudBursting;

cast::concept! {
    name: "cost_aware_cloud_bursting",
    summary: "Local infrastructure can temporarily use external \
              resources under policy control. Composes \
              cost_budget_policy, autoscaling, policy_as_code, \
              human_approval_step, capability_token, \
              resource_quota, workflow_orchestration, and \
              audit_log. Used for temporary GPU use, large AI jobs, \
              backup migration, peak workload handling, and paid \
              resource extensions.",
    anchors: [cast_stdlib::patterns::cost_aware_cloud_bursting::CostAwareCloudBursting],
    tags: ["cast_stdlib", "patterns"],
}
