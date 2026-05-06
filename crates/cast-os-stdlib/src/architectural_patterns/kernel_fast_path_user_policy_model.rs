//! `kernel_fast_path_user_policy_model` — kernel handles fast path; user space manages policy.

/// Sentinel for `kernel_fast_path_user_policy_model`.
pub struct KernelFastPathUserPolicyModel;

cast::concept! {
    name: "kernel_fast_path_user_policy_model",
    summary: "kernel handles fast path; user space manages policy.",
    anchors: [cast_os_stdlib::architectural_patterns::kernel_fast_path_user_policy_model::KernelFastPathUserPolicyModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
