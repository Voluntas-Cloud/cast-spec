//! `plug_and_play_manager` — discover/configure devices dynamically.

/// Sentinel for `plug_and_play_manager`.
pub struct PlugAndPlayManager;

cast::concept! {
    name: "plug_and_play_manager",
    summary: "discover/configure devices dynamically.",
    anchors: [cast_os_stdlib::driver_model::plug_and_play_manager::PlugAndPlayManager],
    tags: ["cast_os_stdlib", "driver_model"],
}
