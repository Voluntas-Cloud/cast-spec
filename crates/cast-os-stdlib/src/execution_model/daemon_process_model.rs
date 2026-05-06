//! `daemon_process_model` — long-running background service process.

/// Sentinel for `daemon_process_model`.
pub struct DaemonProcessModel;

cast::concept! {
    name: "daemon_process_model",
    summary: "long-running background service process.",
    anchors: [cast_os_stdlib::execution_model::daemon_process_model::DaemonProcessModel],
    tags: ["cast_os_stdlib", "execution_model"],
}
