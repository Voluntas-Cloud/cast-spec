//! `round_robin_scheduling` — tasks run in cyclic order.

/// Sentinel for `round_robin_scheduling`.
pub struct RoundRobinScheduling;

cast::concept! {
    name: "round_robin_scheduling",
    summary: "tasks run in cyclic order.",
    anchors: [cast_os_stdlib::scheduling::round_robin_scheduling::RoundRobinScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
