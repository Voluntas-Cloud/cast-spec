//! `capability_ipc_endpoint` — IPC handle conveys authority.

/// Sentinel for `capability_ipc_endpoint`.
pub struct CapabilityIpcEndpoint;

cast::concept! {
    name: "capability_ipc_endpoint",
    summary: "IPC handle conveys authority.",
    anchors: [cast_os_stdlib::ipc::capability_ipc_endpoint::CapabilityIpcEndpoint],
    tags: ["cast_os_stdlib", "ipc"],
}
