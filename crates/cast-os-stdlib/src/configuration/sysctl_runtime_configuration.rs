//! `sysctl_runtime_configuration` — kernel/runtime tunables.

/// Sentinel for `sysctl_runtime_configuration`.
pub struct SysctlRuntimeConfiguration;

cast::concept! {
    name: "sysctl_runtime_configuration",
    summary: "kernel/runtime tunables.",
    anchors: [cast_os_stdlib::configuration::sysctl_runtime_configuration::SysctlRuntimeConfiguration],
    tags: ["cast_os_stdlib", "configuration"],
}
