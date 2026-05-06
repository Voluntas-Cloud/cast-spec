//! `security_label` — object/process security metadata.

/// Sentinel for `security_label`.
pub struct SecurityLabel;

cast::concept! {
    name: "security_label",
    summary: "object/process security metadata.",
    anchors: [cast_os_stdlib::kernel_data_structures::security_label::SecurityLabel],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
