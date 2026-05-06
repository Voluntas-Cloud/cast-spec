//! `receive_side_scaling` — distribute network receive across CPUs.

/// Sentinel for `receive_side_scaling`.
pub struct ReceiveSideScaling;

cast::concept! {
    name: "receive_side_scaling",
    summary: "distribute network receive across CPUs.",
    anchors: [cast_os_stdlib::networking::receive_side_scaling::ReceiveSideScaling],
    tags: ["cast_os_stdlib", "networking"],
}
