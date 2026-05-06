//! `run_queue` — data structure holding runnable tasks.

/// Sentinel for `run_queue`.
pub struct RunQueue;

cast::concept! {
    name: "run_queue",
    summary: "data structure holding runnable tasks.",
    anchors: [cast_os_stdlib::scheduling::run_queue::RunQueue],
    tags: ["cast_os_stdlib", "scheduling"],
}
