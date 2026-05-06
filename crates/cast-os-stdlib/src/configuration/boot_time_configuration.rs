//! `boot_time_configuration` — configuration fixed at boot.

/// Sentinel for `boot_time_configuration`.
pub struct BootTimeConfiguration;

cast::concept! {
    name: "boot_time_configuration",
    summary: "configuration fixed at boot.",
    anchors: [cast_os_stdlib::configuration::boot_time_configuration::BootTimeConfiguration],
    tags: ["cast_os_stdlib", "configuration"],
}
