//! `bus_driver` — driver for bus such as PCI, USB.

/// Sentinel for `bus_driver`.
pub struct BusDriver;

cast::concept! {
    name: "bus_driver",
    summary: "driver for bus such as PCI, USB.",
    anchors: [cast_os_stdlib::driver_model::bus_driver::BusDriver],
    tags: ["cast_os_stdlib", "driver_model"],
}
