//! `device_suspend_resume` — device power lifecycle.

/// Sentinel for `device_suspend_resume`.
pub struct DeviceSuspendResume;

cast::concept! {
    name: "device_suspend_resume",
    summary: "device power lifecycle.",
    anchors: [cast_os_stdlib::power_thermal::device_suspend_resume::DeviceSuspendResume],
    tags: ["cast_os_stdlib", "power_thermal"],
}
