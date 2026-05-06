//! `everything_is_an_object_model` — resources represented as kernel objects.

/// Sentinel for `everything_is_an_object_model`.
pub struct EverythingIsAnObjectModel;

cast::concept! {
    name: "everything_is_an_object_model",
    summary: "resources represented as kernel objects.",
    anchors: [cast_os_stdlib::architectural_patterns::everything_is_an_object_model::EverythingIsAnObjectModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
