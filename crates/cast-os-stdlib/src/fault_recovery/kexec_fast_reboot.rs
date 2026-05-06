//! `kexec_fast_reboot` — boot new kernel without firmware reset.

/// Sentinel for `kexec_fast_reboot`.
pub struct KexecFastReboot;

cast::concept! {
    name: "kexec_fast_reboot",
    summary: "boot new kernel without firmware reset.",
    anchors: [cast_os_stdlib::fault_recovery::kexec_fast_reboot::KexecFastReboot],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
