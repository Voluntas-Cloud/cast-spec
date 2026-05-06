//! `tun_tap_device` — user-space virtual network device.

/// Sentinel for `tun_tap_device`.
pub struct TunTapDevice;

cast::concept! {
    name: "tun_tap_device",
    summary: "user-space virtual network device.",
    anchors: [cast_os_stdlib::networking::tun_tap_device::TunTapDevice],
    tags: ["cast_os_stdlib", "networking"],
}
