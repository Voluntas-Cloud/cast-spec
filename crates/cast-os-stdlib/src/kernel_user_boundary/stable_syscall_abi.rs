//! `stable_syscall_abi` — compatibility promise for syscall behavior.

/// Sentinel for `stable_syscall_abi`.
pub struct StableSyscallAbi;

cast::concept! {
    name: "stable_syscall_abi",
    summary: "compatibility promise for syscall behavior.",
    anchors: [cast_os_stdlib::kernel_user_boundary::stable_syscall_abi::StableSyscallAbi],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
