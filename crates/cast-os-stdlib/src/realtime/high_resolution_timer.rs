//! `high_resolution_timer` — precise timer mechanism.

/// Sentinel for `high_resolution_timer`.
pub struct HighResolutionTimer;

cast::concept! {
    name: "high_resolution_timer",
    summary: "precise timer mechanism.",
    anchors: [cast_os_stdlib::realtime::high_resolution_timer::HighResolutionTimer],
    tags: ["cast_os_stdlib", "realtime"],
}
