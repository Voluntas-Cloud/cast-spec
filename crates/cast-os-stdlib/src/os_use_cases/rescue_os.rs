//! `rescue_os` — minimal recovery environment.

/// Sentinel for `rescue_os`.
pub struct RescueOs;

cast::concept! {
    name: "rescue_os",
    summary: "minimal recovery environment.",
    anchors: [cast_os_stdlib::os_use_cases::rescue_os::RescueOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
