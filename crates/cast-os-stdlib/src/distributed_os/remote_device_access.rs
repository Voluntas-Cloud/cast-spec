//! `remote_device_access` — devices exposed over network.

/// Sentinel for `remote_device_access`.
pub struct RemoteDeviceAccess;

cast::concept! {
    name: "remote_device_access",
    summary: "devices exposed over network.",
    anchors: [cast_os_stdlib::distributed_os::remote_device_access::RemoteDeviceAccess],
    tags: ["cast_os_stdlib", "distributed_os"],
}
