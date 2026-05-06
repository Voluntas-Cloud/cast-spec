//! `minimality` — small trusted computing base.

/// Sentinel for `minimality`.
pub struct Minimality;

cast::concept! {
    name: "minimality",
    summary: "small trusted computing base.",
    anchors: [cast_os_stdlib::design_qualities::minimality::Minimality],
    tags: ["cast_os_stdlib", "design_qualities"],
}
