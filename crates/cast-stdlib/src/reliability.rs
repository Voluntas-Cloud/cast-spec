//! Reliability & resilience patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod backup_restore_test;
pub mod bulkhead_isolation;
pub mod chaos_testing;
pub mod checkpoint_resume;
pub mod circuit_breaker;
pub mod crash_only_design;
pub mod deadline_propagation;
pub mod disaster_recovery_plan;
pub mod exponential_backoff;
pub mod failover;
pub mod fallback_response;
pub mod graceful_shutdown;
pub mod health_check;
pub mod jittered_retry;
pub mod liveness_check;
pub mod quorum_decision;
pub mod readiness_check;
pub mod redundancy;
pub mod retry_with_backoff;
pub mod split_brain_prevention;
pub mod timeout_budget;

cast::concept! {
    name: "reliability",
    summary: "Umbrella for the reliability stdlib category. Reliability & \
              resilience patterns.",
    anchors: [
        crate::reliability::backup_restore_test,
        crate::reliability::bulkhead_isolation,
        crate::reliability::chaos_testing,
        crate::reliability::checkpoint_resume,
        crate::reliability::circuit_breaker,
        crate::reliability::crash_only_design,
        crate::reliability::deadline_propagation,
        crate::reliability::disaster_recovery_plan,
        crate::reliability::exponential_backoff,
        crate::reliability::failover,
        crate::reliability::fallback_response,
        crate::reliability::graceful_shutdown,
        crate::reliability::health_check,
        crate::reliability::jittered_retry,
        crate::reliability::liveness_check,
        crate::reliability::quorum_decision,
        crate::reliability::readiness_check,
        crate::reliability::redundancy,
        crate::reliability::retry_with_backoff,
        crate::reliability::split_brain_prevention,
        crate::reliability::timeout_budget,
    ],
    tags: ["cast_stdlib", "reliability"],
}

/// Sentinel for the reliability stdlib group.
pub struct ReliabilityGroup;

cast::rule! {
    rule: "Backups that are not restored in tests are decorative anxiety blankets.",
    why:  "An untested backup is just hopeful storage. The disaster \
           where you find out the restore path is broken is not when \
           you want to find out.",
    governs: [cast_stdlib::reliability::ReliabilityGroup],
    tags: ["cast_stdlib", "reliability"],
}
