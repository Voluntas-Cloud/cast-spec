//! `user_service_manager` — per-user service supervision.

/// Sentinel for `user_service_manager`.
pub struct UserServiceManager;

cast::concept! {
    name: "user_service_manager",
    summary: "per-user service supervision.",
    anchors: [cast_os_stdlib::service_management::user_service_manager::UserServiceManager],
    tags: ["cast_os_stdlib", "service_management"],
}
