//! `recovery_boot_mode` — minimal repair mode.

/// Sentinel for `recovery_boot_mode`.
pub struct RecoveryBootMode;

cast::concept! {
    name: "recovery_boot_mode",
    summary: "minimal repair mode.",
    anchors: [cast_os_stdlib::boot_init::recovery_boot_mode::RecoveryBootMode],
    tags: ["cast_os_stdlib", "boot_init"],
}
