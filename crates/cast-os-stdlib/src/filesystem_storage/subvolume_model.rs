//! `subvolume_model` — independently managed filesystem subtree.

/// Sentinel for `subvolume_model`.
pub struct SubvolumeModel;

cast::concept! {
    name: "subvolume_model",
    summary: "independently managed filesystem subtree.",
    anchors: [cast_os_stdlib::filesystem_storage::subvolume_model::SubvolumeModel],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
