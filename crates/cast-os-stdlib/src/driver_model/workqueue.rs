//! `workqueue` — deferred kernel work execution.

/// Sentinel for `workqueue`.
pub struct Workqueue;

cast::concept! {
    name: "workqueue",
    summary: "deferred kernel work execution.",
    anchors: [cast_os_stdlib::driver_model::workqueue::Workqueue],
    tags: ["cast_os_stdlib", "driver_model"],
}
