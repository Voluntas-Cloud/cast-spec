//! `uprobe_user_probe` — probe user-space function.

/// Sentinel for `uprobe_user_probe`.
pub struct UprobeUserProbe;

cast::concept! {
    name: "uprobe_user_probe",
    summary: "probe user-space function.",
    anchors: [cast_os_stdlib::observability::uprobe_user_probe::UprobeUserProbe],
    tags: ["cast_os_stdlib", "observability"],
}
