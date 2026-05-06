//! `brokered_ipc` — broker mediates communication.

/// Sentinel for `brokered_ipc`.
pub struct BrokeredIpc;

cast::concept! {
    name: "brokered_ipc",
    summary: "broker mediates communication.",
    anchors: [cast_os_stdlib::ipc::brokered_ipc::BrokeredIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
