//! `preemption` — stop lower-priority work for higher-priority work.

/// Sentinel for `preemption`.
pub struct Preemption;

cast::concept! {
    name: "preemption",
    summary: "Stop lower-priority work to free capacity for \
              higher-priority work. The displaced workload must tolerate \
              being killed mid-stride; otherwise preemption breaks \
              correctness, not just SLOs.",
    anchors: [cast_stdlib::resources::preemption::Preemption],
    tags: ["cast_stdlib", "resources"],
}
