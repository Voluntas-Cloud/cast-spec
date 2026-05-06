//! Resource management & scheduling patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod admission_control;
pub mod affinity_rule;
pub mod anti_affinity_rule;
pub mod bin_packing;
pub mod cost_budget_policy;
pub mod energy_aware_scheduling;
pub mod eviction_policy;
pub mod fair_scheduling;
pub mod gpu_scheduling;
pub mod preemption;
pub mod priority_scheduling;
pub mod quota_enforcement;
pub mod resource_limit;
pub mod resource_request;
pub mod storage_class_selection;
pub mod taint_toleration;

cast::concept! {
    name: "resources",
    summary: "Umbrella for the resources stdlib category. Resource \
              management & scheduling patterns.",
    anchors: [
        crate::resources::admission_control,
        crate::resources::affinity_rule,
        crate::resources::anti_affinity_rule,
        crate::resources::bin_packing,
        crate::resources::cost_budget_policy,
        crate::resources::energy_aware_scheduling,
        crate::resources::eviction_policy,
        crate::resources::fair_scheduling,
        crate::resources::gpu_scheduling,
        crate::resources::preemption,
        crate::resources::priority_scheduling,
        crate::resources::quota_enforcement,
        crate::resources::resource_limit,
        crate::resources::resource_request,
        crate::resources::storage_class_selection,
        crate::resources::taint_toleration,
    ],
    tags: ["cast_stdlib", "resources"],
}

/// Sentinel for the resources stdlib group.
pub struct ResourcesGroup;

cast::rule! {
    rule: "Scheduling should reflect user intent, technical constraints, and money.",
    why:  "Especially money, because clouds are vending machines for \
           regret. Every scheduler that ignores cost rediscovers the \
           same lesson when the bill arrives.",
    governs: [cast_stdlib::resources::ResourcesGroup],
    tags: ["cast_stdlib", "resources"],
}
