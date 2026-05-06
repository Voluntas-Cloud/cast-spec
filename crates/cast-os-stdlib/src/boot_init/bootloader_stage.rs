//! `bootloader_stage` — loads kernel and initial environment.

/// Sentinel for `bootloader_stage`.
pub struct BootloaderStage;

cast::concept! {
    name: "bootloader_stage",
    summary: "loads kernel and initial environment.",
    anchors: [cast_os_stdlib::boot_init::bootloader_stage::BootloaderStage],
    tags: ["cast_os_stdlib", "boot_init"],
}
