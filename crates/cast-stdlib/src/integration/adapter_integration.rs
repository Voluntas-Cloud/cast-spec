//! `adapter_integration` — translate external system API.

/// Sentinel for `adapter_integration`.
pub struct AdapterIntegration;

cast::concept! {
    name: "adapter_integration",
    summary: "Translate an external API to fit the internal model. \
              The adapter is a single, replaceable seam; without it, \
              the external API's vocabulary leaks into every caller \
              and survives long after that vendor relationship ends.",
    anchors: [cast_stdlib::integration::adapter_integration::AdapterIntegration],
    tags: ["cast_stdlib", "integration"],
}
