//! `init_process` — first user-space process.

/// Sentinel for `init_process`.
pub struct InitProcess;

cast::concept! {
    name: "init_process",
    summary: "first user-space process.",
    anchors: [cast_os_stdlib::boot_init::init_process::InitProcess],
    tags: ["cast_os_stdlib", "boot_init"],
}
