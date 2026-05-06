//! `cpu_affinity` — restrict or prefer execution on certain CPUs.

/// Sentinel for `cpu_affinity`.
pub struct CpuAffinity;

cast::concept! {
    name: "cpu_affinity",
    summary: "restrict or prefer execution on certain CPUs.",
    anchors: [cast_os_stdlib::scheduling::cpu_affinity::CpuAffinity],
    tags: ["cast_os_stdlib", "scheduling"],
}
