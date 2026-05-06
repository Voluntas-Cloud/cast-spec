//! `metadata_journaling` — journal metadata only.

/// Sentinel for `metadata_journaling`.
pub struct MetadataJournaling;

cast::concept! {
    name: "metadata_journaling",
    summary: "journal metadata only.",
    anchors: [cast_os_stdlib::filesystem_storage::metadata_journaling::MetadataJournaling],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
