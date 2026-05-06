//! `device_tree_model` — hardware described by tree structure.

/// Sentinel for `device_tree_model`.
pub struct DeviceTreeModel;

cast::concept! {
    name: "device_tree_model",
    summary: "hardware described by tree structure.",
    anchors: [cast_os_stdlib::driver_model::device_tree_model::DeviceTreeModel],
    tags: ["cast_os_stdlib", "driver_model"],
}
