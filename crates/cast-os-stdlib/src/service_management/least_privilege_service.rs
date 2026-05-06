//! `least_privilege_service` — service with restricted rights.

/// Sentinel for `least_privilege_service`.
pub struct LeastPrivilegeService;

cast::concept! {
    name: "least_privilege_service",
    summary: "service with restricted rights.",
    anchors: [cast_os_stdlib::service_management::least_privilege_service::LeastPrivilegeService],
    tags: ["cast_os_stdlib", "service_management"],
}
