//! `a_b_partition_update` — update inactive slot, then boot into it.

/// Sentinel for `a_b_partition_update`.
pub struct ABPartitionUpdate;

cast::concept! {
    name: "a_b_partition_update",
    summary: "update inactive slot, then boot into it.",
    anchors: [cast_os_stdlib::update_evolution::a_b_partition_update::ABPartitionUpdate],
    tags: ["cast_os_stdlib", "update_evolution"],
}
