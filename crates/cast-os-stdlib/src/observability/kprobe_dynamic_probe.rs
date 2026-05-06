//! `kprobe_dynamic_probe` — probe kernel function.

/// Sentinel for `kprobe_dynamic_probe`.
pub struct KprobeDynamicProbe;

cast::concept! {
    name: "kprobe_dynamic_probe",
    summary: "probe kernel function.",
    anchors: [cast_os_stdlib::observability::kprobe_dynamic_probe::KprobeDynamicProbe],
    tags: ["cast_os_stdlib", "observability"],
}
