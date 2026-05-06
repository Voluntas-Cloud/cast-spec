//! `trusted_platform_module` — hardware root for keys/measurements.

/// Sentinel for `trusted_platform_module`.
pub struct TrustedPlatformModule;

cast::concept! {
    name: "trusted_platform_module",
    summary: "hardware root for keys/measurements.",
    anchors: [cast_os_stdlib::security::trusted_platform_module::TrustedPlatformModule],
    tags: ["cast_os_stdlib", "security"],
}
