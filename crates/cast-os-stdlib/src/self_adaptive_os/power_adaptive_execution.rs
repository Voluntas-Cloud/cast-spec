//! `power_adaptive_execution` — workloads scheduled by energy policy.

/// Sentinel for `power_adaptive_execution`.
pub struct PowerAdaptiveExecution;

cast::concept! {
    name: "power_adaptive_execution",
    summary: "workloads scheduled by energy policy.",
    anchors: [cast_os_stdlib::self_adaptive_os::power_adaptive_execution::PowerAdaptiveExecution],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
