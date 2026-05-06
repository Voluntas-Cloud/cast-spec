//! `embedded_os` — constrained hardware OS.

/// Sentinel for `embedded_os`.
pub struct EmbeddedOs;

cast::concept! {
    name: "embedded_os",
    summary: "constrained hardware OS.",
    anchors: [cast_os_stdlib::os_use_cases::embedded_os::EmbeddedOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
