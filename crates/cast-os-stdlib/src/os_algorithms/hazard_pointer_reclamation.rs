//! `hazard_pointer_reclamation` — safe lock-free memory reclamation.

/// Sentinel for `hazard_pointer_reclamation`.
pub struct HazardPointerReclamation;

cast::concept! {
    name: "hazard_pointer_reclamation",
    summary: "safe lock-free memory reclamation.",
    anchors: [cast_os_stdlib::os_algorithms::hazard_pointer_reclamation::HazardPointerReclamation],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
