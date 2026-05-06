//! `cpu_idle_state_management` — enter low-power CPU states.

/// Sentinel for `cpu_idle_state_management`.
pub struct CpuIdleStateManagement;

cast::concept! {
    name: "cpu_idle_state_management",
    summary: "enter low-power CPU states.",
    anchors: [cast_os_stdlib::power_thermal::cpu_idle_state_management::CpuIdleStateManagement],
    tags: ["cast_os_stdlib", "power_thermal"],
}
