//! `bottom_half_handler` — deferred interrupt processing.

/// Sentinel for `bottom_half_handler`.
pub struct BottomHalfHandler;

cast::concept! {
    name: "bottom_half_handler",
    summary: "deferred interrupt processing.",
    anchors: [cast_os_stdlib::driver_model::bottom_half_handler::BottomHalfHandler],
    tags: ["cast_os_stdlib", "driver_model"],
}
