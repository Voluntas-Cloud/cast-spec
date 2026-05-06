//! `pull_based_device_polling_model` — OS polls devices under load.

/// Sentinel for `pull_based_device_polling_model`.
pub struct PullBasedDevicePollingModel;

cast::concept! {
    name: "pull_based_device_polling_model",
    summary: "OS polls devices under load.",
    anchors: [cast_os_stdlib::architectural_patterns::pull_based_device_polling_model::PullBasedDevicePollingModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
