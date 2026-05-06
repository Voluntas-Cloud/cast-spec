//! `kiosk_os` — locked-down single-purpose environment.

/// Sentinel for `kiosk_os`.
pub struct KioskOs;

cast::concept! {
    name: "kiosk_os",
    summary: "locked-down single-purpose environment.",
    anchors: [cast_os_stdlib::os_use_cases::kiosk_os::KioskOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
