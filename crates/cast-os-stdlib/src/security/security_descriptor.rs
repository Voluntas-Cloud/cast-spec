//! `security_descriptor` — Windows-style security metadata object.

/// Sentinel for `security_descriptor`.
pub struct SecurityDescriptor;

cast::concept! {
    name: "security_descriptor",
    summary: "Windows-style security metadata object.",
    anchors: [cast_os_stdlib::security::security_descriptor::SecurityDescriptor],
    tags: ["cast_os_stdlib", "security"],
}
