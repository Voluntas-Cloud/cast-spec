//! `cpu_frequency_scaling` — adjust CPU frequency.

/// Sentinel for `cpu_frequency_scaling`.
pub struct CpuFrequencyScaling;

cast::concept! {
    name: "cpu_frequency_scaling",
    summary: "adjust CPU frequency.",
    anchors: [cast_os_stdlib::power_thermal::cpu_frequency_scaling::CpuFrequencyScaling],
    tags: ["cast_os_stdlib", "power_thermal"],
}
