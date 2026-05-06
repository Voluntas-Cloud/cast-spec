//! `syscall_dispatch_table` — mapping from syscall number/API to kernel implementation.

/// Sentinel for `syscall_dispatch_table`.
pub struct SyscallDispatchTable;

cast::concept! {
    name: "syscall_dispatch_table",
    summary: "mapping from syscall number/API to kernel implementation.",
    anchors: [cast_os_stdlib::kernel_user_boundary::syscall_dispatch_table::SyscallDispatchTable],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
