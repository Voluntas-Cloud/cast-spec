//! `log_structured_filesystem` — filesystem organized around sequential logs.

/// Sentinel for `log_structured_filesystem`.
pub struct LogStructuredFilesystem;

cast::concept! {
    name: "log_structured_filesystem",
    summary: "filesystem organized around sequential logs.",
    anchors: [cast_os_stdlib::filesystem_storage::log_structured_filesystem::LogStructuredFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
