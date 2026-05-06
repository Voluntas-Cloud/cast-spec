//! `tasklet` — older Linux deferred work mechanism.

/// Sentinel for `tasklet`.
pub struct Tasklet;

cast::concept! {
    name: "tasklet",
    summary: "older Linux deferred work mechanism.",
    anchors: [cast_os_stdlib::driver_model::tasklet::Tasklet],
    tags: ["cast_os_stdlib", "driver_model"],
}
