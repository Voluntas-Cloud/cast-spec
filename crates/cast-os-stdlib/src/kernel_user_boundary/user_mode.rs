//! `user_mode` — restricted CPU execution mode.

/// Sentinel for `user_mode`.
pub struct UserMode;

cast::concept! {
    name: "user_mode",
    summary: "restricted CPU execution mode.",
    anchors: [cast_os_stdlib::kernel_user_boundary::user_mode::UserMode],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
