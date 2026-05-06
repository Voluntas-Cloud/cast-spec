//! `bind_mount` — remount subtree elsewhere.

/// Sentinel for `bind_mount`.
pub struct BindMount;

cast::concept! {
    name: "bind_mount",
    summary: "remount subtree elsewhere.",
    anchors: [cast_os_stdlib::filesystem_storage::bind_mount::BindMount],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
