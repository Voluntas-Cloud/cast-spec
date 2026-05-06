//! `user_space_service_os` — core services moved out of kernel.

/// Sentinel for `user_space_service_os`.
pub struct UserSpaceServiceOs;

cast::concept! {
    name: "user_space_service_os",
    summary: "core services moved out of kernel.",
    anchors: [cast_os_stdlib::architectural_patterns::user_space_service_os::UserSpaceServiceOs],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
