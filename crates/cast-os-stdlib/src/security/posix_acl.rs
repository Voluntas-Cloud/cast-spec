//! `posix_acl` — extended Unix access control.

/// Sentinel for `posix_acl`.
pub struct PosixAcl;

cast::concept! {
    name: "posix_acl",
    summary: "extended Unix access control.",
    anchors: [cast_os_stdlib::security::posix_acl::PosixAcl],
    tags: ["cast_os_stdlib", "security"],
}
