//! `flame_graph_profile` — visualize sampled stack costs.

/// Sentinel for `flame_graph_profile`.
pub struct FlameGraphProfile;

cast::concept! {
    name: "flame_graph_profile",
    summary: "visualize sampled stack costs.",
    anchors: [cast_os_stdlib::observability::flame_graph_profile::FlameGraphProfile],
    tags: ["cast_os_stdlib", "observability"],
}
