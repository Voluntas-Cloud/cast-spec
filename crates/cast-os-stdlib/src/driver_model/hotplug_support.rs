//! `hotplug_support` — handle device add/remove at runtime.

/// Sentinel for `hotplug_support`.
pub struct HotplugSupport;

cast::concept! {
    name: "hotplug_support",
    summary: "handle device add/remove at runtime.",
    anchors: [cast_os_stdlib::driver_model::hotplug_support::HotplugSupport],
    tags: ["cast_os_stdlib", "driver_model"],
}
