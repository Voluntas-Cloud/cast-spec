//! `linux_capabilities` — split root privileges into finer units.

/// Sentinel for `linux_capabilities`.
pub struct LinuxCapabilities;

cast::concept! {
    name: "linux_capabilities",
    summary: "split root privileges into finer units.",
    anchors: [cast_os_stdlib::security::linux_capabilities::LinuxCapabilities],
    tags: ["cast_os_stdlib", "security"],
}
