//! `safety_critical_os` — OS for systems where failure may harm people.

/// Sentinel for `safety_critical_os`.
pub struct SafetyCriticalOs;

cast::concept! {
    name: "safety_critical_os",
    summary: "OS for systems where failure may harm people.",
    anchors: [cast_os_stdlib::os_use_cases::safety_critical_os::SafetyCriticalOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
