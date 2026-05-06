//! `livelock` — system active but makes no progress.

/// Sentinel for `livelock`.
pub struct Livelock;

cast::concept! {
    name: "livelock",
    summary: "system active but makes no progress.",
    anchors: [cast_os_stdlib::failure_modes::livelock::Livelock],
    tags: ["cast_os_stdlib", "failure_modes"],
}
