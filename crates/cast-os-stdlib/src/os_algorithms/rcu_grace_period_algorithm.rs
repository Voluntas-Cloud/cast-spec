//! `rcu_grace_period_algorithm` — determine when old data can be freed.

/// Sentinel for `rcu_grace_period_algorithm`.
pub struct RcuGracePeriodAlgorithm;

cast::concept! {
    name: "rcu_grace_period_algorithm",
    summary: "determine when old data can be freed.",
    anchors: [cast_os_stdlib::os_algorithms::rcu_grace_period_algorithm::RcuGracePeriodAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
