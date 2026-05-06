//! `boot_failure` — system cannot reach usable state.

/// Sentinel for `boot_failure`.
pub struct BootFailure;

cast::concept! {
    name: "boot_failure",
    summary: "system cannot reach usable state.",
    anchors: [cast_os_stdlib::failure_modes::boot_failure::BootFailure],
    tags: ["cast_os_stdlib", "failure_modes"],
}
