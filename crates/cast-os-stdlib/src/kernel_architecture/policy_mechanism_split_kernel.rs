//! `policy_mechanism_split_kernel` — kernel provides mechanisms; policy lives elsewhere.

/// Sentinel for `policy_mechanism_split_kernel`.
pub struct PolicyMechanismSplitKernel;

cast::concept! {
    name: "policy_mechanism_split_kernel",
    summary: "kernel provides mechanisms; policy lives elsewhere.",
    anchors: [cast_os_stdlib::kernel_architecture::policy_mechanism_split_kernel::PolicyMechanismSplitKernel],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
