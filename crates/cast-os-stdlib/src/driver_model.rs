//! Device driver architecture.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod acpi_device_model;
pub mod bottom_half_handler;
pub mod bus_driver;
pub mod class_driver;
pub mod device_power_management;
pub mod device_tree_model;
pub mod dma_engine;
pub mod driver_isolation;
pub mod driver_stack;
pub mod filter_driver;
pub mod firmware_loading_path;
pub mod function_driver;
pub mod hotplug_support;
pub mod interrupt_handler;
pub mod kernel_mode_driver;
pub mod linux_driver_model;
pub mod major_minor_device_number;
pub mod plug_and_play_manager;
pub mod softirq;
pub mod tasklet;
pub mod udev_device_event_model;
pub mod user_mode_driver;
pub mod windows_driver_frameworks;
pub mod windows_driver_model;
pub mod workqueue;

cast::concept! {
    name: "driver_model",
    summary: "Umbrella for the driver_model stdlib category. Device driver \
              architecture.",
    anchors: [
        crate::driver_model::acpi_device_model,
        crate::driver_model::bottom_half_handler,
        crate::driver_model::bus_driver,
        crate::driver_model::class_driver,
        crate::driver_model::device_power_management,
        crate::driver_model::device_tree_model,
        crate::driver_model::dma_engine,
        crate::driver_model::driver_isolation,
        crate::driver_model::driver_stack,
        crate::driver_model::filter_driver,
        crate::driver_model::firmware_loading_path,
        crate::driver_model::function_driver,
        crate::driver_model::hotplug_support,
        crate::driver_model::interrupt_handler,
        crate::driver_model::kernel_mode_driver,
        crate::driver_model::linux_driver_model,
        crate::driver_model::major_minor_device_number,
        crate::driver_model::plug_and_play_manager,
        crate::driver_model::softirq,
        crate::driver_model::tasklet,
        crate::driver_model::udev_device_event_model,
        crate::driver_model::user_mode_driver,
        crate::driver_model::windows_driver_frameworks,
        crate::driver_model::windows_driver_model,
        crate::driver_model::workqueue,
    ],
    tags: ["cast_os_stdlib", "driver_model"],
}

/// Sentinel for the driver_model stdlib group.
pub struct DriverModelGroup;
