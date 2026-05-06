//! `distributed_barrier` — wait for participants to reach a point.

/// Sentinel for `distributed_barrier`.
pub struct DistributedBarrier;

cast::concept! {
    name: "distributed_barrier",
    summary: "Wait for participants to reach point. None proceeds past \
              the barrier until all have arrived; substrate for \
              phased computation across nodes.",
    anchors: [cast_stdlib::distributed::distributed_barrier::DistributedBarrier],
    tags: ["cast_stdlib", "distributed"],
}
