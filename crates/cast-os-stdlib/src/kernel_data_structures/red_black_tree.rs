//! `red_black_tree` — balanced tree used in scheduling/timers/mm.

/// Sentinel for `red_black_tree`.
pub struct RedBlackTree;

cast::concept! {
    name: "red_black_tree",
    summary: "balanced tree used in scheduling/timers/mm.",
    anchors: [cast_os_stdlib::kernel_data_structures::red_black_tree::RedBlackTree],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
