//! `path_resolution_algorithm` — resolve pathname to filesystem object.

/// Sentinel for `path_resolution_algorithm`.
pub struct PathResolutionAlgorithm;

cast::concept! {
    name: "path_resolution_algorithm",
    summary: "resolve pathname to filesystem object.",
    anchors: [cast_os_stdlib::os_algorithms::path_resolution_algorithm::PathResolutionAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
