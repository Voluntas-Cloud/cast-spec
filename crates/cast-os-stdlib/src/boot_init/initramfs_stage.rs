//! `initramfs_stage` — temporary early userspace.

/// Sentinel for `initramfs_stage`.
pub struct InitramfsStage;

cast::concept! {
    name: "initramfs_stage",
    summary: "temporary early userspace.",
    anchors: [cast_os_stdlib::boot_init::initramfs_stage::InitramfsStage],
    tags: ["cast_os_stdlib", "boot_init"],
}
