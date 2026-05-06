//! `network_appliance_os` — routing/firewall/switching-focused OS.

/// Sentinel for `network_appliance_os`.
pub struct NetworkApplianceOs;

cast::concept! {
    name: "network_appliance_os",
    summary: "routing/firewall/switching-focused OS.",
    anchors: [cast_os_stdlib::os_use_cases::network_appliance_os::NetworkApplianceOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
