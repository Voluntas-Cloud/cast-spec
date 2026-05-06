//! `energy_model_scheduler_input` — scheduler uses energy model.

/// Sentinel for `energy_model_scheduler_input`.
pub struct EnergyModelSchedulerInput;

cast::concept! {
    name: "energy_model_scheduler_input",
    summary: "scheduler uses energy model.",
    anchors: [cast_os_stdlib::power_thermal::energy_model_scheduler_input::EnergyModelSchedulerInput],
    tags: ["cast_os_stdlib", "power_thermal"],
}
