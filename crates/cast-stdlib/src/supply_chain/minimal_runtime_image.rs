//! `minimal_runtime_image` — ship only what is needed.

/// Sentinel for `minimal_runtime_image`.
pub struct MinimalRuntimeImage;

cast::concept! {
    name: "minimal_runtime_image",
    summary: "Ship only what is needed. No build tools, no shell, \
              no extra binaries — every removed component is a \
              vulnerability that cannot be exploited.",
    anchors: [cast_stdlib::supply_chain::minimal_runtime_image::MinimalRuntimeImage],
    tags: ["cast_stdlib", "supply_chain"],
}
