//! `ioctl_control_interface` — device/control-specific syscall escape hatch, because apparently one generic escape hatch was not enough.

/// Sentinel for `ioctl_control_interface`.
pub struct IoctlControlInterface;

cast::concept! {
    name: "ioctl_control_interface",
    summary: "device/control-specific syscall escape hatch, because \
               apparently one generic escape hatch was not enough.",
    anchors: [cast_os_stdlib::kernel_user_boundary::ioctl_control_interface::IoctlControlInterface],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
