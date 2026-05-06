//! `class_driver` — generic driver for device class.

/// Sentinel for `class_driver`.
pub struct ClassDriver;

cast::concept! {
    name: "class_driver",
    summary: "generic driver for device class.",
    anchors: [cast_os_stdlib::driver_model::class_driver::ClassDriver],
    tags: ["cast_os_stdlib", "driver_model"],
}
