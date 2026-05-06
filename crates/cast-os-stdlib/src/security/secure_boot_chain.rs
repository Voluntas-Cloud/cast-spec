//! `secure_boot_chain` — boot components verified in sequence.

/// Sentinel for `secure_boot_chain`.
pub struct SecureBootChain;

cast::concept! {
    name: "secure_boot_chain",
    summary: "boot components verified in sequence.",
    anchors: [cast_os_stdlib::security::secure_boot_chain::SecureBootChain],
    tags: ["cast_os_stdlib", "security"],
}
