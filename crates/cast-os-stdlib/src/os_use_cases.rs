//! OS use-case architecture concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod ai_accelerator_host_os;
pub mod cloud_host_os;
pub mod container_host_os;
pub mod developer_workstation_os;
pub mod edge_node_os;
pub mod embedded_os;
pub mod general_purpose_desktop_os;
pub mod hypervisor_host_os;
pub mod kiosk_os;
pub mod live_os_image;
pub mod mobile_os;
pub mod network_appliance_os;
pub mod personal_cloud_os;
pub mod real_time_embedded_os;
pub mod rescue_os;
pub mod robotics_os_platform;
pub mod safety_critical_os;
pub mod secure_workstation_os;
pub mod server_os;
pub mod storage_appliance_os;

cast::concept! {
    name: "os_use_cases",
    summary: "Umbrella for the os_use_cases stdlib category. OS use-case \
              architecture concepts.",
    anchors: [
        crate::os_use_cases::ai_accelerator_host_os,
        crate::os_use_cases::cloud_host_os,
        crate::os_use_cases::container_host_os,
        crate::os_use_cases::developer_workstation_os,
        crate::os_use_cases::edge_node_os,
        crate::os_use_cases::embedded_os,
        crate::os_use_cases::general_purpose_desktop_os,
        crate::os_use_cases::hypervisor_host_os,
        crate::os_use_cases::kiosk_os,
        crate::os_use_cases::live_os_image,
        crate::os_use_cases::mobile_os,
        crate::os_use_cases::network_appliance_os,
        crate::os_use_cases::personal_cloud_os,
        crate::os_use_cases::real_time_embedded_os,
        crate::os_use_cases::rescue_os,
        crate::os_use_cases::robotics_os_platform,
        crate::os_use_cases::safety_critical_os,
        crate::os_use_cases::secure_workstation_os,
        crate::os_use_cases::server_os,
        crate::os_use_cases::storage_appliance_os,
    ],
    tags: ["cast_os_stdlib", "os_use_cases"],
}

/// Sentinel for the os_use_cases stdlib group.
pub struct OsUseCasesGroup;
