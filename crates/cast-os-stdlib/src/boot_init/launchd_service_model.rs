//! `launchd_service_model` — Apple service supervision model.

/// Sentinel for `launchd_service_model`.
pub struct LaunchdServiceModel;

cast::concept! {
    name: "launchd_service_model",
    summary: "Apple service supervision model.",
    anchors: [cast_os_stdlib::boot_init::launchd_service_model::LaunchdServiceModel],
    tags: ["cast_os_stdlib", "boot_init"],
}
