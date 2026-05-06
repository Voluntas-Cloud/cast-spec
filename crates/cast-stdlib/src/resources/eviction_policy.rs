//! `eviction_policy` — choose what gets removed under pressure.

/// Sentinel for `eviction_policy`.
pub struct EvictionPolicy;

cast::concept! {
    name: "eviction_policy",
    summary: "Choose what gets removed under pressure. When capacity \
              drops the system already knows the order in which \
              workloads die; making that order implicit is how the \
              wrong thing always goes first.",
    anchors: [cast_stdlib::resources::eviction_policy::EvictionPolicy],
    tags: ["cast_stdlib", "resources"],
}
