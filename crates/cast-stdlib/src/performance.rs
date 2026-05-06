//! Performance & scalability patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod autoscaling;
pub mod batching;
pub mod cache_aside;
pub mod capacity_planning;
pub mod coalescing_writes;
pub mod debounce;
pub mod eager_precomputation;
pub mod horizontal_scaling;
pub mod hot_key_mitigation;
pub mod indexing_strategy;
pub mod latency_budget;
pub mod lazy_evaluation;
pub mod load_shedding;
pub mod negative_cache;
pub mod priority_queueing;
pub mod rate_limiting;
pub mod sharding;
pub mod streaming_processing;
pub mod throughput_limit;
pub mod vertical_scaling;
pub mod write_behind_cache;
pub mod write_through_cache;

cast::concept! {
    name: "performance",
    summary: "Umbrella for the performance stdlib category. Performance & \
              scalability patterns.",
    anchors: [
        crate::performance::autoscaling,
        crate::performance::batching,
        crate::performance::cache_aside,
        crate::performance::capacity_planning,
        crate::performance::coalescing_writes,
        crate::performance::debounce,
        crate::performance::eager_precomputation,
        crate::performance::horizontal_scaling,
        crate::performance::hot_key_mitigation,
        crate::performance::indexing_strategy,
        crate::performance::latency_budget,
        crate::performance::lazy_evaluation,
        crate::performance::load_shedding,
        crate::performance::negative_cache,
        crate::performance::priority_queueing,
        crate::performance::rate_limiting,
        crate::performance::sharding,
        crate::performance::streaming_processing,
        crate::performance::throughput_limit,
        crate::performance::vertical_scaling,
        crate::performance::write_behind_cache,
        crate::performance::write_through_cache,
    ],
    tags: ["cast_stdlib", "performance"],
}

/// Sentinel for the performance stdlib group.
pub struct PerformanceGroup;

cast::rule! {
    rule: "Measure before optimizing.",
    why:  "Unless the code is obviously doing something ridiculous, \
           which naturally it often is. Optimization without \
           measurement is just personal preference dressed up as \
           engineering.",
    governs: [cast_stdlib::performance::PerformanceGroup],
    tags: ["cast_stdlib", "performance"],
}
