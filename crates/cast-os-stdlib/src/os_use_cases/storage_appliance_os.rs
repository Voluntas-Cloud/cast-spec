//! `storage_appliance_os` — filesystem/block/object storage-focused OS.

/// Sentinel for `storage_appliance_os`.
pub struct StorageApplianceOs;

cast::concept! {
    name: "storage_appliance_os",
    summary: "filesystem/block/object storage-focused OS.",
    anchors: [cast_os_stdlib::os_use_cases::storage_appliance_os::StorageApplianceOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
