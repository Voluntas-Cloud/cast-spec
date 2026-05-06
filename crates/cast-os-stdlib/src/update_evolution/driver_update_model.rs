//! `driver_update_model` — install/update device drivers.

/// Sentinel for `driver_update_model`.
pub struct DriverUpdateModel;

cast::concept! {
    name: "driver_update_model",
    summary: "install/update device drivers.",
    anchors: [cast_os_stdlib::update_evolution::driver_update_model::DriverUpdateModel],
    tags: ["cast_os_stdlib", "update_evolution"],
}
