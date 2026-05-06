//! `linux_with_user_space_control_planes` — Linux kernel plus systemd, udev, dbus, containers, etc.

/// Sentinel for `linux_with_user_space_control_planes`.
pub struct LinuxWithUserSpaceControlPlanes;

cast::concept! {
    name: "linux_with_user_space_control_planes",
    summary: "Linux kernel plus systemd, udev, dbus, containers, etc.",
    anchors: [cast_os_stdlib::kernel_families::linux_with_user_space_control_planes::LinuxWithUserSpaceControlPlanes],
    tags: ["cast_os_stdlib", "kernel_families"],
}
