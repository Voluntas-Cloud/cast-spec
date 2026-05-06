//! `measured_boot_chain` — boot state recorded for attestation.

/// Sentinel for `measured_boot_chain`.
pub struct MeasuredBootChain;

cast::concept! {
    name: "measured_boot_chain",
    summary: "boot state recorded for attestation.",
    anchors: [cast_os_stdlib::security::measured_boot_chain::MeasuredBootChain],
    tags: ["cast_os_stdlib", "security"],
}
