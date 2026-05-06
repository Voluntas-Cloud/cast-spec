//! `container_escape_risk` — failure mode where isolation breaks.

/// Sentinel for `container_escape_risk`.
pub struct ContainerEscapeRisk;

cast::concept! {
    name: "container_escape_risk",
    summary: "failure mode where isolation breaks.",
    anchors: [cast_os_stdlib::isolation::container_escape_risk::ContainerEscapeRisk],
    tags: ["cast_os_stdlib", "isolation"],
}
