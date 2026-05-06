//! `mechanism_generality` — primitives can support many policies.

/// Sentinel for `mechanism_generality`.
pub struct MechanismGenerality;

cast::concept! {
    name: "mechanism_generality",
    summary: "primitives can support many policies.",
    anchors: [cast_os_stdlib::design_qualities::mechanism_generality::MechanismGenerality],
    tags: ["cast_os_stdlib", "design_qualities"],
}
