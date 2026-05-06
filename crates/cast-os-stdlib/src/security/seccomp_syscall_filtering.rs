//! `seccomp_syscall_filtering` — restrict allowed syscalls.

/// Sentinel for `seccomp_syscall_filtering`.
pub struct SeccompSyscallFiltering;

cast::concept! {
    name: "seccomp_syscall_filtering",
    summary: "restrict allowed syscalls.",
    anchors: [cast_os_stdlib::security::seccomp_syscall_filtering::SeccompSyscallFiltering],
    tags: ["cast_os_stdlib", "security"],
}
