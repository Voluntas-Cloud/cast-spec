//! `linux_ftrace` — Linux function tracing framework.

/// Sentinel for `linux_ftrace`.
pub struct LinuxFtrace;

cast::concept! {
    name: "linux_ftrace",
    summary: "Linux function tracing framework.",
    anchors: [cast_os_stdlib::observability::linux_ftrace::LinuxFtrace],
    tags: ["cast_os_stdlib", "observability"],
}
