//! `server_os` — optimized for services, networking, storage, uptime.

/// Sentinel for `server_os`.
pub struct ServerOs;

cast::concept! {
    name: "server_os",
    summary: "optimized for services, networking, storage, uptime.",
    anchors: [cast_os_stdlib::os_use_cases::server_os::ServerOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
