//! `firmware_loading_path` — driver loads firmware into device.

/// Sentinel for `firmware_loading_path`.
pub struct FirmwareLoadingPath;

cast::concept! {
    name: "firmware_loading_path",
    summary: "driver loads firmware into device.",
    anchors: [cast_os_stdlib::driver_model::firmware_loading_path::FirmwareLoadingPath],
    tags: ["cast_os_stdlib", "driver_model"],
}
