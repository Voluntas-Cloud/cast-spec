//! `safe_mode_boot` — restricted diagnostic boot.

/// Sentinel for `safe_mode_boot`.
pub struct SafeModeBoot;

cast::concept! {
    name: "safe_mode_boot",
    summary: "restricted diagnostic boot.",
    anchors: [cast_os_stdlib::boot_init::safe_mode_boot::SafeModeBoot],
    tags: ["cast_os_stdlib", "boot_init"],
}
