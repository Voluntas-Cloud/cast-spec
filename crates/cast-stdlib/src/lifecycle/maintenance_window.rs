//! `maintenance_window` — bounded period for disruptive change.

/// Sentinel for `maintenance_window`.
pub struct MaintenanceWindow;

cast::concept! {
    name: "maintenance_window",
    summary: "Bounded period for disruptive change. Pre-announced, \
              short, and watched; anything outside it is supposed to \
              be invisible to users.",
    anchors: [cast_stdlib::lifecycle::maintenance_window::MaintenanceWindow],
    tags: ["cast_stdlib", "lifecycle"],
}
