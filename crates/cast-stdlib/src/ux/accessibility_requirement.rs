//! `accessibility_requirement` — usability across ability contexts.

/// Sentinel for `accessibility_requirement`.
pub struct AccessibilityRequirement;

cast::concept! {
    name: "accessibility_requirement",
    summary: "Usability across ability contexts: keyboard navigation, \
              screen reader semantics, contrast, motion. Treated as a \
              cross-cutting requirement, not a sprint someone gets \
              assigned the week before launch.",
    anchors: [cast_stdlib::ux::accessibility_requirement::AccessibilityRequirement],
    tags: ["cast_stdlib", "ux"],
}
