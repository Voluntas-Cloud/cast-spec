//! `copy_based_ipc` — message data copied between spaces.

/// Sentinel for `copy_based_ipc`.
pub struct CopyBasedIpc;

cast::concept! {
    name: "copy_based_ipc",
    summary: "message data copied between spaces.",
    anchors: [cast_os_stdlib::ipc::copy_based_ipc::CopyBasedIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
