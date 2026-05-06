//! `privilege_separation` — split privileged and unprivileged components.

/// Sentinel for `privilege_separation`.
pub struct PrivilegeSeparation;

cast::concept! {
    name: "privilege_separation",
    summary: "split privileged and unprivileged components.",
    anchors: [cast_os_stdlib::security::privilege_separation::PrivilegeSeparation],
    tags: ["cast_os_stdlib", "security"],
}
