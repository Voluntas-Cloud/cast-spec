//! `resource_limit` — maximum allowed resource use.

/// Sentinel for `resource_limit`.
pub struct ResourceLimit;

cast::concept! {
    name: "resource_limit",
    summary: "Maximum allowed resource use. Crossing the limit is \
              rejected, throttled, or kills the workload depending on \
              the resource; an absent limit silently means \"all of it\".",
    anchors: [cast_stdlib::resources::resource_limit::ResourceLimit],
    tags: ["cast_stdlib", "resources"],
}
