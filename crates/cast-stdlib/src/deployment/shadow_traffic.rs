//! `shadow_traffic` — send copied traffic to new system.

/// Sentinel for `shadow_traffic`.
pub struct ShadowTraffic;

cast::concept! {
    name: "shadow_traffic",
    summary: "Send copied traffic to new system. The new system sees \
              real production load without serving real users; \
              responses are compared, not returned.",
    anchors: [cast_stdlib::deployment::shadow_traffic::ShadowTraffic],
    tags: ["cast_stdlib", "deployment"],
}
