//! `priority_inheritance` — temporarily raise lock holder priority.

/// Sentinel for `priority_inheritance`.
pub struct PriorityInheritance;

cast::concept! {
    name: "priority_inheritance",
    summary: "temporarily raise lock holder priority.",
    anchors: [cast_os_stdlib::scheduling::priority_inheritance::PriorityInheritance],
    tags: ["cast_os_stdlib", "scheduling"],
}
