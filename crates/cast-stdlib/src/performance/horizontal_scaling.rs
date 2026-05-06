//! `horizontal_scaling` — add more instances.

/// Sentinel for `horizontal_scaling`.
pub struct HorizontalScaling;

cast::concept! {
    name: "horizontal_scaling",
    summary: "Add more instances. Capacity grows by replication; \
              works only when the workload partitions cleanly and \
              shared state isn't the bottleneck.",
    anchors: [cast_stdlib::performance::horizontal_scaling::HorizontalScaling],
    tags: ["cast_stdlib", "performance"],
}
