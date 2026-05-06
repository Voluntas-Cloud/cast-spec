//! `filesystem_recovery` — repair or replay after crash.

/// Sentinel for `filesystem_recovery`.
pub struct FilesystemRecovery;

cast::concept! {
    name: "filesystem_recovery",
    summary: "repair or replay after crash.",
    anchors: [cast_os_stdlib::fault_recovery::filesystem_recovery::FilesystemRecovery],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
