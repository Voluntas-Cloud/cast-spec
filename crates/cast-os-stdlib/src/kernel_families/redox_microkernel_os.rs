//! `redox_microkernel_os` — Rust-based microkernel-style OS.

/// Sentinel for `redox_microkernel_os`.
pub struct RedoxMicrokernelOs;

cast::concept! {
    name: "redox_microkernel_os",
    summary: "Rust-based microkernel-style OS.",
    anchors: [cast_os_stdlib::kernel_families::redox_microkernel_os::RedoxMicrokernelOs],
    tags: ["cast_os_stdlib", "kernel_families"],
}
