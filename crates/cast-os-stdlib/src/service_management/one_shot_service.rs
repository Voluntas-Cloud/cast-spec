//! `one_shot_service` — task-like service.

/// Sentinel for `one_shot_service`.
pub struct OneShotService;

cast::concept! {
    name: "one_shot_service",
    summary: "task-like service.",
    anchors: [cast_os_stdlib::service_management::one_shot_service::OneShotService],
    tags: ["cast_os_stdlib", "service_management"],
}
