//! `filesystem_corruption` — persistent state damaged.

/// Sentinel for `filesystem_corruption`.
pub struct FilesystemCorruption;

cast::concept! {
    name: "filesystem_corruption",
    summary: "persistent state damaged.",
    anchors: [cast_os_stdlib::failure_modes::filesystem_corruption::FilesystemCorruption],
    tags: ["cast_os_stdlib", "failure_modes"],
}
