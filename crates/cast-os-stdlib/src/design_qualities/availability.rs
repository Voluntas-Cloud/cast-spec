//! `availability` — remains usable.

/// Sentinel for `availability`.
pub struct Availability;

cast::concept! {
    name: "availability",
    summary: "remains usable.",
    anchors: [cast_os_stdlib::design_qualities::availability::Availability],
    tags: ["cast_os_stdlib", "design_qualities"],
}
