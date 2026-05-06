//! `per_cpu_data_structure` — data replicated per CPU.

/// Sentinel for `per_cpu_data_structure`.
pub struct PerCpuDataStructure;

cast::concept! {
    name: "per_cpu_data_structure",
    summary: "data replicated per CPU.",
    anchors: [cast_os_stdlib::multicore_numa::per_cpu_data_structure::PerCpuDataStructure],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
