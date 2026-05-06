//! `vertical_scaling` — give one instance more resources.

/// Sentinel for `vertical_scaling`.
pub struct VerticalScaling;

cast::concept! {
    name: "vertical_scaling",
    summary: "Give one instance more resources. Bigger box, more CPU/\
              memory; simpler than horizontal scaling but capped by \
              the largest available machine.",
    anchors: [cast_stdlib::performance::vertical_scaling::VerticalScaling],
    tags: ["cast_stdlib", "performance"],
}
