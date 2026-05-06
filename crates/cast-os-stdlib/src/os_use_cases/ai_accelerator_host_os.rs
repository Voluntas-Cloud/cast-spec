//! `ai_accelerator_host_os` — OS managing GPUs/NPUs and model workloads.

/// Sentinel for `ai_accelerator_host_os`.
pub struct AiAcceleratorHostOs;

cast::concept! {
    name: "ai_accelerator_host_os",
    summary: "OS managing GPUs/NPUs and model workloads.",
    anchors: [cast_os_stdlib::os_use_cases::ai_accelerator_host_os::AiAcceleratorHostOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
