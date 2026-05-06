//! `system_sleep_state` — suspend/hibernate behavior.

/// Sentinel for `system_sleep_state`.
pub struct SystemSleepState;

cast::concept! {
    name: "system_sleep_state",
    summary: "suspend/hibernate behavior.",
    anchors: [cast_os_stdlib::power_thermal::system_sleep_state::SystemSleepState],
    tags: ["cast_os_stdlib", "power_thermal"],
}
