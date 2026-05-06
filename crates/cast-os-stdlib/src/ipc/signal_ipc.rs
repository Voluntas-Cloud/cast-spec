//! `signal_ipc` — asynchronous process notification.

/// Sentinel for `signal_ipc`.
pub struct SignalIpc;

cast::concept! {
    name: "signal_ipc",
    summary: "asynchronous process notification.",
    anchors: [cast_os_stdlib::ipc::signal_ipc::SignalIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
