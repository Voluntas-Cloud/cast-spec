//! `network_filesystem` — filesystem over network.

/// Sentinel for `network_filesystem`.
pub struct NetworkFilesystem;

cast::concept! {
    name: "network_filesystem",
    summary: "filesystem over network.",
    anchors: [cast_os_stdlib::filesystem_storage::network_filesystem::NetworkFilesystem],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
