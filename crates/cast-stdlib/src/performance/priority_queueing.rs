//! `priority_queueing` — handle important work first.

/// Sentinel for `priority_queueing`.
pub struct PriorityQueueing;

cast::concept! {
    name: "priority_queueing",
    summary: "Handle important work first. Under overload, latency-\
              sensitive or revenue-critical requests jump the queue; \
              starvation of low-priority work is the cost.",
    anchors: [cast_stdlib::performance::priority_queueing::PriorityQueueing],
    tags: ["cast_stdlib", "performance"],
}
