//! `firmware_update_model` — update device/board firmware.

/// Sentinel for `firmware_update_model`.
pub struct FirmwareUpdateModel;

cast::concept! {
    name: "firmware_update_model",
    summary: "update device/board firmware.",
    anchors: [cast_os_stdlib::update_evolution::firmware_update_model::FirmwareUpdateModel],
    tags: ["cast_os_stdlib", "update_evolution"],
}
