//! `union_filesystem` — multiple filesystem layers merged into one view.

/// Sentinel for `union_filesystem`.
pub struct UnionFilesystem;

cast::concept! {
    name: "union_filesystem",
    summary: "multiple filesystem layers merged into one view.",
    anchors: [cast_os_stdlib::filesystem_storage::union_filesystem::UnionFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
