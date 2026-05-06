//! `kernel_abi` — binary interface exposed by the kernel.

/// Sentinel for `kernel_abi`.
pub struct KernelAbi;

cast::concept! {
    name: "kernel_abi",
    summary: "binary interface exposed by the kernel.",
    anchors: [cast_os_stdlib::kernel_user_boundary::kernel_abi::KernelAbi],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
