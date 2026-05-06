//! `epoch_based_reclamation` — reclaim after readers leave epoch.

/// Sentinel for `epoch_based_reclamation`.
pub struct EpochBasedReclamation;

cast::concept! {
    name: "epoch_based_reclamation",
    summary: "reclaim after readers leave epoch.",
    anchors: [cast_os_stdlib::os_algorithms::epoch_based_reclamation::EpochBasedReclamation],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
