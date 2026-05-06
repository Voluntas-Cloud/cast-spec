//! `secure_boot_policy` — verifies boot component signatures.

/// Sentinel for `secure_boot_policy`.
pub struct SecureBootPolicy;

cast::concept! {
    name: "secure_boot_policy",
    summary: "verifies boot component signatures.",
    anchors: [cast_os_stdlib::boot_init::secure_boot_policy::SecureBootPolicy],
    tags: ["cast_os_stdlib", "boot_init"],
}
