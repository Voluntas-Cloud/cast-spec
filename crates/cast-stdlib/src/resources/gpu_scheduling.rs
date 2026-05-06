//! `gpu_scheduling` — allocate accelerator resources.

/// Sentinel for `gpu_scheduling`.
pub struct GpuScheduling;

cast::concept! {
    name: "gpu_scheduling",
    summary: "Allocate accelerator resources. GPUs and similar devices \
              are scarce, expensive, and non-fungible across vendors and \
              memory tiers; the scheduler must treat them as named \
              resources, not generic capacity.",
    anchors: [cast_stdlib::resources::gpu_scheduling::GpuScheduling],
    tags: ["cast_stdlib", "resources"],
}
