//! `keyring_subsystem` — kernel/user credential storage.

/// Sentinel for `keyring_subsystem`.
pub struct KeyringSubsystem;

cast::concept! {
    name: "keyring_subsystem",
    summary: "kernel/user credential storage.",
    anchors: [cast_os_stdlib::security::keyring_subsystem::KeyringSubsystem],
    tags: ["cast_os_stdlib", "security"],
}
