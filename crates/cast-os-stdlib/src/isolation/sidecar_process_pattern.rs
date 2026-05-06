//! `sidecar_process_pattern` — helper process/container alongside workload.

/// Sentinel for `sidecar_process_pattern`.
pub struct SidecarProcessPattern;

cast::concept! {
    name: "sidecar_process_pattern",
    summary: "helper process/container alongside workload.",
    anchors: [cast_os_stdlib::isolation::sidecar_process_pattern::SidecarProcessPattern],
    tags: ["cast_os_stdlib", "isolation"],
}
