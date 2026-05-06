//! `path_activation` — start service when filesystem path changes.

/// Sentinel for `path_activation`.
pub struct PathActivation;

cast::concept! {
    name: "path_activation",
    summary: "start service when filesystem path changes.",
    anchors: [cast_os_stdlib::service_management::path_activation::PathActivation],
    tags: ["cast_os_stdlib", "service_management"],
}
