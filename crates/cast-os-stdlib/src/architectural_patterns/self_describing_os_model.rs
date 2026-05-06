//! `self_describing_os_model` — OS exposes introspection metadata.

/// Sentinel for `self_describing_os_model`.
pub struct SelfDescribingOsModel;

cast::concept! {
    name: "self_describing_os_model",
    summary: "OS exposes introspection metadata.",
    anchors: [cast_os_stdlib::architectural_patterns::self_describing_os_model::SelfDescribingOsModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
