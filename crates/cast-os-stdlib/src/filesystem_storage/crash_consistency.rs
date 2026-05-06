//! `crash_consistency` — filesystem remains valid after crash.

/// Sentinel for `crash_consistency`.
pub struct CrashConsistency;

cast::concept! {
    name: "crash_consistency",
    summary: "filesystem remains valid after crash.",
    anchors: [cast_os_stdlib::filesystem_storage::crash_consistency::CrashConsistency],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
