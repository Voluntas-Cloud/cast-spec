//! `user_mode_driver` — driver runs outside kernel.

/// Sentinel for `user_mode_driver`.
pub struct UserModeDriver;

cast::concept! {
    name: "user_mode_driver",
    summary: "driver runs outside kernel.",
    anchors: [cast_os_stdlib::driver_model::user_mode_driver::UserModeDriver],
    tags: ["cast_os_stdlib", "driver_model"],
}
