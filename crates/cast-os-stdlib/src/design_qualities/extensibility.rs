//! `extensibility` — supports new features/devices.

/// Sentinel for `extensibility`.
pub struct Extensibility;

cast::concept! {
    name: "extensibility",
    summary: "supports new features/devices.",
    anchors: [cast_os_stdlib::design_qualities::extensibility::Extensibility],
    tags: ["cast_os_stdlib", "design_qualities"],
}
