//! `fsync_semantics` — durability boundary for file writes.

/// Sentinel for `fsync_semantics`.
pub struct FsyncSemantics;

cast::concept! {
    name: "fsync_semantics",
    summary: "durability boundary for file writes.",
    anchors: [cast_os_stdlib::filesystem_storage::fsync_semantics::FsyncSemantics],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
