//! `firmware_boot_stage` — BIOS/UEFI initialization.

/// Sentinel for `firmware_boot_stage`.
pub struct FirmwareBootStage;

cast::concept! {
    name: "firmware_boot_stage",
    summary: "BIOS/UEFI initialization.",
    anchors: [cast_os_stdlib::boot_init::firmware_boot_stage::FirmwareBootStage],
    tags: ["cast_os_stdlib", "boot_init"],
}
