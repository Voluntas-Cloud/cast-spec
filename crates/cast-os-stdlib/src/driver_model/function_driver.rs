//! `function_driver` — driver for specific device function.

/// Sentinel for `function_driver`.
pub struct FunctionDriver;

cast::concept! {
    name: "function_driver",
    summary: "driver for specific device function.",
    anchors: [cast_os_stdlib::driver_model::function_driver::FunctionDriver],
    tags: ["cast_os_stdlib", "driver_model"],
}
