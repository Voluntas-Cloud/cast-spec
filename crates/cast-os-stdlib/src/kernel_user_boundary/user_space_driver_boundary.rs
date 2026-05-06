//! `user_space_driver_boundary` — drivers operate outside kernel privilege.

/// Sentinel for `user_space_driver_boundary`.
pub struct UserSpaceDriverBoundary;

cast::concept! {
    name: "user_space_driver_boundary",
    summary: "drivers operate outside kernel privilege.",
    anchors: [cast_os_stdlib::kernel_user_boundary::user_space_driver_boundary::UserSpaceDriverBoundary],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
