//! `zero_copy_ipc` — shared/mapped transfer avoids copying.

/// Sentinel for `zero_copy_ipc`.
pub struct ZeroCopyIpc;

cast::concept! {
    name: "zero_copy_ipc",
    summary: "shared/mapped transfer avoids copying.",
    anchors: [cast_os_stdlib::ipc::zero_copy_ipc::ZeroCopyIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
