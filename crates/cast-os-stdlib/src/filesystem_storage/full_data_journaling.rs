//! `full_data_journaling` — journal both metadata and data.

/// Sentinel for `full_data_journaling`.
pub struct FullDataJournaling;

cast::concept! {
    name: "full_data_journaling",
    summary: "journal both metadata and data.",
    anchors: [cast_os_stdlib::filesystem_storage::full_data_journaling::FullDataJournaling],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
