//! `kernel_command_line` — boot-time kernel configuration.

/// Sentinel for `kernel_command_line`.
pub struct KernelCommandLine;

cast::concept! {
    name: "kernel_command_line",
    summary: "boot-time kernel configuration.",
    anchors: [cast_os_stdlib::boot_init::kernel_command_line::KernelCommandLine],
    tags: ["cast_os_stdlib", "boot_init"],
}
