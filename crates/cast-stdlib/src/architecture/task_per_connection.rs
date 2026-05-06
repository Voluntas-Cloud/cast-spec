//! `task_per_connection` — accept loop spawning one async task per accepted connection.

/// Sentinel for `task_per_connection`.
pub struct TaskPerConnection;

cast::concept! {
    name: "task_per_connection",
    summary: "An accept loop spawns one independent async task per \
              accepted connection; each task owns its socket for the \
              connection's lifetime, runs its protocol loop, and \
              exits when the peer disconnects. Concurrency is bounded \
              by the connection count plus a runtime-level cap, not \
              by a thread pool. Failures inside one connection don't \
              affect siblings; back-pressure is per-connection. The \
              alternative — single-threaded multiplexing — is denser \
              but costs the per-connection isolation.",
    anchors: [cast_stdlib::architecture::task_per_connection::TaskPerConnection],
    tags: ["cast_stdlib", "architecture"],
}
