//! `control_plane_data_plane_split` — separate decisions from execution.

/// Sentinel for `control_plane_data_plane_split`.
pub struct ControlPlaneDataPlaneSplit;

cast::concept! {
    name: "control_plane_data_plane_split",
    summary: "Separate decisions from execution. Control plane decides \
              what should happen; data plane carries it out at scale.",
    anchors: [cast_stdlib::architecture::control_plane_data_plane_split::ControlPlaneDataPlaneSplit],
    tags: ["cast_stdlib", "architecture"],
}
