//! `linux_driver_model` — Linux device/bus/class model.

/// Sentinel for `linux_driver_model`.
pub struct LinuxDriverModel;

cast::concept! {
    name: "linux_driver_model",
    summary: "Linux device/bus/class model.",
    anchors: [cast_os_stdlib::driver_model::linux_driver_model::LinuxDriverModel],
    tags: ["cast_os_stdlib", "driver_model"],
}
