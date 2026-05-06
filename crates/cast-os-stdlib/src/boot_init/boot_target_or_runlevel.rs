//! `boot_target_or_runlevel` — system startup mode.

/// Sentinel for `boot_target_or_runlevel`.
pub struct BootTargetOrRunlevel;

cast::concept! {
    name: "boot_target_or_runlevel",
    summary: "system startup mode.",
    anchors: [cast_os_stdlib::boot_init::boot_target_or_runlevel::BootTargetOrRunlevel],
    tags: ["cast_os_stdlib", "boot_init"],
}
