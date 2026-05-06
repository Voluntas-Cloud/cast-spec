//! `software_raid` — redundant disk arrays in software.

/// Sentinel for `software_raid`.
pub struct SoftwareRaid;

cast::concept! {
    name: "software_raid",
    summary: "redundant disk arrays in software.",
    anchors: [cast_os_stdlib::filesystem_storage::software_raid::SoftwareRaid],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
