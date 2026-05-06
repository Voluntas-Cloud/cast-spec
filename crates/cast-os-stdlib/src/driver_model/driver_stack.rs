//! `driver_stack` — layered drivers handling a device/function.

/// Sentinel for `driver_stack`.
pub struct DriverStack;

cast::concept! {
    name: "driver_stack",
    summary: "layered drivers handling a device/function.",
    anchors: [cast_os_stdlib::driver_model::driver_stack::DriverStack],
    tags: ["cast_os_stdlib", "driver_model"],
}
