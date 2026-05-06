//! `root_filesystem_mount` — transition to real root filesystem.

/// Sentinel for `root_filesystem_mount`.
pub struct RootFilesystemMount;

cast::concept! {
    name: "root_filesystem_mount",
    summary: "transition to real root filesystem.",
    anchors: [cast_os_stdlib::boot_init::root_filesystem_mount::RootFilesystemMount],
    tags: ["cast_os_stdlib", "boot_init"],
}
