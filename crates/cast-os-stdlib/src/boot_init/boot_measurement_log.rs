//! `boot_measurement_log` — records boot component measurements.

/// Sentinel for `boot_measurement_log`.
pub struct BootMeasurementLog;

cast::concept! {
    name: "boot_measurement_log",
    summary: "records boot component measurements.",
    anchors: [cast_os_stdlib::boot_init::boot_measurement_log::BootMeasurementLog],
    tags: ["cast_os_stdlib", "boot_init"],
}
