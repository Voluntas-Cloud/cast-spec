//! `plan9_distributed_os_model` — OS design where resources are exposed through file-like interfaces.

/// Sentinel for `plan9_distributed_os_model`.
pub struct Plan9DistributedOsModel;

cast::concept! {
    name: "plan9_distributed_os_model",
    summary: "OS design where resources are exposed through file-like \
               interfaces.",
    anchors: [cast_os_stdlib::kernel_families::plan9_distributed_os_model::Plan9DistributedOsModel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
