//! `user_space` — unprivileged application/service execution environment.

/// Sentinel for `user_space`.
pub struct UserSpace;

cast::concept! {
    name: "user_space",
    summary: "unprivileged application/service execution environment.",
    anchors: [cast_os_stdlib::kernel_user_boundary::user_space::UserSpace],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
